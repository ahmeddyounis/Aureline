//! AI explain, debug, and test flows with evidence links to logs, traces,
//! runbooks, and profiles.
//!
//! This module ships the canonical M5 packet for the AI explain/debug/test work
//! loops. Each flow answers a question — explain this behavior, debug this
//! failure, or propose a test — and every claim it makes must cite evidence by
//! id rather than asserting authority on its own. The flow is **read-only**: it
//! never applies a change itself. When a test flow produces a candidate edit it
//! hands that candidate to the evidence-rich patch review apply lane by id and
//! keeps the human-review boundary intact. The packet carries three bound
//! blocks:
//!
//! - An [`AiFlowBlock`] binds the flow to one kind (explain, debug, or test),
//!   intent, target, and scope, records the flow state, and asserts that every
//!   claim must cite evidence and that the flow never self-applies.
//! - An [`EvidenceLinkBlock`] enumerates the evidence links the flow consumed —
//!   each one a typed reference into a log, trace, runbook, or profile source
//!   with its freshness, provenance, and trust labels. It discloses whether
//!   every link resolved and whether any link is stale; when a link cannot be
//!   resolved or is stale, the gap stays visible rather than being silently
//!   dropped.
//! - A [`FlowFindingBlock`] presents the findings the flow produced — each one
//!   cites the evidence link ids that back it, carries a confidence class, and
//!   flags whether it needs human confirmation. Findings that cite no evidence
//!   are counted and surfaced rather than hidden, and no finding may claim
//!   authority beyond its cited evidence.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it cites the
//! [`crate::ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows`]
//! evidence lane for any test-flow apply handoff and the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix, and it projects against the frozen context-assembly contract
//! for evidence-link and omitted-context truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw log lines, raw trace bodies, raw runbook
//! text, raw profile samples, raw symbol names, raw file paths, raw diff bodies,
//! raw prompt bodies, provider payloads, endpoint URLs, credentials, raw token
//! counts, exact prices, and billing-account ids stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ai/ship-ai-explain-debug-and-test-flows-with-evidence-links-to-logs-traces-runbooks-and-profiles.schema.json`](../../../../schemas/ai/ship-ai-explain-debug-and-test-flows-with-evidence-links-to-logs-traces-runbooks-and-profiles.schema.json).
//! The contract doc is
//! [`docs/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles.md`](../../../../docs/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiFlowEvidencePacket`].
pub const AI_FLOW_EVIDENCE_RECORD_KIND: &str = "ai_explain_debug_test_flows_implementation";

/// Schema version for AI explain/debug/test flow records.
pub const AI_FLOW_EVIDENCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AI_FLOW_EVIDENCE_SCHEMA_REF: &str =
    "schemas/ai/ship-ai-explain-debug-and-test-flows-with-evidence-links-to-logs-traces-runbooks-and-profiles.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const AI_FLOW_EVIDENCE_DOC_REF: &str =
    "docs/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const AI_FLOW_EVIDENCE_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen evidence-rich patch review contract.
pub const AI_FLOW_EVIDENCE_PATCH_REVIEW_CONTRACT_REF: &str =
    "docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const AI_FLOW_EVIDENCE_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const AI_FLOW_EVIDENCE_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles";

/// Repo-relative path of the checked support-export artifact.
pub const AI_FLOW_EVIDENCE_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AI_FLOW_EVIDENCE_SUMMARY_REF: &str =
    "artifacts/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles.md";

/// Kind of work loop the flow drives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiFlowKind {
    /// Explain the behavior of a target using cited evidence.
    Explain,
    /// Debug a failure to a root-cause hypothesis using cited evidence.
    Debug,
    /// Propose tests for a target, grounded in cited evidence.
    Test,
}

impl AiFlowKind {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explain => "explain",
            Self::Debug => "debug",
            Self::Test => "test",
        }
    }
}

/// State of the flow lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiFlowState {
    /// The flow was drafted but no evidence has been gathered.
    Drafted,
    /// The flow is gathering evidence links.
    GatheringEvidence,
    /// Evidence links have been resolved and bound.
    EvidenceLinked,
    /// The flow has composed an evidence-backed answer.
    AnswerComposed,
    /// The flow is stopped at a human review gate.
    AwaitingReview,
    /// A test-flow candidate was handed to the evidence-rich patch review apply
    /// flow. The flow itself never applies.
    HandedToApply,
    /// The flow is blocked by policy, trust, or unresolved evidence.
    Blocked,
    /// The flow was discarded while preserving its evidence refs.
    Discarded,
}

impl AiFlowState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Drafted => "drafted",
            Self::GatheringEvidence => "gathering_evidence",
            Self::EvidenceLinked => "evidence_linked",
            Self::AnswerComposed => "answer_composed",
            Self::AwaitingReview => "awaiting_review",
            Self::HandedToApply => "handed_to_apply",
            Self::Blocked => "blocked",
            Self::Discarded => "discarded",
        }
    }
}

/// Kind of evidence an evidence link points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A log record or log query result.
    Log,
    /// An execution or distributed trace.
    Trace,
    /// A runbook, playbook, or operational procedure.
    Runbook,
    /// A performance profile or flamegraph capture.
    Profile,
}

impl EvidenceKind {
    /// Every evidence kind, in declaration order.
    pub const ALL: [Self; 4] = [Self::Log, Self::Trace, Self::Runbook, Self::Profile];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Trace => "trace",
            Self::Runbook => "runbook",
            Self::Profile => "profile",
        }
    }

    /// Source surface that canonically produces this evidence kind.
    pub const fn canonical_source(self) -> EvidenceSourceSurface {
        match self {
            Self::Log => EvidenceSourceSurface::LogStore,
            Self::Trace => EvidenceSourceSurface::TraceStore,
            Self::Runbook => EvidenceSourceSurface::RunbookRegistry,
            Self::Profile => EvidenceSourceSurface::ProfileStore,
        }
    }
}

/// Source surface an evidence link was drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSourceSurface {
    /// The workspace log store.
    LogStore,
    /// The workspace trace store.
    TraceStore,
    /// The runbook registry.
    RunbookRegistry,
    /// The profile store.
    ProfileStore,
}

impl EvidenceSourceSurface {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogStore => "log_store",
            Self::TraceStore => "trace_store",
            Self::RunbookRegistry => "runbook_registry",
            Self::ProfileStore => "profile_store",
        }
    }

    /// Evidence kind this surface canonically produces.
    pub const fn evidence_kind(self) -> EvidenceKind {
        match self {
            Self::LogStore => EvidenceKind::Log,
            Self::TraceStore => EvidenceKind::Trace,
            Self::RunbookRegistry => EvidenceKind::Runbook,
            Self::ProfileStore => EvidenceKind::Profile,
        }
    }
}

/// Freshness class disclosed for an evidence link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshnessClass {
    /// Captured recently and considered current.
    Fresh,
    /// Aging; still usable but approaching its freshness horizon.
    Aging,
    /// Past its freshness horizon; must be disclosed as stale.
    Stale,
    /// Freshness could not be determined.
    Unknown,
}

impl EvidenceFreshnessClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// Whether this class must be disclosed as stale.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale)
    }
}

/// Provenance class disclosed for an evidence link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceProvenanceClass {
    /// A recorded run captured by the workspace.
    RecordedRun,
    /// A live capture taken during this session.
    LiveCapture,
    /// An artifact imported from outside the workspace.
    ImportedArtifact,
    /// A summary synthesized from other evidence.
    SynthesizedSummary,
}

impl EvidenceProvenanceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecordedRun => "recorded_run",
            Self::LiveCapture => "live_capture",
            Self::ImportedArtifact => "imported_artifact",
            Self::SynthesizedSummary => "synthesized_summary",
        }
    }
}

/// Trust class disclosed for an evidence link.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceTrustClass {
    /// Evidence from a trusted, first-party source.
    Trusted,
    /// Evidence whose source could not be verified.
    Unverified,
    /// Evidence from an untrusted or tainted source.
    Untrusted,
}

impl EvidenceTrustClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Unverified => "unverified",
            Self::Untrusted => "untrusted",
        }
    }
}

/// Kind of finding the flow produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowFindingKind {
    /// An explanation of observed behavior.
    Explanation,
    /// A root-cause hypothesis for a failure.
    RootCauseHypothesis,
    /// A reproduction step.
    ReproStep,
    /// A recommended fix.
    FixRecommendation,
    /// A proposed test case.
    TestCase,
    /// A caveat or limitation on the answer.
    Caveat,
}

impl FlowFindingKind {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Explanation => "explanation",
            Self::RootCauseHypothesis => "root_cause_hypothesis",
            Self::ReproStep => "repro_step",
            Self::FixRecommendation => "fix_recommendation",
            Self::TestCase => "test_case",
            Self::Caveat => "caveat",
        }
    }
}

/// Confidence class disclosed for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingConfidenceClass {
    /// Fully backed by resolved, fresh evidence.
    Grounded,
    /// Backed by evidence but with some inference.
    Probable,
    /// Inferred with weak or no direct evidence.
    Speculative,
}

impl FindingConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grounded => "grounded",
            Self::Probable => "probable",
            Self::Speculative => "speculative",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowConsumerSurface {
    /// Desktop explain panel.
    DesktopExplainPanel,
    /// Desktop debug console.
    DesktopDebugConsole,
    /// Desktop test runner.
    DesktopTestRunner,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl FlowConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopExplainPanel,
        Self::DesktopDebugConsole,
        Self::DesktopTestRunner,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopExplainPanel => "desktop_explain_panel",
            Self::DesktopDebugConsole => "desktop_debug_console",
            Self::DesktopTestRunner => "desktop_test_runner",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Qualification class for a consumer surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowSurfaceQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental.
    Experimental,
    /// Surface is unavailable on this row.
    Unavailable,
}

impl FlowSurfaceQualificationClass {
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

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlowDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Required provider or model is unavailable.
    ProviderUnavailable,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
    /// A required evidence link could not be resolved.
    EvidenceUnresolved,
    /// Stale evidence was used without disclosure.
    EvidenceStaleUndisclosed,
    /// A finding asserted a claim without citing evidence and was surfaced as
    /// authoritative.
    UncitedClaimSurfaced,
}

impl FlowDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::EvidenceUnresolved,
        Self::EvidenceStaleUndisclosed,
        Self::UncitedClaimSurfaced,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
            Self::EvidenceUnresolved => "evidence_unresolved",
            Self::EvidenceStaleUndisclosed => "evidence_stale_undisclosed",
            Self::UncitedClaimSurfaced => "uncited_claim_surfaced",
        }
    }
}

/// Flow block binding the flow to one kind, intent, target, and scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFlowBlock {
    /// Kind of work loop this flow drives.
    pub flow_kind: AiFlowKind,
    /// Current flow state.
    pub state: AiFlowState,
    /// Opaque ref to the disclosed intent summary. Never raw prompt text.
    pub intent_summary_ref: String,
    /// Opaque ref to the target under explanation/debug/test. Never a raw symbol
    /// name or file path.
    pub target_ref: String,
    /// Opaque ref to the declared flow scope.
    pub scope_ref: String,
    /// True when every claim must cite evidence. Must be true.
    pub evidence_required_for_claims: bool,
    /// True when the flow never applies a change itself. Must be true.
    pub read_only: bool,
    /// Optional ref into the evidence-rich patch review apply lane for a
    /// test-flow candidate handoff. Never a raw diff body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_handoff_ref: Option<String>,
    /// Opaque refs to the context inputs the flow consumed.
    pub context_input_refs: Vec<String>,
}

/// One evidence link row inside the flow's evidence set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceLinkRow {
    /// Stable evidence link id, referenced by findings.
    pub link_id: String,
    /// Kind of evidence this link points at.
    pub evidence_kind: EvidenceKind,
    /// Source surface this link was drawn from.
    pub source_surface: EvidenceSourceSurface,
    /// Opaque ref to the evidence. Never a raw log line, trace body, runbook
    /// text, or profile sample.
    pub evidence_ref: String,
    /// Disclosed freshness class.
    pub freshness: EvidenceFreshnessClass,
    /// Disclosed provenance class.
    pub provenance: EvidenceProvenanceClass,
    /// Disclosed trust class.
    pub trust: EvidenceTrustClass,
    /// Opaque ref to the disclosed evidence scope.
    pub scope_ref: String,
    /// True when the link resolved to its evidence.
    pub resolved: bool,
    /// True when the link is disclosed in the evidence set. Must be true.
    pub disclosed: bool,
}

/// Evidence-link block enumerating the links the flow consumed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceLinkBlock {
    /// Stable evidence-set id.
    pub evidence_set_id: String,
    /// True when every evidence link resolved.
    pub all_links_resolved: bool,
    /// True when, on a partial resolution, the reason for the gap is disclosed.
    pub unresolved_reason_disclosed: bool,
    /// True when at least one link is stale.
    pub stale_evidence_present: bool,
    /// True when, with stale evidence present, the staleness is disclosed.
    pub stale_disclosed: bool,
    /// Evidence link rows.
    pub link_rows: Vec<EvidenceLinkRow>,
}

/// One finding row produced by the flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowFindingRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Kind of finding.
    pub finding_kind: FlowFindingKind,
    /// Disclosed confidence class.
    pub confidence: FindingConfidenceClass,
    /// Evidence link ids that back this finding. Each must appear in the
    /// evidence-link block.
    pub cited_evidence_link_ids: Vec<String>,
    /// True when the finding cites at least one evidence link. Must agree with
    /// `cited_evidence_link_ids` being non-empty.
    pub evidence_backed: bool,
    /// True when the finding requires human confirmation before being trusted.
    /// Must be true whenever the finding cites no evidence.
    pub requires_human_confirmation: bool,
    /// True when the finding is disclosed. Must be true.
    pub disclosed: bool,
}

/// Finding block presenting the findings the flow produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowFindingBlock {
    /// Stable finding-set id.
    pub finding_set_id: String,
    /// Count of findings that cite no evidence. Must equal the actual count of
    /// uncited finding rows.
    pub uncited_findings_count: u32,
    /// True when no finding claims authority beyond its cited evidence. Must be
    /// true.
    pub no_authority_beyond_evidence: bool,
    /// True when the findings were produced before any apply. Must be true.
    pub produced_before_apply: bool,
    /// Finding rows.
    pub finding_rows: Vec<FlowFindingRow>,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlowSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: FlowConsumerSurface,
    /// True when this surface shows the flow.
    pub shows_flow: bool,
    /// True when this surface shows the evidence links.
    pub shows_evidence_links: bool,
    /// True when this surface shows the findings.
    pub shows_findings: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: FlowSurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

/// Constructor input for [`AiFlowEvidencePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiFlowEvidencePacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical flow id shared across surfaces and evidence.
    pub flow_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Flow block.
    pub flow: AiFlowBlock,
    /// Evidence-link block.
    pub evidence_links: EvidenceLinkBlock,
    /// Finding block.
    pub findings: FlowFindingBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<FlowSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<FlowDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI explain/debug/test flow record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiFlowEvidencePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical flow id shared across surfaces and evidence.
    pub flow_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Flow block.
    pub flow: AiFlowBlock,
    /// Evidence-link block.
    pub evidence_links: EvidenceLinkBlock,
    /// Finding block.
    pub findings: FlowFindingBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<FlowSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<FlowDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiFlowEvidencePacket {
    /// Builds an AI explain/debug/test flow packet from the stable-lane input.
    pub fn new(input: AiFlowEvidencePacketInput) -> Self {
        Self {
            record_kind: AI_FLOW_EVIDENCE_RECORD_KIND.to_owned(),
            schema_version: AI_FLOW_EVIDENCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            flow_id: input.flow_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            flow: input.flow,
            evidence_links: input.evidence_links,
            findings: input.findings,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the flow packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiFlowEvidenceViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_FLOW_EVIDENCE_RECORD_KIND {
            violations.push(AiFlowEvidenceViolation::WrongRecordKind);
        }
        if self.schema_version != AI_FLOW_EVIDENCE_SCHEMA_VERSION {
            violations.push(AiFlowEvidenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.flow_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiFlowEvidenceViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_flow(self, &mut violations);
        validate_evidence_links(self, &mut violations);
        validate_findings(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("flow evidence packet serializes"),
        ) {
            violations.push(AiFlowEvidenceViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("flow evidence packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let resolved_links = self
            .evidence_links
            .link_rows
            .iter()
            .filter(|row| row.resolved)
            .count();
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# AI Explain/Debug/Test Flows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Flow: `{}`\n", self.flow_id));
        out.push_str(&format!(
            "- Kind: `{}` (state: `{}`)\n",
            self.flow.flow_kind.as_str(),
            self.flow.state.as_str()
        ));
        out.push_str(&format!(
            "- Evidence: `{}` ({} links, {} resolved, stale present: {})\n",
            self.evidence_links.evidence_set_id,
            self.evidence_links.link_rows.len(),
            resolved_links,
            self.evidence_links.stale_evidence_present
        ));
        out.push_str(&format!(
            "- Findings: `{}` ({} findings, {} uncited, produced before apply: {})\n",
            self.findings.finding_set_id,
            self.findings.finding_rows.len(),
            self.findings.uncited_findings_count,
            self.findings.produced_before_apply
        ));
        out.push_str(&format!(
            "- Surface parity: {} surfaces ({} stable)\n",
            self.consumer_surface_parity.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Downgrade triggers: {}\n",
            self.downgrade_triggers.len()
        ));
        out
    }
}

/// Validation failures emitted by [`AiFlowEvidencePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiFlowEvidenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Flow block is incomplete.
    FlowIncomplete,
    /// The flow does not require evidence for its claims.
    EvidenceNotRequiredForClaims,
    /// The flow is not read-only.
    FlowNotReadOnly,
    /// The evidence set has no links.
    EvidenceSetEmpty,
    /// An evidence link reached the set without being disclosed.
    HiddenEvidenceLink,
    /// An evidence link is missing required identity or refs.
    EvidenceLinkIncomplete,
    /// The evidence set is partially resolved without disclosing the reason.
    UnresolvedEvidenceUndisclosed,
    /// Stale evidence is present without disclosure.
    StaleEvidenceUndisclosed,
    /// The finding set has no findings.
    FindingSetEmpty,
    /// A finding is missing required identity.
    FindingIncomplete,
    /// A finding is disclosed without being marked disclosed.
    HiddenFinding,
    /// A finding cites an evidence link id absent from the evidence set.
    DanglingEvidenceCitation,
    /// A finding's `evidence_backed` flag disagrees with its citations.
    EvidenceBackedFlagMismatch,
    /// A finding cites no evidence but does not require human confirmation.
    UncitedFindingNotFlagged,
    /// The disclosed uncited-finding count disagrees with the actual count.
    UncitedCountMismatch,
    /// A finding claims authority beyond its cited evidence.
    AuthorityBeyondEvidence,
    /// Findings were not produced before apply.
    FindingsNotProducedBeforeApply,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiFlowEvidenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::FlowIncomplete => "flow_incomplete",
            Self::EvidenceNotRequiredForClaims => "evidence_not_required_for_claims",
            Self::FlowNotReadOnly => "flow_not_read_only",
            Self::EvidenceSetEmpty => "evidence_set_empty",
            Self::HiddenEvidenceLink => "hidden_evidence_link",
            Self::EvidenceLinkIncomplete => "evidence_link_incomplete",
            Self::UnresolvedEvidenceUndisclosed => "unresolved_evidence_undisclosed",
            Self::StaleEvidenceUndisclosed => "stale_evidence_undisclosed",
            Self::FindingSetEmpty => "finding_set_empty",
            Self::FindingIncomplete => "finding_incomplete",
            Self::HiddenFinding => "hidden_finding",
            Self::DanglingEvidenceCitation => "dangling_evidence_citation",
            Self::EvidenceBackedFlagMismatch => "evidence_backed_flag_mismatch",
            Self::UncitedFindingNotFlagged => "uncited_finding_not_flagged",
            Self::UncitedCountMismatch => "uncited_count_mismatch",
            Self::AuthorityBeyondEvidence => "authority_beyond_evidence",
            Self::FindingsNotProducedBeforeApply => "findings_not_produced_before_apply",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

impl fmt::Display for AiFlowEvidenceViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Error for AiFlowEvidenceViolation {}

/// Errors emitted when reading the checked-in flow evidence export.
#[derive(Debug)]
pub enum AiFlowEvidenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiFlowEvidenceViolation>),
}

impl fmt::Display for AiFlowEvidenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "flow evidence export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "flow evidence export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiFlowEvidenceArtifactError {}

/// Returns the checked-in AI explain/debug/test flow export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_ai_flow_evidence_export(
) -> Result<AiFlowEvidencePacket, AiFlowEvidenceArtifactError> {
    let packet: AiFlowEvidencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_ai_explain_debug_and_test_flows_with_evidence_links_to_logs_traces_runbooks_and_profiles/support_export.json"
    )))
    .map_err(AiFlowEvidenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiFlowEvidenceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiFlowEvidencePacket,
    violations: &mut Vec<AiFlowEvidenceViolation>,
) {
    for required in [
        AI_FLOW_EVIDENCE_DOC_REF,
        AI_FLOW_EVIDENCE_SCHEMA_REF,
        AI_FLOW_EVIDENCE_CONTEXT_ASSEMBLY_CONTRACT_REF,
        AI_FLOW_EVIDENCE_PATCH_REVIEW_CONTRACT_REF,
        AI_FLOW_EVIDENCE_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiFlowEvidenceViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_flow(packet: &AiFlowEvidencePacket, violations: &mut Vec<AiFlowEvidenceViolation>) {
    let flow = &packet.flow;
    if flow.intent_summary_ref.trim().is_empty()
        || flow.target_ref.trim().is_empty()
        || flow.scope_ref.trim().is_empty()
        || flow.context_input_refs.is_empty()
    {
        violations.push(AiFlowEvidenceViolation::FlowIncomplete);
    }
    if let Some(handoff) = &flow.apply_handoff_ref {
        if handoff.trim().is_empty() {
            violations.push(AiFlowEvidenceViolation::FlowIncomplete);
        }
    }
    if !flow.evidence_required_for_claims {
        violations.push(AiFlowEvidenceViolation::EvidenceNotRequiredForClaims);
    }
    if !flow.read_only {
        violations.push(AiFlowEvidenceViolation::FlowNotReadOnly);
    }
}

fn validate_evidence_links(
    packet: &AiFlowEvidencePacket,
    violations: &mut Vec<AiFlowEvidenceViolation>,
) {
    let evidence = &packet.evidence_links;
    if evidence.evidence_set_id.trim().is_empty() || evidence.link_rows.is_empty() {
        violations.push(AiFlowEvidenceViolation::EvidenceSetEmpty);
        return;
    }
    let mut any_unresolved = false;
    let mut any_stale = false;
    for link in &evidence.link_rows {
        if link.link_id.trim().is_empty()
            || link.evidence_ref.trim().is_empty()
            || link.scope_ref.trim().is_empty()
        {
            violations.push(AiFlowEvidenceViolation::EvidenceLinkIncomplete);
        }
        if !link.disclosed {
            violations.push(AiFlowEvidenceViolation::HiddenEvidenceLink);
        }
        if !link.resolved {
            any_unresolved = true;
        }
        if link.freshness.is_stale() {
            any_stale = true;
        }
    }
    if any_unresolved && (evidence.all_links_resolved || !evidence.unresolved_reason_disclosed) {
        violations.push(AiFlowEvidenceViolation::UnresolvedEvidenceUndisclosed);
    }
    if any_stale && (!evidence.stale_evidence_present || !evidence.stale_disclosed) {
        violations.push(AiFlowEvidenceViolation::StaleEvidenceUndisclosed);
    }
}

fn validate_findings(packet: &AiFlowEvidencePacket, violations: &mut Vec<AiFlowEvidenceViolation>) {
    let findings = &packet.findings;
    if findings.finding_set_id.trim().is_empty() || findings.finding_rows.is_empty() {
        violations.push(AiFlowEvidenceViolation::FindingSetEmpty);
        return;
    }
    if !findings.produced_before_apply {
        violations.push(AiFlowEvidenceViolation::FindingsNotProducedBeforeApply);
    }
    if !findings.no_authority_beyond_evidence {
        violations.push(AiFlowEvidenceViolation::AuthorityBeyondEvidence);
    }
    let known_links: std::collections::HashSet<&str> = packet
        .evidence_links
        .link_rows
        .iter()
        .map(|link| link.link_id.as_str())
        .collect();
    let mut uncited = 0u32;
    for finding in &findings.finding_rows {
        if finding.finding_id.trim().is_empty() {
            violations.push(AiFlowEvidenceViolation::FindingIncomplete);
        }
        if !finding.disclosed {
            violations.push(AiFlowEvidenceViolation::HiddenFinding);
        }
        let has_citations = !finding.cited_evidence_link_ids.is_empty();
        if has_citations != finding.evidence_backed {
            violations.push(AiFlowEvidenceViolation::EvidenceBackedFlagMismatch);
        }
        for cited in &finding.cited_evidence_link_ids {
            if !known_links.contains(cited.as_str()) {
                violations.push(AiFlowEvidenceViolation::DanglingEvidenceCitation);
            }
        }
        if !has_citations {
            uncited += 1;
            if !finding.requires_human_confirmation {
                violations.push(AiFlowEvidenceViolation::UncitedFindingNotFlagged);
            }
        }
    }
    if findings.uncited_findings_count != uncited {
        violations.push(AiFlowEvidenceViolation::UncitedCountMismatch);
    }
}

fn validate_consumer_surface_parity(
    packet: &AiFlowEvidencePacket,
    violations: &mut Vec<AiFlowEvidenceViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(AiFlowEvidenceViolation::StableClaimNotQualified);
        }
    }
    for required in FlowConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations.push(AiFlowEvidenceViolation::ConsumerSurfaceCoverageMissing);
            break;
        }
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
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
