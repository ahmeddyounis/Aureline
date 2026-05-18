//! Typed gateway baseline for MCP-style and external connector tools.
//!
//! This module extends the provider/model registry into a typed contract for
//! every connector or tool the product can invoke from an AI turn or a
//! user-facing command. Each [`ToolGatewayDescriptor`] binds publisher source,
//! runtime boundary, capability classes, network behavior, credential posture,
//! warm/cold availability state, first-use review state, allowed side effects,
//! data-class allowlists, and output trust posture. Each
//! [`ToolCallTimelineEntry`] lands one tool invocation in the shared evidence
//! timeline with stable id, descriptor lineage, runtime-boundary label,
//! side-effect class, outcome class, taint posture, classification truth, and
//! inspect/remove-from-context action refs.
//!
//! The records carry no raw endpoint URLs, raw spawn commands, raw environment
//! variables, raw API keys, raw OAuth tokens, raw mTLS material, raw
//! request/response bodies, or raw stdio frames. Replay, rerun, and history
//! surfaces preserve the taint posture instead of flattening tool output into
//! trusted context.
//!
//! The cross-tool contracts the gateway projects against are the descriptor
//! schema [`TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF`], the timeline-entry schema
//! [`TOOL_CALL_TIMELINE_ENTRY_SCHEMA_REF`], and the conformance report at
//! [`TOOL_GATEWAY_CONFORMANCE_ARTIFACT_REF`].

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::registry::{
    ExternalToolRegistryEntry, ExternalToolSideEffectClass, RegistryApprovalPostureClass,
    RegistryDataClass, RegistryLifecycleStateClass,
};
use crate::routing::RoutingPolicyContext;

/// Stable record-kind tag carried by [`ToolGatewayDescriptor`] rows.
pub const TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND: &str = "tool_gateway_descriptor_record";

/// Stable record-kind tag carried by [`ToolCallTimelineEntry`] rows.
pub const TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND: &str = "tool_call_timeline_entry_record";

/// Stable record-kind tag carried by [`ToolGatewayConformancePacket`] payloads.
pub const TOOL_GATEWAY_CONFORMANCE_PACKET_RECORD_KIND: &str =
    "tool_gateway_conformance_packet_record";

/// Stable record-kind tag carried by [`ToolGatewaySurfaceRow`] rows.
pub const TOOL_GATEWAY_SURFACE_ROW_RECORD_KIND: &str = "tool_gateway_surface_row_record";

/// Schema version of the gateway descriptor, timeline entry, and conformance packet.
pub const TOOL_GATEWAY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the descriptor boundary schema.
pub const TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/ai/tool_gateway_descriptor.schema.json";

/// Repo-relative path of the timeline-entry boundary schema.
pub const TOOL_CALL_TIMELINE_ENTRY_SCHEMA_REF: &str =
    "schemas/ai/tool_call_timeline_entry.schema.json";

/// Repo-relative path of the protected gateway fixture corpus.
pub const TOOL_GATEWAY_FIXTURE_DIR: &str = "fixtures/ai/m3/mcp_gateway_and_tool_history";

/// Repo-relative path of the checked-in gateway conformance report.
pub const TOOL_GATEWAY_CONFORMANCE_ARTIFACT_REF: &str =
    "artifacts/ai/m3/tool_gateway_conformance_report.md";

/// Source / publisher class for one gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolPublisherSourceClass {
    /// Tool shipped with Aureline itself.
    FirstPartyNativePublisher,
    /// Tool provided by a signed extension manifest.
    SignedExtensionPublisher,
    /// Tool exposed by an MCP server following the Model Context Protocol.
    McpServerPublisher,
    /// Tool the user registered directly.
    UserRegisteredPublisher,
    /// Tool an admin or policy bundle registered for a managed fleet.
    EnterpriseRegisteredPublisher,
    /// Tool used only for parity or record-replay fixtures.
    MockedTestPublisher,
}

impl ToolPublisherSourceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyNativePublisher => "first_party_native_publisher",
            Self::SignedExtensionPublisher => "signed_extension_publisher",
            Self::McpServerPublisher => "mcp_server_publisher",
            Self::UserRegisteredPublisher => "user_registered_publisher",
            Self::EnterpriseRegisteredPublisher => "enterprise_registered_publisher",
            Self::MockedTestPublisher => "mocked_test_publisher",
        }
    }

    /// True when this publisher class requires a signed publisher identity ref.
    pub const fn requires_publisher_identity(self) -> bool {
        !matches!(self, Self::MockedTestPublisher)
    }

    /// True when this publisher requires explicit first-use review before any material run.
    pub const fn requires_first_use_review(self) -> bool {
        matches!(
            self,
            Self::McpServerPublisher
                | Self::UserRegisteredPublisher
                | Self::EnterpriseRegisteredPublisher
                | Self::SignedExtensionPublisher
        )
    }
}

/// Runtime boundary class naming where the tool actually executes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolRuntimeBoundaryClass {
    /// Tool runs in the Aureline process.
    LocalInProcess,
    /// Tool runs as a local subprocess on the same device.
    LocalSubprocessSameDevice,
    /// Tool runs inside a local sandboxed container on the same device.
    LocalSandboxedContainerSameDevice,
    /// Tool runs behind a loopback companion service.
    LocalCompanionServiceLoopback,
    /// Tool runs as a remote vendor-managed service.
    RemoteVendorManagedService,
    /// Tool runs as a remote self-hosted service.
    RemoteSelfHostedService,
    /// Tool runs behind an enterprise gateway broker.
    EnterpriseGatewayBrokeredService,
    /// Tool runs in an extension-provided locus.
    ExtensionProvidedLocus,
    /// Tool runs only in test fixtures.
    MockedTestLocus,
}

impl ToolRuntimeBoundaryClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcess => "local_in_process",
            Self::LocalSubprocessSameDevice => "local_subprocess_same_device",
            Self::LocalSandboxedContainerSameDevice => "local_sandboxed_container_same_device",
            Self::LocalCompanionServiceLoopback => "local_companion_service_loopback",
            Self::RemoteVendorManagedService => "remote_vendor_managed_service",
            Self::RemoteSelfHostedService => "remote_self_hosted_service",
            Self::EnterpriseGatewayBrokeredService => "enterprise_gateway_brokered_service",
            Self::ExtensionProvidedLocus => "extension_provided_locus",
            Self::MockedTestLocus => "mocked_test_locus",
        }
    }

    /// True when the boundary is local to the user's device.
    pub const fn is_local(self) -> bool {
        matches!(
            self,
            Self::LocalInProcess
                | Self::LocalSubprocessSameDevice
                | Self::LocalSandboxedContainerSameDevice
                | Self::LocalCompanionServiceLoopback
        )
    }

    /// True when invocations on this boundary cross the network.
    pub const fn crosses_network(self) -> bool {
        matches!(
            self,
            Self::RemoteVendorManagedService
                | Self::RemoteSelfHostedService
                | Self::EnterpriseGatewayBrokeredService
        )
    }
}

/// Network behavior the descriptor advertises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolNetworkBehaviorClass {
    /// Tool performs no network I/O and stays on-device.
    NoNetworkLocalOnly,
    /// Tool only uses local loopback transports.
    LocalLoopbackOnly,
    /// Tool reaches remote services over HTTPS.
    RemoteHttps,
    /// Tool reaches remote services over gRPC/TLS.
    RemoteGrpcOverTls,
    /// Tool reaches a remote MCP server over streamable HTTP.
    RemoteMcpOverStreamableHttp,
    /// Tool runs in an extension and stays on-device.
    ExtensionMediatedLocal,
    /// Tool runs in an extension and may reach remote services.
    ExtensionMediatedRemote,
    /// Network behavior is unknown and MUST be disclosed before use.
    UnknownMustDisclose,
}

impl ToolNetworkBehaviorClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoNetworkLocalOnly => "no_network_local_only",
            Self::LocalLoopbackOnly => "local_loopback_only",
            Self::RemoteHttps => "remote_https",
            Self::RemoteGrpcOverTls => "remote_grpc_over_tls",
            Self::RemoteMcpOverStreamableHttp => "remote_mcp_over_streamable_http",
            Self::ExtensionMediatedLocal => "extension_mediated_local",
            Self::ExtensionMediatedRemote => "extension_mediated_remote",
            Self::UnknownMustDisclose => "unknown_must_disclose",
        }
    }

    /// True when this behavior reaches the network.
    pub const fn is_remote(self) -> bool {
        matches!(
            self,
            Self::RemoteHttps
                | Self::RemoteGrpcOverTls
                | Self::RemoteMcpOverStreamableHttp
                | Self::ExtensionMediatedRemote
        )
    }
}

/// Credential posture for a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCredentialPostureClass {
    /// Tool needs no credential and runs locally.
    NoCredentialLocalOnly,
    /// Tool is admitted by a signed manifest only.
    SignedManifestOnly,
    /// Credential is BYOK and addressed through the secret broker.
    ByokSecretBroker,
    /// Credential is managed by an enterprise gateway.
    EnterpriseGatewayManaged,
    /// Credential is managed by a vendor-hosted first-party path.
    VendorHostedFirstPartyManaged,
    /// Credential is provided by an extension.
    ExtensionProvidedCredential,
    /// Mocked credential used in fixtures only.
    MockedTestCredential,
    /// Row is disabled and carries no credential.
    DisabledNoCredential,
}

impl ToolCredentialPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCredentialLocalOnly => "no_credential_local_only",
            Self::SignedManifestOnly => "signed_manifest_only",
            Self::ByokSecretBroker => "byok_secret_broker",
            Self::EnterpriseGatewayManaged => "enterprise_gateway_managed",
            Self::VendorHostedFirstPartyManaged => "vendor_hosted_first_party_managed",
            Self::ExtensionProvidedCredential => "extension_provided_credential",
            Self::MockedTestCredential => "mocked_test_credential",
            Self::DisabledNoCredential => "disabled_no_credential",
        }
    }
}

/// Output trust posture for a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolOutputTrustPostureClass {
    /// Output from any external tool is always tainted.
    AlwaysTaintedExternalToolOutput,
    /// Output from any remote service is always tainted.
    AlwaysTaintedRemoteServiceOutput,
    /// Output from any extension-provided tool is always tainted.
    AlwaysTaintedExtensionProvidedOutput,
    /// Output is tainted unless reviewed, then fenced.
    TaintedUnlessReviewedThenFenced,
    /// Output from a trusted local first-party tool is trusted after signing.
    TrustedFirstPartyLocalToolOutput,
    /// Output from a signed extension-provided local tool is trusted after signing.
    TrustedSignedExtensionLocalToolOutput,
    /// Posture is unknown and MUST fence by default.
    PostureUnknownRequiresFenceByDefault,
}

impl ToolOutputTrustPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysTaintedExternalToolOutput => "always_tainted_external_tool_output",
            Self::AlwaysTaintedRemoteServiceOutput => "always_tainted_remote_service_output",
            Self::AlwaysTaintedExtensionProvidedOutput => {
                "always_tainted_extension_provided_output"
            }
            Self::TaintedUnlessReviewedThenFenced => "tainted_unless_reviewed_then_fenced",
            Self::TrustedFirstPartyLocalToolOutput => "trusted_first_party_local_tool_output",
            Self::TrustedSignedExtensionLocalToolOutput => {
                "trusted_signed_extension_local_tool_output"
            }
            Self::PostureUnknownRequiresFenceByDefault => {
                "posture_unknown_requires_fence_by_default"
            }
        }
    }

    /// True when output bytes are tainted before classification.
    pub const fn is_tainted_by_default(self) -> bool {
        !matches!(
            self,
            Self::TrustedFirstPartyLocalToolOutput | Self::TrustedSignedExtensionLocalToolOutput
        )
    }
}

/// Operational availability state of a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolAvailabilityStateClass {
    /// Tool is warm and ready to dispatch.
    WarmAdmittedReady,
    /// Tool is admitted but needs a handshake before first use.
    ColdAdmitPendingHandshake,
    /// Tool is temporarily unavailable due to transport problems.
    UnavailableTemporaryTransport,
    /// Tool is quarantined because its signature could not be verified.
    UnavailableQuarantinedSignature,
    /// Tool is blocked by policy.
    PolicyBlocked,
    /// Tool is blocked by workspace trust.
    TrustBlocked,
    /// Tool has been withdrawn from this gateway state.
    Withdrawn,
}

impl ToolAvailabilityStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarmAdmittedReady => "warm_admitted_ready",
            Self::ColdAdmitPendingHandshake => "cold_admit_pending_handshake",
            Self::UnavailableTemporaryTransport => "unavailable_temporary_transport",
            Self::UnavailableQuarantinedSignature => "unavailable_quarantined_signature",
            Self::PolicyBlocked => "policy_blocked",
            Self::TrustBlocked => "trust_blocked",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the gateway can admit a new invocation on this descriptor.
    pub const fn admits_new_invocation(self) -> bool {
        matches!(
            self,
            Self::WarmAdmittedReady | Self::ColdAdmitPendingHandshake
        )
    }

    /// True when downstream surfaces MUST display a typed denial reason.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::UnavailableTemporaryTransport
                | Self::UnavailableQuarantinedSignature
                | Self::PolicyBlocked
                | Self::TrustBlocked
                | Self::Withdrawn
        )
    }
}

/// Approval posture for a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolApprovalPostureClass {
    /// Dispatch may proceed without a prompt.
    AllowedWithoutPrompt,
    /// Dispatch requires one prompt the first time the tool is used.
    AllowedWithOneTimePrompt,
    /// Dispatch requires one prompt per session.
    AllowedWithPerSessionPrompt,
    /// Dispatch requires a prompt for every invocation.
    AllowedWithPerInvocationPrompt,
    /// Dispatch requires an admin approval ticket.
    RequiresAdminApproval,
    /// Dispatch is denied by policy.
    DeniedByPolicy,
    /// Dispatch is denied by workspace trust.
    DeniedByWorkspaceTrust,
    /// Dispatch is denied because the tool is not admitted.
    DeniedToolNotAdmitted,
}

impl ToolApprovalPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowedWithoutPrompt => "allowed_without_prompt",
            Self::AllowedWithOneTimePrompt => "allowed_with_one_time_prompt",
            Self::AllowedWithPerSessionPrompt => "allowed_with_per_session_prompt",
            Self::AllowedWithPerInvocationPrompt => "allowed_with_per_invocation_prompt",
            Self::RequiresAdminApproval => "requires_admin_approval",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::DeniedByWorkspaceTrust => "denied_by_workspace_trust",
            Self::DeniedToolNotAdmitted => "denied_tool_not_admitted",
        }
    }

    /// True when dispatch requires a fresh approval gate.
    pub const fn requires_approval_gate(self) -> bool {
        matches!(
            self,
            Self::AllowedWithOneTimePrompt
                | Self::AllowedWithPerSessionPrompt
                | Self::AllowedWithPerInvocationPrompt
                | Self::RequiresAdminApproval
        )
    }

    /// True when the descriptor explicitly denies dispatch.
    pub const fn denies_dispatch(self) -> bool {
        matches!(
            self,
            Self::DeniedByPolicy | Self::DeniedByWorkspaceTrust | Self::DeniedToolNotAdmitted
        )
    }
}

/// First-use review state for a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUseReviewStateClass {
    /// Native first-party tool that does not require first-use review.
    NeverRequiredNativeTool,
    /// Tool is awaiting first-use review.
    PendingFirstUseReview,
    /// Tool is approved for use.
    ApprovedForUse,
    /// Tool was rejected and is blocked.
    RejectedBlocked,
    /// Prior first-use approval expired and must be renewed.
    ExpiredRequiresRenewal,
}

impl FirstUseReviewStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverRequiredNativeTool => "never_required_native_tool",
            Self::PendingFirstUseReview => "pending_first_use_review",
            Self::ApprovedForUse => "approved_for_use",
            Self::RejectedBlocked => "rejected_blocked",
            Self::ExpiredRequiresRenewal => "expired_requires_renewal",
        }
    }

    /// True when this state permits a material run.
    pub const fn admits_material_run(self) -> bool {
        matches!(self, Self::NeverRequiredNativeTool | Self::ApprovedForUse)
    }
}

/// Capability class declared by a gateway descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCapabilityClass {
    /// Inspect-only workspace file slices.
    InspectWorkspaceFiles,
    /// Inspect-only workspace symbol summaries.
    InspectWorkspaceSymbols,
    /// Inspect-only workspace diagnostics.
    InspectWorkspaceDiagnostics,
    /// Inspect-only workspace search.
    InspectWorkspaceSearch,
    /// Inspect-only workspace graph summaries.
    InspectWorkspaceGraph,
    /// Reversible edits inside the workspace.
    EditWorkspaceFilesReversible,
    /// Destructive edits inside the workspace.
    EditWorkspaceFilesDestructive,
    /// Local subprocess execution.
    ExecuteLocalSubprocess,
    /// Fetch external documents.
    FetchExternalDocuments,
    /// Fetch external data records.
    FetchExternalData,
    /// Publish reversible external comments.
    PublishExternalComment,
    /// Publish irreversible external artifacts.
    PublishExternalArtifact,
    /// Mutate policy or trust state.
    MutatePolicyOrTrust,
    /// Manage an external resource.
    ManageExternalResource,
    /// Synthesize text.
    SynthesizeText,
    /// Synthesize code.
    SynthesizeCode,
    /// Summarize evidence packets.
    SummarizeEvidence,
    /// Answer with citations.
    AnswerWithCitations,
    /// Capability is unknown and MUST be disclosed.
    UnknownMustDisclose,
}

impl ToolCapabilityClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectWorkspaceFiles => "inspect_workspace_files",
            Self::InspectWorkspaceSymbols => "inspect_workspace_symbols",
            Self::InspectWorkspaceDiagnostics => "inspect_workspace_diagnostics",
            Self::InspectWorkspaceSearch => "inspect_workspace_search",
            Self::InspectWorkspaceGraph => "inspect_workspace_graph",
            Self::EditWorkspaceFilesReversible => "edit_workspace_files_reversible",
            Self::EditWorkspaceFilesDestructive => "edit_workspace_files_destructive",
            Self::ExecuteLocalSubprocess => "execute_local_subprocess",
            Self::FetchExternalDocuments => "fetch_external_documents",
            Self::FetchExternalData => "fetch_external_data",
            Self::PublishExternalComment => "publish_external_comment",
            Self::PublishExternalArtifact => "publish_external_artifact",
            Self::MutatePolicyOrTrust => "mutate_policy_or_trust",
            Self::ManageExternalResource => "manage_external_resource",
            Self::SynthesizeText => "synthesize_text",
            Self::SynthesizeCode => "synthesize_code",
            Self::SummarizeEvidence => "summarize_evidence",
            Self::AnswerWithCitations => "answer_with_citations",
            Self::UnknownMustDisclose => "unknown_must_disclose",
        }
    }
}

/// Side-effect class re-exported and mirrored from
/// [`ExternalToolSideEffectClass`] with the broader gateway vocabulary that
/// includes credential and policy mutations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolSideEffectClass {
    /// Inspect-only tool call.
    InspectOnly,
    /// Local reversible edit.
    LocalReversibleEdit,
    /// Local destructive edit.
    LocalDestructiveEdit,
    /// Tool projects a credential handle (denied unless explicitly admitted).
    CredentialHandleProjection,
    /// Privileged inspection or attach.
    PrivilegedInspectionAttach,
    /// External reversible comment or draft.
    ExternalReversibleComment,
    /// External irreversible publish.
    ExternalIrreversiblePublish,
    /// Policy or workspace-trust mutation.
    PolicyOrTrustMutation,
    /// Capability widening request.
    CapabilityWidening,
    /// Admission of an automation recipe only.
    AutomationAdmissionOnly,
}

impl ToolSideEffectClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOnly => "inspect_only",
            Self::LocalReversibleEdit => "local_reversible_edit",
            Self::LocalDestructiveEdit => "local_destructive_edit",
            Self::CredentialHandleProjection => "credential_handle_projection",
            Self::PrivilegedInspectionAttach => "privileged_inspection_attach",
            Self::ExternalReversibleComment => "external_reversible_comment",
            Self::ExternalIrreversiblePublish => "external_irreversible_publish",
            Self::PolicyOrTrustMutation => "policy_or_trust_mutation",
            Self::CapabilityWidening => "capability_widening",
            Self::AutomationAdmissionOnly => "automation_admission_only",
        }
    }

    /// Maps the registry-shaped side-effect class to the gateway vocabulary.
    pub const fn from_registry(class: ExternalToolSideEffectClass) -> Self {
        match class {
            ExternalToolSideEffectClass::InspectOnly => Self::InspectOnly,
            ExternalToolSideEffectClass::LocalReversibleEdit => Self::LocalReversibleEdit,
            ExternalToolSideEffectClass::LocalDestructiveEdit => Self::LocalDestructiveEdit,
            ExternalToolSideEffectClass::ExternalReversibleComment => {
                Self::ExternalReversibleComment
            }
            ExternalToolSideEffectClass::ExternalIrreversiblePublish => {
                Self::ExternalIrreversiblePublish
            }
            ExternalToolSideEffectClass::PolicyOrTrustMutation => Self::PolicyOrTrustMutation,
        }
    }

    /// True when this side effect requires an approval gate above auto-allow.
    pub const fn requires_approval_gate(self) -> bool {
        !matches!(self, Self::InspectOnly)
    }
}

/// Outcome class recorded on a [`ToolCallTimelineEntry`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallOutcomeClass {
    /// Inspect-only succeeded.
    SucceededInspectOnly,
    /// Local reversible edit succeeded.
    SucceededWithLocalReversibleEdit,
    /// Local destructive edit succeeded.
    SucceededWithLocalDestructiveEdit,
    /// External reversible comment succeeded.
    SucceededWithExternalReversibleComment,
    /// External irreversible publish succeeded.
    SucceededWithExternalIrreversiblePublish,
    /// Dispatch denied by policy.
    DeniedByPolicy,
    /// Dispatch denied by workspace trust.
    DeniedByWorkspaceTrust,
    /// Dispatch denied because an approval ticket was missing.
    DeniedByApprovalMissing,
    /// Dispatch denied because a data class was not allowed.
    DeniedByDataClassNotAllowed,
    /// Dispatch denied because a side-effect class was not allowed.
    DeniedBySideEffectNotAllowed,
    /// Dispatch denied because tainted-context fences would be violated.
    DeniedByTaintViolation,
    /// Transport error.
    ErrorTransport,
    /// Timeout error.
    ErrorTimeout,
    /// Taint violation discovered after the call.
    ErrorTaintViolation,
    /// User cancelled the invocation.
    CancelledByUser,
    /// Support replay reconstructed the call with no real side effect.
    SupportReplayNoSideEffect,
}

impl ToolCallOutcomeClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SucceededInspectOnly => "succeeded_inspect_only",
            Self::SucceededWithLocalReversibleEdit => "succeeded_with_local_reversible_edit",
            Self::SucceededWithLocalDestructiveEdit => "succeeded_with_local_destructive_edit",
            Self::SucceededWithExternalReversibleComment => {
                "succeeded_with_external_reversible_comment"
            }
            Self::SucceededWithExternalIrreversiblePublish => {
                "succeeded_with_external_irreversible_publish"
            }
            Self::DeniedByPolicy => "denied_by_policy",
            Self::DeniedByWorkspaceTrust => "denied_by_workspace_trust",
            Self::DeniedByApprovalMissing => "denied_by_approval_missing",
            Self::DeniedByDataClassNotAllowed => "denied_by_data_class_not_allowed",
            Self::DeniedBySideEffectNotAllowed => "denied_by_side_effect_not_allowed",
            Self::DeniedByTaintViolation => "denied_by_taint_violation",
            Self::ErrorTransport => "error_transport",
            Self::ErrorTimeout => "error_timeout",
            Self::ErrorTaintViolation => "error_taint_violation",
            Self::CancelledByUser => "cancelled_by_user",
            Self::SupportReplayNoSideEffect => "support_replay_no_side_effect",
        }
    }

    /// True when the outcome admits no observable side effect.
    pub const fn is_no_side_effect(self) -> bool {
        matches!(
            self,
            Self::SucceededInspectOnly
                | Self::DeniedByPolicy
                | Self::DeniedByWorkspaceTrust
                | Self::DeniedByApprovalMissing
                | Self::DeniedByDataClassNotAllowed
                | Self::DeniedBySideEffectNotAllowed
                | Self::DeniedByTaintViolation
                | Self::ErrorTransport
                | Self::ErrorTimeout
                | Self::ErrorTaintViolation
                | Self::CancelledByUser
                | Self::SupportReplayNoSideEffect
        )
    }
}

/// Taint posture preserved by the evidence timeline for one tool-call entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallTaintPostureClass {
    /// External tool output that is tainted by default.
    TaintedExternalToolOutputDefault,
    /// Output whose provenance has not been classified.
    TaintedUnclassifiedProvenance,
    /// Output whose confidence has not been classified.
    TaintedUnclassifiedConfidence,
    /// Output whose effect class has not been classified.
    TaintedUnknownEffectClass,
    /// Output whose retrieval state is partial or stale.
    TaintedPartialOrStaleRetrieval,
    /// Output from a trusted local first-party signed tool.
    TrustedFirstPartyLocalSigned,
}

impl ToolCallTaintPostureClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaintedExternalToolOutputDefault => "tainted_external_tool_output_default",
            Self::TaintedUnclassifiedProvenance => "tainted_unclassified_provenance",
            Self::TaintedUnclassifiedConfidence => "tainted_unclassified_confidence",
            Self::TaintedUnknownEffectClass => "tainted_unknown_effect_class",
            Self::TaintedPartialOrStaleRetrieval => "tainted_partial_or_stale_retrieval",
            Self::TrustedFirstPartyLocalSigned => "trusted_first_party_local_signed",
        }
    }

    /// True when the entry must carry a tainted-context fence ref.
    pub const fn requires_fence(self) -> bool {
        !matches!(self, Self::TrustedFirstPartyLocalSigned)
    }
}

/// Classification state for provenance, confidence, or effect class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolCallClassificationStateClass {
    /// Classification has not been performed yet.
    UnclassifiedPendingReview,
    /// Classification is evidence-backed.
    ClassifiedEvidenceBacked,
    /// Classification is inferred from evidence.
    ClassifiedInferred,
    /// Classification is low-confidence.
    ClassifiedLowConfidence,
    /// Classification could not resolve and MUST remain tainted.
    ClassifiedUnknownMustTreatAsTainted,
}

impl ToolCallClassificationStateClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnclassifiedPendingReview => "unclassified_pending_review",
            Self::ClassifiedEvidenceBacked => "classified_evidence_backed",
            Self::ClassifiedInferred => "classified_inferred",
            Self::ClassifiedLowConfidence => "classified_low_confidence",
            Self::ClassifiedUnknownMustTreatAsTainted => "classified_unknown_must_treat_as_tainted",
        }
    }

    /// True when the classification has fully cleared.
    pub const fn is_evidence_backed(self) -> bool {
        matches!(self, Self::ClassifiedEvidenceBacked)
    }
}

/// Lifecycle state mirrored from the registry vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolGatewayLifecycleStateClass {
    /// Preview-admitted descriptor.
    PreviewAdmitted,
    /// Generally admitted descriptor.
    GenerallyAdmitted,
    /// Soft-deprecated descriptor.
    SoftDeprecatedStillAdmitted,
    /// Hard-deprecated descriptor.
    HardDeprecatedDeniesNewSessions,
    /// Withdrawn descriptor.
    Withdrawn,
    /// Quarantined pending review.
    QuarantinedPendingReview,
    /// Mocked test descriptor only.
    MockedTestOnly,
}

impl ToolGatewayLifecycleStateClass {
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

    /// Maps the registry-shaped lifecycle class to the gateway vocabulary.
    pub const fn from_registry(class: RegistryLifecycleStateClass) -> Self {
        match class {
            RegistryLifecycleStateClass::PreviewAdmitted => Self::PreviewAdmitted,
            RegistryLifecycleStateClass::GenerallyAdmitted => Self::GenerallyAdmitted,
            RegistryLifecycleStateClass::SoftDeprecatedStillAdmitted => {
                Self::SoftDeprecatedStillAdmitted
            }
            RegistryLifecycleStateClass::HardDeprecatedDeniesNewSessions => {
                Self::HardDeprecatedDeniesNewSessions
            }
            RegistryLifecycleStateClass::Withdrawn => Self::Withdrawn,
            RegistryLifecycleStateClass::QuarantinedPendingReview => Self::QuarantinedPendingReview,
            RegistryLifecycleStateClass::MockedTestOnly => Self::MockedTestOnly,
        }
    }

    /// True when this state admits a new invocation.
    pub const fn admits_new_invocation(self) -> bool {
        matches!(
            self,
            Self::PreviewAdmitted
                | Self::GenerallyAdmitted
                | Self::SoftDeprecatedStillAdmitted
                | Self::MockedTestOnly
        )
    }
}

/// Surface that must read the same gateway truth as UI, docs/help, support, or CLI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolGatewaySurfaceClass {
    /// AI composer or pre-send sheet.
    Composer,
    /// AI context inspector.
    ContextInspector,
    /// Review workspace or evidence panel.
    ReviewWorkspace,
    /// Documentation or help projection.
    DocsHelp,
    /// Support export or issue-report projection.
    SupportExport,
    /// CLI or headless audit projection.
    Cli,
}

impl ToolGatewaySurfaceClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Composer => "composer",
            Self::ContextInspector => "context_inspector",
            Self::ReviewWorkspace => "review_workspace",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::Cli => "cli",
        }
    }
}

/// Required surfaces that must preserve identical descriptor and timeline refs.
const REQUIRED_SURFACE_CLASSES: &[ToolGatewaySurfaceClass] = &[
    ToolGatewaySurfaceClass::Composer,
    ToolGatewaySurfaceClass::ContextInspector,
    ToolGatewaySurfaceClass::ReviewWorkspace,
    ToolGatewaySurfaceClass::DocsHelp,
    ToolGatewaySurfaceClass::SupportExport,
];

/// Required denied-data classes that the descriptor MUST keep on its deny set.
const REQUIRED_DENIED_DATA_CLASSES: &[RegistryDataClass] = &[
    RegistryDataClass::CredentialHandleDeniedAlways,
    RegistryDataClass::SecretProjectionDeniedAlways,
];

/// One descriptor row on the typed tool gateway.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGatewayDescriptor {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Opaque ref to the matching external-tool registry row, when one exists.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub external_tool_entry_ref: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Tool family label.
    pub tool_family_label: String,
    /// Tool capability version.
    pub tool_capability_version: String,
    /// Source/publisher class.
    pub publisher_source_class: ToolPublisherSourceClass,
    /// Opaque ref to the signed publisher identity record.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub publisher_identity_ref: String,
    /// Runtime boundary class.
    pub runtime_boundary_class: ToolRuntimeBoundaryClass,
    /// Network behavior class.
    pub network_behavior_class: ToolNetworkBehaviorClass,
    /// Credential posture class.
    pub credential_posture_class: ToolCredentialPostureClass,
    /// Output trust posture class.
    pub output_trust_posture_class: ToolOutputTrustPostureClass,
    /// Operational availability state class.
    pub availability_state_class: ToolAvailabilityStateClass,
    /// Lifecycle state class.
    pub lifecycle_state_class: ToolGatewayLifecycleStateClass,
    /// Approval posture class.
    pub approval_posture_class: ToolApprovalPostureClass,
    /// First-use review state class.
    pub first_use_review_state_class: FirstUseReviewStateClass,
    /// Approval ticket ref that admitted first use.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub first_use_review_ticket_ref: String,
    /// Capability classes the tool advertises.
    pub capability_classes: Vec<ToolCapabilityClass>,
    /// Side-effect classes the tool may produce.
    pub allowed_side_effect_classes: Vec<ToolSideEffectClass>,
    /// Data classes the descriptor admits as tool input.
    pub allowed_data_classes: Vec<RegistryDataClass>,
    /// Data classes the descriptor unconditionally denies.
    pub denied_data_classes: Vec<RegistryDataClass>,
    /// Descriptors this one supersedes.
    #[serde(default)]
    pub supersedes_descriptor_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Export-safe explanation label.
    pub explanation_label: String,
    /// Timestamp the descriptor was minted.
    pub minted_at: String,
    /// Timestamp the descriptor was last refreshed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
}

impl ToolGatewayDescriptor {
    /// Drafts a descriptor from a registry [`ExternalToolRegistryEntry`].
    ///
    /// The returned descriptor inherits identity, capability version, lifecycle,
    /// approval posture, and data-class refs from the registry row. Callers
    /// supply the gateway-specific publisher source, runtime boundary, network
    /// behavior, credential posture, output trust posture, availability state,
    /// first-use review state, capability classes, and policy context.
    #[allow(clippy::too_many_arguments)]
    pub fn from_registry_entry(
        descriptor_id: impl Into<String>,
        entry: &ExternalToolRegistryEntry,
        publisher_source_class: ToolPublisherSourceClass,
        publisher_identity_ref: impl Into<String>,
        runtime_boundary_class: ToolRuntimeBoundaryClass,
        network_behavior_class: ToolNetworkBehaviorClass,
        credential_posture_class: ToolCredentialPostureClass,
        output_trust_posture_class: ToolOutputTrustPostureClass,
        availability_state_class: ToolAvailabilityStateClass,
        first_use_review_state_class: FirstUseReviewStateClass,
        capability_classes: Vec<ToolCapabilityClass>,
        policy_context: RoutingPolicyContext,
        minted_at: impl Into<String>,
    ) -> Self {
        let approval_posture_class = map_registry_approval_posture(entry.approval_posture_class);
        let first_use_review_ticket_ref = entry
            .required_approval_ticket_ref
            .clone()
            .unwrap_or_default();
        let mut denied_data_classes = entry.denied_data_classes.clone();
        for required in REQUIRED_DENIED_DATA_CLASSES {
            if !denied_data_classes.contains(required) {
                denied_data_classes.push(*required);
            }
        }
        Self {
            record_kind: TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
            descriptor_id: descriptor_id.into(),
            external_tool_entry_ref: entry.tool_entry_id.clone(),
            display_label: entry.display_label.clone(),
            tool_family_label: entry.tool_family_label.clone(),
            tool_capability_version: entry.tool_capability_version.clone(),
            publisher_source_class,
            publisher_identity_ref: publisher_identity_ref.into(),
            runtime_boundary_class,
            network_behavior_class,
            credential_posture_class,
            output_trust_posture_class,
            availability_state_class,
            lifecycle_state_class: ToolGatewayLifecycleStateClass::from_registry(
                entry.lifecycle_state_class,
            ),
            approval_posture_class,
            first_use_review_state_class,
            first_use_review_ticket_ref,
            capability_classes,
            allowed_side_effect_classes: entry
                .allowed_side_effect_classes
                .iter()
                .copied()
                .map(ToolSideEffectClass::from_registry)
                .collect(),
            allowed_data_classes: entry.allowed_data_classes.clone(),
            denied_data_classes,
            supersedes_descriptor_refs: Vec::new(),
            policy_context,
            explanation_label: entry.explanation_label.clone(),
            minted_at: minted_at.into(),
            last_refreshed_at: None,
        }
    }

    /// True when this descriptor admits a material run inside the current policy.
    pub fn admits_material_run(&self) -> bool {
        self.lifecycle_state_class.admits_new_invocation()
            && self.availability_state_class.admits_new_invocation()
            && !self.approval_posture_class.denies_dispatch()
            && self.first_use_review_state_class.admits_material_run()
    }

    /// True when the descriptor's runtime boundary is local to the user's device.
    pub fn is_local_boundary(&self) -> bool {
        self.runtime_boundary_class.is_local()
    }

    /// True when the descriptor's output bytes must be fenced by default.
    pub fn outputs_are_tainted_by_default(&self) -> bool {
        self.output_trust_posture_class.is_tainted_by_default()
    }

    /// True when the descriptor advertises a mutating side effect.
    pub fn has_mutating_side_effect(&self) -> bool {
        self.allowed_side_effect_classes
            .iter()
            .any(|side_effect| side_effect.requires_approval_gate())
    }

    /// True when the descriptor requires a first-use approval ticket.
    pub fn requires_first_use_review_ticket(&self) -> bool {
        self.publisher_source_class.requires_first_use_review()
            && self.first_use_review_state_class
                != FirstUseReviewStateClass::NeverRequiredNativeTool
    }
}

/// One tool-call entry attached to the shared evidence timeline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCallTimelineEntry {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable tool-call id.
    pub tool_call_id: String,
    /// Descriptor ref the call resolved to.
    pub descriptor_ref: String,
    /// Runtime boundary class observed at dispatch.
    pub runtime_boundary_class: ToolRuntimeBoundaryClass,
    /// Reviewer-visible boundary label.
    pub boundary_label: String,
    /// Side effect predicted at dispatch.
    pub predicted_side_effect_class: ToolSideEffectClass,
    /// Side effect observed after the call returned.
    pub observed_side_effect_class: ToolSideEffectClass,
    /// Outcome class recorded after the call returned.
    pub outcome_class: ToolCallOutcomeClass,
    /// Reviewer-visible outcome summary label.
    pub outcome_summary_label: String,
    /// Taint posture preserved by replay/rerun/history.
    pub taint_posture_class: ToolCallTaintPostureClass,
    /// Provenance classification state.
    pub provenance_classification_state_class: ToolCallClassificationStateClass,
    /// Confidence classification state.
    pub confidence_classification_state_class: ToolCallClassificationStateClass,
    /// Effect-class classification state.
    pub effect_classification_state_class: ToolCallClassificationStateClass,
    /// Data classes sent to the tool.
    pub data_classes_to_be_sent: Vec<RegistryDataClass>,
    /// Data classes returned by the tool.
    #[serde(default)]
    pub data_classes_returned: Vec<RegistryDataClass>,
    /// Opaque ref to the inspect-output action surface.
    pub inspect_action_ref: String,
    /// Opaque ref to the remove-from-context action surface.
    pub remove_from_context_action_ref: String,
    /// Opaque ref to the replay-in-sandbox action surface.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub replay_in_sandbox_action_ref: String,
    /// Opaque ref to the renew-approval action surface.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub renew_approval_action_ref: String,
    /// Approval ticket that admitted the invocation.
    pub originating_approval_ticket_ref: String,
    /// Disclosure ref minted before the invocation.
    pub originating_disclosure_ref: String,
    /// Tainted-context fence ref attached to returned bytes.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tainted_context_fence_ref: String,
    /// Evidence-timeline ref the entry lives under.
    pub evidence_timeline_ref: String,
    /// Rerun-review ref, when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rerun_review_ref: String,
    /// Rollback or history ref, when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub rollback_history_ref: String,
    /// Support export ref, when applicable.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub support_export_ref: String,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Timestamp the entry was emitted.
    pub emitted_at: String,
}

impl ToolCallTimelineEntry {
    /// True when the entry's taint posture requires a fence ref.
    pub fn must_carry_fence(&self) -> bool {
        self.taint_posture_class.requires_fence()
    }

    /// True when all three classification states are evidence-backed.
    pub fn classifications_evidence_backed(&self) -> bool {
        self.provenance_classification_state_class
            .is_evidence_backed()
            && self
                .confidence_classification_state_class
                .is_evidence_backed()
            && self.effect_classification_state_class.is_evidence_backed()
    }
}

/// One surface's proof that it reads the same gateway descriptors and timeline entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGatewaySurfaceRow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Surface class.
    pub surface_class: ToolGatewaySurfaceClass,
    /// Stable projection ref for this surface.
    pub projection_ref: String,
    /// Descriptor refs the surface renders.
    pub descriptor_refs: Vec<String>,
    /// Timeline-entry refs the surface renders.
    pub timeline_entry_refs: Vec<String>,
    /// True when the surface preserves identity, boundary, capability, and approval lineage.
    pub preserves_connector_identity: bool,
    /// True when the surface preserves the same taint posture on every entry.
    pub preserves_taint_posture: bool,
    /// True when the surface excludes raw private material.
    pub raw_private_material_excluded: bool,
    /// True when the surface supports a deterministic JSON export.
    pub supports_json_export: bool,
}

impl ToolGatewaySurfaceRow {
    /// Builds a surface row.
    pub fn new(
        surface_class: ToolGatewaySurfaceClass,
        projection_ref: impl Into<String>,
        descriptor_refs: Vec<String>,
        timeline_entry_refs: Vec<String>,
    ) -> Self {
        Self {
            record_kind: TOOL_GATEWAY_SURFACE_ROW_RECORD_KIND.to_owned(),
            schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
            surface_class,
            projection_ref: projection_ref.into(),
            descriptor_refs,
            timeline_entry_refs,
            preserves_connector_identity: true,
            preserves_taint_posture: true,
            raw_private_material_excluded: true,
            supports_json_export: true,
        }
    }
}

/// Inputs accepted by [`ToolGatewayConformancePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolGatewayConformancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Descriptors covered by the packet.
    pub descriptors: Vec<ToolGatewayDescriptor>,
    /// Tool-call timeline entries covered by the packet.
    pub timeline_entries: Vec<ToolCallTimelineEntry>,
    /// Surface projections covered by the packet.
    pub surface_rows: Vec<ToolGatewaySurfaceRow>,
    /// Source contracts referenced by the packet.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Mint timestamp.
    pub minted_at: String,
}

/// Packet that aggregates gateway descriptors and timeline entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolGatewayConformancePacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Display label safe for UI/docs/support.
    pub display_label: String,
    /// Descriptors aggregated by the packet.
    pub descriptors: Vec<ToolGatewayDescriptor>,
    /// Tool-call timeline entries aggregated by the packet.
    pub timeline_entries: Vec<ToolCallTimelineEntry>,
    /// Surface projections aggregated by the packet.
    pub surface_rows: Vec<ToolGatewaySurfaceRow>,
    /// Source contracts referenced by the packet.
    pub source_contract_refs: Vec<String>,
    /// Policy context.
    pub policy_context: RoutingPolicyContext,
    /// Mint timestamp.
    pub minted_at: String,
}

impl ToolGatewayConformancePacket {
    /// Builds a conformance packet from canonical inputs.
    pub fn new(input: ToolGatewayConformancePacketInput) -> Self {
        Self {
            record_kind: TOOL_GATEWAY_CONFORMANCE_PACKET_RECORD_KIND.to_owned(),
            schema_version: TOOL_GATEWAY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            display_label: input.display_label,
            descriptors: input.descriptors,
            timeline_entries: input.timeline_entries,
            surface_rows: input.surface_rows,
            source_contract_refs: input.source_contract_refs,
            policy_context: input.policy_context,
            minted_at: input.minted_at,
        }
    }

    /// Validates conformance invariants without resolving raw payload bytes.
    pub fn validate(&self) -> Vec<ToolGatewayViolation> {
        let mut violations: Vec<ToolGatewayViolation> = Vec::new();

        if self.record_kind != TOOL_GATEWAY_CONFORMANCE_PACKET_RECORD_KIND {
            violations.push(ToolGatewayViolation::WrongRecordKind);
        }
        if self.schema_version != TOOL_GATEWAY_SCHEMA_VERSION {
            violations.push(ToolGatewayViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.policy_context.policy_epoch_ref.trim().is_empty()
        {
            violations.push(ToolGatewayViolation::MissingIdentity);
        }

        for required in [
            TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF,
            TOOL_CALL_TIMELINE_ENTRY_SCHEMA_REF,
            TOOL_GATEWAY_CONFORMANCE_ARTIFACT_REF,
        ] {
            if !self
                .source_contract_refs
                .iter()
                .any(|reference| reference == required)
            {
                violations.push(ToolGatewayViolation::MissingSourceContracts);
                break;
            }
        }

        if self.descriptors.is_empty() {
            violations.push(ToolGatewayViolation::MissingDescriptors);
        }
        if self.timeline_entries.is_empty() {
            violations.push(ToolGatewayViolation::MissingTimelineEntries);
        }

        validate_descriptors(self, &mut violations);
        validate_timeline_entries(self, &mut violations);
        validate_surface_rows(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("tool gateway packet serializes"),
        ) {
            violations.push(ToolGatewayViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("tool gateway packet serializes")
    }

    /// Deterministic Markdown summary surfaced as the gateway conformance report.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Tool-Gateway Conformance Report\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Descriptors: {}\n", self.descriptors.len()));
        out.push_str(&format!(
            "- Timeline entries: {}\n",
            self.timeline_entries.len()
        ));
        out.push_str(&format!(
            "- Surface projections: {}\n",
            self.surface_rows.len()
        ));
        out.push_str("\n## Descriptor boundary coverage\n");
        let mut boundaries: BTreeSet<&str> = BTreeSet::new();
        for descriptor in &self.descriptors {
            boundaries.insert(descriptor.runtime_boundary_class.as_str());
        }
        for boundary in &boundaries {
            out.push_str(&format!("- `{boundary}`\n"));
        }
        out.push_str("\n## Timeline taint posture coverage\n");
        let mut postures: BTreeSet<&str> = BTreeSet::new();
        for entry in &self.timeline_entries {
            postures.insert(entry.taint_posture_class.as_str());
        }
        for posture in &postures {
            out.push_str(&format!("- `{posture}`\n"));
        }
        out
    }

    /// Returns descriptor ids present on the packet.
    pub fn descriptor_ids(&self) -> Vec<&str> {
        self.descriptors
            .iter()
            .map(|descriptor| descriptor.descriptor_id.as_str())
            .collect()
    }

    /// Returns timeline-entry ids present on the packet.
    pub fn timeline_entry_ids(&self) -> Vec<&str> {
        self.timeline_entries
            .iter()
            .map(|entry| entry.tool_call_id.as_str())
            .collect()
    }
}

/// Validation failures emitted by [`ToolGatewayConformancePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ToolGatewayViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity fields are missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Descriptors are missing or malformed.
    MissingDescriptors,
    /// Timeline entries are missing or malformed.
    MissingTimelineEntries,
    /// Descriptor identity fields are missing.
    DescriptorMissingIdentity,
    /// Descriptor requires a publisher identity ref but none is set.
    DescriptorMissingPublisherIdentity,
    /// Trusted local descriptor does not have a local runtime boundary.
    TrustedDescriptorRequiresLocalBoundary,
    /// Trusted local descriptor does not have a signed publisher identity.
    TrustedDescriptorRequiresSignedPublisher,
    /// Local descriptor advertises a remote network behavior.
    LocalDescriptorAdvertisesRemoteNetwork,
    /// Descriptor requires first-use review but no ticket is set.
    DescriptorMissingFirstUseReviewTicket,
    /// Descriptor has a mutating side effect but no approval gate.
    DescriptorMutatingWithoutApproval,
    /// Descriptor allows a credential or secret class as input.
    DescriptorAllowsForbiddenDataClass,
    /// Descriptor does not deny credential or secret classes.
    DescriptorMissingDeniedDataClasses,
    /// Descriptor admits dispatch with an unknown disclosure posture.
    DescriptorUnknownDisclosureMustBeNarrowed,
    /// Timeline entry references a descriptor that does not exist on the packet.
    TimelineEntryReferencesUnknownDescriptor,
    /// Timeline entry's predicted side effect is not in the descriptor's allowed set.
    TimelineEntryPredictedSideEffectNotAllowed,
    /// Timeline entry's observed side effect is not in the descriptor's allowed set.
    TimelineEntryObservedSideEffectNotAllowed,
    /// Timeline entry uses a data class outside the descriptor's allowed set.
    TimelineEntryUsesForbiddenDataClass,
    /// Tainted timeline entry lacks a fence ref.
    TimelineEntryTaintedWithoutFence,
    /// Trusted timeline entry is missing classification truth.
    TimelineEntryTrustedWithoutClassification,
    /// Trusted timeline entry is not on a local boundary.
    TimelineEntryTrustedNotLocalBoundary,
    /// Timeline entry is missing inspect or remove-from-context action refs.
    TimelineEntryMissingActionRefs,
    /// Timeline entry's outcome is incompatible with its observed side effect.
    TimelineEntryOutcomeMismatchesSideEffect,
    /// Timeline entry is missing identity fields.
    TimelineEntryMissingIdentity,
    /// Required surface projection is missing.
    MissingSurfaceProjection,
    /// Surface projection drifts from the same descriptor or timeline refs.
    SurfaceProjectionDrift,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ToolGatewayViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingDescriptors => "missing_descriptors",
            Self::MissingTimelineEntries => "missing_timeline_entries",
            Self::DescriptorMissingIdentity => "descriptor_missing_identity",
            Self::DescriptorMissingPublisherIdentity => "descriptor_missing_publisher_identity",
            Self::TrustedDescriptorRequiresLocalBoundary => {
                "trusted_descriptor_requires_local_boundary"
            }
            Self::TrustedDescriptorRequiresSignedPublisher => {
                "trusted_descriptor_requires_signed_publisher"
            }
            Self::LocalDescriptorAdvertisesRemoteNetwork => {
                "local_descriptor_advertises_remote_network"
            }
            Self::DescriptorMissingFirstUseReviewTicket => {
                "descriptor_missing_first_use_review_ticket"
            }
            Self::DescriptorMutatingWithoutApproval => "descriptor_mutating_without_approval",
            Self::DescriptorAllowsForbiddenDataClass => "descriptor_allows_forbidden_data_class",
            Self::DescriptorMissingDeniedDataClasses => "descriptor_missing_denied_data_classes",
            Self::DescriptorUnknownDisclosureMustBeNarrowed => {
                "descriptor_unknown_disclosure_must_be_narrowed"
            }
            Self::TimelineEntryReferencesUnknownDescriptor => {
                "timeline_entry_references_unknown_descriptor"
            }
            Self::TimelineEntryPredictedSideEffectNotAllowed => {
                "timeline_entry_predicted_side_effect_not_allowed"
            }
            Self::TimelineEntryObservedSideEffectNotAllowed => {
                "timeline_entry_observed_side_effect_not_allowed"
            }
            Self::TimelineEntryUsesForbiddenDataClass => "timeline_entry_uses_forbidden_data_class",
            Self::TimelineEntryTaintedWithoutFence => "timeline_entry_tainted_without_fence",
            Self::TimelineEntryTrustedWithoutClassification => {
                "timeline_entry_trusted_without_classification"
            }
            Self::TimelineEntryTrustedNotLocalBoundary => {
                "timeline_entry_trusted_not_local_boundary"
            }
            Self::TimelineEntryMissingActionRefs => "timeline_entry_missing_action_refs",
            Self::TimelineEntryOutcomeMismatchesSideEffect => {
                "timeline_entry_outcome_mismatches_side_effect"
            }
            Self::TimelineEntryMissingIdentity => "timeline_entry_missing_identity",
            Self::MissingSurfaceProjection => "missing_surface_projection",
            Self::SurfaceProjectionDrift => "surface_projection_drift",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Errors emitted when reading the checked-in tool-gateway conformance packet.
#[derive(Debug)]
pub enum ToolGatewayArtifactError {
    /// Conformance packet failed to parse.
    Packet(serde_json::Error),
    /// Conformance packet failed validation.
    Validation(Vec<ToolGatewayViolation>),
}

impl fmt::Display for ToolGatewayArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "tool gateway packet parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "tool gateway packet failed validation: {tokens}")
            }
        }
    }
}

impl Error for ToolGatewayArtifactError {}

/// Returns the checked-in tool-gateway conformance packet.
///
/// # Errors
///
/// Returns [`ToolGatewayArtifactError`] when the checked-in fixture cannot be
/// parsed or fails validation.
pub fn current_beta_tool_gateway_conformance_packet(
) -> Result<ToolGatewayConformancePacket, ToolGatewayArtifactError> {
    let packet: ToolGatewayConformancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m3/mcp_gateway_and_tool_history/tool_gateway_conformance_packet.json"
    )))
    .map_err(ToolGatewayArtifactError::Packet)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ToolGatewayArtifactError::Validation(violations))
    }
}

fn map_registry_approval_posture(
    posture: RegistryApprovalPostureClass,
) -> ToolApprovalPostureClass {
    match posture {
        RegistryApprovalPostureClass::AllowedWithoutPrompt => {
            ToolApprovalPostureClass::AllowedWithoutPrompt
        }
        RegistryApprovalPostureClass::AllowedWithPerSessionPrompt => {
            ToolApprovalPostureClass::AllowedWithPerSessionPrompt
        }
        RegistryApprovalPostureClass::AllowedWithPerInvocationPrompt => {
            ToolApprovalPostureClass::AllowedWithPerInvocationPrompt
        }
        RegistryApprovalPostureClass::RequiresAdminApproval => {
            ToolApprovalPostureClass::RequiresAdminApproval
        }
        RegistryApprovalPostureClass::DeniedByPolicy => ToolApprovalPostureClass::DeniedByPolicy,
    }
}

fn validate_descriptors(
    packet: &ToolGatewayConformancePacket,
    violations: &mut Vec<ToolGatewayViolation>,
) {
    let mut seen_descriptor_ids: BTreeSet<&str> = BTreeSet::new();
    for descriptor in &packet.descriptors {
        if descriptor.record_kind != TOOL_GATEWAY_DESCRIPTOR_RECORD_KIND
            || descriptor.schema_version != TOOL_GATEWAY_SCHEMA_VERSION
        {
            violations.push(ToolGatewayViolation::DescriptorMissingIdentity);
            continue;
        }
        if descriptor.descriptor_id.trim().is_empty()
            || descriptor.display_label.trim().is_empty()
            || descriptor.tool_family_label.trim().is_empty()
            || descriptor.tool_capability_version.trim().is_empty()
            || descriptor.explanation_label.trim().is_empty()
            || descriptor.minted_at.trim().is_empty()
            || descriptor.capability_classes.is_empty()
            || descriptor.allowed_side_effect_classes.is_empty()
            || descriptor.allowed_data_classes.is_empty()
        {
            violations.push(ToolGatewayViolation::DescriptorMissingIdentity);
            continue;
        }
        if !seen_descriptor_ids.insert(descriptor.descriptor_id.as_str()) {
            violations.push(ToolGatewayViolation::DescriptorMissingIdentity);
            continue;
        }

        if descriptor
            .publisher_source_class
            .requires_publisher_identity()
            && descriptor.publisher_identity_ref.trim().is_empty()
        {
            violations.push(ToolGatewayViolation::DescriptorMissingPublisherIdentity);
        }

        if matches!(
            descriptor.output_trust_posture_class,
            ToolOutputTrustPostureClass::TrustedFirstPartyLocalToolOutput
                | ToolOutputTrustPostureClass::TrustedSignedExtensionLocalToolOutput
        ) {
            if !descriptor.runtime_boundary_class.is_local() {
                violations.push(ToolGatewayViolation::TrustedDescriptorRequiresLocalBoundary);
            }
            if descriptor.publisher_identity_ref.trim().is_empty() {
                violations.push(ToolGatewayViolation::TrustedDescriptorRequiresSignedPublisher);
            }
        }

        if descriptor.runtime_boundary_class.is_local()
            && descriptor.network_behavior_class.is_remote()
        {
            violations.push(ToolGatewayViolation::LocalDescriptorAdvertisesRemoteNetwork);
        }

        if descriptor.requires_first_use_review_ticket()
            && descriptor.approval_posture_class.requires_approval_gate()
            && descriptor.first_use_review_ticket_ref.trim().is_empty()
        {
            violations.push(ToolGatewayViolation::DescriptorMissingFirstUseReviewTicket);
        }

        if descriptor.has_mutating_side_effect()
            && !descriptor.approval_posture_class.requires_approval_gate()
            && !descriptor.approval_posture_class.denies_dispatch()
        {
            violations.push(ToolGatewayViolation::DescriptorMutatingWithoutApproval);
        }

        if descriptor
            .allowed_data_classes
            .iter()
            .any(|class| matches!(class, RegistryDataClass::CredentialHandleDeniedAlways))
            || descriptor
                .allowed_data_classes
                .iter()
                .any(|class| matches!(class, RegistryDataClass::SecretProjectionDeniedAlways))
        {
            violations.push(ToolGatewayViolation::DescriptorAllowsForbiddenDataClass);
        }

        for required in REQUIRED_DENIED_DATA_CLASSES {
            if !descriptor.denied_data_classes.contains(required) {
                violations.push(ToolGatewayViolation::DescriptorMissingDeniedDataClasses);
                break;
            }
        }

        if matches!(
            descriptor.network_behavior_class,
            ToolNetworkBehaviorClass::UnknownMustDisclose
        ) && descriptor.availability_state_class.admits_new_invocation()
        {
            violations.push(ToolGatewayViolation::DescriptorUnknownDisclosureMustBeNarrowed);
        }
    }
}

fn validate_timeline_entries(
    packet: &ToolGatewayConformancePacket,
    violations: &mut Vec<ToolGatewayViolation>,
) {
    let descriptors_by_id: std::collections::BTreeMap<&str, &ToolGatewayDescriptor> = packet
        .descriptors
        .iter()
        .map(|descriptor| (descriptor.descriptor_id.as_str(), descriptor))
        .collect();

    let mut seen_tool_call_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in &packet.timeline_entries {
        if entry.record_kind != TOOL_CALL_TIMELINE_ENTRY_RECORD_KIND
            || entry.schema_version != TOOL_GATEWAY_SCHEMA_VERSION
        {
            violations.push(ToolGatewayViolation::TimelineEntryMissingIdentity);
            continue;
        }
        if entry.tool_call_id.trim().is_empty()
            || entry.descriptor_ref.trim().is_empty()
            || entry.boundary_label.trim().is_empty()
            || entry.outcome_summary_label.trim().is_empty()
            || entry.originating_approval_ticket_ref.trim().is_empty()
            || entry.originating_disclosure_ref.trim().is_empty()
            || entry.evidence_timeline_ref.trim().is_empty()
            || entry.data_classes_to_be_sent.is_empty()
            || entry.policy_context.policy_epoch_ref.trim().is_empty()
            || entry.emitted_at.trim().is_empty()
        {
            violations.push(ToolGatewayViolation::TimelineEntryMissingIdentity);
            continue;
        }
        if !seen_tool_call_ids.insert(entry.tool_call_id.as_str()) {
            violations.push(ToolGatewayViolation::TimelineEntryMissingIdentity);
            continue;
        }

        let Some(descriptor) = descriptors_by_id.get(entry.descriptor_ref.as_str()) else {
            violations.push(ToolGatewayViolation::TimelineEntryReferencesUnknownDescriptor);
            continue;
        };

        if !descriptor
            .allowed_side_effect_classes
            .contains(&entry.predicted_side_effect_class)
        {
            violations.push(ToolGatewayViolation::TimelineEntryPredictedSideEffectNotAllowed);
        }
        if !descriptor
            .allowed_side_effect_classes
            .contains(&entry.observed_side_effect_class)
        {
            violations.push(ToolGatewayViolation::TimelineEntryObservedSideEffectNotAllowed);
        }
        for class in entry.data_classes_to_be_sent.iter() {
            if !descriptor.allowed_data_classes.contains(class) {
                violations.push(ToolGatewayViolation::TimelineEntryUsesForbiddenDataClass);
                break;
            }
        }
        if entry.must_carry_fence() && entry.tainted_context_fence_ref.trim().is_empty() {
            violations.push(ToolGatewayViolation::TimelineEntryTaintedWithoutFence);
        }
        if matches!(
            entry.taint_posture_class,
            ToolCallTaintPostureClass::TrustedFirstPartyLocalSigned
        ) {
            if !entry.classifications_evidence_backed() {
                violations.push(ToolGatewayViolation::TimelineEntryTrustedWithoutClassification);
            }
            if !entry.runtime_boundary_class.is_local() {
                violations.push(ToolGatewayViolation::TimelineEntryTrustedNotLocalBoundary);
            }
        }
        if entry.inspect_action_ref.trim().is_empty()
            || entry.remove_from_context_action_ref.trim().is_empty()
        {
            violations.push(ToolGatewayViolation::TimelineEntryMissingActionRefs);
        }
        if !outcome_matches_side_effect(entry.outcome_class, entry.observed_side_effect_class) {
            violations.push(ToolGatewayViolation::TimelineEntryOutcomeMismatchesSideEffect);
        }
    }
}

fn validate_surface_rows(
    packet: &ToolGatewayConformancePacket,
    violations: &mut Vec<ToolGatewayViolation>,
) {
    let descriptor_ids: BTreeSet<&str> = packet
        .descriptors
        .iter()
        .map(|descriptor| descriptor.descriptor_id.as_str())
        .collect();
    let entry_ids: BTreeSet<&str> = packet
        .timeline_entries
        .iter()
        .map(|entry| entry.tool_call_id.as_str())
        .collect();

    for required in REQUIRED_SURFACE_CLASSES {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == *required)
        {
            violations.push(ToolGatewayViolation::MissingSurfaceProjection);
            break;
        }
    }

    for row in &packet.surface_rows {
        if row.record_kind != TOOL_GATEWAY_SURFACE_ROW_RECORD_KIND
            || row.schema_version != TOOL_GATEWAY_SCHEMA_VERSION
            || row.projection_ref.trim().is_empty()
            || row.descriptor_refs.is_empty()
            || row.timeline_entry_refs.is_empty()
            || !row.preserves_connector_identity
            || !row.preserves_taint_posture
            || !row.raw_private_material_excluded
            || !row.supports_json_export
        {
            violations.push(ToolGatewayViolation::SurfaceProjectionDrift);
            continue;
        }
        if row
            .descriptor_refs
            .iter()
            .any(|reference| !descriptor_ids.contains(reference.as_str()))
            || row
                .timeline_entry_refs
                .iter()
                .any(|reference| !entry_ids.contains(reference.as_str()))
        {
            violations.push(ToolGatewayViolation::SurfaceProjectionDrift);
        }
    }
}

fn outcome_matches_side_effect(
    outcome: ToolCallOutcomeClass,
    observed: ToolSideEffectClass,
) -> bool {
    if outcome.is_no_side_effect() {
        return matches!(observed, ToolSideEffectClass::InspectOnly)
            || matches!(
                outcome,
                ToolCallOutcomeClass::SucceededInspectOnly
                    | ToolCallOutcomeClass::DeniedByPolicy
                    | ToolCallOutcomeClass::DeniedByWorkspaceTrust
                    | ToolCallOutcomeClass::DeniedByApprovalMissing
                    | ToolCallOutcomeClass::DeniedByDataClassNotAllowed
                    | ToolCallOutcomeClass::DeniedBySideEffectNotAllowed
                    | ToolCallOutcomeClass::DeniedByTaintViolation
                    | ToolCallOutcomeClass::ErrorTransport
                    | ToolCallOutcomeClass::ErrorTimeout
                    | ToolCallOutcomeClass::ErrorTaintViolation
                    | ToolCallOutcomeClass::CancelledByUser
                    | ToolCallOutcomeClass::SupportReplayNoSideEffect
            );
    }
    matches!(
        (outcome, observed),
        (
            ToolCallOutcomeClass::SucceededWithLocalReversibleEdit,
            ToolSideEffectClass::LocalReversibleEdit
        ) | (
            ToolCallOutcomeClass::SucceededWithLocalDestructiveEdit,
            ToolSideEffectClass::LocalDestructiveEdit
        ) | (
            ToolCallOutcomeClass::SucceededWithExternalReversibleComment,
            ToolSideEffectClass::ExternalReversibleComment
        ) | (
            ToolCallOutcomeClass::SucceededWithExternalIrreversiblePublish,
            ToolSideEffectClass::ExternalIrreversiblePublish
        )
    )
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
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
