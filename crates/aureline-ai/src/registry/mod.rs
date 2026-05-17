//! Provider/model registry beta with execution-location labels and route controls.
//!
//! This module owns the first executable registry contract that joins AI
//! providers, models, local model packs, and external tool rows into one
//! source of truth. It resolves local-first and cheapest-qualifying routes
//! from registry state, projects the same rows for UI/docs/support consumers,
//! and can mint an [`crate::routing::AiRoutingPacket`] for the existing
//! headless routing surface.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::routing::{
    AiRouteCandidate, AiRouteProviderClass, AiRoutingPacket, CostEnvelopeClass,
    CostVisibilityClass, ExecutionLocusClass, ExhaustionStateClass, LatencyCostEnvelope,
    LatencyEnvelopeClass, QuotaFamilyClass, QuotaInspector, QuotaScopeClass, QuotaStateClass,
    RegionPostureClass, RetentionStanceClass, RouteChangeCauseClass, RouteChangeLineage,
    RouteOriginClass, RouteSelectionOverrideReasonClass, RouteSelectionReasonClass,
    RoutingPolicyContext, RoutingRunStateClass, SelectedOutcomeClass, TokenCeilingClass,
    ToolCallCeilingClass, WallTimeCeilingClass,
};

/// Stable record-kind tag for [`ProviderModelRegistryPacket`].
pub const PROVIDER_MODEL_REGISTRY_PACKET_RECORD_KIND: &str = "provider_model_registry_beta_packet";

/// Stable record-kind tag for [`ProviderRegistryEntry`].
pub const PROVIDER_MODEL_REGISTRY_PROVIDER_ENTRY_RECORD_KIND: &str =
    "provider_model_registry_provider_entry_record";

/// Stable record-kind tag for [`ModelRegistryEntry`].
pub const PROVIDER_MODEL_REGISTRY_MODEL_ENTRY_RECORD_KIND: &str =
    "provider_model_registry_model_entry_record";

/// Stable record-kind tag for [`ExternalToolRegistryEntry`].
pub const PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND: &str =
    "provider_model_registry_external_tool_entry_record";

/// Stable record-kind tag for [`ClaimedAiSurface`].
pub const PROVIDER_MODEL_REGISTRY_CLAIMED_SURFACE_RECORD_KIND: &str =
    "provider_model_registry_claimed_surface_record";

/// Stable record-kind tag for [`ProviderModelRegistrySupportExport`].
pub const PROVIDER_MODEL_REGISTRY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_model_registry_support_export_record";

/// Schema version for the beta provider/model registry packet and projections.
pub const PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION: u32 = 1;

/// AI feature class a provider or model can serve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiFeatureClass {
    /// Chat completion requests.
    ChatCompletion,
    /// Chat completion requests that can call governed tools.
    ChatWithToolCalls,
    /// Single-line code completion requests.
    CodeCompletionSingleLine,
    /// Multi-line code completion requests.
    CodeCompletionMultiLine,
    /// Fill-in-middle code completion requests.
    FillInMiddle,
    /// JSON-schema constrained structured output requests.
    StructuredOutputJsonSchema,
    /// Text embedding requests.
    EmbeddingsText,
    /// Search or review reranking requests.
    Reranking,
}

impl AiFeatureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChatCompletion => "chat_completion",
            Self::ChatWithToolCalls => "chat_with_tool_calls",
            Self::CodeCompletionSingleLine => "code_completion_single_line",
            Self::CodeCompletionMultiLine => "code_completion_multi_line",
            Self::FillInMiddle => "fill_in_middle",
            Self::StructuredOutputJsonSchema => "structured_output_json_schema",
            Self::EmbeddingsText => "embeddings_text",
            Self::Reranking => "reranking",
        }
    }
}

/// Data class that may be admitted to a provider or external tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryDataClass {
    /// Workspace code slices.
    WorkspaceCodeSliceAllowed,
    /// Workspace symbol summaries.
    WorkspaceSymbolAllowed,
    /// Workspace search result snippets.
    WorkspaceSearchResultAllowed,
    /// Documentation-pack excerpts.
    DocsPackExcerptAllowed,
    /// User-authored prompt text.
    UserSuppliedTextAllowed,
    /// Prior AI turn context.
    AiPriorTurnContextAllowed,
    /// Credential handles, which are always denied as prompt/tool input.
    CredentialHandleDeniedAlways,
    /// Secret projections, which are always denied as prompt/tool input.
    SecretProjectionDeniedAlways,
}

impl RegistryDataClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceCodeSliceAllowed => "workspace_code_slice_allowed",
            Self::WorkspaceSymbolAllowed => "workspace_symbol_allowed",
            Self::WorkspaceSearchResultAllowed => "workspace_search_result_allowed",
            Self::DocsPackExcerptAllowed => "docs_pack_excerpt_allowed",
            Self::UserSuppliedTextAllowed => "user_supplied_text_allowed",
            Self::AiPriorTurnContextAllowed => "ai_prior_turn_context_allowed",
            Self::CredentialHandleDeniedAlways => "credential_handle_denied_always",
            Self::SecretProjectionDeniedAlways => "secret_projection_denied_always",
        }
    }
}

/// Transport class for provider inference traffic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryTransportClass {
    /// In-process function call.
    InProcessCall,
    /// Local loopback HTTP transport.
    LocalHttpLoopback,
    /// Remote HTTPS transport.
    RemoteHttps,
    /// Remote gRPC transport over TLS.
    RemoteGrpcOverTls,
    /// Extension-mediated transport.
    ExtensionMediatedTransport,
    /// Disabled row with no transport.
    DisabledNoTransport,
}

impl RegistryTransportClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessCall => "in_process_call",
            Self::LocalHttpLoopback => "local_http_loopback",
            Self::RemoteHttps => "remote_https",
            Self::RemoteGrpcOverTls => "remote_grpc_over_tls",
            Self::ExtensionMediatedTransport => "extension_mediated_transport",
            Self::DisabledNoTransport => "disabled_no_transport",
        }
    }
}

/// Authentication posture for a provider or tool route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryAuthModeClass {
    /// No authentication because bytes remain local.
    NoAuthLocalOnly,
    /// A signed local model-pack manifest is the only authentication input.
    SignedManifestOnlyLocalPack,
    /// A user-held BYOK API key addressed through the secret broker.
    ByokApiKey,
    /// Enterprise gateway managed credential.
    EnterpriseGatewayManagedCredential,
    /// First-party managed hosted credential.
    VendorHostedFirstPartyManagedCredential,
    /// Extension-provided authentication.
    ExtensionProvidedAuth,
    /// Disabled row with no authentication.
    DisabledNoAuth,
}

impl RegistryAuthModeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthLocalOnly => "no_auth_local_only",
            Self::SignedManifestOnlyLocalPack => "signed_manifest_only_local_pack",
            Self::ByokApiKey => "byok_api_key",
            Self::EnterpriseGatewayManagedCredential => "enterprise_gateway_managed_credential",
            Self::VendorHostedFirstPartyManagedCredential => {
                "vendor_hosted_first_party_managed_credential"
            }
            Self::ExtensionProvidedAuth => "extension_provided_auth",
            Self::DisabledNoAuth => "disabled_no_auth",
        }
    }
}

/// Approval posture for provider and tool dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryApprovalPostureClass {
    /// Dispatch may proceed without a prompt inside the current policy.
    AllowedWithoutPrompt,
    /// Dispatch requires one prompt for the current session.
    AllowedWithPerSessionPrompt,
    /// Dispatch requires a prompt for every invocation.
    AllowedWithPerInvocationPrompt,
    /// Dispatch requires an admin approval ticket.
    RequiresAdminApproval,
    /// Dispatch is denied by policy.
    DeniedByPolicy,
}

impl RegistryApprovalPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedWithoutPrompt => "allowed_without_prompt",
            Self::AllowedWithPerSessionPrompt => "allowed_with_per_session_prompt",
            Self::AllowedWithPerInvocationPrompt => "allowed_with_per_invocation_prompt",
            Self::RequiresAdminApproval => "requires_admin_approval",
            Self::DeniedByPolicy => "denied_by_policy",
        }
    }

    /// True when this posture includes a fresh user or admin approval gate.
    pub const fn requires_approval_gate(self) -> bool {
        matches!(
            self,
            Self::AllowedWithPerSessionPrompt
                | Self::AllowedWithPerInvocationPrompt
                | Self::RequiresAdminApproval
        )
    }
}

/// Lifecycle state for a registry row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryLifecycleStateClass {
    /// Preview-admitted route.
    PreviewAdmitted,
    /// Generally admitted route.
    GenerallyAdmitted,
    /// Soft-deprecated route that still admits current sessions.
    SoftDeprecatedStillAdmitted,
    /// Hard-deprecated route that denies new sessions.
    HardDeprecatedDeniesNewSessions,
    /// Withdrawn route.
    Withdrawn,
    /// Quarantined route pending review.
    QuarantinedPendingReview,
    /// Fixture-only route.
    MockedTestOnly,
}

impl RegistryLifecycleStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewAdmitted => "preview_admitted",
            Self::GenerallyAdmitted => "generally_admitted",
            Self::SoftDeprecatedStillAdmitted => "soft_deprecated_still_admitted",
            Self::HardDeprecatedDeniesNewSessions => "hard_deprecated_denies_new_sessions",
            Self::Withdrawn => "withdrawn",
            Self::QuarantinedPendingReview => "quarantined_pending_review",
            Self::MockedTestOnly => "mocked_test_only",
        }
    }

    /// True when the row can be considered for new dispatch.
    pub const fn admits_new_dispatch(self) -> bool {
        matches!(
            self,
            Self::PreviewAdmitted
                | Self::GenerallyAdmitted
                | Self::SoftDeprecatedStillAdmitted
                | Self::MockedTestOnly
        )
    }
}

/// Route eligibility state authored by policy evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteEligibilityClass {
    /// Route is eligible for dispatch.
    Eligible,
    /// Policy denied the route.
    PolicyDenied,
    /// Quota denied the route.
    QuotaBlocked,
    /// Capability requirements denied the route.
    CapabilityBlocked,
    /// Lifecycle state denied the route.
    LifecycleBlocked,
    /// Workspace trust denied the route.
    WorkspaceTrustBlocked,
    /// Route is for shadow/evaluation use only.
    ShadowOnly,
}

impl RouteEligibilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::PolicyDenied => "policy_denied",
            Self::QuotaBlocked => "quota_blocked",
            Self::CapabilityBlocked => "capability_blocked",
            Self::LifecycleBlocked => "lifecycle_blocked",
            Self::WorkspaceTrustBlocked => "workspace_trust_blocked",
            Self::ShadowOnly => "shadow_only",
        }
    }

    /// True when this route can be selected.
    pub const fn is_eligible(self) -> bool {
        matches!(self, Self::Eligible)
    }
}

/// Routing policy authored in registry state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryRoutingPolicyClass {
    /// Prefer eligible local routes, then fall back to the cheapest qualifying route.
    LocalFirstThenCheapest,
    /// Select the cheapest route that satisfies capability and policy constraints.
    CheapestQualifying,
    /// Select the policy-pinned route when it is eligible.
    PolicyPinned,
    /// User must choose manually before dispatch.
    ManualOnly,
    /// Dispatch is disabled.
    Disabled,
}

impl RegistryRoutingPolicyClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirstThenCheapest => "local_first_then_cheapest",
            Self::CheapestQualifying => "cheapest_qualifying",
            Self::PolicyPinned => "policy_pinned",
            Self::ManualOnly => "manual_only",
            Self::Disabled => "disabled",
        }
    }
}

/// Reason emitted after resolving a route from the registry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryRouteReasonClass {
    /// Local-first policy selected an eligible local route.
    LocalFirstEligibleRouteAdmitted,
    /// Cheapest-qualifying policy selected the lowest-cost eligible route.
    CheapestQualifyingRouteAdmitted,
    /// Policy selected a pinned route.
    PolicyPinnedRouteAdmitted,
    /// Manual route selection is required before dispatch.
    ManualSelectionRequired,
    /// No eligible route exists.
    NoEligibleRoute,
}

impl RegistryRouteReasonClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFirstEligibleRouteAdmitted => "local_first_eligible_route_admitted",
            Self::CheapestQualifyingRouteAdmitted => "cheapest_qualifying_route_admitted",
            Self::PolicyPinnedRouteAdmitted => "policy_pinned_route_admitted",
            Self::ManualSelectionRequired => "manual_selection_required",
            Self::NoEligibleRoute => "no_eligible_route",
        }
    }
}

/// Consumer surface that reads registry state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryConsumerSurfaceClass {
    /// Product UI projection.
    Ui,
    /// Documentation/help projection.
    Docs,
    /// Support-export projection.
    SupportExport,
    /// CLI or headless projection.
    Cli,
}

impl RegistryConsumerSurfaceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::Docs => "docs",
            Self::SupportExport => "support_export",
            Self::Cli => "cli",
        }
    }
}

/// Disclosure chip or readout required before dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistryDisclosureKind {
    /// Provider identity chip.
    ProviderIdentityChip,
    /// Model identity chip.
    ModelIdentityChip,
    /// Execution-location chip.
    ExecutionLocationChip,
    /// Region posture chip.
    RegionPostureChip,
    /// Retention stance chip.
    RetentionStanceChip,
    /// Data-class allowlist readout.
    DataClassAllowlistReadout,
    /// Policy-allowed route-choice readout.
    RouteChoiceReadout,
    /// External-tool identity chip.
    ToolIdentityChip,
    /// External-tool execution-location chip.
    ToolExecutionLocationChip,
    /// External-tool side-effect readout.
    ToolSideEffectReadout,
}

impl RegistryDisclosureKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderIdentityChip => "provider_identity_chip",
            Self::ModelIdentityChip => "model_identity_chip",
            Self::ExecutionLocationChip => "execution_location_chip",
            Self::RegionPostureChip => "region_posture_chip",
            Self::RetentionStanceChip => "retention_stance_chip",
            Self::DataClassAllowlistReadout => "data_class_allowlist_readout",
            Self::RouteChoiceReadout => "route_choice_readout",
            Self::ToolIdentityChip => "tool_identity_chip",
            Self::ToolExecutionLocationChip => "tool_execution_location_chip",
            Self::ToolSideEffectReadout => "tool_side_effect_readout",
        }
    }
}

/// Retrieval/index truth attached to AI surfaces that consume search or embeddings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetrievalTruthStateClass {
    /// Surface does not use retrieval or indexed state.
    NotApplicable,
    /// Retrieval/index state is ready.
    Ready,
    /// Retrieval/index state is partial and labeled.
    PartialLabeled,
    /// Retrieval/index state is stale and labeled.
    StaleLabeled,
    /// Provider or policy limits retrieval and the limitation is labeled.
    ProviderLimitedLabeled,
    /// Partial retrieval/index state lacks a visible label and blocks promotion.
    UnlabelledPartialBlocked,
}

impl RetrievalTruthStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Ready => "ready",
            Self::PartialLabeled => "partial_labeled",
            Self::StaleLabeled => "stale_labeled",
            Self::ProviderLimitedLabeled => "provider_limited_labeled",
            Self::UnlabelledPartialBlocked => "unlabelled_partial_blocked",
        }
    }
}

/// Side-effect class declared by an external tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalToolSideEffectClass {
    /// Inspect-only tool call.
    InspectOnly,
    /// Local reversible edit.
    LocalReversibleEdit,
    /// Local destructive edit.
    LocalDestructiveEdit,
    /// External reversible comment or draft write.
    ExternalReversibleComment,
    /// External irreversible publish.
    ExternalIrreversiblePublish,
    /// Policy or trust mutation.
    PolicyOrTrustMutation,
}

impl ExternalToolSideEffectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::LocalReversibleEdit => "local_reversible_edit",
            Self::LocalDestructiveEdit => "local_destructive_edit",
            Self::ExternalReversibleComment => "external_reversible_comment",
            Self::ExternalIrreversiblePublish => "external_irreversible_publish",
            Self::PolicyOrTrustMutation => "policy_or_trust_mutation",
        }
    }

    /// True when this side effect requires an approval posture above auto-allow.
    pub const fn requires_approval_gate(self) -> bool {
        !matches!(self, Self::InspectOnly)
    }
}

/// Transport class for an external tool row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalToolTransportClass {
    /// In-process native call.
    InProcessCall,
    /// Local stdio-spawned process.
    LocalStdioSpawn,
    /// Local HTTP loopback.
    LocalHttpLoopback,
    /// Remote HTTPS transport.
    RemoteHttps,
    /// Remote MCP streamable HTTP transport.
    RemoteMcpOverStreamableHttp,
    /// Extension-mediated transport.
    ExtensionMediatedTransport,
}

impl ExternalToolTransportClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProcessCall => "in_process_call",
            Self::LocalStdioSpawn => "local_stdio_spawn",
            Self::LocalHttpLoopback => "local_http_loopback",
            Self::RemoteHttps => "remote_https",
            Self::RemoteMcpOverStreamableHttp => "remote_mcp_over_streamable_http",
            Self::ExtensionMediatedTransport => "extension_mediated_transport",
        }
    }
}

/// Execution locus for an external tool row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalToolExecutionLocusClass {
    /// Tool runs in-process.
    LocalInProcess,
    /// Tool runs as a local subprocess on the same device.
    LocalSubprocessSameDevice,
    /// Tool runs behind local loopback service.
    LocalCompanionServiceLoopback,
    /// Tool runs in a remote vendor-managed service.
    RemoteVendorManagedService,
    /// Tool runs behind an enterprise gateway.
    EnterpriseGatewayBrokeredService,
    /// Extension owns the tool locus.
    ExtensionProvidedLocus,
}

impl ExternalToolExecutionLocusClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcess => "local_in_process",
            Self::LocalSubprocessSameDevice => "local_subprocess_same_device",
            Self::LocalCompanionServiceLoopback => "local_companion_service_loopback",
            Self::RemoteVendorManagedService => "remote_vendor_managed_service",
            Self::EnterpriseGatewayBrokeredService => "enterprise_gateway_brokered_service",
            Self::ExtensionProvidedLocus => "extension_provided_locus",
        }
    }
}

/// Routing policy row used by claimed AI surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRoutePolicy {
    /// Stable policy id.
    pub route_policy_id: String,
    /// Human-readable policy label safe for UI/docs/support.
    pub policy_label: String,
    /// Routing policy class.
    pub policy_class: RegistryRoutingPolicyClass,
    /// Provider refs admitted by this policy.
    #[serde(default)]
    pub allowed_provider_entry_refs: Vec<String>,
    /// Model refs admitted by this policy.
    #[serde(default)]
    pub allowed_model_entry_refs: Vec<String>,
    /// Execution loci admitted by this policy.
    #[serde(default)]
    pub allowed_execution_locus_classes: Vec<ExecutionLocusClass>,
    /// Route choices this policy allows operators to see or select.
    #[serde(default)]
    pub allowed_route_choices: Vec<RegistryRoutingPolicyClass>,
    /// Opaque policy epoch ref that produced this row.
    pub policy_epoch_ref: String,
}

/// Provider registry row with execution location and route eligibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistryEntry {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable provider entry id.
    pub provider_entry_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Provider/model family label shown in route disclosures.
    pub provider_family_label: String,
    /// Provider class.
    pub provider_class: AiRouteProviderClass,
    /// Execution locus naming where inference runs.
    pub execution_locus_class: ExecutionLocusClass,
    /// Route origin used by receipts.
    pub route_origin_class: RouteOriginClass,
    /// Transport class.
    pub transport_class: RegistryTransportClass,
    /// Auth mode.
    pub auth_mode_class: RegistryAuthModeClass,
    /// Region posture.
    pub region_posture_class: RegionPostureClass,
    /// Retention stance.
    pub retention_stance_class: RetentionStanceClass,
    /// Quota family.
    pub quota_family_class: QuotaFamilyClass,
    /// Current quota state.
    pub quota_state_class: QuotaStateClass,
    /// Quota owner scope.
    pub quota_scope_class: QuotaScopeClass,
    /// Cost visibility posture.
    pub cost_visibility_class: CostVisibilityClass,
    /// Coarse cost envelope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Coarse latency envelope.
    pub latency_envelope_class: LatencyEnvelopeClass,
    /// Token ceiling bucket.
    pub token_ceiling_class: TokenCeilingClass,
    /// Tool-call ceiling bucket.
    pub tool_call_ceiling_class: ToolCallCeilingClass,
    /// Wall-time ceiling bucket.
    pub wall_time_ceiling_class: WallTimeCeilingClass,
    /// Stable lower-is-better route priority authored by policy.
    pub route_priority: u16,
    /// Feature classes supported by this provider route.
    #[serde(default)]
    pub supported_feature_classes: Vec<AiFeatureClass>,
    /// Data classes admitted as prompt input.
    #[serde(default)]
    pub allowed_data_classes: Vec<RegistryDataClass>,
    /// Data classes denied by policy on this route.
    #[serde(default)]
    pub denied_data_classes: Vec<RegistryDataClass>,
    /// Model rows served by this provider route.
    #[serde(default)]
    pub model_entry_refs: Vec<String>,
    /// Local model-pack refs served by this provider route.
    #[serde(default)]
    pub local_model_pack_refs: Vec<String>,
    /// Route choices policy allows for this row.
    #[serde(default)]
    pub policy_allowed_route_choices: Vec<RegistryRoutingPolicyClass>,
    /// Eligibility state after policy evaluation.
    pub route_eligibility_class: RouteEligibilityClass,
    /// Lifecycle state.
    pub lifecycle_state_class: RegistryLifecycleStateClass,
    /// Approval posture.
    pub approval_posture_class: RegistryApprovalPostureClass,
    /// Opaque disclosure ref for this route choice.
    pub route_disclosure_ref: String,
    /// Opaque budget owner ref.
    pub budget_owner_ref: String,
    /// Opaque budget-routing policy ref.
    pub budget_routing_policy_ref: String,
    /// Opaque graduation packet ref.
    pub graduation_packet_ref: String,
    /// Opaque envelope evidence ref.
    pub envelope_evidence_ref: String,
    /// Local continuity label shown when hosted capability is unavailable.
    pub local_continuity_label: String,
    /// Export-safe explanation for this provider route.
    pub explanation_label: String,
}

impl ProviderRegistryEntry {
    /// True when the provider advertises the requested feature.
    pub fn supports_feature(&self, feature: AiFeatureClass) -> bool {
        self.supported_feature_classes.contains(&feature)
    }

    /// True when policy and lifecycle allow new dispatch.
    pub fn admits_new_dispatch(&self) -> bool {
        let policy_and_lifecycle = self.route_eligibility_class.is_eligible()
            && self.lifecycle_state_class.admits_new_dispatch();
        policy_and_lifecycle
            && (!self.execution_locus_class.is_hosted_model_path()
                || !self.quota_state_class.blocks_hosted_dispatch())
    }

    /// True when this route runs on the local device.
    pub fn is_local_route(&self) -> bool {
        execution_locus_is_local(self.execution_locus_class)
    }
}

/// Model registry row with family and capability metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRegistryEntry {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable model entry id.
    pub model_entry_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Stable family label shared across loci when parity permits.
    pub model_family_label: String,
    /// Stable variant label.
    pub model_variant_label: String,
    /// Model capability version.
    pub model_capability_version: String,
    /// Lifecycle state.
    pub lifecycle_state_class: RegistryLifecycleStateClass,
    /// Feature classes supported by this model row.
    #[serde(default)]
    pub supported_feature_classes: Vec<AiFeatureClass>,
    /// Providers that serve this model row.
    #[serde(default)]
    pub served_by_provider_entry_refs: Vec<String>,
    /// Cost visibility posture.
    pub cost_visibility_class: CostVisibilityClass,
    /// Cost envelope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Latency envelope.
    pub latency_envelope_class: LatencyEnvelopeClass,
}

impl ModelRegistryEntry {
    /// True when the model advertises the requested feature.
    pub fn supports_feature(&self, feature: AiFeatureClass) -> bool {
        self.supported_feature_classes.contains(&feature)
    }
}

/// External tool row governed alongside AI provider routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalToolRegistryEntry {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable tool entry id.
    pub tool_entry_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Tool family label.
    pub tool_family_label: String,
    /// Tool capability version.
    pub tool_capability_version: String,
    /// Tool transport class.
    pub tool_transport_class: ExternalToolTransportClass,
    /// Tool execution locus.
    pub tool_execution_locus_class: ExternalToolExecutionLocusClass,
    /// Tool auth mode.
    pub tool_auth_mode_class: RegistryAuthModeClass,
    /// Output trust posture token safe for support exports.
    pub tool_output_trust_posture_label: String,
    /// Allowed side effects.
    #[serde(default)]
    pub allowed_side_effect_classes: Vec<ExternalToolSideEffectClass>,
    /// Data classes admitted as tool input.
    #[serde(default)]
    pub allowed_data_classes: Vec<RegistryDataClass>,
    /// Data classes denied by policy on this tool.
    #[serde(default)]
    pub denied_data_classes: Vec<RegistryDataClass>,
    /// Route choices policy allows for this tool row.
    #[serde(default)]
    pub policy_allowed_route_choices: Vec<RegistryRoutingPolicyClass>,
    /// Lifecycle state.
    pub lifecycle_state_class: RegistryLifecycleStateClass,
    /// Approval posture.
    pub approval_posture_class: RegistryApprovalPostureClass,
    /// Required approval-ticket ref when the row needs approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_approval_ticket_ref: Option<String>,
    /// Export-safe explanation for this tool row.
    pub explanation_label: String,
}

impl ExternalToolRegistryEntry {
    /// True when this tool has at least one mutating or provider-visible side effect.
    pub fn has_mutating_side_effect(&self) -> bool {
        self.allowed_side_effect_classes
            .iter()
            .any(|side_effect| side_effect.requires_approval_gate())
    }
}

/// Claimed AI surface that must read the registry for provider truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimedAiSurface {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable surface id.
    pub surface_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Required feature class for the route.
    pub required_feature_class: AiFeatureClass,
    /// Route policy ref used by this surface.
    pub routing_policy_id_ref: String,
    /// Provider rows this surface may use.
    #[serde(default)]
    pub allowed_provider_entry_refs: Vec<String>,
    /// Model rows this surface may use.
    #[serde(default)]
    pub allowed_model_entry_refs: Vec<String>,
    /// External tool rows this surface may invoke.
    #[serde(default)]
    pub allowed_external_tool_entry_refs: Vec<String>,
    /// Required disclosures before dispatch.
    #[serde(default)]
    pub required_disclosure_kinds: Vec<RegistryDisclosureKind>,
    /// Retrieval/index truth for the surface.
    pub retrieval_truth_state_class: RetrievalTruthStateClass,
    /// UI projection ref expected to read this registry state.
    pub ui_projection_ref: String,
    /// Docs/help projection ref expected to read this registry state.
    pub docs_projection_ref: String,
    /// Support-export projection ref expected to read this registry state.
    pub support_export_projection_ref: String,
}

/// Projection registration proving a consumer reads one registry state ref.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface_class: RegistryConsumerSurfaceClass,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Registry state ref consumed by the projection.
    pub registry_state_ref: String,
    /// Absolute timestamp the projection was generated.
    pub rendered_at: String,
}

/// Provider/model registry packet consumed by routing, UI, docs, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelRegistryPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable registry id.
    pub registry_id: String,
    /// Policy context used to mint the registry.
    pub policy_context: RoutingPolicyContext,
    /// Route policies available to claimed surfaces.
    #[serde(default)]
    pub route_policies: Vec<RegistryRoutePolicy>,
    /// Provider rows.
    #[serde(default)]
    pub provider_entries: Vec<ProviderRegistryEntry>,
    /// Model rows.
    #[serde(default)]
    pub model_entries: Vec<ModelRegistryEntry>,
    /// External-tool rows.
    #[serde(default)]
    pub external_tool_entries: Vec<ExternalToolRegistryEntry>,
    /// Claimed AI surfaces.
    #[serde(default)]
    pub claimed_surfaces: Vec<ClaimedAiSurface>,
    /// Consumer projections expected to read this registry state.
    #[serde(default)]
    pub consumer_projections: Vec<RegistryConsumerProjection>,
    /// Source contracts consumed by this packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Timestamp the registry state was minted.
    pub minted_at: String,
}

impl ProviderModelRegistryPacket {
    /// Validates the registry packet against beta routing and truth invariants.
    pub fn validate(&self) -> Vec<ProviderModelRegistryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PROVIDER_MODEL_REGISTRY_PACKET_RECORD_KIND {
            violations.push(ProviderModelRegistryViolation::WrongRecordKind);
        }
        if self.schema_version != PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION {
            violations.push(ProviderModelRegistryViolation::WrongSchemaVersion);
        }
        if self.registry_id.trim().is_empty() {
            violations.push(ProviderModelRegistryViolation::MissingRegistryId);
        }
        if self.policy_context.policy_epoch_ref.trim().is_empty() {
            violations.push(ProviderModelRegistryViolation::MissingPolicyEpochRef);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(ProviderModelRegistryViolation::MissingSourceContractRefs);
        }
        if self.provider_entries.is_empty() {
            violations.push(ProviderModelRegistryViolation::MissingProviderEntries);
        }
        if self.model_entries.is_empty() {
            violations.push(ProviderModelRegistryViolation::MissingModelEntries);
        }

        let provider_by_id = self.provider_by_id();
        let model_by_id = self.model_by_id();
        let policy_by_id = self.policy_by_id();

        for provider in &self.provider_entries {
            if provider.record_kind != PROVIDER_MODEL_REGISTRY_PROVIDER_ENTRY_RECORD_KIND
                || provider.schema_version != PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION
            {
                violations.push(ProviderModelRegistryViolation::ProviderRowWrongEnvelope);
            }
            if provider.provider_entry_id.trim().is_empty()
                || provider.display_label.trim().is_empty()
                || provider.provider_family_label.trim().is_empty()
            {
                violations.push(ProviderModelRegistryViolation::ProviderMissingIdentity);
            }
            if provider.execution_locus_class == ExecutionLocusClass::DisabledNoLocus
                && provider.lifecycle_state_class.admits_new_dispatch()
            {
                violations.push(ProviderModelRegistryViolation::ProviderMissingExecutionLocation);
            }
            if provider.model_entry_refs.is_empty() {
                violations.push(ProviderModelRegistryViolation::ProviderMissingModelRefs);
            }
            if provider.policy_allowed_route_choices.is_empty() {
                violations.push(ProviderModelRegistryViolation::ProviderMissingRouteChoices);
            }
            if provider.route_disclosure_ref.trim().is_empty() {
                violations.push(ProviderModelRegistryViolation::ProviderMissingRouteDisclosure);
            }
        }

        for model in &self.model_entries {
            if model.record_kind != PROVIDER_MODEL_REGISTRY_MODEL_ENTRY_RECORD_KIND
                || model.schema_version != PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION
            {
                violations.push(ProviderModelRegistryViolation::ModelRowWrongEnvelope);
            }
            if model.model_entry_id.trim().is_empty()
                || model.display_label.trim().is_empty()
                || model.model_family_label.trim().is_empty()
                || model.model_capability_version.trim().is_empty()
            {
                violations.push(ProviderModelRegistryViolation::ModelMissingIdentity);
            }
            for provider_ref in &model.served_by_provider_entry_refs {
                if !provider_by_id.contains_key(provider_ref.as_str()) {
                    violations.push(ProviderModelRegistryViolation::ModelReferencesMissingProvider);
                }
            }
        }

        for tool in &self.external_tool_entries {
            if tool.record_kind != PROVIDER_MODEL_REGISTRY_EXTERNAL_TOOL_ENTRY_RECORD_KIND
                || tool.schema_version != PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION
            {
                violations.push(ProviderModelRegistryViolation::ExternalToolRowWrongEnvelope);
            }
            if tool.tool_entry_id.trim().is_empty()
                || tool.display_label.trim().is_empty()
                || tool.tool_family_label.trim().is_empty()
            {
                violations.push(ProviderModelRegistryViolation::ExternalToolMissingIdentity);
            }
            if tool.policy_allowed_route_choices.is_empty() {
                violations.push(ProviderModelRegistryViolation::ExternalToolMissingRouteChoices);
            }
            if tool.has_mutating_side_effect()
                && !tool.approval_posture_class.requires_approval_gate()
            {
                violations
                    .push(ProviderModelRegistryViolation::ExternalToolMutatingWithoutApproval);
            }
        }

        for policy in &self.route_policies {
            if policy.route_policy_id.trim().is_empty() || policy.policy_epoch_ref.trim().is_empty()
            {
                violations.push(ProviderModelRegistryViolation::RoutePolicyMissingIdentity);
            }
            for provider_ref in &policy.allowed_provider_entry_refs {
                if !provider_by_id.contains_key(provider_ref.as_str()) {
                    violations
                        .push(ProviderModelRegistryViolation::RoutePolicyReferencesMissingProvider);
                }
            }
            for model_ref in &policy.allowed_model_entry_refs {
                if !model_by_id.contains_key(model_ref.as_str()) {
                    violations
                        .push(ProviderModelRegistryViolation::RoutePolicyReferencesMissingModel);
                }
            }
            if matches!(
                policy.policy_class,
                RegistryRoutingPolicyClass::LocalFirstThenCheapest
            ) && !policy
                .allowed_provider_entry_refs
                .iter()
                .any(|provider_ref| {
                    provider_by_id
                        .get(provider_ref.as_str())
                        .is_some_and(|provider| {
                            provider.is_local_route() && provider.admits_new_dispatch()
                        })
                })
            {
                violations.push(ProviderModelRegistryViolation::LocalFirstPolicyMissingLocalRoute);
            }
            if matches!(
                policy.policy_class,
                RegistryRoutingPolicyClass::CheapestQualifying
            ) && !policy
                .allowed_provider_entry_refs
                .iter()
                .any(|provider_ref| {
                    provider_by_id
                        .get(provider_ref.as_str())
                        .is_some_and(|provider| provider.admits_new_dispatch())
                })
            {
                violations.push(ProviderModelRegistryViolation::CheapestPolicyMissingEligibleRoute);
            }
        }

        for surface in &self.claimed_surfaces {
            if surface.record_kind != PROVIDER_MODEL_REGISTRY_CLAIMED_SURFACE_RECORD_KIND
                || surface.schema_version != PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION
            {
                violations.push(ProviderModelRegistryViolation::ClaimedSurfaceWrongEnvelope);
            }
            if !policy_by_id.contains_key(surface.routing_policy_id_ref.as_str()) {
                violations.push(ProviderModelRegistryViolation::ClaimedSurfaceMissingRoutePolicy);
            }
            if surface.allowed_provider_entry_refs.is_empty() {
                violations
                    .push(ProviderModelRegistryViolation::ClaimedSurfaceMissingProviderChoice);
            }
            if surface.allowed_model_entry_refs.is_empty() {
                violations.push(ProviderModelRegistryViolation::ClaimedSurfaceMissingModelChoice);
            }
            for required in REQUIRED_PROVIDER_DISCLOSURES {
                if !surface.required_disclosure_kinds.contains(required) {
                    violations
                        .push(ProviderModelRegistryViolation::ClaimedSurfaceMissingDisclosure);
                }
            }
            if surface.retrieval_truth_state_class
                == RetrievalTruthStateClass::UnlabelledPartialBlocked
            {
                violations.push(ProviderModelRegistryViolation::UnlabelledRetrievalState);
            }
            if !self
                .projection_matches(RegistryConsumerSurfaceClass::Ui, &surface.ui_projection_ref)
                || !self.projection_matches(
                    RegistryConsumerSurfaceClass::Docs,
                    &surface.docs_projection_ref,
                )
                || !self.projection_matches(
                    RegistryConsumerSurfaceClass::SupportExport,
                    &surface.support_export_projection_ref,
                )
            {
                violations.push(ProviderModelRegistryViolation::MissingUiDocsSupportProjection);
            }
            if self.projection_drifted(RegistryConsumerSurfaceClass::Ui, &surface.ui_projection_ref)
                || self.projection_drifted(
                    RegistryConsumerSurfaceClass::Docs,
                    &surface.docs_projection_ref,
                )
                || self.projection_drifted(
                    RegistryConsumerSurfaceClass::SupportExport,
                    &surface.support_export_projection_ref,
                )
            {
                violations.push(ProviderModelRegistryViolation::ConsumerProjectionDrift);
            }
            for provider_ref in &surface.allowed_provider_entry_refs {
                if !provider_by_id.contains_key(provider_ref.as_str()) {
                    violations.push(
                        ProviderModelRegistryViolation::ClaimedSurfaceReferencesMissingProvider,
                    );
                }
            }
            for model_ref in &surface.allowed_model_entry_refs {
                if !model_by_id.contains_key(model_ref.as_str()) {
                    violations
                        .push(ProviderModelRegistryViolation::ClaimedSurfaceReferencesMissingModel);
                }
            }
        }

        if self.contains_forbidden_boundary_material() {
            violations.push(ProviderModelRegistryViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Resolves the route for a claimed surface from registry state.
    pub fn resolve_route_for_surface(&self, surface_id: &str) -> ProviderModelRouteResolution {
        let Some(surface) = self
            .claimed_surfaces
            .iter()
            .find(|candidate| candidate.surface_id == surface_id)
        else {
            return ProviderModelRouteResolution::missing_surface(
                surface_id,
                self.registry_id.clone(),
            );
        };
        let Some(policy) = self
            .route_policies
            .iter()
            .find(|policy| policy.route_policy_id == surface.routing_policy_id_ref)
        else {
            return ProviderModelRouteResolution::missing_policy(surface, self.registry_id.clone());
        };

        let mut candidates = self.route_candidates(surface, policy);
        let eligible_indices: Vec<usize> = candidates
            .iter()
            .enumerate()
            .filter_map(|(index, candidate)| {
                candidate
                    .route_eligibility_class
                    .is_eligible()
                    .then_some(index)
            })
            .collect();

        let (selected_index, reason_class) = match policy.policy_class {
            RegistryRoutingPolicyClass::LocalFirstThenCheapest => {
                if let Some(index) = cheapest_index(
                    &eligible_indices
                        .iter()
                        .copied()
                        .filter(|index| {
                            execution_locus_is_local(candidates[*index].execution_locus_class)
                        })
                        .collect::<Vec<_>>(),
                    &candidates,
                ) {
                    (
                        Some(index),
                        RegistryRouteReasonClass::LocalFirstEligibleRouteAdmitted,
                    )
                } else {
                    (
                        cheapest_index(&eligible_indices, &candidates),
                        RegistryRouteReasonClass::CheapestQualifyingRouteAdmitted,
                    )
                }
            }
            RegistryRoutingPolicyClass::CheapestQualifying => (
                cheapest_index(&eligible_indices, &candidates),
                RegistryRouteReasonClass::CheapestQualifyingRouteAdmitted,
            ),
            RegistryRoutingPolicyClass::PolicyPinned => (
                eligible_indices.first().copied(),
                RegistryRouteReasonClass::PolicyPinnedRouteAdmitted,
            ),
            RegistryRoutingPolicyClass::ManualOnly => {
                (None, RegistryRouteReasonClass::ManualSelectionRequired)
            }
            RegistryRoutingPolicyClass::Disabled => {
                (None, RegistryRouteReasonClass::NoEligibleRoute)
            }
        };

        let reason_class = if selected_index.is_none()
            && reason_class != RegistryRouteReasonClass::ManualSelectionRequired
        {
            RegistryRouteReasonClass::NoEligibleRoute
        } else {
            reason_class
        };

        for (index, candidate) in candidates.iter_mut().enumerate() {
            candidate.selected = Some(index) == selected_index;
        }
        let selected_candidate = selected_index.map(|index| candidates[index].clone());

        ProviderModelRouteResolution {
            surface_id: surface.surface_id.clone(),
            registry_state_ref: self.registry_id.clone(),
            route_policy_id_ref: policy.route_policy_id.clone(),
            policy_class: policy.policy_class,
            route_reason_class: reason_class,
            selected_candidate,
            candidates,
        }
    }

    /// Projects one set of rows for UI, docs/help, CLI, or support surfaces.
    pub fn surface_rows_for(&self, surface_id: &str) -> Vec<ProviderModelRegistrySurfaceRow> {
        let resolution = self.resolve_route_for_surface(surface_id);
        let surface = self
            .claimed_surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id);
        let mut rows = Vec::new();

        if let Some(surface) = surface {
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "surface",
                "Surface",
                &surface.display_label,
                &surface.surface_id,
                &self.registry_id,
            ));
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "retrieval_truth",
                "Retrieval truth",
                surface.retrieval_truth_state_class.as_str(),
                surface.retrieval_truth_state_class.as_str(),
                &self.registry_id,
            ));
        }

        rows.push(ProviderModelRegistrySurfaceRow::new(
            "route_policy",
            "Route policy",
            resolution.policy_class.as_str(),
            &resolution.route_policy_id_ref,
            &self.registry_id,
        ));
        rows.push(ProviderModelRegistrySurfaceRow::new(
            "route_reason",
            "Route reason",
            resolution.route_reason_class.as_str(),
            resolution.route_reason_class.as_str(),
            &self.registry_id,
        ));

        if let Some(selected) = &resolution.selected_candidate {
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "provider_family",
                "Provider family",
                &selected.provider_family_label,
                &selected.provider_entry_ref,
                &self.registry_id,
            ));
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "model_family",
                "Model family",
                &selected.model_family_label,
                &selected.model_entry_ref,
                &self.registry_id,
            ));
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "execution_location",
                "Execution location",
                selected.execution_locus_class.as_str(),
                selected.execution_locus_class.as_str(),
                &self.registry_id,
            ));
            rows.push(ProviderModelRegistrySurfaceRow::new(
                "policy_allowed_route_choices",
                "Policy-allowed route choices",
                &join_route_choices(&selected.policy_allowed_route_choices),
                &join_route_choices(&selected.policy_allowed_route_choices),
                &self.registry_id,
            ));
        }

        rows
    }

    /// Projects an export-safe support packet from the same registry state.
    pub fn support_export_projection(&self) -> ProviderModelRegistrySupportExport {
        let surface_summaries = self
            .claimed_surfaces
            .iter()
            .map(|surface| {
                let resolution = self.resolve_route_for_surface(&surface.surface_id);
                RegistrySurfaceSupportSummary {
                    surface_id: surface.surface_id.clone(),
                    display_label: surface.display_label.clone(),
                    registry_state_ref: self.registry_id.clone(),
                    route_policy_id_ref: surface.routing_policy_id_ref.clone(),
                    policy_class_token: resolution.policy_class.as_str().to_owned(),
                    route_reason_token: resolution.route_reason_class.as_str().to_owned(),
                    selected_provider_entry_ref: resolution
                        .selected_candidate
                        .as_ref()
                        .map(|candidate| candidate.provider_entry_ref.clone()),
                    selected_model_entry_ref: resolution
                        .selected_candidate
                        .as_ref()
                        .map(|candidate| candidate.model_entry_ref.clone()),
                    execution_location_token: resolution
                        .selected_candidate
                        .as_ref()
                        .map(|candidate| candidate.execution_locus_class.as_str().to_owned()),
                    retrieval_truth_token: surface.retrieval_truth_state_class.as_str().to_owned(),
                    surface_rows: self.surface_rows_for(&surface.surface_id),
                }
            })
            .collect();

        ProviderModelRegistrySupportExport {
            record_kind: PROVIDER_MODEL_REGISTRY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_MODEL_REGISTRY_SCHEMA_VERSION,
            support_export_id: format!(
                "support-export:provider-model-registry:{}",
                self.registry_id
            ),
            registry_state_ref: self.registry_id.clone(),
            policy_epoch_ref: self.policy_context.policy_epoch_ref.clone(),
            trust_state_token: self.policy_context.trust_state.as_str().to_owned(),
            deployment_profile_token: self
                .policy_context
                .deployment_profile_class
                .as_str()
                .to_owned(),
            provider_summaries: self
                .provider_entries
                .iter()
                .map(ProviderRegistrySupportSummary::from)
                .collect(),
            model_summaries: self
                .model_entries
                .iter()
                .map(ModelRegistrySupportSummary::from)
                .collect(),
            external_tool_summaries: self
                .external_tool_entries
                .iter()
                .map(ExternalToolRegistrySupportSummary::from)
                .collect(),
            surface_summaries,
            validation_violation_tokens: self
                .validate()
                .into_iter()
                .map(|violation| violation.as_str().to_owned())
                .collect(),
            source_contract_refs: self.source_contract_refs.clone(),
            minted_at: self.minted_at.clone(),
        }
    }

    /// Builds the existing routing packet from the registry-selected route.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderModelRegistryViolation::NoEligibleRouteForSurface`]
    /// when the surface cannot resolve to an admitted route.
    pub fn routing_packet_for_surface(
        &self,
        surface_id: &str,
        routing_packet_id: impl Into<String>,
        request_workspace_ref: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Result<AiRoutingPacket, ProviderModelRegistryViolation> {
        let resolution = self.resolve_route_for_surface(surface_id);
        let selected = resolution
            .selected_candidate
            .as_ref()
            .ok_or(ProviderModelRegistryViolation::NoEligibleRouteForSurface)?;
        let surface = self
            .claimed_surfaces
            .iter()
            .find(|surface| surface.surface_id == surface_id)
            .ok_or(ProviderModelRegistryViolation::NoEligibleRouteForSurface)?;

        let ai_candidates = resolution
            .candidates
            .iter()
            .filter_map(|candidate| self.ai_route_candidate_from_registry(candidate, &resolution))
            .collect::<Vec<_>>();
        let route_change_lineage =
            self.route_change_lineage_for(surface, selected, &resolution, &ai_candidates);

        let mut source_contract_refs = self.source_contract_refs.clone();
        if !source_contract_refs
            .iter()
            .any(|contract| contract == "docs/ai/m3/provider_model_registry_beta.md")
        {
            source_contract_refs.push("docs/ai/m3/provider_model_registry_beta.md".to_owned());
        }

        Ok(AiRoutingPacket::new(
            routing_packet_id,
            surface_id,
            request_workspace_ref,
            RoutingRunStateClass::PreviewPreDispatch,
            self.policy_context.clone(),
            "capability_lifecycle:beta.ai.provider_model_registry",
            Some("identity_mode_baseline:beta:local_byok_managed_truth".to_owned()),
            ai_candidates,
            selected.candidate_id.clone(),
            route_change_lineage,
            source_contract_refs,
            minted_at,
        ))
    }

    fn route_candidates(
        &self,
        surface: &ClaimedAiSurface,
        policy: &RegistryRoutePolicy,
    ) -> Vec<RegistryRouteCandidate> {
        let model_by_id = self.model_by_id();
        policy
            .allowed_provider_entry_refs
            .iter()
            .filter(|provider_ref| surface.allowed_provider_entry_refs.contains(provider_ref))
            .filter_map(|provider_ref| {
                let provider = self
                    .provider_entries
                    .iter()
                    .find(|provider| &provider.provider_entry_id == provider_ref)?;
                let model = provider
                    .model_entry_refs
                    .iter()
                    .filter(|model_ref| {
                        surface.allowed_model_entry_refs.contains(model_ref)
                            && policy.allowed_model_entry_refs.contains(model_ref)
                    })
                    .filter_map(|model_ref| model_by_id.get(model_ref.as_str()).copied())
                    .find(|model| model.supports_feature(surface.required_feature_class))?;

                Some(RegistryRouteCandidate::from_entries(
                    surface,
                    provider,
                    model,
                    route_eligibility_for(provider, model, surface.required_feature_class),
                ))
            })
            .collect()
    }

    fn ai_route_candidate_from_registry(
        &self,
        candidate: &RegistryRouteCandidate,
        resolution: &ProviderModelRouteResolution,
    ) -> Option<AiRouteCandidate> {
        let provider = self
            .provider_entries
            .iter()
            .find(|provider| provider.provider_entry_id == candidate.provider_entry_ref)?;
        Some(AiRouteCandidate {
            candidate_id: candidate.candidate_id.clone(),
            provider_entry_ref: candidate.provider_entry_ref.clone(),
            provider_label: candidate.provider_label.clone(),
            provider_class: candidate.provider_class,
            model_entry_ref: candidate.model_entry_ref.clone(),
            model_label: candidate.model_label.clone(),
            execution_locus_class: candidate.execution_locus_class,
            route_origin_class: candidate.route_origin_class,
            region_posture_class: provider.region_posture_class,
            retention_stance_class: provider.retention_stance_class,
            quota: QuotaInspector {
                quota_family_class: provider.quota_family_class,
                quota_state_class: provider.quota_state_class,
                quota_scope_class: provider.quota_scope_class,
                budget_owner_ref: provider.budget_owner_ref.clone(),
                quota_meter_ref: None,
                quota_forecast_ref: None,
                usage_export_ref: None,
                explanation_label: provider.explanation_label.clone(),
                local_continuity_label: provider.local_continuity_label.clone(),
                recovery_action_ref: Some("action:ai-routing:inspect-registry".to_owned()),
            },
            envelope: LatencyCostEnvelope {
                latency_envelope_class: provider.latency_envelope_class,
                cost_envelope_class: provider.cost_envelope_class,
                cost_visibility_class: provider.cost_visibility_class,
                token_ceiling_class: provider.token_ceiling_class,
                tool_call_ceiling_class: provider.tool_call_ceiling_class,
                wall_time_ceiling_class: provider.wall_time_ceiling_class,
                budget_routing_policy_ref: provider.budget_routing_policy_ref.clone(),
                graduation_packet_ref: provider.graduation_packet_ref.clone(),
                envelope_evidence_ref: provider.envelope_evidence_ref.clone(),
                explanation_label: provider.explanation_label.clone(),
            },
            route_selection_reason_class: route_reason_for(resolution.route_reason_class),
            route_selection_override_reason_class: route_override_for(
                resolution.route_reason_class,
            ),
            exhaustion_state_class: exhaustion_state_for(
                candidate.route_eligibility_class,
                provider,
            ),
            selected_outcome_class: selected_outcome_for(candidate, provider),
            route_selection_disclosure_ref: route_reason_for(resolution.route_reason_class)
                .requires_route_change_lineage()
                .then(|| provider.route_disclosure_ref.clone()),
            originating_approval_ticket_ref: None,
            explanation_label: candidate.explanation_label.clone(),
        })
    }

    fn route_change_lineage_for(
        &self,
        surface: &ClaimedAiSurface,
        selected: &RegistryRouteCandidate,
        resolution: &ProviderModelRouteResolution,
        ai_candidates: &[AiRouteCandidate],
    ) -> Vec<RouteChangeLineage> {
        if !route_reason_for(resolution.route_reason_class).requires_route_change_lineage() {
            return Vec::new();
        }
        let Some(provider) = self
            .provider_entries
            .iter()
            .find(|provider| provider.provider_entry_id == selected.provider_entry_ref)
        else {
            return Vec::new();
        };
        let from_candidate_ref = ai_candidates
            .iter()
            .find(|candidate| candidate.candidate_id != selected.candidate_id)
            .map(|candidate| candidate.candidate_id.clone());
        vec![RouteChangeLineage {
            lineage_id: format!("route-lineage:{}:{}", self.registry_id, surface.surface_id),
            cause_class: RouteChangeCauseClass::PolicyOverride,
            from_candidate_ref,
            to_candidate_ref: selected.candidate_id.clone(),
            route_selection_disclosure_ref: provider.route_disclosure_ref.clone(),
            policy_epoch_ref: self.policy_context.policy_epoch_ref.clone(),
            visible_disclosure_label: format!(
                "{} selected {} from registry state {}.",
                resolution.route_reason_class.as_str(),
                selected.provider_label,
                self.registry_id
            ),
        }]
    }

    fn projection_matches(
        &self,
        class: RegistryConsumerSurfaceClass,
        projection_ref: &str,
    ) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface_class == class
                && projection.projection_ref == projection_ref
                && projection.registry_state_ref == self.registry_id
        })
    }

    fn projection_drifted(
        &self,
        class: RegistryConsumerSurfaceClass,
        projection_ref: &str,
    ) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface_class == class
                && projection.projection_ref == projection_ref
                && projection.registry_state_ref != self.registry_id
        })
    }

    fn provider_by_id(&self) -> BTreeMap<&str, &ProviderRegistryEntry> {
        self.provider_entries
            .iter()
            .map(|provider| (provider.provider_entry_id.as_str(), provider))
            .collect()
    }

    fn model_by_id(&self) -> BTreeMap<&str, &ModelRegistryEntry> {
        self.model_entries
            .iter()
            .map(|model| (model.model_entry_id.as_str(), model))
            .collect()
    }

    fn policy_by_id(&self) -> BTreeMap<&str, &RegistryRoutePolicy> {
        self.route_policies
            .iter()
            .map(|policy| (policy.route_policy_id.as_str(), policy))
            .collect()
    }

    fn contains_forbidden_boundary_material(&self) -> bool {
        serde_json::to_value(self)
            .ok()
            .is_some_and(|value| json_contains_forbidden_boundary_material(&value))
    }
}

/// Resolved route for one claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelRouteResolution {
    /// Surface that was resolved.
    pub surface_id: String,
    /// Registry state ref that produced the resolution.
    pub registry_state_ref: String,
    /// Route policy ref used for selection.
    pub route_policy_id_ref: String,
    /// Route policy class.
    pub policy_class: RegistryRoutingPolicyClass,
    /// Route reason class.
    pub route_reason_class: RegistryRouteReasonClass,
    /// Selected candidate, when one is admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_candidate: Option<RegistryRouteCandidate>,
    /// All candidates considered.
    #[serde(default)]
    pub candidates: Vec<RegistryRouteCandidate>,
}

impl ProviderModelRouteResolution {
    fn missing_surface(surface_id: &str, registry_state_ref: String) -> Self {
        Self {
            surface_id: surface_id.to_owned(),
            registry_state_ref,
            route_policy_id_ref: String::new(),
            policy_class: RegistryRoutingPolicyClass::Disabled,
            route_reason_class: RegistryRouteReasonClass::NoEligibleRoute,
            selected_candidate: None,
            candidates: Vec::new(),
        }
    }

    fn missing_policy(surface: &ClaimedAiSurface, registry_state_ref: String) -> Self {
        Self {
            surface_id: surface.surface_id.clone(),
            registry_state_ref,
            route_policy_id_ref: surface.routing_policy_id_ref.clone(),
            policy_class: RegistryRoutingPolicyClass::Disabled,
            route_reason_class: RegistryRouteReasonClass::NoEligibleRoute,
            selected_candidate: None,
            candidates: Vec::new(),
        }
    }
}

/// Candidate considered by the registry route resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistryRouteCandidate {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Provider entry ref.
    pub provider_entry_ref: String,
    /// Provider display label.
    pub provider_label: String,
    /// Provider family label.
    pub provider_family_label: String,
    /// Provider class.
    pub provider_class: AiRouteProviderClass,
    /// Model entry ref.
    pub model_entry_ref: String,
    /// Model display label.
    pub model_label: String,
    /// Model family label.
    pub model_family_label: String,
    /// Execution locus.
    pub execution_locus_class: ExecutionLocusClass,
    /// Route origin.
    pub route_origin_class: RouteOriginClass,
    /// Route choices allowed by policy.
    #[serde(default)]
    pub policy_allowed_route_choices: Vec<RegistryRoutingPolicyClass>,
    /// Eligibility state.
    pub route_eligibility_class: RouteEligibilityClass,
    /// Lifecycle state.
    pub lifecycle_state_class: RegistryLifecycleStateClass,
    /// Quota state.
    pub quota_state_class: QuotaStateClass,
    /// Cost envelope.
    pub cost_envelope_class: CostEnvelopeClass,
    /// Latency envelope.
    pub latency_envelope_class: LatencyEnvelopeClass,
    /// Lower-is-better route priority.
    pub route_priority: u16,
    /// Whether this candidate was selected.
    pub selected: bool,
    /// Export-safe explanation.
    pub explanation_label: String,
}

impl RegistryRouteCandidate {
    fn from_entries(
        surface: &ClaimedAiSurface,
        provider: &ProviderRegistryEntry,
        model: &ModelRegistryEntry,
        route_eligibility_class: RouteEligibilityClass,
    ) -> Self {
        Self {
            candidate_id: format!(
                "candidate:{}:{}:{}",
                surface.surface_id, provider.provider_entry_id, model.model_entry_id
            ),
            provider_entry_ref: provider.provider_entry_id.clone(),
            provider_label: provider.display_label.clone(),
            provider_family_label: provider.provider_family_label.clone(),
            provider_class: provider.provider_class,
            model_entry_ref: model.model_entry_id.clone(),
            model_label: model.display_label.clone(),
            model_family_label: model.model_family_label.clone(),
            execution_locus_class: provider.execution_locus_class,
            route_origin_class: provider.route_origin_class,
            policy_allowed_route_choices: provider.policy_allowed_route_choices.clone(),
            route_eligibility_class,
            lifecycle_state_class: provider.lifecycle_state_class,
            quota_state_class: provider.quota_state_class,
            cost_envelope_class: provider.cost_envelope_class,
            latency_envelope_class: provider.latency_envelope_class,
            route_priority: provider.route_priority,
            selected: false,
            explanation_label: provider.explanation_label.clone(),
        }
    }
}

/// One projected row for UI, docs/help, CLI, or support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelRegistrySurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// Display label.
    pub label: String,
    /// Human-readable value.
    pub value_label: String,
    /// Token or opaque ref backing the value.
    pub value_token: String,
    /// Registry state ref the row was read from.
    pub registry_state_ref: String,
}

impl ProviderModelRegistrySurfaceRow {
    fn new(
        row_id: &str,
        label: &str,
        value_label: &str,
        value_token: &str,
        registry_state_ref: &str,
    ) -> Self {
        Self {
            row_id: row_id.to_owned(),
            label: label.to_owned(),
            value_label: value_label.to_owned(),
            value_token: value_token.to_owned(),
            registry_state_ref: registry_state_ref.to_owned(),
        }
    }
}

/// Export-safe provider summary in [`ProviderModelRegistrySupportExport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderRegistrySupportSummary {
    /// Provider entry ref.
    pub provider_entry_ref: String,
    /// Provider label.
    pub display_label: String,
    /// Provider family label.
    pub provider_family_label: String,
    /// Provider class token.
    pub provider_class_token: String,
    /// Execution location token.
    pub execution_location_token: String,
    /// Route origin token.
    pub route_origin_token: String,
    /// Policy-allowed route choices.
    #[serde(default)]
    pub policy_allowed_route_choice_tokens: Vec<String>,
    /// Eligibility token.
    pub eligibility_token: String,
}

impl From<&ProviderRegistryEntry> for ProviderRegistrySupportSummary {
    fn from(provider: &ProviderRegistryEntry) -> Self {
        Self {
            provider_entry_ref: provider.provider_entry_id.clone(),
            display_label: provider.display_label.clone(),
            provider_family_label: provider.provider_family_label.clone(),
            provider_class_token: provider.provider_class.as_str().to_owned(),
            execution_location_token: provider.execution_locus_class.as_str().to_owned(),
            route_origin_token: provider.route_origin_class.as_str().to_owned(),
            policy_allowed_route_choice_tokens: provider
                .policy_allowed_route_choices
                .iter()
                .map(|choice| choice.as_str().to_owned())
                .collect(),
            eligibility_token: provider.route_eligibility_class.as_str().to_owned(),
        }
    }
}

/// Export-safe model summary in [`ProviderModelRegistrySupportExport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelRegistrySupportSummary {
    /// Model entry ref.
    pub model_entry_ref: String,
    /// Model label.
    pub display_label: String,
    /// Model family label.
    pub model_family_label: String,
    /// Model capability version.
    pub model_capability_version: String,
    /// Lifecycle token.
    pub lifecycle_token: String,
}

impl From<&ModelRegistryEntry> for ModelRegistrySupportSummary {
    fn from(model: &ModelRegistryEntry) -> Self {
        Self {
            model_entry_ref: model.model_entry_id.clone(),
            display_label: model.display_label.clone(),
            model_family_label: model.model_family_label.clone(),
            model_capability_version: model.model_capability_version.clone(),
            lifecycle_token: model.lifecycle_state_class.as_str().to_owned(),
        }
    }
}

/// Export-safe external-tool summary in [`ProviderModelRegistrySupportExport`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalToolRegistrySupportSummary {
    /// Tool entry ref.
    pub tool_entry_ref: String,
    /// Tool label.
    pub display_label: String,
    /// Tool family label.
    pub tool_family_label: String,
    /// Transport token.
    pub tool_transport_token: String,
    /// Execution-location token.
    pub tool_execution_location_token: String,
    /// Approval posture token.
    pub approval_posture_token: String,
    /// Side-effect tokens.
    #[serde(default)]
    pub allowed_side_effect_tokens: Vec<String>,
}

impl From<&ExternalToolRegistryEntry> for ExternalToolRegistrySupportSummary {
    fn from(tool: &ExternalToolRegistryEntry) -> Self {
        Self {
            tool_entry_ref: tool.tool_entry_id.clone(),
            display_label: tool.display_label.clone(),
            tool_family_label: tool.tool_family_label.clone(),
            tool_transport_token: tool.tool_transport_class.as_str().to_owned(),
            tool_execution_location_token: tool.tool_execution_locus_class.as_str().to_owned(),
            approval_posture_token: tool.approval_posture_class.as_str().to_owned(),
            allowed_side_effect_tokens: tool
                .allowed_side_effect_classes
                .iter()
                .map(|side_effect| side_effect.as_str().to_owned())
                .collect(),
        }
    }
}

/// Export-safe summary for one claimed surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegistrySurfaceSupportSummary {
    /// Surface id.
    pub surface_id: String,
    /// Surface display label.
    pub display_label: String,
    /// Registry state ref used by this summary.
    pub registry_state_ref: String,
    /// Route policy ref.
    pub route_policy_id_ref: String,
    /// Policy class token.
    pub policy_class_token: String,
    /// Route reason token.
    pub route_reason_token: String,
    /// Selected provider ref, when one was admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_provider_entry_ref: Option<String>,
    /// Selected model ref, when one was admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_model_entry_ref: Option<String>,
    /// Selected execution-location token, when one was admitted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_location_token: Option<String>,
    /// Retrieval/index truth token.
    pub retrieval_truth_token: String,
    /// Rows projected from the same registry state.
    #[serde(default)]
    pub surface_rows: Vec<ProviderModelRegistrySurfaceRow>,
}

/// Export-safe registry projection for support bundles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderModelRegistrySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Support export id.
    pub support_export_id: String,
    /// Registry state ref that produced this export.
    pub registry_state_ref: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Workspace trust-state token.
    pub trust_state_token: String,
    /// Deployment-profile token.
    pub deployment_profile_token: String,
    /// Provider summaries.
    #[serde(default)]
    pub provider_summaries: Vec<ProviderRegistrySupportSummary>,
    /// Model summaries.
    #[serde(default)]
    pub model_summaries: Vec<ModelRegistrySupportSummary>,
    /// External-tool summaries.
    #[serde(default)]
    pub external_tool_summaries: Vec<ExternalToolRegistrySupportSummary>,
    /// Surface summaries.
    #[serde(default)]
    pub surface_summaries: Vec<RegistrySurfaceSupportSummary>,
    /// Validation violation tokens present on the source registry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_violation_tokens: Vec<String>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Timestamp the source registry was minted.
    pub minted_at: String,
}

impl ProviderModelRegistrySupportExport {
    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only support export fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("provider/model registry export serializes")
    }
}

/// Validation failures emitted by [`ProviderModelRegistryPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProviderModelRegistryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Registry id is missing.
    MissingRegistryId,
    /// Policy epoch ref is missing.
    MissingPolicyEpochRef,
    /// Source contract refs are missing.
    MissingSourceContractRefs,
    /// Provider entries are missing.
    MissingProviderEntries,
    /// Model entries are missing.
    MissingModelEntries,
    /// Provider row envelope is wrong.
    ProviderRowWrongEnvelope,
    /// Provider identity fields are missing.
    ProviderMissingIdentity,
    /// Provider execution location is missing.
    ProviderMissingExecutionLocation,
    /// Provider model refs are missing.
    ProviderMissingModelRefs,
    /// Provider route choices are missing.
    ProviderMissingRouteChoices,
    /// Provider route disclosure ref is missing.
    ProviderMissingRouteDisclosure,
    /// Model row envelope is wrong.
    ModelRowWrongEnvelope,
    /// Model identity fields are missing.
    ModelMissingIdentity,
    /// Model references a missing provider.
    ModelReferencesMissingProvider,
    /// External-tool row envelope is wrong.
    ExternalToolRowWrongEnvelope,
    /// External-tool identity fields are missing.
    ExternalToolMissingIdentity,
    /// External-tool route choices are missing.
    ExternalToolMissingRouteChoices,
    /// External-tool mutating side effect lacks an approval posture.
    ExternalToolMutatingWithoutApproval,
    /// Route policy identity is missing.
    RoutePolicyMissingIdentity,
    /// Route policy references a missing provider.
    RoutePolicyReferencesMissingProvider,
    /// Route policy references a missing model.
    RoutePolicyReferencesMissingModel,
    /// Local-first policy lacks an eligible local route.
    LocalFirstPolicyMissingLocalRoute,
    /// Cheapest policy lacks an eligible route.
    CheapestPolicyMissingEligibleRoute,
    /// Claimed surface row envelope is wrong.
    ClaimedSurfaceWrongEnvelope,
    /// Claimed surface route policy is missing.
    ClaimedSurfaceMissingRoutePolicy,
    /// Claimed surface has no provider choice.
    ClaimedSurfaceMissingProviderChoice,
    /// Claimed surface has no model choice.
    ClaimedSurfaceMissingModelChoice,
    /// Claimed surface lacks required disclosure kinds.
    ClaimedSurfaceMissingDisclosure,
    /// Claimed surface exposes unlabelled partial retrieval/index state.
    UnlabelledRetrievalState,
    /// Claimed surface lacks UI/docs/support projections from this state.
    MissingUiDocsSupportProjection,
    /// UI/docs/support projections disagree about registry state.
    ConsumerProjectionDrift,
    /// Claimed surface references a missing provider row.
    ClaimedSurfaceReferencesMissingProvider,
    /// Claimed surface references a missing model row.
    ClaimedSurfaceReferencesMissingModel,
    /// No eligible route exists for the requested surface.
    NoEligibleRouteForSurface,
    /// Exportable registry fields contain raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ProviderModelRegistryViolation {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingRegistryId => "missing_registry_id",
            Self::MissingPolicyEpochRef => "missing_policy_epoch_ref",
            Self::MissingSourceContractRefs => "missing_source_contract_refs",
            Self::MissingProviderEntries => "missing_provider_entries",
            Self::MissingModelEntries => "missing_model_entries",
            Self::ProviderRowWrongEnvelope => "provider_row_wrong_envelope",
            Self::ProviderMissingIdentity => "provider_missing_identity",
            Self::ProviderMissingExecutionLocation => "provider_missing_execution_location",
            Self::ProviderMissingModelRefs => "provider_missing_model_refs",
            Self::ProviderMissingRouteChoices => "provider_missing_route_choices",
            Self::ProviderMissingRouteDisclosure => "provider_missing_route_disclosure",
            Self::ModelRowWrongEnvelope => "model_row_wrong_envelope",
            Self::ModelMissingIdentity => "model_missing_identity",
            Self::ModelReferencesMissingProvider => "model_references_missing_provider",
            Self::ExternalToolRowWrongEnvelope => "external_tool_row_wrong_envelope",
            Self::ExternalToolMissingIdentity => "external_tool_missing_identity",
            Self::ExternalToolMissingRouteChoices => "external_tool_missing_route_choices",
            Self::ExternalToolMutatingWithoutApproval => "external_tool_mutating_without_approval",
            Self::RoutePolicyMissingIdentity => "route_policy_missing_identity",
            Self::RoutePolicyReferencesMissingProvider => {
                "route_policy_references_missing_provider"
            }
            Self::RoutePolicyReferencesMissingModel => "route_policy_references_missing_model",
            Self::LocalFirstPolicyMissingLocalRoute => "local_first_policy_missing_local_route",
            Self::CheapestPolicyMissingEligibleRoute => "cheapest_policy_missing_eligible_route",
            Self::ClaimedSurfaceWrongEnvelope => "claimed_surface_wrong_envelope",
            Self::ClaimedSurfaceMissingRoutePolicy => "claimed_surface_missing_route_policy",
            Self::ClaimedSurfaceMissingProviderChoice => "claimed_surface_missing_provider_choice",
            Self::ClaimedSurfaceMissingModelChoice => "claimed_surface_missing_model_choice",
            Self::ClaimedSurfaceMissingDisclosure => "claimed_surface_missing_disclosure",
            Self::UnlabelledRetrievalState => "unlabelled_retrieval_state",
            Self::MissingUiDocsSupportProjection => "missing_ui_docs_support_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ClaimedSurfaceReferencesMissingProvider => {
                "claimed_surface_references_missing_provider"
            }
            Self::ClaimedSurfaceReferencesMissingModel => {
                "claimed_surface_references_missing_model"
            }
            Self::NoEligibleRouteForSurface => "no_eligible_route_for_surface",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

const REQUIRED_PROVIDER_DISCLOSURES: &[RegistryDisclosureKind] = &[
    RegistryDisclosureKind::ProviderIdentityChip,
    RegistryDisclosureKind::ModelIdentityChip,
    RegistryDisclosureKind::ExecutionLocationChip,
    RegistryDisclosureKind::RegionPostureChip,
    RegistryDisclosureKind::RetentionStanceChip,
    RegistryDisclosureKind::DataClassAllowlistReadout,
    RegistryDisclosureKind::RouteChoiceReadout,
];

fn execution_locus_is_local(locus: ExecutionLocusClass) -> bool {
    matches!(
        locus,
        ExecutionLocusClass::LocalInProcess
            | ExecutionLocusClass::LocalSandboxProcess
            | ExecutionLocusClass::LocalCompanionService
    )
}

fn cheapest_index(indices: &[usize], candidates: &[RegistryRouteCandidate]) -> Option<usize> {
    indices.iter().copied().min_by_key(|index| {
        let candidate = &candidates[*index];
        (
            cost_rank(candidate.cost_envelope_class),
            candidate.route_priority,
            candidate.provider_entry_ref.clone(),
        )
    })
}

fn cost_rank(cost: CostEnvelopeClass) -> u16 {
    match cost {
        CostEnvelopeClass::BundledNoIncrementalCost => 0,
        CostEnvelopeClass::FreeTierRateLimited => 1,
        CostEnvelopeClass::FlatFeeSubscriptionBand => 2,
        CostEnvelopeClass::VendorHostedEntitlementBand => 3,
        CostEnvelopeClass::EnterprisePooledQuotaBand => 4,
        CostEnvelopeClass::MeteredPerRequestLowVolumeBand => 10,
        CostEnvelopeClass::MeteredPerTokenLowVolumeBand => 11,
        CostEnvelopeClass::MeteredPerRequestMediumVolumeBand => 20,
        CostEnvelopeClass::MeteredPerTokenMediumVolumeBand => 21,
        CostEnvelopeClass::MeteredPerRequestHighVolumeBand => 30,
        CostEnvelopeClass::MeteredPerTokenHighVolumeBand => 31,
        CostEnvelopeClass::EstimatedUnverifiedBand => 90,
        CostEnvelopeClass::EnvelopeUnknownUnverifiedCost => 100,
    }
}

fn route_eligibility_for(
    provider: &ProviderRegistryEntry,
    model: &ModelRegistryEntry,
    required_feature: AiFeatureClass,
) -> RouteEligibilityClass {
    if !provider.lifecycle_state_class.admits_new_dispatch()
        || !model.lifecycle_state_class.admits_new_dispatch()
    {
        return RouteEligibilityClass::LifecycleBlocked;
    }
    if !provider.route_eligibility_class.is_eligible() {
        return provider.route_eligibility_class;
    }
    if provider.execution_locus_class.is_hosted_model_path()
        && provider.quota_state_class.blocks_hosted_dispatch()
    {
        return RouteEligibilityClass::QuotaBlocked;
    }
    if !provider.supports_feature(required_feature) || !model.supports_feature(required_feature) {
        return RouteEligibilityClass::CapabilityBlocked;
    }
    RouteEligibilityClass::Eligible
}

fn route_reason_for(reason: RegistryRouteReasonClass) -> RouteSelectionReasonClass {
    match reason {
        RegistryRouteReasonClass::LocalFirstEligibleRouteAdmitted
        | RegistryRouteReasonClass::PolicyPinnedRouteAdmitted => {
            RouteSelectionReasonClass::PolicyPinnedSpecificRoute
        }
        RegistryRouteReasonClass::CheapestQualifyingRouteAdmitted => {
            RouteSelectionReasonClass::CheapestQualifyingRouteAdmitted
        }
        RegistryRouteReasonClass::ManualSelectionRequired
        | RegistryRouteReasonClass::NoEligibleRoute => {
            RouteSelectionReasonClass::NoRouteAdmittedDisabledWithTypedDenial
        }
    }
}

fn route_override_for(reason: RegistryRouteReasonClass) -> RouteSelectionOverrideReasonClass {
    match reason {
        RegistryRouteReasonClass::LocalFirstEligibleRouteAdmitted
        | RegistryRouteReasonClass::PolicyPinnedRouteAdmitted => {
            RouteSelectionOverrideReasonClass::PolicyPinnedMoreExpensiveRoute
        }
        RegistryRouteReasonClass::CheapestQualifyingRouteAdmitted => {
            RouteSelectionOverrideReasonClass::NoOverrideCheapestWasUsed
        }
        RegistryRouteReasonClass::ManualSelectionRequired
        | RegistryRouteReasonClass::NoEligibleRoute => {
            RouteSelectionOverrideReasonClass::CheapestRouteFailedCapabilityCheck
        }
    }
}

fn exhaustion_state_for(
    eligibility: RouteEligibilityClass,
    provider: &ProviderRegistryEntry,
) -> ExhaustionStateClass {
    match eligibility {
        RouteEligibilityClass::Eligible => ExhaustionStateClass::NotExhaustedRouteAdmitted,
        RouteEligibilityClass::QuotaBlocked => ExhaustionStateClass::QuotaFamilyExhausted,
        RouteEligibilityClass::PolicyDenied => ExhaustionStateClass::EligibilityRevokedPolicy,
        RouteEligibilityClass::WorkspaceTrustBlocked => {
            ExhaustionStateClass::EligibilityRevokedWorkspaceTrust
        }
        RouteEligibilityClass::LifecycleBlocked => {
            if provider.lifecycle_state_class
                == RegistryLifecycleStateClass::QuarantinedPendingReview
            {
                ExhaustionStateClass::EligibilityRevokedPackQuarantined
            } else {
                ExhaustionStateClass::EligibilityRevokedProviderDisabled
            }
        }
        RouteEligibilityClass::CapabilityBlocked | RouteEligibilityClass::ShadowOnly => {
            ExhaustionStateClass::EligibilityRevokedPolicy
        }
    }
}

fn selected_outcome_for(
    candidate: &RegistryRouteCandidate,
    provider: &ProviderRegistryEntry,
) -> SelectedOutcomeClass {
    if candidate.selected {
        return SelectedOutcomeClass::SelectedThisPath;
    }
    match exhaustion_state_for(candidate.route_eligibility_class, provider) {
        ExhaustionStateClass::QuotaFamilyExhausted => {
            SelectedOutcomeClass::NotSelectedQuotaExhausted
        }
        ExhaustionStateClass::EligibilityRevokedProviderDisabled => {
            SelectedOutcomeClass::NotSelectedDeprecatedOrWithdrawn
        }
        ExhaustionStateClass::EligibilityRevokedPackQuarantined => {
            SelectedOutcomeClass::NotSelectedPackUnverified
        }
        ExhaustionStateClass::EligibilityRevokedPolicy
        | ExhaustionStateClass::EligibilityRevokedWorkspaceTrust => {
            SelectedOutcomeClass::NotSelectedPolicyPin
        }
        _ => SelectedOutcomeClass::NotSelectedFailedCapability,
    }
}

fn join_route_choices(choices: &[RegistryRoutingPolicyClass]) -> String {
    if choices.is_empty() {
        "none".to_owned()
    } else {
        choices
            .iter()
            .map(|choice| choice.as_str())
            .collect::<Vec<_>>()
            .join("|")
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
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key=")
        || lower.contains("api-key=")
        || lower.contains("raw_api_key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
}

#[cfg(test)]
mod tests;
