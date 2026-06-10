//! AI review findings, ownership hints, and review-workspace integration with
//! durable anchors.
//!
//! This module ships the canonical M5 packet for AI-assisted code review. An AI
//! review pass produces **findings** about a change, each one anchored to a
//! location by a **durable anchor** that survives edits, hints at the **owners**
//! of the affected areas, and integrates into the **review workspace** behind a
//! human gate. The pass is **read-only**: it never applies a change and never
//! self-publishes findings without human review. Every finding must cite
//! evidence by id rather than asserting authority on its own. The packet carries
//! four bound blocks:
//!
//! - A [`ReviewFindingsBlock`] presents the findings the pass produced — each
//!   one carries a finding class, severity, and confidence, binds to a
//!   [`DurableAnchor`], cites the evidence refs that back it, and flags whether
//!   it needs human confirmation. Findings that cite no evidence are counted and
//!   surfaced rather than hidden, and no finding may claim authority beyond its
//!   cited evidence.
//! - Each [`DurableAnchor`] binds a finding to a location by an anchor strategy
//!   (symbol path, content hash, structural node, or line range) so the finding
//!   survives edits. When the anchored location drifts or is lost, the anchor
//!   discloses the drift and its rebind disposition rather than silently
//!   reattaching to the wrong place or vanishing.
//! - An [`OwnershipHintsBlock`] presents ownership hints for the affected areas —
//!   each one a typed reference to an owner drawn from a code-owners file, commit
//!   history, a declared team, or a heuristic, carrying its confidence and
//!   whether it is a reviewer suggestion. The hints are **advisory**: the AI
//!   never auto-assigns a reviewer or treats a hint as authority.
//! - A [`ReviewWorkspaceIntegrationBlock`] binds the findings to the review
//!   workspace — its publish state, destination, the review-pack digest it
//!   projects against, and the evidence-packet lineage it inherits. Publishing
//!   into review requires a human gate; the AI never self-publishes.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it cites the prior canonical
//! [`crate::ai_review_assist`] truth lane for finding rows, scope selectors,
//! publish-to-review sheets, and resolution memory; the
//! [`crate::ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows`]
//! evidence lane for evidence-packet lineage; and the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix. It projects against the frozen context-assembly contract for
//! evidence-citation and omitted-context truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw source lines, raw symbol names, raw file
//! paths, raw diff bodies, raw owner identities, email addresses, raw prompt
//! bodies, provider payloads, endpoint URLs, credentials, raw token counts,
//! exact prices, and billing-account ids stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/implement-ai-review-findings-ownership-hints-and-review-workspace-integration-with-durable-anchors.schema.json`](../../../../schemas/ai/implement-ai-review-findings-ownership-hints-and-review-workspace-integration-with-durable-anchors.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors.md`](../../../../docs/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiReviewFindingsPacket`].
pub const AI_REVIEW_FINDINGS_RECORD_KIND: &str =
    "ai_review_findings_ownership_anchors_implementation";

/// Schema version for AI review-findings records.
pub const AI_REVIEW_FINDINGS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AI_REVIEW_FINDINGS_SCHEMA_REF: &str =
    "schemas/ai/implement-ai-review-findings-ownership-hints-and-review-workspace-integration-with-durable-anchors.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const AI_REVIEW_FINDINGS_DOC_REF: &str =
    "docs/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const AI_REVIEW_FINDINGS_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the prior canonical AI review-assist truth contract.
pub const AI_REVIEW_FINDINGS_REVIEW_ASSIST_CONTRACT_REF: &str =
    "docs/ai/m4/ai-review-assist-and-publish-truth.md";

/// Repo-relative path of the stable review-pack evaluator contract.
pub const AI_REVIEW_FINDINGS_REVIEW_PACK_CONTRACT_REF: &str =
    "docs/review/m4/review-pack-evaluator-and-local-ci-parity.md";

/// Repo-relative path of the frozen evidence-rich patch review contract.
pub const AI_REVIEW_FINDINGS_EVIDENCE_PACKET_CONTRACT_REF: &str =
    "docs/ai/m5/ship_evidence_rich_patch_review_with_diff_packets_validation_receipts_and_rollback_handles_across_apply_flows.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const AI_REVIEW_FINDINGS_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const AI_REVIEW_FINDINGS_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors";

/// Repo-relative path of the checked support-export artifact.
pub const AI_REVIEW_FINDINGS_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AI_REVIEW_FINDINGS_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors.md";

/// Class of issue an AI review finding raises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFindingClass {
    /// A correctness or logic risk.
    BugRisk,
    /// A security or privacy concern.
    Security,
    /// A performance or resource concern.
    Performance,
    /// A maintainability or design concern.
    Maintainability,
    /// A style or convention concern.
    Style,
    /// A missing-test or coverage gap.
    TestGap,
    /// A documentation gap.
    Documentation,
}

impl ReviewFindingClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BugRisk => "bug_risk",
            Self::Security => "security",
            Self::Performance => "performance",
            Self::Maintainability => "maintainability",
            Self::Style => "style",
            Self::TestGap => "test_gap",
            Self::Documentation => "documentation",
        }
    }
}

/// Severity class disclosed for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverityClass {
    /// Must be resolved before merge.
    Blocker,
    /// A significant issue that should be resolved.
    Major,
    /// A minor issue.
    Minor,
    /// A trivial nit, non-blocking.
    Nit,
}

impl FindingSeverityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::Nit => "nit",
        }
    }
}

/// Confidence class disclosed for a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingConfidenceClass {
    /// Fully backed by resolved evidence.
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

/// Resolution state of a finding within the review workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingResolutionState {
    /// The finding is open and unaddressed.
    Open,
    /// A human acknowledged the finding.
    Acknowledged,
    /// The finding was resolved.
    Resolved,
    /// The finding was dismissed by a human.
    Dismissed,
    /// The finding was deferred to a later pass.
    Deferred,
}

impl FindingResolutionState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
            Self::Dismissed => "dismissed",
            Self::Deferred => "deferred",
        }
    }
}

/// Strategy a durable anchor uses to bind a finding to a location.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorStrategy {
    /// Anchored to a resolved symbol path.
    SymbolPath,
    /// Anchored to a content hash of the surrounding region.
    ContentHash,
    /// Anchored to a structural syntax-tree node.
    StructuralNode,
    /// Anchored to a line range. The weakest, drift-prone strategy.
    LineRange,
}

impl AnchorStrategy {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymbolPath => "symbol_path",
            Self::ContentHash => "content_hash",
            Self::StructuralNode => "structural_node",
            Self::LineRange => "line_range",
        }
    }
}

/// Lifecycle state of a durable anchor after edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnchorState {
    /// The anchor is bound to its original location.
    Bound,
    /// The anchored location moved; the anchor has not yet rebound.
    Drifted,
    /// The anchor reattached to the moved location.
    Rebound,
    /// The anchored location no longer exists; the anchor is lost.
    Lost,
}

impl AnchorState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bound => "bound",
            Self::Drifted => "drifted",
            Self::Rebound => "rebound",
            Self::Lost => "lost",
        }
    }

    /// Whether this state means the original location moved or vanished.
    pub const fn is_disturbed(self) -> bool {
        matches!(self, Self::Drifted | Self::Lost | Self::Rebound)
    }
}

/// Source an ownership hint was drawn from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipSource {
    /// A checked-in code-owners file.
    CodeownersFile,
    /// Commit or blame history.
    CommitHistory,
    /// A declared team or area mapping.
    DeclaredTeam,
    /// A heuristic inference. The weakest source.
    Heuristic,
}

impl OwnershipSource {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeownersFile => "codeowners_file",
            Self::CommitHistory => "commit_history",
            Self::DeclaredTeam => "declared_team",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Confidence class disclosed for an ownership hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnershipConfidenceClass {
    /// Drawn from an authoritative, explicit source.
    Strong,
    /// Drawn from a plausible but indirect source.
    Moderate,
    /// Inferred heuristically.
    Weak,
}

impl OwnershipConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Strong => "strong",
            Self::Moderate => "moderate",
            Self::Weak => "weak",
        }
    }
}

/// Publish state of the findings within the review workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishState {
    /// The findings are a private draft.
    Draft,
    /// The findings are staged for human review.
    Staged,
    /// The findings were published into the review surface after a human gate.
    PublishedToReview,
    /// The findings were withdrawn from review.
    Withdrawn,
}

impl PublishState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Staged => "staged",
            Self::PublishedToReview => "published_to_review",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Whether this state means findings are live in the review surface.
    pub const fn is_published(self) -> bool {
        matches!(self, Self::PublishedToReview)
    }
}

/// Destination the findings publish into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishDestination {
    /// A side-branch review pack.
    ReviewPack,
    /// Inline review threads in the editor.
    InlineThread,
    /// A set of pull-request comments.
    PrCommentSet,
}

impl PublishDestination {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPack => "review_pack",
            Self::InlineThread => "inline_thread",
            Self::PrCommentSet => "pr_comment_set",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewConsumerSurface {
    /// Desktop review panel.
    DesktopReviewPanel,
    /// Desktop editor gutter / inline annotations.
    DesktopEditorGutter,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Browser companion surface.
    BrowserCompanion,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl ReviewConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopReviewPanel,
        Self::DesktopEditorGutter,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReviewPanel => "desktop_review_panel",
            Self::DesktopEditorGutter => "desktop_editor_gutter",
            Self::CliHeadless => "cli_headless",
            Self::BrowserCompanion => "browser_companion",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Qualification class for a consumer surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSurfaceQualificationClass {
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

impl ReviewSurfaceQualificationClass {
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
pub enum ReviewDowngradeTrigger {
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
    /// A durable anchor drifted or was lost without disclosure.
    AnchorLostUndisclosed,
    /// An ownership hint was treated as authority rather than advice.
    OwnershipHintTreatedAsAuthority,
    /// Findings reached the review surface without a human gate.
    PublishedWithoutHumanGate,
    /// A finding asserted a claim without citing evidence and was surfaced as
    /// authoritative.
    UncitedClaimSurfaced,
}

impl ReviewDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::AnchorLostUndisclosed,
        Self::OwnershipHintTreatedAsAuthority,
        Self::PublishedWithoutHumanGate,
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
            Self::AnchorLostUndisclosed => "anchor_lost_undisclosed",
            Self::OwnershipHintTreatedAsAuthority => "ownership_hint_treated_as_authority",
            Self::PublishedWithoutHumanGate => "published_without_human_gate",
            Self::UncitedClaimSurfaced => "uncited_claim_surfaced",
        }
    }
}

/// A durable anchor binding a finding to a location that survives edits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableAnchor {
    /// Stable anchor id.
    pub anchor_id: String,
    /// Strategy used to bind the anchor.
    pub strategy: AnchorStrategy,
    /// Opaque ref to the anchored target. Never a raw symbol name or file path.
    pub target_ref: String,
    /// Opaque ref to the anchored scope.
    pub scope_ref: String,
    /// Lifecycle state after edits.
    pub state: AnchorState,
    /// True when the anchored location moved or vanished.
    pub drift_detected: bool,
    /// True when the anchor's drift or rebind disposition is disclosed. Must be
    /// true whenever `drift_detected` is true.
    pub rebind_disclosed: bool,
    /// True when the anchor is durable across edits. Must be true.
    pub durable: bool,
}

/// One AI review finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFindingRow {
    /// Stable finding id, referenced by the workspace publish set.
    pub finding_id: String,
    /// Class of issue this finding raises.
    pub finding_class: ReviewFindingClass,
    /// Disclosed severity class.
    pub severity: FindingSeverityClass,
    /// Disclosed confidence class.
    pub confidence: FindingConfidenceClass,
    /// Durable anchor binding this finding to a location.
    pub anchor: DurableAnchor,
    /// Evidence refs that back this finding.
    pub cited_evidence_refs: Vec<String>,
    /// True when the finding cites at least one evidence ref. Must agree with
    /// `cited_evidence_refs` being non-empty.
    pub evidence_backed: bool,
    /// Resolution state within the review workspace.
    pub resolution_state: FindingResolutionState,
    /// True when the finding requires human confirmation before being trusted.
    /// Must be true whenever the finding cites no evidence.
    pub requires_human_confirmation: bool,
    /// True when the finding is disclosed. Must be true.
    pub disclosed: bool,
}

/// Finding block presenting the findings the review pass produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewFindingsBlock {
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
    pub finding_rows: Vec<ReviewFindingRow>,
}

/// One ownership hint row for an affected area.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipHintRow {
    /// Stable hint id.
    pub hint_id: String,
    /// Source this hint was drawn from.
    pub ownership_source: OwnershipSource,
    /// Opaque ref to the owner. Never a raw identity, handle, or email address.
    pub owner_ref: String,
    /// Disclosed confidence class.
    pub confidence: OwnershipConfidenceClass,
    /// True when this hint is offered as a reviewer suggestion.
    pub suggested_reviewer: bool,
    /// Opaque ref to the area this hint covers.
    pub scope_ref: String,
    /// True when this hint is advisory only. Must be true.
    pub advisory: bool,
    /// True when the hint is disclosed. Must be true.
    pub disclosed: bool,
}

/// Ownership-hint block presenting hints for the affected areas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnershipHintsBlock {
    /// Stable hint-set id.
    pub hint_set_id: String,
    /// True when every hint is advisory rather than authoritative. Must be true.
    pub hints_are_advisory: bool,
    /// True when the AI never auto-assigns a reviewer from these hints. Must be
    /// true.
    pub no_auto_assignment: bool,
    /// Ownership hint rows.
    pub hint_rows: Vec<OwnershipHintRow>,
}

/// Review-workspace integration block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceIntegrationBlock {
    /// Stable review-workspace id.
    pub workspace_id: String,
    /// Current publish state of the findings.
    pub publish_state: PublishState,
    /// Destination the findings publish into.
    pub publish_destination: PublishDestination,
    /// True when publishing into review requires a human gate. Must be true.
    pub human_gate_required: bool,
    /// True when a human gate cleared the publish. Must be true whenever the
    /// publish state is published-to-review.
    pub human_gated: bool,
    /// Opaque ref to the review-pack digest this lane projects against.
    pub review_pack_digest_ref: String,
    /// Opaque refs to the evidence-packet lineage the findings inherit.
    pub evidence_packet_lineage_refs: Vec<String>,
    /// Finding ids published into the review surface. Each must appear in the
    /// finding block.
    pub published_finding_ids: Vec<String>,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: ReviewConsumerSurface,
    /// True when this surface shows the findings.
    pub shows_findings: bool,
    /// True when this surface shows the durable anchors.
    pub shows_anchors: bool,
    /// True when this surface shows the ownership hints.
    pub shows_ownership_hints: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: ReviewSurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

/// Constructor input for [`AiReviewFindingsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiReviewFindingsPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical review-pass id shared across surfaces and evidence.
    pub review_pass_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Finding block.
    pub findings: ReviewFindingsBlock,
    /// Ownership-hint block.
    pub ownership_hints: OwnershipHintsBlock,
    /// Review-workspace integration block.
    pub workspace_integration: ReviewWorkspaceIntegrationBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<ReviewSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<ReviewDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI review-findings record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewFindingsPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical review-pass id shared across surfaces and evidence.
    pub review_pass_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Finding block.
    pub findings: ReviewFindingsBlock,
    /// Ownership-hint block.
    pub ownership_hints: OwnershipHintsBlock,
    /// Review-workspace integration block.
    pub workspace_integration: ReviewWorkspaceIntegrationBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<ReviewSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<ReviewDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiReviewFindingsPacket {
    /// Builds an AI review-findings packet from the stable-lane input.
    pub fn new(input: AiReviewFindingsPacketInput) -> Self {
        Self {
            record_kind: AI_REVIEW_FINDINGS_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_FINDINGS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            review_pass_id: input.review_pass_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            findings: input.findings,
            ownership_hints: input.ownership_hints,
            workspace_integration: input.workspace_integration,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the review-findings packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiReviewFindingsViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_REVIEW_FINDINGS_RECORD_KIND {
            violations.push(AiReviewFindingsViolation::WrongRecordKind);
        }
        if self.schema_version != AI_REVIEW_FINDINGS_SCHEMA_VERSION {
            violations.push(AiReviewFindingsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.review_pass_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiReviewFindingsViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_findings(self, &mut violations);
        validate_ownership_hints(self, &mut violations);
        validate_workspace_integration(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review findings packet serializes"),
        ) {
            violations.push(AiReviewFindingsViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("review findings packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let drifted_anchors = self
            .findings
            .finding_rows
            .iter()
            .filter(|row| row.anchor.state.is_disturbed())
            .count();
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# AI Review Findings, Ownership Hints, and Durable Anchors\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Review pass: `{}`\n", self.review_pass_id));
        out.push_str(&format!(
            "- Findings: `{}` ({} findings, {} uncited, {} drifted anchors)\n",
            self.findings.finding_set_id,
            self.findings.finding_rows.len(),
            self.findings.uncited_findings_count,
            drifted_anchors
        ));
        out.push_str(&format!(
            "- Ownership hints: `{}` ({} hints, advisory: {}, no auto-assign: {})\n",
            self.ownership_hints.hint_set_id,
            self.ownership_hints.hint_rows.len(),
            self.ownership_hints.hints_are_advisory,
            self.ownership_hints.no_auto_assignment
        ));
        out.push_str(&format!(
            "- Workspace: `{}` (state: `{}`, destination: `{}`, human gated: {})\n",
            self.workspace_integration.workspace_id,
            self.workspace_integration.publish_state.as_str(),
            self.workspace_integration.publish_destination.as_str(),
            self.workspace_integration.human_gated
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

/// Validation failures emitted by [`AiReviewFindingsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiReviewFindingsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The finding set has no findings.
    FindingSetEmpty,
    /// A finding is missing required identity.
    FindingIncomplete,
    /// A finding is disclosed without being marked disclosed.
    HiddenFinding,
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
    /// A durable anchor is missing required identity or refs.
    AnchorIncomplete,
    /// A durable anchor is not marked durable.
    AnchorNotDurable,
    /// A durable anchor drifted or was lost without disclosure.
    AnchorDriftUndisclosed,
    /// The ownership-hint set is not marked advisory.
    OwnershipHintsNotAdvisory,
    /// The ownership-hint set allows auto-assignment.
    OwnershipAutoAssignmentAllowed,
    /// An ownership hint is missing required identity or refs.
    OwnershipHintIncomplete,
    /// An ownership hint reached the set without being disclosed.
    HiddenOwnershipHint,
    /// The review-workspace integration is missing required identity or refs.
    WorkspaceIntegrationIncomplete,
    /// The review-workspace integration does not require a human gate.
    HumanGateNotRequired,
    /// Findings reached the review surface without a human gate.
    PublishedWithoutHumanGate,
    /// A published finding id is absent from the finding set.
    DanglingPublishedFinding,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiReviewFindingsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::FindingSetEmpty => "finding_set_empty",
            Self::FindingIncomplete => "finding_incomplete",
            Self::HiddenFinding => "hidden_finding",
            Self::EvidenceBackedFlagMismatch => "evidence_backed_flag_mismatch",
            Self::UncitedFindingNotFlagged => "uncited_finding_not_flagged",
            Self::UncitedCountMismatch => "uncited_count_mismatch",
            Self::AuthorityBeyondEvidence => "authority_beyond_evidence",
            Self::FindingsNotProducedBeforeApply => "findings_not_produced_before_apply",
            Self::AnchorIncomplete => "anchor_incomplete",
            Self::AnchorNotDurable => "anchor_not_durable",
            Self::AnchorDriftUndisclosed => "anchor_drift_undisclosed",
            Self::OwnershipHintsNotAdvisory => "ownership_hints_not_advisory",
            Self::OwnershipAutoAssignmentAllowed => "ownership_auto_assignment_allowed",
            Self::OwnershipHintIncomplete => "ownership_hint_incomplete",
            Self::HiddenOwnershipHint => "hidden_ownership_hint",
            Self::WorkspaceIntegrationIncomplete => "workspace_integration_incomplete",
            Self::HumanGateNotRequired => "human_gate_not_required",
            Self::PublishedWithoutHumanGate => "published_without_human_gate",
            Self::DanglingPublishedFinding => "dangling_published_finding",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

impl fmt::Display for AiReviewFindingsViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Error for AiReviewFindingsViolation {}

/// Errors emitted when reading the checked-in review-findings export.
#[derive(Debug)]
pub enum AiReviewFindingsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiReviewFindingsViolation>),
}

impl fmt::Display for AiReviewFindingsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "review findings export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "review findings export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiReviewFindingsArtifactError {}

/// Returns the checked-in AI review-findings export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_ai_review_findings_export(
) -> Result<AiReviewFindingsPacket, AiReviewFindingsArtifactError> {
    let packet: AiReviewFindingsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_ai_review_findings_ownership_hints_and_review_workspace_integration_with_durable_anchors/support_export.json"
    )))
    .map_err(AiReviewFindingsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiReviewFindingsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiReviewFindingsPacket,
    violations: &mut Vec<AiReviewFindingsViolation>,
) {
    for required in [
        AI_REVIEW_FINDINGS_DOC_REF,
        AI_REVIEW_FINDINGS_SCHEMA_REF,
        AI_REVIEW_FINDINGS_CONTEXT_ASSEMBLY_CONTRACT_REF,
        AI_REVIEW_FINDINGS_REVIEW_ASSIST_CONTRACT_REF,
        AI_REVIEW_FINDINGS_REVIEW_PACK_CONTRACT_REF,
        AI_REVIEW_FINDINGS_EVIDENCE_PACKET_CONTRACT_REF,
        AI_REVIEW_FINDINGS_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiReviewFindingsViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_findings(
    packet: &AiReviewFindingsPacket,
    violations: &mut Vec<AiReviewFindingsViolation>,
) {
    let findings = &packet.findings;
    if findings.finding_set_id.trim().is_empty() || findings.finding_rows.is_empty() {
        violations.push(AiReviewFindingsViolation::FindingSetEmpty);
        return;
    }
    if !findings.produced_before_apply {
        violations.push(AiReviewFindingsViolation::FindingsNotProducedBeforeApply);
    }
    if !findings.no_authority_beyond_evidence {
        violations.push(AiReviewFindingsViolation::AuthorityBeyondEvidence);
    }
    let mut uncited = 0u32;
    for finding in &findings.finding_rows {
        if finding.finding_id.trim().is_empty() {
            violations.push(AiReviewFindingsViolation::FindingIncomplete);
        }
        if !finding.disclosed {
            violations.push(AiReviewFindingsViolation::HiddenFinding);
        }
        let has_citations = !finding.cited_evidence_refs.is_empty();
        if has_citations != finding.evidence_backed {
            violations.push(AiReviewFindingsViolation::EvidenceBackedFlagMismatch);
        }
        if !has_citations {
            uncited += 1;
            if !finding.requires_human_confirmation {
                violations.push(AiReviewFindingsViolation::UncitedFindingNotFlagged);
            }
        }
        validate_anchor(&finding.anchor, violations);
    }
    if findings.uncited_findings_count != uncited {
        violations.push(AiReviewFindingsViolation::UncitedCountMismatch);
    }
}

fn validate_anchor(anchor: &DurableAnchor, violations: &mut Vec<AiReviewFindingsViolation>) {
    if anchor.anchor_id.trim().is_empty()
        || anchor.target_ref.trim().is_empty()
        || anchor.scope_ref.trim().is_empty()
    {
        violations.push(AiReviewFindingsViolation::AnchorIncomplete);
    }
    if !anchor.durable {
        violations.push(AiReviewFindingsViolation::AnchorNotDurable);
    }
    let disturbed = anchor.state.is_disturbed() || anchor.drift_detected;
    if disturbed && !anchor.rebind_disclosed {
        violations.push(AiReviewFindingsViolation::AnchorDriftUndisclosed);
    }
}

fn validate_ownership_hints(
    packet: &AiReviewFindingsPacket,
    violations: &mut Vec<AiReviewFindingsViolation>,
) {
    let hints = &packet.ownership_hints;
    if !hints.hints_are_advisory {
        violations.push(AiReviewFindingsViolation::OwnershipHintsNotAdvisory);
    }
    if !hints.no_auto_assignment {
        violations.push(AiReviewFindingsViolation::OwnershipAutoAssignmentAllowed);
    }
    for hint in &hints.hint_rows {
        if hint.hint_id.trim().is_empty()
            || hint.owner_ref.trim().is_empty()
            || hint.scope_ref.trim().is_empty()
        {
            violations.push(AiReviewFindingsViolation::OwnershipHintIncomplete);
        }
        if !hint.advisory {
            violations.push(AiReviewFindingsViolation::OwnershipHintsNotAdvisory);
        }
        if !hint.disclosed {
            violations.push(AiReviewFindingsViolation::HiddenOwnershipHint);
        }
    }
}

fn validate_workspace_integration(
    packet: &AiReviewFindingsPacket,
    violations: &mut Vec<AiReviewFindingsViolation>,
) {
    let workspace = &packet.workspace_integration;
    if workspace.workspace_id.trim().is_empty()
        || workspace.review_pack_digest_ref.trim().is_empty()
        || workspace.evidence_packet_lineage_refs.is_empty()
    {
        violations.push(AiReviewFindingsViolation::WorkspaceIntegrationIncomplete);
    }
    if !workspace.human_gate_required {
        violations.push(AiReviewFindingsViolation::HumanGateNotRequired);
    }
    if workspace.publish_state.is_published() && !workspace.human_gated {
        violations.push(AiReviewFindingsViolation::PublishedWithoutHumanGate);
    }
    let known_findings: std::collections::HashSet<&str> = packet
        .findings
        .finding_rows
        .iter()
        .map(|finding| finding.finding_id.as_str())
        .collect();
    for published in &workspace.published_finding_ids {
        if !known_findings.contains(published.as_str()) {
            violations.push(AiReviewFindingsViolation::DanglingPublishedFinding);
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &AiReviewFindingsPacket,
    violations: &mut Vec<AiReviewFindingsViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(AiReviewFindingsViolation::StableClaimNotQualified);
        }
    }
    for required in ReviewConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations.push(AiReviewFindingsViolation::ConsumerSurfaceCoverageMissing);
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
        || lower.contains('@')
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
