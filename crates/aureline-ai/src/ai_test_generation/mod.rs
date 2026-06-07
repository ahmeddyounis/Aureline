//! Stable AI test-generation proposal, assumption, sandbox, and impact records.
//!
//! This module owns one export-safe packet for AI-assisted test generation.
//! The packet binds a concrete trigger, assumption-review sheet, generated-test
//! diff risk classes, sandbox-validation record, and coverage-impact note for
//! each candidate test proposal. It keeps generated tests in draft/review
//! posture until assumptions, diff scope, and sandbox state are inspectable, and
//! it prevents one sandbox pass from silently becoming release or benchmark
//! coverage truth.
//!
//! The packet references the older
//! [`schemas/testing/ai_test_generation_gate.schema.json`](../../../schemas/testing/ai_test_generation_gate.schema.json)
//! admission gate rather than replacing it. That gate remains canonical for
//! selector/protected-path admission; this packet is canonical for the stable
//! AI test-generation review lane and support/export projection.
//!
//! The record is export-safe. It carries ids, refs, closed vocabulary tokens,
//! counts, short summaries, lineage refs, and policy classes only. Raw generated
//! test source, raw patch bodies, raw diffs, raw runner logs, raw stdout/stderr,
//! raw prompts, provider payloads, credentials, exact endpoint URLs, absolute
//! local paths, and secret values stay outside this boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiTestGenerationTruthPacket`].
pub const AI_TEST_GENERATION_TRUTH_RECORD_KIND: &str = "ai_test_generation_truth";

/// Schema version for AI test-generation truth records.
pub const AI_TEST_GENERATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI test-generation truth boundary schema.
pub const AI_TEST_GENERATION_TRUTH_SCHEMA_REF: &str =
    "schemas/ai/ai-test-generation-assumption-and-sandbox-truth.schema.json";

/// Repo-relative path of the AI test-generation truth contract doc.
pub const AI_TEST_GENERATION_TRUTH_AI_DOC_REF: &str =
    "docs/ai/m4/ai-test-generation-assumption-and-sandbox-truth.md";

/// Repo-relative path of the frozen testing admission gate this lane references.
pub const AI_TEST_GENERATION_GATE_SCHEMA_REF: &str =
    "schemas/testing/ai_test_generation_gate.schema.json";

/// Repo-relative path of the testing intelligence contract this lane projects.
pub const AI_TEST_GENERATION_TESTING_CONTRACT_REF: &str =
    "docs/testing/test_intelligence_and_acceptance_contract.md";

/// Repo-relative path of the protected AI test-generation fixture directory.
pub const AI_TEST_GENERATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/ai/m4/ai-test-generation-assumption-and-sandbox-truth";

/// Repo-relative path of the checked AI test-generation support export.
pub const AI_TEST_GENERATION_TRUTH_ARTIFACT_REF: &str =
    "artifacts/ai/m4/ai-test-generation-assumption-and-sandbox-truth/support_export.json";

/// Repo-relative path of the checked AI test-generation Markdown summary.
pub const AI_TEST_GENERATION_TRUTH_SUMMARY_REF: &str =
    "artifacts/ai/m4/ai-test-generation-assumption-and-sandbox-truth/summary.md";

/// Concrete trigger that motivated an AI-generated test proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestProposalTriggerClass {
    /// A bug report or issue motivated the candidate.
    BugReport,
    /// An uncovered branch or path motivated the candidate.
    UncoveredBranch,
    /// A failing example or regression trace motivated the candidate.
    FailingExample,
    /// A changed symbol or file motivated the candidate.
    ChangedSymbol,
    /// A release-facing regression gap motivated the candidate.
    ReleaseRegressionGap,
}

impl TestProposalTriggerClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BugReport => "bug_report",
            Self::UncoveredBranch => "uncovered_branch",
            Self::FailingExample => "failing_example",
            Self::ChangedSymbol => "changed_symbol",
            Self::ReleaseRegressionGap => "release_regression_gap",
        }
    }
}

/// Review posture of an AI-generated test candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCandidateReviewState {
    /// Candidate has not yet been executed.
    DraftOnly,
    /// Candidate can be reviewed, but remains draft-class.
    ReviewReadyDraft,
    /// Candidate is blocked and can only be reviewed or exported.
    BlockedReviewOnly,
    /// Candidate was promoted as trusted or applied without the review lane.
    TrustedApplied,
}

impl TestCandidateReviewState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftOnly => "draft_only",
            Self::ReviewReadyDraft => "review_ready_draft",
            Self::BlockedReviewOnly => "blocked_review_only",
            Self::TrustedApplied => "trusted_applied",
        }
    }
}

/// Confidence class retained on the candidate after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCandidateConfidenceClass {
    /// High confidence, still review-first.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
}

impl TestCandidateConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    const fn requires_retained_label(self) -> bool {
        matches!(self, Self::Low)
    }
}

/// Flaky-risk class retained on the candidate after validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCandidateFlakyRiskClass {
    /// No flaky risk is currently known.
    NoneKnown,
    /// Flaky risk is possible but not proven.
    PotentiallyFlaky,
    /// Flaky behavior is suspected from available evidence.
    SuspectedFlaky,
}

impl TestCandidateFlakyRiskClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneKnown => "none_known",
            Self::PotentiallyFlaky => "potentially_flaky",
            Self::SuspectedFlaky => "suspected_flaky",
        }
    }

    const fn requires_retained_label(self) -> bool {
        matches!(self, Self::PotentiallyFlaky | Self::SuspectedFlaky)
    }
}

/// Validation status carried by each stable object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTestGenerationValidationStatus {
    /// Object is inspectable and consistent.
    Inspectable,
    /// Object is pending review.
    PendingReview,
    /// Object is blocked by policy, trust, or missing evidence.
    Blocked,
}

impl AiTestGenerationValidationStatus {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspectable => "inspectable",
            Self::PendingReview => "pending_review",
            Self::Blocked => "blocked",
        }
    }

    const fn is_inspectable(self) -> bool {
        matches!(self, Self::Inspectable)
    }
}

/// Source family for one assumption-review row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionClass {
    /// Fixture creation or fixture selection.
    FixtureCreation,
    /// Mock or stub behavior.
    MockBehavior,
    /// Clock, randomness, or deterministic-seed behavior.
    ClockOrRandomness,
    /// Environment-variable assumption.
    EnvironmentVariable,
    /// File-system dependency or expectation.
    FileSystemExpectation,
    /// Network dependency or expectation.
    NetworkExpectation,
    /// Runtime, toolchain, or framework dependency.
    RuntimeDependency,
    /// Unsupported path or limitation.
    UnsupportedPath,
}

impl AssumptionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixtureCreation => "fixture_creation",
            Self::MockBehavior => "mock_behavior",
            Self::ClockOrRandomness => "clock_or_randomness",
            Self::EnvironmentVariable => "environment_variable",
            Self::FileSystemExpectation => "file_system_expectation",
            Self::NetworkExpectation => "network_expectation",
            Self::RuntimeDependency => "runtime_dependency",
            Self::UnsupportedPath => "unsupported_path",
        }
    }
}

/// Assumption risk class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionRiskClass {
    /// Low review risk.
    Low,
    /// Medium review risk.
    Medium,
    /// High review risk.
    High,
}

/// Generated-test diff risk class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedTestDiffClass {
    /// New or changed logic assertion.
    LogicAssertion,
    /// Helper, fixture, mock, or harness addition.
    HelperOrFixtureAddition,
    /// Snapshot or golden update.
    SnapshotOrGoldenUpdate,
}

impl GeneratedTestDiffClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LogicAssertion => "logic_assertion",
            Self::HelperOrFixtureAddition => "helper_or_fixture_addition",
            Self::SnapshotOrGoldenUpdate => "snapshot_or_golden_update",
        }
    }
}

/// Sandbox execution locus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxTargetClass {
    /// Local test sandbox.
    LocalSandbox,
    /// Remote/helper sandbox.
    RemoteHelper,
    /// Policy-constrained or reduced sandbox.
    ConstrainedMode,
}

impl SandboxTargetClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSandbox => "local_sandbox",
            Self::RemoteHelper => "remote_helper",
            Self::ConstrainedMode => "constrained_mode",
        }
    }
}

/// Sandbox validation outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxOutcomeClass {
    /// Validation passed in the stated sandbox.
    Passed,
    /// Validation failed.
    Failed,
    /// Validation timed out.
    TimedOut,
    /// Validation was skipped.
    Skipped,
    /// Validation was blocked by workspace trust or policy.
    BlockedByTrust,
}

impl SandboxOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::TimedOut => "timed_out",
            Self::Skipped => "skipped",
            Self::BlockedByTrust => "blocked_by_trust",
        }
    }
}

/// Coverage-impact truth class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageImpactClass {
    /// Impact was measured from a comparable post-run coverage lane.
    Measured,
    /// Impact is estimated only.
    Estimated,
    /// Impact refers to stale coverage.
    Stale,
    /// The target does not support coverage impact.
    Unsupported,
    /// Coverage is not comparable for this target family.
    NotComparable,
}

impl CoverageImpactClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Estimated => "estimated",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
            Self::NotComparable => "not_comparable",
        }
    }

    const fn is_measured(self) -> bool {
        matches!(self, Self::Measured)
    }
}

/// Bulk-apply posture for generated test candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BulkApplyPostureClass {
    /// Bulk apply is unavailable because review material is not inspectable.
    UnavailableUntilInspectable,
    /// Bulk apply is unavailable because generated tests remain review-first.
    UnavailableReviewRequired,
    /// Bulk apply is available and must be rejected for this stable packet.
    Available,
}

impl BulkApplyPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnavailableUntilInspectable => "unavailable_until_inspectable",
            Self::UnavailableReviewRequired => "unavailable_review_required",
            Self::Available => "available",
        }
    }
}

/// Consumer surface that projects AI test-generation truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTestGenerationConsumerSurface {
    /// UI suggestion card.
    SuggestionCard,
    /// Test explorer overlay.
    TestExplorerOverlay,
    /// Coverage view.
    CoverageView,
    /// CLI or headless export.
    CliHeadless,
    /// Support export.
    SupportExport,
    /// Release evidence packet.
    ReleasePacket,
}

impl AiTestGenerationConsumerSurface {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuggestionCard => "suggestion_card",
            Self::TestExplorerOverlay => "test_explorer_overlay",
            Self::CoverageView => "coverage_view",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
        }
    }

    /// Surfaces required before this lane can claim stable consumer parity.
    pub const fn required_surfaces() -> [Self; 6] {
        [
            Self::SuggestionCard,
            Self::TestExplorerOverlay,
            Self::CoverageView,
            Self::CliHeadless,
            Self::SupportExport,
            Self::ReleasePacket,
        ]
    }
}

/// Export-safe lineage carried by every stable object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTestGenerationLineage {
    /// Stable object id.
    pub object_id: String,
    /// Evidence refs that ground this object.
    pub evidence_refs: Vec<String>,
    /// Export lineage refs that let support packets reopen the object.
    pub export_lineage_refs: Vec<String>,
    /// Validation status of this object.
    pub validation_status: AiTestGenerationValidationStatus,
}

impl AiTestGenerationLineage {
    fn is_complete(&self) -> bool {
        !self.object_id.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self
                .evidence_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
            && !self.export_lineage_refs.is_empty()
            && self
                .export_lineage_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
    }
}

/// Test-generation brief explaining why a candidate exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationBrief {
    /// Stable candidate id shared by all candidate-owned objects.
    pub candidate_id: String,
    /// Brief lineage.
    pub lineage: AiTestGenerationLineage,
    /// Concrete trigger class.
    pub trigger_class: TestProposalTriggerClass,
    /// Ref to the motivating trigger.
    pub trigger_ref: String,
    /// Target symbol/file/test-item refs.
    pub target_refs: Vec<String>,
    /// Requested test type label.
    pub requested_test_type: String,
    /// Language/framework target label.
    pub framework_target: String,
    /// Confidence class shown to reviewers.
    pub confidence_class: TestCandidateConfidenceClass,
    /// Flaky-risk class shown to reviewers.
    pub flaky_risk_class: TestCandidateFlakyRiskClass,
    /// True when low-confidence/flaky labels survive a sandbox pass.
    pub risk_labels_retained_after_pass: bool,
}

/// One assumption in an assumption-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionReviewRow {
    /// Stable assumption id.
    pub assumption_id: String,
    /// Assumption class.
    pub assumption_class: AssumptionClass,
    /// Ref that grounds the assumption.
    pub source_ref: String,
    /// Review risk class.
    pub risk_class: AssumptionRiskClass,
    /// True when the row is inspectable before apply.
    pub inspectable_before_apply: bool,
    /// Unsupported-path note or limitation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsupported_note: Option<String>,
}

/// Assumption-review sheet for one generated test candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionReviewSheet {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Sheet lineage.
    pub lineage: AiTestGenerationLineage,
    /// Assumption rows.
    pub rows: Vec<AssumptionReviewRow>,
}

/// One generated-test diff class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTestDiffClassRow {
    /// Stable diff-class row id.
    pub diff_class_id: String,
    /// Diff risk class.
    pub diff_class: GeneratedTestDiffClass,
    /// Opaque refs to changed patch hunks or files.
    pub change_refs: Vec<String>,
    /// Number of files in this class.
    pub file_count: u32,
    /// True when this class is separated from other risk classes.
    pub separated_from_other_classes: bool,
    /// Snapshot/golden baseline ref for snapshot or golden updates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_or_golden_baseline_ref: Option<String>,
}

/// Generated-test diff record for one candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTestDiffRecord {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Diff lineage.
    pub lineage: AiTestGenerationLineage,
    /// Content-addressed patch digest ref.
    pub patch_digest_ref: String,
    /// Review-safe write-scope label.
    pub write_scope_label: String,
    /// Diff class rows.
    pub classes: Vec<GeneratedTestDiffClassRow>,
}

/// Sandbox-validation record for one candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxValidationRecord {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Sandbox validation lineage.
    pub lineage: AiTestGenerationLineage,
    /// Sandbox target class.
    pub target_class: SandboxTargetClass,
    /// Exact target/environment lineage refs.
    pub target_environment_refs: Vec<String>,
    /// Network policy token or ref.
    pub network_policy_ref: String,
    /// File-system policy token or ref.
    pub file_policy_ref: String,
    /// Secret policy token or ref.
    pub secret_policy_ref: String,
    /// Validation outcome.
    pub outcome_class: SandboxOutcomeClass,
    /// Trust or policy blocker reason when blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_by_trust_reason: Option<String>,
    /// Opaque command/run ref that actually executed.
    pub executed_run_ref: String,
    /// Opaque logs ref.
    pub logs_ref: String,
    /// True when rerun is available.
    pub rerun_available: bool,
    /// True when logs can be opened.
    pub open_logs_available: bool,
}

/// Coverage-impact note for one candidate and target family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageImpactNote {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Coverage-impact lineage.
    pub lineage: AiTestGenerationLineage,
    /// Target family label.
    pub target_family: String,
    /// Coverage-impact class.
    pub impact_class: CoverageImpactClass,
    /// Expected covered area refs.
    pub expected_covered_area_refs: Vec<String>,
    /// Measured coverage result ref when impact is measured.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measured_coverage_ref: Option<String>,
    /// Review-safe delta summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta_summary: Option<String>,
    /// True only when this candidate is allowed to count in release or benchmark truth.
    pub counts_for_release_or_benchmark_truth: bool,
}

/// Per-candidate draft-state summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTestCandidateRow {
    /// Stable candidate id.
    pub candidate_id: String,
    /// Review posture.
    pub review_state: TestCandidateReviewState,
    /// Gate record ref from the testing admission schema.
    pub gate_record_ref: String,
    /// Brief object id.
    pub brief_id: String,
    /// Assumption sheet object id.
    pub assumption_sheet_id: String,
    /// Diff object id.
    pub diff_record_id: String,
    /// Sandbox object id.
    pub sandbox_record_id: String,
    /// Coverage-impact object id.
    pub coverage_note_id: String,
    /// True when the current client can inspect the assumption sheet.
    pub assumptions_inspectable: bool,
    /// True when the current client can inspect diff classes.
    pub diff_classes_inspectable: bool,
    /// True when the current client can inspect sandbox state.
    pub sandbox_state_inspectable: bool,
    /// Bulk-apply posture.
    pub bulk_apply_posture: BulkApplyPostureClass,
}

/// One consumer projection row proving packet vocabulary parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTestGenerationConsumerProjection {
    /// Consumer surface.
    pub surface: AiTestGenerationConsumerSurface,
    /// True when the surface preserves draft-class generated-test state.
    pub preserves_draft_class: bool,
    /// True when the surface distinguishes AI-generated from human-authored tests.
    pub distinguishes_ai_generated_tests: bool,
    /// True when the surface preserves diff risk classes.
    pub separates_diff_classes: bool,
    /// True when the surface preserves sandbox validation state.
    pub preserves_sandbox_validation: bool,
    /// True when the surface distinguishes measured, estimated, stale, unsupported, and incomparable impact.
    pub preserves_coverage_impact_class: bool,
    /// Export ref consumed by this surface.
    pub export_ref: String,
}

/// Constructor input for [`AiTestGenerationTruthPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiTestGenerationTruthPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Workspace or workflow id.
    pub workflow_id: String,
    /// Packet display label.
    pub display_label: String,
    /// Candidate rows.
    pub candidates: Vec<AiTestCandidateRow>,
    /// Test-generation briefs.
    pub briefs: Vec<TestGenerationBrief>,
    /// Assumption-review sheets.
    pub assumption_sheets: Vec<AssumptionReviewSheet>,
    /// Generated-test diff records.
    pub generated_diffs: Vec<GeneratedTestDiffRecord>,
    /// Sandbox-validation records.
    pub sandbox_validations: Vec<SandboxValidationRecord>,
    /// Coverage-impact notes.
    pub coverage_impact_notes: Vec<CoverageImpactNote>,
    /// Consumer projections.
    pub consumer_projections: Vec<AiTestGenerationConsumerProjection>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI test-generation truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTestGenerationTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Workspace or workflow id.
    pub workflow_id: String,
    /// Packet display label.
    pub display_label: String,
    /// Candidate rows.
    pub candidates: Vec<AiTestCandidateRow>,
    /// Test-generation briefs.
    pub briefs: Vec<TestGenerationBrief>,
    /// Assumption-review sheets.
    pub assumption_sheets: Vec<AssumptionReviewSheet>,
    /// Generated-test diff records.
    pub generated_diffs: Vec<GeneratedTestDiffRecord>,
    /// Sandbox-validation records.
    pub sandbox_validations: Vec<SandboxValidationRecord>,
    /// Coverage-impact notes.
    pub coverage_impact_notes: Vec<CoverageImpactNote>,
    /// Consumer projections.
    pub consumer_projections: Vec<AiTestGenerationConsumerProjection>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiTestGenerationTruthPacket {
    /// Builds an AI test-generation truth packet from stable-lane input.
    pub fn new(input: AiTestGenerationTruthPacketInput) -> Self {
        Self {
            record_kind: AI_TEST_GENERATION_TRUTH_RECORD_KIND.to_owned(),
            schema_version: AI_TEST_GENERATION_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_id: input.workflow_id,
            display_label: input.display_label,
            candidates: input.candidates,
            briefs: input.briefs,
            assumption_sheets: input.assumption_sheets,
            generated_diffs: input.generated_diffs,
            sandbox_validations: input.sandbox_validations,
            coverage_impact_notes: input.coverage_impact_notes,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates stable AI test-generation truth invariants.
    pub fn validate(&self) -> Vec<AiTestGenerationTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_TEST_GENERATION_TRUTH_RECORD_KIND {
            violations.push(AiTestGenerationTruthViolation::WrongRecordKind);
        }
        if self.schema_version != AI_TEST_GENERATION_TRUTH_SCHEMA_VERSION {
            violations.push(AiTestGenerationTruthViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiTestGenerationTruthViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_candidates(self, &mut violations);
        validate_briefs(self, &mut violations);
        validate_assumptions(self, &mut violations);
        validate_diffs(self, &mut violations);
        validate_sandbox(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_consumers(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai test-generation packet serializes"),
        ) {
            violations.push(AiTestGenerationTruthViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai test-generation packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let draft_candidates = self
            .candidates
            .iter()
            .filter(|candidate| {
                matches!(
                    candidate.review_state,
                    TestCandidateReviewState::DraftOnly
                        | TestCandidateReviewState::ReviewReadyDraft
                        | TestCandidateReviewState::BlockedReviewOnly
                )
            })
            .count();
        let measured_notes = self
            .coverage_impact_notes
            .iter()
            .filter(|note| note.impact_class.is_measured())
            .count();
        let release_counted = self
            .coverage_impact_notes
            .iter()
            .filter(|note| note.counts_for_release_or_benchmark_truth)
            .count();
        let mut out = String::new();
        out.push_str("# AI Test-Generation Assumption And Sandbox Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Workflow: `{}`\n", self.workflow_id));
        out.push_str(&format!("- Candidates: {}\n", self.candidates.len()));
        out.push_str(&format!("- Draft-class candidates: {}\n", draft_candidates));
        out.push_str(&format!("- Briefs: {}\n", self.briefs.len()));
        out.push_str(&format!(
            "- Assumption sheets: {}\n",
            self.assumption_sheets.len()
        ));
        out.push_str(&format!("- Diff records: {}\n", self.generated_diffs.len()));
        out.push_str(&format!(
            "- Sandbox records: {}\n",
            self.sandbox_validations.len()
        ));
        out.push_str(&format!(
            "- Coverage notes: {} ({} measured, {} counted for release or benchmark truth)\n",
            self.coverage_impact_notes.len(),
            measured_notes,
            release_counted
        ));
        out.push_str(&format!(
            "- Consumer projections: {}\n",
            self.consumer_projections.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in AI test-generation export.
#[derive(Debug)]
pub enum AiTestGenerationTruthArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiTestGenerationTruthViolation>),
}

impl fmt::Display for AiTestGenerationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "ai test-generation export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai test-generation export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiTestGenerationTruthArtifactError {}

/// Validation failures emitted by [`AiTestGenerationTruthPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiTestGenerationTruthViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Candidate objects are incomplete or not linked to a row.
    CandidateLinkageIncomplete,
    /// A candidate escaped draft/review-only posture.
    CandidateNotDraftClass,
    /// Current-client inspection state is incomplete.
    CandidateInspectionIncomplete,
    /// Bulk apply is incorrectly available.
    BulkApplyAvailable,
    /// A proposal lacks a concrete trigger or evidence.
    MissingConcreteTrigger,
    /// Low-confidence or flaky-risk labels were dropped after validation.
    RiskLabelsDroppedAfterPass,
    /// Assumption-review sheet is incomplete or not inspectable.
    AssumptionReviewIncomplete,
    /// Generated-test diff classes are incomplete.
    DiffClassIncomplete,
    /// Logic, helper/fixture, or snapshot/golden edits were flattened together.
    DiffClassNotSeparated,
    /// Sandbox validation lineage, target, policy, or action state is incomplete.
    SandboxValidationIncomplete,
    /// Sandbox blocked state lacks a trust/policy reason.
    SandboxBlockedReasonMissing,
    /// Coverage-impact note is incomplete.
    CoverageImpactIncomplete,
    /// Candidate coverage is being counted as release or benchmark truth.
    CandidateCoveragePromoted,
    /// Consumer surface parity is incomplete.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiTestGenerationTruthViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::CandidateLinkageIncomplete => "candidate_linkage_incomplete",
            Self::CandidateNotDraftClass => "candidate_not_draft_class",
            Self::CandidateInspectionIncomplete => "candidate_inspection_incomplete",
            Self::BulkApplyAvailable => "bulk_apply_available",
            Self::MissingConcreteTrigger => "missing_concrete_trigger",
            Self::RiskLabelsDroppedAfterPass => "risk_labels_dropped_after_pass",
            Self::AssumptionReviewIncomplete => "assumption_review_incomplete",
            Self::DiffClassIncomplete => "diff_class_incomplete",
            Self::DiffClassNotSeparated => "diff_class_not_separated",
            Self::SandboxValidationIncomplete => "sandbox_validation_incomplete",
            Self::SandboxBlockedReasonMissing => "sandbox_blocked_reason_missing",
            Self::CoverageImpactIncomplete => "coverage_impact_incomplete",
            Self::CandidateCoveragePromoted => "candidate_coverage_promoted",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in AI test-generation truth export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_ai_test_generation_truth_export(
) -> Result<AiTestGenerationTruthPacket, AiTestGenerationTruthArtifactError> {
    let packet: AiTestGenerationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/ai-test-generation-assumption-and-sandbox-truth/support_export.json"
    )))
    .map_err(AiTestGenerationTruthArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiTestGenerationTruthArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for required in [
        AI_TEST_GENERATION_TRUTH_AI_DOC_REF,
        AI_TEST_GENERATION_TESTING_CONTRACT_REF,
        AI_TEST_GENERATION_GATE_SCHEMA_REF,
        AI_TEST_GENERATION_TRUTH_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiTestGenerationTruthViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_candidates(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    let brief_ids = object_candidate_ids(
        packet
            .briefs
            .iter()
            .map(|brief| (&brief.lineage, &brief.candidate_id)),
    );
    let sheet_ids = object_candidate_ids(
        packet
            .assumption_sheets
            .iter()
            .map(|sheet| (&sheet.lineage, &sheet.candidate_id)),
    );
    let diff_ids = object_candidate_ids(
        packet
            .generated_diffs
            .iter()
            .map(|diff| (&diff.lineage, &diff.candidate_id)),
    );
    let sandbox_ids = object_candidate_ids(
        packet
            .sandbox_validations
            .iter()
            .map(|sandbox| (&sandbox.lineage, &sandbox.candidate_id)),
    );
    let coverage_ids = object_candidate_ids(
        packet
            .coverage_impact_notes
            .iter()
            .map(|coverage| (&coverage.lineage, &coverage.candidate_id)),
    );
    for candidate in &packet.candidates {
        if candidate.candidate_id.trim().is_empty()
            || candidate.gate_record_ref.trim().is_empty()
            || object_mismatch(&brief_ids, &candidate.brief_id, &candidate.candidate_id)
            || object_mismatch(
                &sheet_ids,
                &candidate.assumption_sheet_id,
                &candidate.candidate_id,
            )
            || object_mismatch(
                &diff_ids,
                &candidate.diff_record_id,
                &candidate.candidate_id,
            )
            || object_mismatch(
                &sandbox_ids,
                &candidate.sandbox_record_id,
                &candidate.candidate_id,
            )
            || object_mismatch(
                &coverage_ids,
                &candidate.coverage_note_id,
                &candidate.candidate_id,
            )
        {
            violations.push(AiTestGenerationTruthViolation::CandidateLinkageIncomplete);
            break;
        }
        if !matches!(
            candidate.review_state,
            TestCandidateReviewState::DraftOnly
                | TestCandidateReviewState::ReviewReadyDraft
                | TestCandidateReviewState::BlockedReviewOnly
        ) {
            violations.push(AiTestGenerationTruthViolation::CandidateNotDraftClass);
            break;
        }
        if !candidate.assumptions_inspectable
            || !candidate.diff_classes_inspectable
            || !candidate.sandbox_state_inspectable
        {
            violations.push(AiTestGenerationTruthViolation::CandidateInspectionIncomplete);
        }
        if matches!(
            candidate.bulk_apply_posture,
            BulkApplyPostureClass::UnavailableUntilInspectable
                | BulkApplyPostureClass::UnavailableReviewRequired
        ) {
        } else {
            violations.push(AiTestGenerationTruthViolation::BulkApplyAvailable);
        }
    }
    if packet.candidates.is_empty() {
        violations.push(AiTestGenerationTruthViolation::CandidateLinkageIncomplete);
    }
}

fn validate_briefs(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for brief in &packet.briefs {
        if !brief.lineage.is_complete()
            || !brief.lineage.validation_status.is_inspectable()
            || brief.candidate_id.trim().is_empty()
            || brief.trigger_ref.trim().is_empty()
            || brief.target_refs.is_empty()
            || brief
                .target_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
            || brief.requested_test_type.trim().is_empty()
            || brief.framework_target.trim().is_empty()
        {
            violations.push(AiTestGenerationTruthViolation::MissingConcreteTrigger);
            break;
        }
        if (brief.confidence_class.requires_retained_label()
            || brief.flaky_risk_class.requires_retained_label())
            && !brief.risk_labels_retained_after_pass
        {
            violations.push(AiTestGenerationTruthViolation::RiskLabelsDroppedAfterPass);
            break;
        }
    }
    if packet.briefs.is_empty() {
        violations.push(AiTestGenerationTruthViolation::MissingConcreteTrigger);
    }
}

fn validate_assumptions(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for sheet in &packet.assumption_sheets {
        if !sheet.lineage.is_complete()
            || !sheet.lineage.validation_status.is_inspectable()
            || sheet.candidate_id.trim().is_empty()
            || sheet.rows.is_empty()
        {
            violations.push(AiTestGenerationTruthViolation::AssumptionReviewIncomplete);
            break;
        }
        for row in &sheet.rows {
            if row.assumption_id.trim().is_empty()
                || row.source_ref.trim().is_empty()
                || !row.inspectable_before_apply
                || (row.assumption_class == AssumptionClass::UnsupportedPath
                    && !row
                        .unsupported_note
                        .as_deref()
                        .is_some_and(|note| !note.trim().is_empty()))
            {
                violations.push(AiTestGenerationTruthViolation::AssumptionReviewIncomplete);
                return;
            }
        }
    }
    if packet.assumption_sheets.is_empty() {
        violations.push(AiTestGenerationTruthViolation::AssumptionReviewIncomplete);
    }
}

fn validate_diffs(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for diff in &packet.generated_diffs {
        if !diff.lineage.is_complete()
            || !diff.lineage.validation_status.is_inspectable()
            || diff.candidate_id.trim().is_empty()
            || diff.patch_digest_ref.trim().is_empty()
            || diff.write_scope_label.trim().is_empty()
            || diff.classes.is_empty()
        {
            violations.push(AiTestGenerationTruthViolation::DiffClassIncomplete);
            break;
        }
        for class_row in &diff.classes {
            if class_row.diff_class_id.trim().is_empty()
                || class_row.change_refs.is_empty()
                || class_row
                    .change_refs
                    .iter()
                    .any(|reference| reference.trim().is_empty())
                || class_row.file_count == 0
                || (class_row.diff_class == GeneratedTestDiffClass::SnapshotOrGoldenUpdate
                    && !class_row
                        .snapshot_or_golden_baseline_ref
                        .as_deref()
                        .is_some_and(|reference| !reference.trim().is_empty()))
            {
                violations.push(AiTestGenerationTruthViolation::DiffClassIncomplete);
                return;
            }
            if !class_row.separated_from_other_classes {
                violations.push(AiTestGenerationTruthViolation::DiffClassNotSeparated);
                return;
            }
        }
        let distinct_classes: BTreeSet<GeneratedTestDiffClass> =
            diff.classes.iter().map(|row| row.diff_class).collect();
        if distinct_classes.len() != diff.classes.len() {
            violations.push(AiTestGenerationTruthViolation::DiffClassNotSeparated);
            return;
        }
    }
    if packet.generated_diffs.is_empty() {
        violations.push(AiTestGenerationTruthViolation::DiffClassIncomplete);
    }
}

fn validate_sandbox(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for sandbox in &packet.sandbox_validations {
        if !sandbox.lineage.is_complete()
            || sandbox.candidate_id.trim().is_empty()
            || sandbox.target_environment_refs.is_empty()
            || sandbox
                .target_environment_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
            || sandbox.network_policy_ref.trim().is_empty()
            || sandbox.file_policy_ref.trim().is_empty()
            || sandbox.secret_policy_ref.trim().is_empty()
            || sandbox.executed_run_ref.trim().is_empty()
            || sandbox.logs_ref.trim().is_empty()
            || !sandbox.rerun_available
            || !sandbox.open_logs_available
        {
            violations.push(AiTestGenerationTruthViolation::SandboxValidationIncomplete);
            break;
        }
        if sandbox.outcome_class == SandboxOutcomeClass::BlockedByTrust
            && !sandbox
                .blocked_by_trust_reason
                .as_deref()
                .is_some_and(|reason| !reason.trim().is_empty())
        {
            violations.push(AiTestGenerationTruthViolation::SandboxBlockedReasonMissing);
            break;
        }
    }
    if packet.sandbox_validations.is_empty() {
        violations.push(AiTestGenerationTruthViolation::SandboxValidationIncomplete);
    }
}

fn validate_coverage(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for note in &packet.coverage_impact_notes {
        if !note.lineage.is_complete()
            || !note.lineage.validation_status.is_inspectable()
            || note.candidate_id.trim().is_empty()
            || note.target_family.trim().is_empty()
            || note.expected_covered_area_refs.is_empty()
            || note
                .expected_covered_area_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
            || (note.impact_class.is_measured()
                && (!note
                    .measured_coverage_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.trim().is_empty())
                    || !note
                        .delta_summary
                        .as_deref()
                        .is_some_and(|summary| !summary.trim().is_empty())))
        {
            violations.push(AiTestGenerationTruthViolation::CoverageImpactIncomplete);
            break;
        }
        if note.counts_for_release_or_benchmark_truth {
            violations.push(AiTestGenerationTruthViolation::CandidateCoveragePromoted);
            break;
        }
    }
    if packet.coverage_impact_notes.is_empty() {
        violations.push(AiTestGenerationTruthViolation::CoverageImpactIncomplete);
    }
}

fn validate_consumers(
    packet: &AiTestGenerationTruthPacket,
    violations: &mut Vec<AiTestGenerationTruthViolation>,
) {
    for required in AiTestGenerationConsumerSurface::required_surfaces() {
        if !packet
            .consumer_projections
            .iter()
            .any(|row| row.surface == required)
        {
            violations.push(AiTestGenerationTruthViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
    for row in &packet.consumer_projections {
        if row.export_ref.trim().is_empty()
            || !row.preserves_draft_class
            || !row.distinguishes_ai_generated_tests
            || !row.separates_diff_classes
            || !row.preserves_sandbox_validation
            || !row.preserves_coverage_impact_class
        {
            violations.push(AiTestGenerationTruthViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn object_candidate_ids<'a>(
    lineages: impl Iterator<Item = (&'a AiTestGenerationLineage, &'a String)>,
) -> BTreeMap<String, String> {
    lineages
        .map(|(lineage, candidate_id)| (lineage.object_id.clone(), candidate_id.clone()))
        .collect::<BTreeMap<_, _>>()
}

fn object_mismatch(
    object_candidates: &BTreeMap<String, String>,
    object_id: &str,
    candidate_id: &str,
) -> bool {
    object_candidates
        .get(object_id)
        .is_none_or(|owner_candidate_id| owner_candidate_id != candidate_id)
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
        || lower.contains("raw_prompt")
        || lower.contains("raw_diff")
        || lower.contains("raw_generated")
        || lower.contains("stdout:")
        || lower.contains("stderr:")
        || lower.contains("/users/")
}

#[cfg(test)]
mod tests;
