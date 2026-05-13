//! Test identity, attempt ledger, watch-state, and imported-CI alpha records.
//!
//! This module is the runtime-owned alpha projection that connects pytest
//! launch-wedge contracts to durable test identity, session plans, append-only
//! attempt ledgers, watch-controller state, imported-CI read-only evidence, and
//! exportable support rows. It consumes the existing pytest run contract and
//! execution-context object instead of minting runner-specific truth.

use serde::{Deserialize, Serialize};

use crate::discovery::pytest::{PytestLaunchReadiness, PytestRunContract, PytestSelectionKind};
use crate::execution_context::ExecutionContext;

/// Schema version emitted for test-attempt alpha records.
pub const TEST_ATTEMPT_ALPHA_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for a full launch-wedge packet.
pub const TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND: &str = "test_attempt_alpha_packet";
/// Stable record-kind tag for a test-item identity projection.
pub const TEST_ITEM_IDENTITY_PROJECTION_RECORD_KIND: &str = "test_item_identity_projection";
/// Stable record-kind tag for a session plan.
pub const TEST_SESSION_PLAN_RECORD_KIND: &str = "test_session_plan";
/// Stable record-kind tag for one append-only attempt record.
pub const TEST_ATTEMPT_RECORD_KIND: &str = "test_attempt_record";
/// Stable record-kind tag for the alpha watch-controller projection.
pub const TEST_WATCH_CONTROLLER_RECORD_KIND: &str = "test_watch_controller";
/// Stable record-kind tag for the imported-CI projection.
pub const IMPORTED_CI_PROJECTION_RECORD_KIND: &str = "imported_ci_projection";
/// Stable record-kind tag for the stability verdict.
pub const TEST_STABILITY_VERDICT_RECORD_KIND: &str = "test_stability_verdict";
/// Stable record-kind tag for the launch-wedge consumer projection.
pub const TEST_LAUNCH_WEDGE_PROJECTION_RECORD_KIND: &str = "test_launch_wedge_projection";
/// Stable record-kind tag for support/export packets.
pub const TEST_ATTEMPT_SUPPORT_EXPORT_RECORD_KIND: &str = "test_attempt_support_export";

/// Identity stability for a launch-wedge test row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIdentityStability {
    /// A durable canonical item id exists and may be reused across reruns.
    Stable,
    /// Discovery is partial or dynamic, so rerun/debug must keep the caveat visible.
    DynamicPartialDiscovery,
    /// The row is imported evidence and cannot become local truth by itself.
    ImportedReadOnly,
    /// The prior identity needs an explicit remap before rerun/debug.
    RemapReviewRequired,
    /// Identity cannot be classified; mutating actions fail closed.
    IdentityUnknownRequiresReview,
}

impl TestIdentityStability {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::DynamicPartialDiscovery => "dynamic_partial_discovery",
            Self::ImportedReadOnly => "imported_read_only",
            Self::RemapReviewRequired => "remap_review_required",
            Self::IdentityUnknownRequiresReview => "identity_unknown_requires_review",
        }
    }
}

/// User-visible mode for a test session plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSessionMode {
    /// Run the selected tests once.
    RunSelectedTests,
    /// Keep a watch session armed for the selected scope.
    WatchTests,
    /// Rerun only failures from a predecessor attempt.
    RerunFailedTests,
    /// Launch a debug session from a test row.
    DebugFromTest,
    /// Project imported provider CI evidence as read-only or parity-qualified truth.
    ImportProviderCiResults,
}

impl TestSessionMode {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunSelectedTests => "run_selected_tests",
            Self::WatchTests => "watch_tests",
            Self::RerunFailedTests => "rerun_failed_tests",
            Self::DebugFromTest => "debug_from_test",
            Self::ImportProviderCiResults => "import_provider_ci_results",
        }
    }
}

/// Attempt kind inside a test session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestAttemptKind {
    /// First execution attempt for a planned test session.
    InitialTestRun,
    /// One cycle produced by watch mode.
    WatchCycle,
    /// Failed-only rerun derived from a predecessor attempt.
    RerunFailed,
    /// Debug launch derived from a test attempt.
    DebugFromTest,
    /// Provider CI evidence imported into the session.
    ProviderCiImport,
    /// Local rerun used to compare imported provider evidence.
    LocalParityRerun,
    /// Support or release packet reconstruction attempt.
    Reconstruction,
}

impl TestAttemptKind {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialTestRun => "initial_test_run",
            Self::WatchCycle => "watch_cycle",
            Self::RerunFailed => "rerun_failed",
            Self::DebugFromTest => "debug_from_test",
            Self::ProviderCiImport => "provider_ci_import",
            Self::LocalParityRerun => "local_parity_rerun",
            Self::Reconstruction => "reconstruction",
        }
    }
}

/// Attempt result class for the alpha ledger.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestAttemptResultState {
    /// The attempt is queued but has not started.
    Queued,
    /// The attempt is running.
    Running,
    /// The attempt passed in its declared target context.
    Passed,
    /// The attempt failed in its declared target context.
    Failed,
    /// Imported provider evidence reported failure.
    ImportedFailed,
    /// Imported evidence is stale or no longer comparable.
    ImportedStale,
    /// The attempt is blocked before execution.
    Blocked,
    /// The result cannot be classified; automatic green roll-up is blocked.
    UnknownRequiresReview,
}

impl TestAttemptResultState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::ImportedFailed => "imported_failed",
            Self::ImportedStale => "imported_stale",
            Self::Blocked => "blocked",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Controlled watch-state vocabulary for the alpha test contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestWatchState {
    /// Watch is actively running and current inside its evidence window.
    Running,
    /// Watch has observed input and is buffering a bounded batch.
    Buffering,
    /// Watch is intentionally paused or inactive.
    Paused,
    /// Watch is running with a visible fidelity reduction.
    Degraded,
    /// Watch evidence is stale and must not render as current.
    Stale,
}

impl TestWatchState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Buffering => "buffering",
            Self::Paused => "paused",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
        }
    }
}

/// Reason a watch controller is not fully live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestWatchDegradationReason {
    /// No degradation is known.
    None,
    /// Watch is paused because the current session is not a watch session.
    NotAWatchSession,
    /// The adapter can only provide partial discovery.
    PartialDiscovery,
    /// A bounded watch batch is buffering.
    BufferingBatch,
    /// Imported provider evidence cannot arm a local watch controller.
    ImportedReadOnlyEvidence,
    /// Source or target drift makes the current evidence stale.
    SourceOrTargetDrift,
    /// The target is unreachable or disconnected.
    TargetUnavailable,
}

impl TestWatchDegradationReason {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NotAWatchSession => "not_a_watch_session",
            Self::PartialDiscovery => "partial_discovery",
            Self::BufferingBatch => "buffering_batch",
            Self::ImportedReadOnlyEvidence => "imported_read_only_evidence",
            Self::SourceOrTargetDrift => "source_or_target_drift",
            Self::TargetUnavailable => "target_unavailable",
        }
    }
}

/// Source drift between attempts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSourceDriftState {
    /// No source drift is known.
    NoSourceDrift,
    /// Source changed and a current-context rerun is required.
    SourceChangedRequiresCurrentContextRerun,
    /// Imported provider evidence has no local source binding yet.
    ProviderOnlyNoLocalSource,
    /// Drift is unknown and must fail closed.
    SourceDriftUnknownRequiresReview,
}

impl TestSourceDriftState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSourceDrift => "no_source_drift",
            Self::SourceChangedRequiresCurrentContextRerun => {
                "source_changed_requires_current_context_rerun"
            }
            Self::ProviderOnlyNoLocalSource => "provider_only_no_local_source",
            Self::SourceDriftUnknownRequiresReview => "source_drift_unknown_requires_review",
        }
    }
}

/// Imported-CI projection class for a session or attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedCiProjectionClass {
    /// No imported CI evidence participates.
    NotImportedCi,
    /// Provider evidence is authoritative for the provider but read-only locally.
    AuthoritativeImportedReadOnly,
    /// Provider or mirrored evidence is outside its freshness/comparability window.
    StaleImportedReadOnly,
    /// A fresh local attempt reconfirmed the imported evidence relationship.
    FreshLocalReconfirmation,
    /// The projection cannot be classified; automatic actions fail closed.
    ImportedCiProjectionUnknownRequiresReview,
}

impl ImportedCiProjectionClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotImportedCi => "not_imported_ci",
            Self::AuthoritativeImportedReadOnly => "authoritative_imported_read_only",
            Self::StaleImportedReadOnly => "stale_imported_read_only",
            Self::FreshLocalReconfirmation => "fresh_local_reconfirmation",
            Self::ImportedCiProjectionUnknownRequiresReview => {
                "imported_ci_projection_unknown_requires_review"
            }
        }
    }
}

/// Authority class for one imported evidence signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedSignalAuthority {
    /// No imported signal is present.
    None,
    /// The signal is authoritative imported evidence only.
    AuthoritativeImportedEvidence,
    /// The signal is imported but stale.
    StaleImportedEvidence,
    /// A fresh local rerun reconfirmed the imported signal relationship.
    FreshLocalReconfirmation,
}

impl ImportedSignalAuthority {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AuthoritativeImportedEvidence => "authoritative_imported_evidence",
            Self::StaleImportedEvidence => "stale_imported_evidence",
            Self::FreshLocalReconfirmation => "fresh_local_reconfirmation",
        }
    }
}

/// Coverage merge class for local and imported evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageMergeClass {
    /// Coverage was not requested.
    NotRequested,
    /// Imported or local coverage exists but only partially covers the scope.
    CoveragePartial,
    /// Compatible coverage evidence was merged for the declared scope.
    CoverageMerged,
    /// Coverage exists only as authoritative imported evidence.
    AuthoritativeImportedEvidence,
    /// Coverage exists only as stale imported evidence.
    StaleImportedEvidence,
    /// Coverage merge status cannot be classified.
    CoverageUnknownRequiresReview,
}

impl CoverageMergeClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::CoveragePartial => "coverage_partial",
            Self::CoverageMerged => "coverage_merged",
            Self::AuthoritativeImportedEvidence => "authoritative_imported_evidence",
            Self::StaleImportedEvidence => "stale_imported_evidence",
            Self::CoverageUnknownRequiresReview => "coverage_unknown_requires_review",
        }
    }
}

/// Flaky/stability verdict state shared by UI and export rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FlakyVerdictState {
    /// Intermittent behavior is suspected but not reproduced comparably.
    SuspectedFlaky,
    /// Comparable attempts reproduced divergent outcomes.
    ReproducedFlaky,
    /// Prior flaky or muted state cleared through the evidence window.
    StableAgain,
    /// Delivery or execution is muted.
    Muted,
    /// Evidence is missing, stale, or contradictory.
    Unknown,
}

impl FlakyVerdictState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::StableAgain => "stable_again",
            Self::Muted => "muted",
            Self::Unknown => "unknown",
        }
    }
}

/// Snapshot review state for one attempt or projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotReviewState {
    /// Snapshot review is not in scope.
    NotRequired,
    /// Snapshot or golden evidence requires review.
    SnapshotReviewRequired,
    /// Snapshot review is blocked by policy or missing preview.
    SnapshotReviewBlocked,
    /// Snapshot review was completed for the declared scope.
    SnapshotReviewCompleted,
    /// Snapshot review state cannot be classified.
    SnapshotReviewUnknownRequiresReview,
}

impl SnapshotReviewState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::SnapshotReviewRequired => "snapshot_review_required",
            Self::SnapshotReviewBlocked => "snapshot_review_blocked",
            Self::SnapshotReviewCompleted => "snapshot_review_completed",
            Self::SnapshotReviewUnknownRequiresReview => "snapshot_review_unknown_requires_review",
        }
    }
}

/// AI test-generation admission state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiTestGenerationGateState {
    /// AI test generation was not requested.
    NotRequested,
    /// AI-generated tests may only proceed through preview/review.
    PreviewRequired,
    /// AI test generation is blocked for the declared scope.
    AiTestGenerationBlocked,
    /// Gate state cannot be classified.
    AiTestGenerationUnknownRequiresReview,
}

impl AiTestGenerationGateState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::PreviewRequired => "preview_required",
            Self::AiTestGenerationBlocked => "ai_test_generation_blocked",
            Self::AiTestGenerationUnknownRequiresReview => {
                "ai_test_generation_unknown_requires_review"
            }
        }
    }
}

/// Surface that consumes the launch-wedge projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestConsumerSurface {
    /// Runtime launch wedge.
    LaunchWedge,
    /// Shell test tree.
    TestTree,
    /// Inline editor result marker.
    EditorInline,
    /// CLI structured output.
    CliOutput,
    /// Support/export packet.
    SupportExport,
}

impl TestConsumerSurface {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchWedge => "launch_wedge",
            Self::TestTree => "test_tree",
            Self::EditorInline => "editor_inline",
            Self::CliOutput => "cli_output",
            Self::SupportExport => "support_export",
        }
    }
}

/// Runtime projection of durable test-item identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestItemIdentityProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable identity projection id.
    pub identity_projection_id: String,
    /// Stability class for this item or selection.
    pub identity_stability: TestIdentityStability,
    /// Stable stability token.
    pub identity_stability_token: String,
    /// Canonical test item ref when the runner can name one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_test_item_ref: Option<String>,
    /// Selector ref safe for support/export surfaces.
    pub selector_ref: String,
    /// Digest of the display label, never the raw label body.
    pub selection_label_digest: String,
    /// Digest refs for covered source refs.
    pub source_ref_digests: Vec<String>,
    /// True when rerun/debug must show a remap or widening review first.
    pub remap_or_widening_review_required: bool,
    /// Export-safe identity summary.
    pub support_summary: String,
}

/// Session plan presented by the launch wedge and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSessionPlan {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable session id.
    pub test_session_id: String,
    /// Stable session-plan id.
    pub session_plan_id: String,
    /// Session mode.
    pub session_mode: TestSessionMode,
    /// Stable session-mode token.
    pub session_mode_token: String,
    /// Ref to the identity projection backing the selection.
    pub identity_projection_ref: String,
    /// Canonical item refs included in the selection.
    pub canonical_test_item_refs: Vec<String>,
    /// Execution-context ref used by the plan.
    pub execution_context_ref: String,
    /// Target id used by the plan.
    pub target_id: String,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Retry policy token for this alpha plan.
    pub retry_policy_token: String,
    /// Watch policy token for this alpha plan.
    pub watch_policy_token: String,
    /// Selection widening policy token.
    pub selection_widening_rule_token: String,
    /// True when the plan can be exported without raw runner payloads.
    pub export_safe: bool,
    /// Export-safe plan summary.
    pub support_summary: String,
}

/// One append-only attempt in a test session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestAttemptRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable attempt id.
    pub test_attempt_id: String,
    /// Parent test session ref.
    pub parent_test_session_ref: String,
    /// One-based attempt index.
    pub attempt_index: u32,
    /// Attempt kind.
    pub attempt_kind: TestAttemptKind,
    /// Stable attempt-kind token.
    pub attempt_kind_token: String,
    /// Result state.
    pub result_state: TestAttemptResultState,
    /// Stable result-state token.
    pub result_state_token: String,
    /// Predecessor attempt ref when this attempt derives from one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_attempt_ref: Option<String>,
    /// Originating provider or imported attempt ref when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_attempt_ref: Option<String>,
    /// Execution attempt ref on the generic execution rail.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_attempt_ref: Option<String>,
    /// Execution-context ref used at attempt open.
    pub execution_context_ref: String,
    /// Target id used at attempt open.
    pub target_id: String,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical item refs covered by this attempt.
    pub canonical_test_item_refs: Vec<String>,
    /// Selector ref covered by this attempt.
    pub selector_ref: String,
    /// Source drift state.
    pub source_drift_state: TestSourceDriftState,
    /// Stable source-drift token.
    pub source_drift_state_token: String,
    /// Watch state at attempt open.
    pub watch_state_at_attempt: TestWatchState,
    /// Stable watch-state token.
    pub watch_state_at_attempt_token: String,
    /// Imported-CI projection class for this attempt.
    pub imported_ci_projection_class: ImportedCiProjectionClass,
    /// Stable imported-CI projection token.
    pub imported_ci_projection_token: String,
    /// Coverage merge class.
    pub coverage_merge_class: CoverageMergeClass,
    /// Stable coverage token.
    pub coverage_merge_token: String,
    /// Flaky/stability verdict state for this attempt.
    pub flaky_verdict_state: FlakyVerdictState,
    /// Stable flaky/stability token.
    pub flaky_verdict_token: String,
    /// Snapshot review state.
    pub snapshot_review_state: SnapshotReviewState,
    /// Stable snapshot-review token.
    pub snapshot_review_token: String,
    /// AI test-generation gate state.
    pub ai_test_generation_gate_state: AiTestGenerationGateState,
    /// Stable AI gate token.
    pub ai_test_generation_gate_token: String,
    /// Artifact refs retained on governed artifact rails.
    pub artifact_refs: Vec<String>,
    /// Raw event refs retained on governed raw-event rails.
    pub raw_event_refs: Vec<String>,
    /// Local rerun plan that can refresh imported or stale evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_rerun_plan_ref: Option<String>,
    /// Export-safe attempt summary.
    pub support_summary: String,
}

impl TestAttemptRecord {
    /// Returns controlled vocabulary tokens contributed by this attempt.
    pub fn controlled_state_tokens(&self) -> Vec<String> {
        vec![
            self.result_state_token.clone(),
            self.watch_state_at_attempt_token.clone(),
            self.imported_ci_projection_token.clone(),
            self.coverage_merge_token.clone(),
            self.flaky_verdict_token.clone(),
            self.snapshot_review_token.clone(),
            self.ai_test_generation_gate_token.clone(),
        ]
    }
}

/// Watch-controller projection for the test session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestWatchController {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable watch-controller id.
    pub watch_controller_id: String,
    /// Parent session ref.
    pub test_session_ref: String,
    /// Controlled watch state.
    pub watch_state: TestWatchState,
    /// Stable watch-state token.
    pub watch_state_token: String,
    /// Degradation or pause reason.
    pub degradation_reason: TestWatchDegradationReason,
    /// Stable degradation-reason token.
    pub degradation_reason_token: String,
    /// Latest attempt ref visible on consumers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Last successful attempt ref when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_attempt_ref: Option<String>,
    /// Number of buffered or backlogged changes.
    pub buffered_change_count: u32,
    /// True when a live state can be shown as current.
    pub current_truth_claim_allowed: bool,
    /// Export-safe watch summary.
    pub support_summary: String,
}

/// Imported-CI read-only projection for the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedCiProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub imported_ci_projection_id: String,
    /// Parent session ref.
    pub test_session_ref: String,
    /// Imported-CI projection class.
    pub projection_class: ImportedCiProjectionClass,
    /// Stable projection token.
    pub projection_token: String,
    /// True when imported evidence is read-only and cannot mutate local truth.
    pub read_only_imported_evidence: bool,
    /// Provider run ref when imported evidence exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_run_ref: Option<String>,
    /// Imported attempt refs.
    pub imported_attempt_refs: Vec<String>,
    /// Fresh local attempts that reconfirm imported evidence.
    pub local_reconfirmation_attempt_refs: Vec<String>,
    /// Coverage signal authority.
    pub coverage_signal_authority: ImportedSignalAuthority,
    /// Stable coverage-authority token.
    pub coverage_signal_authority_token: String,
    /// Flaky signal authority.
    pub flaky_signal_authority: ImportedSignalAuthority,
    /// Stable flaky-authority token.
    pub flaky_signal_authority_token: String,
    /// Snapshot signal authority.
    pub snapshot_signal_authority: ImportedSignalAuthority,
    /// Stable snapshot-authority token.
    pub snapshot_signal_authority_token: String,
    /// Local rerun plan that can refresh imported failures.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fresh_local_rerun_plan_ref: Option<String>,
    /// True when the projection explicitly does not claim current local truth.
    pub does_not_claim_live_local_truth: bool,
    /// Export-safe imported-CI summary.
    pub support_summary: String,
}

/// Stability verdict projected from the attempt ledger.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestStabilityVerdict {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable verdict id.
    pub stability_verdict_id: String,
    /// Parent session ref.
    pub test_session_ref: String,
    /// Flaky/stability verdict.
    pub verdict_state: FlakyVerdictState,
    /// Stable verdict token.
    pub verdict_state_token: String,
    /// Attempt refs backing the verdict.
    pub evidence_attempt_refs: Vec<String>,
    /// True when automatic green roll-up is blocked.
    pub blocks_green_rollup: bool,
    /// Export-safe verdict summary.
    pub support_summary: String,
}

/// Launch-wedge projection that surfaces the ledger without raw runner text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestLaunchWedgeProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub launch_wedge_projection_id: String,
    /// Parent session ref.
    pub test_session_ref: String,
    /// Surfaces consuming this projection.
    pub consumer_surfaces: Vec<TestConsumerSurface>,
    /// Current attempt ref for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_attempt_ref: Option<String>,
    /// True when attempt history must be reachable.
    pub attempt_history_visible: bool,
    /// True when imported evidence is visibly read-only.
    pub imported_ci_read_only_badge_visible: bool,
    /// True when a rerun plan is offered to refresh stale/imported evidence.
    pub fresh_local_rerun_plan_available: bool,
    /// Identity-state token surfaced by the row.
    pub identity_stability_token: String,
    /// Controlled state tokens visible to support and review surfaces.
    pub controlled_state_tokens: Vec<String>,
    /// Support/export ref for the projection.
    pub support_export_ref: String,
}

/// Support/export packet for test-attempt alpha state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestAttemptSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Session plan ref.
    pub session_plan_ref: String,
    /// Attempt refs included in order.
    pub attempt_refs: Vec<String>,
    /// Controlled state tokens included in the export.
    pub controlled_state_tokens: Vec<String>,
    /// Imported-CI projection ref.
    pub imported_ci_projection_ref: String,
    /// True when imported evidence is read-only.
    pub read_only_imported_evidence: bool,
    /// Local rerun plans linked from stale/imported evidence.
    pub fresh_local_rerun_plan_refs: Vec<String>,
    /// Export-safe lines suitable for CLI/support review.
    pub summary_lines: Vec<String>,
}

impl TestAttemptSupportExport {
    /// Renders stable plaintext lines for CLI or support review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Test attempt support export: {}\n", self.support_export_id);
        out.push_str(&format!("Session plan: {}\n", self.session_plan_ref));
        out.push_str(&format!(
            "Controlled states: {}\n",
            self.controlled_state_tokens.join(",")
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

/// Full alpha packet consumed by launch-wedge and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestAttemptAlphaPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Identity projection for the selected row or scope.
    pub identity_projection: TestItemIdentityProjection,
    /// Session plan for the selected row or scope.
    pub session_plan: TestSessionPlan,
    /// Append-only attempts included in the packet.
    pub attempts: Vec<TestAttemptRecord>,
    /// Watch-controller projection.
    pub watch_controller: TestWatchController,
    /// Imported-CI projection.
    pub imported_ci_projection: ImportedCiProjection,
    /// Stability verdict.
    pub stability_verdict: TestStabilityVerdict,
    /// Launch-wedge consumer projection.
    pub launch_wedge_projection: TestLaunchWedgeProjection,
    /// Support/export packet.
    pub support_export: TestAttemptSupportExport,
}

impl TestAttemptAlphaPacket {
    /// Builds a default local pytest packet from a run contract.
    pub fn from_pytest_contract(
        contract: &PytestRunContract,
        context: &ExecutionContext,
        generated_at: &str,
    ) -> Self {
        Self::from_pytest_contract_with_options(
            contract,
            context,
            TestAttemptAlphaOptions::local_run(),
            generated_at,
        )
    }

    /// Builds a watch-mode pytest packet from a run contract.
    pub fn from_pytest_watch_contract(
        contract: &PytestRunContract,
        context: &ExecutionContext,
        watch_state: TestWatchState,
        generated_at: &str,
    ) -> Self {
        let degradation_reason = match watch_state {
            TestWatchState::Running => TestWatchDegradationReason::None,
            TestWatchState::Buffering => TestWatchDegradationReason::BufferingBatch,
            TestWatchState::Paused => TestWatchDegradationReason::NotAWatchSession,
            TestWatchState::Degraded => TestWatchDegradationReason::PartialDiscovery,
            TestWatchState::Stale => TestWatchDegradationReason::SourceOrTargetDrift,
        };
        Self::from_pytest_contract_with_options(
            contract,
            context,
            TestAttemptAlphaOptions {
                session_mode: TestSessionMode::WatchTests,
                attempt_kind: TestAttemptKind::WatchCycle,
                watch_state,
                watch_degradation_reason: degradation_reason,
                ..TestAttemptAlphaOptions::local_run()
            },
            generated_at,
        )
    }

    /// Builds an imported-CI failure packet linked to fresh local rerun plans.
    pub fn imported_ci_failure_projection(
        contract: &PytestRunContract,
        context: &ExecutionContext,
        provider_run_ref: impl Into<String>,
        local_rerun_plan_ref: impl Into<String>,
        generated_at: &str,
    ) -> Self {
        let provider_run_ref = provider_run_ref.into();
        let local_rerun_plan_ref = local_rerun_plan_ref.into();
        let identity_projection =
            identity_projection_for(contract, TestIdentityStability::ImportedReadOnly);
        let session_plan = session_plan_for(
            contract,
            context,
            &identity_projection,
            TestSessionMode::ImportProviderCiResults,
            "fresh_local_rerun_required",
            "imported_read_only_no_watch",
        );
        let session_ref = session_plan.test_session_id.clone();
        let base_attempt_id = stable_token(&contract.attempt_id);
        let imported_attempt = attempt_for(
            contract,
            context,
            &session_ref,
            1,
            format!("test-attempt:{base_attempt_id}:imported"),
            TestAttemptKind::ProviderCiImport,
            TestAttemptResultState::ImportedFailed,
            None,
            None,
            TestSourceDriftState::ProviderOnlyNoLocalSource,
            TestWatchState::Stale,
            ImportedCiProjectionClass::AuthoritativeImportedReadOnly,
            CoverageMergeClass::CoveragePartial,
            FlakyVerdictState::SuspectedFlaky,
            SnapshotReviewState::SnapshotReviewRequired,
            AiTestGenerationGateState::AiTestGenerationBlocked,
            Some(local_rerun_plan_ref.clone()),
            "Imported provider failure is read-only and linked to a fresh local rerun plan.",
        );
        let local_repro_attempt = attempt_for(
            contract,
            context,
            &session_ref,
            2,
            format!("test-attempt:{base_attempt_id}:local-repro"),
            TestAttemptKind::LocalParityRerun,
            TestAttemptResultState::Failed,
            Some(imported_attempt.test_attempt_id.clone()),
            Some(imported_attempt.test_attempt_id.clone()),
            TestSourceDriftState::SourceChangedRequiresCurrentContextRerun,
            TestWatchState::Paused,
            ImportedCiProjectionClass::FreshLocalReconfirmation,
            CoverageMergeClass::CoverageMerged,
            FlakyVerdictState::ReproducedFlaky,
            SnapshotReviewState::SnapshotReviewRequired,
            AiTestGenerationGateState::AiTestGenerationBlocked,
            Some(local_rerun_plan_ref.clone()),
            "Fresh local rerun reproduced the provider failure without converting imported evidence into live truth.",
        );
        let stable_attempt = attempt_for(
            contract,
            context,
            &session_ref,
            3,
            format!("test-attempt:{base_attempt_id}:stable-window"),
            TestAttemptKind::LocalParityRerun,
            TestAttemptResultState::Passed,
            Some(local_repro_attempt.test_attempt_id.clone()),
            Some(imported_attempt.test_attempt_id.clone()),
            TestSourceDriftState::NoSourceDrift,
            TestWatchState::Paused,
            ImportedCiProjectionClass::FreshLocalReconfirmation,
            CoverageMergeClass::CoverageMerged,
            FlakyVerdictState::StableAgain,
            SnapshotReviewState::SnapshotReviewRequired,
            AiTestGenerationGateState::AiTestGenerationBlocked,
            Some(local_rerun_plan_ref.clone()),
            "Later comparable local evidence marked the row stable again while preserving prior flaky evidence.",
        );
        let attempts = vec![imported_attempt, local_repro_attempt, stable_attempt];
        let watch_controller = watch_controller_for(
            &session_ref,
            TestWatchState::Stale,
            TestWatchDegradationReason::ImportedReadOnlyEvidence,
            attempts
                .last()
                .map(|attempt| attempt.test_attempt_id.clone()),
        );
        let imported_ci_projection = imported_ci_projection_for(
            &session_ref,
            ImportedCiProjectionClass::FreshLocalReconfirmation,
            Some(provider_run_ref),
            attempts
                .iter()
                .filter(|attempt| attempt.attempt_kind == TestAttemptKind::ProviderCiImport)
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect(),
            attempts
                .iter()
                .filter(|attempt| attempt.attempt_kind == TestAttemptKind::LocalParityRerun)
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect(),
            ImportedSignalAuthority::FreshLocalReconfirmation,
            ImportedSignalAuthority::FreshLocalReconfirmation,
            ImportedSignalAuthority::AuthoritativeImportedEvidence,
            Some(local_rerun_plan_ref),
        );
        let stability_verdict = stability_verdict_for(
            &session_ref,
            FlakyVerdictState::StableAgain,
            attempts
                .iter()
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect(),
        );
        packet_from_parts(
            generated_at,
            identity_projection,
            session_plan,
            attempts,
            watch_controller,
            imported_ci_projection,
            stability_verdict,
        )
    }

    /// Returns controlled state tokens exported by the packet.
    pub fn controlled_state_tokens(&self) -> Vec<String> {
        let mut tokens = vec![
            self.identity_projection.identity_stability_token.clone(),
            self.session_plan.session_mode_token.clone(),
            self.watch_controller.watch_state_token.clone(),
            self.imported_ci_projection.projection_token.clone(),
            self.imported_ci_projection
                .coverage_signal_authority_token
                .clone(),
            self.imported_ci_projection
                .flaky_signal_authority_token
                .clone(),
            self.imported_ci_projection
                .snapshot_signal_authority_token
                .clone(),
            self.stability_verdict.verdict_state_token.clone(),
        ];
        for attempt in &self.attempts {
            tokens.extend(attempt.controlled_state_tokens());
        }
        dedupe_tokens(tokens)
    }

    fn from_pytest_contract_with_options(
        contract: &PytestRunContract,
        context: &ExecutionContext,
        options: TestAttemptAlphaOptions,
        generated_at: &str,
    ) -> Self {
        let identity_stability = identity_stability_for(contract);
        let identity_projection = identity_projection_for(contract, identity_stability);
        let session_plan = session_plan_for(
            contract,
            context,
            &identity_projection,
            options.session_mode,
            options.retry_policy_token,
            options.watch_policy_token,
        );
        let session_ref = session_plan.test_session_id.clone();
        let result_state = if contract.readiness == PytestLaunchReadiness::Blocked {
            TestAttemptResultState::Blocked
        } else {
            options.attempt_result_state
        };
        let attempt = attempt_for(
            contract,
            context,
            &session_ref,
            1,
            format!("test-attempt:{}", stable_token(&contract.attempt_id)),
            options.attempt_kind,
            result_state,
            None,
            None,
            options.source_drift_state,
            options.watch_state,
            options.imported_ci_projection_class,
            options.coverage_merge_class,
            options.flaky_verdict_state,
            options.snapshot_review_state,
            options.ai_test_generation_gate_state,
            options.local_rerun_plan_ref.map(ToOwned::to_owned),
            "Pytest attempt is projected through the shared test-attempt alpha ledger.",
        );
        let attempts = vec![attempt];
        let watch_controller = watch_controller_for(
            &session_ref,
            options.watch_state,
            options.watch_degradation_reason,
            attempts
                .last()
                .map(|attempt| attempt.test_attempt_id.clone()),
        );
        let imported_ci_projection = imported_ci_projection_for(
            &session_ref,
            options.imported_ci_projection_class,
            options.provider_run_ref.map(ToOwned::to_owned),
            Vec::new(),
            Vec::new(),
            options.coverage_signal_authority,
            options.flaky_signal_authority,
            options.snapshot_signal_authority,
            options.local_rerun_plan_ref.map(ToOwned::to_owned),
        );
        let stability_verdict = stability_verdict_for(
            &session_ref,
            options.flaky_verdict_state,
            attempts
                .iter()
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect(),
        );
        packet_from_parts(
            generated_at,
            identity_projection,
            session_plan,
            attempts,
            watch_controller,
            imported_ci_projection,
            stability_verdict,
        )
    }
}

/// Options for building an alpha test-attempt packet from a pytest contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestAttemptAlphaOptions<'a> {
    /// Session mode.
    pub session_mode: TestSessionMode,
    /// Attempt kind.
    pub attempt_kind: TestAttemptKind,
    /// Attempt result state used when the contract is not blocked.
    pub attempt_result_state: TestAttemptResultState,
    /// Watch state.
    pub watch_state: TestWatchState,
    /// Watch degradation reason.
    pub watch_degradation_reason: TestWatchDegradationReason,
    /// Imported-CI projection class.
    pub imported_ci_projection_class: ImportedCiProjectionClass,
    /// Provider run ref when imported CI is present.
    pub provider_run_ref: Option<&'a str>,
    /// Local rerun plan ref for stale/imported evidence.
    pub local_rerun_plan_ref: Option<&'a str>,
    /// Coverage merge class.
    pub coverage_merge_class: CoverageMergeClass,
    /// Coverage signal authority.
    pub coverage_signal_authority: ImportedSignalAuthority,
    /// Flaky verdict state.
    pub flaky_verdict_state: FlakyVerdictState,
    /// Flaky signal authority.
    pub flaky_signal_authority: ImportedSignalAuthority,
    /// Snapshot review state.
    pub snapshot_review_state: SnapshotReviewState,
    /// Snapshot signal authority.
    pub snapshot_signal_authority: ImportedSignalAuthority,
    /// AI test-generation gate state.
    pub ai_test_generation_gate_state: AiTestGenerationGateState,
    /// Source drift state.
    pub source_drift_state: TestSourceDriftState,
    /// Retry policy token.
    pub retry_policy_token: &'a str,
    /// Watch policy token.
    pub watch_policy_token: &'a str,
}

impl<'a> TestAttemptAlphaOptions<'a> {
    /// Returns default options for a local one-shot pytest run.
    pub const fn local_run() -> Self {
        Self {
            session_mode: TestSessionMode::RunSelectedTests,
            attempt_kind: TestAttemptKind::InitialTestRun,
            attempt_result_state: TestAttemptResultState::Running,
            watch_state: TestWatchState::Paused,
            watch_degradation_reason: TestWatchDegradationReason::NotAWatchSession,
            imported_ci_projection_class: ImportedCiProjectionClass::NotImportedCi,
            provider_run_ref: None,
            local_rerun_plan_ref: None,
            coverage_merge_class: CoverageMergeClass::NotRequested,
            coverage_signal_authority: ImportedSignalAuthority::None,
            flaky_verdict_state: FlakyVerdictState::Unknown,
            flaky_signal_authority: ImportedSignalAuthority::None,
            snapshot_review_state: SnapshotReviewState::NotRequired,
            snapshot_signal_authority: ImportedSignalAuthority::None,
            ai_test_generation_gate_state: AiTestGenerationGateState::NotRequested,
            source_drift_state: TestSourceDriftState::NoSourceDrift,
            retry_policy_token: "explicit_user_rerun",
            watch_policy_token: "not_armed",
        }
    }
}

fn identity_projection_for(
    contract: &PytestRunContract,
    stability: TestIdentityStability,
) -> TestItemIdentityProjection {
    let selector_token = match (
        &contract.selection.selector,
        &contract.selection.test_item_id,
    ) {
        (_, Some(test_item_id)) => stable_token(test_item_id),
        (Some(selector), None) => stable_token(selector),
        (None, None) => stable_token(&contract.task_id),
    };
    TestItemIdentityProjection {
        record_kind: TEST_ITEM_IDENTITY_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        identity_projection_id: format!("test-identity-projection:{selector_token}"),
        identity_stability: stability,
        identity_stability_token: stability.as_str().to_owned(),
        canonical_test_item_ref: contract.selection.test_item_id.clone(),
        selector_ref: format!("selector:pytest:{selector_token}"),
        selection_label_digest: digest_token(&contract.selection.label),
        source_ref_digests: contract
            .selection
            .source_refs
            .iter()
            .map(|source_ref| digest_token(source_ref))
            .collect(),
        remap_or_widening_review_required: stability != TestIdentityStability::Stable,
        support_summary: match stability {
            TestIdentityStability::Stable => {
                "Pytest selection has a stable canonical item identity.".to_owned()
            }
            TestIdentityStability::ImportedReadOnly => {
                "Imported provider evidence is read-only until a local target-bound rerun lands."
                    .to_owned()
            }
            _ => "Pytest selection requires visible review before widening or rerun.".to_owned(),
        },
    }
}

fn session_plan_for(
    contract: &PytestRunContract,
    context: &ExecutionContext,
    identity_projection: &TestItemIdentityProjection,
    session_mode: TestSessionMode,
    retry_policy_token: &str,
    watch_policy_token: &str,
) -> TestSessionPlan {
    let session_token = stable_token(&contract.run_id);
    TestSessionPlan {
        record_kind: TEST_SESSION_PLAN_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        test_session_id: format!("test-session:{session_token}"),
        session_plan_id: format!("test-session-plan:{session_token}"),
        session_mode,
        session_mode_token: session_mode.as_str().to_owned(),
        identity_projection_ref: identity_projection.identity_projection_id.clone(),
        canonical_test_item_refs: identity_projection
            .canonical_test_item_ref
            .clone()
            .into_iter()
            .collect(),
        execution_context_ref: context.execution_context_id.clone(),
        target_id: context.target_identity.canonical_target_id.clone(),
        target_class_token: context.target_identity.target_class.as_str().to_owned(),
        retry_policy_token: retry_policy_token.to_owned(),
        watch_policy_token: watch_policy_token.to_owned(),
        selection_widening_rule_token: if identity_projection.remap_or_widening_review_required {
            "widening_review_required".to_owned()
        } else {
            "no_widening_allowed".to_owned()
        },
        export_safe: true,
        support_summary: "Session plan preserves selection, target, retry, and watch policy refs."
            .to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn attempt_for(
    contract: &PytestRunContract,
    context: &ExecutionContext,
    session_ref: &str,
    attempt_index: u32,
    test_attempt_id: String,
    attempt_kind: TestAttemptKind,
    result_state: TestAttemptResultState,
    predecessor_attempt_ref: Option<String>,
    origin_attempt_ref: Option<String>,
    source_drift_state: TestSourceDriftState,
    watch_state: TestWatchState,
    imported_ci_projection_class: ImportedCiProjectionClass,
    coverage_merge_class: CoverageMergeClass,
    flaky_verdict_state: FlakyVerdictState,
    snapshot_review_state: SnapshotReviewState,
    ai_test_generation_gate_state: AiTestGenerationGateState,
    local_rerun_plan_ref: Option<String>,
    support_summary: &str,
) -> TestAttemptRecord {
    TestAttemptRecord {
        record_kind: TEST_ATTEMPT_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        test_attempt_id,
        parent_test_session_ref: session_ref.to_owned(),
        attempt_index,
        attempt_kind,
        attempt_kind_token: attempt_kind.as_str().to_owned(),
        result_state,
        result_state_token: result_state.as_str().to_owned(),
        predecessor_attempt_ref,
        origin_attempt_ref,
        execution_attempt_ref: Some(format!(
            "execution-attempt:{}:{}",
            stable_token(&contract.attempt_id),
            attempt_index
        )),
        execution_context_ref: context.execution_context_id.clone(),
        target_id: context.target_identity.canonical_target_id.clone(),
        target_class_token: context.target_identity.target_class.as_str().to_owned(),
        canonical_test_item_refs: contract
            .selection
            .test_item_id
            .clone()
            .into_iter()
            .collect(),
        selector_ref: selector_ref_for(contract),
        source_drift_state,
        source_drift_state_token: source_drift_state.as_str().to_owned(),
        watch_state_at_attempt: watch_state,
        watch_state_at_attempt_token: watch_state.as_str().to_owned(),
        imported_ci_projection_class,
        imported_ci_projection_token: imported_ci_projection_class.as_str().to_owned(),
        coverage_merge_class,
        coverage_merge_token: coverage_merge_class.as_str().to_owned(),
        flaky_verdict_state,
        flaky_verdict_token: flaky_verdict_state.as_str().to_owned(),
        snapshot_review_state,
        snapshot_review_token: snapshot_review_state.as_str().to_owned(),
        ai_test_generation_gate_state,
        ai_test_generation_gate_token: ai_test_generation_gate_state.as_str().to_owned(),
        artifact_refs: vec![format!(
            "artifact:test-attempt:{}",
            stable_token(&contract.run_id)
        )],
        raw_event_refs: vec![format!(
            "raw-event:test-attempt:{}",
            stable_token(&contract.trace_id)
        )],
        local_rerun_plan_ref,
        support_summary: support_summary.to_owned(),
    }
}

fn watch_controller_for(
    session_ref: &str,
    watch_state: TestWatchState,
    degradation_reason: TestWatchDegradationReason,
    latest_attempt_ref: Option<String>,
) -> TestWatchController {
    let current_truth_claim_allowed = matches!(watch_state, TestWatchState::Running);
    TestWatchController {
        record_kind: TEST_WATCH_CONTROLLER_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        watch_controller_id: format!("test-watch-controller:{}", stable_token(session_ref)),
        test_session_ref: session_ref.to_owned(),
        watch_state,
        watch_state_token: watch_state.as_str().to_owned(),
        degradation_reason,
        degradation_reason_token: degradation_reason.as_str().to_owned(),
        latest_attempt_ref: latest_attempt_ref.clone(),
        last_successful_attempt_ref: if current_truth_claim_allowed {
            latest_attempt_ref
        } else {
            None
        },
        buffered_change_count: if watch_state == TestWatchState::Buffering {
            1
        } else {
            0
        },
        current_truth_claim_allowed,
        support_summary: match watch_state {
            TestWatchState::Running => "Watch is running with current evidence.",
            TestWatchState::Buffering => "Watch is buffering a bounded change batch.",
            TestWatchState::Paused => "Watch is paused or inactive for this session.",
            TestWatchState::Degraded => "Watch is degraded and results must show the caveat.",
            TestWatchState::Stale => {
                "Watch evidence is stale and cannot imply current local truth."
            }
        }
        .to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn imported_ci_projection_for(
    session_ref: &str,
    projection_class: ImportedCiProjectionClass,
    provider_run_ref: Option<String>,
    imported_attempt_refs: Vec<String>,
    local_reconfirmation_attempt_refs: Vec<String>,
    coverage_signal_authority: ImportedSignalAuthority,
    flaky_signal_authority: ImportedSignalAuthority,
    snapshot_signal_authority: ImportedSignalAuthority,
    fresh_local_rerun_plan_ref: Option<String>,
) -> ImportedCiProjection {
    ImportedCiProjection {
        record_kind: IMPORTED_CI_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        imported_ci_projection_id: format!("imported-ci-projection:{}", stable_token(session_ref)),
        test_session_ref: session_ref.to_owned(),
        projection_class,
        projection_token: projection_class.as_str().to_owned(),
        read_only_imported_evidence: !matches!(
            projection_class,
            ImportedCiProjectionClass::NotImportedCi
        ),
        provider_run_ref,
        imported_attempt_refs,
        local_reconfirmation_attempt_refs,
        coverage_signal_authority,
        coverage_signal_authority_token: coverage_signal_authority.as_str().to_owned(),
        flaky_signal_authority,
        flaky_signal_authority_token: flaky_signal_authority.as_str().to_owned(),
        snapshot_signal_authority,
        snapshot_signal_authority_token: snapshot_signal_authority.as_str().to_owned(),
        fresh_local_rerun_plan_ref,
        does_not_claim_live_local_truth: !matches!(
            projection_class,
            ImportedCiProjectionClass::FreshLocalReconfirmation
                | ImportedCiProjectionClass::NotImportedCi
        ),
        support_summary: match projection_class {
            ImportedCiProjectionClass::NotImportedCi => {
                "No imported CI evidence participates in this session."
            }
            ImportedCiProjectionClass::AuthoritativeImportedReadOnly => {
                "Imported CI evidence is provider-authoritative and read-only locally."
            }
            ImportedCiProjectionClass::StaleImportedReadOnly => {
                "Imported CI evidence is stale and requires local refresh before current claims."
            }
            ImportedCiProjectionClass::FreshLocalReconfirmation => {
                "Fresh local rerun evidence is linked without overwriting imported provenance."
            }
            ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview => {
                "Imported CI projection is unknown and requires review."
            }
        }
        .to_owned(),
    }
}

fn stability_verdict_for(
    session_ref: &str,
    verdict_state: FlakyVerdictState,
    evidence_attempt_refs: Vec<String>,
) -> TestStabilityVerdict {
    TestStabilityVerdict {
        record_kind: TEST_STABILITY_VERDICT_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        stability_verdict_id: format!("test-stability-verdict:{}", stable_token(session_ref)),
        test_session_ref: session_ref.to_owned(),
        verdict_state,
        verdict_state_token: verdict_state.as_str().to_owned(),
        evidence_attempt_refs,
        blocks_green_rollup: !matches!(verdict_state, FlakyVerdictState::StableAgain),
        support_summary: match verdict_state {
            FlakyVerdictState::SuspectedFlaky => {
                "Intermittent evidence is suspected but not yet comparable."
            }
            FlakyVerdictState::ReproducedFlaky => {
                "Comparable attempts reproduced divergent outcomes."
            }
            FlakyVerdictState::StableAgain => {
                "Prior instability cleared through a stable evidence window."
            }
            FlakyVerdictState::Muted => "The row is muted and remains visible in counts.",
            FlakyVerdictState::Unknown => "Stability verdict is unknown.",
        }
        .to_owned(),
    }
}

fn packet_from_parts(
    generated_at: &str,
    identity_projection: TestItemIdentityProjection,
    session_plan: TestSessionPlan,
    attempts: Vec<TestAttemptRecord>,
    watch_controller: TestWatchController,
    imported_ci_projection: ImportedCiProjection,
    stability_verdict: TestStabilityVerdict,
) -> TestAttemptAlphaPacket {
    let packet_id = format!(
        "test-attempt-alpha:{}:{}",
        stable_token(&session_plan.test_session_id),
        stable_token(generated_at)
    );
    let controlled_state_tokens = {
        let mut tokens = vec![
            identity_projection.identity_stability_token.clone(),
            session_plan.session_mode_token.clone(),
            watch_controller.watch_state_token.clone(),
            imported_ci_projection.projection_token.clone(),
            imported_ci_projection
                .coverage_signal_authority_token
                .clone(),
            imported_ci_projection.flaky_signal_authority_token.clone(),
            imported_ci_projection
                .snapshot_signal_authority_token
                .clone(),
            stability_verdict.verdict_state_token.clone(),
        ];
        for attempt in &attempts {
            tokens.extend(attempt.controlled_state_tokens());
        }
        dedupe_tokens(tokens)
    };
    let fresh_local_rerun_plan_refs = {
        let mut refs = imported_ci_projection
            .fresh_local_rerun_plan_ref
            .clone()
            .into_iter()
            .collect::<Vec<_>>();
        refs.extend(
            attempts
                .iter()
                .filter_map(|attempt| attempt.local_rerun_plan_ref.clone()),
        );
        dedupe_tokens(refs)
    };
    let attempt_refs = attempts
        .iter()
        .map(|attempt| attempt.test_attempt_id.clone())
        .collect::<Vec<_>>();
    let support_export_id = format!("support-export:{}", stable_token(&packet_id));
    let launch_wedge_projection = TestLaunchWedgeProjection {
        record_kind: TEST_LAUNCH_WEDGE_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        launch_wedge_projection_id: format!("test-launch-wedge:{}", stable_token(&packet_id)),
        test_session_ref: session_plan.test_session_id.clone(),
        consumer_surfaces: vec![
            TestConsumerSurface::LaunchWedge,
            TestConsumerSurface::TestTree,
            TestConsumerSurface::EditorInline,
            TestConsumerSurface::CliOutput,
            TestConsumerSurface::SupportExport,
        ],
        current_attempt_ref: attempt_refs.last().cloned(),
        attempt_history_visible: true,
        imported_ci_read_only_badge_visible: imported_ci_projection.read_only_imported_evidence,
        fresh_local_rerun_plan_available: !fresh_local_rerun_plan_refs.is_empty(),
        identity_stability_token: identity_projection.identity_stability_token.clone(),
        controlled_state_tokens: controlled_state_tokens.clone(),
        support_export_ref: support_export_id.clone(),
    };
    let support_export = TestAttemptSupportExport {
        record_kind: TEST_ATTEMPT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        support_export_id,
        generated_at: generated_at.to_owned(),
        session_plan_ref: session_plan.session_plan_id.clone(),
        attempt_refs,
        controlled_state_tokens,
        imported_ci_projection_ref: imported_ci_projection.imported_ci_projection_id.clone(),
        read_only_imported_evidence: imported_ci_projection.read_only_imported_evidence,
        fresh_local_rerun_plan_refs,
        summary_lines: attempts
            .iter()
            .map(|attempt| {
                format!(
                    "attempt={} result={} coverage={} flaky={} snapshot={} ai_gate={}",
                    attempt.test_attempt_id,
                    attempt.result_state_token,
                    attempt.coverage_merge_token,
                    attempt.flaky_verdict_token,
                    attempt.snapshot_review_token,
                    attempt.ai_test_generation_gate_token
                )
            })
            .collect(),
    };
    TestAttemptAlphaPacket {
        record_kind: TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
        packet_id,
        generated_at: generated_at.to_owned(),
        identity_projection,
        session_plan,
        attempts,
        watch_controller,
        imported_ci_projection,
        stability_verdict,
        launch_wedge_projection,
        support_export,
    }
}

fn identity_stability_for(contract: &PytestRunContract) -> TestIdentityStability {
    if contract.selection.test_item_id.is_some()
        && contract.selection.kind == PytestSelectionKind::DiscoveredItem
        && contract.readiness != PytestLaunchReadiness::Blocked
    {
        TestIdentityStability::Stable
    } else if contract.selection.test_item_id.is_some() {
        TestIdentityStability::DynamicPartialDiscovery
    } else {
        TestIdentityStability::DynamicPartialDiscovery
    }
}

fn selector_ref_for(contract: &PytestRunContract) -> String {
    match (
        &contract.selection.selector,
        &contract.selection.test_item_id,
    ) {
        (_, Some(test_item_id)) => format!("selector:pytest:{}", stable_token(test_item_id)),
        (Some(selector), None) => format!("selector:pytest:{}", stable_token(selector)),
        (None, None) => format!("selector:pytest:{}", stable_token(&contract.task_id)),
    }
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

fn digest_token(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sha256:{hash:064x}")
}

fn dedupe_tokens(tokens: Vec<String>) -> Vec<String> {
    let mut out = Vec::new();
    for token in tokens {
        if !out.contains(&token) {
            out.push(token);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::detectors::python::PythonEnvironmentDetectorConfig;
    use crate::discovery::pytest::{PytestDiscoverer, PytestDiscovererConfig};
    use crate::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        TargetClass, TrustState,
    };

    fn fixture_root(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/python_task_discovery_alpha")
            .join(name)
    }

    fn alpha_fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/test_attempt_alpha")
            .join(name)
    }

    fn resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:python".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace/python".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:workspace:python".to_owned(),
                capsule_hash: "sha256:python".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "test-attempt-alpha".to_owned(),
        })
    }

    fn discoverer() -> PytestDiscoverer {
        PytestDiscoverer::new(PytestDiscovererConfig {
            python_detector: PythonEnvironmentDetectorConfig {
                ambient_python_version: Some("3.12.7".to_owned()),
                ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
                ambient_uv_version: Some("0.5.7".to_owned()),
                ambient_poetry_version: Some("1.8.4".to_owned()),
                ..PythonEnvironmentDetectorConfig::default()
            },
            workspace_revision: Some("rev:python".to_owned()),
        })
    }

    fn pytest_contract() -> (PytestRunContract, ExecutionContext) {
        let mut resolver = resolver();
        let context = resolver.resolve(ExecutionContextRequest::test_seed(
            "test.run.pytest",
            TrustState::Trusted,
            "2026-05-13T18:40:00Z",
        ));
        let discovery = discoverer().discover_workspace(
            &fixture_root("ready_uv"),
            context.clone(),
            "2026-05-13T18:40:01Z",
        );
        let contract = discovery
            .contract_for_selector("tests/test_api.py::test_health")
            .expect("test contract")
            .clone();
        (contract, context)
    }

    #[test]
    fn test_attempt_alpha_local_pytest_packet_preserves_stable_identity_and_exportable_attempt_history(
    ) {
        let (contract, context) = pytest_contract();
        let packet = TestAttemptAlphaPacket::from_pytest_watch_contract(
            &contract,
            &context,
            TestWatchState::Running,
            "2026-05-13T18:40:02Z",
        );

        assert_eq!(packet.record_kind, TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND);
        assert_eq!(
            packet.identity_projection.identity_stability,
            TestIdentityStability::Stable
        );
        assert_eq!(
            packet
                .identity_projection
                .canonical_test_item_ref
                .as_deref(),
            contract.selection.test_item_id.as_deref()
        );
        assert_eq!(
            packet.session_plan.session_mode,
            TestSessionMode::WatchTests
        );
        assert_eq!(packet.watch_controller.watch_state, TestWatchState::Running);
        assert!(packet.watch_controller.current_truth_claim_allowed);
        assert!(packet.launch_wedge_projection.attempt_history_visible);
        assert!(packet
            .support_export
            .controlled_state_tokens
            .contains(&"running".to_owned()));
    }

    #[test]
    fn test_attempt_alpha_imported_ci_packet_keeps_imported_evidence_read_only_and_links_local_rerun(
    ) {
        let (contract, context) = pytest_contract();
        let packet = TestAttemptAlphaPacket::imported_ci_failure_projection(
            &contract,
            &context,
            "provider-run:ci:pytest:884",
            "rerun-plan:local:pytest:health",
            "2026-05-13T18:41:00Z",
        );

        assert_eq!(
            packet.identity_projection.identity_stability,
            TestIdentityStability::ImportedReadOnly
        );
        assert!(packet.imported_ci_projection.read_only_imported_evidence);
        assert!(
            packet
                .imported_ci_projection
                .does_not_claim_live_local_truth
                == false
        );
        assert!(
            packet
                .launch_wedge_projection
                .imported_ci_read_only_badge_visible
        );
        assert!(
            packet
                .launch_wedge_projection
                .fresh_local_rerun_plan_available
        );
        assert!(packet
            .support_export
            .fresh_local_rerun_plan_refs
            .contains(&"rerun-plan:local:pytest:health".to_owned()));
        for token in [
            "coverage_partial",
            "coverage_merged",
            "suspected_flaky",
            "reproduced_flaky",
            "stable_again",
            "snapshot_review_required",
            "ai_test_generation_blocked",
        ] {
            assert!(
                packet
                    .support_export
                    .controlled_state_tokens
                    .contains(&token.to_owned()),
                "missing controlled token {token}"
            );
        }
    }

    #[test]
    fn test_attempt_alpha_protected_fixture_round_trips_and_carries_hardening_tokens() {
        let payload = std::fs::read_to_string(alpha_fixture("controlled_states_imported_ci.json"))
            .expect("fixture must read");
        let packet: TestAttemptAlphaPacket =
            serde_json::from_str(&payload).expect("fixture must parse");

        assert_eq!(packet.record_kind, TEST_ATTEMPT_ALPHA_PACKET_RECORD_KIND);
        assert_eq!(packet.watch_controller.watch_state, TestWatchState::Stale);
        assert!(packet.imported_ci_projection.read_only_imported_evidence);
        for token in [
            "coverage_partial",
            "coverage_merged",
            "suspected_flaky",
            "reproduced_flaky",
            "stable_again",
            "snapshot_review_required",
            "ai_test_generation_blocked",
        ] {
            assert!(
                packet
                    .support_export
                    .controlled_state_tokens
                    .contains(&token.to_owned()),
                "missing controlled token {token}"
            );
        }
        assert!(packet
            .support_export
            .render_plaintext()
            .contains("Controlled states"));
    }
}
