//! Certification of AI workflow scorecards, red-team packs, and downgrade rules
//! for each shipped M5 AI mode.
//!
//! This module locks the canonical M5 depth certification for every shipped AI
//! mode — inline edit, patch review, explain, debug, test, refactor, and branch
//! or worktree agents — into one export-safe packet. Each
//! [`AiModeCertification`] binds a mode to its claimed qualification, a workflow
//! scorecard scored against fixed thresholds, a red-team pack whose required
//! attack vectors must be handled, and a closed set of downgrade rules that
//! narrow the claim instead of hiding the mode.
//!
//! The certification is the single source of truth for whether each shipped M5
//! AI mode may keep its public claim. It reuses the qualification-class and
//! downgrade-trigger vocabularies frozen by the M5 AI workflow matrix lane
//! rather than inventing parallel terms, and it references that matrix's schema
//! and checked-in export by path so no surface may stay greener than this
//! packet. Raw prompt bodies, raw diffs, raw provider payloads, credentials,
//! exact token counts, and exact cost amounts stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ai/certify-ai-workflow-scorecards-red-team-packs-and-downgrade-rules-for-each-shipped-m5-ai-mode.schema.json`](../../../../schemas/ai/certify-ai-workflow-scorecards-red-team-packs-and-downgrade-rules-for-each-shipped-m5-ai-mode.schema.json).
//! The contract doc is
//! [`docs/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode.md`](../../../../docs/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/`](../../../../fixtures/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
    M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`AiModeCertificationPacket`].
pub const AI_MODE_CERTIFICATION_RECORD_KIND: &str =
    "certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode";

/// Schema version for AI mode certification records.
pub const AI_MODE_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const AI_MODE_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ai/certify-ai-workflow-scorecards-red-team-packs-and-downgrade-rules-for-each-shipped-m5-ai-mode.schema.json";

/// Repo-relative path of the certification contract doc.
pub const AI_MODE_CERTIFICATION_DOC_REF: &str =
    "docs/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode.md";

/// Repo-relative path of the protected fixture directory.
pub const AI_MODE_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode";

/// Repo-relative path of the checked support-export artifact.
pub const AI_MODE_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const AI_MODE_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode.md";

/// One shipped M5 AI mode certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiMode {
    /// Composer inline edit and scoped-apply quick-edit workflow.
    InlineEdit,
    /// AI-assisted patch review, finding publication, and resolution.
    PatchReview,
    /// Explain flow over code, logs, traces, runbooks, and profiles.
    Explain,
    /// Debug flow with evidence links to logs, traces, and runbooks.
    Debug,
    /// Test-generation flow with assumption review and sandbox validation.
    Test,
    /// Refactor planner with impact sets, previews, and safety classes.
    Refactor,
    /// Background branch-agent or worktree-isolated long-running task.
    BranchOrWorktreeAgent,
}

impl M5AiMode {
    /// Every shipped mode, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::InlineEdit,
        Self::PatchReview,
        Self::Explain,
        Self::Debug,
        Self::Test,
        Self::Refactor,
        Self::BranchOrWorktreeAgent,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineEdit => "inline_edit",
            Self::PatchReview => "patch_review",
            Self::Explain => "explain",
            Self::Debug => "debug",
            Self::Test => "test",
            Self::Refactor => "refactor",
            Self::BranchOrWorktreeAgent => "branch_or_worktree_agent",
        }
    }
}

/// A trust dimension scored on every mode's workflow scorecard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiScorecardDimension {
    /// Evidence packets exist, are current, and cite their sources.
    EvidenceIntegrity,
    /// Used and omitted context stay inspectable.
    ContextVisibility,
    /// The mode never claims wider scope than it qualifies for.
    ScopeHonesty,
    /// Mutating side effects require preview and human approval.
    ApprovalGating,
    /// Applied changes carry a rollback or checkpoint handle.
    RollbackSafety,
    /// Provider, host, and trust posture are disclosed.
    ProviderTrustDisclosure,
    /// Omitted or truncated context is visibly disclosed.
    OmittedContextDisclosure,
}

impl AiScorecardDimension {
    /// Every required scorecard dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EvidenceIntegrity,
        Self::ContextVisibility,
        Self::ScopeHonesty,
        Self::ApprovalGating,
        Self::RollbackSafety,
        Self::ProviderTrustDisclosure,
        Self::OmittedContextDisclosure,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceIntegrity => "evidence_integrity",
            Self::ContextVisibility => "context_visibility",
            Self::ScopeHonesty => "scope_honesty",
            Self::ApprovalGating => "approval_gating",
            Self::RollbackSafety => "rollback_safety",
            Self::ProviderTrustDisclosure => "provider_trust_disclosure",
            Self::OmittedContextDisclosure => "omitted_context_disclosure",
        }
    }
}

/// Pass/warn/fail status earned by one scorecard dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiScorecardStatus {
    /// Score meets or exceeds the threshold with margin.
    Pass,
    /// Score meets the threshold but sits at the borderline.
    Warn,
    /// Score is below the threshold.
    Fail,
}

impl AiScorecardStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Warn => "warn",
            Self::Fail => "fail",
        }
    }

    /// Whether the recorded status is consistent with its score and threshold.
    ///
    /// `Pass` and `Warn` both require the score to meet the threshold; `Fail`
    /// requires it to fall short.
    pub const fn is_consistent(self, score: u8, threshold: u8) -> bool {
        match self {
            Self::Pass | Self::Warn => score >= threshold,
            Self::Fail => score < threshold,
        }
    }
}

/// One scored row of a mode's workflow scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeScorecardRow {
    /// Trust dimension being scored.
    pub dimension: AiScorecardDimension,
    /// Achieved score, on a 0..=100 scale.
    pub score: u8,
    /// Minimum score required for this dimension.
    pub threshold: u8,
    /// Recorded pass/warn/fail status.
    pub status: AiScorecardStatus,
}

/// A red-team attack vector that the certification must account for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRedTeamVector {
    /// Untrusted instructions try to override system or repo policy.
    PromptInjection,
    /// Tainted context tries to exfiltrate secrets or private data.
    TaintedContextExfiltration,
    /// The mode tries to touch files or scope it does not qualify for.
    ScopeEscape,
    /// The agent tries to approve its own mutating tool call.
    SelfApprovedMutation,
    /// A branch agent tries to escape its isolated worktree.
    WorktreeIsolationBypass,
    /// A change tries to apply without preview and approval.
    UnreviewedApply,
    /// A credential or secret tries to leak into an export.
    CredentialLeakInExport,
    /// A stale or missing proof tries to ride a promotion through.
    StaleEvidencePromotion,
}

impl AiRedTeamVector {
    /// Every required red-team vector, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PromptInjection,
        Self::TaintedContextExfiltration,
        Self::ScopeEscape,
        Self::SelfApprovedMutation,
        Self::WorktreeIsolationBypass,
        Self::UnreviewedApply,
        Self::CredentialLeakInExport,
        Self::StaleEvidencePromotion,
    ];

    /// Vectors that apply to every AI mode regardless of apply capability.
    ///
    /// These may never be left [`AiRedTeamHandling::NotApplicable`] on a mode
    /// that carries a claimed qualification.
    pub const ALWAYS_APPLICABLE: [Self; 4] = [
        Self::PromptInjection,
        Self::TaintedContextExfiltration,
        Self::CredentialLeakInExport,
        Self::StaleEvidencePromotion,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PromptInjection => "prompt_injection",
            Self::TaintedContextExfiltration => "tainted_context_exfiltration",
            Self::ScopeEscape => "scope_escape",
            Self::SelfApprovedMutation => "self_approved_mutation",
            Self::WorktreeIsolationBypass => "worktree_isolation_bypass",
            Self::UnreviewedApply => "unreviewed_apply",
            Self::CredentialLeakInExport => "credential_leak_in_export",
            Self::StaleEvidencePromotion => "stale_evidence_promotion",
        }
    }
}

/// How a red-team vector is handled for a given mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRedTeamHandling {
    /// The attack is structurally blocked before it can take effect.
    Blocked,
    /// The attack is detected and contained with residual risk disclosed.
    Mitigated,
    /// The vector does not apply to this mode's capabilities.
    NotApplicable,
}

impl AiRedTeamHandling {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::Mitigated => "mitigated",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this disposition counts as a covered (non-open) outcome.
    pub const fn is_covered(self) -> bool {
        matches!(self, Self::Blocked | Self::Mitigated)
    }
}

/// One red-team scenario in a mode's red-team pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeRedTeamScenario {
    /// Attack vector under test.
    pub vector: AiRedTeamVector,
    /// Disposition of the vector for this mode.
    pub handling: AiRedTeamHandling,
    /// Ref to the control or contract that enforces the disposition.
    pub guard_ref: String,
}

/// One downgrade rule that narrows a mode's claim when a trigger fires.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeDowngradeRule {
    /// Trigger that fires this rule.
    pub trigger: M5AiWorkflowDowngradeTrigger,
    /// Qualification the mode narrows to when the trigger fires.
    pub narrowed_to: M5AiWorkflowQualificationClass,
    /// Whether tooling enforces this rule automatically.
    pub auto_enforced: bool,
    /// Review-safe rationale for the narrowing.
    pub rationale: String,
}

/// Full certification for one shipped M5 AI mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeCertification {
    /// Shipped mode being certified.
    pub mode: M5AiMode,
    /// Qualification class claimed for this mode.
    pub claimed_qualification: M5AiWorkflowQualificationClass,
    /// Review-safe scope summary.
    pub scope_summary: String,
    /// Workflow scorecard rows.
    pub scorecard: Vec<AiModeScorecardRow>,
    /// Red-team pack scenarios.
    pub red_team_pack: Vec<AiModeRedTeamScenario>,
    /// Downgrade rules that narrow the claim.
    pub downgrade_rules: Vec<AiModeDowngradeRule>,
    /// Required evidence packet refs backing the claim.
    pub evidence_packet_refs: Vec<String>,
}

impl AiModeCertification {
    /// Qualification this mode narrows to when `trigger` fires.
    ///
    /// Returns the claimed qualification unchanged when no rule matches the
    /// trigger; this is the deterministic downgrade automation consumers and
    /// release tooling project instead of re-deriving narrowing locally.
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

    /// Whether this mode carries a publicly claimed qualification.
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
}

/// Proof freshness block for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows claimed modes.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`AiModeCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiModeCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-mode certifications.
    pub mode_certifications: Vec<AiModeCertification>,
    /// Proof freshness block.
    pub proof_freshness: AiModeCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI mode certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiModeCertificationPacket {
    /// Record kind; must equal [`AI_MODE_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AI_MODE_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Per-mode certifications.
    pub mode_certifications: Vec<AiModeCertification>,
    /// Proof freshness block.
    pub proof_freshness: AiModeCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiModeCertificationPacket {
    /// Builds an AI mode certification packet from stable-lane input.
    pub fn new(input: AiModeCertificationPacketInput) -> Self {
        Self {
            record_kind: AI_MODE_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: AI_MODE_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            mode_certifications: input.mode_certifications,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the AI mode certification invariants.
    pub fn validate(&self) -> Vec<AiModeCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != AI_MODE_CERTIFICATION_RECORD_KIND {
            violations.push(AiModeCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != AI_MODE_CERTIFICATION_SCHEMA_VERSION {
            violations.push(AiModeCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiModeCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_modes_present(self, &mut violations);
        for cert in &self.mode_certifications {
            validate_mode_certification(cert, &mut violations);
        }
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai mode certification packet serializes"),
        ) {
            violations.push(AiModeCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Count of modes whose claimed qualification is Stable.
    pub fn stable_mode_count(&self) -> usize {
        self.mode_certifications
            .iter()
            .filter(|cert| cert.claimed_qualification.is_stable())
            .count()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai mode certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 AI Mode Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Modes: {} ({} stable)\n",
            self.mode_certifications.len(),
            self.stable_mode_count()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Modes\n\n");
        for cert in &self.mode_certifications {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                cert.mode.as_str(),
                cert.claimed_qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", cert.scope_summary));
            out.push_str(&format!(
                "  - Scorecard: {} dimensions\n",
                cert.scorecard.len()
            ));
            out.push_str(&format!(
                "  - Red-team: {} vectors\n",
                cert.red_team_pack.len()
            ));
            out.push_str(&format!(
                "  - Downgrade rules: {}\n",
                cert.downgrade_rules.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in AI mode certification export.
#[derive(Debug)]
pub enum AiModeCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiModeCertificationViolation>),
}

impl fmt::Display for AiModeCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "ai mode certification export parse failed: {error}"
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
                    "ai mode certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiModeCertificationArtifactError {}

/// Validation failures emitted by [`AiModeCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiModeCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required mode is missing from the certification.
    RequiredModeMissing,
    /// A mode appears more than once.
    DuplicateMode,
    /// A mode certification row is incomplete.
    ModeRowIncomplete,
    /// A scorecard does not cover every required dimension.
    ScorecardDimensionMissing,
    /// A scorecard score or threshold is out of the 0..=100 range.
    ScorecardScoreOutOfRange,
    /// A scorecard status is inconsistent with its score and threshold.
    ScorecardStatusInconsistent,
    /// A Stable-claimed mode has a scorecard dimension that is not passing.
    StableModeScorecardNotPassing,
    /// A Beta-claimed mode has a failing scorecard dimension.
    BetaModeScorecardFailing,
    /// A red-team pack does not cover every required vector.
    RedTeamVectorMissing,
    /// A red-team scenario is missing its guard ref.
    RedTeamGuardMissing,
    /// An always-applicable red-team vector is left uncovered on a claimed mode.
    RedTeamCriticalVectorUncovered,
    /// A claimed mode is missing required evidence packet refs.
    ClaimedModeMissingEvidence,
    /// A mode has no downgrade rules.
    DowngradeRulesMissing,
    /// A mode's downgrade rules omit the proof-stale trigger.
    DowngradeRuleMissingProofStale,
    /// A downgrade rule does not narrow below the claimed qualification.
    DowngradeRuleNotNarrowing,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiModeCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredModeMissing => "required_mode_missing",
            Self::DuplicateMode => "duplicate_mode",
            Self::ModeRowIncomplete => "mode_row_incomplete",
            Self::ScorecardDimensionMissing => "scorecard_dimension_missing",
            Self::ScorecardScoreOutOfRange => "scorecard_score_out_of_range",
            Self::ScorecardStatusInconsistent => "scorecard_status_inconsistent",
            Self::StableModeScorecardNotPassing => "stable_mode_scorecard_not_passing",
            Self::BetaModeScorecardFailing => "beta_mode_scorecard_failing",
            Self::RedTeamVectorMissing => "red_team_vector_missing",
            Self::RedTeamGuardMissing => "red_team_guard_missing",
            Self::RedTeamCriticalVectorUncovered => "red_team_critical_vector_uncovered",
            Self::ClaimedModeMissingEvidence => "claimed_mode_missing_evidence",
            Self::DowngradeRulesMissing => "downgrade_rules_missing",
            Self::DowngradeRuleMissingProofStale => "downgrade_rule_missing_proof_stale",
            Self::DowngradeRuleNotNarrowing => "downgrade_rule_not_narrowing",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in AI mode certification export.
pub fn current_ai_mode_certification_export(
) -> Result<AiModeCertificationPacket, AiModeCertificationArtifactError> {
    let packet: AiModeCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/certify_ai_workflow_scorecards_red_team_packs_and_downgrade_rules_for_each_shipped_m5_ai_mode/support_export.json"
    )))
    .map_err(AiModeCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiModeCertificationArtifactError::Validation(violations))
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
    packet: &AiModeCertificationPacket,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        AI_MODE_CERTIFICATION_SCHEMA_REF,
        AI_MODE_CERTIFICATION_DOC_REF,
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
        M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AiModeCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_modes_present(
    packet: &AiModeCertificationPacket,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    let mut seen: BTreeSet<M5AiMode> = BTreeSet::new();
    for cert in &packet.mode_certifications {
        if !seen.insert(cert.mode) {
            violations.push(AiModeCertificationViolation::DuplicateMode);
        }
    }
    for required in M5AiMode::ALL {
        if !seen.contains(&required) {
            violations.push(AiModeCertificationViolation::RequiredModeMissing);
            return;
        }
    }
}

fn validate_mode_certification(
    cert: &AiModeCertification,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    if cert.scope_summary.trim().is_empty() {
        violations.push(AiModeCertificationViolation::ModeRowIncomplete);
    }

    validate_scorecard(cert, violations);
    validate_red_team_pack(cert, violations);
    validate_downgrade_rules(cert, violations);

    if cert.is_claimed() && cert.evidence_packet_refs.is_empty() {
        violations.push(AiModeCertificationViolation::ClaimedModeMissingEvidence);
    }
}

fn validate_scorecard(
    cert: &AiModeCertification,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    let covered: BTreeSet<AiScorecardDimension> =
        cert.scorecard.iter().map(|row| row.dimension).collect();
    for required in AiScorecardDimension::ALL {
        if !covered.contains(&required) {
            violations.push(AiModeCertificationViolation::ScorecardDimensionMissing);
            break;
        }
    }

    let is_stable = cert.claimed_qualification.is_stable();
    let is_beta = matches!(
        cert.claimed_qualification,
        M5AiWorkflowQualificationClass::Beta
    );
    for row in &cert.scorecard {
        if row.score > 100 || row.threshold > 100 {
            violations.push(AiModeCertificationViolation::ScorecardScoreOutOfRange);
        }
        if !row.status.is_consistent(row.score, row.threshold) {
            violations.push(AiModeCertificationViolation::ScorecardStatusInconsistent);
        }
        if is_stable && row.status != AiScorecardStatus::Pass {
            violations.push(AiModeCertificationViolation::StableModeScorecardNotPassing);
        }
        if is_beta && row.status == AiScorecardStatus::Fail {
            violations.push(AiModeCertificationViolation::BetaModeScorecardFailing);
        }
    }
}

fn validate_red_team_pack(
    cert: &AiModeCertification,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    let mut covered: BTreeSet<AiRedTeamVector> = BTreeSet::new();
    for scenario in &cert.red_team_pack {
        covered.insert(scenario.vector);
        if scenario.guard_ref.trim().is_empty() {
            violations.push(AiModeCertificationViolation::RedTeamGuardMissing);
        }
    }
    for required in AiRedTeamVector::ALL {
        if !covered.contains(&required) {
            violations.push(AiModeCertificationViolation::RedTeamVectorMissing);
            break;
        }
    }

    if cert.is_claimed() {
        for scenario in &cert.red_team_pack {
            if AiRedTeamVector::ALWAYS_APPLICABLE.contains(&scenario.vector)
                && !scenario.handling.is_covered()
            {
                violations.push(AiModeCertificationViolation::RedTeamCriticalVectorUncovered);
                break;
            }
        }
    }
}

fn validate_downgrade_rules(
    cert: &AiModeCertification,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    if cert.downgrade_rules.is_empty() {
        violations.push(AiModeCertificationViolation::DowngradeRulesMissing);
        return;
    }

    if !cert
        .downgrade_rules
        .iter()
        .any(|rule| rule.trigger == M5AiWorkflowDowngradeTrigger::ProofStale)
    {
        violations.push(AiModeCertificationViolation::DowngradeRuleMissingProofStale);
    }

    let claimed_rank = qualification_rank(cert.claimed_qualification);
    for rule in &cert.downgrade_rules {
        if qualification_rank(rule.narrowed_to) >= claimed_rank {
            violations.push(AiModeCertificationViolation::DowngradeRuleNotNarrowing);
            break;
        }
    }
}

fn validate_proof_freshness(
    packet: &AiModeCertificationPacket,
    violations: &mut Vec<AiModeCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AiModeCertificationViolation::ProofFreshnessIncomplete);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
