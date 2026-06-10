//! Routing policy, quota families, per-session cost bands, and fallback chains.
//!
//! This module materializes the route-governance surface into one export-safe
//! truth packet whose unit of truth is a [`RoutingPolicyRow`]: a single governed
//! surface binding the provider/locality mode its routing policy currently
//! resolves to, the quota family that governs it, the per-session cost band that
//! prices it, and the ordered fallback chain that keeps the surface reachable
//! when the primary route is exhausted, blocked, or unavailable. The packet is
//! the canonical routing-policy source for shell, docs, support export, and
//! release tooling; consumers project it instead of re-deriving quota, cost, or
//! fallback state by hand.
//!
//! The packet refuses to present a routing policy greener than its cost, quota,
//! and continuity posture can back. Every governed surface must carry a fallback
//! chain whose hops are strictly ordered and whose tail is a non-AI terminal
//! fallback reachable without any model, so a surface is never stranded when AI
//! routes run out. A claimed surface must resolve to exactly one selected hop
//! whose mode matches the resolved mode, so the disclosed route is the route that
//! ran. A claimed surface priced on a metered or subscription band must disclose
//! who is charged rather than leaving the charge unknown, and a per-session cost
//! band that is only an estimate may not back a Stable claim, so cost is never
//! hidden behind generic AI language. A surface whose quota family is exhausted
//! or whose per-session budget is spent may not keep claiming Stable; the
//! exhaustion narrows the claim instead of hiding it. Every surface carries a
//! closed set of downgrade rules — including the stale-proof and
//! provider-unavailable triggers — that narrow the claim instead of hiding the
//! route, reusing the qualification, downgrade-trigger, and rollback-posture
//! vocabularies frozen by the M5 AI workflow matrix lane so no policy row may stay
//! greener than its evidence.
//!
//! Raw provider endpoints, credential bodies, raw provider payloads, and exact
//! spend values stay outside the support boundary; the packet carries modes,
//! families, bands, scopes, and review-safe labels only.
//!
//! The boundary schema is
//! [`schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json`](../../../../schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains.md`](../../../../docs/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/`](../../../../fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/).

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
use crate::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::PROVIDER_MODEL_REGISTRY_SCHEMA_REF;

/// Stable record-kind tag carried by [`RoutingPolicyPacket`].
pub const ROUTING_POLICY_RECORD_KIND: &str =
    "implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains";

/// Schema version for routing-policy records.
pub const ROUTING_POLICY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ROUTING_POLICY_SCHEMA_REF: &str =
    "schemas/ai/implement-routing-policy-quota-families-per-session-cost-bands-and-fallback-chains.schema.json";

/// Repo-relative path of the routing-policy contract doc.
pub const ROUTING_POLICY_DOC_REF: &str =
    "docs/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains.md";

/// Repo-relative path of the protected fixture directory.
pub const ROUTING_POLICY_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains";

/// Repo-relative path of the checked support-export artifact.
pub const ROUTING_POLICY_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ROUTING_POLICY_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains.md";

/// Provider/locality mode a routing policy resolves a surface to.
///
/// The mode is disclosed explicitly so cost, provider, and locality never hide
/// behind generic AI language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutePolicyModeClass {
    /// Runs on-device with no egress.
    Local,
    /// Runs against a vendor through the user's own credential.
    Byok,
    /// Runs against a first-party managed endpoint.
    Managed,
    /// Runs through a brokered enterprise gateway.
    EnterpriseGateway,
}

impl RoutePolicyModeClass {
    /// Every mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Local,
        Self::Byok,
        Self::Managed,
        Self::EnterpriseGateway,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Byok => "byok",
            Self::Managed => "managed",
            Self::EnterpriseGateway => "enterprise_gateway",
        }
    }

    /// Whether this mode sends a request off the device.
    pub const fn is_egress(self) -> bool {
        !matches!(self, Self::Local)
    }
}

/// Quota family that governs how a surface's requests are rationed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaFamilyClass {
    /// On-device usage with no enforced ceiling.
    PerUserLocalUnmetered,
    /// Per-user vendor quota billed against a BYOK credential.
    PerUserByokVendorQuota,
    /// Pooled quota shared across an organisation.
    OrganisationPooledQuota,
    /// Pooled quota brokered through an enterprise gateway.
    EnterpriseGatewayPooledQuota,
    /// Entitlement granted by a first-party managed plan.
    ManagedEntitlementQuota,
    /// Rate-limited free tier.
    FreeTierRateLimited,
    /// Metered paid tier.
    PaidTierMetered,
}

impl QuotaFamilyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUserLocalUnmetered => "per_user_local_unmetered",
            Self::PerUserByokVendorQuota => "per_user_byok_vendor_quota",
            Self::OrganisationPooledQuota => "organisation_pooled_quota",
            Self::EnterpriseGatewayPooledQuota => "enterprise_gateway_pooled_quota",
            Self::ManagedEntitlementQuota => "managed_entitlement_quota",
            Self::FreeTierRateLimited => "free_tier_rate_limited",
            Self::PaidTierMetered => "paid_tier_metered",
        }
    }
}

/// State of a surface's quota family at packet mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaStateClass {
    /// Within the quota ceiling.
    WithinLimit,
    /// Approaching the ceiling.
    Warning,
    /// Ceiling reached; new dispatch is blocked.
    Exhausted,
    /// In a grace window after the ceiling.
    Grace,
    /// Held by policy regardless of remaining quota.
    PausedByPolicy,
    /// Quota state could not be verified.
    UnknownUnverified,
}

impl QuotaStateClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinLimit => "within_limit",
            Self::Warning => "warning",
            Self::Exhausted => "exhausted",
            Self::Grace => "grace",
            Self::PausedByPolicy => "paused_by_policy",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }

    /// Whether the quota currently blocks new dispatch on the primary route.
    pub const fn blocks_dispatch(self) -> bool {
        matches!(self, Self::Exhausted | Self::PausedByPolicy)
    }
}

/// Scope a quota family is accounted against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaScopeClass {
    /// Accounted per interactive session.
    PerSession,
    /// Accounted per user.
    PerUser,
    /// Accounted per workspace.
    PerWorkspace,
    /// Accounted per organisation.
    PerOrganisation,
    /// Accounted per deployment profile.
    PerDeploymentProfile,
    /// Accounted on the local device only.
    LocalDevice,
    /// Accounted against an enterprise pool.
    EnterprisePool,
}

impl QuotaScopeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerSession => "per_session",
            Self::PerUser => "per_user",
            Self::PerWorkspace => "per_workspace",
            Self::PerOrganisation => "per_organisation",
            Self::PerDeploymentProfile => "per_deployment_profile",
            Self::LocalDevice => "local_device",
            Self::EnterprisePool => "enterprise_pool",
        }
    }
}

/// Quota inspector block for a routing-policy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaInspector {
    /// Quota family governing the surface.
    pub family: QuotaFamilyClass,
    /// State of the quota at mint time.
    pub state: QuotaStateClass,
    /// Scope the quota is accounted against.
    pub scope: QuotaScopeClass,
    /// Review-safe label naming who owns the budget (no raw account id).
    pub budget_owner_label: String,
    /// Review-safe explanation of the quota posture.
    pub explanation_label: String,
}

/// Per-session cost band a surface is priced on.
///
/// Bands are coarse, review-safe brackets; the packet never carries an exact
/// spend value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostBandClass {
    /// Bundled into the plan with no incremental cost.
    BundledNoIncrementalCost,
    /// Free tier subject to rate limiting.
    FreeTierRateLimited,
    /// Metered, low per-session volume.
    MeteredLowBand,
    /// Metered, medium per-session volume.
    MeteredMediumBand,
    /// Metered, high per-session volume.
    MeteredHighBand,
    /// Flat-fee subscription.
    FlatFeeSubscriptionBand,
    /// Estimated only; not yet measured.
    EstimatedUnverifiedBand,
}

impl CostBandClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledNoIncrementalCost => "bundled_no_incremental_cost",
            Self::FreeTierRateLimited => "free_tier_rate_limited",
            Self::MeteredLowBand => "metered_low_band",
            Self::MeteredMediumBand => "metered_medium_band",
            Self::MeteredHighBand => "metered_high_band",
            Self::FlatFeeSubscriptionBand => "flat_fee_subscription_band",
            Self::EstimatedUnverifiedBand => "estimated_unverified_band",
        }
    }

    /// Whether this band charges the user incrementally and so demands a
    /// charge-disclosure label.
    pub const fn is_charged(self) -> bool {
        matches!(
            self,
            Self::MeteredLowBand
                | Self::MeteredMediumBand
                | Self::MeteredHighBand
                | Self::FlatFeeSubscriptionBand
        )
    }
}

/// Whether a per-session cost band reflects a measured or estimated figure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMeasurementClass {
    /// A pre-dispatch estimate band.
    EstimateBand,
    /// A measured, verified figure.
    ActualMeasured,
    /// A measured figure that could not be verified.
    ActualUnverified,
}

impl CostMeasurementClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EstimateBand => "estimate_band",
            Self::ActualMeasured => "actual_measured",
            Self::ActualUnverified => "actual_unverified",
        }
    }
}

/// Who is charged for a surface's per-session spend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargedDisclosureClass {
    /// No charge; the route stays on-device.
    NotChargedLocal,
    /// No incremental charge; bundled into the plan.
    NotChargedBundled,
    /// The user is charged per metered use.
    ChargedUserMetered,
    /// The user is charged through a subscription.
    ChargedUserSubscription,
    /// An organisation pool absorbs the charge.
    ChargedOrganisationPooled,
    /// Who is charged could not be determined.
    ChargeUnknownUnverified,
}

impl ChargedDisclosureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotChargedLocal => "not_charged_local",
            Self::NotChargedBundled => "not_charged_bundled",
            Self::ChargedUserMetered => "charged_user_metered",
            Self::ChargedUserSubscription => "charged_user_subscription",
            Self::ChargedOrganisationPooled => "charged_organisation_pooled",
            Self::ChargeUnknownUnverified => "charge_unknown_unverified",
        }
    }

    /// Whether the charge owner is disclosed (anything but the unknown class).
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::ChargeUnknownUnverified)
    }
}

/// Per-session cost band block for a routing-policy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerSessionCostBand {
    /// Cost band the surface is priced on for one session.
    pub band: CostBandClass,
    /// Whether the band is measured or merely estimated.
    pub measurement: CostMeasurementClass,
    /// Who is charged for the session's spend.
    pub charged: ChargedDisclosureClass,
    /// Review-safe label naming the per-session budget owner.
    pub session_budget_owner_label: String,
    /// True when the per-session budget has been spent.
    pub exhausted_this_session: bool,
    /// Review-safe explanation of the per-session cost posture.
    pub explanation_label: String,
}

/// Reason a fallback hop exists in the chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackHopReasonClass {
    /// The primary, cheapest qualifying route.
    PrimaryCheapestQualifying,
    /// Reached after the prior route's quota was exhausted.
    FallbackAfterQuotaExhausted,
    /// Reached after the prior route's per-session budget was exhausted.
    FallbackAfterBudgetExhausted,
    /// Reached after the prior route's provider became unavailable.
    FallbackAfterProviderUnavailable,
    /// Reached after policy blocked the prior route.
    FallbackAfterPolicyBlocked,
    /// The terminal non-AI path that needs no model.
    NonAiTerminalFallback,
}

impl FallbackHopReasonClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryCheapestQualifying => "primary_cheapest_qualifying",
            Self::FallbackAfterQuotaExhausted => "fallback_after_quota_exhausted",
            Self::FallbackAfterBudgetExhausted => "fallback_after_budget_exhausted",
            Self::FallbackAfterProviderUnavailable => "fallback_after_provider_unavailable",
            Self::FallbackAfterPolicyBlocked => "fallback_after_policy_blocked",
            Self::NonAiTerminalFallback => "non_ai_terminal_fallback",
        }
    }

    /// Whether this hop is the terminal non-AI fallback.
    pub const fn is_non_ai_terminal(self) -> bool {
        matches!(self, Self::NonAiTerminalFallback)
    }
}

/// Outcome of a fallback hop at mint time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackHopOutcomeClass {
    /// This hop is the one currently selected to serve the surface.
    Selected,
    /// This hop is available but not currently selected.
    AvailableNotSelected,
    /// This hop was skipped because its quota or budget was exhausted.
    ExhaustedSkipped,
    /// This hop was skipped because policy blocked it.
    BlockedSkipped,
    /// This hop is the reachable non-AI terminal fallback.
    TerminalReachable,
}

impl FallbackHopOutcomeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Selected => "selected",
            Self::AvailableNotSelected => "available_not_selected",
            Self::ExhaustedSkipped => "exhausted_skipped",
            Self::BlockedSkipped => "blocked_skipped",
            Self::TerminalReachable => "terminal_reachable",
        }
    }

    /// Whether this hop is the currently selected route.
    pub const fn is_selected(self) -> bool {
        matches!(self, Self::Selected)
    }
}

/// One hop in a surface's ordered fallback chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackHop {
    /// Position in the chain, starting at 0 and strictly increasing.
    pub order: u32,
    /// Provider/locality mode this hop routes to.
    pub mode: RoutePolicyModeClass,
    /// Reason this hop exists in the chain.
    pub reason: FallbackHopReasonClass,
    /// Outcome of this hop at mint time.
    pub outcome: FallbackHopOutcomeClass,
    /// Review-safe label describing the hop.
    pub label: String,
}

/// One downgrade rule that narrows a surface's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the surface narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// One routing-policy row binding quota, per-session cost, and fallback truth
/// for a governed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyRow {
    /// Stable routing-policy id.
    pub policy_id: String,
    /// Stable id of the governed surface.
    pub surface_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Provider/locality mode the policy currently resolves to.
    pub resolved_mode: RoutePolicyModeClass,
    /// Qualification class claimed for this surface.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Quota inspector block.
    pub quota: QuotaInspector,
    /// Per-session cost band block.
    pub session_cost_band: PerSessionCostBand,
    /// Ordered fallback chain ending in a non-AI terminal hop.
    pub fallback_chain: Vec<FallbackHop>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<RoutingPolicyDowngradeRule>,
    /// Rollback posture for a policy change on this surface.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// True when the rollback path has been drilled and verified.
    pub rollback_verified: bool,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl RoutingPolicyRow {
    /// Whether this surface carries a publicly claimed qualification.
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

    /// The currently selected fallback hop, if any.
    pub fn selected_hop(&self) -> Option<&FallbackHop> {
        self.fallback_chain
            .iter()
            .find(|hop| hop.outcome.is_selected())
    }

    /// Whether the chain contains a reachable non-AI terminal fallback.
    pub fn has_non_ai_terminal_fallback(&self) -> bool {
        self.fallback_chain
            .iter()
            .any(|hop| hop.reason.is_non_ai_terminal())
    }

    /// Whether this surface's per-session budget or quota currently blocks the
    /// primary route.
    pub fn is_exhausted(&self) -> bool {
        self.quota.state.blocks_dispatch() || self.session_cost_band.exhausted_this_session
    }

    /// Qualification this surface narrows to when `trigger` fires.
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

    /// Renders a deterministic, review-safe inspector card for this surface.
    pub fn render_inspector(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("### Policy `{}`\n", self.policy_id));
        out.push_str(&format!(
            "- Surface: `{}` ({})\n",
            self.surface_id, self.surface_label
        ));
        out.push_str(&format!(
            "- Qualification: `{}`\n",
            self.claimed_qualification.as_str()
        ));
        out.push_str(&format!("- Mode: `{}`\n", self.resolved_mode.as_str()));
        out.push_str(&format!(
            "- Quota: `{}` / `{}` / `{}` ({})\n",
            self.quota.family.as_str(),
            self.quota.state.as_str(),
            self.quota.scope.as_str(),
            self.quota.budget_owner_label
        ));
        out.push_str(&format!(
            "- Session cost: `{}` / `{}` / `{}` (exhausted: {}) ({})\n",
            self.session_cost_band.band.as_str(),
            self.session_cost_band.measurement.as_str(),
            self.session_cost_band.charged.as_str(),
            self.session_cost_band.exhausted_this_session,
            self.session_cost_band.explanation_label
        ));
        out.push_str("- Fallback chain:\n");
        for hop in &self.fallback_chain {
            out.push_str(&format!(
                "  {}. `{}` / `{}` / `{}` ({})\n",
                hop.order,
                hop.mode.as_str(),
                hop.reason.as_str(),
                hop.outcome.as_str(),
                hop.label
            ));
        }
        out
    }
}

/// Proof freshness block for the routing-policy packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed surfaces.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`RoutingPolicyPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPolicyPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Routing-policy rows.
    pub policies: Vec<RoutingPolicyRow>,
    /// Proof freshness block.
    pub proof_freshness: RoutingPolicyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe routing-policy packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyPacket {
    /// Record kind; must equal [`ROUTING_POLICY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ROUTING_POLICY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable catalogue label.
    pub catalogue_label: String,
    /// Routing-policy rows.
    pub policies: Vec<RoutingPolicyRow>,
    /// Proof freshness block.
    pub proof_freshness: RoutingPolicyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RoutingPolicyPacket {
    /// Builds a routing-policy packet from stable-lane input.
    pub fn new(input: RoutingPolicyPacketInput) -> Self {
        Self {
            record_kind: ROUTING_POLICY_RECORD_KIND.to_owned(),
            schema_version: ROUTING_POLICY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            catalogue_label: input.catalogue_label,
            policies: input.policies,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the routing-policy invariants.
    pub fn validate(&self) -> Vec<RoutingPolicyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ROUTING_POLICY_RECORD_KIND {
            violations.push(RoutingPolicyViolation::WrongRecordKind);
        }
        if self.schema_version != ROUTING_POLICY_SCHEMA_VERSION {
            violations.push(RoutingPolicyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.catalogue_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RoutingPolicyViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_policies_present(self, &mut violations);
        for policy in &self.policies {
            validate_policy(policy, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("routing-policy packet serializes"),
        ) {
            violations.push(RoutingPolicyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of surfaces carrying a publicly claimed qualification.
    pub fn claimed_policy_count(&self) -> usize {
        self.policies
            .iter()
            .filter(|policy| policy.is_claimed())
            .count()
    }

    /// Count of surfaces whose quota or per-session budget is exhausted.
    pub fn exhausted_policy_count(&self) -> usize {
        self.policies
            .iter()
            .filter(|policy| policy.is_exhausted())
            .count()
    }

    /// Count of surfaces priced on a charged (metered or subscription) band.
    pub fn charged_band_count(&self) -> usize {
        self.policies
            .iter()
            .filter(|policy| policy.session_cost_band.band.is_charged())
            .count()
    }

    /// Count of surfaces whose fallback chain carries a non-AI terminal hop.
    pub fn non_ai_fallback_count(&self) -> usize {
        self.policies
            .iter()
            .filter(|policy| policy.has_non_ai_terminal_fallback())
            .count()
    }

    /// Returns the policy row for `policy_id`, if present.
    pub fn policy(&self, policy_id: &str) -> Option<&RoutingPolicyRow> {
        self.policies
            .iter()
            .find(|policy| policy.policy_id == policy_id)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("routing-policy packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Routing Policy, Quota Families, Per-Session Cost Bands, And Fallback Chains\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.catalogue_label));
        out.push_str(&format!(
            "- Policies: {} ({} claimed, {} charged-band, {} exhausted, {} with non-AI fallback)\n",
            self.policies.len(),
            self.claimed_policy_count(),
            self.charged_band_count(),
            self.exhausted_policy_count(),
            self.non_ai_fallback_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Policy inspectors\n\n");
        for policy in &self.policies {
            out.push_str(&policy.render_inspector());
            out.push('\n');
        }
        out
    }
}

/// Errors emitted when reading the checked-in routing-policy export.
#[derive(Debug)]
pub enum RoutingPolicyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RoutingPolicyViolation>),
}

impl fmt::Display for RoutingPolicyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "routing-policy export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "routing-policy export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RoutingPolicyArtifactError {}

/// Validation failures emitted by [`RoutingPolicyPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingPolicyViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no policies.
    NoPolicies,
    /// A policy id appears more than once.
    DuplicatePolicy,
    /// A policy row is missing a required identity or label field.
    PolicyRowIncomplete,
    /// A policy's fallback chain is empty.
    FallbackChainEmpty,
    /// A fallback chain's hop orders are not strictly increasing from zero.
    FallbackChainNotOrdered,
    /// A fallback chain has no non-AI terminal fallback hop.
    MissingNonAiTerminalFallback,
    /// More than one fallback hop is marked selected.
    MultipleSelectedHops,
    /// A claimed surface has no selected fallback hop.
    ClaimedPolicyNoSelectedHop,
    /// The selected hop's mode does not match the resolved mode.
    SelectedHopModeMismatch,
    /// A charged cost band omits its charge-disclosure label.
    ChargedBandUndisclosed,
    /// A claimed surface's per-session cost is only an estimate.
    EstimatedBandClaimsStable,
    /// An exhausted surface still claims Stable.
    ExhaustedPolicyClaimsStable,
    /// A claimed surface is missing required evidence packet refs.
    ClaimedPolicyMissingEvidence,
    /// A claimed surface's reversing rollback path is not verified.
    ClaimedRollbackUnverified,
    /// A surface has no downgrade rules.
    DowngradeRulesMissing,
    /// A surface's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A surface's downgrade rules omit the provider-unavailable trigger.
    DowngradeRuleMissingProviderUnavailable,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RoutingPolicyViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoPolicies => "no_policies",
            Self::DuplicatePolicy => "duplicate_policy",
            Self::PolicyRowIncomplete => "policy_row_incomplete",
            Self::FallbackChainEmpty => "fallback_chain_empty",
            Self::FallbackChainNotOrdered => "fallback_chain_not_ordered",
            Self::MissingNonAiTerminalFallback => "missing_non_ai_terminal_fallback",
            Self::MultipleSelectedHops => "multiple_selected_hops",
            Self::ClaimedPolicyNoSelectedHop => "claimed_policy_no_selected_hop",
            Self::SelectedHopModeMismatch => "selected_hop_mode_mismatch",
            Self::ChargedBandUndisclosed => "charged_band_undisclosed",
            Self::EstimatedBandClaimsStable => "estimated_band_claims_stable",
            Self::ExhaustedPolicyClaimsStable => "exhausted_policy_claims_stable",
            Self::ClaimedPolicyMissingEvidence => "claimed_policy_missing_evidence",
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

/// Reads and validates the checked-in routing-policy export.
pub fn current_routing_policy_export() -> Result<RoutingPolicyPacket, RoutingPolicyArtifactError> {
    let packet: RoutingPolicyPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_routing_policy_quota_families_per_session_cost_bands_and_fallback_chains/support_export.json"
    )))
    .map_err(RoutingPolicyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RoutingPolicyArtifactError::Validation(violations))
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

fn validate_source_contracts(
    packet: &RoutingPolicyPacket,
    violations: &mut Vec<RoutingPolicyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ROUTING_POLICY_SCHEMA_REF,
        ROUTING_POLICY_DOC_REF,
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RoutingPolicyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_policies_present(
    packet: &RoutingPolicyPacket,
    violations: &mut Vec<RoutingPolicyViolation>,
) {
    if packet.policies.is_empty() {
        violations.push(RoutingPolicyViolation::NoPolicies);
        return;
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for policy in &packet.policies {
        if !seen.insert(policy.policy_id.as_str()) {
            violations.push(RoutingPolicyViolation::DuplicatePolicy);
        }
    }
}

fn validate_policy(policy: &RoutingPolicyRow, violations: &mut Vec<RoutingPolicyViolation>) {
    if policy.policy_id.trim().is_empty()
        || policy.surface_id.trim().is_empty()
        || policy.surface_label.trim().is_empty()
        || policy.quota.budget_owner_label.trim().is_empty()
        || policy.quota.explanation_label.trim().is_empty()
        || policy
            .session_cost_band
            .session_budget_owner_label
            .trim()
            .is_empty()
        || policy.session_cost_band.explanation_label.trim().is_empty()
    {
        violations.push(RoutingPolicyViolation::PolicyRowIncomplete);
    }

    validate_fallback_chain(policy, violations);
    validate_cost_band(policy, violations);

    // An exhausted quota or per-session budget narrows the claim instead of
    // hiding behind a Stable label.
    if policy.is_exhausted()
        && policy.claimed_qualification == M5AiWorkflowQualificationClass::Stable
    {
        violations.push(RoutingPolicyViolation::ExhaustedPolicyClaimsStable);
    }

    if policy.is_claimed() && policy.evidence_packet_refs.is_empty() {
        violations.push(RoutingPolicyViolation::ClaimedPolicyMissingEvidence);
    }

    // A claimed surface whose policy change can be reversed must have drilled
    // that reversal; a non-applicable posture carries no reversal to verify.
    if policy.is_claimed()
        && policy.rollback_posture != M5AiWorkflowRollbackPosture::NotApplicable
        && !policy.rollback_verified
    {
        violations.push(RoutingPolicyViolation::ClaimedRollbackUnverified);
    }

    validate_downgrade_rules(policy, violations);
}

fn validate_fallback_chain(
    policy: &RoutingPolicyRow,
    violations: &mut Vec<RoutingPolicyViolation>,
) {
    if policy.fallback_chain.is_empty() {
        violations.push(RoutingPolicyViolation::FallbackChainEmpty);
        return;
    }

    // Hop orders must start at zero and strictly increase so the chain reads as
    // an unambiguous sequence.
    let mut ordered = true;
    for (index, hop) in policy.fallback_chain.iter().enumerate() {
        if hop.order as usize != index {
            ordered = false;
            break;
        }
    }
    if !ordered {
        violations.push(RoutingPolicyViolation::FallbackChainNotOrdered);
    }

    if !policy.has_non_ai_terminal_fallback() {
        violations.push(RoutingPolicyViolation::MissingNonAiTerminalFallback);
    }

    let selected = policy
        .fallback_chain
        .iter()
        .filter(|hop| hop.outcome.is_selected())
        .count();
    if selected > 1 {
        violations.push(RoutingPolicyViolation::MultipleSelectedHops);
    }
    if policy.is_claimed() && selected == 0 {
        violations.push(RoutingPolicyViolation::ClaimedPolicyNoSelectedHop);
    }
    if let Some(hop) = policy.selected_hop() {
        if hop.mode != policy.resolved_mode {
            violations.push(RoutingPolicyViolation::SelectedHopModeMismatch);
        }
    }
}

fn validate_cost_band(policy: &RoutingPolicyRow, violations: &mut Vec<RoutingPolicyViolation>) {
    let band = &policy.session_cost_band;

    // A band that charges the user must say who is charged.
    if band.band.is_charged() && !band.charged.is_disclosed() {
        violations.push(RoutingPolicyViolation::ChargedBandUndisclosed);
    }

    // An estimate-only or estimated band may not back a Stable claim.
    if policy.claimed_qualification == M5AiWorkflowQualificationClass::Stable
        && (band.band == CostBandClass::EstimatedUnverifiedBand
            || band.measurement == CostMeasurementClass::EstimateBand)
    {
        violations.push(RoutingPolicyViolation::EstimatedBandClaimsStable);
    }
}

fn validate_downgrade_rules(
    policy: &RoutingPolicyRow,
    violations: &mut Vec<RoutingPolicyViolation>,
) {
    if policy.downgrade_rules.is_empty() {
        violations.push(RoutingPolicyViolation::DowngradeRulesMissing);
        return;
    }

    if !policy
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(RoutingPolicyViolation::DowngradeRuleMissingProofStale);
    }

    // Quota and per-session budget exhaustion narrow through the
    // provider-unavailable trigger, so every row must carry it.
    if !policy
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProviderUnavailable)
    {
        violations.push(RoutingPolicyViolation::DowngradeRuleMissingProviderUnavailable);
    }

    let claimed_rank = qualification_rank(policy.claimed_qualification);
    for rule in &policy.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(RoutingPolicyViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &RoutingPolicyPacket,
    violations: &mut Vec<RoutingPolicyViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(RoutingPolicyViolation::ProofFreshnessIncomplete);
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
