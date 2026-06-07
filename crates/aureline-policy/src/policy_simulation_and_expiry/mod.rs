//! Stable policy-simulation, exception-preview, approval-history, and expiry packet.
//!
//! This module projects the beta policy simulation records from
//! [`crate::simulation`] into the stable product objects consumed by desktop,
//! CLI/headless, managed-admin, and support handoff surfaces. The packet keeps
//! policy diffs, exception scopes, approval history, expiry banners, chronology
//! refs, and reapproval triggers together so waivers and remembered decisions
//! cannot become invisible permanent state.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::simulation::{
    audit_policy_simulation_beta_page, DashboardBucketClass, ExceptionalAuthorityRecord,
    MemoryStateClass, PolicySimulationBetaDefectKind, PolicySimulationBetaPage,
    PolicySimulationRecord, RememberedDecisionDriftReason, RememberedDecisionRecord,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every stable policy simulation and expiry record.
pub const POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by desktop, CLI/headless, admin, and support surfaces.
pub const POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF: &str =
    "policy:simulation_exception_expiry_stable:v1";

/// Record-kind tag for [`PolicySimulationAndExpiryPage`] payloads.
pub const POLICY_SIMULATION_AND_EXPIRY_PAGE_RECORD_KIND: &str =
    "policy_simulation_exception_expiry_page_record";

/// Record-kind tag for [`PolicySimulationView`] payloads.
pub const POLICY_SIMULATION_VIEW_RECORD_KIND: &str = "policy_simulation_view_record";

/// Record-kind tag for [`ExceptionPreviewSheet`] payloads.
pub const EXCEPTION_PREVIEW_SHEET_RECORD_KIND: &str = "policy_exception_preview_sheet_record";

/// Record-kind tag for [`ApprovalHistoryRow`] payloads.
pub const APPROVAL_HISTORY_ROW_RECORD_KIND: &str = "policy_approval_history_row_record";

/// Record-kind tag for [`PolicyDiffImpactSummary`] payloads.
pub const POLICY_DIFF_IMPACT_SUMMARY_RECORD_KIND: &str = "policy_diff_impact_summary_record";

/// Record-kind tag for [`ExpiryBanner`] payloads.
pub const EXPIRY_BANNER_RECORD_KIND: &str = "policy_expiry_banner_record";

/// Record-kind tag for [`PolicySimulationExceptionExpiryReviewPacket`] payloads.
pub const POLICY_SIMULATION_EXCEPTION_EXPIRY_REVIEW_PACKET_RECORD_KIND: &str =
    "policy_simulation_exception_expiry_review_packet_record";

/// Record-kind tag for [`PolicySimulationAndExpiryDefect`] payloads.
pub const POLICY_SIMULATION_AND_EXPIRY_DEFECT_RECORD_KIND: &str =
    "policy_simulation_exception_expiry_defect_record";

/// Record-kind tag for [`PolicySimulationAndExpirySupportExport`] payloads.
pub const POLICY_SIMULATION_AND_EXPIRY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_simulation_exception_expiry_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const POLICY_SIMULATION_AND_EXPIRY_DOC_REF: &str =
    "docs/enterprise/m4/policy-simulation-exception-and-expiry.md";

/// Repo-relative path of the stable artifact summary for this lane.
pub const POLICY_SIMULATION_AND_EXPIRY_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/policy-simulation-exception-and-expiry.md";

/// Product surface that must project the same policy simulation truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySimulationProjectionSurfaceClass {
    /// Desktop policy center, trust sheet, or approval drawer.
    Desktop,
    /// CLI or headless explain output.
    CliHeadless,
    /// Managed-admin handoff, support export, or case packet.
    AdminSupportHandoff,
}

impl PolicySimulationProjectionSurfaceClass {
    /// All required projection surfaces in canonical order.
    pub const ALL: [Self; 3] = [Self::Desktop, Self::CliHeadless, Self::AdminSupportHandoff];

    /// Stable token recorded on review packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::AdminSupportHandoff => "admin_support_handoff",
        }
    }
}

/// Subject represented by an expiry banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpirySubjectClass {
    /// Policy exception record.
    PolicyException,
    /// Policy waiver record.
    PolicyWaiver,
    /// Remembered approval or remembered decision record.
    RememberedDecision,
}

impl ExpirySubjectClass {
    /// Stable token recorded on expiry banners.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyException => "policy_exception",
            Self::PolicyWaiver => "policy_waiver",
            Self::RememberedDecision => "remembered_decision",
        }
    }
}

/// Material drift trigger that forces remembered-decision revalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReapprovalTriggerClass {
    /// Target identity or object binding changed.
    TargetDrift,
    /// Policy epoch or rule posture changed.
    PolicyDrift,
    /// Target, provider, runtime, or bundle version changed.
    VersionDrift,
    /// Authority issuer, authority epoch, or root of authority changed.
    AuthorityDrift,
}

impl ReapprovalTriggerClass {
    /// All stable drift triggers remembered decisions must expose.
    pub const ALL: [Self; 4] = [
        Self::TargetDrift,
        Self::PolicyDrift,
        Self::VersionDrift,
        Self::AuthorityDrift,
    ];

    /// Stable token recorded on approval rows and review packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetDrift => "target_drift",
            Self::PolicyDrift => "policy_drift",
            Self::VersionDrift => "version_drift",
            Self::AuthorityDrift => "authority_drift",
        }
    }
}

/// Stability qualification for the stable policy simulation and expiry packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySimulationAndExpiryQualificationClass {
    /// All stable product objects are complete and cross-surface projections agree.
    Stable,
    /// The packet is reviewable but a non-withdrawal defect narrows the claim.
    NeedsReview,
    /// Raw private material crossed the export boundary.
    Withdrawn,
}

impl PolicySimulationAndExpiryQualificationClass {
    /// Stable token recorded on summaries.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NeedsReview => "needs_review",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Typed defect for the stable policy simulation and expiry packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicySimulationAndExpiryDefectKind {
    /// The upstream beta policy simulation page has defects.
    SourcePolicySimulationDefects,
    /// A simulation view lacks changed keys or feature areas.
    SimulationViewMissingChangedAreas,
    /// A simulation view is missing policy diff linkage.
    SimulationViewMissingDiffSummary,
    /// An exception or waiver sheet is incomplete.
    ExceptionPreviewSheetIncomplete,
    /// An exception or remembered decision is missing an expiry banner.
    ExpiryBannerMissing,
    /// A high-risk remembered decision lacks a bounded expiry.
    IndefiniteHighRiskRememberedDecision,
    /// Approval history lacks one of the material drift triggers.
    ApprovalHistoryMissingReapprovalTrigger,
    /// Review packet does not cover every required surface.
    CrossSurfaceProjectionMissing,
    /// Review packet lacks chronology lineage.
    ChronologyMissing,
    /// Raw private material crossed the stable export boundary.
    RawPrivateMaterialExposed,
}

impl PolicySimulationAndExpiryDefectKind {
    /// Stable token recorded on defect rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourcePolicySimulationDefects => "source_policy_simulation_defects",
            Self::SimulationViewMissingChangedAreas => "simulation_view_missing_changed_areas",
            Self::SimulationViewMissingDiffSummary => "simulation_view_missing_diff_summary",
            Self::ExceptionPreviewSheetIncomplete => "exception_preview_sheet_incomplete",
            Self::ExpiryBannerMissing => "expiry_banner_missing",
            Self::IndefiniteHighRiskRememberedDecision => {
                "indefinite_high_risk_remembered_decision"
            }
            Self::ApprovalHistoryMissingReapprovalTrigger => {
                "approval_history_missing_reapproval_trigger"
            }
            Self::CrossSurfaceProjectionMissing => "cross_surface_projection_missing",
            Self::ChronologyMissing => "chronology_missing",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
        }
    }

    /// True when this defect withdraws the packet.
    pub const fn withdraws_packet(self) -> bool {
        matches!(self, Self::RawPrivateMaterialExposed)
    }
}

/// Stable policy simulation view shown before enforcement changes land.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable simulation view id.
    pub simulation_view_id: String,
    /// Source beta simulation id.
    pub source_simulation_id: String,
    /// Actor who requested or sourced the preview.
    pub actor_ref: String,
    /// Source policy or admin surface that produced the preview.
    pub source_ref: String,
    /// Scope under preview.
    pub scope_ref: String,
    /// UTC creation time for the preview.
    pub created_at: String,
    /// UTC instant when the change would become effective or needs review.
    pub review_target_at: String,
    /// Changed policy keys or feature areas.
    pub changed_keys_or_feature_areas: Vec<String>,
    /// Previous policy, setting, or bundle value.
    pub previous_value_ref: String,
    /// Simulated policy, setting, or bundle value.
    pub simulated_value_ref: String,
    /// User-visible consequence of the simulated change.
    pub user_visible_consequence: String,
    /// Affected surface refs.
    pub affected_surface_refs: Vec<String>,
    /// Degraded-mode consequences for affected surfaces.
    pub degraded_mode_consequences: Vec<String>,
    /// Notes shown when stale/offline context affects the preview.
    pub stale_offline_notes: Vec<String>,
    /// Linked policy diff summary refs.
    pub policy_diff_summary_refs: Vec<String>,
    /// Linked exception preview sheet refs.
    pub exception_preview_sheet_refs: Vec<String>,
    /// Linked approval history row refs.
    pub approval_history_row_refs: Vec<String>,
    /// Linked expiry banner refs.
    pub expiry_banner_refs: Vec<String>,
    /// Export-safe audit and chronology lineage refs.
    pub export_safe_lineage_refs: Vec<String>,
}

/// Exception or waiver preview sheet with exact bypass and lapse behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExceptionPreviewSheet {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable sheet id.
    pub sheet_id: String,
    /// Source exception or waiver id.
    pub exception_ref: String,
    /// Actor or policy source that created the sheet.
    pub source_ref: String,
    /// Scope under exception preview.
    pub scope_ref: String,
    /// UTC creation time.
    pub created_at: String,
    /// Scope and operation bypassed by the exception.
    pub exact_bypass_scope: String,
    /// Owner or approver responsible for the exception.
    pub owner_or_approver_ref: String,
    /// Export-safe reason label.
    pub reason: String,
    /// Mitigation that makes the exception acceptable while active.
    pub mitigation: String,
    /// Evidence links supporting the exception.
    pub evidence_link_refs: Vec<String>,
    /// UTC instant when the exception expires or requires review.
    pub expiry_target_at: String,
    /// Fallback behavior once the exception lapses.
    pub fallback_behavior_on_lapse: String,
    /// Export-safe audit and chronology lineage refs.
    pub export_safe_lineage_refs: Vec<String>,
    /// Linked simulation view refs.
    pub linked_simulation_view_refs: Vec<String>,
}

/// Approval history row for a governed remembered decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalHistoryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable row id.
    pub approval_history_row_id: String,
    /// Source remembered-decision id.
    pub remembered_decision_ref: String,
    /// Source policy or exception ref that governs the remembered decision.
    pub source_ref: String,
    /// Actor binding.
    pub actor_ref: String,
    /// Object binding.
    pub object_ref: String,
    /// Action family binding.
    pub action_family_token: String,
    /// Environment binding.
    pub environment_ref: String,
    /// Scope where the remembered decision applies.
    pub scope_ref: String,
    /// UTC creation time.
    pub created_at: String,
    /// UTC expiry time.
    pub expires_at: String,
    /// UTC review prompt target.
    pub review_target_at: String,
    /// Current remembered-decision state token.
    pub memory_state_token: String,
    /// Revoke action ref exposed by UI and CLI.
    pub revoke_action_ref: String,
    /// Open-details action ref exposed by UI and CLI.
    pub open_details_action_ref: String,
    /// Material drift triggers that force reapproval.
    pub reapproval_triggers: Vec<ReapprovalTriggerClass>,
    /// Stable tokens for [`Self::reapproval_triggers`].
    pub reapproval_trigger_tokens: Vec<String>,
    /// Drift reasons currently detected.
    pub invalidation_reason_tokens: Vec<String>,
    /// True when this row is bounded by an expiry horizon.
    pub bounded_by_expiry: bool,
    /// Export-safe audit and chronology lineage refs.
    pub export_safe_lineage_refs: Vec<String>,
}

/// Policy diff and impact summary for a changed key or feature area.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyDiffImpactSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable diff summary id.
    pub diff_summary_id: String,
    /// Source simulation id.
    pub simulation_ref: String,
    /// Actor who requested or sourced the diff.
    pub actor_ref: String,
    /// Policy or settings source under comparison.
    pub source_ref: String,
    /// Scope under diff.
    pub scope_ref: String,
    /// UTC creation time.
    pub created_at: String,
    /// UTC review target.
    pub review_target_at: String,
    /// Changed policy key or feature area.
    pub changed_key_or_feature_area: String,
    /// Previous value or policy state.
    pub previous_value: String,
    /// Simulated value or policy state.
    pub simulated_value: String,
    /// User-visible consequence.
    pub user_visible_consequence: String,
    /// Affected surfaces.
    pub affected_surface_refs: Vec<String>,
    /// Degraded-mode consequences.
    pub degraded_mode_consequences: Vec<String>,
    /// Stale/offline notes.
    pub stale_offline_notes: Vec<String>,
    /// Export-safe audit and chronology lineage refs.
    pub export_safe_lineage_refs: Vec<String>,
}

/// Expiry banner that keeps exceptions and remembered approvals time-bounded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryBanner {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable banner id.
    pub banner_id: String,
    /// Subject class.
    pub subject_class: ExpirySubjectClass,
    /// Stable token for [`Self::subject_class`].
    pub subject_class_token: String,
    /// Subject ref.
    pub subject_ref: String,
    /// Actor or source responsible for the expiring object.
    pub actor_or_source_ref: String,
    /// Scope where the expiring object applies.
    pub scope_ref: String,
    /// UTC creation time.
    pub created_at: String,
    /// What expires.
    pub what_expires: String,
    /// Exact UTC expiry date and time.
    pub exact_expiry_at: String,
    /// Export-safe relative time label.
    pub relative_time_label: String,
    /// Consequence when expiry is reached.
    pub consequence_on_expiry: String,
    /// Renewal or review action ref.
    pub renew_or_review_action_ref: String,
    /// Export-safe audit and chronology lineage refs.
    pub export_safe_lineage_refs: Vec<String>,
}

/// One reviewable packet tying simulation, exception, approval, expiry, and chronology truth together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationExceptionExpiryReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable review packet id.
    pub review_packet_id: String,
    /// Linked policy diff summaries.
    pub policy_diff_summary_refs: Vec<String>,
    /// Linked simulation views.
    pub simulation_outcome_refs: Vec<String>,
    /// Owners for approvals, exceptions, or waivers.
    pub approval_or_waiver_owner_refs: Vec<String>,
    /// Linked expiry banners.
    pub expiry_banner_refs: Vec<String>,
    /// Chronology and audit refs required for handoff.
    pub chronology_refs: Vec<String>,
    /// Stable reapproval triggers represented by the packet.
    pub reapproval_triggers: Vec<ReapprovalTriggerClass>,
    /// Stable tokens for [`Self::reapproval_triggers`].
    pub reapproval_trigger_tokens: Vec<String>,
    /// Surfaces that ingest this exact packet.
    pub surface_projections: Vec<PolicySimulationProjectionSurfaceClass>,
    /// Stable tokens for [`Self::surface_projections`].
    pub surface_projection_tokens: Vec<String>,
}

/// Aggregate summary for a stable policy simulation and expiry page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationAndExpirySummary {
    /// Number of simulation views.
    pub simulation_view_count: usize,
    /// Number of exception preview sheets.
    pub exception_preview_sheet_count: usize,
    /// Number of approval history rows.
    pub approval_history_row_count: usize,
    /// Number of policy diff summaries.
    pub policy_diff_summary_count: usize,
    /// Number of expiry banners.
    pub expiry_banner_count: usize,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// Overall qualification token.
    pub overall_qualification_token: String,
}

impl PolicySimulationAndExpirySummary {
    /// Builds a summary from stable page parts and typed defects.
    pub fn from_page_parts(
        simulation_views: &[PolicySimulationView],
        exception_preview_sheets: &[ExceptionPreviewSheet],
        approval_history_rows: &[ApprovalHistoryRow],
        policy_diff_summaries: &[PolicyDiffImpactSummary],
        expiry_banners: &[ExpiryBanner],
        defects: &[PolicySimulationAndExpiryDefect],
    ) -> Self {
        let mut defect_counts_by_kind = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }
        let qualification = if defects
            .iter()
            .any(|defect| defect.defect_kind.withdraws_packet())
        {
            PolicySimulationAndExpiryQualificationClass::Withdrawn
        } else if defects.is_empty() {
            PolicySimulationAndExpiryQualificationClass::Stable
        } else {
            PolicySimulationAndExpiryQualificationClass::NeedsReview
        };
        Self {
            simulation_view_count: simulation_views.len(),
            exception_preview_sheet_count: exception_preview_sheets.len(),
            approval_history_row_count: approval_history_rows.len(),
            policy_diff_summary_count: policy_diff_summaries.len(),
            expiry_banner_count: expiry_banners.len(),
            defect_count: defects.len(),
            defect_counts_by_kind,
            overall_qualification_token: qualification.as_str().to_owned(),
        }
    }
}

/// Stable defect row for policy simulation and expiry packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationAndExpiryDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable defect id.
    pub defect_id: String,
    /// Defect kind.
    pub defect_kind: PolicySimulationAndExpiryDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id.
    pub subject_id: String,
    /// Export-safe note.
    pub note: String,
}

impl PolicySimulationAndExpiryDefect {
    fn new(
        defect_kind: PolicySimulationAndExpiryDefectKind,
        subject_id: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let subject_id = subject_id.into();
        Self {
            record_kind: POLICY_SIMULATION_AND_EXPIRY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:simulation-expiry:{}:{}",
                defect_kind.as_str(),
                subject_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id,
            note: note.into(),
        }
    }
}

/// Stable page consumed by policy center, approval sheets, CLI explain, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationAndExpiryPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC generation timestamp.
    pub generated_at: String,
    /// Source beta page.
    pub source_policy_simulation_page: PolicySimulationBetaPage,
    /// Stable simulation views.
    pub simulation_views: Vec<PolicySimulationView>,
    /// Stable exception preview sheets.
    pub exception_preview_sheets: Vec<ExceptionPreviewSheet>,
    /// Stable approval history rows.
    pub approval_history_rows: Vec<ApprovalHistoryRow>,
    /// Stable policy diff summaries.
    pub policy_diff_summaries: Vec<PolicyDiffImpactSummary>,
    /// Stable expiry banners.
    pub expiry_banners: Vec<ExpiryBanner>,
    /// One cross-surface review packet.
    pub review_packet: PolicySimulationExceptionExpiryReviewPacket,
    /// Typed defects.
    pub defects: Vec<PolicySimulationAndExpiryDefect>,
    /// Aggregate summary.
    pub summary: PolicySimulationAndExpirySummary,
}

impl PolicySimulationAndExpiryPage {
    /// Builds a stable page from the beta policy simulation page.
    pub fn from_policy_simulation_page(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        source_policy_simulation_page: PolicySimulationBetaPage,
    ) -> Self {
        let generated_at = generated_at.into();
        let exception_preview_sheets =
            build_exception_preview_sheets(&source_policy_simulation_page);
        let approval_history_rows = build_approval_history_rows(&source_policy_simulation_page);
        let expiry_banners = build_expiry_banners(&source_policy_simulation_page);
        let policy_diff_summaries = build_policy_diff_summaries(&source_policy_simulation_page);
        let simulation_views = build_simulation_views(
            &source_policy_simulation_page,
            &exception_preview_sheets,
            &approval_history_rows,
            &expiry_banners,
            &policy_diff_summaries,
        );
        let review_packet = build_review_packet(
            "policy:simulation-expiry:review-packet:default",
            &source_policy_simulation_page,
            &simulation_views,
            &exception_preview_sheets,
            &approval_history_rows,
            &expiry_banners,
            &policy_diff_summaries,
        );
        let mut page = Self {
            record_kind: POLICY_SIMULATION_AND_EXPIRY_PAGE_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at,
            source_policy_simulation_page,
            simulation_views,
            exception_preview_sheets,
            approval_history_rows,
            policy_diff_summaries,
            expiry_banners,
            review_packet,
            defects: Vec::new(),
            summary: PolicySimulationAndExpirySummary::from_page_parts(
                &[],
                &[],
                &[],
                &[],
                &[],
                &[],
            ),
        };
        page.defects = audit_policy_simulation_and_expiry_page(&page);
        page.summary = PolicySimulationAndExpirySummary::from_page_parts(
            &page.simulation_views,
            &page.exception_preview_sheets,
            &page.approval_history_rows,
            &page.policy_diff_summaries,
            &page.expiry_banners,
            &page.defects,
        );
        page
    }

    /// True when the packet qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == PolicySimulationAndExpiryQualificationClass::Stable.as_str()
    }

    /// True when desktop, CLI/headless, and admin/support surfaces are all covered.
    pub fn covers_required_projection_surfaces(&self) -> bool {
        let projected: BTreeSet<PolicySimulationProjectionSurfaceClass> = self
            .review_packet
            .surface_projections
            .iter()
            .copied()
            .collect();
        PolicySimulationProjectionSurfaceClass::ALL
            .iter()
            .all(|surface| projected.contains(surface))
    }

    /// True when every approval history row includes all material drift triggers.
    pub fn approval_history_revalidates_on_material_drift(&self) -> bool {
        self.approval_history_rows.iter().all(row_has_all_triggers)
    }
}

/// Support-export wrapper for the stable policy simulation and expiry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicySimulationAndExpirySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Durable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub exported_at: String,
    /// Embedded stable page.
    pub page: PolicySimulationAndExpiryPage,
    /// Defect counts by token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
}

impl PolicySimulationAndExpirySupportExport {
    /// Builds a support export from a stable page.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: PolicySimulationAndExpiryPage,
    ) -> Self {
        Self {
            record_kind: POLICY_SIMULATION_AND_EXPIRY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            defect_counts_by_kind: page.summary.defect_counts_by_kind.clone(),
            page,
            raw_private_material_excluded: true,
        }
    }
}

/// Builds the seeded stable policy simulation and expiry page.
pub fn seeded_policy_simulation_and_expiry_page() -> PolicySimulationAndExpiryPage {
    PolicySimulationAndExpiryPage::from_policy_simulation_page(
        "policy:simulation-expiry:default",
        "Policy simulation, exception preview, approval history, and expiry governance",
        "2026-06-01T00:00:00Z",
        crate::simulation::seeded_policy_simulation_beta_page(),
    )
}

/// Validates a stable policy simulation and expiry page.
pub fn validate_policy_simulation_and_expiry_page(
    page: &PolicySimulationAndExpiryPage,
) -> Result<(), Vec<PolicySimulationAndExpiryDefect>> {
    let defects = audit_policy_simulation_and_expiry_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes typed defects for a stable policy simulation and expiry page.
pub fn audit_policy_simulation_and_expiry_page(
    page: &PolicySimulationAndExpiryPage,
) -> Vec<PolicySimulationAndExpiryDefect> {
    let mut defects = Vec::new();
    let source_defects = audit_policy_simulation_beta_page(&page.source_policy_simulation_page);
    if source_defects.iter().any(|defect| {
        defect.defect_kind == PolicySimulationBetaDefectKind::RawPrivateMaterialExposed
    }) {
        defects.push(PolicySimulationAndExpiryDefect::new(
            PolicySimulationAndExpiryDefectKind::RawPrivateMaterialExposed,
            "source_policy_simulation_page",
            "source policy simulation page exposed raw private material",
        ));
        return defects;
    }
    if !source_defects.is_empty() {
        defects.push(PolicySimulationAndExpiryDefect::new(
            PolicySimulationAndExpiryDefectKind::SourcePolicySimulationDefects,
            "source_policy_simulation_page",
            "source policy simulation page has defects and must be reviewed before stable projection",
        ));
    }

    for view in &page.simulation_views {
        if view.changed_keys_or_feature_areas.is_empty() {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::SimulationViewMissingChangedAreas,
                view.simulation_view_id.clone(),
                "simulation view must preview changed keys or feature areas",
            ));
        }
        if view.policy_diff_summary_refs.is_empty() {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::SimulationViewMissingDiffSummary,
                view.simulation_view_id.clone(),
                "simulation view must link to policy diff and impact summary records",
            ));
        }
        if view.actor_ref.is_empty()
            || view.source_ref.is_empty()
            || view.scope_ref.is_empty()
            || view.created_at.is_empty()
            || view.review_target_at.is_empty()
            || view.export_safe_lineage_refs.is_empty()
        {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ChronologyMissing,
                view.simulation_view_id.clone(),
                "simulation view must carry actor/source, scope, created/review time, and export-safe lineage",
            ));
        }
    }

    for sheet in &page.exception_preview_sheets {
        if sheet.exact_bypass_scope.is_empty()
            || sheet.source_ref.is_empty()
            || sheet.scope_ref.is_empty()
            || sheet.created_at.is_empty()
            || sheet.owner_or_approver_ref.is_empty()
            || sheet.reason.is_empty()
            || sheet.mitigation.is_empty()
            || sheet.evidence_link_refs.is_empty()
            || sheet.expiry_target_at.is_empty()
            || sheet.fallback_behavior_on_lapse.is_empty()
        {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ExceptionPreviewSheetIncomplete,
                sheet.sheet_id.clone(),
                "exception preview sheets must name scope, owner, reason, mitigation, evidence, expiry, and lapse fallback",
            ));
        }
    }

    for summary in &page.policy_diff_summaries {
        if summary.actor_ref.is_empty()
            || summary.source_ref.is_empty()
            || summary.scope_ref.is_empty()
            || summary.created_at.is_empty()
            || summary.review_target_at.is_empty()
            || summary.export_safe_lineage_refs.is_empty()
        {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ChronologyMissing,
                summary.diff_summary_id.clone(),
                "policy diff summaries must carry actor/source, scope, created/review time, and export-safe lineage",
            ));
        }
    }

    let banner_subjects: BTreeSet<&str> = page
        .expiry_banners
        .iter()
        .map(|banner| banner.subject_ref.as_str())
        .collect();
    for exception in &page.source_policy_simulation_page.exceptions {
        if !banner_subjects.contains(exception.exception_id.as_str()) {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ExpiryBannerMissing,
                exception.exception_id.clone(),
                "every exception or waiver must have an expiry banner",
            ));
        }
    }
    for remembered in &page.source_policy_simulation_page.remembered_decisions {
        if !banner_subjects.contains(remembered.remembered_decision_id.as_str()) {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ExpiryBannerMissing,
                remembered.remembered_decision_id.clone(),
                "every remembered decision must have an expiry banner",
            ));
        }
    }

    for row in &page.approval_history_rows {
        if !row.bounded_by_expiry && is_high_risk_action_family(&row.action_family_token) {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::IndefiniteHighRiskRememberedDecision,
                row.approval_history_row_id.clone(),
                "destructive, networked, provider-backed, remote, or secret-bearing remembered decisions must not be indefinite by default",
            ));
        }
        if !row_has_all_triggers(row) {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ApprovalHistoryMissingReapprovalTrigger,
                row.approval_history_row_id.clone(),
                "approval history rows must expose target, policy, version, and authority drift triggers",
            ));
        }
        if row.source_ref.is_empty()
            || row.scope_ref.is_empty()
            || row.created_at.is_empty()
            || row.review_target_at.is_empty()
            || row.export_safe_lineage_refs.is_empty()
        {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ChronologyMissing,
                row.approval_history_row_id.clone(),
                "approval history rows must carry source, scope, created/review time, and export-safe lineage",
            ));
        }
    }

    for banner in &page.expiry_banners {
        if banner.actor_or_source_ref.is_empty()
            || banner.scope_ref.is_empty()
            || banner.created_at.is_empty()
            || banner.exact_expiry_at.is_empty()
            || banner.export_safe_lineage_refs.is_empty()
        {
            defects.push(PolicySimulationAndExpiryDefect::new(
                PolicySimulationAndExpiryDefectKind::ExpiryBannerMissing,
                banner.banner_id.clone(),
                "expiry banners must carry actor/source, scope, created time, exact expiry, and export-safe lineage",
            ));
        }
    }

    if !page.covers_required_projection_surfaces() {
        defects.push(PolicySimulationAndExpiryDefect::new(
            PolicySimulationAndExpiryDefectKind::CrossSurfaceProjectionMissing,
            page.review_packet.review_packet_id.clone(),
            "review packet must project the same truth across desktop, CLI/headless, and admin/support handoff",
        ));
    }
    if page.review_packet.chronology_refs.is_empty() {
        defects.push(PolicySimulationAndExpiryDefect::new(
            PolicySimulationAndExpiryDefectKind::ChronologyMissing,
            page.review_packet.review_packet_id.clone(),
            "review packet must include chronology refs for audit and support handoff",
        ));
    }

    defects
}

fn build_policy_diff_summaries(
    source_page: &PolicySimulationBetaPage,
) -> Vec<PolicyDiffImpactSummary> {
    source_page
        .simulations
        .iter()
        .map(|simulation| {
            let affected_surface_refs: Vec<String> = simulation
                .affected_surfaces
                .iter()
                .map(|surface| surface.surface_ref.subject_id.clone())
                .collect();
            let degraded_mode_consequences = simulation
                .affected_surfaces
                .iter()
                .map(|surface| {
                    format!(
                        "{} => {}: {}",
                        surface.surface_ref.subject_id,
                        surface.degraded_mode_token,
                        surface.consequence
                    )
                })
                .collect();
            PolicyDiffImpactSummary {
                record_kind: POLICY_DIFF_IMPACT_SUMMARY_RECORD_KIND.to_owned(),
                schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
                shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
                diff_summary_id: diff_summary_id(&simulation.request.simulation_id),
                simulation_ref: simulation.request.simulation_id.clone(),
                actor_ref: simulation.request.requested_by.stable_id.clone(),
                source_ref: simulation.request.proposed_ref.clone(),
                scope_ref: simulation.request.scope.scope_id.clone(),
                created_at: simulation.request.evaluated_at.clone(),
                review_target_at: simulation.request.effective_from.clone(),
                changed_key_or_feature_area: changed_areas(simulation).join(","),
                previous_value: simulation.request.baseline_ref.clone(),
                simulated_value: simulation.request.proposed_ref.clone(),
                user_visible_consequence: simulation_consequence(simulation),
                affected_surface_refs,
                degraded_mode_consequences,
                stale_offline_notes: vec![format!(
                    "stale/offline replay must refresh policy epoch before applying {}",
                    simulation.request.proposed_ref
                )],
                export_safe_lineage_refs: simulation_lineage_refs(source_page, simulation),
            }
        })
        .collect()
}

fn build_simulation_views(
    source_page: &PolicySimulationBetaPage,
    exception_sheets: &[ExceptionPreviewSheet],
    approval_rows: &[ApprovalHistoryRow],
    expiry_banners: &[ExpiryBanner],
    diff_summaries: &[PolicyDiffImpactSummary],
) -> Vec<PolicySimulationView> {
    source_page
        .simulations
        .iter()
        .map(|simulation| {
            let source_id = &simulation.request.simulation_id;
            let exception_preview_sheet_refs = exception_sheets
                .iter()
                .filter(|sheet| {
                    simulation
                        .exception_preview_refs
                        .iter()
                        .any(|exception_ref| exception_ref == &sheet.exception_ref)
                })
                .map(|sheet| sheet.sheet_id.clone())
                .collect();
            let approval_history_row_refs = approval_rows
                .iter()
                .filter(|row| {
                    simulation
                        .remembered_decision_preview_refs
                        .iter()
                        .any(|memory_ref| memory_ref == &row.remembered_decision_ref)
                })
                .map(|row| row.approval_history_row_id.clone())
                .collect();
            let expiry_banner_refs = expiry_banners
                .iter()
                .filter(|banner| {
                    simulation
                        .exception_preview_refs
                        .iter()
                        .chain(simulation.remembered_decision_preview_refs.iter())
                        .any(|subject_ref| subject_ref == &banner.subject_ref)
                })
                .map(|banner| banner.banner_id.clone())
                .collect();
            let policy_diff_summary_refs = diff_summaries
                .iter()
                .filter(|summary| &summary.simulation_ref == source_id)
                .map(|summary| summary.diff_summary_id.clone())
                .collect();
            PolicySimulationView {
                record_kind: POLICY_SIMULATION_VIEW_RECORD_KIND.to_owned(),
                schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
                shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
                simulation_view_id: simulation_view_id(source_id),
                source_simulation_id: source_id.clone(),
                actor_ref: simulation.request.requested_by.stable_id.clone(),
                source_ref: simulation.request.proposed_ref.clone(),
                scope_ref: simulation.request.scope.scope_id.clone(),
                created_at: simulation.request.evaluated_at.clone(),
                review_target_at: simulation.request.effective_from.clone(),
                changed_keys_or_feature_areas: changed_areas(simulation),
                previous_value_ref: simulation.request.baseline_ref.clone(),
                simulated_value_ref: simulation.request.proposed_ref.clone(),
                user_visible_consequence: simulation_consequence(simulation),
                affected_surface_refs: simulation
                    .affected_surfaces
                    .iter()
                    .map(|surface| surface.surface_ref.subject_id.clone())
                    .collect(),
                degraded_mode_consequences: simulation
                    .affected_surfaces
                    .iter()
                    .map(|surface| {
                        format!("{}: {}", surface.degraded_mode_token, surface.consequence)
                    })
                    .collect(),
                stale_offline_notes: vec![
                    "offline or stale policy caches must refresh before enforcement changes apply"
                        .to_owned(),
                ],
                policy_diff_summary_refs,
                exception_preview_sheet_refs,
                approval_history_row_refs,
                expiry_banner_refs,
                export_safe_lineage_refs: simulation_lineage_refs(source_page, simulation),
            }
        })
        .collect()
}

fn build_exception_preview_sheets(
    source_page: &PolicySimulationBetaPage,
) -> Vec<ExceptionPreviewSheet> {
    source_page
        .exceptions
        .iter()
        .map(|exception| {
            let linked_simulation_view_refs = source_page
                .simulations
                .iter()
                .filter(|simulation| {
                    simulation
                        .exception_preview_refs
                        .iter()
                        .any(|exception_ref| exception_ref == &exception.exception_id)
                })
                .map(|simulation| simulation_view_id(&simulation.request.simulation_id))
                .collect();
            ExceptionPreviewSheet {
                record_kind: EXCEPTION_PREVIEW_SHEET_RECORD_KIND.to_owned(),
                schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
                shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
                sheet_id: exception_sheet_id(&exception.exception_id),
                exception_ref: exception.exception_id.clone(),
                source_ref: exception.exception_kind_token.clone(),
                scope_ref: exception.scope.scope_id.clone(),
                created_at: exception.time_horizon.declared_at.clone(),
                exact_bypass_scope: format!(
                    "{}:{}",
                    exception.scope.scope_kind_token, exception.scope.scope_id
                ),
                owner_or_approver_ref: exception.owner.stable_id.clone(),
                reason: format!(
                    "{} scoped exception preview",
                    exception.exception_kind_token
                ),
                mitigation: format!(
                    "limit bypass to {} until {} and keep {} as revocation path",
                    exception.scope.scope_id,
                    exception.time_horizon.expires_at,
                    exception.revocation_path_token
                ),
                evidence_link_refs: exception.evidence_trail_refs.clone(),
                expiry_target_at: exception.time_horizon.expires_at.clone(),
                fallback_behavior_on_lapse: format!(
                    "fallback to {} via {}",
                    exception.dashboard_bucket_token, exception.renewal_path_token
                ),
                export_safe_lineage_refs: exception.audit_lineage_refs.clone(),
                linked_simulation_view_refs,
            }
        })
        .collect()
}

fn build_approval_history_rows(source_page: &PolicySimulationBetaPage) -> Vec<ApprovalHistoryRow> {
    source_page
        .remembered_decisions
        .iter()
        .map(|remembered| ApprovalHistoryRow {
            record_kind: APPROVAL_HISTORY_ROW_RECORD_KIND.to_owned(),
            schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
            shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
            approval_history_row_id: approval_history_row_id(&remembered.remembered_decision_id),
            remembered_decision_ref: remembered.remembered_decision_id.clone(),
            source_ref: remembered
                .related_exception_ref
                .clone()
                .unwrap_or_else(|| remembered.policy_epoch.clone()),
            actor_ref: remembered.bound_actor.stable_id.clone(),
            object_ref: remembered.object.subject_id.clone(),
            action_family_token: remembered.action_family_token.clone(),
            environment_ref: remembered.environment.environment_id.clone(),
            scope_ref: remembered.environment.workspace_ref.clone(),
            created_at: remembered.time_horizon.declared_at.clone(),
            expires_at: remembered.time_horizon.expires_at.clone(),
            review_target_at: remembered.time_horizon.reapproval_prompt_at.clone(),
            memory_state_token: remembered.memory_state_token.clone(),
            revoke_action_ref: format!(
                "policy.approval.revoke:{}",
                remembered.remembered_decision_id
            ),
            open_details_action_ref: format!(
                "policy.approval.open_details:{}",
                remembered.remembered_decision_id
            ),
            reapproval_triggers: ReapprovalTriggerClass::ALL.to_vec(),
            reapproval_trigger_tokens: ReapprovalTriggerClass::ALL
                .iter()
                .map(|trigger| trigger.as_str().to_owned())
                .collect(),
            invalidation_reason_tokens: remembered.invalidation_reason_tokens.clone(),
            bounded_by_expiry: !remembered.time_horizon.expires_at.is_empty(),
            export_safe_lineage_refs: remembered.audit_lineage_refs.clone(),
        })
        .collect()
}

fn build_expiry_banners(source_page: &PolicySimulationBetaPage) -> Vec<ExpiryBanner> {
    let exception_banners = source_page.exceptions.iter().map(exception_expiry_banner);
    let memory_banners = source_page
        .remembered_decisions
        .iter()
        .map(remembered_decision_expiry_banner);
    exception_banners.chain(memory_banners).collect()
}

fn exception_expiry_banner(exception: &ExceptionalAuthorityRecord) -> ExpiryBanner {
    let subject_class = if exception.exception_kind_token.contains("waiver") {
        ExpirySubjectClass::PolicyWaiver
    } else {
        ExpirySubjectClass::PolicyException
    };
    ExpiryBanner {
        record_kind: EXPIRY_BANNER_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
        banner_id: expiry_banner_id(&exception.exception_id),
        subject_class,
        subject_class_token: subject_class.as_str().to_owned(),
        subject_ref: exception.exception_id.clone(),
        actor_or_source_ref: exception.owner.stable_id.clone(),
        scope_ref: exception.scope.scope_id.clone(),
        created_at: exception.time_horizon.declared_at.clone(),
        what_expires: format!(
            "{} for {}",
            exception.exception_kind_token, exception.scope.scope_id
        ),
        exact_expiry_at: exception.time_horizon.expires_at.clone(),
        relative_time_label: format!("review by {}", exception.time_horizon.reapproval_prompt_at),
        consequence_on_expiry: format!(
            "actions fall back to {} and require {}",
            exception.dashboard_bucket_token, exception.renewal_path_token
        ),
        renew_or_review_action_ref: format!("policy.exception.review:{}", exception.exception_id),
        export_safe_lineage_refs: exception.audit_lineage_refs.clone(),
    }
}

fn remembered_decision_expiry_banner(remembered: &RememberedDecisionRecord) -> ExpiryBanner {
    ExpiryBanner {
        record_kind: EXPIRY_BANNER_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
        banner_id: expiry_banner_id(&remembered.remembered_decision_id),
        subject_class: ExpirySubjectClass::RememberedDecision,
        subject_class_token: ExpirySubjectClass::RememberedDecision.as_str().to_owned(),
        subject_ref: remembered.remembered_decision_id.clone(),
        actor_or_source_ref: remembered.bound_actor.stable_id.clone(),
        scope_ref: remembered.environment.workspace_ref.clone(),
        created_at: remembered.time_horizon.declared_at.clone(),
        what_expires: format!(
            "remembered {} approval for {}",
            remembered.action_family_token, remembered.object.subject_id
        ),
        exact_expiry_at: remembered.time_horizon.expires_at.clone(),
        relative_time_label: format!(
            "reprompt by {}",
            remembered.time_horizon.reapproval_prompt_at
        ),
        consequence_on_expiry: "fresh approval is required before reuse after drift or expiry"
            .to_owned(),
        renew_or_review_action_ref: format!(
            "policy.approval.review:{}",
            remembered.remembered_decision_id
        ),
        export_safe_lineage_refs: remembered.audit_lineage_refs.clone(),
    }
}

fn build_review_packet(
    review_packet_id: &str,
    source_page: &PolicySimulationBetaPage,
    simulation_views: &[PolicySimulationView],
    exception_preview_sheets: &[ExceptionPreviewSheet],
    approval_rows: &[ApprovalHistoryRow],
    expiry_banners: &[ExpiryBanner],
    diff_summaries: &[PolicyDiffImpactSummary],
) -> PolicySimulationExceptionExpiryReviewPacket {
    let mut owner_refs: BTreeSet<String> = exception_preview_sheets
        .iter()
        .map(|sheet| sheet.owner_or_approver_ref.clone())
        .collect();
    owner_refs.extend(approval_rows.iter().map(|row| row.actor_ref.clone()));

    let mut chronology_refs: BTreeSet<String> = source_page
        .action_time_policy_states
        .iter()
        .flat_map(|snapshot| snapshot.audit_event_refs.iter().cloned())
        .collect();
    chronology_refs.extend(
        exception_preview_sheets
            .iter()
            .flat_map(|sheet| sheet.export_safe_lineage_refs.iter().cloned()),
    );

    PolicySimulationExceptionExpiryReviewPacket {
        record_kind: POLICY_SIMULATION_EXCEPTION_EXPIRY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: POLICY_SIMULATION_AND_EXPIRY_SCHEMA_VERSION,
        shared_contract_ref: POLICY_SIMULATION_AND_EXPIRY_SHARED_CONTRACT_REF.to_owned(),
        review_packet_id: review_packet_id.to_owned(),
        policy_diff_summary_refs: diff_summaries
            .iter()
            .map(|summary| summary.diff_summary_id.clone())
            .collect(),
        simulation_outcome_refs: simulation_views
            .iter()
            .map(|view| view.simulation_view_id.clone())
            .collect(),
        approval_or_waiver_owner_refs: owner_refs.into_iter().collect(),
        expiry_banner_refs: expiry_banners
            .iter()
            .map(|banner| banner.banner_id.clone())
            .collect(),
        chronology_refs: chronology_refs.into_iter().collect(),
        reapproval_triggers: ReapprovalTriggerClass::ALL.to_vec(),
        reapproval_trigger_tokens: ReapprovalTriggerClass::ALL
            .iter()
            .map(|trigger| trigger.as_str().to_owned())
            .collect(),
        surface_projections: PolicySimulationProjectionSurfaceClass::ALL.to_vec(),
        surface_projection_tokens: PolicySimulationProjectionSurfaceClass::ALL
            .iter()
            .map(|surface| surface.as_str().to_owned())
            .collect(),
    }
}

fn changed_areas(simulation: &PolicySimulationRecord) -> Vec<String> {
    let mut areas = BTreeSet::new();
    areas.insert(simulation.request.change_class_token.clone());
    for surface in &simulation.affected_surfaces {
        areas.insert(surface.protected_path_change_token.clone());
        areas.insert(surface.action_family_token.clone());
        for command_id in &surface.command_ids {
            areas.insert(command_id.clone());
        }
    }
    areas.into_iter().collect()
}

fn simulation_consequence(simulation: &PolicySimulationRecord) -> String {
    simulation
        .affected_surfaces
        .iter()
        .map(|surface| surface.consequence.as_str())
        .collect::<Vec<_>>()
        .join(" ")
}

fn simulation_lineage_refs(
    source_page: &PolicySimulationBetaPage,
    simulation: &PolicySimulationRecord,
) -> Vec<String> {
    let refs: Vec<String> = source_page
        .action_time_policy_states
        .iter()
        .filter(|snapshot| {
            simulation
                .affected_surfaces
                .iter()
                .any(|surface| surface.action_family == snapshot.action_family)
        })
        .flat_map(|snapshot| snapshot.audit_event_refs.iter().cloned())
        .collect();
    if refs.is_empty() {
        vec![format!(
            "audit:policy-simulation:{}",
            simulation.request.simulation_id
        )]
    } else {
        refs
    }
}

fn row_has_all_triggers(row: &ApprovalHistoryRow) -> bool {
    let triggers: BTreeSet<ReapprovalTriggerClass> =
        row.reapproval_triggers.iter().copied().collect();
    ReapprovalTriggerClass::ALL
        .iter()
        .all(|trigger| triggers.contains(trigger))
}

fn is_high_risk_action_family(action_family_token: &str) -> bool {
    matches!(
        action_family_token,
        "connected_provider_mutation"
            | "ai_apply_mutation"
            | "settings_write"
            | "records_lifecycle"
    )
}

fn simulation_view_id(source_id: &str) -> String {
    format!("policy:simulation-view:{source_id}")
}

fn exception_sheet_id(exception_id: &str) -> String {
    format!("policy:exception-preview:{exception_id}")
}

fn approval_history_row_id(remembered_decision_id: &str) -> String {
    format!("policy:approval-history:{remembered_decision_id}")
}

fn diff_summary_id(simulation_id: &str) -> String {
    format!("policy:diff-impact:{simulation_id}")
}

fn expiry_banner_id(subject_id: &str) -> String {
    format!("policy:expiry-banner:{subject_id}")
}

#[allow(dead_code)]
fn drift_reason_to_trigger(reason: RememberedDecisionDriftReason) -> ReapprovalTriggerClass {
    match reason {
        RememberedDecisionDriftReason::ActorChanged
        | RememberedDecisionDriftReason::ObjectChanged
        | RememberedDecisionDriftReason::ActionFamilyChanged
        | RememberedDecisionDriftReason::EnvironmentChanged => ReapprovalTriggerClass::TargetDrift,
        RememberedDecisionDriftReason::PolicyEpochChanged => ReapprovalTriggerClass::PolicyDrift,
        RememberedDecisionDriftReason::TargetVersionChanged
        | RememberedDecisionDriftReason::ExpiryElapsed => ReapprovalTriggerClass::VersionDrift,
        RememberedDecisionDriftReason::AuthorityDrift => ReapprovalTriggerClass::AuthorityDrift,
    }
}

#[allow(dead_code)]
fn memory_requires_reapproval(
    memory_state: MemoryStateClass,
    bucket: DashboardBucketClass,
) -> bool {
    matches!(
        memory_state,
        MemoryStateClass::Expired
            | MemoryStateClass::RequiresReapproval
            | MemoryStateClass::ForceRetiredByPolicy
    ) || bucket == DashboardBucketClass::DriftDetected
}
