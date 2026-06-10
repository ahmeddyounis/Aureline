//! AI-assisted test generation, assumption review, sandbox validation, and
//! coverage-impact notes.
//!
//! This module ships the canonical M5 packet for AI-assisted test generation. A
//! generation pass produces **test proposals** for a change, surfaces the
//! **assumptions** each proposal makes, validates the proposals in an isolated
//! **sandbox**, and records **coverage-impact** notes — all without ever letting
//! a generated test apply itself, count as trusted coverage proof, or stand in
//! for a real release/benchmark run. The pass is **read-only**: it never applies
//! a change and never self-promotes a generated test. Every claim a proposal
//! makes must cite evidence by id rather than asserting authority on its own. The
//! packet binds four blocks:
//!
//! - A [`TestProposalsBlock`] presents the candidate tests the pass produced —
//!   each carries a proposal class, a generated-diff risk class, and a review
//!   state that is never auto-applied; binds to a [`DurableAnchor`] on the target
//!   under test; references the sandbox run that validated it; cites the evidence
//!   refs that back it; and flags whether it needs human review. Proposals that
//!   cite no evidence are counted and surfaced rather than hidden, and no proposal
//!   may claim authority beyond its cited evidence.
//! - An [`AssumptionReviewBlock`] presents the assumptions the generated tests
//!   make — each a typed assumption with its confidence and whether it has been
//!   validated. Unvalidated assumptions are counted and surfaced, and any
//!   unvalidated assumption must require human confirmation rather than passing
//!   silently.
//! - A [`SandboxValidationBlock`] presents the sandbox runs that exercised the
//!   proposals — each carries its profile, outcome, and isolation posture. Runs
//!   stay isolated and non-leaking, and a sandbox pass is never silently promoted
//!   into release or benchmark coverage truth.
//! - A [`CoverageImpactBlock`] presents the coverage-impact notes — each carries
//!   a measurement basis (measured or estimated) and a delta direction. Estimated
//!   coverage is labeled as estimated and never presented as measured.
//!
//! The packet references upstream M4/M5 lanes by id rather than embedding their
//! content: it cites the prior canonical
//! [`crate::ai_test_generation`] truth lane for proposal triggers,
//! assumption sheets, sandbox-validation lineage, and coverage impact; the frozen
//! testing-intelligence contract for the admission gate; the sandbox-profiles
//! contract for isolation posture; and the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix. It projects against the frozen context-assembly contract for
//! evidence-citation and omitted-context truth.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw generated test source, raw patch bodies,
//! raw diffs, raw runner logs, raw stdout/stderr, raw symbol names, raw file
//! paths, raw prompt bodies, provider payloads, endpoint URLs, credentials, raw
//! token counts, exact prices, and billing-account ids stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ai/add-ai-assisted-test-generation-assumption-review-sandbox-validation-and-coverage-impact-notes.schema.json`](../../../../schemas/ai/add-ai-assisted-test-generation-assumption-review-sandbox-validation-and-coverage-impact-notes.schema.json).
//! The contract doc is
//! [`docs/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes.md`](../../../../docs/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`GeneratedTestReviewPacket`].
pub const GENERATED_TEST_REVIEW_RECORD_KIND: &str =
    "ai_test_generation_sandbox_coverage_implementation";

/// Schema version for AI generated-test-review records.
pub const GENERATED_TEST_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GENERATED_TEST_REVIEW_SCHEMA_REF: &str =
    "schemas/ai/add-ai-assisted-test-generation-assumption-review-sandbox-validation-and-coverage-impact-notes.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const GENERATED_TEST_REVIEW_DOC_REF: &str =
    "docs/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const GENERATED_TEST_REVIEW_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the prior canonical AI test-generation truth contract.
pub const GENERATED_TEST_REVIEW_TEST_GENERATION_CONTRACT_REF: &str =
    "docs/ai/m4/ai-test-generation-assumption-and-sandbox-truth.md";

/// Repo-relative path of the testing-intelligence and acceptance contract.
pub const GENERATED_TEST_REVIEW_TESTING_CONTRACT_REF: &str =
    "docs/testing/test_intelligence_and_acceptance_contract.md";

/// Repo-relative path of the sandbox-profiles and fallbacks contract.
pub const GENERATED_TEST_REVIEW_SANDBOX_CONTRACT_REF: &str =
    "docs/runtime/sandbox-profiles-and-fallbacks.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const GENERATED_TEST_REVIEW_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const GENERATED_TEST_REVIEW_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes";

/// Repo-relative path of the checked support-export artifact.
pub const GENERATED_TEST_REVIEW_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const GENERATED_TEST_REVIEW_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes.md";

/// Concrete trigger or shape of an AI-generated test proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestProposalClass {
    /// Reproduces a reported bug or regression.
    BugRegression,
    /// Covers a branch or path with no existing coverage.
    UncoveredBranch,
    /// Exercises a changed symbol or signature.
    ChangedSymbol,
    /// Pins a boundary or edge condition.
    BoundaryCondition,
    /// Asserts a property or invariant.
    PropertyInvariant,
    /// Closes a release-facing regression gap.
    ReleaseRegressionGap,
}

impl TestProposalClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BugRegression => "bug_regression",
            Self::UncoveredBranch => "uncovered_branch",
            Self::ChangedSymbol => "changed_symbol",
            Self::BoundaryCondition => "boundary_condition",
            Self::PropertyInvariant => "property_invariant",
            Self::ReleaseRegressionGap => "release_regression_gap",
        }
    }
}

/// Risk class of the diff a generated test proposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedDiffRiskClass {
    /// Adds a new test file only; touches no existing code.
    AdditiveTestOnly,
    /// Edits an existing test.
    TouchesExistingTest,
    /// Touches production code, not just tests.
    TouchesProductionCode,
    /// Touches a protected path. The highest-risk class.
    TouchesProtectedPath,
}

impl GeneratedDiffRiskClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdditiveTestOnly => "additive_test_only",
            Self::TouchesExistingTest => "touches_existing_test",
            Self::TouchesProductionCode => "touches_production_code",
            Self::TouchesProtectedPath => "touches_protected_path",
        }
    }
}

/// Review posture of a generated test proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalReviewState {
    /// Draft only; not yet review-ready.
    DraftOnly,
    /// Review-ready, but remains draft-class.
    ReviewReadyDraft,
    /// Blocked; can only be reviewed or exported.
    BlockedReviewOnly,
    /// The proposal was auto-applied without human review. Forbidden on this lane.
    AutoApplied,
}

impl ProposalReviewState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftOnly => "draft_only",
            Self::ReviewReadyDraft => "review_ready_draft",
            Self::BlockedReviewOnly => "blocked_review_only",
            Self::AutoApplied => "auto_applied",
        }
    }

    /// Whether this state means the proposal was applied without human review.
    pub const fn is_auto_applied(self) -> bool {
        matches!(self, Self::AutoApplied)
    }
}

/// Class of assumption a generated test makes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionClass {
    /// Assumes a particular input shape or schema.
    InputShape,
    /// Assumes a particular environment state.
    EnvironmentState,
    /// Assumes a dependency behaves a certain way.
    DependencyBehavior,
    /// Assumes a timing or ordering relationship.
    TimingOrdering,
    /// Assumes an external service is reachable or stubbed.
    ExternalService,
    /// Assumes a fixture or seed state.
    FixtureState,
}

impl AssumptionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InputShape => "input_shape",
            Self::EnvironmentState => "environment_state",
            Self::DependencyBehavior => "dependency_behavior",
            Self::TimingOrdering => "timing_ordering",
            Self::ExternalService => "external_service",
            Self::FixtureState => "fixture_state",
        }
    }
}

/// Confidence class disclosed for an assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssumptionConfidenceClass {
    /// Fully backed by resolved evidence.
    Grounded,
    /// Backed by evidence but with some inference.
    Probable,
    /// Inferred with weak or no direct evidence.
    Speculative,
}

impl AssumptionConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Grounded => "grounded",
            Self::Probable => "probable",
            Self::Speculative => "speculative",
        }
    }
}

/// Sandbox profile a validation run executed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxProfileClass {
    /// An ephemeral container.
    EphemeralContainer,
    /// An in-process isolate.
    InProcessIsolate,
    /// A run with the network denied.
    NetworkDenied,
    /// A run with a scratch filesystem.
    FilesystemScratch,
}

impl SandboxProfileClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralContainer => "ephemeral_container",
            Self::InProcessIsolate => "in_process_isolate",
            Self::NetworkDenied => "network_denied",
            Self::FilesystemScratch => "filesystem_scratch",
        }
    }
}

/// Outcome class of a sandbox validation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SandboxOutcomeClass {
    /// The run passed.
    Passed,
    /// The run failed an assertion.
    Failed,
    /// The run errored before completing.
    Errored,
    /// The run timed out.
    TimedOut,
    /// The run was skipped.
    Skipped,
}

impl SandboxOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Errored => "errored",
            Self::TimedOut => "timed_out",
            Self::Skipped => "skipped",
        }
    }
}

/// Whether a coverage-impact note is measured or estimated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageMeasurementBasis {
    /// Measured from an instrumented run.
    Measured,
    /// Estimated without a measured run.
    Estimated,
}

impl CoverageMeasurementBasis {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Measured => "measured",
            Self::Estimated => "estimated",
        }
    }

    /// Whether this basis is an estimate rather than a measurement.
    pub const fn is_estimated(self) -> bool {
        matches!(self, Self::Estimated)
    }
}

/// Direction of a coverage-impact delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageDeltaDirection {
    /// Coverage increases.
    Increase,
    /// Coverage is unchanged.
    NoChange,
    /// Coverage decreases.
    Decrease,
    /// The direction is not yet known.
    Unknown,
}

impl CoverageDeltaDirection {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Increase => "increase",
            Self::NoChange => "no_change",
            Self::Decrease => "decrease",
            Self::Unknown => "unknown",
        }
    }
}

/// Strategy a durable anchor uses to bind a proposal to a location.
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

/// Consumer surface that must project this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestGenConsumerSurface {
    /// Desktop test panel.
    DesktopTestPanel,
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

impl TestGenConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopTestPanel,
        Self::DesktopEditorGutter,
        Self::CliHeadless,
        Self::BrowserCompanion,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopTestPanel => "desktop_test_panel",
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
pub enum TestGenSurfaceQualificationClass {
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

impl TestGenSurfaceQualificationClass {
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
pub enum TestGenDowngradeTrigger {
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
    /// A generated test was auto-applied without human review.
    GeneratedTestAutoApplied,
    /// A sandbox pass was treated as release or benchmark coverage truth.
    SandboxTreatedAsReleaseTruth,
    /// Estimated coverage was presented as measured.
    EstimatedCoveragePresentedAsMeasured,
    /// An assumption was surfaced as validated without confirmation.
    UncitedAssumptionSurfaced,
}

impl TestGenDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
        Self::GeneratedTestAutoApplied,
        Self::SandboxTreatedAsReleaseTruth,
        Self::EstimatedCoveragePresentedAsMeasured,
        Self::UncitedAssumptionSurfaced,
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
            Self::GeneratedTestAutoApplied => "generated_test_auto_applied",
            Self::SandboxTreatedAsReleaseTruth => "sandbox_treated_as_release_truth",
            Self::EstimatedCoveragePresentedAsMeasured => {
                "estimated_coverage_presented_as_measured"
            }
            Self::UncitedAssumptionSurfaced => "uncited_assumption_surfaced",
        }
    }
}

/// A durable anchor binding a proposal to a location that survives edits.
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

/// One AI-generated test proposal row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestProposalRow {
    /// Stable proposal id, referenced by the sandbox runs.
    pub proposal_id: String,
    /// Class of this proposal.
    pub proposal_class: TestProposalClass,
    /// Risk class of the diff this proposal carries.
    pub diff_risk: GeneratedDiffRiskClass,
    /// Review posture; never auto-applied.
    pub review_state: ProposalReviewState,
    /// Durable anchor binding this proposal to a location.
    pub anchor: DurableAnchor,
    /// Sandbox run id that validated this proposal. Must reference a known run.
    pub sandbox_run_ref: String,
    /// Evidence refs that back this proposal.
    pub cited_evidence_refs: Vec<String>,
    /// True when the proposal cites at least one evidence ref. Must agree with
    /// `cited_evidence_refs` being non-empty.
    pub evidence_backed: bool,
    /// True when the proposal requires human review before being trusted. Must be
    /// true whenever the proposal cites no evidence.
    pub requires_human_review: bool,
    /// True when the proposal is disclosed. Must be true.
    pub disclosed: bool,
}

/// Proposals block presenting the candidate tests the pass produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestProposalsBlock {
    /// Stable proposal-set id.
    pub proposal_set_id: String,
    /// Count of proposals that cite no evidence. Must equal the actual count of
    /// uncited proposal rows.
    pub uncited_proposals_count: u32,
    /// True when no proposal claims authority beyond its cited evidence. Must be
    /// true.
    pub no_authority_beyond_evidence: bool,
    /// True when the proposals were produced before any apply. Must be true.
    pub produced_before_apply: bool,
    /// True when no proposal is auto-applied. Must be true.
    pub never_auto_applied: bool,
    /// Proposal rows.
    pub proposal_rows: Vec<TestProposalRow>,
}

/// One assumption-review row for a generated test.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionRow {
    /// Stable assumption id.
    pub assumption_id: String,
    /// Class of this assumption.
    pub assumption_class: AssumptionClass,
    /// Disclosed confidence class.
    pub confidence: AssumptionConfidenceClass,
    /// True when this assumption has been validated.
    pub validated: bool,
    /// True when this assumption requires human confirmation. Must be true
    /// whenever the assumption is unvalidated.
    pub requires_human_confirmation: bool,
    /// Opaque ref to the area this assumption covers.
    pub scope_ref: String,
    /// True when the assumption is disclosed. Must be true.
    pub disclosed: bool,
}

/// Assumption-review block presenting the assumptions the tests make.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssumptionReviewBlock {
    /// Stable assumption-sheet id.
    pub assumption_sheet_id: String,
    /// Count of unvalidated assumptions. Must equal the actual count of
    /// unvalidated assumption rows.
    pub unvalidated_assumptions_count: u32,
    /// True when every assumption is surfaced rather than hidden. Must be true.
    pub assumptions_surfaced: bool,
    /// Assumption rows.
    pub assumption_rows: Vec<AssumptionRow>,
}

/// One sandbox validation run row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxRunRow {
    /// Stable run id, referenced by proposal rows.
    pub run_id: String,
    /// Sandbox profile this run executed under.
    pub profile: SandboxProfileClass,
    /// Outcome class of this run.
    pub outcome: SandboxOutcomeClass,
    /// True when this run is isolated from the host. Must be true.
    pub isolated: bool,
    /// True when the run leaked outside the sandbox. Must be false.
    pub leaked_outside_sandbox: bool,
    /// Proposal ids this run validated. Each must appear in the proposal block.
    pub validated_proposal_ids: Vec<String>,
    /// True when the run is disclosed. Must be true.
    pub disclosed: bool,
}

/// Sandbox-validation block presenting the runs that exercised the proposals.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SandboxValidationBlock {
    /// Stable sandbox-session id.
    pub sandbox_session_id: String,
    /// True when every run is isolated. Must be true.
    pub runs_isolated: bool,
    /// True when a sandbox pass is not treated as release/benchmark coverage
    /// truth. Must be true.
    pub sandbox_is_not_release_truth: bool,
    /// Sandbox run rows.
    pub run_rows: Vec<SandboxRunRow>,
}

/// One coverage-impact note row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageImpactRow {
    /// Stable note id.
    pub note_id: String,
    /// Opaque ref to the area this note covers.
    pub target_ref: String,
    /// Whether this note is measured or estimated.
    pub measurement_basis: CoverageMeasurementBasis,
    /// Direction of the coverage delta.
    pub delta_direction: CoverageDeltaDirection,
    /// True when an estimated note is labeled as estimated. Must be true whenever
    /// the measurement basis is estimated.
    pub estimated_labeled: bool,
    /// True when the note is disclosed. Must be true.
    pub disclosed: bool,
}

/// Coverage-impact block presenting the coverage notes the pass produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageImpactBlock {
    /// Stable impact-set id.
    pub impact_set_id: String,
    /// Count of estimated notes. Must equal the actual count of estimated note
    /// rows.
    pub estimated_notes_count: u32,
    /// True when no estimated note is presented as measured. Must be true.
    pub no_estimate_as_measured: bool,
    /// Coverage-impact note rows.
    pub impact_rows: Vec<CoverageImpactRow>,
}

/// One cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenSurfaceParityRow {
    /// Consumer surface this row covers.
    pub surface: TestGenConsumerSurface,
    /// True when this surface shows the proposals.
    pub shows_proposals: bool,
    /// True when this surface shows the assumption review.
    pub shows_assumptions: bool,
    /// True when this surface shows the sandbox validation.
    pub shows_sandbox: bool,
    /// True when this surface shows the coverage impact.
    pub shows_coverage: bool,
    /// True when this surface is reachable for this packet.
    pub reachable: bool,
    /// Qualification class for this surface projection.
    pub qualification: TestGenSurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

/// Constructor input for [`GeneratedTestReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedTestReviewPacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical generation-pass id shared across surfaces and evidence.
    pub generation_pass_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Proposals block.
    pub proposals: TestProposalsBlock,
    /// Assumption-review block.
    pub assumptions: AssumptionReviewBlock,
    /// Sandbox-validation block.
    pub sandbox: SandboxValidationBlock,
    /// Coverage-impact block.
    pub coverage_impact: CoverageImpactBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<TestGenSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<TestGenDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI generated-test-review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedTestReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical generation-pass id shared across surfaces and evidence.
    pub generation_pass_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Proposals block.
    pub proposals: TestProposalsBlock,
    /// Assumption-review block.
    pub assumptions: AssumptionReviewBlock,
    /// Sandbox-validation block.
    pub sandbox: SandboxValidationBlock,
    /// Coverage-impact block.
    pub coverage_impact: CoverageImpactBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<TestGenSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<TestGenDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl GeneratedTestReviewPacket {
    /// Builds an AI generated-test-review packet from the stable-lane input.
    pub fn new(input: GeneratedTestReviewPacketInput) -> Self {
        Self {
            record_kind: GENERATED_TEST_REVIEW_RECORD_KIND.to_owned(),
            schema_version: GENERATED_TEST_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generation_pass_id: input.generation_pass_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            proposals: input.proposals,
            assumptions: input.assumptions,
            sandbox: input.sandbox,
            coverage_impact: input.coverage_impact,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the generated-test-review packet's stable-line invariants.
    pub fn validate(&self) -> Vec<GeneratedTestReviewViolation> {
        let mut violations = Vec::new();
        if self.record_kind != GENERATED_TEST_REVIEW_RECORD_KIND {
            violations.push(GeneratedTestReviewViolation::WrongRecordKind);
        }
        if self.schema_version != GENERATED_TEST_REVIEW_SCHEMA_VERSION {
            violations.push(GeneratedTestReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.generation_pass_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(GeneratedTestReviewViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_sandbox(self, &mut violations);
        validate_proposals(self, &mut violations);
        validate_assumptions(self, &mut violations);
        validate_coverage_impact(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("generated test review packet serializes"),
        ) {
            violations.push(GeneratedTestReviewViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("generated test review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# AI Test Generation, Assumption Review, Sandbox Validation, and Coverage Impact\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Generation pass: `{}`\n",
            self.generation_pass_id
        ));
        out.push_str(&format!(
            "- Proposals: `{}` ({} proposals, {} uncited, never auto-applied: {})\n",
            self.proposals.proposal_set_id,
            self.proposals.proposal_rows.len(),
            self.proposals.uncited_proposals_count,
            self.proposals.never_auto_applied
        ));
        out.push_str(&format!(
            "- Assumptions: `{}` ({} assumptions, {} unvalidated, surfaced: {})\n",
            self.assumptions.assumption_sheet_id,
            self.assumptions.assumption_rows.len(),
            self.assumptions.unvalidated_assumptions_count,
            self.assumptions.assumptions_surfaced
        ));
        out.push_str(&format!(
            "- Sandbox: `{}` ({} runs, isolated: {}, not release truth: {})\n",
            self.sandbox.sandbox_session_id,
            self.sandbox.run_rows.len(),
            self.sandbox.runs_isolated,
            self.sandbox.sandbox_is_not_release_truth
        ));
        out.push_str(&format!(
            "- Coverage impact: `{}` ({} notes, {} estimated, no estimate-as-measured: {})\n",
            self.coverage_impact.impact_set_id,
            self.coverage_impact.impact_rows.len(),
            self.coverage_impact.estimated_notes_count,
            self.coverage_impact.no_estimate_as_measured
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

/// Validation failures emitted by [`GeneratedTestReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneratedTestReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The proposal set has no proposals.
    ProposalSetEmpty,
    /// A proposal is missing required identity.
    ProposalIncomplete,
    /// A proposal is disclosed without being marked disclosed.
    HiddenProposal,
    /// A proposal's `evidence_backed` flag disagrees with its citations.
    EvidenceBackedFlagMismatch,
    /// A proposal cites no evidence but does not require human review.
    UncitedProposalNotFlagged,
    /// The disclosed uncited-proposal count disagrees with the actual count.
    UncitedCountMismatch,
    /// A proposal claims authority beyond its cited evidence.
    AuthorityBeyondEvidence,
    /// Proposals were not produced before apply.
    ProposalsNotProducedBeforeApply,
    /// A generated test was auto-applied without human review.
    GeneratedTestAutoApplied,
    /// The proposal block does not assert the never-auto-applied invariant.
    AutoApplyAllowed,
    /// A proposal references no sandbox run.
    ProposalMissingSandboxRef,
    /// A proposal references a sandbox run absent from the sandbox block.
    DanglingSandboxRef,
    /// A durable anchor is missing required identity or refs.
    AnchorIncomplete,
    /// A durable anchor is not marked durable.
    AnchorNotDurable,
    /// A durable anchor drifted or was lost without disclosure.
    AnchorDriftUndisclosed,
    /// The assumption block does not surface its assumptions.
    AssumptionsNotSurfaced,
    /// An assumption is missing required identity or refs.
    AssumptionIncomplete,
    /// An assumption reached the sheet without being disclosed.
    HiddenAssumption,
    /// An unvalidated assumption does not require human confirmation.
    UnvalidatedAssumptionNotFlagged,
    /// The disclosed unvalidated-assumption count disagrees with the actual count.
    UnvalidatedCountMismatch,
    /// The sandbox session has no runs.
    SandboxSessionEmpty,
    /// The sandbox block does not assert the runs-isolated invariant.
    SandboxNotIsolated,
    /// A sandbox pass is treated as release or benchmark coverage truth.
    SandboxTreatedAsReleaseTruth,
    /// A sandbox run is missing required identity.
    SandboxRunIncomplete,
    /// A sandbox run is not isolated.
    SandboxRunNotIsolated,
    /// A sandbox run leaked outside the sandbox.
    SandboxLeak,
    /// A sandbox run reached the block without being disclosed.
    HiddenSandboxRun,
    /// A sandbox run validated a proposal absent from the proposal set.
    DanglingValidatedProposal,
    /// The coverage-impact set has no notes.
    CoverageImpactSetEmpty,
    /// A coverage-impact note is missing required identity or refs.
    CoverageNoteIncomplete,
    /// A coverage-impact note reached the set without being disclosed.
    HiddenCoverageNote,
    /// An estimated coverage note is not labeled as estimated.
    EstimatedCoverageUnlabeled,
    /// The coverage block presents an estimate as measured.
    EstimateAsMeasured,
    /// The disclosed estimated-note count disagrees with the actual count.
    EstimatedCountMismatch,
    /// A consumer surface is not covered by the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims Stable without qualifying for it.
    StableClaimNotQualified,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl GeneratedTestReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ProposalSetEmpty => "proposal_set_empty",
            Self::ProposalIncomplete => "proposal_incomplete",
            Self::HiddenProposal => "hidden_proposal",
            Self::EvidenceBackedFlagMismatch => "evidence_backed_flag_mismatch",
            Self::UncitedProposalNotFlagged => "uncited_proposal_not_flagged",
            Self::UncitedCountMismatch => "uncited_count_mismatch",
            Self::AuthorityBeyondEvidence => "authority_beyond_evidence",
            Self::ProposalsNotProducedBeforeApply => "proposals_not_produced_before_apply",
            Self::GeneratedTestAutoApplied => "generated_test_auto_applied",
            Self::AutoApplyAllowed => "auto_apply_allowed",
            Self::ProposalMissingSandboxRef => "proposal_missing_sandbox_ref",
            Self::DanglingSandboxRef => "dangling_sandbox_ref",
            Self::AnchorIncomplete => "anchor_incomplete",
            Self::AnchorNotDurable => "anchor_not_durable",
            Self::AnchorDriftUndisclosed => "anchor_drift_undisclosed",
            Self::AssumptionsNotSurfaced => "assumptions_not_surfaced",
            Self::AssumptionIncomplete => "assumption_incomplete",
            Self::HiddenAssumption => "hidden_assumption",
            Self::UnvalidatedAssumptionNotFlagged => "unvalidated_assumption_not_flagged",
            Self::UnvalidatedCountMismatch => "unvalidated_count_mismatch",
            Self::SandboxSessionEmpty => "sandbox_session_empty",
            Self::SandboxNotIsolated => "sandbox_not_isolated",
            Self::SandboxTreatedAsReleaseTruth => "sandbox_treated_as_release_truth",
            Self::SandboxRunIncomplete => "sandbox_run_incomplete",
            Self::SandboxRunNotIsolated => "sandbox_run_not_isolated",
            Self::SandboxLeak => "sandbox_leak",
            Self::HiddenSandboxRun => "hidden_sandbox_run",
            Self::DanglingValidatedProposal => "dangling_validated_proposal",
            Self::CoverageImpactSetEmpty => "coverage_impact_set_empty",
            Self::CoverageNoteIncomplete => "coverage_note_incomplete",
            Self::HiddenCoverageNote => "hidden_coverage_note",
            Self::EstimatedCoverageUnlabeled => "estimated_coverage_unlabeled",
            Self::EstimateAsMeasured => "estimate_as_measured",
            Self::EstimatedCountMismatch => "estimated_count_mismatch",
            Self::ConsumerSurfaceCoverageMissing => "consumer_surface_coverage_missing",
            Self::StableClaimNotQualified => "stable_claim_not_qualified",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

impl fmt::Display for GeneratedTestReviewViolation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_str())
    }
}

impl Error for GeneratedTestReviewViolation {}

/// Errors emitted when reading the checked-in generated-test-review export.
#[derive(Debug)]
pub enum GeneratedTestReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GeneratedTestReviewViolation>),
}

impl fmt::Display for GeneratedTestReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "generated test review export parse failed: {error}"
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
                    "generated test review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for GeneratedTestReviewArtifactError {}

/// Returns the checked-in AI generated-test-review export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_generated_test_review_export(
) -> Result<GeneratedTestReviewPacket, GeneratedTestReviewArtifactError> {
    let packet: GeneratedTestReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_ai_assisted_test_generation_assumption_review_sandbox_validation_and_coverage_impact_notes/support_export.json"
    )))
    .map_err(GeneratedTestReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GeneratedTestReviewArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    for required in [
        GENERATED_TEST_REVIEW_DOC_REF,
        GENERATED_TEST_REVIEW_SCHEMA_REF,
        GENERATED_TEST_REVIEW_CONTEXT_ASSEMBLY_CONTRACT_REF,
        GENERATED_TEST_REVIEW_TEST_GENERATION_CONTRACT_REF,
        GENERATED_TEST_REVIEW_TESTING_CONTRACT_REF,
        GENERATED_TEST_REVIEW_SANDBOX_CONTRACT_REF,
        GENERATED_TEST_REVIEW_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(GeneratedTestReviewViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_proposals(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    let proposals = &packet.proposals;
    if proposals.proposal_set_id.trim().is_empty() || proposals.proposal_rows.is_empty() {
        violations.push(GeneratedTestReviewViolation::ProposalSetEmpty);
        return;
    }
    if !proposals.produced_before_apply {
        violations.push(GeneratedTestReviewViolation::ProposalsNotProducedBeforeApply);
    }
    if !proposals.no_authority_beyond_evidence {
        violations.push(GeneratedTestReviewViolation::AuthorityBeyondEvidence);
    }
    if !proposals.never_auto_applied {
        violations.push(GeneratedTestReviewViolation::AutoApplyAllowed);
    }
    let known_runs: std::collections::HashSet<&str> = packet
        .sandbox
        .run_rows
        .iter()
        .map(|run| run.run_id.as_str())
        .collect();
    let mut uncited = 0u32;
    for proposal in &proposals.proposal_rows {
        if proposal.proposal_id.trim().is_empty() {
            violations.push(GeneratedTestReviewViolation::ProposalIncomplete);
        }
        if !proposal.disclosed {
            violations.push(GeneratedTestReviewViolation::HiddenProposal);
        }
        if proposal.review_state.is_auto_applied() {
            violations.push(GeneratedTestReviewViolation::GeneratedTestAutoApplied);
        }
        let has_citations = !proposal.cited_evidence_refs.is_empty();
        if has_citations != proposal.evidence_backed {
            violations.push(GeneratedTestReviewViolation::EvidenceBackedFlagMismatch);
        }
        if !has_citations {
            uncited += 1;
            if !proposal.requires_human_review {
                violations.push(GeneratedTestReviewViolation::UncitedProposalNotFlagged);
            }
        }
        if proposal.sandbox_run_ref.trim().is_empty() {
            violations.push(GeneratedTestReviewViolation::ProposalMissingSandboxRef);
        } else if !known_runs.contains(proposal.sandbox_run_ref.as_str()) {
            violations.push(GeneratedTestReviewViolation::DanglingSandboxRef);
        }
        validate_anchor(&proposal.anchor, violations);
    }
    if proposals.uncited_proposals_count != uncited {
        violations.push(GeneratedTestReviewViolation::UncitedCountMismatch);
    }
}

fn validate_anchor(anchor: &DurableAnchor, violations: &mut Vec<GeneratedTestReviewViolation>) {
    if anchor.anchor_id.trim().is_empty()
        || anchor.target_ref.trim().is_empty()
        || anchor.scope_ref.trim().is_empty()
    {
        violations.push(GeneratedTestReviewViolation::AnchorIncomplete);
    }
    if !anchor.durable {
        violations.push(GeneratedTestReviewViolation::AnchorNotDurable);
    }
    let disturbed = anchor.state.is_disturbed() || anchor.drift_detected;
    if disturbed && !anchor.rebind_disclosed {
        violations.push(GeneratedTestReviewViolation::AnchorDriftUndisclosed);
    }
}

fn validate_assumptions(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    let assumptions = &packet.assumptions;
    if !assumptions.assumptions_surfaced {
        violations.push(GeneratedTestReviewViolation::AssumptionsNotSurfaced);
    }
    let mut unvalidated = 0u32;
    for assumption in &assumptions.assumption_rows {
        if assumption.assumption_id.trim().is_empty() || assumption.scope_ref.trim().is_empty() {
            violations.push(GeneratedTestReviewViolation::AssumptionIncomplete);
        }
        if !assumption.disclosed {
            violations.push(GeneratedTestReviewViolation::HiddenAssumption);
        }
        if !assumption.validated {
            unvalidated += 1;
            if !assumption.requires_human_confirmation {
                violations.push(GeneratedTestReviewViolation::UnvalidatedAssumptionNotFlagged);
            }
        }
    }
    if assumptions.unvalidated_assumptions_count != unvalidated {
        violations.push(GeneratedTestReviewViolation::UnvalidatedCountMismatch);
    }
}

fn validate_sandbox(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    let sandbox = &packet.sandbox;
    if sandbox.sandbox_session_id.trim().is_empty() || sandbox.run_rows.is_empty() {
        violations.push(GeneratedTestReviewViolation::SandboxSessionEmpty);
        return;
    }
    if !sandbox.runs_isolated {
        violations.push(GeneratedTestReviewViolation::SandboxNotIsolated);
    }
    if !sandbox.sandbox_is_not_release_truth {
        violations.push(GeneratedTestReviewViolation::SandboxTreatedAsReleaseTruth);
    }
    let known_proposals: std::collections::HashSet<&str> = packet
        .proposals
        .proposal_rows
        .iter()
        .map(|proposal| proposal.proposal_id.as_str())
        .collect();
    for run in &sandbox.run_rows {
        if run.run_id.trim().is_empty() {
            violations.push(GeneratedTestReviewViolation::SandboxRunIncomplete);
        }
        if !run.isolated {
            violations.push(GeneratedTestReviewViolation::SandboxRunNotIsolated);
        }
        if run.leaked_outside_sandbox {
            violations.push(GeneratedTestReviewViolation::SandboxLeak);
        }
        if !run.disclosed {
            violations.push(GeneratedTestReviewViolation::HiddenSandboxRun);
        }
        for proposal_id in &run.validated_proposal_ids {
            if !known_proposals.contains(proposal_id.as_str()) {
                violations.push(GeneratedTestReviewViolation::DanglingValidatedProposal);
            }
        }
    }
}

fn validate_coverage_impact(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    let coverage = &packet.coverage_impact;
    if coverage.impact_set_id.trim().is_empty() || coverage.impact_rows.is_empty() {
        violations.push(GeneratedTestReviewViolation::CoverageImpactSetEmpty);
        return;
    }
    if !coverage.no_estimate_as_measured {
        violations.push(GeneratedTestReviewViolation::EstimateAsMeasured);
    }
    let mut estimated = 0u32;
    for note in &coverage.impact_rows {
        if note.note_id.trim().is_empty() || note.target_ref.trim().is_empty() {
            violations.push(GeneratedTestReviewViolation::CoverageNoteIncomplete);
        }
        if !note.disclosed {
            violations.push(GeneratedTestReviewViolation::HiddenCoverageNote);
        }
        if note.measurement_basis.is_estimated() {
            estimated += 1;
            if !note.estimated_labeled {
                violations.push(GeneratedTestReviewViolation::EstimatedCoverageUnlabeled);
            }
        }
    }
    if coverage.estimated_notes_count != estimated {
        violations.push(GeneratedTestReviewViolation::EstimatedCountMismatch);
    }
}

fn validate_consumer_surface_parity(
    packet: &GeneratedTestReviewPacket,
    violations: &mut Vec<GeneratedTestReviewViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(GeneratedTestReviewViolation::StableClaimNotQualified);
        }
    }
    for required in TestGenConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations.push(GeneratedTestReviewViolation::ConsumerSurfaceCoverageMissing);
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
