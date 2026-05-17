//! Tainted-context policy fences and beta support export.
//!
//! This module owns the AI-side packet that joins tainted input labels,
//! policy narrowing decisions, approval fences, and consumer-surface parity
//! into one export-safe record. It complements the lower-level evidence
//! packet fence rows: evidence packets prove a given mutation carried the
//! fence, while this packet proves that suspicious or externally sourced
//! context narrowed execution authority before any run could widen scope.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_content_safety::SuspiciousContentDetection;
use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`TaintedContextBetaPacket`].
pub const TAINTED_CONTEXT_BETA_PACKET_RECORD_KIND: &str = "tainted_context_beta_packet";

/// Schema version for tainted-context beta support packets.
pub const TAINTED_CONTEXT_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the tainted-context beta boundary schema.
pub const TAINTED_CONTEXT_BETA_SCHEMA_REF: &str = "schemas/ai/tainted_context.schema.json";

/// Repo-relative path of the tainted-context beta reviewer contract.
pub const TAINTED_CONTEXT_BETA_AI_DOC_REF: &str = "docs/ai/m3/tainted_context_beta.md";

/// Repo-relative path of the protected tainted-context fixture corpus.
pub const TAINTED_CONTEXT_BETA_FIXTURE_DIR: &str = "fixtures/ai/m3/tainted_context";

/// Repo-relative path of the checked-in tainted-context beta support export.
pub const TAINTED_CONTEXT_BETA_ARTIFACT_REF: &str =
    "artifacts/ai/m3/tainted_context_beta_support_export.json";

const PROMPT_INJECTION_TAINT_CONTRACT_REF: &str = "docs/ai/prompt_injection_and_taint_contract.md";
const CONTEXT_ASSEMBLY_CONTRACT_REF: &str = "docs/ai/context_assembly_contract.md";

const REQUIRED_EFFECTIVE_MODES: &[TaintedContextRunModeClass] = &[
    TaintedContextRunModeClass::ExplainOnly,
    TaintedContextRunModeClass::LocalOnly,
    TaintedContextRunModeClass::PreviewOnly,
    TaintedContextRunModeClass::Blocked,
];

/// Product/support surface that must preserve tainted-context truth refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextSurfaceClass {
    /// Prompt composer or pre-send sheet.
    Composer,
    /// AI context inspector.
    ContextInspector,
    /// Review workspace or mutation preview surface.
    ReviewWorkspace,
    /// Documentation/help projection.
    DocsHelp,
    /// Support export or issue-report projection.
    SupportExport,
    /// CLI or headless audit projection.
    Cli,
}

impl TaintedContextSurfaceClass {
    /// Stable token used in support exports and fixtures.
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

/// Input-source class resolved for a context row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextInputSourceClass {
    /// First-party workspace content.
    RepositoryWorkspaceContent,
    /// Workspace search result packet.
    WorkspaceSearchResult,
    /// Terminal or command output.
    TerminalCommandOutput,
    /// Log capture.
    LogCapture,
    /// External tool response.
    ToolCallExternalResponse,
    /// Model Context Protocol server response.
    McpServerResponse,
    /// Web search result excerpt.
    WebSearchResult,
    /// Third-party or vendor docs excerpt.
    ExternalDocsExcerpt,
    /// Connected-provider payload.
    ConnectedProviderPayload,
    /// Extension-proposed input.
    ExtensionProposedInput,
    /// User pasted text or dropped content.
    UserSuppliedPaste,
    /// Prior AI output carried forward.
    AiPriorTurnResponse,
    /// Source was quarantined by policy.
    PolicyQuarantinedSource,
    /// Source could not be classified and must fail closed.
    UnknownUnclassifiedSource,
}

impl TaintedContextInputSourceClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepositoryWorkspaceContent => "repository_workspace_content",
            Self::WorkspaceSearchResult => "workspace_search_result",
            Self::TerminalCommandOutput => "terminal_command_output",
            Self::LogCapture => "log_capture",
            Self::ToolCallExternalResponse => "tool_call_external_response",
            Self::McpServerResponse => "mcp_server_response",
            Self::WebSearchResult => "web_search_result",
            Self::ExternalDocsExcerpt => "external_docs_excerpt",
            Self::ConnectedProviderPayload => "connected_provider_payload",
            Self::ExtensionProposedInput => "extension_proposed_input",
            Self::UserSuppliedPaste => "user_supplied_paste",
            Self::AiPriorTurnResponse => "ai_prior_turn_response",
            Self::PolicyQuarantinedSource => "policy_quarantined_source",
            Self::UnknownUnclassifiedSource => "unknown_unclassified_source",
        }
    }
}

/// Taint posture assigned to a context row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextTaintClass {
    /// First-party data that does not carry policy authority.
    TrustedFirstPartyData,
    /// Tainted evidence that must remain fenced.
    TaintedEvidence,
    /// Policy quarantined source that may only expose a summary ref.
    PolicyQuarantined,
    /// Unknown source treated as tainted and fail-closed.
    UnknownMustTreatAsTainted,
}

impl TaintedContextTaintClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedFirstPartyData => "trusted_first_party_data",
            Self::TaintedEvidence => "tainted_evidence",
            Self::PolicyQuarantined => "policy_quarantined",
            Self::UnknownMustTreatAsTainted => "unknown_must_treat_as_tainted",
        }
    }

    /// Returns true when this posture requires a tainted fence.
    pub const fn requires_fence(self) -> bool {
        matches!(
            self,
            Self::TaintedEvidence | Self::PolicyQuarantined | Self::UnknownMustTreatAsTainted
        )
    }
}

/// Origin locus disclosed for a context source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextOriginLocusClass {
    /// Local in-process source.
    LocalInProcess,
    /// Local subprocess on the same device.
    LocalSubprocessSameDevice,
    /// Local sandboxed container on the same device.
    LocalSandboxedContainerSameDevice,
    /// Remote vendor-managed service.
    RemoteVendorManagedService,
    /// Remote self-hosted service.
    RemoteSelfHostedService,
    /// Enterprise gateway brokered service.
    EnterpriseGatewayBrokeredService,
    /// Extension-provided locus.
    ExtensionProvidedLocus,
    /// Unknown locus that must be disclosed and cannot widen authority.
    UnknownLocusMustBeDisclosed,
}

impl TaintedContextOriginLocusClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalInProcess => "local_in_process",
            Self::LocalSubprocessSameDevice => "local_subprocess_same_device",
            Self::LocalSandboxedContainerSameDevice => "local_sandboxed_container_same_device",
            Self::RemoteVendorManagedService => "remote_vendor_managed_service",
            Self::RemoteSelfHostedService => "remote_self_hosted_service",
            Self::EnterpriseGatewayBrokeredService => "enterprise_gateway_brokered_service",
            Self::ExtensionProvidedLocus => "extension_provided_locus",
            Self::UnknownLocusMustBeDisclosed => "unknown_locus_must_be_disclosed",
        }
    }
}

/// Reason a source was treated as tainted or narrowing-relevant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextReasonClass {
    /// Source originated outside first-party workspace authority.
    ExternalSource,
    /// Source was runtime, terminal, log, or tool output.
    RuntimeOrToolOutput,
    /// Shared content-safety detector found suspicious presentation.
    SuspiciousContent,
    /// Imperative text could be confused with instructions.
    ImperativeTextDetected,
    /// Source attempted to authorize or widen a privileged action.
    AuthorizationOrWideningAttempt,
    /// Source is quarantined by policy.
    PolicyQuarantined,
    /// Source class or locus could not be resolved.
    UnknownUnclassified,
    /// Retrieval state was partial, stale, or provider-unavailable.
    PartialOrStaleRetrieval,
}

impl TaintedContextReasonClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalSource => "external_source",
            Self::RuntimeOrToolOutput => "runtime_or_tool_output",
            Self::SuspiciousContent => "suspicious_content",
            Self::ImperativeTextDetected => "imperative_text_detected",
            Self::AuthorizationOrWideningAttempt => "authorization_or_widening_attempt",
            Self::PolicyQuarantined => "policy_quarantined",
            Self::UnknownUnclassified => "unknown_unclassified",
            Self::PartialOrStaleRetrieval => "partial_or_stale_retrieval",
        }
    }
}

/// Retrieval truth attached to a tainted source row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextRetrievalTruthClass {
    /// Retrieval state is not applicable to this source.
    NotApplicable,
    /// Retrieval packet is promotable and fully labeled.
    Promotable,
    /// Retrieval came from a partial index.
    PartialIndex,
    /// Retrieval came from stale state.
    Stale,
    /// Provider was unavailable and fallback/import truth must be shown.
    ProviderUnavailable,
}

impl TaintedContextRetrievalTruthClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::Promotable => "promotable",
            Self::PartialIndex => "partial_index",
            Self::Stale => "stale",
            Self::ProviderUnavailable => "provider_unavailable",
        }
    }

    /// Returns true when a user-visible retrieval label is mandatory.
    pub const fn requires_label(self) -> bool {
        matches!(
            self,
            Self::PartialIndex | Self::Stale | Self::ProviderUnavailable
        )
    }
}

/// Effective execution mode after tainted-context policy is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextRunModeClass {
    /// Full run authority requested before policy narrowing.
    FullRun,
    /// Explain-only mode with no tool execution or mutation.
    ExplainOnly,
    /// Local-only mode with remote/provider widening removed.
    LocalOnly,
    /// Preview-only mode that can show a diff but cannot apply it.
    PreviewOnly,
    /// Blocked mode with no admitted run path.
    Blocked,
}

impl TaintedContextRunModeClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRun => "full_run",
            Self::ExplainOnly => "explain_only",
            Self::LocalOnly => "local_only",
            Self::PreviewOnly => "preview_only",
            Self::Blocked => "blocked",
        }
    }
}

/// Policy narrowing outcome for a decision row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextPolicyNarrowingClass {
    /// No authority was narrowed.
    NoNarrowing,
    /// Policy narrowed the run to explain-only.
    NarrowedToExplainOnly,
    /// Policy narrowed the run to local-only.
    NarrowedToLocalOnly,
    /// Policy narrowed the run to preview-only.
    NarrowedToPreviewOnly,
    /// Policy blocked the run.
    BlockedByPolicy,
}

impl TaintedContextPolicyNarrowingClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoNarrowing => "no_narrowing",
            Self::NarrowedToExplainOnly => "narrowed_to_explain_only",
            Self::NarrowedToLocalOnly => "narrowed_to_local_only",
            Self::NarrowedToPreviewOnly => "narrowed_to_preview_only",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }
}

/// Approval fence posture attached to a tainted-context decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextApprovalRequirementClass {
    /// No approval is needed because no privileged action remains.
    NoApprovalRequiredExplainOnly,
    /// Existing approval is insufficient after tainted input arrived.
    FreshApprovalRequiredAfterTaintedInput,
    /// Preview can render, but apply requires explicit approval.
    PreviewRequiresApprovalBeforeApply,
    /// No approval path is admitted for the requested action.
    MutationBlockedNoApprovalPath,
}

impl TaintedContextApprovalRequirementClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalRequiredExplainOnly => "no_approval_required_explain_only",
            Self::FreshApprovalRequiredAfterTaintedInput => {
                "fresh_approval_required_after_tainted_input"
            }
            Self::PreviewRequiresApprovalBeforeApply => "preview_requires_approval_before_apply",
            Self::MutationBlockedNoApprovalPath => "mutation_blocked_no_approval_path",
        }
    }
}

/// Promotion gate for the beta packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaintedContextPromotionGateClass {
    /// Packet can be promoted because docs/support/surface truth agrees.
    Promotable,
    /// Packet is blocked by missing or drifting operator truth.
    BlockedByTruthDrift,
}

impl TaintedContextPromotionGateClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::BlockedByTruthDrift => "blocked_by_truth_drift",
        }
    }
}

/// Policy context copied onto the tainted-context packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextPolicyContext {
    /// Policy epoch that admitted or denied the run.
    pub policy_epoch_ref: String,
    /// Workspace trust-state token.
    pub trust_state: String,
    /// Deployment-profile token.
    pub deployment_profile_class: String,
    /// Execution context ref for the evaluated run.
    pub execution_context_ref: String,
}

/// One tainted or narrowing-relevant context source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextSourceRow {
    /// Stable source ref.
    pub source_ref: String,
    /// Assembly segment ref covered by this row.
    pub segment_ref: String,
    /// Input-source class.
    pub input_source_class: TaintedContextInputSourceClass,
    /// Taint class assigned at assembly time.
    pub taint_class: TaintedContextTaintClass,
    /// Origin locus disclosed for the source.
    pub origin_locus_class: TaintedContextOriginLocusClass,
    /// Reasons the context was considered tainted or narrowing-relevant.
    pub reason_classes: Vec<TaintedContextReasonClass>,
    /// Shared suspicious-content detector outcome token, if a detector ran.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspicious_detector_outcome_token: Option<String>,
    /// Suspicious-content class tokens from the shared detector.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suspicious_content_tokens: Vec<String>,
    /// Count of suspicious findings observed by the shared detector.
    pub suspicious_finding_count: u32,
    /// Retrieval truth state for search/docs/retrieval-derived context.
    pub retrieval_truth_class: TaintedContextRetrievalTruthClass,
    /// User-visible retrieval label when state is partial, stale, or degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retrieval_truth_label: Option<String>,
    /// Evidence or assembly fence ref covering the source.
    pub fence_ref: String,
    /// Fence strategy token preserved from the context/evidence contract.
    pub fence_strategy_token: String,
    /// Usage-constraint tokens preserved across handoff.
    pub usage_constraint_tokens: Vec<String>,
    /// Review-safe explanation shown to users and support.
    pub user_visible_explanation_label: String,
    /// True when raw bodies are forbidden on this boundary.
    pub raw_body_forbidden: bool,
}

impl TaintedContextSourceRow {
    /// Returns true when this row was classified as suspicious by content safety.
    pub fn has_suspicious_content(&self) -> bool {
        self.reason_classes
            .contains(&TaintedContextReasonClass::SuspiciousContent)
            || self.suspicious_finding_count > 0
            || !self.suspicious_content_tokens.is_empty()
    }
}

/// One policy decision that narrowed or blocked a requested run mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextNarrowingDecisionRow {
    /// Stable decision ref.
    pub decision_ref: String,
    /// Mode requested before tainted-context policy was applied.
    pub requested_mode_class: TaintedContextRunModeClass,
    /// Effective mode after policy narrowing.
    pub effective_mode_class: TaintedContextRunModeClass,
    /// Policy narrowing class.
    pub policy_narrowing_class: TaintedContextPolicyNarrowingClass,
    /// Source refs that caused or informed the narrowing.
    pub source_refs: Vec<String>,
    /// Authority or capability tokens removed from the request.
    pub narrowed_authority_tokens: Vec<String>,
    /// Capability tokens denied outright.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub denied_capability_tokens: Vec<String>,
    /// Approval fence ref for this decision.
    pub approval_fence_ref: String,
    /// Approval requirement class after narrowing.
    pub approval_requirement_class: TaintedContextApprovalRequirementClass,
    /// Review-safe explanation shown to users and support.
    pub user_visible_reason_label: String,
}

/// Explicit approval fence for one tainted-context decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextApprovalFenceRow {
    /// Stable approval fence ref.
    pub approval_fence_ref: String,
    /// Decision ref this fence belongs to.
    pub decision_ref: String,
    /// Source refs covered by the fence.
    pub source_refs: Vec<String>,
    /// Approval requirement class.
    pub approval_requirement_class: TaintedContextApprovalRequirementClass,
    /// Approval ticket ref when a valid ticket already exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Audit event refs emitted for the fence.
    pub audit_event_refs: Vec<String>,
    /// Prompt-injection evaluation ref when the lower-level evaluator emitted one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_injection_evaluation_ref: Option<String>,
    /// True when provider, tool, or workspace writes are impossible without this fence.
    pub blocks_hidden_provider_write: bool,
    /// True when the fence is reconstructible from audit/support state.
    pub auditable: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_explanation_label: String,
}

/// One consumer surface's proof that it reads the same tainted-context refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextSurfaceRow {
    /// Surface class.
    pub surface_class: TaintedContextSurfaceClass,
    /// Stable projection ref for this surface.
    pub projection_ref: String,
    /// Packet ref consumed by the surface.
    pub packet_ref: String,
    /// True when tainted source refs are visible on the surface.
    pub source_refs_visible: bool,
    /// True when narrowing decision refs are visible on the surface.
    pub narrowing_decision_refs_visible: bool,
    /// True when approval fence refs are visible on the surface.
    pub approval_fence_refs_visible: bool,
    /// True when the surface preserves raw-body exclusion.
    pub raw_private_material_excluded: bool,
    /// True when the surface keeps the packet refs rather than reminting truth.
    pub preserves_operator_truth: bool,
    /// True when the surface can reach JSON export.
    pub supports_json_export: bool,
    /// True when the surface can reach Markdown summary.
    pub supports_markdown_summary: bool,
}

/// Constructor input for [`TaintedContextBetaPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaintedContextBetaInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Retrieval packet ref, when retrieval supplied tainted context.
    pub retrieval_packet_ref: String,
    /// Tainted source rows.
    pub source_rows: Vec<TaintedContextSourceRow>,
    /// Policy narrowing decisions.
    pub narrowing_decisions: Vec<TaintedContextNarrowingDecisionRow>,
    /// Approval fences.
    pub approval_fences: Vec<TaintedContextApprovalFenceRow>,
    /// Surface parity rows.
    pub surface_rows: Vec<TaintedContextSurfaceRow>,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref surfaced to support and review.
    pub json_export_ref: String,
    /// Markdown summary ref surfaced to support and review.
    pub markdown_summary_ref: String,
    /// Policy context.
    pub policy_context: TaintedContextPolicyContext,
    /// Promotion gate class.
    pub promotion_gate_class: TaintedContextPromotionGateClass,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe beta packet joining tainted context, policy narrowing, and approval fences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Retrieval packet ref, when retrieval supplied tainted context.
    pub retrieval_packet_ref: String,
    /// Required effective mode tokens for the beta claim.
    pub required_effective_mode_tokens: Vec<String>,
    /// Observed effective mode tokens from narrowing decisions.
    pub observed_effective_mode_tokens: Vec<String>,
    /// Source rows visible before approval or execution.
    pub source_rows: Vec<TaintedContextSourceRow>,
    /// Policy narrowing decisions.
    pub narrowing_decisions: Vec<TaintedContextNarrowingDecisionRow>,
    /// Explicit approval fences.
    pub approval_fences: Vec<TaintedContextApprovalFenceRow>,
    /// Surface rows that must agree on operator truth.
    pub surface_rows: Vec<TaintedContextSurfaceRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref surfaced to support and review.
    pub json_export_ref: String,
    /// Markdown summary ref surfaced to support and review.
    pub markdown_summary_ref: String,
    /// Policy context.
    pub policy_context: TaintedContextPolicyContext,
    /// Promotion gate class.
    pub promotion_gate_class: TaintedContextPromotionGateClass,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TaintedContextBetaPacket {
    /// Builds a tainted-context beta packet from canonical rows.
    pub fn new(input: TaintedContextBetaInput) -> Self {
        Self {
            record_kind: TAINTED_CONTEXT_BETA_PACKET_RECORD_KIND.to_owned(),
            schema_version: TAINTED_CONTEXT_BETA_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            context_snapshot_ref: input.context_snapshot_ref,
            evidence_packet_ref: input.evidence_packet_ref,
            retrieval_packet_ref: input.retrieval_packet_ref,
            required_effective_mode_tokens: REQUIRED_EFFECTIVE_MODES
                .iter()
                .map(|mode| mode.as_str().to_owned())
                .collect(),
            observed_effective_mode_tokens: ordered_effective_mode_tokens(
                input
                    .narrowing_decisions
                    .iter()
                    .map(|decision| decision.effective_mode_class),
            ),
            source_rows: input.source_rows,
            narrowing_decisions: input.narrowing_decisions,
            approval_fences: input.approval_fences,
            surface_rows: input.surface_rows,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            policy_context: input.policy_context,
            promotion_gate_class: input.promotion_gate_class,
            minted_at: input.minted_at,
        }
    }

    /// Validates beta promotion truth without resolving raw bodies.
    pub fn validate(&self) -> Vec<TaintedContextBetaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != TAINTED_CONTEXT_BETA_PACKET_RECORD_KIND {
            violations.push(TaintedContextBetaViolation::WrongRecordKind);
        }
        if self.schema_version != TAINTED_CONTEXT_BETA_SCHEMA_VERSION {
            violations.push(TaintedContextBetaViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.context_snapshot_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.policy_context.policy_epoch_ref.trim().is_empty()
            || self.policy_context.trust_state.trim().is_empty()
        {
            violations.push(TaintedContextBetaViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_sources(self, &mut violations);
        validate_decisions(self, &mut violations);
        validate_approval_fences(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        if self.promotion_gate_class == TaintedContextPromotionGateClass::Promotable
            && !violations.is_empty()
        {
            violations.push(TaintedContextBetaViolation::PromotionGateTruthDrift);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("tainted context beta packet serializes"),
        ) {
            violations.push(TaintedContextBetaViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("tainted context beta packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Tainted Context Beta\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Effective modes: `{}`\n",
            self.observed_effective_mode_tokens.join(",")
        ));
        out.push_str(&format!(
            "- Tainted source rows: {}\n",
            self.source_rows.len()
        ));
        out.push_str(&format!(
            "- Policy narrowing decisions: {}\n",
            self.narrowing_decisions.len()
        ));
        out.push_str(&format!(
            "- Approval fences: {}\n",
            self.approval_fences.len()
        ));
        out.push_str(&format!(
            "- Surface projections: {}\n",
            self.surface_rows.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in tainted-context support export.
#[derive(Debug)]
pub enum TaintedContextBetaArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TaintedContextBetaViolation>),
}

impl fmt::Display for TaintedContextBetaArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "tainted context export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "tainted context export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TaintedContextBetaArtifactError {}

/// Validation failures emitted by [`TaintedContextBetaPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TaintedContextBetaViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Required effective mode coverage is missing.
    MissingEffectiveModeCoverage,
    /// Source rows are missing or malformed.
    MissingSourceRows,
    /// A tainted source lacks a fence or usage constraints.
    SourceMissingFence,
    /// Suspicious source lacks shared detector tokens.
    SuspiciousSourceMissingDetectorTruth,
    /// Partial, stale, or unavailable retrieval lacks a visible label.
    RetrievalTruthUnlabelled,
    /// Policy narrowing decision is missing or malformed.
    MissingNarrowingDecision,
    /// Policy decision points at an unknown source.
    NarrowingDecisionUnknownSource,
    /// Policy decision points at a missing approval fence.
    NarrowingDecisionMissingApprovalFence,
    /// Blocked decision does not deny any capability.
    BlockedDecisionMissingDeniedCapability,
    /// Non-blocked narrowing does not name narrowed authority.
    NarrowedDecisionMissingAuthorityToken,
    /// Approval fence is missing or not auditable.
    ApprovalFenceNotAuditable,
    /// Approval fence points at an unknown decision.
    ApprovalFenceUnknownDecision,
    /// Required consumer surface projection is missing.
    MissingSurfaceProjection,
    /// Surface projection does not preserve the same operator truth refs.
    SurfaceProjectionDrift,
    /// Promotable packet contains truth drift.
    PromotionGateTruthDrift,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl TaintedContextBetaViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingEffectiveModeCoverage => "missing_effective_mode_coverage",
            Self::MissingSourceRows => "missing_source_rows",
            Self::SourceMissingFence => "source_missing_fence",
            Self::SuspiciousSourceMissingDetectorTruth => {
                "suspicious_source_missing_detector_truth"
            }
            Self::RetrievalTruthUnlabelled => "retrieval_truth_unlabelled",
            Self::MissingNarrowingDecision => "missing_narrowing_decision",
            Self::NarrowingDecisionUnknownSource => "narrowing_decision_unknown_source",
            Self::NarrowingDecisionMissingApprovalFence => {
                "narrowing_decision_missing_approval_fence"
            }
            Self::BlockedDecisionMissingDeniedCapability => {
                "blocked_decision_missing_denied_capability"
            }
            Self::NarrowedDecisionMissingAuthorityToken => {
                "narrowed_decision_missing_authority_token"
            }
            Self::ApprovalFenceNotAuditable => "approval_fence_not_auditable",
            Self::ApprovalFenceUnknownDecision => "approval_fence_unknown_decision",
            Self::MissingSurfaceProjection => "missing_surface_projection",
            Self::SurfaceProjectionDrift => "surface_projection_drift",
            Self::PromotionGateTruthDrift => "promotion_gate_truth_drift",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in beta support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_beta_tainted_context_support_export(
) -> Result<TaintedContextBetaPacket, TaintedContextBetaArtifactError> {
    let packet: TaintedContextBetaPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m3/tainted_context_beta_support_export.json"
    )))
    .map_err(TaintedContextBetaArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TaintedContextBetaArtifactError::Validation(violations))
    }
}

/// Returns detector outcome and finding class tokens for a suspicious-content detection.
pub fn suspicious_detector_tokens(
    detection: &SuspiciousContentDetection,
) -> (Option<String>, Vec<String>, u32) {
    let mut tokens = BTreeSet::new();
    for finding in &detection.findings {
        tokens.insert(finding.class.as_str().to_owned());
    }
    (
        Some(detection.outcome.as_str().to_owned()),
        tokens.into_iter().collect(),
        detection.findings.len() as u32,
    )
}

fn validate_source_contracts(
    packet: &TaintedContextBetaPacket,
    violations: &mut Vec<TaintedContextBetaViolation>,
) {
    for required in [
        TAINTED_CONTEXT_BETA_AI_DOC_REF,
        TAINTED_CONTEXT_BETA_SCHEMA_REF,
        PROMPT_INJECTION_TAINT_CONTRACT_REF,
        CONTEXT_ASSEMBLY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(TaintedContextBetaViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_sources(
    packet: &TaintedContextBetaPacket,
    violations: &mut Vec<TaintedContextBetaViolation>,
) {
    if packet.source_rows.is_empty() {
        violations.push(TaintedContextBetaViolation::MissingSourceRows);
    }

    for row in &packet.source_rows {
        if row.source_ref.trim().is_empty()
            || row.segment_ref.trim().is_empty()
            || row.reason_classes.is_empty()
            || row.user_visible_explanation_label.trim().is_empty()
            || !row.raw_body_forbidden
        {
            violations.push(TaintedContextBetaViolation::MissingSourceRows);
            break;
        }
        if row.taint_class.requires_fence()
            && (row.fence_ref.trim().is_empty()
                || row.fence_strategy_token.trim().is_empty()
                || row.usage_constraint_tokens.is_empty())
        {
            violations.push(TaintedContextBetaViolation::SourceMissingFence);
            break;
        }
        if row.has_suspicious_content()
            && (row.suspicious_detector_outcome_token.is_none()
                || row.suspicious_content_tokens.is_empty()
                || row.suspicious_finding_count == 0)
        {
            violations.push(TaintedContextBetaViolation::SuspiciousSourceMissingDetectorTruth);
            break;
        }
        if row.retrieval_truth_class.requires_label()
            && row
                .retrieval_truth_label
                .as_deref()
                .map_or(true, |label| label.trim().is_empty())
        {
            violations.push(TaintedContextBetaViolation::RetrievalTruthUnlabelled);
            break;
        }
    }
}

fn validate_decisions(
    packet: &TaintedContextBetaPacket,
    violations: &mut Vec<TaintedContextBetaViolation>,
) {
    for required in &packet.required_effective_mode_tokens {
        if !packet.observed_effective_mode_tokens.contains(required) {
            violations.push(TaintedContextBetaViolation::MissingEffectiveModeCoverage);
            break;
        }
    }
    if packet.narrowing_decisions.is_empty() {
        violations.push(TaintedContextBetaViolation::MissingNarrowingDecision);
    }

    let source_refs: BTreeSet<_> = packet
        .source_rows
        .iter()
        .map(|source| source.source_ref.as_str())
        .collect();
    let approval_fence_refs: BTreeSet<_> = packet
        .approval_fences
        .iter()
        .map(|fence| fence.approval_fence_ref.as_str())
        .collect();

    for decision in &packet.narrowing_decisions {
        if decision.decision_ref.trim().is_empty()
            || decision.source_refs.is_empty()
            || decision.approval_fence_ref.trim().is_empty()
            || decision.user_visible_reason_label.trim().is_empty()
        {
            violations.push(TaintedContextBetaViolation::MissingNarrowingDecision);
            break;
        }
        if decision
            .source_refs
            .iter()
            .any(|reference| !source_refs.contains(reference.as_str()))
        {
            violations.push(TaintedContextBetaViolation::NarrowingDecisionUnknownSource);
            break;
        }
        if !approval_fence_refs.contains(decision.approval_fence_ref.as_str()) {
            violations.push(TaintedContextBetaViolation::NarrowingDecisionMissingApprovalFence);
            break;
        }
        if decision.effective_mode_class == TaintedContextRunModeClass::Blocked {
            if decision.denied_capability_tokens.is_empty() {
                violations
                    .push(TaintedContextBetaViolation::BlockedDecisionMissingDeniedCapability);
                break;
            }
        } else if decision.policy_narrowing_class != TaintedContextPolicyNarrowingClass::NoNarrowing
            && decision.narrowed_authority_tokens.is_empty()
        {
            violations.push(TaintedContextBetaViolation::NarrowedDecisionMissingAuthorityToken);
            break;
        }
    }
}

fn validate_approval_fences(
    packet: &TaintedContextBetaPacket,
    violations: &mut Vec<TaintedContextBetaViolation>,
) {
    let decision_refs: BTreeSet<_> = packet
        .narrowing_decisions
        .iter()
        .map(|decision| decision.decision_ref.as_str())
        .collect();
    let source_refs: BTreeSet<_> = packet
        .source_rows
        .iter()
        .map(|source| source.source_ref.as_str())
        .collect();

    for fence in &packet.approval_fences {
        if fence.approval_fence_ref.trim().is_empty()
            || fence.decision_ref.trim().is_empty()
            || fence.source_refs.is_empty()
            || fence.audit_event_refs.is_empty()
            || !fence.auditable
            || !fence.blocks_hidden_provider_write
            || fence.user_visible_explanation_label.trim().is_empty()
        {
            violations.push(TaintedContextBetaViolation::ApprovalFenceNotAuditable);
            break;
        }
        if !decision_refs.contains(fence.decision_ref.as_str()) {
            violations.push(TaintedContextBetaViolation::ApprovalFenceUnknownDecision);
            break;
        }
        if fence
            .source_refs
            .iter()
            .any(|reference| !source_refs.contains(reference.as_str()))
        {
            violations.push(TaintedContextBetaViolation::ApprovalFenceNotAuditable);
            break;
        }
    }
}

fn validate_surface_rows(
    packet: &TaintedContextBetaPacket,
    violations: &mut Vec<TaintedContextBetaViolation>,
) {
    for required in [
        TaintedContextSurfaceClass::Composer,
        TaintedContextSurfaceClass::ContextInspector,
        TaintedContextSurfaceClass::ReviewWorkspace,
        TaintedContextSurfaceClass::DocsHelp,
        TaintedContextSurfaceClass::SupportExport,
    ] {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(TaintedContextBetaViolation::MissingSurfaceProjection);
            break;
        }
    }

    for row in &packet.surface_rows {
        if row.projection_ref.trim().is_empty()
            || row.packet_ref != packet.packet_id
            || !row.source_refs_visible
            || !row.narrowing_decision_refs_visible
            || !row.approval_fence_refs_visible
            || !row.raw_private_material_excluded
            || !row.preserves_operator_truth
            || !row.supports_json_export
        {
            violations.push(TaintedContextBetaViolation::SurfaceProjectionDrift);
            break;
        }
    }
}

fn ordered_effective_mode_tokens(
    modes: impl Iterator<Item = TaintedContextRunModeClass>,
) -> Vec<String> {
    let observed: BTreeSet<_> = modes.collect();
    [
        TaintedContextRunModeClass::FullRun,
        TaintedContextRunModeClass::ExplainOnly,
        TaintedContextRunModeClass::LocalOnly,
        TaintedContextRunModeClass::PreviewOnly,
        TaintedContextRunModeClass::Blocked,
    ]
    .into_iter()
    .filter(|mode| observed.contains(mode))
    .map(|mode| mode.as_str().to_owned())
    .collect()
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

fn contains_forbidden_boundary_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("oauth_token")
        || lower.contains("raw_prompt")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
