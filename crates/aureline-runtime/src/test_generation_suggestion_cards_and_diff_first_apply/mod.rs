//! AI-assisted test-generation suggestion cards with uncovered-path / bug
//! provenance, named assumptions, sandbox validation, and diff-first apply parity
//! for the M5 test-intelligence lane.
//!
//! Where [`crate::coverage_overlays_and_snapshot_golden_review`] governs whether the
//! coverage and snapshot **evidence** drawn over the editor is trustworthy, this
//! module governs whether an **AI-assisted test proposal** is trustworthy enough to
//! flow through the same preview / diff / apply / revert pipeline the rest of the
//! lane uses. A generated test is not a free-text suggestion and an apply is not a
//! blind write: both become inspectable, evidence-bound, export-safe records.
//!
//! * a [`TestGenerationSuggestionCard`] ties a durable [`GeneratedTestSubject`]
//!   (keyed by a [`DurableTestNodeKind`] and a non-display fingerprint, so a
//!   parameterized template never collapses into a concrete invocation) to the
//!   [`TargetReference`] symbols / files it exercises, the [`GenerationSourceKind`]
//!   that motivated it (an uncovered coverage / branch path, a bug reproduction, a
//!   regression guard, or a changed-code gap), a list of named [`AssumptionEntry`]
//!   rows, the [`GeneratedFileEntry`] files it would write, a [`SandboxValidation`]
//!   posture, and an [`ApplyPosture`];
//! * the proposal is **evidence-bound, not free-text**: every card carries one or
//!   more [`EvidenceReference`] rows over the [`EvidenceObjectKind`] vocabulary
//!   (coverage overlay, discovery snapshot, session plan, attempt record, stability
//!   verdict, diagnostic, bug report), each addressable by a non-display fingerprint
//!   so generated-test review can reopen the same discovery / session / coverage
//!   object that motivated the proposal;
//! * the proposal cannot **bypass preview / diff / apply / revert discipline**: an
//!   [`ApplyPosture`] carries an [`ApplyState`], a `preview_first` flag, a diff ref,
//!   an optional revert ref, and a `widens_beyond_evidence` flag. An applied card
//!   must have been previewed, must carry a [`ValidationPosture::SandboxValidatedPass`]
//!   from an isolated sandbox run, must not widen beyond its evidenced scope, must
//!   carry a follow-on rerun ref, and must be locally generated — an imported
//!   proposal is held read-only and never reads as a local apply.
//!
//! [`TestGenerationProposalPacket::validate`] refuses a packet that exposes an apply
//! path before naming assumptions, evidence basis, affected files, and a determinate
//! validation posture; that applies a generated test without an isolated sandbox
//! pass; that bypasses the preview-first diff pipeline; that silently widens beyond
//! the evidenced scope; that lets an imported proposal read as a local apply; that
//! relies on free-text justification instead of a reopenable evidence object; or that
//! collapses a parameterized template into a concrete invocation.
//!
//! Generated source bodies, raw model prompts / completions, sandbox stdout, diff
//! bytes, raw provider payloads, provider cursors, credentials, and host names never
//! cross this boundary; the packet carries only typed class tokens, booleans, counts,
//! opaque ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json`](../../../../schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json).
//! The contract doc is
//! [`docs/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md`](../../../../docs/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/`](../../../../fixtures/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`TestGenerationProposalPacket`].
pub const TEST_GENERATION_RECORD_KIND: &str = "test_generation_proposal_packet";

/// Schema version for the test-generation proposal packet.
pub const TEST_GENERATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TEST_GENERATION_SCHEMA_REF: &str =
    "schemas/testing/test-generation-suggestion-cards-and-diff-first-apply.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_GENERATION_DOC_REF: &str =
    "docs/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md";

/// Repo-relative path of the checked support-export artifact.
pub const TEST_GENERATION_ARTIFACT_REF: &str =
    "artifacts/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const TEST_GENERATION_SUMMARY_REF: &str =
    "artifacts/testing/m5/test-generation-suggestion-cards-and-diff-first-apply.md";

/// Repo-relative path of the protected fixture directory.
pub const TEST_GENERATION_FIXTURE_DIR: &str =
    "fixtures/testing/m5/test-generation-suggestion-cards-and-diff-first-apply";

/// What motivated an AI-assisted test proposal. This is the uncovered-path / bug
/// provenance anchor: a proposal is never an unmotivated suggestion, it is tied to a
/// concrete gap in the lane's existing evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GenerationSourceKind {
    /// An uncovered line path surfaced by a coverage overlay.
    UncoveredCoveragePath,
    /// An uncovered branch path surfaced by a coverage overlay.
    UncoveredBranchPath,
    /// A reproduction for a reported or observed bug.
    BugReproduction,
    /// A guard added to pin behavior against a regression.
    RegressionGuard,
    /// Changed code in the diff that carries no test.
    ChangedCodeGap,
    /// Source cannot be classified; treated as non-actionable until reviewed.
    UnknownRequiresReview,
}

impl GenerationSourceKind {
    /// Every source kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UncoveredCoveragePath,
        Self::UncoveredBranchPath,
        Self::BugReproduction,
        Self::RegressionGuard,
        Self::ChangedCodeGap,
        Self::UnknownRequiresReview,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UncoveredCoveragePath => "uncovered_coverage_path",
            Self::UncoveredBranchPath => "uncovered_branch_path",
            Self::BugReproduction => "bug_reproduction",
            Self::RegressionGuard => "regression_guard",
            Self::ChangedCodeGap => "changed_code_gap",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether the source is an uncovered-path gap (line or branch).
    pub const fn is_uncovered_path(self) -> bool {
        matches!(
            self,
            Self::UncoveredCoveragePath | Self::UncoveredBranchPath
        )
    }

    /// Whether the source is a bug-driven proposal.
    pub const fn is_bug_source(self) -> bool {
        matches!(self, Self::BugReproduction)
    }
}

/// Kind of evidence object a proposal binds to, so a reviewer can reopen the exact
/// discovery / session / coverage object rather than read a free-text justification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceObjectKind {
    /// A coverage overlay / legend record.
    CoverageOverlay,
    /// A test discovery snapshot.
    DiscoverySnapshot,
    /// A canonical session plan.
    SessionPlan,
    /// An append-only attempt record.
    AttemptRecord,
    /// A stability verdict / quarantine record.
    StabilityVerdict,
    /// A diagnostic / failing-test record.
    DiagnosticRecord,
    /// A reported bug.
    BugReport,
}

impl EvidenceObjectKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageOverlay => "coverage_overlay",
            Self::DiscoverySnapshot => "discovery_snapshot",
            Self::SessionPlan => "session_plan",
            Self::AttemptRecord => "attempt_record",
            Self::StabilityVerdict => "stability_verdict",
            Self::DiagnosticRecord => "diagnostic_record",
            Self::BugReport => "bug_report",
        }
    }
}

/// Posture of the sandbox validation a generated test went through. A generated test
/// is run in an isolated sandbox before it can be applied, so a pass is evidence and
/// not a promise.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPosture {
    /// The generated test ran in an isolated sandbox and passed; the only posture
    /// that may gate a local apply.
    SandboxValidatedPass,
    /// The generated test ran in the sandbox and failed (e.g. it demonstrates the
    /// bug it was generated to reproduce). It cannot be applied as a passing test.
    SandboxValidatedFail,
    /// Sandbox validation is in progress.
    SandboxValidationPending,
    /// Sandbox validation could not run (toolchain / environment error).
    SandboxValidationError,
    /// The generated test has not been validated; an apply is blocked.
    NotValidated,
    /// The proposal was imported and not locally validated; held read-only.
    ImportedUnvalidated,
}

impl ValidationPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SandboxValidatedPass,
        Self::SandboxValidatedFail,
        Self::SandboxValidationPending,
        Self::SandboxValidationError,
        Self::NotValidated,
        Self::ImportedUnvalidated,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SandboxValidatedPass => "sandbox_validated_pass",
            Self::SandboxValidatedFail => "sandbox_validated_fail",
            Self::SandboxValidationPending => "sandbox_validation_pending",
            Self::SandboxValidationError => "sandbox_validation_error",
            Self::NotValidated => "not_validated",
            Self::ImportedUnvalidated => "imported_unvalidated",
        }
    }

    /// Whether this posture is an isolated-sandbox pass that may gate a local apply.
    pub const fn is_validated_pass(self) -> bool {
        matches!(self, Self::SandboxValidatedPass)
    }

    /// Whether validation has at least reached a determinate state (not pending and
    /// not entirely unvalidated), which an apply path requires.
    pub const fn is_determinate(self) -> bool {
        !matches!(self, Self::SandboxValidationPending | Self::NotValidated)
    }
}

/// State of a proposal in the preview / diff / apply / revert pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyState {
    /// Awaiting preview; no apply path has appeared.
    PendingPreview,
    /// The diff was previewed; an apply path is available.
    Previewed,
    /// The diff was applied to the working tree (preview-first).
    Applied,
    /// A previously applied diff was reverted.
    Reverted,
    /// The proposal was rejected; nothing applied.
    Rejected,
    /// Apply is blocked until sandbox validation succeeds.
    BlockedNeedsValidation,
}

impl ApplyState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingPreview => "pending_preview",
            Self::Previewed => "previewed",
            Self::Applied => "applied",
            Self::Reverted => "reverted",
            Self::Rejected => "rejected",
            Self::BlockedNeedsValidation => "blocked_needs_validation",
        }
    }

    /// Whether an apply path has appeared for the card (preview, apply, or revert).
    /// Disclosure of assumptions, evidence, files, and a determinate validation
    /// posture is required once this is true.
    pub const fn exposes_apply_path(self) -> bool {
        matches!(self, Self::Previewed | Self::Applied | Self::Reverted)
    }

    /// Whether this state mutates the working tree.
    pub const fn is_applied(self) -> bool {
        matches!(self, Self::Applied)
    }
}

/// Provenance of a proposal: locally generated or imported from a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalProvenance {
    /// Generated locally against local evidence.
    LocallyGenerated,
    /// Imported from a provider; read-only and never a local apply.
    ImportedProposal,
    /// Provenance cannot be classified; treated as non-authoritative.
    UnknownRequiresReview,
}

impl ProposalProvenance {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocallyGenerated => "locally_generated",
            Self::ImportedProposal => "imported_proposal",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether this provenance is imported / provider-backed and read-only.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedProposal)
    }
}

/// Kind of file a proposal would write or modify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedFileKind {
    /// A brand-new test file.
    NewTestFile,
    /// Tests appended to an existing test file.
    AppendedTestFile,
    /// A new fixture / support file.
    NewFixtureFile,
    /// A new snapshot / golden baseline.
    NewSnapshotBaseline,
}

impl GeneratedFileKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NewTestFile => "new_test_file",
            Self::AppendedTestFile => "appended_test_file",
            Self::NewFixtureFile => "new_fixture_file",
            Self::NewSnapshotBaseline => "new_snapshot_baseline",
        }
    }
}

/// One target symbol / file a generated test exercises, keyed by a non-display
/// fingerprint rather than a label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetReference {
    /// Durable target id.
    pub target_id: String,
    /// Reviewable file ref the target lives in.
    pub file_ref: String,
    /// Reviewable symbol label (function / type / module).
    pub symbol_label: String,
    /// Non-display fingerprint token. Must differ from
    /// [`target_id`](TargetReference::target_id).
    pub target_fingerprint_token: String,
}

impl TargetReference {
    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.target_fingerprint_token.trim();
        !token.is_empty() && token != self.target_id.trim()
    }

    /// Whether the target carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.target_id.trim().is_empty()
            && !self.file_ref.trim().is_empty()
            && !self.symbol_label.trim().is_empty()
            && self.fingerprint_independent_of_id()
    }
}

/// One reopenable evidence object a proposal binds to. This is what keeps a proposal
/// from relying on free-text justification: a reviewer reopens the referenced object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReference {
    /// Kind of evidence object.
    pub evidence_kind: EvidenceObjectKind,
    /// Opaque, reconstructable ref of the evidence object.
    pub evidence_ref: String,
    /// Non-display fingerprint token. Must differ from
    /// [`evidence_ref`](EvidenceReference::evidence_ref).
    pub evidence_fingerprint_token: String,
    /// Export-safe reviewable summary of why the object motivated the proposal.
    pub summary: String,
}

impl EvidenceReference {
    /// Whether the fingerprint is a real non-display basis distinct from the ref.
    pub fn fingerprint_independent_of_ref(&self) -> bool {
        let token = self.evidence_fingerprint_token.trim();
        !token.is_empty() && token != self.evidence_ref.trim()
    }

    /// Whether the reference can actually be reopened (ref present, fingerprint a real
    /// basis, summary present).
    pub fn is_valid(&self) -> bool {
        !self.evidence_ref.trim().is_empty()
            && self.fingerprint_independent_of_ref()
            && !self.summary.trim().is_empty()
    }
}

/// One assumption a proposal names so a reviewer can confirm or reject it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionEntry {
    /// Reviewable assumption summary.
    pub summary: String,
    /// Whether the assumption requires explicit confirmation before apply.
    pub requires_confirmation: bool,
}

impl AssumptionEntry {
    /// Whether the assumption carries a non-empty summary.
    pub fn is_valid(&self) -> bool {
        !self.summary.trim().is_empty()
    }
}

/// One file a proposal would write or modify, carrying a reconstructable diff ref.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedFileEntry {
    /// Reviewable file ref.
    pub file_ref: String,
    /// Kind of change.
    pub change_kind: GeneratedFileKind,
    /// Reconstructable diff-summary ref (never raw diff bytes).
    pub diff_summary_ref: String,
}

impl GeneratedFileEntry {
    /// Whether the entry carries a file ref and a diff ref.
    pub fn is_valid(&self) -> bool {
        !self.file_ref.trim().is_empty() && !self.diff_summary_ref.trim().is_empty()
    }
}

/// Sandbox-validation block for a generated test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxValidation {
    /// Validation posture.
    pub posture: ValidationPosture,
    /// Sandbox run / session ref (reconstructable elsewhere), present when a run
    /// occurred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sandbox_run_ref: Option<String>,
    /// Count of generated tests executed in the sandbox.
    pub tests_executed: u32,
    /// Count of generated tests that passed in the sandbox.
    pub tests_passed: u32,
    /// Whether the sandbox ran isolated from the working tree.
    pub isolated: bool,
    /// Export-safe validation summary.
    pub summary: String,
}

impl SandboxValidation {
    /// Whether the pass / executed counts are well-formed.
    pub const fn counts_consistent(&self) -> bool {
        self.tests_passed <= self.tests_executed
    }

    /// Whether a [`ValidationPosture::SandboxValidatedPass`] is backed by an isolated
    /// run that actually executed and passed every generated test.
    pub fn pass_is_backed(&self) -> bool {
        if self.posture.is_validated_pass() {
            self.isolated
                && self.sandbox_run_ref.is_some()
                && self.tests_executed >= 1
                && self.tests_passed == self.tests_executed
        } else {
            true
        }
    }

    /// Whether the validation block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.counts_consistent()
            && self.pass_is_backed()
            && !self.summary.trim().is_empty()
            && self
                .sandbox_run_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Preview / diff / apply / revert posture for a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyPosture {
    /// Pipeline state.
    pub state: ApplyState,
    /// Whether the change is presented preview-first (never a blind write).
    pub preview_first: bool,
    /// Reconstructable diff ref the apply path operates on.
    pub diff_ref: String,
    /// Revert ref, present once applied or reverted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revert_ref: Option<String>,
    /// Whether the proposal would widen the test scope beyond its evidenced basis.
    /// A card that widens may never be applied; it routes to review.
    pub widens_beyond_evidence: bool,
}

impl ApplyPosture {
    /// Whether the posture is well-formed: a reverted card carries a revert ref, an
    /// applied card was presented preview-first, and the diff ref is present.
    pub fn is_valid(&self) -> bool {
        !self.diff_ref.trim().is_empty()
            && (self.state != ApplyState::Reverted || self.revert_ref.is_some())
            && (!self.state.is_applied() || self.preview_first)
            && self
                .revert_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Durable subject of a generated test, keyed by a node kind and a non-display
/// fingerprint distinct from its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTestSubject {
    /// Durable node id of the generated test.
    pub subject_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a parameterized
    /// template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](GeneratedTestSubject::subject_id).
    pub subject_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl GeneratedTestSubject {
    /// Whether this subject is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// An AI-assisted test-generation suggestion card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationSuggestionCard {
    /// Stable card id.
    pub card_id: String,
    /// Durable subject the card would generate.
    pub subject: GeneratedTestSubject,
    /// What motivated the proposal.
    pub source_kind: GenerationSourceKind,
    /// Provenance of the proposal (local or imported).
    pub provenance: ProposalProvenance,
    /// Target symbols / files the generated test exercises.
    pub targets: Vec<TargetReference>,
    /// Reopenable evidence objects the proposal binds to.
    pub evidence_basis: Vec<EvidenceReference>,
    /// Named assumptions a reviewer can confirm or reject.
    pub assumptions: Vec<AssumptionEntry>,
    /// Files the proposal would write or modify.
    pub generated_files: Vec<GeneratedFileEntry>,
    /// Sandbox-validation block.
    pub sandbox_validation: SandboxValidation,
    /// Preview / diff / apply / revert posture.
    pub apply_posture: ApplyPosture,
    /// Origin provider ref, present iff the proposal is imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_provider_ref: Option<String>,
    /// Follow-on rerun ref linking the applied test back into the run / session lane,
    /// so generated tests are diagnosed like ordinary changes. Present once applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_on_rerun_ref: Option<String>,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe card summary.
    pub support_summary: String,
}

impl TestGenerationSuggestionCard {
    /// Count of files the proposal would write or modify.
    pub fn generated_file_count(&self) -> usize {
        self.generated_files.len()
    }

    /// Whether the proposal is imported / provider-backed.
    pub fn is_imported(&self) -> bool {
        self.provenance.is_imported()
    }

    /// Whether an apply path has appeared for the card.
    pub fn exposes_apply_path(&self) -> bool {
        self.apply_posture.state.exposes_apply_path()
    }

    /// Whether the proposal names assumptions, evidence basis, affected files, and a
    /// determinate validation posture before any apply path appears.
    pub fn apply_path_disclosure_consistent(&self) -> bool {
        if self.exposes_apply_path() {
            !self.assumptions.is_empty()
                && !self.evidence_basis.is_empty()
                && !self.targets.is_empty()
                && !self.generated_files.is_empty()
                && self.sandbox_validation.posture.is_determinate()
        } else {
            true
        }
    }

    /// Whether evidence is reopenable rather than free-text: at least one evidence
    /// object is referenced and every reference resolves.
    pub fn evidence_reopenable(&self) -> bool {
        !self.evidence_basis.is_empty()
            && self.evidence_basis.iter().all(EvidenceReference::is_valid)
    }

    /// Whether the imported markers agree, and an imported proposal is never applied
    /// as a local result and never reads as a local validated pass.
    pub fn imported_markers_consistent(&self) -> bool {
        let markers = self.is_imported() == self.origin_provider_ref.is_some()
            && self.is_imported() == self.subject.is_imported();
        if self.is_imported() {
            markers
                && self.sandbox_validation.posture == ValidationPosture::ImportedUnvalidated
                && !self.apply_posture.state.is_applied()
        } else {
            markers && self.sandbox_validation.posture != ValidationPosture::ImportedUnvalidated
        }
    }

    /// Whether the apply gate is respected: an applied card was previewed, carries an
    /// isolated sandbox pass, does not widen beyond evidence, carries a follow-on
    /// rerun ref, and is locally generated.
    pub fn apply_gate_consistent(&self) -> bool {
        if self.apply_posture.state.is_applied() {
            self.apply_posture.preview_first
                && self.sandbox_validation.posture.is_validated_pass()
                && !self.apply_posture.widens_beyond_evidence
                && self.follow_on_rerun_ref.is_some()
                && !self.is_imported()
        } else {
            true
        }
    }

    /// Whether every field required to record this card is present and its invariants
    /// hold.
    pub fn is_valid(&self) -> bool {
        !self.card_id.trim().is_empty()
            && self.subject.is_valid()
            && !self.targets.is_empty()
            && self.targets.iter().all(TargetReference::is_valid)
            && self.evidence_reopenable()
            && self.assumptions.iter().all(AssumptionEntry::is_valid)
            && !self.generated_files.is_empty()
            && self
                .generated_files
                .iter()
                .all(GeneratedFileEntry::is_valid)
            && self.sandbox_validation.is_valid()
            && self.apply_posture.is_valid()
            && self.apply_path_disclosure_consistent()
            && self.imported_markers_consistent()
            && self.apply_gate_consistent()
            && !self.captured_at.trim().is_empty()
            && !self.support_summary.trim().is_empty()
            && self
                .origin_provider_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
            && self
                .follow_on_rerun_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationGuardrails {
    /// Proposals name assumptions, evidence, files, and a posture before apply.
    pub disclosure_before_apply: bool,
    /// Proposals bind to reopenable evidence objects, not free-text justification.
    pub evidence_bound_not_free_text: bool,
    /// Generated tests are sandbox-validated before any local apply.
    pub sandbox_validated_before_apply: bool,
    /// Proposals flow through the same preview / diff / apply / revert pipeline.
    pub preview_diff_apply_revert_parity: bool,
    /// Proposals never silently widen beyond their evidenced scope.
    pub no_silent_scope_widening: bool,
    /// Imported / provider-backed proposals never read as a local apply.
    pub imported_never_reads_as_local: bool,
    /// Parameterized templates and concrete invocations keep distinct identity.
    pub template_invocation_distinct: bool,
}

impl TestGenerationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.disclosure_before_apply
            && self.evidence_bound_not_free_text
            && self.sandbox_validated_before_apply
            && self.preview_diff_apply_revert_parity
            && self.no_silent_scope_widening
            && self.imported_never_reads_as_local
            && self.template_invocation_distinct
    }
}

/// Consumer projection block: the surfaces that read this packet without re-deriving
/// test-generation truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationConsumerProjection {
    /// The test-generation suggestion-card UI normalizes onto these cards.
    pub suggestion_card_ui_normalized: bool,
    /// The preview / diff / apply / revert pipeline normalizes onto these postures.
    pub diff_apply_pipeline_normalized: bool,
    /// Generated-test review reopens these evidence objects.
    pub evidence_reopen_normalized: bool,
    /// Follow-on rerun / diagnose flows read these refs.
    pub rerun_diagnose_normalized: bool,
    /// Release and support exports read the same records.
    pub release_support_export_normalized: bool,
}

impl TestGenerationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.suggestion_card_ui_normalized
            && self.diff_apply_pipeline_normalized
            && self.evidence_reopen_normalized
            && self.rerun_diagnose_normalized
            && self.release_support_export_normalized
    }
}

/// Constructor input for [`TestGenerationProposalPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestGenerationProposalPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Test-generation suggestion cards.
    pub cards: Vec<TestGenerationSuggestionCard>,
    /// Guardrail invariants block.
    pub guardrails: TestGenerationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestGenerationConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe test-generation proposal packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationProposalPacket {
    /// Record kind; must equal [`TEST_GENERATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_GENERATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Test-generation suggestion cards.
    pub cards: Vec<TestGenerationSuggestionCard>,
    /// Guardrail invariants block.
    pub guardrails: TestGenerationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestGenerationConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TestGenerationProposalPacket {
    /// Builds a test-generation proposal packet.
    pub fn new(input: TestGenerationProposalPacketInput) -> Self {
        Self {
            record_kind: TEST_GENERATION_RECORD_KIND.to_owned(),
            schema_version: TEST_GENERATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            cards: input.cards,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Source kinds represented across cards.
    pub fn represented_source_kinds(&self) -> BTreeSet<GenerationSourceKind> {
        self.cards.iter().map(|c| c.source_kind).collect()
    }

    /// Validation postures represented across cards.
    pub fn represented_validation_postures(&self) -> BTreeSet<ValidationPosture> {
        self.cards
            .iter()
            .map(|c| c.sandbox_validation.posture)
            .collect()
    }

    /// Apply states represented across cards.
    pub fn represented_apply_states(&self) -> BTreeSet<ApplyState> {
        self.cards.iter().map(|c| c.apply_posture.state).collect()
    }

    /// Subject node kinds represented across cards.
    pub fn represented_subject_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.cards.iter().map(|c| c.subject.node_kind).collect()
    }

    /// Resolves a card by its id.
    pub fn card(&self, card_id: &str) -> Option<&TestGenerationSuggestionCard> {
        self.cards.iter().find(|c| c.card_id == card_id)
    }

    /// Count of imported cards.
    pub fn imported_card_count(&self) -> usize {
        self.cards.iter().filter(|c| c.is_imported()).count()
    }

    /// Count of applied cards.
    pub fn applied_card_count(&self) -> usize {
        self.cards
            .iter()
            .filter(|c| c.apply_posture.state.is_applied())
            .count()
    }

    /// Count of cards blocked pending sandbox validation.
    pub fn blocked_card_count(&self) -> usize {
        self.cards
            .iter()
            .filter(|c| c.apply_posture.state == ApplyState::BlockedNeedsValidation)
            .count()
    }

    /// Validates the test-generation invariants.
    pub fn validate(&self) -> Vec<TestGenerationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != TEST_GENERATION_RECORD_KIND {
            violations.push(TestGenerationViolation::WrongRecordKind);
        }
        if self.schema_version != TEST_GENERATION_SCHEMA_VERSION {
            violations.push(TestGenerationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(TestGenerationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_generation_matrix(self, &mut violations);
        validate_cards(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(TestGenerationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(TestGenerationViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("test generation packet serializes"),
        ) {
            violations.push(TestGenerationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("test generation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Generation Suggestion Cards And Diff-First Apply\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Cards: {} ({} imported, {} applied, {} blocked)\n",
            self.cards.len(),
            self.imported_card_count(),
            self.applied_card_count(),
            self.blocked_card_count()
        ));
        out.push_str(&format!(
            "- Source kinds: {} / {}\n",
            self.represented_source_kinds().len(),
            GenerationSourceKind::ALL.len()
        ));
        out.push_str("\n## Suggestion cards\n\n");
        for card in &self.cards {
            out.push_str(&format!(
                "- **{}** [{}] source `{}`, {} target(s), {} file(s)\n",
                card.card_id,
                card.provenance.as_str(),
                card.source_kind.as_str(),
                card.targets.len(),
                card.generated_file_count()
            ));
            out.push_str(&format!(
                "  - subject `{}` ({}), {} assumption(s), {} evidence ref(s)\n",
                card.subject.subject_id,
                card.subject.node_kind.as_str(),
                card.assumptions.len(),
                card.evidence_basis.len()
            ));
            out.push_str(&format!(
                "  - validation `{}` → apply `{}` (preview-first {})\n",
                card.sandbox_validation.posture.as_str(),
                card.apply_posture.state.as_str(),
                card.apply_posture.preview_first
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum TestGenerationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestGenerationViolation>),
}

impl fmt::Display for TestGenerationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "test generation export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "test generation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TestGenerationArtifactError {}

/// Validation failures emitted by [`TestGenerationProposalPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestGenerationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// Cards do not exercise both an uncovered-path and a bug source.
    SourceKindCaseMissing,
    /// Cards do not exercise both a sandbox-validated pass and an unvalidated state.
    ValidationPostureCaseMissing,
    /// Cards do not exercise both an applied and a blocked-needs-validation state.
    ApplyStateCaseMissing,
    /// No card holds an imported proposal read-only.
    ImportedProposalCaseMissing,
    /// No card exercises the no-silent-widening guard (widens-beyond-evidence routed
    /// to review).
    WideningGuardCaseMissing,
    /// Cards collapse a parameterized template into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// A card is incomplete.
    CardInvalid,
    /// A card's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A card exposes an apply path before naming assumptions, evidence, files, and a
    /// determinate validation posture.
    ApplyPathWithoutDisclosure,
    /// A card relies on free-text justification instead of a reopenable evidence
    /// object.
    EvidenceNotReopenable,
    /// A generated test was applied without an isolated sandbox pass.
    AppliedWithoutSandboxValidation,
    /// A generated test was applied without the preview-first diff pipeline.
    AppliedBypassesPreview,
    /// A generated test was applied while widening beyond its evidenced scope.
    AppliedWidensBeyondEvidence,
    /// An applied generated test carries no follow-on rerun linkage.
    AppliedWithoutRerunLinkage,
    /// An imported proposal reads as a local apply or local validated pass.
    ImportedReadsAsLocal,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl TestGenerationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SourceKindCaseMissing => "source_kind_case_missing",
            Self::ValidationPostureCaseMissing => "validation_posture_case_missing",
            Self::ApplyStateCaseMissing => "apply_state_case_missing",
            Self::ImportedProposalCaseMissing => "imported_proposal_case_missing",
            Self::WideningGuardCaseMissing => "widening_guard_case_missing",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::CardInvalid => "card_invalid",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::ApplyPathWithoutDisclosure => "apply_path_without_disclosure",
            Self::EvidenceNotReopenable => "evidence_not_reopenable",
            Self::AppliedWithoutSandboxValidation => "applied_without_sandbox_validation",
            Self::AppliedBypassesPreview => "applied_bypasses_preview",
            Self::AppliedWidensBeyondEvidence => "applied_widens_beyond_evidence",
            Self::AppliedWithoutRerunLinkage => "applied_without_rerun_linkage",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_test_generation_export(
) -> Result<TestGenerationProposalPacket, TestGenerationArtifactError> {
    let packet: TestGenerationProposalPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/test-generation-suggestion-cards-and-diff-first-apply/support_export.json"
    )))
    .map_err(TestGenerationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestGenerationArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &TestGenerationProposalPacket,
    violations: &mut Vec<TestGenerationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        TEST_GENERATION_SCHEMA_REF,
        TEST_GENERATION_DOC_REF,
        TEST_GENERATION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(TestGenerationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_generation_matrix(
    packet: &TestGenerationProposalPacket,
    violations: &mut Vec<TestGenerationViolation>,
) {
    let sources = packet.represented_source_kinds();
    if !(sources.iter().any(|s| s.is_uncovered_path()) && sources.iter().any(|s| s.is_bug_source()))
    {
        violations.push(TestGenerationViolation::SourceKindCaseMissing);
    }

    let postures = packet.represented_validation_postures();
    let has_unvalidated = postures.contains(&ValidationPosture::NotValidated)
        || postures.contains(&ValidationPosture::SandboxValidationPending);
    if !(postures.contains(&ValidationPosture::SandboxValidatedPass) && has_unvalidated) {
        violations.push(TestGenerationViolation::ValidationPostureCaseMissing);
    }

    let states = packet.represented_apply_states();
    if !(states.contains(&ApplyState::Applied)
        && states.contains(&ApplyState::BlockedNeedsValidation))
    {
        violations.push(TestGenerationViolation::ApplyStateCaseMissing);
    }

    if packet.imported_card_count() == 0 {
        violations.push(TestGenerationViolation::ImportedProposalCaseMissing);
    }

    if !packet
        .cards
        .iter()
        .any(|c| c.apply_posture.widens_beyond_evidence)
    {
        violations.push(TestGenerationViolation::WideningGuardCaseMissing);
    }

    let subject_kinds = packet.represented_subject_kinds();
    if !(subject_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && subject_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(TestGenerationViolation::TemplateCollapsedWithInvocation);
    }
}

fn validate_cards(
    packet: &TestGenerationProposalPacket,
    violations: &mut Vec<TestGenerationViolation>,
) {
    for card in &packet.cards {
        if !card.is_valid() {
            violations.push(TestGenerationViolation::CardInvalid);
        }
        if !card.subject.fingerprint_independent_of_id() {
            violations.push(TestGenerationViolation::FingerprintSubstitutesIdentity);
        }
        if !card.apply_path_disclosure_consistent() {
            violations.push(TestGenerationViolation::ApplyPathWithoutDisclosure);
        }
        if !card.evidence_reopenable() {
            violations.push(TestGenerationViolation::EvidenceNotReopenable);
        }
        if !card.imported_markers_consistent() {
            violations.push(TestGenerationViolation::ImportedReadsAsLocal);
        }
        if card.apply_posture.state.is_applied() {
            if !card.sandbox_validation.posture.is_validated_pass() {
                violations.push(TestGenerationViolation::AppliedWithoutSandboxValidation);
            }
            if !card.apply_posture.preview_first {
                violations.push(TestGenerationViolation::AppliedBypassesPreview);
            }
            if card.apply_posture.widens_beyond_evidence {
                violations.push(TestGenerationViolation::AppliedWidensBeyondEvidence);
            }
            if card.follow_on_rerun_ref.is_none() {
                violations.push(TestGenerationViolation::AppliedWithoutRerunLinkage);
            }
        }
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
