//! Customer-visible usage export, budget attribution, and managed or
//! offline-safe reporting for AI lanes.
//!
//! This module materializes the customer-facing AI usage-reporting surface into
//! one export-safe truth packet whose unit of truth is a [`UsageReportRow`]: a
//! single reporting period for one governed AI lane binding the customer-visible
//! usage export that the customer can read, the budget attribution that splits
//! the period's spend across the dimensions that consumed it, and the managed or
//! offline-safe reporting continuity that says whether the report can still be
//! produced when the managed service is unavailable. The packet is the canonical
//! usage-reporting source for shell, docs, support export, and release tooling;
//! consumers project it instead of re-deriving usage, attribution, or continuity
//! state by hand.
//!
//! The packet refuses to present a usage report greener than its export,
//! attribution, and continuity posture can back. A customer-visible export
//! discloses its completeness, and a partial or estimate-only export may not back
//! a Stable claim; an export may not advertise itself as available when the
//! report itself is unavailable. Budget attribution always carries at least one
//! line, no line may attribute more spend than the period's total, a charged
//! total or line discloses who is charged rather than leaving the charge unknown,
//! and an estimate-only attribution may not back a Stable claim. Reporting
//! continuity is honest about offline reach: a lane that claims an offline-safe or
//! offline-fallback continuity must actually be generatable offline, a local lane
//! can never be managed-only, an offline fallback can only have served the report
//! when the lane is offline-capable, and a degraded or unavailable report narrows
//! the claim instead of hiding behind a Stable label. Region and retention are
//! disclosed explicitly so locality never hides behind generic reporting language:
//! a local lane keeps its bytes on-device with no provider retention, and a
//! policy-blocked region or retention posture narrows the claim. Every report
//! carries a closed set of downgrade rules — including the stale-proof and
//! provider-unavailable triggers — that narrow the claim instead of hiding the
//! lane, reusing the qualification, downgrade-trigger, and rollback-posture
//! vocabularies frozen by the M5 AI workflow matrix lane, the mode, quota, and
//! cost-band vocabularies frozen by the routing-policy lane, and the region and
//! retention vocabularies frozen by the provider-route disclosure lane, so no
//! usage report may stay greener than its evidence.
//!
//! Raw provider endpoints, credential bodies, raw provider payloads, exact token
//! counts, exact usage counts, and exact spend amounts stay outside the support
//! boundary; the packet carries modes, families, coarse bands, share classes, and
//! review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json`](../../../../schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes.md`](../../../../docs/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/`](../../../../fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use crate::implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains::{
    ChargedDisclosureClass, CostBandClass, CostMeasurementClass, QuotaFamilyClass,
    RoutePolicyModeClass, ROUTING_POLICY_SCHEMA_REF,
};
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::{
    RouteRegionClass, RouteRetentionClass, PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`UsageReportingPacket`].
pub const USAGE_REPORTING_RECORD_KIND: &str =
    "implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes";

/// Schema version for usage-reporting records.
pub const USAGE_REPORTING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const USAGE_REPORTING_SCHEMA_REF: &str =
    "schemas/ai/implement-customer-visible-usage-export-budget-attribution-and-managed-or-offline-safe-reporting-for-ai-lanes.schema.json";

/// Repo-relative path of the usage-reporting contract doc.
pub const USAGE_REPORTING_DOC_REF: &str =
    "docs/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes.md";

/// Repo-relative path of the protected fixture directory.
pub const USAGE_REPORTING_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes";

/// Repo-relative path of the checked support-export artifact.
pub const USAGE_REPORTING_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const USAGE_REPORTING_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes.md";

/// Whether a usage report can still be produced when the managed service is
/// unavailable.
///
/// Continuity is disclosed explicitly so managed-service dependence never hides
/// behind generic reporting language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportingContinuityClass {
    /// The report can only be produced through the managed service.
    ManagedOnly,
    /// The managed service produces the report, but a verified offline fallback
    /// can produce it when the managed service is unavailable.
    ManagedWithOfflineFallback,
    /// The report can be produced offline without a managed service.
    OfflineSafe,
    /// The report is produced entirely on the local device with no managed
    /// service involved.
    LocalOnly,
}

impl ReportingContinuityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedOnly => "managed_only",
            Self::ManagedWithOfflineFallback => "managed_with_offline_fallback",
            Self::OfflineSafe => "offline_safe",
            Self::LocalOnly => "local_only",
        }
    }

    /// Whether this continuity class promises the report can be produced offline.
    ///
    /// An offline-safe, offline-fallback, or local-only lane must actually be
    /// generatable offline, so each requires the offline-capability flag.
    pub const fn requires_offline_capability(self) -> bool {
        matches!(
            self,
            Self::ManagedWithOfflineFallback | Self::OfflineSafe | Self::LocalOnly
        )
    }

    /// Whether this continuity class depends on the managed service at all.
    pub const fn is_managed_dependent(self) -> bool {
        matches!(self, Self::ManagedOnly | Self::ManagedWithOfflineFallback)
    }
}

/// Generation state of a usage report at packet mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportGenerationStateClass {
    /// The report was produced successfully and is current and complete.
    Generated,
    /// The report was produced but is incomplete or otherwise degraded.
    DegradedPartial,
    /// The managed service was unavailable, so the report was served from the
    /// offline fallback — the continuity feature working as designed.
    ManagedUnavailableUsedOffline,
    /// The managed service was unavailable and no offline fallback exists, so the
    /// report could not be produced.
    ManagedUnavailableNoFallback,
    /// The report has not been generated yet.
    Pending,
}

impl ReportGenerationStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Generated => "generated",
            Self::DegradedPartial => "degraded_partial",
            Self::ManagedUnavailableUsedOffline => "managed_unavailable_used_offline",
            Self::ManagedUnavailableNoFallback => "managed_unavailable_no_fallback",
            Self::Pending => "pending",
        }
    }

    /// Whether the report was served from the offline fallback.
    pub const fn used_offline_fallback(self) -> bool {
        matches!(self, Self::ManagedUnavailableUsedOffline)
    }

    /// Whether the report could not be produced.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::ManagedUnavailableNoFallback)
    }

    /// Whether the report shows a degraded state to the customer.
    ///
    /// A clean offline fallback that served the full report is degraded only in
    /// that it carries an offline-served label; a partial, unavailable, or
    /// pending report is degraded in substance.
    pub const fn is_degraded(self) -> bool {
        matches!(
            self,
            Self::DegradedPartial
                | Self::ManagedUnavailableUsedOffline
                | Self::ManagedUnavailableNoFallback
                | Self::Pending
        )
    }

    /// Whether this generation state can back a Stable claim.
    ///
    /// A successfully generated report or a clean offline fallback that served the
    /// full report backs Stable; a partial, unavailable, or pending report does
    /// not.
    pub const fn backs_stable_claim(self) -> bool {
        matches!(self, Self::Generated | Self::ManagedUnavailableUsedOffline)
    }
}

/// Whether the customer-visible export is available to download.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportAvailabilityClass {
    /// The export is available to download now.
    AvailableNow,
    /// The export is produced on request.
    OnRequest,
    /// The export is unavailable.
    Unavailable,
}

impl ExportAvailabilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableNow => "available_now",
            Self::OnRequest => "on_request",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether the export presents itself as reachable by the customer.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::AvailableNow | Self::OnRequest)
    }
}

/// How complete the customer-visible usage export is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportCompletenessClass {
    /// Every line item for the period is present and measured.
    Complete,
    /// Some line items are missing or degraded.
    PartialDegraded,
    /// The export carries estimates rather than measured figures.
    EstimateOnly,
}

impl ReportCompletenessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::PartialDegraded => "partial_degraded",
            Self::EstimateOnly => "estimate_only",
        }
    }

    /// Whether this completeness class can back a Stable claim.
    pub const fn backs_stable_claim(self) -> bool {
        matches!(self, Self::Complete)
    }
}

/// Redaction posture applied to the customer-visible export.
///
/// The export carries coarse, review-safe figures only; raw token counts and
/// exact spend amounts never cross the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRedactionClass {
    /// Costs are bucketed into coarse bands with no exact amounts.
    CoarseBandsOnly,
    /// Usage is aggregated per dimension with no per-request detail.
    AggregatedPerDimension,
    /// Both coarse bands and per-dimension aggregation are applied.
    CoarseBandsAndAggregated,
}

impl ExportRedactionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoarseBandsOnly => "coarse_bands_only",
            Self::AggregatedPerDimension => "aggregated_per_dimension",
            Self::CoarseBandsAndAggregated => "coarse_bands_and_aggregated",
        }
    }
}

/// The customer-visible usage export for a reporting period.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageExportBlock {
    /// Whether the export is available to the customer.
    pub availability: ExportAvailabilityClass,
    /// Whether the export is presented to the customer (never operator-only).
    pub customer_visible: bool,
    /// How complete the export is.
    pub completeness: ReportCompletenessClass,
    /// Redaction posture keeping raw counts and amounts out of the export.
    pub redaction: ExportRedactionClass,
    /// Review-safe label for the export format offered to the customer.
    pub format_label: String,
    /// Review-safe explanation of the export posture.
    pub explanation_label: String,
}

/// Dimension a slice of period spend is attributed to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionDimensionClass {
    /// Attributed to a workspace.
    PerWorkspace,
    /// Attributed to a user.
    PerUser,
    /// Attributed to a project.
    PerProject,
    /// Attributed to a model.
    PerModel,
    /// Attributed to a provider.
    PerProvider,
    /// Attributed to a feature surface.
    PerFeatureSurface,
    /// Attributed to an interactive session.
    PerSession,
}

impl AttributionDimensionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerWorkspace => "per_workspace",
            Self::PerUser => "per_user",
            Self::PerProject => "per_project",
            Self::PerModel => "per_model",
            Self::PerProvider => "per_provider",
            Self::PerFeatureSurface => "per_feature_surface",
            Self::PerSession => "per_session",
        }
    }
}

/// Coarse share bracket of the period's spend a line accounts for.
///
/// Shares are review-safe brackets; the packet never carries an exact
/// percentage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionShareClass {
    /// A negligible slice of the period's spend.
    Negligible,
    /// A minor slice of the period's spend.
    Minor,
    /// A moderate slice of the period's spend.
    Moderate,
    /// A major slice of the period's spend.
    Major,
    /// The dominant slice of the period's spend.
    Dominant,
}

impl AttributionShareClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Negligible => "negligible",
            Self::Minor => "minor",
            Self::Moderate => "moderate",
            Self::Major => "major",
            Self::Dominant => "dominant",
        }
    }
}

/// One attributed slice of a reporting period's spend.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAttributionLine {
    /// Dimension this slice is attributed to.
    pub dimension: AttributionDimensionClass,
    /// Review-safe label naming the attributed subject (no raw account id).
    pub subject_label: String,
    /// Coarse cost band this slice accounts for.
    pub cost_band: CostBandClass,
    /// Who is charged for this slice.
    pub charged: ChargedDisclosureClass,
    /// Coarse share bracket of the period total this slice accounts for.
    pub share: AttributionShareClass,
    /// Review-safe note for the line.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_label: String,
}

/// Budget attribution splitting a reporting period's spend across dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetAttributionBlock {
    /// Coarse running cost band for the whole reporting period.
    pub total_cost_band: CostBandClass,
    /// Whether the total cost band is measured or merely estimated.
    pub measurement: CostMeasurementClass,
    /// Who is charged for the period's spend.
    pub charged: ChargedDisclosureClass,
    /// Review-safe label naming the billing owner (no raw account id).
    pub billing_owner_label: String,
    /// Attributed slices of the period's spend.
    pub lines: Vec<BudgetAttributionLine>,
    /// Review-safe explanation of the attribution posture.
    pub explanation_label: String,
}

/// One downgrade rule that narrows a report's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageReportDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the report narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One usage report row binding usage-export, budget-attribution, and reporting
/// continuity truth for one reporting period of one governed AI lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageReportRow {
    /// Stable report id.
    pub report_id: String,
    /// Stable id of the governed AI lane the report covers.
    pub lane_id: String,
    /// Human-readable lane label.
    pub lane_label: String,
    /// Review-safe label for the reporting period (no raw dates required).
    pub reporting_period_label: String,
    /// Provider/locality mode the lane resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Quota family that rations the lane.
    pub quota_family: QuotaFamilyClass,
    /// Qualification class claimed for this report.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// How the report can be produced when the managed service is unavailable.
    pub continuity: ReportingContinuityClass,
    /// Whether the report is generatable offline without a managed service.
    pub offline_generatable: bool,
    /// Generation state at mint time.
    pub generation_state: ReportGenerationStateClass,
    /// Region posture disclosed for the lane.
    pub region: RouteRegionClass,
    /// Retention posture disclosed for the lane.
    pub retention: RouteRetentionClass,
    /// Customer-visible usage export.
    pub usage_export: UsageExportBlock,
    /// Budget attribution for the period.
    pub budget_attribution: BudgetAttributionBlock,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<UsageReportDowngradeRule>,
    /// Rollback posture for a reporting-policy change on this lane.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl UsageReportRow {
    /// Whether this report carries a publicly claimed qualification.
    ///
    /// Stable, Beta, and Preview are claimed lanes; Experimental, Held, and
    /// Unavailable are not.
    pub fn is_claimed(&self) -> bool {
        matches!(
            self.claimed_qualification,
            M5AiWorkflowQualificationClass::Stable
                | M5AiWorkflowQualificationClass::Beta
                | M5AiWorkflowQualificationClass::Preview
        )
    }

    /// The attribution line for `dimension`, if present.
    pub fn attribution_line(
        &self,
        dimension: AttributionDimensionClass,
    ) -> Option<&BudgetAttributionLine> {
        self.budget_attribution
            .lines
            .iter()
            .find(|line| line.dimension == dimension)
    }

    /// Whether the report shows any degraded, partial, or unavailable state.
    pub fn has_degraded_state(&self) -> bool {
        self.generation_state.is_degraded()
            || !self.usage_export.completeness.backs_stable_claim()
            || self.usage_export.availability == ExportAvailabilityClass::Unavailable
    }

    /// Whether the report's region or retention posture is policy-blocked.
    pub fn is_policy_blocked(&self) -> bool {
        self.region == RouteRegionClass::PolicyBlocked
            || self.retention == RouteRetentionClass::PolicyBlocked
    }

    /// Qualification this report narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches; this is
    /// the deterministic downgrade automation consumers and release tooling
    /// project instead of re-deriving narrowing locally.
    pub fn narrowed_qualification(
        &self,
        trigger: M5AiWorkflowDowngradeTrigger,
    ) -> M5AiWorkflowQualificationClass {
        self.downgrade_rules
            .iter()
            .find(|rule| rule.trigger == trigger)
            .map(|rule| rule.narrowed_to)
            .unwrap_or(self.claimed_qualification)
    }

    /// Renders a deterministic, review-safe inspector card for this report.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Report `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Lane: `{}` ({})\n",
            self.lane_id, self.lane_label
        ));
        out.push_str(&format!("- Period: {}\n", self.reporting_period_label));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Mode: `{}`\n", self.resolved_mode.as_str()));
        out.push_str(&format!(
            "- Continuity: `{}` (offline generatable: {})\n",
            self.continuity.as_str(),
            self.offline_generatable
        ));
        out.push_str(&format!(
            "- Generation: `{}`\n",
            self.generation_state.as_str()
        ));
        out.push_str(&format!(
            "- Region / retention: `{}` / `{}`\n",
            self.region.as_str(),
            self.retention.as_str()
        ));
        out.push_str(&format!(
            "- Export: `{}` / `{}` / `{}` (customer visible: {}) ({})\n",
            self.usage_export.availability.as_str(),
            self.usage_export.completeness.as_str(),
            self.usage_export.redaction.as_str(),
            self.usage_export.customer_visible,
            self.usage_export.format_label
        ));
        out.push_str(&format!(
            "- Attribution total: `{}` / `{}` / `{}` ({})\n",
            self.budget_attribution.total_cost_band.as_str(),
            self.budget_attribution.measurement.as_str(),
            self.budget_attribution.charged.as_str(),
            self.budget_attribution.billing_owner_label
        ));
        out.push_str("- Attribution lines:\n");
        for line in &self.budget_attribution.lines {
            out.push_str(&format!(
                "  - `{}` / `{}` / `{}` / `{}` ({})\n",
                line.dimension.as_str(),
                line.cost_band.as_str(),
                line.share.as_str(),
                line.charged.as_str(),
                line.subject_label
            ));
        }
        out
    }
}

/// Proof freshness block for the usage-reporting packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageReportingProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed reports.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`UsageReportingPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UsageReportingPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Usage report rows.
    pub reports: Vec<UsageReportRow>,
    /// Proof freshness block.
    pub proof_freshness: UsageReportingProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe customer-visible usage-reporting packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageReportingPacket {
    /// Record kind; must equal [`USAGE_REPORTING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`USAGE_REPORTING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Usage report rows.
    pub reports: Vec<UsageReportRow>,
    /// Proof freshness block.
    pub proof_freshness: UsageReportingProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl UsageReportingPacket {
    /// Builds a usage-reporting packet from stable-lane input.
    pub fn new(input: UsageReportingPacketInput) -> Self {
        Self {
            record_kind: USAGE_REPORTING_RECORD_KIND.to_owned(),
            schema_version: USAGE_REPORTING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            reports: input.reports,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the usage-reporting invariants.
    pub fn validate(&self) -> Vec<UsageReportingViolation> {
        let mut violations = Vec::new();

        if self.record_kind != USAGE_REPORTING_RECORD_KIND {
            violations.push(UsageReportingViolation::WrongRecordKind);
        }
        if self.schema_version != USAGE_REPORTING_SCHEMA_VERSION {
            violations.push(UsageReportingViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(UsageReportingViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_reports_present(self, &mut violations);
        for report in &self.reports {
            validate_report(report, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("usage reporting packet serializes"),
        ) {
            violations.push(UsageReportingViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of reports carrying a publicly claimed qualification.
    pub fn claimed_report_count(&self) -> usize {
        self.reports.iter().filter(|r| r.is_claimed()).count()
    }

    /// Count of reports showing any degraded, partial, or unavailable state.
    pub fn degraded_report_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|r| r.has_degraded_state())
            .count()
    }

    /// Count of reports that remain generatable offline.
    pub fn offline_capable_report_count(&self) -> usize {
        self.reports
            .iter()
            .filter(|r| r.offline_generatable)
            .count()
    }

    /// Returns the report row for `report_id`, if present.
    pub fn report(&self, report_id: &str) -> Option<&UsageReportRow> {
        self.reports.iter().find(|r| r.report_id == report_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("usage reporting packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Customer-Visible Usage Export, Budget Attribution, And Managed Or Offline-Safe Reporting\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Reports: {} ({} claimed, {} degraded, {} offline-capable)\n",
            self.reports.len(),
            self.claimed_report_count(),
            self.degraded_report_count(),
            self.offline_capable_report_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Report inspectors\n\n");
        for report in &self.reports {
            out.push_str(&report.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in usage-reporting export.
#[derive(Debug)]
pub enum UsageReportingArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<UsageReportingViolation>),
}

impl fmt::Display for UsageReportingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "usage reporting export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "usage reporting export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for UsageReportingArtifactError {}

/// Validation failures emitted by [`UsageReportingPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsageReportingViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no reports.
    NoReports,
    /// A report id appears more than once.
    DuplicateReport,
    /// A report row is missing a required identity or label field.
    ReportRowIncomplete,
    /// A budget attribution carries no lines.
    AttributionLinesMissing,
    /// An attribution line attributes more spend than the period total.
    AttributionLineExceedsTotal,
    /// A charged total or line omits its charge-disclosure label.
    AttributionChargeUndisclosed,
    /// A claimed report's attribution is only an estimate.
    EstimatedAttributionClaimsStable,
    /// A customer-visible export is missing its format or explanation label.
    ExportLabelMissing,
    /// An export advertises availability while the report is unavailable.
    ExportAvailableButReportUnavailable,
    /// A claimed report's export is partial or estimate-only.
    IncompleteExportClaimsStable,
    /// An offline-capable continuity class is not generatable offline.
    OfflineContinuityNotGeneratable,
    /// A local-mode report claims a managed-only continuity.
    LocalRouteManagedOnly,
    /// An offline fallback served the report on a lane with no offline capability.
    OfflineFallbackWithoutCapability,
    /// A degraded or unavailable report still claims Stable.
    DegradedReportClaimsStable,
    /// A local-mode report does not keep its bytes on-device with no retention.
    LocalRouteRegionRetentionMismatch,
    /// An off-device report claims an on-device region.
    RegionLocalityMismatch,
    /// A policy-blocked region or retention posture still claims Stable.
    PolicyBlockedClaimsStable,
    /// A claimed report is missing required evidence packet refs.
    ClaimedReportMissingEvidence,
    /// A claimed report's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A report has no downgrade rules.
    DowngradeRulesMissing,
    /// A report's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A report's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl UsageReportingViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoReports => "no_reports",
            Self::DuplicateReport => "duplicate_report",
            Self::ReportRowIncomplete => "report_row_incomplete",
            Self::AttributionLinesMissing => "attribution_lines_missing",
            Self::AttributionLineExceedsTotal => "attribution_line_exceeds_total",
            Self::AttributionChargeUndisclosed => "attribution_charge_undisclosed",
            Self::EstimatedAttributionClaimsStable => "estimated_attribution_claims_stable",
            Self::ExportLabelMissing => "export_label_missing",
            Self::ExportAvailableButReportUnavailable => "export_available_but_report_unavailable",
            Self::IncompleteExportClaimsStable => "incomplete_export_claims_stable",
            Self::OfflineContinuityNotGeneratable => "offline_continuity_not_generatable",
            Self::LocalRouteManagedOnly => "local_route_managed_only",
            Self::OfflineFallbackWithoutCapability => "offline_fallback_without_capability",
            Self::DegradedReportClaimsStable => "degraded_report_claims_stable",
            Self::LocalRouteRegionRetentionMismatch => "local_route_region_retention_mismatch",
            Self::RegionLocalityMismatch => "region_locality_mismatch",
            Self::PolicyBlockedClaimsStable => "policy_blocked_claims_stable",
            Self::ClaimedReportMissingEvidence => "claimed_report_missing_evidence",
            Self::ClaimedRollbackUnverified => "claimed_rollback_unverified",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleMissingProviderUnavailable => {
                "downgrade_rule_missing_provider_unavailable"
            }
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in usage-reporting export.
pub fn current_usage_reporting_export() -> Result<UsageReportingPacket, UsageReportingArtifactError>
{
    let packet: UsageReportingPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_customer_visible_usage_export_budget_attribution_and_managed_or_offline_safe_reporting_for_ai_lanes/support_export.json"
    )))
    .map_err(UsageReportingArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(UsageReportingArtifactError::Validation(violations))
    }
}

/// Ordinal rank used to compare qualification severity for downgrade rules.
///
/// Higher means a stronger public claim, so a downgrade must move to a strictly
/// lower rank.
fn qualification_rank(class: M5AiWorkflowQualificationClass) -> u8 {
    match class {
        M5AiWorkflowQualificationClass::Unavailable => 0,
        M5AiWorkflowQualificationClass::Held => 1,
        M5AiWorkflowQualificationClass::Experimental => 2,
        M5AiWorkflowQualificationClass::Preview => 3,
        M5AiWorkflowQualificationClass::Beta => 4,
        M5AiWorkflowQualificationClass::Stable => 5,
    }
}

/// Ordinal rank used to assert no attributed slice exceeds the period total.
///
/// Higher means a more expensive coarse band; an estimate band ranks above every
/// measured band so an unverified estimate never reads as cheaper than a measured
/// figure.
fn cost_band_rank(band: CostBandClass) -> u8 {
    match band {
        CostBandClass::BundledNoIncrementalCost => 0,
        CostBandClass::FreeTierRateLimited => 1,
        CostBandClass::FlatFeeSubscriptionBand => 2,
        CostBandClass::MeteredLowBand => 3,
        CostBandClass::MeteredMediumBand => 4,
        CostBandClass::MeteredHighBand => 5,
        CostBandClass::EstimatedUnverifiedBand => 6,
    }
}

fn validate_source_contracts(
    packet: &UsageReportingPacket,
    violations: &mut Vec<UsageReportingViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        USAGE_REPORTING_SCHEMA_REF,
        USAGE_REPORTING_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        ROUTING_POLICY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(UsageReportingViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_reports_present(
    packet: &UsageReportingPacket,
    violations: &mut Vec<UsageReportingViolation>,
) {
    if packet.reports.is_empty() {
        violations.push(UsageReportingViolation::NoReports);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for report in &packet.reports {
        if !seen.insert(report.report_id.as_str()) {
            violations.push(UsageReportingViolation::DuplicateReport);
        }
    }
}

fn validate_report(report: &UsageReportRow, violations: &mut Vec<UsageReportingViolation>) {
    if report.report_id.trim().is_empty()
        || report.lane_id.trim().is_empty()
        || report.lane_label.trim().is_empty()
        || report.reporting_period_label.trim().is_empty()
        || report
            .budget_attribution
            .billing_owner_label
            .trim()
            .is_empty()
        || report
            .budget_attribution
            .explanation_label
            .trim()
            .is_empty()
        || report.usage_export.explanation_label.trim().is_empty()
    {
        violations.push(UsageReportingViolation::ReportRowIncomplete);
    }

    validate_export(report, violations);
    validate_attribution(report, violations);
    validate_continuity(report, violations);
    validate_locality(report, violations);

    if report.is_claimed() && report.evidence_packet_refs.is_empty() {
        violations.push(UsageReportingViolation::ClaimedReportMissingEvidence);
    }

    // A claimed report whose reporting-policy change can be reversed must have
    // drilled that reversal; a non-applicable posture carries no reversal to
    // verify.
    if report.is_claimed()
        && report.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !report.rollback_verified
    {
        violations.push(UsageReportingViolation::ClaimedRollbackUnverified);
    }

    validate_downgrade_rules(report, violations);
}

fn validate_export(report: &UsageReportRow, violations: &mut Vec<UsageReportingViolation>) {
    let export = &report.usage_export;

    // A customer-visible export must name its format so the customer knows what
    // they receive.
    if export.customer_visible && export.format_label.trim().is_empty() {
        violations.push(UsageReportingViolation::ExportLabelMissing);
    }

    // An export may not advertise itself as reachable while the report could not
    // be produced.
    if export.availability.is_reachable() && report.generation_state.is_unavailable() {
        violations.push(UsageReportingViolation::ExportAvailableButReportUnavailable);
    }

    // A partial or estimate-only export may not back a Stable claim.
    if report.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && !export.completeness.backs_stable_claim()
    {
        violations.push(UsageReportingViolation::IncompleteExportClaimsStable);
    }
}

fn validate_attribution(report: &UsageReportRow, violations: &mut Vec<UsageReportingViolation>) {
    let attribution = &report.budget_attribution;

    // Every period must split its spend across at least one attributed slice.
    if attribution.lines.is_empty() {
        violations.push(UsageReportingViolation::AttributionLinesMissing);
    }

    let total_rank = cost_band_rank(attribution.total_cost_band);
    for line in &attribution.lines {
        // No attributed slice can cost more than the period total.
        if cost_band_rank(line.cost_band) > total_rank {
            violations.push(UsageReportingViolation::AttributionLineExceedsTotal);
            break;
        }
    }

    // A charged total, or any charged line, must say who is charged.
    let charge_undisclosed = (attribution.total_cost_band.is_charged()
        && !attribution.charged.is_disclosed())
        || attribution
            .lines
            .iter()
            .any(|line| line.cost_band.is_charged() && !line.charged.is_disclosed());
    if charge_undisclosed {
        violations.push(UsageReportingViolation::AttributionChargeUndisclosed);
    }

    // An estimate-only attribution may not back a Stable claim.
    if report.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && (attribution.total_cost_band == CostBandClass::EstimatedUnverifiedBand
            || attribution.measurement == CostMeasurementClass::EstimateBand)
    {
        violations.push(UsageReportingViolation::EstimatedAttributionClaimsStable);
    }
}

fn validate_continuity(report: &UsageReportRow, violations: &mut Vec<UsageReportingViolation>) {
    // A continuity class that promises offline reach must actually be generatable
    // offline.
    if report.continuity.requires_offline_capability() && !report.offline_generatable {
        violations.push(UsageReportingViolation::OfflineContinuityNotGeneratable);
    }

    // A local lane can always run offline, so it can never be managed-only.
    if report.resolved_mode == RoutePolicyModeClass::Local
        && report.continuity == ReportingContinuityClass::ManagedOnly
    {
        violations.push(UsageReportingViolation::LocalRouteManagedOnly);
    }

    // An offline fallback can only have served the report on a lane that is
    // actually offline-capable.
    if report.generation_state.used_offline_fallback()
        && (!report.offline_generatable || !report.continuity.requires_offline_capability())
    {
        violations.push(UsageReportingViolation::OfflineFallbackWithoutCapability);
    }

    // A degraded or unavailable report narrows the claim instead of claiming
    // Stable.
    if report.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && !report.generation_state.backs_stable_claim()
    {
        violations.push(UsageReportingViolation::DegradedReportClaimsStable);
    }
}

fn validate_locality(report: &UsageReportRow, violations: &mut Vec<UsageReportingViolation>) {
    // A local lane keeps its bytes on-device with no provider retention.
    if report.resolved_mode == RoutePolicyModeClass::Local
        && (report.region != RouteRegionClass::OnDeviceOnly
            || report.retention != RouteRetentionClass::NoRetentionLocalOnly)
    {
        violations.push(UsageReportingViolation::LocalRouteRegionRetentionMismatch);
    }

    // An off-device lane may not claim its bytes stay on-device.
    if report.resolved_mode.is_egress() && report.region == RouteRegionClass::OnDeviceOnly {
        violations.push(UsageReportingViolation::RegionLocalityMismatch);
    }

    // A policy-blocked region or retention posture narrows the claim.
    if report.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && report.is_policy_blocked()
    {
        violations.push(UsageReportingViolation::PolicyBlockedClaimsStable);
    }
}

fn validate_downgrade_rules(
    report: &UsageReportRow,
    violations: &mut Vec<UsageReportingViolation>,
) {
    if report.downgrade_rules.is_empty() {
        violations.push(UsageReportingViolation::DowngradeRulesMissing);
        return;
    }

    if !report
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(UsageReportingViolation::DowngradeRuleMissingProofStale);
    }

    // A managed-service outage narrows through the provider-unavailable trigger,
    // so every report must carry it.
    if !report
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(UsageReportingViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(report.claimed_qualification);
    for rule in &report.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(UsageReportingViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &UsageReportingPacket,
    violations: &mut Vec<UsageReportingViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(UsageReportingViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("https://")
                || lower.contains("http://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
