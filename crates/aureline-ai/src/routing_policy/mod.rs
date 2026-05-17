//! Beta cost-routing policy, spend receipts, and claimed-surface export.
//!
//! This module joins the provider/model registry, graduation state, routing
//! packet, and spend-receipt contract into one headless support packet. It is
//! metadata-only: records carry opaque refs, coarse cost bands, quota families,
//! budget owner refs, and policy-disclosure refs, never raw prices, raw token
//! counts, provider payloads, endpoint URLs, credentials, or billing-account ids.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::evidence::RouteSpendLineage;
use crate::graduation::{current_beta_graduation_state, AiGraduationState};
use crate::registry::{
    ProviderModelRegistryPacket, ProviderModelRegistryViolation, RegistryRouteCandidate,
    RegistryRoutingPolicyClass, RouteEligibilityClass,
};
use crate::routing::{
    AiRoutingPacket, CostEnvelopeClass, CostVisibilityClass, ExecutionLocusClass, QuotaFamilyClass,
    RouteOriginClass, RoutingRunStateClass,
};

/// Stable record-kind tag carried by [`CostRoutingBetaPacket`].
pub const COST_ROUTING_BETA_PACKET_RECORD_KIND: &str = "cost_routing_beta_packet";

/// Stable record-kind tag carried by [`SpendReceiptRecord`].
pub const SPEND_RECEIPT_RECORD_KIND: &str = "spend_receipt_record";

/// Schema version for the cost-routing beta packet.
pub const COST_ROUTING_BETA_SCHEMA_VERSION: u32 = 1;

/// Schema version re-exported from `/schemas/ai/spend_receipt.schema.json`.
pub const SPEND_RECEIPT_SCHEMA_VERSION: u32 = 1;

const REQUIRED_SPEND_ATTRIBUTION_DIMENSIONS: &[SpendAttributionDimensionClass] = &[
    SpendAttributionDimensionClass::WorkflowOrSurfaceIdDimension,
    SpendAttributionDimensionClass::ProviderEntryIdDimension,
    SpendAttributionDimensionClass::ModelEntryIdDimension,
    SpendAttributionDimensionClass::ExecutionLocusClassDimension,
    SpendAttributionDimensionClass::RegionPostureClassDimension,
    SpendAttributionDimensionClass::RetentionStanceClassDimension,
    SpendAttributionDimensionClass::QuotaFamilyClassDimension,
];

const REQUIRED_BUDGET_SCOPE_OUTCOMES: &[BudgetScopeClass] = &[
    BudgetScopeClass::PerRequest,
    BudgetScopeClass::PerSession,
    BudgetScopeClass::PerAgentInvocation,
];

/// Spend-attribution dimension recorded on an AI spend receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpendAttributionDimensionClass {
    /// Workflow or surface id dimension.
    WorkflowOrSurfaceIdDimension,
    /// AI feature class dimension.
    FeatureClassDimension,
    /// Provider registry entry id dimension.
    ProviderEntryIdDimension,
    /// Model registry entry id dimension.
    ModelEntryIdDimension,
    /// Execution locus dimension.
    ExecutionLocusClassDimension,
    /// Region posture dimension.
    RegionPostureClassDimension,
    /// Retention stance dimension.
    RetentionStanceClassDimension,
    /// Quota family dimension.
    QuotaFamilyClassDimension,
    /// Deployment profile dimension.
    DeploymentProfileClassDimension,
    /// Policy epoch dimension.
    PolicyEpochDimension,
    /// Branch-agent chain dimension.
    AgentInvocationChainIdDimension,
    /// Session id dimension.
    SessionIdDimension,
    /// Command invocation dimension.
    CommandInvocationIdDimension,
}

impl SpendAttributionDimensionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkflowOrSurfaceIdDimension => "workflow_or_surface_id_dimension",
            Self::FeatureClassDimension => "feature_class_dimension",
            Self::ProviderEntryIdDimension => "provider_entry_id_dimension",
            Self::ModelEntryIdDimension => "model_entry_id_dimension",
            Self::ExecutionLocusClassDimension => "execution_locus_class_dimension",
            Self::RegionPostureClassDimension => "region_posture_class_dimension",
            Self::RetentionStanceClassDimension => "retention_stance_class_dimension",
            Self::QuotaFamilyClassDimension => "quota_family_class_dimension",
            Self::DeploymentProfileClassDimension => "deployment_profile_class_dimension",
            Self::PolicyEpochDimension => "policy_epoch_dimension",
            Self::AgentInvocationChainIdDimension => "agent_invocation_chain_id_dimension",
            Self::SessionIdDimension => "session_id_dimension",
            Self::CommandInvocationIdDimension => "command_invocation_id_dimension",
        }
    }
}

/// Budget scope recorded on a spend receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetScopeClass {
    /// Per-request budget scope.
    PerRequest,
    /// Per-session budget scope.
    PerSession,
    /// Per-agent-invocation budget scope.
    PerAgentInvocation,
    /// Per-workflow budget scope.
    PerWorkflow,
    /// Per-user budget scope.
    PerUser,
    /// Per-organisation budget scope.
    PerOrganisation,
    /// Per-deployment-profile budget scope.
    PerDeploymentProfile,
    /// Per-policy-bundle budget scope.
    PerPolicyBundle,
}

impl BudgetScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerRequest => "per_request",
            Self::PerSession => "per_session",
            Self::PerAgentInvocation => "per_agent_invocation",
            Self::PerWorkflow => "per_workflow",
            Self::PerUser => "per_user",
            Self::PerOrganisation => "per_organisation",
            Self::PerDeploymentProfile => "per_deployment_profile",
            Self::PerPolicyBundle => "per_policy_bundle",
        }
    }
}

/// Budget-scope outcome recorded on a spend receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetScopeOutcomeClass {
    /// Scope landed under the published band.
    ScopeUnderBand,
    /// Scope landed at the published band.
    ScopeAtBand,
    /// Scope exceeded the band and dispatch was blocked.
    ScopeOverBandBlocked,
    /// Scope was not applicable for this run.
    ScopeNotApplicable,
    /// Outcome is unknown before dispatch.
    ScopeOutcomeUnknownPreDispatch,
}

impl BudgetScopeOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScopeUnderBand => "scope_under_band",
            Self::ScopeAtBand => "scope_at_band",
            Self::ScopeOverBandBlocked => "scope_over_band_blocked",
            Self::ScopeNotApplicable => "scope_not_applicable",
            Self::ScopeOutcomeUnknownPreDispatch => "scope_outcome_unknown_pre_dispatch",
        }
    }
}

/// Charge-locus class recorded on a spend receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WasChargedToUserClass {
    /// Local route with no user charge.
    NotChargedLocal,
    /// Bundled route with no per-call charge.
    NotChargedBundled,
    /// User BYOK metered route.
    ChargedUserByokMetered,
    /// User BYOK subscription route.
    ChargedUserByokSubscription,
    /// Organisation pooled route.
    ChargedOrganisationPooled,
    /// Organisation subscription route.
    ChargedOrganisationSubscription,
    /// Charge locus is not verified.
    ChargeUnknownUnverified,
    /// Run was blocked or cancelled before a charge accrued.
    NotChargedRunBlocked,
}

impl WasChargedToUserClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotChargedLocal => "not_charged_local",
            Self::NotChargedBundled => "not_charged_bundled",
            Self::ChargedUserByokMetered => "charged_user_byok_metered",
            Self::ChargedUserByokSubscription => "charged_user_byok_subscription",
            Self::ChargedOrganisationPooled => "charged_organisation_pooled",
            Self::ChargedOrganisationSubscription => "charged_organisation_subscription",
            Self::ChargeUnknownUnverified => "charge_unknown_unverified",
            Self::NotChargedRunBlocked => "not_charged_run_blocked",
        }
    }
}

/// Redaction class carried by spend and cost-routing exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default projection.
    MetadataSafeDefault,
    /// Operator-only restricted projection.
    OperatorOnlyRestricted,
    /// Internal-support restricted projection.
    InternalSupportRestricted,
    /// Signing-evidence-only projection.
    SigningEvidenceOnly,
}

impl RedactionClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Policy context copied onto a spend receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendPolicyContext {
    /// Policy epoch that admitted the route.
    pub policy_epoch: String,
    /// Workspace trust-state token.
    pub trust_state: String,
    /// Deployment-profile token.
    #[serde(default)]
    pub deployment_profile_class: String,
    /// Execution-context id, when the route was planned from one.
    #[serde(default)]
    pub execution_context_id: String,
}

/// One spend-attribution value row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendAttributionValueRow {
    /// Dimension this value satisfies.
    pub dimension_class: SpendAttributionDimensionClass,
    /// Opaque ref or class token for the dimension value.
    pub value_ref: String,
    /// Review-safe explanation for the value.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_summary: String,
}

/// One budget-scope outcome row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BudgetScopeOutcomeRow {
    /// Budget scope covered by this outcome.
    pub budget_scope_class: BudgetScopeClass,
    /// Outcome class for the scope.
    pub budget_scope_outcome_class: BudgetScopeOutcomeClass,
    /// Cost band applied to this scope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Review-safe explanation for the scope outcome.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_summary: String,
}

/// Metadata-only spend receipt for one AI invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpendReceiptRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub spend_receipt_schema_version: u32,
    /// Stable spend-receipt id.
    pub spend_receipt_id: String,
    /// Workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI and support exports.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub display_label: String,
    /// Paired provider-route receipt ref.
    pub route_receipt_ref: String,
    /// Context assembly ref.
    pub assembly_id_ref: String,
    /// Run-state class mirrored from the route receipt.
    pub run_state_class: RoutingRunStateClass,
    /// Coarse cost envelope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Cost visibility posture.
    pub cost_visibility_class: CostVisibilityClass,
    /// Quota family.
    pub quota_family_class: QuotaFamilyClass,
    /// Spend-attribution dimensions emitted by this receipt.
    #[serde(default)]
    pub spend_attribution_dimensions: Vec<SpendAttributionDimensionClass>,
    /// Spend-attribution values emitted by this receipt.
    #[serde(default)]
    pub spend_attribution_values: Vec<SpendAttributionValueRow>,
    /// Budget-scope outcomes emitted by this receipt.
    #[serde(default)]
    pub budget_scope_outcomes: Vec<BudgetScopeOutcomeRow>,
    /// Charge-locus class.
    pub was_charged_to_user_class: WasChargedToUserClass,
    /// Budget-routing policy ref that admitted the run.
    pub originating_budget_routing_policy_ref: String,
    /// Graduation packet ref that admitted the run.
    #[serde(default)]
    pub originating_graduation_packet_ref: String,
    /// Route-selection disclosure ref for non-cheapest or fallback routes.
    #[serde(default)]
    pub originating_route_selection_disclosure_ref: String,
    /// Approval ticket ref that admitted a route override, when present.
    #[serde(default)]
    pub originating_approval_ticket_ref: String,
    /// Branch-agent chain id, when the run belongs to one.
    #[serde(default)]
    pub branch_agent_chain_id: String,
    /// Branch-agent hop index, when the run belongs to one.
    #[serde(default)]
    pub branch_agent_hop_index: Option<u32>,
    /// Opaque provider cost-unit ref, when metering produced one.
    #[serde(default)]
    pub opaque_provider_cost_unit_ref: String,
    /// Superseded spend receipt refs.
    #[serde(default)]
    pub supersedes_spend_receipt_refs: Vec<String>,
    /// Policy context that admitted the receipt.
    pub policy_context: SpendPolicyContext,
    /// Redaction class for this receipt.
    pub redaction_class: RedactionClass,
    /// Review-safe receipt summary.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub notes_summary: String,
    /// Timestamp the receipt was minted.
    pub minted_at: String,
    /// Timestamp the receipt was last updated.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub last_updated_at: String,
}

impl SpendReceiptRecord {
    /// Builds a metadata-only spend receipt from a selected routing packet.
    pub fn from_routing_packet(
        routing_packet: &AiRoutingPacket,
        spend_receipt_id: impl Into<String>,
        route_receipt_ref: impl Into<String>,
        assembly_id_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let selected = routing_packet.selected_route();
        let minted_at = minted_at.into();
        let cost_envelope_class = selected
            .map(|candidate| candidate.envelope.cost_envelope_class)
            .unwrap_or(CostEnvelopeClass::EnvelopeUnknownUnverifiedCost);
        let cost_visibility_class = selected
            .map(|candidate| candidate.envelope.cost_visibility_class)
            .unwrap_or(CostVisibilityClass::EstimatedUnverified);
        let quota_family_class = selected
            .map(|candidate| candidate.quota.quota_family_class)
            .unwrap_or(QuotaFamilyClass::QuotaUnknownUnverified);
        let was_charged_to_user_class = selected
            .map(|candidate| {
                charge_locus_for(
                    routing_packet.run_state_class,
                    candidate.execution_locus_class,
                    candidate.route_origin_class,
                    candidate.envelope.cost_visibility_class,
                    candidate.quota.quota_family_class,
                )
            })
            .unwrap_or(WasChargedToUserClass::NotChargedRunBlocked);
        let route_selection_disclosure_ref = selected
            .and_then(|candidate| candidate.route_selection_disclosure_ref.clone())
            .unwrap_or_default();

        Self {
            record_kind: SPEND_RECEIPT_RECORD_KIND.to_owned(),
            spend_receipt_schema_version: SPEND_RECEIPT_SCHEMA_VERSION,
            spend_receipt_id: spend_receipt_id.into(),
            workflow_or_surface_id: routing_packet.workflow_or_surface_id.clone(),
            display_label: format!(
                "Spend receipt for {}",
                routing_packet.workflow_or_surface_id
            ),
            route_receipt_ref: route_receipt_ref.into(),
            assembly_id_ref: assembly_id_ref.into(),
            run_state_class: routing_packet.run_state_class,
            cost_envelope_class,
            cost_visibility_class,
            quota_family_class,
            spend_attribution_dimensions: spend_attribution_dimensions(),
            spend_attribution_values: selected
                .map(|candidate| spend_attribution_values(routing_packet, candidate))
                .unwrap_or_default(),
            budget_scope_outcomes: default_budget_scope_outcomes(
                routing_packet.run_state_class,
                cost_envelope_class,
            ),
            was_charged_to_user_class,
            originating_budget_routing_policy_ref: selected
                .map(|candidate| candidate.envelope.budget_routing_policy_ref.clone())
                .unwrap_or_default(),
            originating_graduation_packet_ref: selected
                .map(|candidate| candidate.envelope.graduation_packet_ref.clone())
                .unwrap_or_default(),
            originating_route_selection_disclosure_ref: route_selection_disclosure_ref,
            originating_approval_ticket_ref: selected
                .and_then(|candidate| candidate.originating_approval_ticket_ref.clone())
                .unwrap_or_default(),
            branch_agent_chain_id: String::new(),
            branch_agent_hop_index: None,
            opaque_provider_cost_unit_ref: String::new(),
            supersedes_spend_receipt_refs: Vec::new(),
            policy_context: SpendPolicyContext {
                policy_epoch: routing_packet.policy_context.policy_epoch_ref.clone(),
                trust_state: routing_packet
                    .policy_context
                    .trust_state
                    .as_str()
                    .to_owned(),
                deployment_profile_class: routing_packet
                    .policy_context
                    .deployment_profile_class
                    .as_str()
                    .to_owned(),
                execution_context_id: routing_packet
                    .policy_context
                    .execution_context_ref
                    .clone()
                    .unwrap_or_default(),
            },
            redaction_class: RedactionClass::MetadataSafeDefault,
            notes_summary: selected
                .map(|candidate| {
                    format!(
                        "{} cost band with {} charge posture.",
                        candidate.envelope.cost_envelope_class.as_str(),
                        was_charged_to_user_class.as_str()
                    )
                })
                .unwrap_or_else(|| "No selected route was available for spend attribution.".into()),
            minted_at: minted_at.clone(),
            last_updated_at: minted_at,
        }
    }

    /// Validates this receipt against the spend-receipt boundary contract.
    pub fn validate(&self) -> Vec<CostRoutingBetaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != SPEND_RECEIPT_RECORD_KIND {
            violations.push(CostRoutingBetaViolation::SpendReceiptWrongRecordKind);
        }
        if self.spend_receipt_schema_version != SPEND_RECEIPT_SCHEMA_VERSION {
            violations.push(CostRoutingBetaViolation::SpendReceiptWrongSchemaVersion);
        }
        if self.spend_receipt_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.route_receipt_ref.trim().is_empty()
            || self.assembly_id_ref.trim().is_empty()
            || self.originating_budget_routing_policy_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CostRoutingBetaViolation::SpendReceiptMissingIdentity);
        }
        if !self.has_required_spend_attribution_dimensions() {
            violations.push(CostRoutingBetaViolation::SpendReceiptMissingAttributionDimensions);
        }
        if !self.has_required_budget_scope_outcomes() {
            violations.push(CostRoutingBetaViolation::SpendReceiptMissingBudgetScopeOutcomes);
        }
        if run_state_blocks_or_cancels(self.run_state_class)
            && self.was_charged_to_user_class != WasChargedToUserClass::NotChargedRunBlocked
        {
            violations.push(CostRoutingBetaViolation::SpendReceiptChargeLocusMismatch);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("spend receipt serializes"),
        ) {
            violations.push(CostRoutingBetaViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Validates this spend receipt against the routing packet it cites.
    pub fn validate_against_routing_packet(
        &self,
        routing_packet: &AiRoutingPacket,
    ) -> Vec<CostRoutingBetaViolation> {
        let mut violations = self.validate();
        let Some(selected) = routing_packet.selected_route() else {
            violations.push(CostRoutingBetaViolation::SpendReceiptRouteMismatch);
            return violations;
        };

        if self.workflow_or_surface_id != routing_packet.workflow_or_surface_id
            || self.run_state_class != routing_packet.run_state_class
            || self.cost_envelope_class != selected.envelope.cost_envelope_class
            || self.cost_visibility_class != selected.envelope.cost_visibility_class
            || self.quota_family_class != selected.quota.quota_family_class
            || self.originating_budget_routing_policy_ref
                != selected.envelope.budget_routing_policy_ref
            || self.originating_graduation_packet_ref != selected.envelope.graduation_packet_ref
        {
            violations.push(CostRoutingBetaViolation::SpendReceiptRouteMismatch);
        }

        let selected_disclosure = selected
            .route_selection_disclosure_ref
            .as_deref()
            .unwrap_or_default();
        if selected
            .route_selection_reason_class
            .requires_route_change_lineage()
            && self
                .originating_route_selection_disclosure_ref
                .trim()
                .is_empty()
        {
            violations.push(CostRoutingBetaViolation::PolicyLimitedRouteMissingDisclosure);
        }
        if self.originating_route_selection_disclosure_ref != selected_disclosure {
            violations.push(CostRoutingBetaViolation::SpendReceiptRouteMismatch);
        }

        let value_dimensions = self
            .spend_attribution_values
            .iter()
            .map(|row| row.dimension_class)
            .collect::<BTreeSet<_>>();
        for required in REQUIRED_SPEND_ATTRIBUTION_DIMENSIONS {
            if !value_dimensions.contains(required) {
                violations.push(CostRoutingBetaViolation::SpendReceiptMissingAttributionDimensions);
                break;
            }
        }

        violations
    }

    /// Deterministic export-safe JSON for support bundles.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only receipt fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("spend receipt serializes")
    }

    fn has_required_spend_attribution_dimensions(&self) -> bool {
        let dimensions = self
            .spend_attribution_dimensions
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        REQUIRED_SPEND_ATTRIBUTION_DIMENSIONS
            .iter()
            .all(|required| dimensions.contains(required))
    }

    fn has_required_budget_scope_outcomes(&self) -> bool {
        let scopes = self
            .budget_scope_outcomes
            .iter()
            .map(|row| row.budget_scope_class)
            .collect::<BTreeSet<_>>();
        REQUIRED_BUDGET_SCOPE_OUTCOMES
            .iter()
            .all(|required| scopes.contains(required))
    }
}

/// One claimed beta surface's cost-routing and spend-receipt readout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostRoutingSurfaceRow {
    /// Stable claimed-surface id.
    pub surface_id: String,
    /// Display label for the claimed surface.
    pub display_label: String,
    /// Provider/model registry state ref.
    pub registry_state_ref: String,
    /// Graduation state ref used for promotion gating.
    pub graduation_state_ref: String,
    /// Route policy ref that selected the route.
    pub route_policy_id_ref: String,
    /// Route policy class token.
    pub policy_class_token: String,
    /// Route reason token.
    pub route_reason_token: String,
    /// Route override reason token.
    pub route_selection_override_reason_token: String,
    /// Selected provider entry ref.
    pub selected_provider_entry_ref: String,
    /// Selected model entry ref.
    pub selected_model_entry_ref: String,
    /// Selected execution-locus token.
    pub selected_execution_locus_token: String,
    /// Selected route-origin token.
    pub selected_route_origin_token: String,
    /// Selected cost-envelope token.
    pub selected_cost_envelope_token: String,
    /// Selected cost-visibility token.
    pub selected_cost_visibility_token: String,
    /// Selected quota-family token.
    pub selected_quota_family_token: String,
    /// Budget owner ref disclosed for the selected route.
    pub budget_owner_ref: String,
    /// Budget-routing policy ref disclosed for the selected route.
    pub budget_routing_policy_ref: String,
    /// Provider-route receipt ref paired with the spend receipt.
    pub route_receipt_ref: String,
    /// Spend receipt ref emitted for this claimed row.
    pub spend_receipt_ref: String,
    /// Cheapest qualifying candidate provider ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cheapest_candidate_provider_entry_ref: Option<String>,
    /// Cheapest qualifying candidate model ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cheapest_candidate_model_entry_ref: Option<String>,
    /// Cheapest qualifying candidate cost-envelope token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cheapest_candidate_cost_envelope_token: Option<String>,
    /// Whether the selected route is the cheapest qualifying candidate.
    pub selected_is_cheapest_qualifying: bool,
    /// Route-selection disclosure ref required for policy-limited or fallback paths.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_selection_disclosure_ref: Option<String>,
    /// Graduation promotion-gate token.
    pub promotion_gate_token: String,
    /// Graduation packet freshness token.
    pub packet_freshness_token: String,
    /// Effective support-class token from graduation state.
    pub effective_support_class_token: String,
    /// Local continuity label for non-AI fallback.
    pub local_continuity_label: String,
}

/// Export packet joining claimed-route decisions and spend receipts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CostRoutingBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable cost-routing packet id.
    pub cost_routing_packet_id: String,
    /// Registry state read by this packet.
    pub registry_state_ref: String,
    /// Graduation state read by this packet.
    pub graduation_state_ref: String,
    /// Timestamp used for freshness and packet projection.
    pub as_of: String,
    /// Claimed-surface route rows.
    #[serde(default)]
    pub surface_rows: Vec<CostRoutingSurfaceRow>,
    /// Spend receipts emitted for claimed-surface rows.
    #[serde(default)]
    pub spend_receipts: Vec<SpendReceiptRecord>,
    /// Route/spend lineage rows suitable for evidence-packet handoff.
    #[serde(default)]
    pub evidence_lineage_rows: Vec<RouteSpendLineage>,
    /// Source contracts consumed by this packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Timestamp the packet was minted.
    pub minted_at: String,
}

impl CostRoutingBetaPacket {
    /// Builds a cost-routing packet from registry and graduation state.
    ///
    /// # Errors
    ///
    /// Returns a registry violation when a claimed surface cannot resolve to an
    /// admitted route.
    pub fn from_registry_and_graduation(
        registry: &ProviderModelRegistryPacket,
        graduation_state: &AiGraduationState,
        cost_routing_packet_id: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Result<Self, ProviderModelRegistryViolation> {
        let minted_at = minted_at.into();
        let mut surface_rows = Vec::new();
        let mut spend_receipts = Vec::new();
        let mut evidence_lineage_rows = Vec::new();

        for surface in &registry.claimed_surfaces {
            let routing_packet_id = format!(
                "routing-packet:{}:cost-routing-beta",
                receipt_safe_fragment(&surface.surface_id)
            );
            let route_receipt_ref = format!(
                "route-receipt:{}:cost-routing-beta",
                receipt_safe_fragment(&surface.surface_id)
            );
            let spend_receipt_id = format!(
                "spend-receipt:{}:cost-routing-beta",
                receipt_safe_fragment(&surface.surface_id)
            );
            let assembly_id_ref = format!(
                "assembly:{}:cost-routing-beta",
                receipt_safe_fragment(&surface.surface_id)
            );
            let routing_packet = registry.routing_packet_for_surface(
                &surface.surface_id,
                routing_packet_id,
                format!(
                    "request-workspace:{}:cost-routing-beta",
                    receipt_safe_fragment(&surface.surface_id)
                ),
                &minted_at,
            )?;
            let spend_receipt = SpendReceiptRecord::from_routing_packet(
                &routing_packet,
                spend_receipt_id,
                &route_receipt_ref,
                assembly_id_ref,
                &minted_at,
            );
            let evidence_lineage = RouteSpendLineage::from_routing_packet(
                &routing_packet,
                &route_receipt_ref,
                spend_receipt.spend_receipt_id.clone(),
            );
            let row = cost_routing_surface_row(
                registry,
                graduation_state,
                surface,
                &routing_packet,
                &spend_receipt,
                &route_receipt_ref,
            );
            surface_rows.push(row);
            evidence_lineage_rows.push(evidence_lineage);
            spend_receipts.push(spend_receipt);
        }

        Ok(Self {
            record_kind: COST_ROUTING_BETA_PACKET_RECORD_KIND.to_owned(),
            schema_version: COST_ROUTING_BETA_SCHEMA_VERSION,
            cost_routing_packet_id: cost_routing_packet_id.into(),
            registry_state_ref: registry.registry_id.clone(),
            graduation_state_ref: graduation_state.graduation_state_id.clone(),
            as_of: graduation_state.as_of.clone(),
            surface_rows,
            spend_receipts,
            evidence_lineage_rows,
            source_contract_refs: cost_routing_source_contract_refs(registry, graduation_state),
            minted_at,
        })
    }

    /// Validates claimed-surface route cost truth and spend receipt parity.
    pub fn validate(&self) -> Vec<CostRoutingBetaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != COST_ROUTING_BETA_PACKET_RECORD_KIND {
            violations.push(CostRoutingBetaViolation::WrongRecordKind);
        }
        if self.schema_version != COST_ROUTING_BETA_SCHEMA_VERSION {
            violations.push(CostRoutingBetaViolation::WrongSchemaVersion);
        }
        if self.cost_routing_packet_id.trim().is_empty()
            || self.registry_state_ref.trim().is_empty()
            || self.graduation_state_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CostRoutingBetaViolation::MissingPacketIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(CostRoutingBetaViolation::MissingSourceContractRefs);
        }
        if self.surface_rows.is_empty() {
            violations.push(CostRoutingBetaViolation::MissingClaimedSurfaceRows);
        }
        if self.surface_rows.len() != self.spend_receipts.len()
            || self.surface_rows.len() != self.evidence_lineage_rows.len()
        {
            violations.push(CostRoutingBetaViolation::ReceiptLineageCountMismatch);
        }

        for row in &self.surface_rows {
            if row.surface_id.trim().is_empty()
                || row.display_label.trim().is_empty()
                || row.route_policy_id_ref.trim().is_empty()
                || row.selected_provider_entry_ref.trim().is_empty()
                || row.selected_model_entry_ref.trim().is_empty()
                || row.route_receipt_ref.trim().is_empty()
                || row.spend_receipt_ref.trim().is_empty()
            {
                violations.push(CostRoutingBetaViolation::SurfaceRowMissingIdentity);
            }
            if row.selected_cost_envelope_token.trim().is_empty()
                || row.selected_cost_envelope_token == "envelope_unknown_unverified_cost"
                || row.selected_cost_visibility_token.trim().is_empty()
            {
                violations.push(CostRoutingBetaViolation::SurfaceRowMissingCostClass);
            }
            if row.budget_owner_ref.trim().is_empty()
                || row.budget_routing_policy_ref.trim().is_empty()
            {
                violations.push(CostRoutingBetaViolation::SurfaceRowMissingBudgetOwner);
            }
            if !row.selected_is_cheapest_qualifying
                && row
                    .route_selection_disclosure_ref
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
            {
                violations.push(CostRoutingBetaViolation::PolicyLimitedRouteMissingDisclosure);
            }
            if row.policy_class_token == RegistryRoutingPolicyClass::CheapestQualifying.as_str()
                && !row.selected_is_cheapest_qualifying
            {
                violations.push(CostRoutingBetaViolation::CheapestPolicyDidNotSelectCheapest);
            }
            if row.promotion_gate_token != "promotable" {
                violations.push(CostRoutingBetaViolation::PromotionGateNotPromotable);
            }
        }

        for receipt in &self.spend_receipts {
            violations.extend(receipt.validate());
            if !self.surface_rows.iter().any(|row| {
                row.spend_receipt_ref == receipt.spend_receipt_id
                    && row.route_receipt_ref == receipt.route_receipt_ref
                    && row.selected_cost_envelope_token == receipt.cost_envelope_class.as_str()
                    && row.selected_cost_visibility_token == receipt.cost_visibility_class.as_str()
                    && row.selected_quota_family_token == receipt.quota_family_class.as_str()
            }) {
                violations.push(CostRoutingBetaViolation::SpendReceiptRouteMismatch);
            }
        }

        for lineage in &self.evidence_lineage_rows {
            if !self.surface_rows.iter().any(|row| {
                row.route_receipt_ref == lineage.route_receipt_ref
                    && row.spend_receipt_ref == lineage.spend_receipt_ref
                    && row.selected_cost_envelope_token == lineage.cost_envelope_token
                    && row.budget_owner_ref == lineage.budget_owner_ref
            }) {
                violations.push(CostRoutingBetaViolation::EvidenceLineageMismatch);
            }
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("cost-routing packet serializes"),
        ) {
            violations.push(CostRoutingBetaViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON for support bundles.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("cost-routing packet serializes")
    }
}

/// Errors emitted when reading checked-in beta routing artifacts.
#[derive(Debug)]
pub enum CostRoutingBetaArtifactError {
    /// Provider/model registry artifact failed to parse.
    Registry(serde_json::Error),
    /// Graduation artifact failed to parse.
    Graduation(serde_json::Error),
    /// A claimed surface could not resolve to an admitted route.
    Routing(ProviderModelRegistryViolation),
}

impl fmt::Display for CostRoutingBetaArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Registry(error) => write!(formatter, "registry artifact parse failed: {error}"),
            Self::Graduation(error) => {
                write!(formatter, "graduation artifact parse failed: {error}")
            }
            Self::Routing(violation) => {
                write!(formatter, "cost routing artifact failed: {violation:?}")
            }
        }
    }
}

impl Error for CostRoutingBetaArtifactError {}

/// Validation failures emitted by cost-routing and spend-receipt checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CostRoutingBetaViolation {
    /// Cost-routing record kind is wrong.
    WrongRecordKind,
    /// Cost-routing schema version is wrong.
    WrongSchemaVersion,
    /// Cost-routing packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs are missing.
    MissingSourceContractRefs,
    /// No claimed-surface rows were projected.
    MissingClaimedSurfaceRows,
    /// Surface row identity is incomplete.
    SurfaceRowMissingIdentity,
    /// Surface row lacks an estimated or actual cost class.
    SurfaceRowMissingCostClass,
    /// Surface row lacks a budget owner or budget policy ref.
    SurfaceRowMissingBudgetOwner,
    /// A policy-limited or fallback route lacks a disclosure ref.
    PolicyLimitedRouteMissingDisclosure,
    /// Cheapest-qualifying policy selected a non-cheapest candidate.
    CheapestPolicyDidNotSelectCheapest,
    /// Graduation gate is not promotable for a claimed row.
    PromotionGateNotPromotable,
    /// Receipt and evidence-lineage counts do not match surface rows.
    ReceiptLineageCountMismatch,
    /// Spend-receipt record kind is wrong.
    SpendReceiptWrongRecordKind,
    /// Spend-receipt schema version is wrong.
    SpendReceiptWrongSchemaVersion,
    /// Spend receipt identity fields are incomplete.
    SpendReceiptMissingIdentity,
    /// Spend receipt lacks the minimum attribution dimensions.
    SpendReceiptMissingAttributionDimensions,
    /// Spend receipt lacks required budget-scope outcomes.
    SpendReceiptMissingBudgetScopeOutcomes,
    /// Spend receipt charge locus disagrees with run state.
    SpendReceiptChargeLocusMismatch,
    /// Spend receipt does not match the route it cites.
    SpendReceiptRouteMismatch,
    /// Evidence lineage does not match route/spend rows.
    EvidenceLineageMismatch,
    /// Exportable fields contain raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CostRoutingBetaViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "cost_routing_wrong_record_kind",
            Self::WrongSchemaVersion => "cost_routing_wrong_schema_version",
            Self::MissingPacketIdentity => "cost_routing_missing_packet_identity",
            Self::MissingSourceContractRefs => "cost_routing_missing_source_contract_refs",
            Self::MissingClaimedSurfaceRows => "cost_routing_missing_claimed_surface_rows",
            Self::SurfaceRowMissingIdentity => "cost_routing_surface_row_missing_identity",
            Self::SurfaceRowMissingCostClass => "cost_routing_surface_row_missing_cost_class",
            Self::SurfaceRowMissingBudgetOwner => "cost_routing_surface_row_missing_budget_owner",
            Self::PolicyLimitedRouteMissingDisclosure => {
                "cost_routing_policy_limited_route_missing_disclosure"
            }
            Self::CheapestPolicyDidNotSelectCheapest => {
                "cost_routing_cheapest_policy_did_not_select_cheapest"
            }
            Self::PromotionGateNotPromotable => "cost_routing_promotion_gate_not_promotable",
            Self::ReceiptLineageCountMismatch => "cost_routing_receipt_lineage_count_mismatch",
            Self::SpendReceiptWrongRecordKind => "spend_receipt_wrong_record_kind",
            Self::SpendReceiptWrongSchemaVersion => "spend_receipt_wrong_schema_version",
            Self::SpendReceiptMissingIdentity => "spend_receipt_missing_identity",
            Self::SpendReceiptMissingAttributionDimensions => {
                "spend_receipt_missing_attribution_dimensions"
            }
            Self::SpendReceiptMissingBudgetScopeOutcomes => {
                "spend_receipt_missing_budget_scope_outcomes"
            }
            Self::SpendReceiptChargeLocusMismatch => "spend_receipt_charge_locus_mismatch",
            Self::SpendReceiptRouteMismatch => "spend_receipt_route_mismatch",
            Self::EvidenceLineageMismatch => "cost_routing_evidence_lineage_mismatch",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in beta cost-routing packet.
///
/// # Errors
///
/// Returns an artifact error if the registry or graduation state cannot be
/// parsed, or if a claimed surface no longer resolves to an admitted route.
pub fn current_beta_cost_routing_packet(
) -> Result<CostRoutingBetaPacket, CostRoutingBetaArtifactError> {
    let registry: ProviderModelRegistryPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/provider_model_registry_beta/registry_packet.json"
    )))
    .map_err(CostRoutingBetaArtifactError::Registry)?;
    let graduation_state =
        current_beta_graduation_state().map_err(CostRoutingBetaArtifactError::Graduation)?;
    CostRoutingBetaPacket::from_registry_and_graduation(
        &registry,
        &graduation_state,
        "cost-routing-beta:claimed-ai-surfaces:2026-05-17",
        "2026-05-17T12:35:00Z",
    )
    .map_err(CostRoutingBetaArtifactError::Routing)
}

fn cost_routing_surface_row(
    registry: &ProviderModelRegistryPacket,
    graduation_state: &AiGraduationState,
    surface: &crate::registry::ClaimedAiSurface,
    routing_packet: &AiRoutingPacket,
    spend_receipt: &SpendReceiptRecord,
    route_receipt_ref: &str,
) -> CostRoutingSurfaceRow {
    let resolution = registry.resolve_route_for_surface(&surface.surface_id);
    let selected = routing_packet
        .selected_route()
        .expect("routing packet resolved a selected route");
    let cheapest = cheapest_eligible_candidate(&resolution.candidates);
    let selected_is_cheapest_qualifying = cheapest
        .map(|candidate| candidate.candidate_id == selected.candidate_id)
        .unwrap_or(false);
    let status = graduation_state.surface_status(registry, &surface.surface_id);

    CostRoutingSurfaceRow {
        surface_id: surface.surface_id.clone(),
        display_label: surface.display_label.clone(),
        registry_state_ref: registry.registry_id.clone(),
        graduation_state_ref: graduation_state.graduation_state_id.clone(),
        route_policy_id_ref: resolution.route_policy_id_ref,
        policy_class_token: resolution.policy_class.as_str().to_owned(),
        route_reason_token: selected.route_selection_reason_class.as_str().to_owned(),
        route_selection_override_reason_token: selected
            .route_selection_override_reason_class
            .as_str()
            .to_owned(),
        selected_provider_entry_ref: selected.provider_entry_ref.clone(),
        selected_model_entry_ref: selected.model_entry_ref.clone(),
        selected_execution_locus_token: selected.execution_locus_class.as_str().to_owned(),
        selected_route_origin_token: selected.route_origin_class.as_str().to_owned(),
        selected_cost_envelope_token: selected.envelope.cost_envelope_class.as_str().to_owned(),
        selected_cost_visibility_token: selected.envelope.cost_visibility_class.as_str().to_owned(),
        selected_quota_family_token: selected.quota.quota_family_class.as_str().to_owned(),
        budget_owner_ref: selected.quota.budget_owner_ref.clone(),
        budget_routing_policy_ref: selected.envelope.budget_routing_policy_ref.clone(),
        route_receipt_ref: route_receipt_ref.to_owned(),
        spend_receipt_ref: spend_receipt.spend_receipt_id.clone(),
        cheapest_candidate_provider_entry_ref: cheapest
            .map(|candidate| candidate.provider_entry_ref.clone()),
        cheapest_candidate_model_entry_ref: cheapest
            .map(|candidate| candidate.model_entry_ref.clone()),
        cheapest_candidate_cost_envelope_token: cheapest
            .map(|candidate| candidate.cost_envelope_class.as_str().to_owned()),
        selected_is_cheapest_qualifying,
        route_selection_disclosure_ref: selected.route_selection_disclosure_ref.clone(),
        promotion_gate_token: status.gate_state.as_str().to_owned(),
        packet_freshness_token: status.freshness_class.as_str().to_owned(),
        effective_support_class_token: status.effective_support_class.as_str().to_owned(),
        local_continuity_label: selected.quota.local_continuity_label.clone(),
    }
}

fn cheapest_eligible_candidate(
    candidates: &[RegistryRouteCandidate],
) -> Option<&RegistryRouteCandidate> {
    candidates
        .iter()
        .filter(|candidate| candidate.route_eligibility_class == RouteEligibilityClass::Eligible)
        .min_by_key(|candidate| {
            (
                candidate.cost_envelope_class.cost_rank(),
                candidate.route_priority,
                candidate.provider_entry_ref.as_str(),
            )
        })
}

fn cost_routing_source_contract_refs(
    registry: &ProviderModelRegistryPacket,
    graduation_state: &AiGraduationState,
) -> Vec<String> {
    let mut refs = BTreeSet::new();
    refs.insert("docs/ai/m3/cost_routing_beta.md".to_owned());
    refs.insert("docs/ai/spend_and_route_receipt_contract.md".to_owned());
    refs.insert("schemas/ai/spend_receipt.schema.json".to_owned());
    refs.insert("schemas/ai/provider_route_receipt.schema.json".to_owned());
    refs.extend(registry.source_contract_refs.iter().cloned());
    refs.extend(graduation_state.source_contract_refs.iter().cloned());
    refs.into_iter().collect()
}

fn receipt_safe_fragment(value: &str) -> String {
    value.replace(':', ".")
}

fn spend_attribution_dimensions() -> Vec<SpendAttributionDimensionClass> {
    let mut dimensions = REQUIRED_SPEND_ATTRIBUTION_DIMENSIONS.to_vec();
    dimensions.push(SpendAttributionDimensionClass::DeploymentProfileClassDimension);
    dimensions.push(SpendAttributionDimensionClass::PolicyEpochDimension);
    dimensions
}

fn spend_attribution_values(
    routing_packet: &AiRoutingPacket,
    candidate: &crate::routing::AiRouteCandidate,
) -> Vec<SpendAttributionValueRow> {
    vec![
        attribution_value(
            SpendAttributionDimensionClass::WorkflowOrSurfaceIdDimension,
            &routing_packet.workflow_or_surface_id,
            "Workflow or surface id.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::ProviderEntryIdDimension,
            &candidate.provider_entry_ref,
            "Selected provider entry id.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::ModelEntryIdDimension,
            &candidate.model_entry_ref,
            "Selected model entry id.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::ExecutionLocusClassDimension,
            candidate.execution_locus_class.as_str(),
            "Selected execution locus.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::RegionPostureClassDimension,
            candidate.region_posture_class.as_str(),
            "Selected region posture.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::RetentionStanceClassDimension,
            candidate.retention_stance_class.as_str(),
            "Selected retention stance.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::QuotaFamilyClassDimension,
            candidate.quota.quota_family_class.as_str(),
            "Selected quota family.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::DeploymentProfileClassDimension,
            routing_packet
                .policy_context
                .deployment_profile_class
                .as_str(),
            "Deployment profile in force.",
        ),
        attribution_value(
            SpendAttributionDimensionClass::PolicyEpochDimension,
            &routing_packet.policy_context.policy_epoch_ref,
            "Policy epoch in force.",
        ),
    ]
}

fn attribution_value(
    dimension_class: SpendAttributionDimensionClass,
    value_ref: &str,
    notes_summary: &str,
) -> SpendAttributionValueRow {
    SpendAttributionValueRow {
        dimension_class,
        value_ref: value_ref.to_owned(),
        notes_summary: notes_summary.to_owned(),
    }
}

fn default_budget_scope_outcomes(
    run_state_class: RoutingRunStateClass,
    cost_envelope_class: CostEnvelopeClass,
) -> Vec<BudgetScopeOutcomeRow> {
    let request_and_session = if matches!(run_state_class, RoutingRunStateClass::PreviewPreDispatch)
    {
        BudgetScopeOutcomeClass::ScopeOutcomeUnknownPreDispatch
    } else if matches!(run_state_class, RoutingRunStateClass::BudgetBlockedRefusal) {
        BudgetScopeOutcomeClass::ScopeOverBandBlocked
    } else {
        BudgetScopeOutcomeClass::ScopeUnderBand
    };

    vec![
        BudgetScopeOutcomeRow {
            budget_scope_class: BudgetScopeClass::PerRequest,
            budget_scope_outcome_class: request_and_session,
            cost_envelope_class,
            notes_summary: "Per-request scope uses the selected route cost band.".to_owned(),
        },
        BudgetScopeOutcomeRow {
            budget_scope_class: BudgetScopeClass::PerSession,
            budget_scope_outcome_class: request_and_session,
            cost_envelope_class,
            notes_summary: "Per-session scope uses the selected route cost band.".to_owned(),
        },
        BudgetScopeOutcomeRow {
            budget_scope_class: BudgetScopeClass::PerAgentInvocation,
            budget_scope_outcome_class: BudgetScopeOutcomeClass::ScopeNotApplicable,
            cost_envelope_class,
            notes_summary: "No branch-agent chain is attached to this receipt.".to_owned(),
        },
    ]
}

fn charge_locus_for(
    run_state_class: RoutingRunStateClass,
    execution_locus_class: ExecutionLocusClass,
    route_origin_class: RouteOriginClass,
    cost_visibility_class: CostVisibilityClass,
    quota_family_class: QuotaFamilyClass,
) -> WasChargedToUserClass {
    if run_state_blocks_or_cancels(run_state_class) {
        return WasChargedToUserClass::NotChargedRunBlocked;
    }
    if matches!(
        execution_locus_class,
        ExecutionLocusClass::LocalInProcess
            | ExecutionLocusClass::LocalSandboxProcess
            | ExecutionLocusClass::LocalCompanionService
    ) {
        return WasChargedToUserClass::NotChargedLocal;
    }
    if cost_visibility_class == CostVisibilityClass::BundledNoIncrementalCost {
        return WasChargedToUserClass::NotChargedBundled;
    }
    match (
        route_origin_class,
        cost_visibility_class,
        quota_family_class,
    ) {
        (
            RouteOriginClass::ByokUserCredential,
            CostVisibilityClass::MeteredPerRequest | CostVisibilityClass::MeteredPerToken,
            _,
        ) => WasChargedToUserClass::ChargedUserByokMetered,
        (RouteOriginClass::ByokUserCredential, CostVisibilityClass::FlatFeeSubscription, _) => {
            WasChargedToUserClass::ChargedUserByokSubscription
        }
        (RouteOriginClass::EnterpriseGateway, _, _)
        | (_, _, QuotaFamilyClass::EnterpriseGatewayPooledQuota) => {
            WasChargedToUserClass::ChargedOrganisationPooled
        }
        (_, CostVisibilityClass::FlatFeeSubscription, _) => {
            WasChargedToUserClass::ChargedOrganisationSubscription
        }
        _ => WasChargedToUserClass::ChargeUnknownUnverified,
    }
}

fn run_state_blocks_or_cancels(run_state_class: RoutingRunStateClass) -> bool {
    matches!(
        run_state_class,
        RoutingRunStateClass::BudgetBlockedRefusal
            | RoutingRunStateClass::RouteBlockedRefusal
            | RoutingRunStateClass::CancelledByUser
            | RoutingRunStateClass::CancelledByPolicy
    )
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
