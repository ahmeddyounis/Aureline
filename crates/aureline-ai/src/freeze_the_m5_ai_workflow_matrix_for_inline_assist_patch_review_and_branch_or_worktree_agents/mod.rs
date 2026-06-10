//! Frozen M5 AI workflow matrix for inline assist, patch review, and branch or
//! worktree agents.
//!
//! This module locks the canonical M5 depth qualification for three AI workflow
//! lanes — inline assist, patch review, and branch or worktree agents — into one
//! export-safe packet. Each [`M5AiWorkflowMatrixLaneRow`] binds a lane to its
//! qualification class, required evidence packet refs, downgrade triggers,
//! rollback posture, source contracts, and consumer surface parity.
//!
//! The matrix is the single source of truth for whether these lanes may ship as
//! Stable, Beta, Preview, or must narrow further. It references upstream M4
//! qualification packets and M5 control-plane artifacts by id rather than
//! embedding their content. Raw prompt bodies, raw diffs, raw provider payloads,
//! credentials, exact token counts, and exact cost amounts stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/freeze-the-m5-ai-workflow-matrix-for-inline-assist-patch-review-and-branch-or-worktree-agents.schema.json`](../../../../schemas/ai/freeze-the-m5-ai-workflow-matrix-for-inline-assist-patch-review-and-branch-or-worktree-agents.schema.json).
//! The contract doc is
//! [`docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md`](../../../../docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/`](../../../../fixtures/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiWorkflowMatrixPacket`].
pub const M5_AI_WORKFLOW_MATRIX_RECORD_KIND: &str =
    "freeze_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents";

/// Schema version for M5 AI workflow matrix records.
pub const M5_AI_WORKFLOW_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_AI_WORKFLOW_MATRIX_SCHEMA_REF: &str =
    "schemas/ai/freeze-the-m5-ai-workflow-matrix-for-inline-assist-patch-review-and-branch-or-worktree-agents.schema.json";

/// Repo-relative path of the M5 AI workflow matrix contract doc.
pub const M5_AI_WORKFLOW_MATRIX_DOC_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the frozen prompt-composer contract (inline assist).
pub const M5_AI_WORKFLOW_MATRIX_INLINE_ASSIST_CONTRACT_REF: &str =
    "docs/ai/prompt_composer_contract.md";

/// Repo-relative path of the frozen review-assist publish contract (patch review).
pub const M5_AI_WORKFLOW_MATRIX_PATCH_REVIEW_CONTRACT_REF: &str =
    "docs/ai/review_assist_publish_contract.md";

/// Repo-relative path of the protected multi-file patch review sequence.
pub const M5_AI_WORKFLOW_MATRIX_PATCH_SEQUENCE_REF: &str =
    "artifacts/ai/multifile_patch_review_sequence.md";

/// Repo-relative path of the frozen branch-agent lifecycle contract.
pub const M5_AI_WORKFLOW_MATRIX_BRANCH_AGENT_CONTRACT_REF: &str =
    "docs/ai/background_branch_agent_lifecycle.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_WORKFLOW_MATRIX_FIXTURE_DIR: &str =
    "fixtures/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_AI_WORKFLOW_MATRIX_SUMMARY_REF: &str =
    "artifacts/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// One of the three M5 AI workflow lanes governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowLane {
    /// Inline composer assist and quick-edit workflows.
    InlineAssist,
    /// AI-assisted patch review, finding publication, and resolution.
    PatchReview,
    /// Background branch-agent or worktree-isolated long-running tasks.
    BranchOrWorktreeAgent,
}

impl M5AiWorkflowLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::InlineAssist,
        Self::PatchReview,
        Self::BranchOrWorktreeAgent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineAssist => "inline_assist",
            Self::PatchReview => "patch_review",
            Self::BranchOrWorktreeAgent => "branch_or_worktree_agent",
        }
    }
}

/// Qualification class for an M5 AI workflow lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending upstream resolution.
    Held,
}

impl M5AiWorkflowQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence requirement level for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this lane's current qualification.
    NotApplicable,
}

impl M5AiWorkflowEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Required provider or model is unavailable.
    ProviderUnavailable,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond qualified boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5AiWorkflowDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderUnavailable => "provider_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowRollbackPosture {
    /// Fully reversible without data loss.
    FullyReversible,
    /// Reversible via captured checkpoints.
    CheckpointReversible,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for read-only or non-mutating lanes.
    NotApplicable,
}

impl M5AiWorkflowRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversible => "fully_reversible",
            Self::CheckpointReversible => "checkpoint_reversible",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiWorkflowConsumerSurface {
    /// Desktop composer or inline assist UI.
    DesktopComposer,
    /// Desktop review workspace.
    DesktopReviewWorkspace,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Browser or companion follow-up.
    BrowserCompanion,
    /// Support/export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl M5AiWorkflowConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopComposer,
        Self::DesktopReviewWorkspace,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopComposer => "desktop_composer",
            Self::DesktopReviewWorkspace => "desktop_review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::BrowserCompanion => "browser_companion",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// One row in the M5 AI workflow matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiWorkflowMatrixLaneRow {
    /// Workflow lane.
    pub lane: M5AiWorkflowLane,
    /// Qualification class earned by this lane.
    pub qualification: M5AiWorkflowQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5AiWorkflowEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5AiWorkflowDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5AiWorkflowRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<M5AiWorkflowConsumerSurface>,
}

/// Security and policy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiWorkflowMatrixSecurityReview {
    /// Mutating tools cannot be self-approved by the agent.
    pub no_self_approved_mutating_tools: bool,
    /// Worktree isolation cannot be bypassed.
    pub no_worktree_isolation_bypass: bool,
    /// Preview and approval are required before any apply path.
    pub preview_approval_required_before_apply: bool,
    /// Evidence packets cite their source contracts.
    pub evidence_packets_cite_source_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale proof automatically blocks promotion.
    pub stale_proof_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiWorkflowMatrixConsumerProjection {
    /// Desktop composer shows qualification truth.
    pub desktop_composer_shows_qualification: bool,
    /// Desktop review workspace shows qualification truth.
    pub desktop_review_shows_qualification: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Browser / companion shows qualification truth.
    pub browser_companion_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_qualification: bool,
    /// Preview / Labs lanes are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_lanes: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiWorkflowMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5AiWorkflowMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiWorkflowMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5AiWorkflowMatrixLaneRow>,
    /// Security review block.
    pub security_review: M5AiWorkflowMatrixSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiWorkflowMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiWorkflowMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 AI workflow matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiWorkflowMatrixPacket {
    /// Record kind; must equal [`M5_AI_WORKFLOW_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_WORKFLOW_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5AiWorkflowMatrixLaneRow>,
    /// Security review block.
    pub security_review: M5AiWorkflowMatrixSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiWorkflowMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiWorkflowMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiWorkflowMatrixPacket {
    /// Builds an M5 AI workflow matrix packet from stable-lane input.
    pub fn new(input: M5AiWorkflowMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_AI_WORKFLOW_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_AI_WORKFLOW_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            lane_rows: input.lane_rows,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 AI workflow matrix invariants.
    pub fn validate(&self) -> Vec<M5AiWorkflowMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AI_WORKFLOW_MATRIX_RECORD_KIND {
            violations.push(M5AiWorkflowMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_WORKFLOW_MATRIX_SCHEMA_VERSION {
            violations.push(M5AiWorkflowMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiWorkflowMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 ai workflow matrix packet serializes"),
        ) {
            violations.push(M5AiWorkflowMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 ai workflow matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 AI Workflow Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.lane.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Evidence: {} ({} refs)\n",
                row.evidence_requirement.as_str(),
                row.required_evidence_packet_refs.len()
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 AI workflow matrix export.
#[derive(Debug)]
pub enum M5AiWorkflowMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiWorkflowMatrixViolation>),
}

impl fmt::Display for M5AiWorkflowMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai workflow matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 ai workflow matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiWorkflowMatrixArtifactError {}

/// Validation failures emitted by [`M5AiWorkflowMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiWorkflowMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the matrix.
    RequiredLaneMissing,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A lane claiming Stable is missing required evidence packet refs.
    StableLaneMissingEvidence,
    /// A lane has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5AiWorkflowMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::StableLaneMissingEvidence => "stable_lane_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 AI workflow matrix export.
pub fn current_stable_m5_ai_workflow_matrix_export(
) -> Result<M5AiWorkflowMatrixPacket, M5AiWorkflowMatrixArtifactError> {
    let packet: M5AiWorkflowMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents/support_export.json"
    )))
    .map_err(M5AiWorkflowMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiWorkflowMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AiWorkflowMatrixPacket,
    violations: &mut Vec<M5AiWorkflowMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_DOC_REF,
        M5_AI_WORKFLOW_MATRIX_INLINE_ASSIST_CONTRACT_REF,
        M5_AI_WORKFLOW_MATRIX_PATCH_REVIEW_CONTRACT_REF,
        M5_AI_WORKFLOW_MATRIX_PATCH_SEQUENCE_REF,
        M5_AI_WORKFLOW_MATRIX_BRANCH_AGENT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiWorkflowMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &M5AiWorkflowMatrixPacket,
    violations: &mut Vec<M5AiWorkflowMatrixViolation>,
) {
    let present: BTreeSet<M5AiWorkflowLane> = packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in M5AiWorkflowLane::ALL {
        if !present.contains(&required) {
            violations.push(M5AiWorkflowMatrixViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5AiWorkflowMatrixViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5AiWorkflowMatrixViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiWorkflowMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiWorkflowMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_security_review(
    packet: &M5AiWorkflowMatrixPacket,
    violations: &mut Vec<M5AiWorkflowMatrixViolation>,
) {
    for (field, ok) in [
        (
            "no_self_approved_mutating_tools",
            packet.security_review.no_self_approved_mutating_tools,
        ),
        (
            "no_worktree_isolation_bypass",
            packet.security_review.no_worktree_isolation_bypass,
        ),
        (
            "preview_approval_required_before_apply",
            packet
                .security_review
                .preview_approval_required_before_apply,
        ),
        (
            "evidence_packets_cite_source_contracts",
            packet
                .security_review
                .evidence_packets_cite_source_contracts,
        ),
        (
            "downgrade_narrows_instead_of_hides",
            packet.security_review.downgrade_narrows_instead_of_hides,
        ),
        (
            "stale_proof_blocks_promotion",
            packet.security_review.stale_proof_blocks_promotion,
        ),
    ] {
        if !ok {
            let _ = field;
            violations.push(M5AiWorkflowMatrixViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiWorkflowMatrixPacket,
    violations: &mut Vec<M5AiWorkflowMatrixViolation>,
) {
    for (field, ok) in [
        (
            "desktop_composer_shows_qualification",
            packet
                .consumer_projection
                .desktop_composer_shows_qualification,
        ),
        (
            "desktop_review_shows_qualification",
            packet
                .consumer_projection
                .desktop_review_shows_qualification,
        ),
        (
            "cli_headless_shows_qualification",
            packet.consumer_projection.cli_headless_shows_qualification,
        ),
        (
            "browser_companion_shows_qualification",
            packet
                .consumer_projection
                .browser_companion_shows_qualification,
        ),
        (
            "support_export_shows_qualification",
            packet
                .consumer_projection
                .support_export_shows_qualification,
        ),
        (
            "diagnostics_shows_qualification",
            packet.consumer_projection.diagnostics_shows_qualification,
        ),
        (
            "preview_labs_label_for_unqualified_lanes",
            packet
                .consumer_projection
                .preview_labs_label_for_unqualified_lanes,
        ),
    ] {
        if !ok {
            let _ = field;
            violations.push(M5AiWorkflowMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiWorkflowMatrixPacket,
    violations: &mut Vec<M5AiWorkflowMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiWorkflowMatrixViolation::ProofFreshnessIncomplete);
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
                || lower.contains("token")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
