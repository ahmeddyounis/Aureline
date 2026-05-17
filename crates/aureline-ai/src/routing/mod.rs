//! Provider routing, quota explainability, and latency/cost envelope alpha.
//!
//! This module owns the first canonical routing packet for claimed hosted
//! model paths. The packet is intentionally narrower than the full provider
//! arbitration service described by the cross-tool schemas: it records the
//! provider/model route that would be used, the quota state and budget owner
//! that gate it, the latency/cost envelope the user should expect, and the
//! visible route-change lineage when policy or fallback selects a different
//! path.
//!
//! The packet is safe for support/export use by construction. It carries
//! opaque refs, enum tokens, and short reviewable labels only. It never stores
//! raw provider URLs, raw endpoint hostnames, raw credentials, raw provider
//! payloads, exact token counts, exact cost amounts, or billing account ids.

use serde::{Deserialize, Serialize};

use aureline_runtime::ExecutionContext;

/// Stable record-kind tag carried on serialized [`AiRoutingPacket`] payloads.
pub const AI_ROUTING_PACKET_RECORD_KIND: &str = "ai_routing_cost_alpha_packet_record";

/// Stable record-kind tag carried on serialized [`AiRoutingSupportPacket`] payloads.
pub const AI_ROUTING_SUPPORT_PACKET_RECORD_KIND: &str =
    "ai_routing_cost_alpha_support_packet_record";

/// Schema version of the alpha routing packet and support projection.
pub const AI_ROUTING_SCHEMA_VERSION: u32 = 1;

/// Run-state class re-exported from the provider-route receipt contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutingRunStateClass {
    /// Minted before the request leaves the device.
    PreviewPreDispatch,
    /// The request completed and bytes were returned.
    PostRunCompleted,
    /// The request dispatched but failed without usable bytes.
    PostRunFailed,
    /// The user cancelled the request.
    CancelledByUser,
    /// Policy cancelled the request.
    CancelledByPolicy,
    /// A budget cap denied dispatch.
    BudgetBlockedRefusal,
    /// No admitted route existed.
    RouteBlockedRefusal,
}

impl RoutingRunStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPreDispatch => "preview_pre_dispatch",
            Self::PostRunCompleted => "post_run_completed",
            Self::PostRunFailed => "post_run_failed",
            Self::CancelledByUser => "cancelled_by_user",
            Self::CancelledByPolicy => "cancelled_by_policy",
            Self::BudgetBlockedRefusal => "budget_blocked_refusal",
            Self::RouteBlockedRefusal => "route_blocked_refusal",
        }
    }
}

/// Provider class re-exported from the provider registry contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRouteProviderClass {
    /// Customer-operated first-party provider endpoint.
    FirstPartySelfHosted,
    /// First-party-operated managed provider relationship.
    FirstPartyManaged,
    /// User- or org-connected vendor provider.
    ConnectedProviderVendor,
    /// User- or org-connected self-hosted provider.
    ConnectedProviderSelfHosted,
    /// Extension-authored provider.
    ExtensionProvidedProvider,
    /// Fixture-only mocked provider.
    MockedTestProvider,
    /// Disabled row that must not dispatch.
    DisabledNoProvider,
}

impl AiRouteProviderClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartySelfHosted => "first_party_self_hosted",
            Self::FirstPartyManaged => "first_party_managed",
            Self::ConnectedProviderVendor => "connected_provider_vendor",
            Self::ConnectedProviderSelfHosted => "connected_provider_self_hosted",
            Self::ExtensionProvidedProvider => "extension_provided_provider",
            Self::MockedTestProvider => "mocked_test_provider",
            Self::DisabledNoProvider => "disabled_no_provider",
        }
    }
}

/// Execution locus for the selected model path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionLocusClass {
    /// Inference runs in the Aureline process.
    LocalInProcess,
    /// Inference runs in a local sandboxed process or container.
    LocalSandboxProcess,
    /// Inference runs in a local companion service.
    LocalCompanionService,
    /// BYOK calls land directly on a vendor endpoint.
    ByokRemoteVendorDirect,
    /// BYOK calls land directly on a self-hosted endpoint.
    ByokRemoteSelfHostedDirect,
    /// Calls are brokered through an enterprise gateway.
    EnterpriseGatewayBrokered,
    /// First-party managed calls land on a hosted vendor path.
    VendorHostedFirstPartyManaged,
    /// An extension owns the provider locus.
    ExtensionProvidedLocus,
    /// Fixture-only mocked locus.
    MockedTestLocus,
    /// Disabled row with no execution locus.
    DisabledNoLocus,
}

impl ExecutionLocusClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcess => "local_in_process",
            Self::LocalSandboxProcess => "local_sandbox_process",
            Self::LocalCompanionService => "local_companion_service",
            Self::ByokRemoteVendorDirect => "byok_remote_vendor_direct",
            Self::ByokRemoteSelfHostedDirect => "byok_remote_self_hosted_direct",
            Self::EnterpriseGatewayBrokered => "enterprise_gateway_brokered",
            Self::VendorHostedFirstPartyManaged => "vendor_hosted_first_party_managed",
            Self::ExtensionProvidedLocus => "extension_provided_locus",
            Self::MockedTestLocus => "mocked_test_locus",
            Self::DisabledNoLocus => "disabled_no_locus",
        }
    }

    /// True when the route leaves the local device or is owned by an external provider.
    pub const fn is_hosted_model_path(self) -> bool {
        matches!(
            self,
            Self::ByokRemoteVendorDirect
                | Self::ByokRemoteSelfHostedDirect
                | Self::EnterpriseGatewayBrokered
                | Self::VendorHostedFirstPartyManaged
                | Self::ExtensionProvidedLocus
        )
    }
}

/// Region-posture class re-exported from the provider registry contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegionPostureClass {
    /// Bytes remain on the local device.
    LocalDeviceOnly,
    /// Bytes are pinned to one region.
    SingleRegionPinned,
    /// Bytes are pinned to a named region set.
    MultiRegionPinned,
    /// Failover may happen inside a bounded region set.
    RegionFailoverBoundedSet,
    /// Provider chooses region by default.
    RegionUnpinnedVendorDefault,
    /// Region posture is unverified.
    RegionUnknownUnverified,
    /// Region posture is policy-blocked.
    RegionPolicyBlocked,
}

impl RegionPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::SingleRegionPinned => "single_region_pinned",
            Self::MultiRegionPinned => "multi_region_pinned",
            Self::RegionFailoverBoundedSet => "region_failover_bounded_set",
            Self::RegionUnpinnedVendorDefault => "region_unpinned_vendor_default",
            Self::RegionUnknownUnverified => "region_unknown_unverified",
            Self::RegionPolicyBlocked => "region_policy_blocked",
        }
    }
}

/// Retention stance re-exported from the provider registry contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionStanceClass {
    /// Local-only route with no provider retention.
    NoRetentionLocalOnly,
    /// Provider promises request/response body discard.
    NoRetentionPromisedBodyDiscarded,
    /// Provider retains bodies for bounded operator access only.
    BoundedRetentionOperatorAccessOnly,
    /// Provider offers bounded retention with user export.
    BoundedRetentionWithUserExport,
    /// Provider retains only under legal hold rules.
    BoundedRetentionWithLegalHoldOnly,
    /// Provider retains without training use.
    UnboundedRetentionNotUsedForTraining,
    /// Provider may train on retained content.
    UnboundedRetentionUsedForTraining,
    /// Retention posture is unverified.
    RetentionUnknownUnverified,
    /// Retention posture is policy-blocked.
    RetentionPolicyBlocked,
}

impl RetentionStanceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRetentionLocalOnly => "no_retention_local_only",
            Self::NoRetentionPromisedBodyDiscarded => "no_retention_promised_body_discarded",
            Self::BoundedRetentionOperatorAccessOnly => "bounded_retention_operator_access_only",
            Self::BoundedRetentionWithUserExport => "bounded_retention_with_user_export",
            Self::BoundedRetentionWithLegalHoldOnly => "bounded_retention_with_legal_hold_only",
            Self::UnboundedRetentionNotUsedForTraining => {
                "unbounded_retention_not_used_for_training"
            }
            Self::UnboundedRetentionUsedForTraining => "unbounded_retention_used_for_training",
            Self::RetentionUnknownUnverified => "retention_unknown_unverified",
            Self::RetentionPolicyBlocked => "retention_policy_blocked",
        }
    }
}

/// Quota family re-exported from the provider registry contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaFamilyClass {
    /// Local route has no hard remote quota.
    PerUserLocalNoLimit,
    /// Local route may soft-throttle.
    PerUserLocalSoftThrottle,
    /// BYOK route consumes the user's vendor quota.
    PerUserByokVendorQuota,
    /// BYOK route consumes an organisation vendor quota.
    PerUserByokOrganizationQuota,
    /// Enterprise gateway uses a pooled quota.
    EnterpriseGatewayPooledQuota,
    /// Vendor-hosted managed path uses entitlement quota.
    VendorHostedEntitlementQuota,
    /// Free tier is rate-limited.
    FreeTierRateLimited,
    /// Paid tier is metered.
    PaidTierMetered,
    /// Policy overrides quota family.
    PolicyEnforcedQuotaOverride,
    /// Quota family is unverified.
    QuotaUnknownUnverified,
}

impl QuotaFamilyClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUserLocalNoLimit => "per_user_local_no_limit",
            Self::PerUserLocalSoftThrottle => "per_user_local_soft_throttle",
            Self::PerUserByokVendorQuota => "per_user_byok_vendor_quota",
            Self::PerUserByokOrganizationQuota => "per_user_byok_organization_quota",
            Self::EnterpriseGatewayPooledQuota => "enterprise_gateway_pooled_quota",
            Self::VendorHostedEntitlementQuota => "vendor_hosted_entitlement_quota",
            Self::FreeTierRateLimited => "free_tier_rate_limited",
            Self::PaidTierMetered => "paid_tier_metered",
            Self::PolicyEnforcedQuotaOverride => "policy_enforced_quota_override",
            Self::QuotaUnknownUnverified => "quota_unknown_unverified",
        }
    }
}

/// Current quota state shown to users and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaStateClass {
    /// Quota is available for the selected route.
    WithinLimit,
    /// Quota is close to a limit and should warn.
    Warning,
    /// Quota is exhausted and must deny or fallback.
    Exhausted,
    /// A grace window is active.
    Grace,
    /// Policy pauses this quota family.
    PausedByPolicy,
    /// Cached quota state is stale but labeled.
    Stale,
    /// Quota state is not verified.
    UnknownUnverified,
}

impl QuotaStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinLimit => "within_limit",
            Self::Warning => "warning",
            Self::Exhausted => "exhausted",
            Self::Grace => "grace",
            Self::PausedByPolicy => "paused_by_policy",
            Self::Stale => "stale",
            Self::UnknownUnverified => "unknown_unverified",
        }
    }

    /// True when new hosted dispatch should be blocked unless a fallback route is selected.
    pub const fn blocks_hosted_dispatch(self) -> bool {
        matches!(
            self,
            Self::Exhausted | Self::PausedByPolicy | Self::UnknownUnverified
        )
    }
}

/// Owner scope for the quota inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotaScopeClass {
    /// Personal quota.
    Personal,
    /// Workspace quota.
    Workspace,
    /// Organisation quota.
    Organisation,
    /// Deployment-profile quota.
    DeploymentProfile,
    /// Vendor-owned BYOK quota.
    ByokProvider,
    /// Enterprise pool quota.
    EnterprisePool,
    /// Managed entitlement quota.
    VendorHostedEntitlement,
    /// Local device quota.
    LocalDevice,
}

impl QuotaScopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Personal => "personal",
            Self::Workspace => "workspace",
            Self::Organisation => "organisation",
            Self::DeploymentProfile => "deployment_profile",
            Self::ByokProvider => "byok_provider",
            Self::EnterprisePool => "enterprise_pool",
            Self::VendorHostedEntitlement => "vendor_hosted_entitlement",
            Self::LocalDevice => "local_device",
        }
    }
}

/// Cost-visibility class re-exported from the context assembly contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostVisibilityClass {
    /// Provider charges per request.
    MeteredPerRequest,
    /// Provider charges per token.
    MeteredPerToken,
    /// Cost is covered by a flat subscription.
    FlatFeeSubscription,
    /// Cost is bundled with no incremental charge.
    BundledNoIncrementalCost,
    /// Cost estimate is not verified.
    EstimatedUnverified,
    /// Provider does not disclose cost.
    UndisclosedByProvider,
}

impl CostVisibilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MeteredPerRequest => "metered_per_request",
            Self::MeteredPerToken => "metered_per_token",
            Self::FlatFeeSubscription => "flat_fee_subscription",
            Self::BundledNoIncrementalCost => "bundled_no_incremental_cost",
            Self::EstimatedUnverified => "estimated_unverified",
            Self::UndisclosedByProvider => "undisclosed_by_provider",
        }
    }
}

/// Coarse cost envelope the route is admitted under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CostEnvelopeClass {
    /// Bundled with no incremental cost.
    BundledNoIncrementalCost,
    /// Free tier with rate limiting.
    FreeTierRateLimited,
    /// Low-volume per-request band.
    MeteredPerRequestLowVolumeBand,
    /// Medium-volume per-request band.
    MeteredPerRequestMediumVolumeBand,
    /// High-volume per-request band.
    MeteredPerRequestHighVolumeBand,
    /// Low-volume per-token band.
    MeteredPerTokenLowVolumeBand,
    /// Medium-volume per-token band.
    MeteredPerTokenMediumVolumeBand,
    /// High-volume per-token band.
    MeteredPerTokenHighVolumeBand,
    /// Flat subscription band.
    FlatFeeSubscriptionBand,
    /// Enterprise pooled quota band.
    EnterprisePooledQuotaBand,
    /// Vendor-hosted entitlement band.
    VendorHostedEntitlementBand,
    /// Estimate exists but is unverified.
    EstimatedUnverifiedBand,
    /// Cost envelope is unknown.
    EnvelopeUnknownUnverifiedCost,
}

impl CostEnvelopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledNoIncrementalCost => "bundled_no_incremental_cost",
            Self::FreeTierRateLimited => "free_tier_rate_limited",
            Self::MeteredPerRequestLowVolumeBand => "metered_per_request_low_volume_band",
            Self::MeteredPerRequestMediumVolumeBand => "metered_per_request_medium_volume_band",
            Self::MeteredPerRequestHighVolumeBand => "metered_per_request_high_volume_band",
            Self::MeteredPerTokenLowVolumeBand => "metered_per_token_low_volume_band",
            Self::MeteredPerTokenMediumVolumeBand => "metered_per_token_medium_volume_band",
            Self::MeteredPerTokenHighVolumeBand => "metered_per_token_high_volume_band",
            Self::FlatFeeSubscriptionBand => "flat_fee_subscription_band",
            Self::EnterprisePooledQuotaBand => "enterprise_pooled_quota_band",
            Self::VendorHostedEntitlementBand => "vendor_hosted_entitlement_band",
            Self::EstimatedUnverifiedBand => "estimated_unverified_band",
            Self::EnvelopeUnknownUnverifiedCost => "envelope_unknown_unverified_cost",
        }
    }

    /// True when the route admits a verified or bounded cost band.
    pub const fn is_verified(self) -> bool {
        !matches!(self, Self::EnvelopeUnknownUnverifiedCost)
    }

    /// Lower-is-cheaper rank used by budget-routing policy checks.
    pub const fn cost_rank(self) -> u16 {
        match self {
            Self::BundledNoIncrementalCost => 0,
            Self::FreeTierRateLimited => 1,
            Self::FlatFeeSubscriptionBand => 2,
            Self::VendorHostedEntitlementBand => 3,
            Self::EnterprisePooledQuotaBand => 4,
            Self::MeteredPerRequestLowVolumeBand => 10,
            Self::MeteredPerTokenLowVolumeBand => 11,
            Self::MeteredPerRequestMediumVolumeBand => 20,
            Self::MeteredPerTokenMediumVolumeBand => 21,
            Self::MeteredPerRequestHighVolumeBand => 30,
            Self::MeteredPerTokenHighVolumeBand => 31,
            Self::EstimatedUnverifiedBand => 90,
            Self::EnvelopeUnknownUnverifiedCost => 100,
        }
    }
}

/// Coarse latency envelope the route is admitted under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyEnvelopeClass {
    /// P50 under 50ms and P99 under 250ms.
    #[serde(rename = "p50_under_50ms_p99_under_250ms")]
    P50Under50MsP99Under250Ms,
    /// P50 under 250ms and P99 under 1000ms.
    #[serde(rename = "p50_under_250ms_p99_under_1000ms")]
    P50Under250MsP99Under1000Ms,
    /// P50 under 1s and P99 under 5s.
    #[serde(rename = "p50_under_1s_p99_under_5s")]
    P50Under1SP99Under5S,
    /// P50 under 5s and P99 under 30s.
    #[serde(rename = "p50_under_5s_p99_under_30s")]
    P50Under5SP99Under30S,
    /// Long-running envelope.
    #[serde(rename = "p50_over_5s_long_running")]
    P50Over5SLongRunning,
    /// Streaming route with first token under 500ms.
    #[serde(rename = "streaming_first_token_under_500ms")]
    StreamingFirstTokenUnder500Ms,
    /// Streaming route with first token over 500ms.
    #[serde(rename = "streaming_first_token_over_500ms")]
    StreamingFirstTokenOver500Ms,
    /// Interactive envelope is unspecified.
    InteractiveEnvelopeUnspecified,
    /// Background envelope is unspecified.
    BackgroundEnvelopeUnspecified,
    /// Latency envelope is unverified.
    EnvelopeUnknownUnverified,
}

impl LatencyEnvelopeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::P50Under50MsP99Under250Ms => "p50_under_50ms_p99_under_250ms",
            Self::P50Under250MsP99Under1000Ms => "p50_under_250ms_p99_under_1000ms",
            Self::P50Under1SP99Under5S => "p50_under_1s_p99_under_5s",
            Self::P50Under5SP99Under30S => "p50_under_5s_p99_under_30s",
            Self::P50Over5SLongRunning => "p50_over_5s_long_running",
            Self::StreamingFirstTokenUnder500Ms => "streaming_first_token_under_500ms",
            Self::StreamingFirstTokenOver500Ms => "streaming_first_token_over_500ms",
            Self::InteractiveEnvelopeUnspecified => "interactive_envelope_unspecified",
            Self::BackgroundEnvelopeUnspecified => "background_envelope_unspecified",
            Self::EnvelopeUnknownUnverified => "envelope_unknown_unverified",
        }
    }

    /// True when the route admits a measured or intentionally bounded latency band.
    pub const fn is_verified(self) -> bool {
        !matches!(
            self,
            Self::InteractiveEnvelopeUnspecified
                | Self::BackgroundEnvelopeUnspecified
                | Self::EnvelopeUnknownUnverified
        )
    }
}

/// Token ceiling for the selected route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenCeilingClass {
    /// Under two thousand tokens.
    #[serde(rename = "tokens_under_2k")]
    TokensUnder2K,
    /// Under eight thousand tokens.
    #[serde(rename = "tokens_under_8k")]
    TokensUnder8K,
    /// Under thirty-two thousand tokens.
    #[serde(rename = "tokens_under_32k")]
    TokensUnder32K,
    /// Under one hundred twenty-eight thousand tokens.
    #[serde(rename = "tokens_under_128k")]
    TokensUnder128K,
    /// Over one hundred twenty-eight thousand tokens.
    #[serde(rename = "tokens_over_128k")]
    TokensOver128K,
    /// No explicit ceiling is published.
    TokensNoExplicitCeiling,
    /// Token ceiling is unverified.
    TokensUnknownUnverified,
}

impl TokenCeilingClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokensUnder2K => "tokens_under_2k",
            Self::TokensUnder8K => "tokens_under_8k",
            Self::TokensUnder32K => "tokens_under_32k",
            Self::TokensUnder128K => "tokens_under_128k",
            Self::TokensOver128K => "tokens_over_128k",
            Self::TokensNoExplicitCeiling => "tokens_no_explicit_ceiling",
            Self::TokensUnknownUnverified => "tokens_unknown_unverified",
        }
    }
}

/// Tool-call ceiling for the selected route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallCeilingClass {
    /// No tool calls are admitted.
    NoToolCallsAdmitted,
    /// Under four tool calls.
    #[serde(rename = "bounded_tool_calls_under_4")]
    BoundedToolCallsUnder4,
    /// Under sixteen tool calls.
    #[serde(rename = "bounded_tool_calls_under_16")]
    BoundedToolCallsUnder16,
    /// Under sixty-four tool calls.
    #[serde(rename = "bounded_tool_calls_under_64")]
    BoundedToolCallsUnder64,
    /// Unbounded tool calls require admin approval.
    UnboundedToolCallsAdminOnly,
    /// Tool-call ceiling is unverified.
    ToolCallsUnknownUnverified,
}

impl ToolCallCeilingClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoToolCallsAdmitted => "no_tool_calls_admitted",
            Self::BoundedToolCallsUnder4 => "bounded_tool_calls_under_4",
            Self::BoundedToolCallsUnder16 => "bounded_tool_calls_under_16",
            Self::BoundedToolCallsUnder64 => "bounded_tool_calls_under_64",
            Self::UnboundedToolCallsAdminOnly => "unbounded_tool_calls_admin_only",
            Self::ToolCallsUnknownUnverified => "tool_calls_unknown_unverified",
        }
    }
}

/// Wall-time ceiling for the selected route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WallTimeCeilingClass {
    /// Under five seconds.
    #[serde(rename = "wall_time_under_5s")]
    WallTimeUnder5S,
    /// Under thirty seconds.
    #[serde(rename = "wall_time_under_30s")]
    WallTimeUnder30S,
    /// Under five minutes.
    #[serde(rename = "wall_time_under_5m")]
    WallTimeUnder5M,
    /// Under thirty minutes.
    #[serde(rename = "wall_time_under_30m")]
    WallTimeUnder30M,
    /// Long-running admin-only path.
    WallTimeLongRunningAdminOnly,
    /// Wall-time ceiling is unverified.
    WallTimeUnknownUnverified,
}

impl WallTimeCeilingClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WallTimeUnder5S => "wall_time_under_5s",
            Self::WallTimeUnder30S => "wall_time_under_30s",
            Self::WallTimeUnder5M => "wall_time_under_5m",
            Self::WallTimeUnder30M => "wall_time_under_30m",
            Self::WallTimeLongRunningAdminOnly => "wall_time_long_running_admin_only",
            Self::WallTimeUnknownUnverified => "wall_time_unknown_unverified",
        }
    }
}

/// Route origin re-exported from the provider-route receipt contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteOriginClass {
    /// Inference stayed local.
    StayedLocal,
    /// User-owned credential route.
    ByokUserCredential,
    /// Enterprise gateway route.
    EnterpriseGateway,
    /// Vendor-hosted managed route.
    VendorHostedManaged,
    /// Extension-provided route.
    ExtensionProvided,
    /// Fixture-only mocked route.
    MockedTest,
    /// No route ran.
    DisabledNoRoute,
}

impl RouteOriginClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StayedLocal => "stayed_local",
            Self::ByokUserCredential => "byok_user_credential",
            Self::EnterpriseGateway => "enterprise_gateway",
            Self::VendorHostedManaged => "vendor_hosted_managed",
            Self::ExtensionProvided => "extension_provided",
            Self::MockedTest => "mocked_test",
            Self::DisabledNoRoute => "disabled_no_route",
        }
    }
}

/// Route-selection reason re-exported from the budget contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSelectionReasonClass {
    /// Cheapest qualifying route was admitted.
    CheapestQualifyingRouteAdmitted,
    /// No cheaper qualifying route existed.
    NoCheaperQualifyingRouteExisted,
    /// A more expensive route was admitted with an override reason.
    OverrideMoreExpensiveRouteAdmitted,
    /// Fallback after the cheapest route exhausted.
    FallbackAfterCheapestExhausted,
    /// Fallback after the cheapest route was blocked.
    FallbackAfterCheapestBlocked,
    /// User explicitly chose the route.
    UserChoseSpecificRouteExplicitly,
    /// Policy pinned the route.
    PolicyPinnedSpecificRoute,
    /// Shadow route for parity only.
    ShadowRouteForParityOnly,
    /// No route admitted.
    NoRouteAdmittedDisabledWithTypedDenial,
}

impl RouteSelectionReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CheapestQualifyingRouteAdmitted => "cheapest_qualifying_route_admitted",
            Self::NoCheaperQualifyingRouteExisted => "no_cheaper_qualifying_route_existed",
            Self::OverrideMoreExpensiveRouteAdmitted => "override_more_expensive_route_admitted",
            Self::FallbackAfterCheapestExhausted => "fallback_after_cheapest_exhausted",
            Self::FallbackAfterCheapestBlocked => "fallback_after_cheapest_blocked",
            Self::UserChoseSpecificRouteExplicitly => "user_chose_specific_route_explicitly",
            Self::PolicyPinnedSpecificRoute => "policy_pinned_specific_route",
            Self::ShadowRouteForParityOnly => "shadow_route_for_parity_only",
            Self::NoRouteAdmittedDisabledWithTypedDenial => {
                "no_route_admitted_disabled_with_typed_denial"
            }
        }
    }

    /// True when the selected route must carry visible route-change lineage.
    pub const fn requires_route_change_lineage(self) -> bool {
        !matches!(
            self,
            Self::CheapestQualifyingRouteAdmitted | Self::NoCheaperQualifyingRouteExisted
        )
    }

    /// True when the selected route is a fallback path.
    pub const fn is_fallback(self) -> bool {
        matches!(
            self,
            Self::FallbackAfterCheapestExhausted | Self::FallbackAfterCheapestBlocked
        )
    }
}

/// Route-selection override reason re-exported from the budget contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteSelectionOverrideReasonClass {
    /// Cheapest route was used.
    NoOverrideCheapestWasUsed,
    /// Cheapest route failed capability check.
    CheapestRouteFailedCapabilityCheck,
    /// Cheapest route failed region posture.
    CheapestRouteFailedRegionPosture,
    /// Cheapest route failed retention stance.
    CheapestRouteFailedRetentionStance,
    /// Cheapest route failed data-class allowlist.
    CheapestRouteFailedDataClassAllowlist,
    /// Cheapest route failed quota-family check.
    CheapestRouteFailedQuotaFamilyCheck,
    /// Cheapest route failed offline posture.
    CheapestRouteFailedOfflinePosture,
    /// Cheapest route failed determinism posture.
    CheapestRouteFailedDeterminismPosture,
    /// Cheapest route failed taint posture.
    CheapestRouteFailedTaintPosture,
    /// Cheapest route failed approval posture.
    CheapestRouteFailedApprovalPosture,
    /// Cheapest route quota was exhausted.
    CheapestRouteQuotaExhausted,
    /// Cheapest route budget was exhausted.
    CheapestRouteBudgetExhausted,
    /// Cheapest route circuit is open.
    CheapestRouteCircuitOpenRecentFailures,
    /// Cheapest route is deprecated or withdrawn.
    CheapestRouteDeprecatedOrWithdrawn,
    /// Cheapest route pack is unverified or quarantined.
    CheapestRoutePackUnverifiedOrQuarantined,
    /// User choice overrides the cheapest route.
    UserExplicitChoiceOverridesCheapest,
    /// Policy pins a more expensive route.
    PolicyPinnedMoreExpensiveRoute,
    /// Shadow route is paired with live route.
    ShadowRoutePairedWithLiveRoute,
}

impl RouteSelectionOverrideReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOverrideCheapestWasUsed => "no_override_cheapest_was_used",
            Self::CheapestRouteFailedCapabilityCheck => "cheapest_route_failed_capability_check",
            Self::CheapestRouteFailedRegionPosture => "cheapest_route_failed_region_posture",
            Self::CheapestRouteFailedRetentionStance => "cheapest_route_failed_retention_stance",
            Self::CheapestRouteFailedDataClassAllowlist => {
                "cheapest_route_failed_data_class_allowlist"
            }
            Self::CheapestRouteFailedQuotaFamilyCheck => "cheapest_route_failed_quota_family_check",
            Self::CheapestRouteFailedOfflinePosture => "cheapest_route_failed_offline_posture",
            Self::CheapestRouteFailedDeterminismPosture => {
                "cheapest_route_failed_determinism_posture"
            }
            Self::CheapestRouteFailedTaintPosture => "cheapest_route_failed_taint_posture",
            Self::CheapestRouteFailedApprovalPosture => "cheapest_route_failed_approval_posture",
            Self::CheapestRouteQuotaExhausted => "cheapest_route_quota_exhausted",
            Self::CheapestRouteBudgetExhausted => "cheapest_route_budget_exhausted",
            Self::CheapestRouteCircuitOpenRecentFailures => {
                "cheapest_route_circuit_open_recent_failures"
            }
            Self::CheapestRouteDeprecatedOrWithdrawn => "cheapest_route_deprecated_or_withdrawn",
            Self::CheapestRoutePackUnverifiedOrQuarantined => {
                "cheapest_route_pack_unverified_or_quarantined"
            }
            Self::UserExplicitChoiceOverridesCheapest => "user_explicit_choice_overrides_cheapest",
            Self::PolicyPinnedMoreExpensiveRoute => "policy_pinned_more_expensive_route",
            Self::ShadowRoutePairedWithLiveRoute => "shadow_route_paired_with_live_route",
        }
    }
}

/// Exhaustion state re-exported from the budget contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExhaustionStateClass {
    /// No exhaustion affected route selection.
    NotExhaustedRouteAdmitted,
    /// Per-request budget exhausted.
    PerRequestBudgetExhausted,
    /// Per-session budget exhausted.
    PerSessionBudgetExhausted,
    /// Per-agent invocation budget exhausted.
    PerAgentInvocationBudgetExhausted,
    /// Per-workflow budget exhausted.
    PerWorkflowBudgetExhausted,
    /// Per-user budget exhausted.
    PerUserBudgetExhausted,
    /// Per-organisation budget exhausted.
    PerOrganisationBudgetExhausted,
    /// Per-deployment-profile budget exhausted.
    PerDeploymentProfileBudgetExhausted,
    /// Quota family exhausted.
    QuotaFamilyExhausted,
    /// Agent ceiling exhausted.
    AgentCeilingExhausted,
    /// Circuit is open after recent failures.
    CircuitOpenRecentFailures,
    /// Eligibility revoked by policy.
    EligibilityRevokedPolicy,
    /// Eligibility revoked by workspace trust.
    EligibilityRevokedWorkspaceTrust,
    /// Eligibility revoked by pack quarantine.
    EligibilityRevokedPackQuarantined,
    /// Eligibility revoked by provider disablement.
    EligibilityRevokedProviderDisabled,
}

impl ExhaustionStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotExhaustedRouteAdmitted => "not_exhausted_route_admitted",
            Self::PerRequestBudgetExhausted => "per_request_budget_exhausted",
            Self::PerSessionBudgetExhausted => "per_session_budget_exhausted",
            Self::PerAgentInvocationBudgetExhausted => "per_agent_invocation_budget_exhausted",
            Self::PerWorkflowBudgetExhausted => "per_workflow_budget_exhausted",
            Self::PerUserBudgetExhausted => "per_user_budget_exhausted",
            Self::PerOrganisationBudgetExhausted => "per_organisation_budget_exhausted",
            Self::PerDeploymentProfileBudgetExhausted => "per_deployment_profile_budget_exhausted",
            Self::QuotaFamilyExhausted => "quota_family_exhausted",
            Self::AgentCeilingExhausted => "agent_ceiling_exhausted",
            Self::CircuitOpenRecentFailures => "circuit_open_recent_failures",
            Self::EligibilityRevokedPolicy => "eligibility_revoked_policy",
            Self::EligibilityRevokedWorkspaceTrust => "eligibility_revoked_workspace_trust",
            Self::EligibilityRevokedPackQuarantined => "eligibility_revoked_pack_quarantined",
            Self::EligibilityRevokedProviderDisabled => "eligibility_revoked_provider_disabled",
        }
    }
}

/// Candidate-selection outcome re-exported from the route comparison contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectedOutcomeClass {
    /// This candidate is the selected route.
    SelectedThisPath,
    /// Candidate failed capability checks.
    NotSelectedFailedCapability,
    /// Candidate failed region posture.
    NotSelectedFailedRegionPosture,
    /// Candidate failed retention stance.
    NotSelectedFailedRetentionStance,
    /// Candidate failed data-class allowlist.
    NotSelectedFailedDataClassAllowlist,
    /// Candidate failed quota family.
    NotSelectedFailedQuotaFamily,
    /// Candidate failed offline posture.
    NotSelectedFailedOfflinePosture,
    /// Candidate failed determinism posture.
    NotSelectedFailedDeterminismPosture,
    /// Candidate failed taint posture.
    NotSelectedFailedTaintPosture,
    /// Candidate failed approval posture.
    NotSelectedFailedApprovalPosture,
    /// Candidate quota is exhausted.
    NotSelectedQuotaExhausted,
    /// Candidate budget is exhausted.
    NotSelectedBudgetExhausted,
    /// Candidate circuit is open.
    NotSelectedCircuitOpen,
    /// Candidate is deprecated or withdrawn.
    NotSelectedDeprecatedOrWithdrawn,
    /// Candidate pack is unverified or quarantined.
    NotSelectedPackUnverified,
    /// Candidate was not selected because policy pinned another route.
    NotSelectedPolicyPin,
    /// Candidate was not selected because the user chose another route.
    NotSelectedUserExplicitChoice,
    /// Candidate is shadow-only.
    NotSelectedShadowOnlyRoute,
}

impl SelectedOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedThisPath => "selected_this_path",
            Self::NotSelectedFailedCapability => "not_selected_failed_capability",
            Self::NotSelectedFailedRegionPosture => "not_selected_failed_region_posture",
            Self::NotSelectedFailedRetentionStance => "not_selected_failed_retention_stance",
            Self::NotSelectedFailedDataClassAllowlist => "not_selected_failed_data_class_allowlist",
            Self::NotSelectedFailedQuotaFamily => "not_selected_failed_quota_family",
            Self::NotSelectedFailedOfflinePosture => "not_selected_failed_offline_posture",
            Self::NotSelectedFailedDeterminismPosture => "not_selected_failed_determinism_posture",
            Self::NotSelectedFailedTaintPosture => "not_selected_failed_taint_posture",
            Self::NotSelectedFailedApprovalPosture => "not_selected_failed_approval_posture",
            Self::NotSelectedQuotaExhausted => "not_selected_quota_exhausted",
            Self::NotSelectedBudgetExhausted => "not_selected_budget_exhausted",
            Self::NotSelectedCircuitOpen => "not_selected_circuit_open",
            Self::NotSelectedDeprecatedOrWithdrawn => "not_selected_deprecated_or_withdrawn",
            Self::NotSelectedPackUnverified => "not_selected_pack_unverified",
            Self::NotSelectedPolicyPin => "not_selected_policy_pin",
            Self::NotSelectedUserExplicitChoice => "not_selected_user_explicit_choice",
            Self::NotSelectedShadowOnlyRoute => "not_selected_shadow_only_route",
        }
    }
}

/// Cause class for visible route-change lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChangeCauseClass {
    /// No route change happened.
    NoRouteChange,
    /// Policy selected or pinned a different route.
    PolicyOverride,
    /// Fallback after quota exhaustion.
    FallbackAfterQuotaExhaustion,
    /// Fallback after budget exhaustion.
    FallbackAfterBudgetExhaustion,
    /// Fallback after circuit-open failures.
    FallbackAfterCircuitOpen,
    /// Fallback after provider disablement.
    FallbackAfterProviderDisabled,
    /// User selected a different route.
    UserExplicitChoice,
}

impl RouteChangeCauseClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRouteChange => "no_route_change",
            Self::PolicyOverride => "policy_override",
            Self::FallbackAfterQuotaExhaustion => "fallback_after_quota_exhaustion",
            Self::FallbackAfterBudgetExhaustion => "fallback_after_budget_exhaustion",
            Self::FallbackAfterCircuitOpen => "fallback_after_circuit_open",
            Self::FallbackAfterProviderDisabled => "fallback_after_provider_disabled",
            Self::UserExplicitChoice => "user_explicit_choice",
        }
    }
}

/// Workspace trust state carried in the routing packet's policy context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyTrustState {
    /// Workspace is trusted.
    Trusted,
    /// Workspace is restricted.
    Restricted,
}

impl PolicyTrustState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
        }
    }
}

/// Deployment profile carried in the routing packet's policy context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileClass {
    /// Individual local profile.
    IndividualLocal,
    /// Customer self-hosted profile.
    SelfHosted,
    /// Managed cloud profile.
    ManagedCloud,
    /// Managed fleet profile.
    ManagedFleet,
    /// Air-gapped profile.
    AirGapped,
}

impl DeploymentProfileClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::ManagedCloud => "managed_cloud",
            Self::ManagedFleet => "managed_fleet",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// Policy context that bounded the route decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingPolicyContext {
    /// Opaque policy epoch ref.
    pub policy_epoch_ref: String,
    /// Workspace trust state.
    pub trust_state: PolicyTrustState,
    /// Deployment profile.
    pub deployment_profile_class: DeploymentProfileClass,
    /// Opaque execution-context ref when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
}

/// Export-safe execution-context summary attached to AI routing packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRoutingExecutionContextSummary {
    /// Execution-context id that bounded the AI tool route.
    pub execution_context_ref: String,
    /// Workspace id copied from the execution context.
    pub workspace_id: String,
    /// Surface token that minted the execution context.
    pub surface_token: String,
    /// Present toolchain tokens from the shared workspace detector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_toolchain_tokens: Vec<String>,
    /// Absent toolchain tokens from the shared workspace detector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub absent_toolchain_tokens: Vec<String>,
}

impl AiRoutingExecutionContextSummary {
    /// Projects an export-safe routing summary from the canonical context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        Self {
            execution_context_ref: context.execution_context_id.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            detected_toolchain_tokens: context
                .workspace_toolchain_discovery
                .as_ref()
                .map(|report| report.present_toolchain_tokens())
                .unwrap_or_default(),
            absent_toolchain_tokens: context
                .workspace_toolchain_discovery
                .as_ref()
                .map(|report| report.absent_toolchain_tokens())
                .unwrap_or_default(),
        }
    }
}

/// Quota inspector attached to one route candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuotaInspector {
    /// Quota family from the provider registry.
    pub quota_family_class: QuotaFamilyClass,
    /// Current quota state.
    pub quota_state_class: QuotaStateClass,
    /// Owner scope of the quota.
    pub quota_scope_class: QuotaScopeClass,
    /// Opaque owner ref such as workspace, org, or provider quota pool.
    pub budget_owner_ref: String,
    /// Opaque meter ref for quota details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota_meter_ref: Option<String>,
    /// Opaque forecast ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quota_forecast_ref: Option<String>,
    /// Opaque usage-export ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_export_ref: Option<String>,
    /// Export-safe quota explanation.
    pub explanation_label: String,
    /// Export-safe label for what still works locally when quota blocks hosted use.
    pub local_continuity_label: String,
    /// Recovery action ref such as view forecast, switch scope, or continue local.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_action_ref: Option<String>,
}

impl QuotaInspector {
    /// True when the quota inspector is specific enough for a hosted route.
    pub fn discloses_quota_state(&self) -> bool {
        self.quota_family_class != QuotaFamilyClass::QuotaUnknownUnverified
            && self.quota_state_class != QuotaStateClass::UnknownUnverified
            && !self.budget_owner_ref.trim().is_empty()
            && !self.explanation_label.trim().is_empty()
            && !self.local_continuity_label.trim().is_empty()
    }
}

/// Latency and cost envelope attached to one route candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatencyCostEnvelope {
    /// Coarse latency envelope.
    pub latency_envelope_class: LatencyEnvelopeClass,
    /// Coarse cost envelope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Cost visibility posture.
    pub cost_visibility_class: CostVisibilityClass,
    /// Token ceiling bucket.
    pub token_ceiling_class: TokenCeilingClass,
    /// Tool-call ceiling bucket.
    pub tool_call_ceiling_class: ToolCallCeilingClass,
    /// Wall-time ceiling bucket.
    pub wall_time_ceiling_class: WallTimeCeilingClass,
    /// Opaque budget-routing policy ref.
    pub budget_routing_policy_ref: String,
    /// Opaque graduation or rollout packet ref.
    pub graduation_packet_ref: String,
    /// Opaque envelope evidence ref.
    pub envelope_evidence_ref: String,
    /// Export-safe envelope explanation.
    pub explanation_label: String,
}

impl LatencyCostEnvelope {
    /// True when both the latency and cost envelope are usable before dispatch.
    pub fn discloses_latency_cost(&self) -> bool {
        self.latency_envelope_class.is_verified()
            && self.cost_envelope_class.is_verified()
            && !matches!(
                self.cost_visibility_class,
                CostVisibilityClass::UndisclosedByProvider
            )
            && !self.budget_routing_policy_ref.trim().is_empty()
            && !self.graduation_packet_ref.trim().is_empty()
            && !self.envelope_evidence_ref.trim().is_empty()
            && !self.explanation_label.trim().is_empty()
    }
}

/// One provider/model path the route planner considered.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRouteCandidate {
    /// Stable candidate id within the routing packet.
    pub candidate_id: String,
    /// Opaque provider registry entry ref.
    pub provider_entry_ref: String,
    /// Export-safe provider label.
    pub provider_label: String,
    /// Provider class.
    pub provider_class: AiRouteProviderClass,
    /// Opaque model registry entry ref.
    pub model_entry_ref: String,
    /// Export-safe model label.
    pub model_label: String,
    /// Execution locus.
    pub execution_locus_class: ExecutionLocusClass,
    /// Route origin.
    pub route_origin_class: RouteOriginClass,
    /// Region posture.
    pub region_posture_class: RegionPostureClass,
    /// Retention stance.
    pub retention_stance_class: RetentionStanceClass,
    /// Quota inspector for this candidate.
    pub quota: QuotaInspector,
    /// Latency/cost envelope for this candidate.
    pub envelope: LatencyCostEnvelope,
    /// Route-selection reason for this candidate.
    pub route_selection_reason_class: RouteSelectionReasonClass,
    /// Route-selection override reason for this candidate.
    pub route_selection_override_reason_class: RouteSelectionOverrideReasonClass,
    /// Exhaustion state for this candidate.
    pub exhaustion_state_class: ExhaustionStateClass,
    /// Whether this candidate was selected.
    pub selected_outcome_class: SelectedOutcomeClass,
    /// Opaque route-selection disclosure ref when an override or fallback happened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_selection_disclosure_ref: Option<String>,
    /// Opaque approval ticket ref when selection required one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_approval_ticket_ref: Option<String>,
    /// Export-safe explanation for this candidate's selection or rejection.
    pub explanation_label: String,
}

impl AiRouteCandidate {
    /// True when this candidate is the selected route.
    pub const fn is_selected(&self) -> bool {
        matches!(
            self.selected_outcome_class,
            SelectedOutcomeClass::SelectedThisPath
        )
    }

    /// True when this candidate is a hosted or external model path.
    pub const fn is_hosted_model_path(&self) -> bool {
        self.execution_locus_class.is_hosted_model_path()
    }

    fn has_identity(&self) -> bool {
        !self.provider_entry_ref.trim().is_empty()
            && !self.model_entry_ref.trim().is_empty()
            && !self.provider_label.trim().is_empty()
            && !self.model_label.trim().is_empty()
    }

    fn route_selection_disclosure_present(&self) -> bool {
        self.route_selection_disclosure_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    }
}

/// Visible lineage row explaining why selection changed route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteChangeLineage {
    /// Stable lineage id.
    pub lineage_id: String,
    /// Cause of the route change.
    pub cause_class: RouteChangeCauseClass,
    /// Candidate id that lost eligibility or was overridden.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_candidate_ref: Option<String>,
    /// Candidate id that became selected.
    pub to_candidate_ref: String,
    /// Opaque route-selection disclosure ref.
    pub route_selection_disclosure_ref: String,
    /// Opaque policy epoch ref.
    pub policy_epoch_ref: String,
    /// Export-safe explanation shown to the user and support packet.
    pub visible_disclosure_label: String,
}

impl RouteChangeLineage {
    /// True when this lineage row is visible enough to explain a changed route.
    pub fn is_visible(&self) -> bool {
        self.cause_class != RouteChangeCauseClass::NoRouteChange
            && !self.to_candidate_ref.trim().is_empty()
            && !self.route_selection_disclosure_ref.trim().is_empty()
            && !self.policy_epoch_ref.trim().is_empty()
            && !self.visible_disclosure_label.trim().is_empty()
    }
}

/// Canonical routing packet for the bounded hosted-model alpha lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRoutingPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub routing_packet_id: String,
    /// Workflow or surface that requested the route.
    pub workflow_or_surface_id: String,
    /// Opaque request workspace ref.
    pub request_workspace_ref: String,
    /// Current run state for the route decision.
    pub run_state_class: RoutingRunStateClass,
    /// Policy context used to resolve the route.
    pub policy_context: RoutingPolicyContext,
    /// Capability-lifecycle row ref that owns this alpha surface.
    pub capability_lifecycle_row_ref: String,
    /// Identity-mode baseline ref consumed for local-vs-managed truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_mode_baseline_ref: Option<String>,
    /// Candidate routes considered.
    pub candidates: Vec<AiRouteCandidate>,
    /// Candidate id selected for use.
    pub selected_candidate_ref: String,
    /// Visible route-change lineage rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub route_change_lineage: Vec<RouteChangeLineage>,
    /// Source contracts this packet consumes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_contract_refs: Vec<String>,
    /// Execution-context summary when the route was planned from a resolved context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_summary: Option<AiRoutingExecutionContextSummary>,
    /// Timestamp the packet was minted.
    pub minted_at: String,
}

impl AiRoutingPacket {
    /// Builds a routing packet with the stable record kind and schema version.
    // Keep this constructor field-shaped so policy, lifecycle, candidate,
    // lineage, source-contract, and timestamp evidence stays explicit.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        routing_packet_id: impl Into<String>,
        workflow_or_surface_id: impl Into<String>,
        request_workspace_ref: impl Into<String>,
        run_state_class: RoutingRunStateClass,
        policy_context: RoutingPolicyContext,
        capability_lifecycle_row_ref: impl Into<String>,
        identity_mode_baseline_ref: Option<String>,
        candidates: Vec<AiRouteCandidate>,
        selected_candidate_ref: impl Into<String>,
        route_change_lineage: Vec<RouteChangeLineage>,
        source_contract_refs: Vec<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: AI_ROUTING_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_ROUTING_SCHEMA_VERSION,
            routing_packet_id: routing_packet_id.into(),
            workflow_or_surface_id: workflow_or_surface_id.into(),
            request_workspace_ref: request_workspace_ref.into(),
            run_state_class,
            policy_context,
            capability_lifecycle_row_ref: capability_lifecycle_row_ref.into(),
            identity_mode_baseline_ref,
            candidates,
            selected_candidate_ref: selected_candidate_ref.into(),
            route_change_lineage,
            source_contract_refs,
            execution_context_summary: None,
            minted_at: minted_at.into(),
        }
    }

    /// Attaches the canonical execution context that bounded this route.
    pub fn with_execution_context(mut self, context: &ExecutionContext) -> Self {
        self.policy_context.execution_context_ref = Some(context.execution_context_id.clone());
        self.execution_context_summary =
            Some(AiRoutingExecutionContextSummary::from_context(context));
        self
    }

    /// Returns the selected route candidate, when the selected id resolves.
    pub fn selected_route(&self) -> Option<&AiRouteCandidate> {
        self.candidates
            .iter()
            .find(|candidate| candidate.candidate_id == self.selected_candidate_ref)
    }

    /// Project a shell, CLI, or support surface row list from the selected route.
    pub fn surface_rows(&self) -> Vec<AiRoutingSurfaceRow> {
        let Some(selected) = self.selected_route() else {
            return Vec::new();
        };
        let mut rows = vec![
            AiRoutingSurfaceRow::new(
                "provider",
                "Provider",
                &selected.provider_label,
                selected.provider_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "model",
                "Model",
                &selected.model_label,
                &selected.model_entry_ref,
            ),
            AiRoutingSurfaceRow::new(
                "execution_locus",
                "Execution locus",
                selected.execution_locus_class.as_str(),
                selected.execution_locus_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "route_origin",
                "Route origin",
                selected.route_origin_class.as_str(),
                selected.route_origin_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "quota_state",
                "Quota state",
                selected.quota.quota_state_class.as_str(),
                selected.quota.quota_state_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "quota_family",
                "Quota family",
                selected.quota.quota_family_class.as_str(),
                selected.quota.quota_family_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "latency_envelope",
                "Latency envelope",
                selected.envelope.latency_envelope_class.as_str(),
                selected.envelope.latency_envelope_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "cost_envelope",
                "Cost envelope",
                selected.envelope.cost_envelope_class.as_str(),
                selected.envelope.cost_envelope_class.as_str(),
            ),
            AiRoutingSurfaceRow::new(
                "run_state",
                "Run state",
                self.run_state_class.as_str(),
                self.run_state_class.as_str(),
            ),
        ];
        if let Some(summary) = &self.execution_context_summary {
            rows.push(AiRoutingSurfaceRow::new(
                "execution_context",
                "Execution context",
                &summary.execution_context_ref,
                &summary.surface_token,
            ));
            rows.push(AiRoutingSurfaceRow::new(
                "toolchain_detection",
                "Toolchain detection",
                &join_tokens(&summary.detected_toolchain_tokens),
                &join_tokens(&summary.detected_toolchain_tokens),
            ));
        }
        if let Some(lineage) = self
            .route_change_lineage
            .iter()
            .find(|row| row.to_candidate_ref == selected.candidate_id)
        {
            rows.push(AiRoutingSurfaceRow::new(
                "route_change",
                "Route change",
                &lineage.visible_disclosure_label,
                lineage.cause_class.as_str(),
            ));
        }
        rows
    }

    /// Project an export-safe support packet from the routing packet.
    pub fn support_packet(&self) -> AiRoutingSupportPacket {
        let selected = self.selected_route();
        let route_change_rows = self
            .route_change_lineage
            .iter()
            .map(|lineage| AiRoutingSupportRouteChangeRow {
                lineage_id: lineage.lineage_id.clone(),
                cause_token: lineage.cause_class.as_str().to_owned(),
                from_candidate_ref: lineage.from_candidate_ref.clone(),
                to_candidate_ref: lineage.to_candidate_ref.clone(),
                route_selection_disclosure_ref: lineage.route_selection_disclosure_ref.clone(),
                visible_disclosure_label: lineage.visible_disclosure_label.clone(),
            })
            .collect();
        let validation_violation_tokens = self
            .validate()
            .into_iter()
            .map(|violation| violation.as_str().to_owned())
            .collect();

        AiRoutingSupportPacket {
            record_kind: AI_ROUTING_SUPPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_ROUTING_SCHEMA_VERSION,
            support_packet_id: format!("support-export:ai-routing:{}", self.routing_packet_id),
            routing_packet_ref: self.routing_packet_id.clone(),
            workflow_or_surface_id: self.workflow_or_surface_id.clone(),
            request_workspace_ref: self.request_workspace_ref.clone(),
            policy_epoch_ref: self.policy_context.policy_epoch_ref.clone(),
            trust_state_token: self.policy_context.trust_state.as_str().to_owned(),
            deployment_profile_token: self
                .policy_context
                .deployment_profile_class
                .as_str()
                .to_owned(),
            execution_context_ref: self
                .execution_context_summary
                .as_ref()
                .map(|summary| summary.execution_context_ref.clone())
                .or_else(|| self.policy_context.execution_context_ref.clone()),
            detected_toolchain_tokens: self
                .execution_context_summary
                .as_ref()
                .map(|summary| summary.detected_toolchain_tokens.clone())
                .unwrap_or_default(),
            absent_toolchain_tokens: self
                .execution_context_summary
                .as_ref()
                .map(|summary| summary.absent_toolchain_tokens.clone())
                .unwrap_or_default(),
            selected_provider_entry_ref: selected
                .map(|candidate| candidate.provider_entry_ref.clone())
                .unwrap_or_default(),
            selected_model_entry_ref: selected
                .map(|candidate| candidate.model_entry_ref.clone())
                .unwrap_or_default(),
            selected_provider_label: selected
                .map(|candidate| candidate.provider_label.clone())
                .unwrap_or_default(),
            selected_model_label: selected
                .map(|candidate| candidate.model_label.clone())
                .unwrap_or_default(),
            execution_locus_token: selected
                .map(|candidate| candidate.execution_locus_class.as_str().to_owned())
                .unwrap_or_default(),
            route_origin_token: selected
                .map(|candidate| candidate.route_origin_class.as_str().to_owned())
                .unwrap_or_default(),
            quota_state_token: selected
                .map(|candidate| candidate.quota.quota_state_class.as_str().to_owned())
                .unwrap_or_default(),
            quota_family_token: selected
                .map(|candidate| candidate.quota.quota_family_class.as_str().to_owned())
                .unwrap_or_default(),
            quota_scope_token: selected
                .map(|candidate| candidate.quota.quota_scope_class.as_str().to_owned())
                .unwrap_or_default(),
            latency_envelope_token: selected
                .map(|candidate| {
                    candidate
                        .envelope
                        .latency_envelope_class
                        .as_str()
                        .to_owned()
                })
                .unwrap_or_default(),
            cost_envelope_token: selected
                .map(|candidate| candidate.envelope.cost_envelope_class.as_str().to_owned())
                .unwrap_or_default(),
            cost_visibility_token: selected
                .map(|candidate| candidate.envelope.cost_visibility_class.as_str().to_owned())
                .unwrap_or_default(),
            route_selection_reason_token: selected
                .map(|candidate| candidate.route_selection_reason_class.as_str().to_owned())
                .unwrap_or_default(),
            route_selection_override_reason_token: selected
                .map(|candidate| {
                    candidate
                        .route_selection_override_reason_class
                        .as_str()
                        .to_owned()
                })
                .unwrap_or_default(),
            exhaustion_state_token: selected
                .map(|candidate| candidate.exhaustion_state_class.as_str().to_owned())
                .unwrap_or_default(),
            budget_owner_ref: selected
                .map(|candidate| candidate.quota.budget_owner_ref.clone())
                .unwrap_or_default(),
            local_continuity_label: selected
                .map(|candidate| candidate.quota.local_continuity_label.clone())
                .unwrap_or_default(),
            route_change_rows,
            validation_violation_tokens,
            capability_lifecycle_row_ref: self.capability_lifecycle_row_ref.clone(),
            identity_mode_baseline_ref: self.identity_mode_baseline_ref.clone(),
            source_contract_refs: self.source_contract_refs.clone(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Validate the packet against hosted-route alpha invariants.
    pub fn validate(&self) -> Vec<AiRoutingViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_ROUTING_PACKET_RECORD_KIND {
            violations.push(AiRoutingViolation::WrongRecordKind);
        }
        if self.schema_version != AI_ROUTING_SCHEMA_VERSION {
            violations.push(AiRoutingViolation::WrongSchemaVersion);
        }
        if self.routing_packet_id.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingPacketId);
        }
        if self.workflow_or_surface_id.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingWorkflowOrSurfaceId);
        }
        if self.request_workspace_ref.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingRequestWorkspaceRef);
        }
        if self.policy_context.policy_epoch_ref.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingPolicyEpochRef);
        }
        if self.capability_lifecycle_row_ref.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingCapabilityLifecycleRowRef);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(AiRoutingViolation::MissingSourceContractRefs);
        }
        if self.selected_candidate_ref.trim().is_empty() {
            violations.push(AiRoutingViolation::MissingSelectedCandidateRef);
        }
        if self.candidates.is_empty() {
            violations.push(AiRoutingViolation::MissingCandidates);
        }
        let selected_count = self
            .candidates
            .iter()
            .filter(|candidate| candidate.is_selected())
            .count();
        if selected_count != 1 {
            violations.push(AiRoutingViolation::SelectionStateInconsistent);
        }
        let selected = self.selected_route();
        if selected.is_none() {
            violations.push(AiRoutingViolation::SelectedCandidateNotFound);
        }
        if let Some(selected) = selected {
            if !selected.is_selected() {
                violations.push(AiRoutingViolation::SelectionStateInconsistent);
            }
            if selected.is_hosted_model_path() {
                if !selected.has_identity() {
                    violations.push(AiRoutingViolation::HostedRouteMissingProviderModel);
                }
                if !selected.quota.discloses_quota_state() {
                    violations.push(AiRoutingViolation::HostedRouteMissingQuotaState);
                }
                if !selected.envelope.discloses_latency_cost() {
                    violations.push(AiRoutingViolation::HostedRouteMissingLatencyCostEnvelope);
                }
                if self
                    .identity_mode_baseline_ref
                    .as_deref()
                    .map_or(true, |value| value.trim().is_empty())
                {
                    violations.push(AiRoutingViolation::MissingIdentityBaselineRefForHostedRoute);
                }
                if selected.quota.quota_state_class.blocks_hosted_dispatch() {
                    violations.push(AiRoutingViolation::HostedRouteQuotaBlockedButSelected);
                }
            }
            if selected
                .route_selection_reason_class
                .requires_route_change_lineage()
            {
                let visible_lineage = self.route_change_lineage.iter().any(|lineage| {
                    lineage.to_candidate_ref == selected.candidate_id && lineage.is_visible()
                });
                if !visible_lineage {
                    violations.push(AiRoutingViolation::RouteChangeMissingVisibleLineage);
                }
                let has_alternative = self
                    .candidates
                    .iter()
                    .any(|candidate| candidate.candidate_id != selected.candidate_id);
                if !has_alternative {
                    violations.push(AiRoutingViolation::RouteChangeMissingAlternative);
                }
                if !selected.route_selection_disclosure_present() {
                    violations.push(AiRoutingViolation::RouteSelectionDisclosureMissing);
                }
            }
            if selected.route_selection_reason_class.is_fallback()
                && selected.exhaustion_state_class
                    == ExhaustionStateClass::NotExhaustedRouteAdmitted
            {
                violations.push(AiRoutingViolation::FallbackMissingExhaustionState);
            }
            if selected.quota.local_continuity_label.trim().is_empty() {
                violations.push(AiRoutingViolation::MissingLocalContinuityLabel);
            }
        }

        if packet_contains_forbidden_boundary_material(self) {
            violations.push(AiRoutingViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON for support bundles.
    ///
    /// # Panics
    ///
    /// Panics only if serializing the metadata-only support packet fails.
    pub fn export_safe_support_json(&self) -> String {
        self.support_packet().export_safe_json()
    }
}

/// One shell, CLI, or support surface row projected from a routing packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRoutingSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Export-safe display label.
    pub label: String,
    /// Export-safe value label.
    pub value_label: String,
    /// Stable token or opaque ref backing the value.
    pub value_token: String,
}

impl AiRoutingSurfaceRow {
    fn new(row_id: &str, label: &str, value_label: &str, value_token: &str) -> Self {
        Self {
            row_id: row_id.to_owned(),
            label: label.to_owned(),
            value_label: value_label.to_owned(),
            value_token: value_token.to_owned(),
        }
    }
}

/// Export-safe support projection for one routing packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRoutingSupportPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support packet id.
    pub support_packet_id: String,
    /// Source routing packet ref.
    pub routing_packet_ref: String,
    /// Workflow or surface that requested the route.
    pub workflow_or_surface_id: String,
    /// Opaque request workspace ref.
    pub request_workspace_ref: String,
    /// Opaque policy epoch ref.
    pub policy_epoch_ref: String,
    /// Workspace trust-state token.
    pub trust_state_token: String,
    /// Deployment-profile token.
    pub deployment_profile_token: String,
    /// Execution-context id that bounded the route, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_ref: Option<String>,
    /// Present toolchain tokens copied from the execution context.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub detected_toolchain_tokens: Vec<String>,
    /// Absent toolchain tokens copied from the execution context.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub absent_toolchain_tokens: Vec<String>,
    /// Selected provider registry ref.
    pub selected_provider_entry_ref: String,
    /// Selected model registry ref.
    pub selected_model_entry_ref: String,
    /// Export-safe provider label.
    pub selected_provider_label: String,
    /// Export-safe model label.
    pub selected_model_label: String,
    /// Execution locus token.
    pub execution_locus_token: String,
    /// Route origin token.
    pub route_origin_token: String,
    /// Quota state token.
    pub quota_state_token: String,
    /// Quota family token.
    pub quota_family_token: String,
    /// Quota owner scope token.
    pub quota_scope_token: String,
    /// Latency envelope token.
    pub latency_envelope_token: String,
    /// Cost envelope token.
    pub cost_envelope_token: String,
    /// Cost visibility token.
    pub cost_visibility_token: String,
    /// Route-selection reason token.
    pub route_selection_reason_token: String,
    /// Route-selection override reason token.
    pub route_selection_override_reason_token: String,
    /// Exhaustion-state token.
    pub exhaustion_state_token: String,
    /// Budget owner ref.
    pub budget_owner_ref: String,
    /// Local continuity label.
    pub local_continuity_label: String,
    /// Visible route-change rows.
    pub route_change_rows: Vec<AiRoutingSupportRouteChangeRow>,
    /// Validation violations present on the source packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_violation_tokens: Vec<String>,
    /// Capability-lifecycle row ref.
    pub capability_lifecycle_row_ref: String,
    /// Identity-mode baseline ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_mode_baseline_ref: Option<String>,
    /// Source contract refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_contract_refs: Vec<String>,
    /// Timestamp the source packet was minted.
    pub minted_at: String,
}

impl AiRoutingSupportPacket {
    /// Deterministic export-safe JSON. The projection carries metadata only.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only support packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("routing support packet serializes")
    }
}

/// Export-safe route-change row inside [`AiRoutingSupportPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiRoutingSupportRouteChangeRow {
    /// Source lineage id.
    pub lineage_id: String,
    /// Route-change cause token.
    pub cause_token: String,
    /// Candidate id that lost eligibility or was overridden.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_candidate_ref: Option<String>,
    /// Candidate id that became selected.
    pub to_candidate_ref: String,
    /// Route-selection disclosure ref.
    pub route_selection_disclosure_ref: String,
    /// Export-safe explanation.
    pub visible_disclosure_label: String,
}

/// Validation failures emitted by [`AiRoutingPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiRoutingViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Packet id is missing.
    MissingPacketId,
    /// Workflow or surface id is missing.
    MissingWorkflowOrSurfaceId,
    /// Request workspace ref is missing.
    MissingRequestWorkspaceRef,
    /// Policy epoch ref is missing.
    MissingPolicyEpochRef,
    /// Capability-lifecycle row ref is missing.
    MissingCapabilityLifecycleRowRef,
    /// Source contract refs are missing.
    MissingSourceContractRefs,
    /// Candidate list is empty.
    MissingCandidates,
    /// Selected candidate ref is missing.
    MissingSelectedCandidateRef,
    /// Selected candidate ref does not resolve.
    SelectedCandidateNotFound,
    /// Selected outcome fields are inconsistent.
    SelectionStateInconsistent,
    /// Hosted route lacks provider or model identity.
    HostedRouteMissingProviderModel,
    /// Hosted route lacks quota state.
    HostedRouteMissingQuotaState,
    /// Hosted route lacks latency or cost envelope.
    HostedRouteMissingLatencyCostEnvelope,
    /// Hosted route lacks identity baseline ref.
    MissingIdentityBaselineRefForHostedRoute,
    /// Selected hosted route is blocked by quota state.
    HostedRouteQuotaBlockedButSelected,
    /// Route change lacks visible lineage.
    RouteChangeMissingVisibleLineage,
    /// Route change lacks an alternative row.
    RouteChangeMissingAlternative,
    /// Route-selection disclosure ref is missing.
    RouteSelectionDisclosureMissing,
    /// Fallback route lacks exhaustion state.
    FallbackMissingExhaustionState,
    /// Local continuity label is missing.
    MissingLocalContinuityLabel,
    /// Packet contains raw boundary material forbidden in exports.
    RawBoundaryMaterialInExport,
}

impl AiRoutingViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketId => "missing_packet_id",
            Self::MissingWorkflowOrSurfaceId => "missing_workflow_or_surface_id",
            Self::MissingRequestWorkspaceRef => "missing_request_workspace_ref",
            Self::MissingPolicyEpochRef => "missing_policy_epoch_ref",
            Self::MissingCapabilityLifecycleRowRef => "missing_capability_lifecycle_row_ref",
            Self::MissingSourceContractRefs => "missing_source_contract_refs",
            Self::MissingCandidates => "missing_candidates",
            Self::MissingSelectedCandidateRef => "missing_selected_candidate_ref",
            Self::SelectedCandidateNotFound => "selected_candidate_not_found",
            Self::SelectionStateInconsistent => "selection_state_inconsistent",
            Self::HostedRouteMissingProviderModel => "hosted_route_missing_provider_model",
            Self::HostedRouteMissingQuotaState => "hosted_route_missing_quota_state",
            Self::HostedRouteMissingLatencyCostEnvelope => {
                "hosted_route_missing_latency_cost_envelope"
            }
            Self::MissingIdentityBaselineRefForHostedRoute => {
                "missing_identity_baseline_ref_for_hosted_route"
            }
            Self::HostedRouteQuotaBlockedButSelected => "hosted_route_quota_blocked_but_selected",
            Self::RouteChangeMissingVisibleLineage => "route_change_missing_visible_lineage",
            Self::RouteChangeMissingAlternative => "route_change_missing_alternative",
            Self::RouteSelectionDisclosureMissing => "route_selection_disclosure_missing",
            Self::FallbackMissingExhaustionState => "fallback_missing_exhaustion_state",
            Self::MissingLocalContinuityLabel => "missing_local_continuity_label",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

fn packet_contains_forbidden_boundary_material(packet: &AiRoutingPacket) -> bool {
    let packet_strings = [
        packet.routing_packet_id.as_str(),
        packet.workflow_or_surface_id.as_str(),
        packet.request_workspace_ref.as_str(),
        packet.capability_lifecycle_row_ref.as_str(),
        packet.minted_at.as_str(),
    ];
    packet_strings
        .iter()
        .any(|value| contains_forbidden_boundary_material(value))
        || contains_forbidden_boundary_material(&packet.policy_context.policy_epoch_ref)
        || packet
            .policy_context
            .execution_context_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || packet
            .identity_mode_baseline_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || packet
            .source_contract_refs
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
        || packet
            .candidates
            .iter()
            .any(candidate_contains_forbidden_boundary_material)
        || packet.route_change_lineage.iter().any(|lineage| {
            [
                lineage.lineage_id.as_str(),
                lineage.to_candidate_ref.as_str(),
                lineage.route_selection_disclosure_ref.as_str(),
                lineage.policy_epoch_ref.as_str(),
                lineage.visible_disclosure_label.as_str(),
            ]
            .iter()
            .any(|value| contains_forbidden_boundary_material(value))
                || lineage
                    .from_candidate_ref
                    .as_deref()
                    .is_some_and(contains_forbidden_boundary_material)
        })
}

fn join_tokens(tokens: &[String]) -> String {
    if tokens.is_empty() {
        "none".to_owned()
    } else {
        tokens.join("|")
    }
}

fn candidate_contains_forbidden_boundary_material(candidate: &AiRouteCandidate) -> bool {
    [
        candidate.candidate_id.as_str(),
        candidate.provider_entry_ref.as_str(),
        candidate.provider_label.as_str(),
        candidate.model_entry_ref.as_str(),
        candidate.model_label.as_str(),
        candidate.quota.budget_owner_ref.as_str(),
        candidate.quota.explanation_label.as_str(),
        candidate.quota.local_continuity_label.as_str(),
        candidate.envelope.budget_routing_policy_ref.as_str(),
        candidate.envelope.graduation_packet_ref.as_str(),
        candidate.envelope.envelope_evidence_ref.as_str(),
        candidate.envelope.explanation_label.as_str(),
        candidate.explanation_label.as_str(),
    ]
    .iter()
    .any(|value| contains_forbidden_boundary_material(value))
        || candidate
            .quota
            .quota_meter_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || candidate
            .quota
            .quota_forecast_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || candidate
            .quota
            .usage_export_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || candidate
            .quota
            .recovery_action_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || candidate
            .route_selection_disclosure_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
        || candidate
            .originating_approval_ticket_ref
            .as_deref()
            .is_some_and(contains_forbidden_boundary_material)
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}

#[cfg(test)]
mod tests;
