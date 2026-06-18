//! AI-surface package-mutation proposal review: the AI composer's propose-only
//! view of a governed package mutation.
//!
//! This module owns the AI-side packet that keeps AI-generated package-mutation
//! proposals honest. The AI surface is a *proposer*, never an executor: every
//! [`AiMutationProposalRow`] is preview-first, routes through the governed
//! review contract, carries no hidden scripting, and binds by reference to the
//! `aureline-deps` `automation_governance` packet, the frozen package-state
//! matrix, and the reviewed-mutation contract. It never claims write authority
//! to mutate a manifest or lockfile.
//!
//! Where the `aureline-deps` `automation_governance` lane owns the canonical,
//! cross-surface governed proposal (AI, recipe, CLI, and desktop baseline), this
//! module is the AI composer's surface contract over that lane: it proves the AI
//! proposal reuses the same manifest/lockfile impact preview, requests the same
//! validation tasks, mirrors the same safe-fallback decision, and shows the same
//! result class and rollback handle the governed contract recorded — so AI
//! convenience never becomes a bypass lane around lockfile-safe review.
//!
//! The packet is metadata-oriented. It carries ids, refs, closed-vocabulary
//! tokens, redaction-aware preview text, and short labels. Raw diffs, raw
//! provider payloads, provider URLs, credentials, prompt text, and secret values
//! stay outside this boundary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiPackageMutationReviewPacket`].
pub const AI_PACKAGE_MUTATION_REVIEW_RECORD_KIND: &str = "ai_package_mutation_review";

/// Schema version for AI package-mutation-review records.
pub const AI_PACKAGE_MUTATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI package-mutation-review boundary schema.
pub const AI_PACKAGE_MUTATION_REVIEW_SCHEMA_REF: &str =
    "schemas/ai/package-mutation-review.schema.json";

/// Repo-relative path of the AI package-mutation-review contract doc.
pub const AI_PACKAGE_MUTATION_REVIEW_AI_DOC_REF: &str = "docs/ai/m5/package-mutation-review.md";

/// Repo-relative path of the checked AI package-mutation-review support export.
pub const AI_PACKAGE_MUTATION_REVIEW_ARTIFACT_REF: &str =
    "artifacts/ai/m5/package_mutation_review/support_export.json";

/// Repo-relative path of the protected AI package-mutation-review fixture dir.
pub const AI_PACKAGE_MUTATION_REVIEW_FIXTURE_DIR: &str = "fixtures/ai/m5/package-mutation-review";

/// Repo-relative path of the governed cross-surface mutation contract this AI
/// surface binds to.
pub const AI_PACKAGE_MUTATION_REVIEW_GOVERNANCE_CONTRACT_REF: &str =
    "artifacts/deps/m5/automation-governance.json";

/// The AI mutation intent behind a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiMutationIntent {
    /// Add a new direct dependency.
    AddDependency,
    /// Upgrade an existing dependency.
    UpgradeDependency,
    /// Remove a dependency.
    RemoveDependency,
    /// Re-resolve / relock the dependency set.
    RelockDependencies,
}

impl AiMutationIntent {
    /// Every intent, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AddDependency,
        Self::UpgradeDependency,
        Self::RemoveDependency,
        Self::RelockDependencies,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AddDependency => "add_dependency",
            Self::UpgradeDependency => "upgrade_dependency",
            Self::RemoveDependency => "remove_dependency",
            Self::RelockDependencies => "relock_dependencies",
        }
    }
}

/// The write authority the AI surface carries over a proposal.
///
/// Neither variant can execute a mutation: the AI surface either stages a
/// proposal for governed review or only inspects it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiWriteAuthority {
    /// May stage a proposal for governed review, but not apply it.
    ProposeOnly,
    /// Inspect-only; carries no write or stage authority.
    InspectOnly,
}

impl AiWriteAuthority {
    /// Every write authority, in declaration order.
    pub const ALL: [Self; 2] = [Self::ProposeOnly, Self::InspectOnly];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProposeOnly => "propose_only",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// Whether this authority can execute a mutation. The AI surface never can.
    pub const fn can_execute(self) -> bool {
        false
    }
}

/// The safe-fallback class an AI proposal mirrors from the governed contract.
///
/// These tokens match the `aureline-deps` `automation_governance`
/// `ExecutionDecision` vocabulary so the AI surface reflects the same decision
/// the governed contract recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeFallbackClass {
    /// All capabilities held; the governed mutation may commit after review.
    ProceedAfterReview,
    /// Narrowed to inspect-only.
    NarrowToInspectOnly,
    /// Narrowed to a redaction-safe export.
    NarrowToExportOnly,
    /// Handed off to the provider browser flow.
    HandoffToBrowser,
    /// Handed off to a CLI/headless flow.
    HandoffToCli,
    /// No safe execution path; blocked.
    BlockedNoSafePath,
}

impl SafeFallbackClass {
    /// Every safe-fallback class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProceedAfterReview,
        Self::NarrowToInspectOnly,
        Self::NarrowToExportOnly,
        Self::HandoffToBrowser,
        Self::HandoffToCli,
        Self::BlockedNoSafePath,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedAfterReview => "proceed_after_review",
            Self::NarrowToInspectOnly => "narrow_to_inspect_only",
            Self::NarrowToExportOnly => "narrow_to_export_only",
            Self::HandoffToBrowser => "handoff_to_browser",
            Self::HandoffToCli => "handoff_to_cli",
            Self::BlockedNoSafePath => "blocked_no_safe_path",
        }
    }

    /// Whether this fallback permits the governed mutation to commit.
    pub const fn permits_commit(self) -> bool {
        matches!(self, Self::ProceedAfterReview)
    }
}

/// The result class an AI proposal mirrors from the governed contract.
///
/// These tokens match the `aureline-deps` `automation_governance`
/// `GovernedResultClass` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiResultClass {
    /// Preview produced; awaiting review.
    PreviewPending,
    /// Reviewed and ready to commit.
    ReviewedReady,
    /// Blocked: no safe execution path.
    BlockedUnsafe,
    /// Narrowed to inspect-only.
    NarrowedInspectOnly,
    /// Handed off to a browser or CLI flow.
    HandedOff,
    /// Committed after governed review.
    CommittedReviewed,
    /// Committed then rolled back.
    RolledBack,
}

impl AiResultClass {
    /// Every result class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::PreviewPending,
        Self::ReviewedReady,
        Self::BlockedUnsafe,
        Self::NarrowedInspectOnly,
        Self::HandedOff,
        Self::CommittedReviewed,
        Self::RolledBack,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPending => "preview_pending",
            Self::ReviewedReady => "reviewed_ready",
            Self::BlockedUnsafe => "blocked_unsafe",
            Self::NarrowedInspectOnly => "narrowed_inspect_only",
            Self::HandedOff => "handed_off",
            Self::CommittedReviewed => "committed_reviewed",
            Self::RolledBack => "rolled_back",
        }
    }

    /// Whether this result represents a committed (post-write) state.
    pub const fn is_committed(self) -> bool {
        matches!(self, Self::CommittedReviewed | Self::RolledBack)
    }
}

/// A validation task an AI proposal requests, mirroring the governed
/// `ValidationTaskKind` vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiValidationKind {
    /// Build the affected packages.
    Build,
    /// Run affected tests.
    Test,
    /// Run lint checks.
    Lint,
    /// Run type checks.
    Typecheck,
    /// Run a security/advisory audit.
    SecurityAudit,
    /// Run a license-review check.
    LicenseReview,
    /// Verify lockfile consistency.
    LockfileVerify,
}

impl AiValidationKind {
    /// Every validation kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Build,
        Self::Test,
        Self::Lint,
        Self::Typecheck,
        Self::SecurityAudit,
        Self::LicenseReview,
        Self::LockfileVerify,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Test => "test",
            Self::Lint => "lint",
            Self::Typecheck => "typecheck",
            Self::SecurityAudit => "security_audit",
            Self::LicenseReview => "license_review",
            Self::LockfileVerify => "lockfile_verify",
        }
    }
}

/// The ecosystem an AI proposal targets, mirroring the governed `EcosystemKind`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiEcosystem {
    /// Cargo / crates.io.
    Cargo,
    /// Node with pnpm.
    NodePnpm,
    /// Python with pip.
    PythonPip,
    /// Any other qualified ecosystem.
    Other,
}

impl AiEcosystem {
    /// Every ecosystem, in declaration order.
    pub const ALL: [Self; 4] = [Self::Cargo, Self::NodePnpm, Self::PythonPip, Self::Other];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cargo => "cargo",
            Self::NodePnpm => "node_pnpm",
            Self::PythonPip => "python_pip",
            Self::Other => "other",
        }
    }
}

/// Whether a safe-fallback class and a result class are coherent.
const fn fallback_allows_result(fallback: SafeFallbackClass, result: AiResultClass) -> bool {
    match result {
        AiResultClass::PreviewPending
        | AiResultClass::ReviewedReady
        | AiResultClass::CommittedReviewed
        | AiResultClass::RolledBack => {
            matches!(fallback, SafeFallbackClass::ProceedAfterReview)
        }
        AiResultClass::BlockedUnsafe => matches!(fallback, SafeFallbackClass::BlockedNoSafePath),
        AiResultClass::NarrowedInspectOnly => {
            matches!(fallback, SafeFallbackClass::NarrowToInspectOnly)
        }
        AiResultClass::HandedOff => matches!(
            fallback,
            SafeFallbackClass::HandoffToBrowser
                | SafeFallbackClass::HandoffToCli
                | SafeFallbackClass::NarrowToExportOnly
        ),
    }
}

/// One AI-surface package-mutation proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiMutationProposalRow {
    /// Stable proposal id within this packet.
    pub proposal_id: String,
    /// The governed-contract proposal this AI proposal binds to.
    pub governed_proposal_ref: String,
    /// AI mutation intent.
    pub intent: AiMutationIntent,
    /// Ecosystem the mutation targets.
    pub ecosystem: AiEcosystem,
    /// Human-readable label.
    pub label: String,
    /// AI-surface write authority (never executing).
    pub write_authority: AiWriteAuthority,
    /// Whether the proposal is preview-first (must be `true`).
    pub preview_first: bool,
    /// Whether the proposal routes through governed review (must be `true`).
    pub routes_through_governed_review: bool,
    /// Whether the proposal avoids hidden scripting (must be `true`).
    pub no_hidden_scripting: bool,
    /// Validation tasks the AI proposal requests.
    #[serde(default)]
    pub requested_validation: Vec<AiValidationKind>,
    /// Safe-fallback class mirrored from the governed contract.
    pub safe_fallback: SafeFallbackClass,
    /// Result class mirrored from the governed contract.
    pub result_class: AiResultClass,
    /// Cross-surface rollback handle ref; never a raw URL.
    pub rollback_handle_ref: String,
    /// Redacted manifest path; never a raw URL.
    pub redacted_manifest_path: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl AiMutationProposalRow {
    /// Whether the AI surface is propose-only/inspect-only and never executes.
    pub const fn is_propose_only(&self) -> bool {
        !self.write_authority.can_execute()
    }

    /// Whether the proposal asserts the governed posture: preview-first, routed
    /// through governed review, and free of hidden scripting.
    pub const fn asserts_governed(&self) -> bool {
        self.preview_first && self.routes_through_governed_review && self.no_hidden_scripting
    }

    /// Whether the proposal's result is coherent with its safe-fallback class.
    pub const fn result_is_coherent(&self) -> bool {
        fallback_allows_result(self.safe_fallback, self.result_class)
    }
}

/// Summary counts derived from the proposals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiPackageMutationReviewSummary {
    /// Total proposals.
    pub total_proposals: usize,
    /// Add-dependency proposals.
    pub add_proposals: usize,
    /// Upgrade-dependency proposals.
    pub upgrade_proposals: usize,
    /// Remove-dependency proposals.
    pub remove_proposals: usize,
    /// Relock proposals.
    pub relock_proposals: usize,
    /// Propose-only proposals.
    pub propose_only_proposals: usize,
    /// Inspect-only proposals.
    pub inspect_only_proposals: usize,
    /// Proposals whose governed fallback permits commit.
    pub proceed_proposals: usize,
    /// Proposals narrowed or handed off (not proceed, not blocked).
    pub narrowed_or_handoff_proposals: usize,
    /// Proposals blocked with no safe path.
    pub blocked_proposals: usize,
    /// Proposals whose result is a committed (post-write) state.
    pub committed_proposals: usize,
}

/// One row of the redaction-safe export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPackageMutationReviewExportRow {
    /// Proposal id.
    pub row_id: String,
    /// Governed-contract proposal ref.
    pub governed_proposal_ref: String,
    /// Intent token.
    pub intent: String,
    /// Ecosystem token.
    pub ecosystem: String,
    /// Label.
    pub label: String,
    /// Safe-fallback token.
    pub safe_fallback: String,
    /// Result-class token.
    pub result_class: String,
    /// Human-readable summary.
    pub summary: String,
}

/// Redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiPackageMutationReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<AiPackageMutationReviewExportRow>,
    /// Whether every proposal is propose-only / inspect-only.
    pub all_propose_only: bool,
    /// Whether every proposal asserts the governed posture.
    pub all_governed: bool,
}

/// Typed AI package-mutation-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AiPackageMutationReviewPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Governed cross-surface mutation packet this AI surface binds to.
    pub references_governance_packet_id: String,
    /// Frozen package-state matrix this packet binds to.
    pub references_matrix_id: String,
    /// Reviewed-mutation contract this packet reuses.
    pub references_reviewed_flows_id: String,
    /// Closed intent vocabulary.
    pub intents: Vec<AiMutationIntent>,
    /// Closed write-authority vocabulary.
    pub write_authorities: Vec<AiWriteAuthority>,
    /// Closed safe-fallback vocabulary.
    pub safe_fallback_classes: Vec<SafeFallbackClass>,
    /// Closed result-class vocabulary.
    pub result_classes: Vec<AiResultClass>,
    /// Closed validation-kind vocabulary.
    pub validation_kinds: Vec<AiValidationKind>,
    /// AI-surface proposals.
    #[serde(default)]
    pub proposals: Vec<AiMutationProposalRow>,
    /// Summary counts.
    pub summary: AiPackageMutationReviewSummary,
}

impl AiPackageMutationReviewPacket {
    /// Returns the proposal for `proposal_id`.
    pub fn proposal(&self, proposal_id: &str) -> Option<&AiMutationProposalRow> {
        self.proposals
            .iter()
            .find(|row| row.proposal_id == proposal_id)
    }

    /// Whether every proposal is propose-only / inspect-only and never executes.
    pub fn all_propose_only(&self) -> bool {
        self.proposals.iter().all(|p| p.is_propose_only())
    }

    /// Whether every proposal asserts the governed posture.
    pub fn all_governed(&self) -> bool {
        self.proposals.iter().all(|p| p.asserts_governed())
    }

    /// Recomputes the summary block from the proposals.
    pub fn computed_summary(&self) -> AiPackageMutationReviewSummary {
        let intent_count =
            |intent: AiMutationIntent| self.proposals.iter().filter(|p| p.intent == intent).count();
        AiPackageMutationReviewSummary {
            total_proposals: self.proposals.len(),
            add_proposals: intent_count(AiMutationIntent::AddDependency),
            upgrade_proposals: intent_count(AiMutationIntent::UpgradeDependency),
            remove_proposals: intent_count(AiMutationIntent::RemoveDependency),
            relock_proposals: intent_count(AiMutationIntent::RelockDependencies),
            propose_only_proposals: self
                .proposals
                .iter()
                .filter(|p| p.write_authority == AiWriteAuthority::ProposeOnly)
                .count(),
            inspect_only_proposals: self
                .proposals
                .iter()
                .filter(|p| p.write_authority == AiWriteAuthority::InspectOnly)
                .count(),
            proceed_proposals: self
                .proposals
                .iter()
                .filter(|p| p.safe_fallback.permits_commit())
                .count(),
            narrowed_or_handoff_proposals: self
                .proposals
                .iter()
                .filter(|p| {
                    !p.safe_fallback.permits_commit()
                        && p.safe_fallback != SafeFallbackClass::BlockedNoSafePath
                })
                .count(),
            blocked_proposals: self
                .proposals
                .iter()
                .filter(|p| p.safe_fallback == SafeFallbackClass::BlockedNoSafePath)
                .count(),
            committed_proposals: self
                .proposals
                .iter()
                .filter(|p| p.result_class.is_committed())
                .count(),
        }
    }

    /// Produces a redaction-safe export projection.
    pub fn export_projection(&self) -> AiPackageMutationReviewExportProjection {
        let rows = self
            .proposals
            .iter()
            .map(|p| AiPackageMutationReviewExportRow {
                row_id: p.proposal_id.clone(),
                governed_proposal_ref: p.governed_proposal_ref.clone(),
                intent: p.intent.as_str().to_owned(),
                ecosystem: p.ecosystem.as_str().to_owned(),
                label: p.label.clone(),
                safe_fallback: p.safe_fallback.as_str().to_owned(),
                result_class: p.result_class.as_str().to_owned(),
                summary: format!(
                    "{} {} via {} -> {} (validation {})",
                    p.intent.as_str(),
                    p.ecosystem.as_str(),
                    p.safe_fallback.as_str(),
                    p.result_class.as_str(),
                    p.requested_validation.len(),
                ),
            })
            .collect();
        AiPackageMutationReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_propose_only: self.all_propose_only(),
            all_governed: self.all_governed(),
        }
    }

    /// Returns the corpus-coverage gaps: any intent, safe-fallback class, result
    /// class, or validation kind the proposals do not exercise.
    pub fn corpus_coverage_gaps(&self) -> Vec<AiPackageMutationReviewViolation> {
        let mut gaps = Vec::new();
        let intents: BTreeSet<AiMutationIntent> = self.proposals.iter().map(|p| p.intent).collect();
        for required in AiMutationIntent::ALL {
            if !intents.contains(&required) {
                gaps.push(AiPackageMutationReviewViolation::MissingCorpusState {
                    field: "intent",
                    state: required.as_str(),
                });
            }
        }
        let fallbacks: BTreeSet<SafeFallbackClass> =
            self.proposals.iter().map(|p| p.safe_fallback).collect();
        for required in SafeFallbackClass::ALL {
            if !fallbacks.contains(&required) {
                gaps.push(AiPackageMutationReviewViolation::MissingCorpusState {
                    field: "safe_fallback",
                    state: required.as_str(),
                });
            }
        }
        let results: BTreeSet<AiResultClass> =
            self.proposals.iter().map(|p| p.result_class).collect();
        for required in AiResultClass::ALL {
            if !results.contains(&required) {
                gaps.push(AiPackageMutationReviewViolation::MissingCorpusState {
                    field: "result_class",
                    state: required.as_str(),
                });
            }
        }
        let kinds: BTreeSet<AiValidationKind> = self
            .proposals
            .iter()
            .flat_map(|p| p.requested_validation.iter().copied())
            .collect();
        for required in AiValidationKind::ALL {
            if !kinds.contains(&required) {
                gaps.push(AiPackageMutationReviewViolation::MissingCorpusState {
                    field: "validation_kind",
                    state: required.as_str(),
                });
            }
        }
        gaps
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<AiPackageMutationReviewViolation> {
        let mut violations = Vec::new();
        if self.schema_version != AI_PACKAGE_MUTATION_REVIEW_SCHEMA_VERSION {
            violations.push(AiPackageMutationReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != AI_PACKAGE_MUTATION_REVIEW_RECORD_KIND {
            violations.push(AiPackageMutationReviewViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("as_of", &self.as_of),
            (
                "references_governance_packet_id",
                &self.references_governance_packet_id,
            ),
            ("references_matrix_id", &self.references_matrix_id),
            (
                "references_reviewed_flows_id",
                &self.references_reviewed_flows_id,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(AiPackageMutationReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab_checks: [(&'static str, bool); 5] = [
            ("intents", self.intents == AiMutationIntent::ALL.to_vec()),
            (
                "write_authorities",
                self.write_authorities == AiWriteAuthority::ALL.to_vec(),
            ),
            (
                "safe_fallback_classes",
                self.safe_fallback_classes == SafeFallbackClass::ALL.to_vec(),
            ),
            (
                "result_classes",
                self.result_classes == AiResultClass::ALL.to_vec(),
            ),
            (
                "validation_kinds",
                self.validation_kinds == AiValidationKind::ALL.to_vec(),
            ),
        ];
        for (field, ok) in vocab_checks {
            if !ok {
                violations
                    .push(AiPackageMutationReviewViolation::ClosedVocabularyMismatch { field });
            }
        }
        let mut seen = BTreeSet::new();
        for proposal in &self.proposals {
            if !seen.insert(proposal.proposal_id.clone()) {
                violations.push(AiPackageMutationReviewViolation::DuplicateRowId {
                    row_id: proposal.proposal_id.clone(),
                });
            }
            self.validate_proposal(proposal, &mut violations);
        }
        if self.summary != self.computed_summary() {
            violations.push(AiPackageMutationReviewViolation::SummaryMismatch);
        }
        violations
    }

    fn validate_proposal(
        &self,
        proposal: &AiMutationProposalRow,
        violations: &mut Vec<AiPackageMutationReviewViolation>,
    ) {
        for (field, value) in [
            ("proposal_id", &proposal.proposal_id),
            ("governed_proposal_ref", &proposal.governed_proposal_ref),
            ("label", &proposal.label),
            ("rollback_handle_ref", &proposal.rollback_handle_ref),
            ("redacted_manifest_path", &proposal.redacted_manifest_path),
            ("note", &proposal.note),
        ] {
            if value.trim().is_empty() {
                violations.push(AiPackageMutationReviewViolation::EmptyField {
                    id: proposal.proposal_id.clone(),
                    field_name: field,
                });
            }
        }
        for (field, value) in [
            ("rollback_handle_ref", &proposal.rollback_handle_ref),
            ("redacted_manifest_path", &proposal.redacted_manifest_path),
        ] {
            if value.contains("://") {
                violations.push(AiPackageMutationReviewViolation::RawUrlLeak {
                    id: proposal.proposal_id.clone(),
                    field_name: field,
                });
            }
        }
        // The AI surface may never claim execution authority.
        if proposal.write_authority.can_execute() {
            violations.push(AiPackageMutationReviewViolation::AiSurfaceClaimsExecution {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // Every proposal must be preview-first and routed through governed review.
        if !proposal.preview_first || !proposal.routes_through_governed_review {
            violations.push(AiPackageMutationReviewViolation::UngovernedProposal {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // No proposal may turn package mutation into hidden scripting.
        if !proposal.no_hidden_scripting {
            violations.push(AiPackageMutationReviewViolation::HiddenScriptingAllowed {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
        // The result class must be coherent with the safe-fallback class.
        if !proposal.result_is_coherent() {
            violations.push(AiPackageMutationReviewViolation::ResultFallbackMismatch {
                proposal_id: proposal.proposal_id.clone(),
            });
        }
    }
}

/// A validation violation for the AI package-mutation-review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiPackageMutationReviewViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A proposal id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A required corpus state is missing.
    MissingCorpusState {
        /// Field that must exercise the state.
        field: &'static str,
        /// Missing state token.
        state: &'static str,
    },
    /// The AI surface claims execution authority it can never carry.
    AiSurfaceClaimsExecution {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal is not preview-first or not routed through governed review.
    UngovernedProposal {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal would allow package mutation to become hidden scripting.
    HiddenScriptingAllowed {
        /// Proposal id.
        proposal_id: String,
    },
    /// A proposal's result class disagrees with its safe-fallback class.
    ResultFallbackMismatch {
        /// Proposal id.
        proposal_id: String,
    },
    /// A redacted field leaks a raw URL.
    RawUrlLeak {
        /// Row id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// Summary counts disagree with the proposals.
    SummaryMismatch,
}

impl fmt::Display for AiPackageMutationReviewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate proposal row id {row_id}")
            }
            Self::MissingCorpusState { field, state } => {
                write!(f, "packet corpus does not exercise {field} state {state}")
            }
            Self::AiSurfaceClaimsExecution { proposal_id } => {
                write!(f, "proposal {proposal_id} claims AI execution authority")
            }
            Self::UngovernedProposal { proposal_id } => write!(
                f,
                "proposal {proposal_id} is not preview-first or not governed-reviewed"
            ),
            Self::HiddenScriptingAllowed { proposal_id } => write!(
                f,
                "proposal {proposal_id} would allow package mutation as hidden scripting"
            ),
            Self::ResultFallbackMismatch { proposal_id } => write!(
                f,
                "proposal {proposal_id} result class disagrees with its safe-fallback class"
            ),
            Self::RawUrlLeak { id, field_name } => {
                write!(f, "{id} field {field_name} leaks a raw URL")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the proposals"),
        }
    }
}

impl Error for AiPackageMutationReviewViolation {}

/// Loads the embedded AI package-mutation-review packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`AiPackageMutationReviewPacket`].
pub fn current_ai_package_mutation_review_packet(
) -> Result<AiPackageMutationReviewPacket, serde_json::Error> {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/package_mutation_review/support_export.json"
    )))
}

#[cfg(test)]
mod tests;
