//! Stable AI route and spend truth records.
//!
//! This module stabilizes the AI route/spend lane into one export-safe packet
//! that binds the preflight estimate card, the live run strip, the post-run
//! receipt, the distinct quota-family summary, and the route-downgrade banner
//! for a single material AI action — plus the non-AI fallback path that always
//! remains, the typed provider/model/external-tool registry resolution, and the
//! cumulative-spend posture any visible batch/agent lane must carry to claim the
//! Stable line.
//!
//! It does not re-derive routing, spend-receipt, registry, or run-history truth.
//! The [`crate::routing::AiRoutingPacket`] route lane, the
//! [`crate::routing_policy::SpendReceiptRecord`] spend lane, the
//! [`crate::registry::ProviderModelRegistryPacket`] registry lane, and the
//! frozen provider-route/spend-receipt boundary
//! ([`schemas/ai/provider_route_receipt.schema.json`](../../../schemas/ai/provider_route_receipt.schema.json)
//! and
//! [`schemas/ai/spend_receipt.schema.json`](../../../schemas/ai/spend_receipt.schema.json))
//! remain canonical for their own slices. This packet re-exports those classes
//! verbatim, references their receipts by id, and adds the visibility invariants
//! a user, admin, support engineer, or release packet needs to answer — for one
//! evidence id — what route an action intended to use, what budget family it
//! draws from, what it actually consumed, why a downgrade happened, and what
//! manual path remains when AI is unavailable.
//!
//! The frozen contracts this lane projects against are the spend/route-receipt
//! contract
//! ([`docs/ai/spend_and_route_receipt_contract.md`](../../../docs/ai/spend_and_route_receipt_contract.md))
//! and the model-graduation/budget contract
//! ([`docs/ai/model_graduation_and_budget_contract.md`](../../../docs/ai/model_graduation_and_budget_contract.md)).
//!
//! The record is export-safe. It carries refs, registry ids, state tokens,
//! coarse classes, counts, and review labels only. Raw provider payloads,
//! endpoint URLs, credentials, raw token counts, exact prices, and
//! billing-account ids stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::registry::{RegistryAuthModeClass, RegistryTransportClass};
use crate::routing::{
    CostEnvelopeClass, ExecutionLocusClass, LatencyEnvelopeClass, QuotaFamilyClass, QuotaScopeClass,
    QuotaStateClass, RegionPostureClass, RetentionStanceClass, RouteChangeCauseClass,
};

/// Stable record-kind tag carried by [`AiRouteSpendTruthPacket`].
pub const AI_ROUTE_SPEND_TRUTH_RECORD_KIND: &str = "ai_route_spend_truth";

/// Schema version for AI route/spend truth records.
pub const AI_ROUTE_SPEND_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI route/spend truth boundary schema.
pub const AI_ROUTE_SPEND_TRUTH_SCHEMA_REF: &str = "schemas/ai/ai-route-receipt.schema.json";

/// Repo-relative path of the AI route/spend truth contract doc.
pub const AI_ROUTE_SPEND_TRUTH_AI_DOC_REF: &str =
    "docs/ai/m4/stabilize_ai_route_and_spend_truth.md";

/// Repo-relative path of the frozen spend/route-receipt contract.
pub const AI_ROUTE_SPEND_TRUTH_RECEIPT_CONTRACT_REF: &str =
    "docs/ai/spend_and_route_receipt_contract.md";

/// Repo-relative path of the frozen model-graduation/budget contract.
pub const AI_ROUTE_SPEND_TRUTH_BUDGET_CONTRACT_REF: &str =
    "docs/ai/model_graduation_and_budget_contract.md";

/// Repo-relative path of the protected AI route/spend truth fixture directory.
pub const AI_ROUTE_SPEND_TRUTH_FIXTURE_DIR: &str =
    "fixtures/ai/m4/stabilize_ai_route_and_spend_truth";

/// Repo-relative path of the checked AI route/spend truth export.
pub const AI_ROUTE_SPEND_TRUTH_ARTIFACT_REF: &str =
    "artifacts/ai/m4/stabilize_ai_route_and_spend_truth/support_export.json";

/// Repo-relative path of the checked AI route/spend truth Markdown summary.
pub const AI_ROUTE_SPEND_TRUTH_SUMMARY_REF: &str =
    "artifacts/ai/m4/stabilize_ai_route_and_spend_truth/summary.md";

/// Coarse route class an AI action intends to use or actually ran on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteClass {
    /// Inference stays on the local device.
    Local,
    /// A user- or org-held credential reaches a vendor or self-hosted endpoint.
    Byok,
    /// A first-party managed hosted path.
    Managed,
    /// An enterprise gateway brokers the call.
    EnterpriseGateway,
}

impl RouteClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Byok => "byok",
            Self::Managed => "managed",
            Self::EnterpriseGateway => "enterprise_gateway",
        }
    }

    /// True when this route leaves the local device.
    pub const fn is_egress_route(self) -> bool {
        matches!(self, Self::Byok | Self::Managed | Self::EnterpriseGateway)
    }

    /// True when the execution locus is consistent with this route class.
    fn consistent_with_locus(self, locus: ExecutionLocusClass) -> bool {
        match self {
            Self::Local => matches!(
                locus,
                ExecutionLocusClass::LocalInProcess
                    | ExecutionLocusClass::LocalSandboxProcess
                    | ExecutionLocusClass::LocalCompanionService
            ),
            Self::Byok => matches!(
                locus,
                ExecutionLocusClass::ByokRemoteVendorDirect
                    | ExecutionLocusClass::ByokRemoteSelfHostedDirect
            ),
            Self::Managed => matches!(
                locus,
                ExecutionLocusClass::VendorHostedFirstPartyManaged
                    | ExecutionLocusClass::ExtensionProvidedLocus
            ),
            Self::EnterpriseGateway => {
                matches!(locus, ExecutionLocusClass::EnterpriseGatewayBrokered)
            }
        }
    }
}

/// Quota-family flow that owns the budget a material AI action draws from.
///
/// These flows stay distinct so a blocked action explains which budget actually
/// closed and who owns it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiActionFlowClass {
    /// Interactive composer / chat turns.
    Composer,
    /// AI review and explanation flows.
    Review,
    /// Agent or background long-running flows.
    AgentBackground,
    /// One-shot generation (edits, scaffolds, completions).
    Generation,
    /// Tool- or connector-assisted flows that hop through an external gateway.
    ToolConnectorAssisted,
}

impl AiActionFlowClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Composer => "composer",
            Self::Review => "review",
            Self::AgentBackground => "agent_background",
            Self::Generation => "generation",
            Self::ToolConnectorAssisted => "tool_connector_assisted",
        }
    }

    /// Quota-family flows the summary must cover for a stable claim.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::Composer,
            Self::Review,
            Self::AgentBackground,
            Self::Generation,
            Self::ToolConnectorAssisted,
        ]
    }

    /// True when this flow can run as a batch or branch-agent lane.
    const fn is_batch_or_agent_lane(self) -> bool {
        matches!(self, Self::AgentBackground)
    }
}

/// Live run-strip phase for an in-flight or settled AI action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunPhaseClass {
    /// Estimate shown; nothing has dispatched.
    PreflightEstimate,
    /// The route is dispatching.
    Dispatching,
    /// The route is streaming output.
    Streaming,
    /// The route is settling (final usage being recorded).
    Settling,
    /// The run has settled and a receipt is available.
    Settled,
}

impl RunPhaseClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreflightEstimate => "preflight_estimate",
            Self::Dispatching => "dispatching",
            Self::Streaming => "streaming",
            Self::Settling => "settling",
            Self::Settled => "settled",
        }
    }
}

/// Terminal outcome class carried by the post-run receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunOutcomeClass {
    /// The action completed on its originally intended route.
    CompletedClean,
    /// The action completed after a disclosed route downgrade.
    CompletedWithDowngrade,
    /// The action failed after dispatch.
    FailedAfterDispatch,
    /// The action was blocked before dispatch (quota, policy, route).
    BlockedBeforeDispatch,
    /// The user cancelled the action.
    CancelledByUser,
}

impl RunOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompletedClean => "completed_clean",
            Self::CompletedWithDowngrade => "completed_with_downgrade",
            Self::FailedAfterDispatch => "failed_after_dispatch",
            Self::BlockedBeforeDispatch => "blocked_before_dispatch",
            Self::CancelledByUser => "cancelled_by_user",
        }
    }

    /// True when the action actually dispatched against a route.
    const fn dispatched(self) -> bool {
        matches!(
            self,
            Self::CompletedClean | Self::CompletedWithDowngrade | Self::FailedAfterDispatch
        )
    }

    /// True when the outcome reflects a disclosed downgrade.
    const fn is_downgrade_outcome(self) -> bool {
        matches!(self, Self::CompletedWithDowngrade)
    }
}

/// How the cost class in a receipt was established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostMeasurementClass {
    /// The cost band is a preflight estimate only.
    EstimateBand,
    /// The cost band reflects a measured post-run usage record.
    ActualMeasured,
    /// The cost band is the actual class but the amount was not verified.
    ActualUnverified,
}

impl CostMeasurementClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EstimateBand => "estimate_band",
            Self::ActualMeasured => "actual_measured",
            Self::ActualUnverified => "actual_unverified",
        }
    }

    /// True when the measurement reflects a real post-run record.
    const fn is_actual(self) -> bool {
        matches!(self, Self::ActualMeasured | Self::ActualUnverified)
    }
}

/// Scarce local-resource class surfaced so `Local` is never implied free.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalResourceClass {
    /// Wall-clock time the local run consumes.
    WallTime,
    /// Resident memory the local run consumes.
    Memory,
    /// Battery the local run consumes.
    Battery,
    /// Accelerator (GPU/NPU) the local run consumes.
    Accelerator,
}

impl LocalResourceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WallTime => "wall_time",
            Self::Memory => "memory",
            Self::Battery => "battery",
            Self::Accelerator => "accelerator",
        }
    }
}

/// Coarse cost band for a scarce local resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceCostBandClass {
    /// No meaningful cost.
    Negligible,
    /// Low cost.
    Low,
    /// Moderate cost.
    Moderate,
    /// High cost.
    High,
    /// The resource is constrained and may degrade or block the run.
    Constrained,
}

impl ResourceCostBandClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Negligible => "negligible",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Constrained => "constrained",
        }
    }
}

/// Stable-qualification posture for a visible batch/agent lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableQualificationClass {
    /// Lane qualifies for the Stable line.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental.
    Experimental,
    /// Lane is not available.
    Unavailable,
}

impl StableQualificationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
        }
    }

    const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Typed provider/model/external-tool registry resolution for the action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteRegistryResolution {
    /// Whether the route resolved against the typed registry.
    pub resolved: bool,
    /// Provider registry id.
    pub provider_id: String,
    /// Provider review-safe label.
    pub provider_label: String,
    /// Model registry id.
    pub model_id: String,
    /// Model version aligned with on-screen review.
    pub model_version: String,
    /// Model review-safe label.
    pub model_label: String,
    /// Transport class for this route.
    pub transport_class: RegistryTransportClass,
    /// Auth mode for this route.
    pub auth_mode: RegistryAuthModeClass,
    /// Retention posture for this route.
    pub retention_posture: RetentionStanceClass,
    /// Region posture for this route.
    pub region_posture: RegionPostureClass,
    /// Quota family (billing structure) for this route.
    pub quota_family: QuotaFamilyClass,
    /// Execution locus for the selected model path.
    pub execution_locus: ExecutionLocusClass,
    /// Local-model-pack provenance ref (required when the route stays local).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_model_pack_provenance_ref: Option<String>,
    /// External-tool / MCP gateway hop locus refs disclosed for this action.
    pub external_tool_locus_refs: Vec<String>,
}

/// Preflight estimate card shown before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreflightEstimateCard {
    /// Whether the estimate card was shown before send.
    pub shown_before_send: bool,
    /// Route class the action intends to use.
    pub intended_route_class: RouteClass,
    /// Estimated cost band before send.
    pub estimated_cost_band: CostEnvelopeClass,
    /// Estimated latency band before send.
    pub estimated_latency_band: LatencyEnvelopeClass,
    /// Quota family flow the estimate draws from.
    pub quota_family_flow: AiActionFlowClass,
    /// Quota family (billing structure) the estimate draws from.
    pub quota_family: QuotaFamilyClass,
    /// Scarce local-resource cost rows (required when the route is local).
    pub local_resource_costs: Vec<LocalResourceCostRow>,
    /// Review-safe approval note (e.g. one-time egress approval).
    pub approval_note_label: String,
    /// Review-safe policy note.
    pub policy_note_label: String,
}

/// One scarce local-resource cost row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalResourceCostRow {
    /// Scarce resource class.
    pub resource_class: LocalResourceClass,
    /// Coarse cost band for this resource.
    pub cost_band: ResourceCostBandClass,
}

/// Live run strip carrying current route truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LiveRunStrip {
    /// Whether the live strip is present for this action.
    pub present: bool,
    /// Current run phase.
    pub phase: RunPhaseClass,
    /// Route class currently in use.
    pub current_route_class: RouteClass,
    /// Provider label currently in use.
    pub current_provider_label: String,
    /// Model label currently in use.
    pub current_model_label: String,
    /// Whether the route is disclosed live (never hidden).
    pub route_disclosed: bool,
}

/// Post-run receipt carried after completion or failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostRunReceipt {
    /// Whether the receipt is present.
    pub present: bool,
    /// Terminal outcome class.
    pub outcome: RunOutcomeClass,
    /// Route class the action actually ran on.
    pub actual_route_class: RouteClass,
    /// Actual cost band class.
    pub actual_cost_band: CostEnvelopeClass,
    /// How the cost class was established.
    pub cost_measurement: CostMeasurementClass,
    /// Quota family flow the receipt drew from.
    pub quota_family_flow: AiActionFlowClass,
    /// Quota family (billing structure) the receipt drew from.
    pub quota_family: QuotaFamilyClass,
    /// Bound provider-route-receipt ref.
    pub route_receipt_ref: String,
    /// Bound spend-receipt ref.
    pub spend_receipt_ref: String,
}

/// One distinct quota-family summary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaSummaryRow {
    /// Quota family flow this row covers.
    pub flow_class: AiActionFlowClass,
    /// Quota family (billing structure) for this flow.
    pub quota_family: QuotaFamilyClass,
    /// Owner scope for this quota.
    pub quota_scope: QuotaScopeClass,
    /// Current quota state.
    pub quota_state: QuotaStateClass,
    /// Review-safe budget-owner label (who owns this budget).
    pub budget_owner_label: String,
    /// Whether this quota family blocked the current action.
    pub blocked_this_action: bool,
}

/// Route-downgrade banner preserving both the original and current routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteDowngradeBanner {
    /// Whether the action was downgraded.
    pub downgraded: bool,
    /// Cause of the downgrade.
    pub cause: RouteChangeCauseClass,
    /// Original route class Aureline intended to use.
    pub original_route_class: RouteClass,
    /// Original provider label.
    pub original_provider_label: String,
    /// Original model label.
    pub original_model_label: String,
    /// Current route class that actually ran.
    pub current_route_class: RouteClass,
    /// Current provider label.
    pub current_provider_label: String,
    /// Current model label.
    pub current_model_label: String,
    /// Whether both routes are preserved and visible.
    pub both_routes_preserved: bool,
    /// Whether the route was switched silently (must be false).
    pub silent_switch: bool,
    /// Route-change disclosure ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
}

/// Non-AI fallback path that always remains when AI closes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NonAiFallbackPath {
    /// Whether a non-AI fallback is available.
    pub available: bool,
    /// Review-safe fallback label.
    pub fallback_label: String,
    /// Command ref for the manual path.
    pub fallback_command_ref: String,
    /// Whether the fallback is reachable without any AI route.
    pub reachable_without_ai: bool,
}

/// Cumulative-spend posture for a visible batch or branch-agent lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CumulativeSpendPosture {
    /// Whether this batch/agent lane is visible on the stable line.
    pub lane_visible: bool,
    /// Review-safe lane label.
    pub lane_label: String,
    /// Whether a cumulative receipt is available for the lane.
    pub cumulative_receipt_available: bool,
    /// Cumulative spend band across the lane.
    pub cumulative_spend_band: CostEnvelopeClass,
    /// Remaining-budget band for the lane.
    pub remaining_budget_band: CostEnvelopeClass,
    /// Number of hops covered by the cumulative rollup.
    pub hop_count: u32,
    /// Stable-qualification posture for this lane.
    pub qualification: StableQualificationClass,
    /// Whether this lane claims the Stable line.
    pub claimed_stable: bool,
}

/// Exportable evidence lineage binding the in-product evidence id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteSpendEvidenceExport {
    /// Evidence id shown in-product and reused by admin/support exports.
    pub evidence_id: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Admin inspector ref that resolves the same evidence id.
    pub admin_inspector_ref: String,
    /// Support export ref that resolves the same evidence id.
    pub support_export_ref: String,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// Constructor input for [`AiRouteSpendTruthPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiRouteSpendTruthPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical action id shared across surfaces and receipts.
    pub action_id: String,
    /// Display label.
    pub display_label: String,
    /// Quota family flow that owns this action.
    pub action_flow_class: AiActionFlowClass,
    /// Whether this is a material AI action (estimate/live/receipt required).
    pub material_action: bool,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this action was evaluated under.
    pub policy_epoch_ref: String,
    /// Typed registry resolution.
    pub registry_resolution: RouteRegistryResolution,
    /// Preflight estimate card.
    pub preflight: PreflightEstimateCard,
    /// Live run strip.
    pub live_run: LiveRunStrip,
    /// Post-run receipt.
    pub receipt: PostRunReceipt,
    /// Distinct quota-family summary rows.
    pub quota_summary: Vec<QuotaSummaryRow>,
    /// Route-downgrade banner.
    pub downgrade: RouteDowngradeBanner,
    /// Non-AI fallback path.
    pub fallback: NonAiFallbackPath,
    /// Cumulative-spend posture (required when a batch/agent lane is visible).
    pub cumulative_spend: Option<CumulativeSpendPosture>,
    /// Exportable evidence lineage.
    pub evidence_export: RouteSpendEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI route/spend truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRouteSpendTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical action id shared across surfaces and receipts.
    pub action_id: String,
    /// Display label.
    pub display_label: String,
    /// Quota family flow that owns this action.
    pub action_flow_class: AiActionFlowClass,
    /// Whether this is a material AI action.
    pub material_action: bool,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this action was evaluated under.
    pub policy_epoch_ref: String,
    /// Typed registry resolution.
    pub registry_resolution: RouteRegistryResolution,
    /// Preflight estimate card.
    pub preflight: PreflightEstimateCard,
    /// Live run strip.
    pub live_run: LiveRunStrip,
    /// Post-run receipt.
    pub receipt: PostRunReceipt,
    /// Distinct quota-family summary rows.
    pub quota_summary: Vec<QuotaSummaryRow>,
    /// Route-downgrade banner.
    pub downgrade: RouteDowngradeBanner,
    /// Non-AI fallback path.
    pub fallback: NonAiFallbackPath,
    /// Cumulative-spend posture for a visible batch/agent lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cumulative_spend: Option<CumulativeSpendPosture>,
    /// Exportable evidence lineage.
    pub evidence_export: RouteSpendEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiRouteSpendTruthPacket {
    /// Builds an AI route/spend truth packet from the stable-lane input.
    pub fn new(input: AiRouteSpendTruthPacketInput) -> Self {
        Self {
            record_kind: AI_ROUTE_SPEND_TRUTH_RECORD_KIND.to_owned(),
            schema_version: AI_ROUTE_SPEND_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            action_id: input.action_id,
            display_label: input.display_label,
            action_flow_class: input.action_flow_class,
            material_action: input.material_action,
            claimed_stable: input.claimed_stable,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            registry_resolution: input.registry_resolution,
            preflight: input.preflight,
            live_run: input.live_run,
            receipt: input.receipt,
            quota_summary: input.quota_summary,
            downgrade: input.downgrade,
            fallback: input.fallback,
            cumulative_spend: input.cumulative_spend,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the route/spend truth packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiRouteSpendTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_ROUTE_SPEND_TRUTH_RECORD_KIND {
            violations.push(AiRouteSpendTruthViolation::WrongRecordKind);
        }
        if self.schema_version != AI_ROUTE_SPEND_TRUTH_SCHEMA_VERSION {
            violations.push(AiRouteSpendTruthViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.action_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiRouteSpendTruthViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_registry_resolution(self, &mut violations);
        validate_preflight(self, &mut violations);
        validate_live_and_receipt(self, &mut violations);
        validate_quota_summary(self, &mut violations);
        validate_downgrade(self, &mut violations);
        validate_fallback(self, &mut violations);
        validate_cumulative_spend(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai route/spend truth packet serializes"),
        ) {
            violations.push(AiRouteSpendTruthViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai route/spend truth packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let blocked_families = self
            .quota_summary
            .iter()
            .filter(|row| row.blocked_this_action)
            .count();
        let mut out = String::new();
        out.push_str("# AI Route and Spend Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Action id: `{}`\n", self.action_id));
        out.push_str(&format!("- Evidence id: `{}`\n", self.evidence_export.evidence_id));
        out.push_str(&format!(
            "- Flow: `{}` (material: {})\n",
            self.action_flow_class.as_str(),
            self.material_action
        ));
        out.push_str(&format!(
            "- Intended route: `{}` / `{}` (`{}`)\n",
            self.preflight.intended_route_class.as_str(),
            self.registry_resolution.provider_label,
            self.registry_resolution.model_label
        ));
        out.push_str(&format!(
            "- Actual route: `{}` (outcome `{}`, cost `{}` / `{}`)\n",
            self.receipt.actual_route_class.as_str(),
            self.receipt.outcome.as_str(),
            self.receipt.actual_cost_band.as_str(),
            self.receipt.cost_measurement.as_str()
        ));
        out.push_str(&format!(
            "- Downgrade: {} (cause `{}`, both routes preserved: {})\n",
            self.downgrade.downgraded,
            self.downgrade.cause.as_str(),
            self.downgrade.both_routes_preserved
        ));
        out.push_str(&format!(
            "- Quota families: {} rows ({} blocked this action)\n",
            self.quota_summary.len(),
            blocked_families
        ));
        out.push_str(&format!(
            "- Non-AI fallback available: {} (`{}`)\n",
            self.fallback.available, self.fallback.fallback_label
        ));
        if let Some(cumulative) = &self.cumulative_spend {
            out.push_str(&format!(
                "- Batch/agent lane: `{}` cumulative receipt available: {} ({})\n",
                cumulative.lane_label,
                cumulative.cumulative_receipt_available,
                cumulative.qualification.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in route/spend truth export.
#[derive(Debug)]
pub enum AiRouteSpendTruthArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiRouteSpendTruthViolation>),
}

impl fmt::Display for AiRouteSpendTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "ai route/spend truth export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai route/spend truth export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiRouteSpendTruthArtifactError {}

/// Validation failures emitted by [`AiRouteSpendTruthPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiRouteSpendTruthViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The registry resolution is incomplete.
    RegistryResolutionIncomplete,
    /// The route class is inconsistent with the resolved execution locus.
    RouteLocusInconsistent,
    /// A local route did not disclose local-model-pack provenance.
    LocalModelPackProvenanceMissing,
    /// The preflight estimate card was not shown before send.
    EstimateNotShownBeforeSend,
    /// A local route did not surface scarce local-resource cost classes.
    LocalResourceCostMissing,
    /// The live run strip is absent or did not disclose route truth.
    LiveRouteTruthMissing,
    /// The post-run receipt is absent or incomplete after completion/failure.
    PostRunReceiptMissing,
    /// A quota family flow is not covered by the summary.
    QuotaFamilyCoverageMissing,
    /// The owning flow's quota summary row is missing or inconsistent.
    QuotaSummaryInconsistent,
    /// A downgrade did not preserve both the original and current routes.
    DowngradeRoutesNotPreserved,
    /// A route was switched silently.
    SilentRouteSwitch,
    /// A non-AI fallback path is not available (AI-only dead end).
    NonAiFallbackMissing,
    /// A visible batch/agent lane lacks cumulative receipt truth but claims Stable.
    CumulativeSpendTruthMissing,
    /// Evidence export refs or the shared evidence id are missing.
    EvidenceExportRefsMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiRouteSpendTruthViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RegistryResolutionIncomplete => "registry_resolution_incomplete",
            Self::RouteLocusInconsistent => "route_locus_inconsistent",
            Self::LocalModelPackProvenanceMissing => "local_model_pack_provenance_missing",
            Self::EstimateNotShownBeforeSend => "estimate_not_shown_before_send",
            Self::LocalResourceCostMissing => "local_resource_cost_missing",
            Self::LiveRouteTruthMissing => "live_route_truth_missing",
            Self::PostRunReceiptMissing => "post_run_receipt_missing",
            Self::QuotaFamilyCoverageMissing => "quota_family_coverage_missing",
            Self::QuotaSummaryInconsistent => "quota_summary_inconsistent",
            Self::DowngradeRoutesNotPreserved => "downgrade_routes_not_preserved",
            Self::SilentRouteSwitch => "silent_route_switch",
            Self::NonAiFallbackMissing => "non_ai_fallback_missing",
            Self::CumulativeSpendTruthMissing => "cumulative_spend_truth_missing",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in AI route/spend truth export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_ai_route_spend_truth_export(
) -> Result<AiRouteSpendTruthPacket, AiRouteSpendTruthArtifactError> {
    let packet: AiRouteSpendTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/stabilize_ai_route_and_spend_truth/support_export.json"
    )))
    .map_err(AiRouteSpendTruthArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiRouteSpendTruthArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    for required in [
        AI_ROUTE_SPEND_TRUTH_AI_DOC_REF,
        AI_ROUTE_SPEND_TRUTH_RECEIPT_CONTRACT_REF,
        AI_ROUTE_SPEND_TRUTH_BUDGET_CONTRACT_REF,
        AI_ROUTE_SPEND_TRUTH_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiRouteSpendTruthViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_registry_resolution(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let registry = &packet.registry_resolution;
    // A stable invocation can only claim a route once the typed registry
    // resolves provider/model identity with all of its posture classes.
    if !registry.resolved
        || registry.provider_id.trim().is_empty()
        || registry.provider_label.trim().is_empty()
        || registry.model_id.trim().is_empty()
        || registry.model_version.trim().is_empty()
        || registry.model_label.trim().is_empty()
    {
        violations.push(AiRouteSpendTruthViolation::RegistryResolutionIncomplete);
    }
    if registry
        .external_tool_locus_refs
        .iter()
        .any(|reference| reference.trim().is_empty())
    {
        violations.push(AiRouteSpendTruthViolation::RegistryResolutionIncomplete);
    }

    let intended = packet.preflight.intended_route_class;
    // The route class and the resolved execution locus must agree so a hosted
    // hop can never hide behind a local label and vice versa.
    if !intended.consistent_with_locus(registry.execution_locus) {
        violations.push(AiRouteSpendTruthViolation::RouteLocusInconsistent);
    }

    // Local routes must disclose the model-pack provenance; a local pack stays
    // visible rather than implying a free, consequence-free run.
    if intended == RouteClass::Local
        && !registry
            .local_model_pack_provenance_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        violations.push(AiRouteSpendTruthViolation::LocalModelPackProvenanceMissing);
    }
}

fn validate_preflight(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let preflight = &packet.preflight;
    // Material actions must show the estimate before send.
    if packet.material_action && !preflight.shown_before_send {
        violations.push(AiRouteSpendTruthViolation::EstimateNotShownBeforeSend);
    }
    if preflight.approval_note_label.trim().is_empty()
        || preflight.policy_note_label.trim().is_empty()
    {
        violations.push(AiRouteSpendTruthViolation::EstimateNotShownBeforeSend);
    }

    // Local routes are scarce too: time, memory, battery, or accelerator cost
    // classes must be surfaced instead of implying `Local` is free.
    if preflight.intended_route_class == RouteClass::Local {
        let has_band = preflight
            .local_resource_costs
            .iter()
            .any(|row| !matches!(row.cost_band, ResourceCostBandClass::Negligible));
        if preflight.local_resource_costs.is_empty() || !has_band {
            violations.push(AiRouteSpendTruthViolation::LocalResourceCostMissing);
        }
    }
}

fn validate_live_and_receipt(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    // Material actions must show live-route truth while in flight.
    if packet.material_action && (!packet.live_run.present || !packet.live_run.route_disclosed) {
        violations.push(AiRouteSpendTruthViolation::LiveRouteTruthMissing);
    }
    if packet.live_run.present
        && (packet.live_run.current_provider_label.trim().is_empty()
            || packet.live_run.current_model_label.trim().is_empty())
    {
        violations.push(AiRouteSpendTruthViolation::LiveRouteTruthMissing);
    }

    let receipt = &packet.receipt;
    // A material action that dispatched (completed or failed) must carry a
    // post-run receipt with route and spend refs.
    let requires_receipt = packet.material_action && receipt.outcome.dispatched();
    if requires_receipt
        && (!receipt.present
            || receipt.route_receipt_ref.trim().is_empty()
            || receipt.spend_receipt_ref.trim().is_empty())
    {
        violations.push(AiRouteSpendTruthViolation::PostRunReceiptMissing);
    }
    // A present receipt that reflects a dispatched run must carry an actual
    // measurement class, not a lingering estimate.
    if receipt.present && receipt.outcome.dispatched() && !receipt.cost_measurement.is_actual() {
        violations.push(AiRouteSpendTruthViolation::PostRunReceiptMissing);
    }
}

fn validate_quota_summary(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    // Quota families stay distinct: the summary must cover every flow.
    for required in AiActionFlowClass::required_coverage() {
        if !packet
            .quota_summary
            .iter()
            .any(|row| row.flow_class == required)
        {
            violations.push(AiRouteSpendTruthViolation::QuotaFamilyCoverageMissing);
            break;
        }
    }

    for row in &packet.quota_summary {
        if row.budget_owner_label.trim().is_empty() {
            violations.push(AiRouteSpendTruthViolation::QuotaSummaryInconsistent);
            break;
        }
    }

    // The owning flow's row must exist and, if it blocked the action, the
    // outcome and downgrade/fallback story must agree.
    let owning = packet
        .quota_summary
        .iter()
        .find(|row| row.flow_class == packet.action_flow_class);
    match owning {
        None => violations.push(AiRouteSpendTruthViolation::QuotaSummaryInconsistent),
        Some(row) => {
            let blocked_outcome = matches!(packet.receipt.outcome, RunOutcomeClass::BlockedBeforeDispatch);
            // A row that blocked the owning action must line up with a blocked
            // outcome or a disclosed downgrade away from the exhausted route.
            if row.blocked_this_action
                && !blocked_outcome
                && !packet.downgrade.downgraded
            {
                violations.push(AiRouteSpendTruthViolation::QuotaSummaryInconsistent);
            }
        }
    }
}

fn validate_downgrade(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let downgrade = &packet.downgrade;
    // Silent route switching is forbidden in every case.
    if downgrade.silent_switch {
        violations.push(AiRouteSpendTruthViolation::SilentRouteSwitch);
    }

    if downgrade.downgraded {
        let original_present = !downgrade.original_provider_label.trim().is_empty()
            && !downgrade.original_model_label.trim().is_empty();
        let current_present = !downgrade.current_provider_label.trim().is_empty()
            && !downgrade.current_model_label.trim().is_empty();
        let disclosure_present = downgrade
            .disclosure_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty());
        // A downgrade must preserve both routes, name a real cause, and disclose.
        if !downgrade.both_routes_preserved
            || !original_present
            || !current_present
            || !disclosure_present
            || downgrade.cause == RouteChangeCauseClass::NoRouteChange
        {
            violations.push(AiRouteSpendTruthViolation::DowngradeRoutesNotPreserved);
        }
        // A downgraded run must reflect the downgrade in its receipt outcome and
        // its current route must match what actually ran.
        if !packet.receipt.outcome.is_downgrade_outcome()
            && packet.receipt.outcome.dispatched()
        {
            violations.push(AiRouteSpendTruthViolation::DowngradeRoutesNotPreserved);
        }
        if downgrade.current_route_class != packet.receipt.actual_route_class
            && packet.receipt.outcome.dispatched()
        {
            violations.push(AiRouteSpendTruthViolation::DowngradeRoutesNotPreserved);
        }
    } else if downgrade.cause != RouteChangeCauseClass::NoRouteChange {
        // No downgrade means no route-change cause.
        violations.push(AiRouteSpendTruthViolation::DowngradeRoutesNotPreserved);
    }
}

fn validate_fallback(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let fallback = &packet.fallback;
    // Users must never be stranded in an AI-only dead end.
    if !fallback.available
        || !fallback.reachable_without_ai
        || fallback.fallback_label.trim().is_empty()
        || fallback.fallback_command_ref.trim().is_empty()
    {
        violations.push(AiRouteSpendTruthViolation::NonAiFallbackMissing);
    }
}

fn validate_cumulative_spend(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let Some(cumulative) = &packet.cumulative_spend else {
        // A batch/agent flow claiming Stable must carry a cumulative posture.
        if packet.claimed_stable && packet.action_flow_class.is_batch_or_agent_lane() {
            violations.push(AiRouteSpendTruthViolation::CumulativeSpendTruthMissing);
        }
        return;
    };

    // A visible batch/agent lane that claims Stable must show cumulative receipt
    // truth; if it cannot, it must narrow below Stable rather than implying it.
    if cumulative.lane_visible
        && cumulative.claimed_stable
        && (!cumulative.cumulative_receipt_available || !cumulative.qualification.is_stable())
    {
        violations.push(AiRouteSpendTruthViolation::CumulativeSpendTruthMissing);
    }
    // A lane without cumulative receipt truth must not claim Stable.
    if !cumulative.cumulative_receipt_available && cumulative.qualification.is_stable() {
        violations.push(AiRouteSpendTruthViolation::CumulativeSpendTruthMissing);
    }
    if cumulative.lane_label.trim().is_empty() {
        violations.push(AiRouteSpendTruthViolation::CumulativeSpendTruthMissing);
    }
}

fn validate_evidence_export(
    packet: &AiRouteSpendTruthPacket,
    violations: &mut Vec<AiRouteSpendTruthViolation>,
) {
    let export = &packet.evidence_export;
    // The shared evidence id is the join key admin/support exports reconstruct
    // the same run from.
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(AiRouteSpendTruthViolation::EvidenceExportRefsMissing);
    }
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
    // The typed `byok_api_key` auth-mode token is a controlled enum value, not raw
    // credential material, so it must not trip the credential scan below.
    if value == RegistryAuthModeClass::ByokApiKey.as_str() {
        return false;
    }
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}

#[cfg(test)]
mod tests;
