//! Beta triage and release-trust packets for test rows.
//!
//! This module consumes the beta test-runner and test-quality projections and
//! publishes the release-facing trust packet that keeps watch-mode state,
//! flaky verdict lineage, snapshot or baseline mutation review, and
//! mute/quarantine debt visible together. It does not parse runner logs or own
//! discovery; it joins the canonical item/session/attempt ids already emitted
//! by [`crate::testing`] and [`crate::testing_quality`].
//!
//! The machine-readable boundaries live in
//! [`/schemas/testing/watch_state.schema.json`](../../../../schemas/testing/watch_state.schema.json),
//! [`/schemas/testing/flaky_verdict.schema.json`](../../../../schemas/testing/flaky_verdict.schema.json),
//! [`/schemas/testing/test_quarantine_record.schema.json`](../../../../schemas/testing/test_quarantine_record.schema.json),
//! and
//! [`/schemas/testing/test_trust_packet.schema.json`](../../../../schemas/testing/test_trust_packet.schema.json).

use serde::{Deserialize, Serialize};

use crate::testing::{TestRunnerBetaFramework, TestRunnerBetaProjection};
use crate::testing_quality::{FlakyTruthPacket, TestQualityProjection, TestQualityRowTruth};
use crate::tests::{
    FlakyVerdictState, ImportedCiProjectionClass, TestAttemptAlphaPacket, TestAttemptRecord,
    TestAttemptResultState, TestWatchController, TestWatchDegradationReason, TestWatchState,
};

/// Schema version for beta test triage and trust packets.
pub const TEST_TRIAGE_TRUST_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for watch-state packets.
pub const WATCH_STATE_PACKET_RECORD_KIND: &str = "test_watch_state_packet_record";

/// Stable record-kind tag for flaky-verdict packets.
pub const FLAKY_VERDICT_PACKET_RECORD_KIND: &str = "test_flaky_verdict_packet_record";

/// Stable record-kind tag for snapshot/baseline mutation reviews.
pub const SNAPSHOT_MUTATION_REVIEW_RECORD_KIND: &str = "test_snapshot_mutation_review_record";

/// Stable record-kind tag for governed mute/quarantine records.
pub const TEST_QUARANTINE_RECORD_KIND: &str = "test_quarantine_record";

/// Stable record-kind tag for release-facing test-trust packets.
pub const TEST_TRUST_PACKET_RECORD_KIND: &str = "test_trust_packet_record";

/// Watch-mode state exposed to product surfaces and release packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchModeState {
    /// Native watch is live and may claim current truth.
    Live,
    /// Watch is active but narrowed by partial discovery, batching, or scope.
    Reduced,
    /// Watch fell back to polling or a stale-check cycle.
    Polling,
    /// Watch cannot run for the row or target.
    Unavailable,
    /// Evidence is imported only and cannot arm local watch truth.
    ImportedOnly,
}

impl WatchModeState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reduced => "reduced",
            Self::Polling => "polling",
            Self::Unavailable => "unavailable",
            Self::ImportedOnly => "imported_only",
        }
    }

    /// Returns true when the state is anything weaker than live watch truth.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::Live)
    }
}

/// Exact reason watch mode is not fully live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchModeDowngradeReason {
    /// No downgrade is active.
    None,
    /// The session is not a watch session.
    NotAWatchSession,
    /// Discovery is partial or adapter-limited.
    PartialDiscovery,
    /// The watch controller is holding a bounded backlog.
    BoundedBacklog,
    /// Native watcher capability is unavailable and polling is in use.
    NativeWatcherUnavailablePolling,
    /// Provider evidence is imported read-only.
    ImportedReadOnlyEvidence,
    /// Source or target drift invalidated current watch truth.
    SourceOrTargetDrift,
    /// The target or remote endpoint is unavailable.
    TargetUnavailable,
    /// Reconnect is required before watch truth can resume.
    ReconnectRequired,
    /// The downgrade cannot be classified.
    UnknownRequiresReview,
}

impl WatchModeDowngradeReason {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NotAWatchSession => "not_a_watch_session",
            Self::PartialDiscovery => "partial_discovery",
            Self::BoundedBacklog => "bounded_backlog",
            Self::NativeWatcherUnavailablePolling => "native_watcher_unavailable_polling",
            Self::ImportedReadOnlyEvidence => "imported_read_only_evidence",
            Self::SourceOrTargetDrift => "source_or_target_drift",
            Self::TargetUnavailable => "target_unavailable",
            Self::ReconnectRequired => "reconnect_required",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Shared identity row used by every triage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTriageIdentity {
    /// Canonical test item the row refers to.
    pub canonical_test_item_ref: String,
    /// Selector ref used by tree, inline, CLI, and support flows.
    pub selector_ref: String,
    /// Test session ref the row belongs to.
    pub test_session_ref: String,
    /// Beta framework that owns the row.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Test-tree row ref.
    pub tree_row_ref: String,
    /// Inline result row ref.
    pub inline_row_ref: String,
}

impl TestTriageIdentity {
    /// Builds triage identity from a quality row.
    pub fn from_quality_row(row: &TestQualityRowTruth) -> Self {
        Self {
            canonical_test_item_ref: row.identity.canonical_test_item_ref.clone(),
            selector_ref: row.identity.selector_ref.clone(),
            test_session_ref: row.identity.test_session_ref.clone(),
            framework: row.identity.framework,
            framework_token: row.identity.framework_token.clone(),
            tree_row_ref: row.tree_row_ref.clone(),
            inline_row_ref: row.inline_row_ref.clone(),
        }
    }
}

/// Watch-state packet exported for one claimed beta test row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchStatePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub watch_state_packet_id: String,
    /// Row identity.
    pub identity: TestTriageIdentity,
    /// Source alpha watch-controller ref.
    pub watch_controller_ref: String,
    /// Release-facing watch state.
    pub watch_state: WatchModeState,
    /// Stable release-facing watch-state token.
    pub watch_state_token: String,
    /// Exact downgrade reasons.
    pub downgrade_reasons: Vec<WatchModeDowngradeReason>,
    /// Stable downgrade-reason tokens.
    pub downgrade_reason_tokens: Vec<String>,
    /// Source alpha watch-state token.
    pub source_watch_state_token: String,
    /// Source alpha degradation-reason token.
    pub source_degradation_reason_token: String,
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
    /// True when reconnect/rerun keeps the prior watch downgrade lineage.
    pub reconnect_preserves_state: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl WatchStatePacket {
    /// Builds a watch-state packet from an alpha packet.
    pub fn from_alpha_packet(
        identity: TestTriageIdentity,
        packet: &TestAttemptAlphaPacket,
    ) -> Self {
        Self::from_watch_controller(
            identity,
            &packet.watch_controller,
            packet.imported_ci_projection.projection_class,
        )
    }

    /// Builds a watch-state packet from the source watch controller.
    pub fn from_watch_controller(
        identity: TestTriageIdentity,
        controller: &TestWatchController,
        imported_ci_projection_class: ImportedCiProjectionClass,
    ) -> Self {
        let watch_state = release_watch_state(controller, imported_ci_projection_class);
        let downgrade_reasons =
            release_watch_downgrade_reasons(controller, imported_ci_projection_class);
        let downgrade_reason_tokens = downgrade_reasons
            .iter()
            .map(|reason| reason.as_str().to_owned())
            .collect::<Vec<_>>();
        let summary = if watch_state == WatchModeState::Live {
            "Watch is live for this row.".to_owned()
        } else {
            format!(
                "Watch is {} because {}.",
                watch_state.as_str(),
                downgrade_reason_tokens.join(",")
            )
        };
        Self {
            record_kind: WATCH_STATE_PACKET_RECORD_KIND.to_owned(),
            schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
            watch_state_packet_id: format!(
                "test-watch-state:{}:{}",
                stable_token(&identity.test_session_ref),
                stable_token(&identity.canonical_test_item_ref)
            ),
            identity,
            watch_controller_ref: controller.watch_controller_id.clone(),
            watch_state,
            watch_state_token: watch_state.as_str().to_owned(),
            downgrade_reasons,
            downgrade_reason_tokens,
            source_watch_state_token: controller.watch_state_token.clone(),
            source_degradation_reason_token: controller.degradation_reason_token.clone(),
            latest_attempt_ref: controller.latest_attempt_ref.clone(),
            last_successful_attempt_ref: controller.last_successful_attempt_ref.clone(),
            buffered_change_count: controller.buffered_change_count,
            current_truth_claim_allowed: controller.current_truth_claim_allowed,
            reconnect_preserves_state: watch_state.is_degraded(),
            summary,
        }
    }
}

/// One attempt input used to classify a flaky verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlakyVerdictAttemptInput {
    /// Attempt ref.
    pub attempt_ref: String,
    /// One-based attempt index inside the session.
    pub attempt_index: u32,
    /// Attempt-kind token.
    pub attempt_kind_token: String,
    /// Result-state token.
    pub result_state_token: String,
    /// Execution-context ref used by the attempt.
    pub execution_context_ref: String,
    /// Target id used by the attempt.
    pub target_id: String,
    /// Target-class token.
    pub target_class_token: String,
    /// Imported-CI projection token for the attempt.
    pub imported_ci_projection_token: String,
    /// Source-drift token for the attempt.
    pub source_drift_state_token: String,
    /// Watch-state token at attempt open.
    pub watch_state_at_attempt_token: String,
    /// Predecessor attempt ref when this attempt derives from one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_attempt_ref: Option<String>,
    /// Origin attempt ref when this attempt derives from imported evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_attempt_ref: Option<String>,
}

impl FlakyVerdictAttemptInput {
    /// Projects a governed attempt input from the canonical attempt ledger.
    pub fn from_attempt(attempt: &TestAttemptRecord) -> Self {
        Self {
            attempt_ref: attempt.test_attempt_id.clone(),
            attempt_index: attempt.attempt_index,
            attempt_kind_token: attempt.attempt_kind_token.clone(),
            result_state_token: attempt.result_state_token.clone(),
            execution_context_ref: attempt.execution_context_ref.clone(),
            target_id: attempt.target_id.clone(),
            target_class_token: attempt.target_class_token.clone(),
            imported_ci_projection_token: attempt.imported_ci_projection_token.clone(),
            source_drift_state_token: attempt.source_drift_state_token.clone(),
            watch_state_at_attempt_token: attempt.watch_state_at_attempt_token.clone(),
            predecessor_attempt_ref: attempt.predecessor_attempt_ref.clone(),
            origin_attempt_ref: attempt.origin_attempt_ref.clone(),
        }
    }
}

/// Flaky-verdict packet with concrete attempt-ledger inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlakyVerdictPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub flaky_verdict_packet_id: String,
    /// Row identity.
    pub identity: TestTriageIdentity,
    /// Verdict state.
    pub verdict_state: FlakyVerdictState,
    /// Stable verdict token.
    pub verdict_token: String,
    /// Latest attempt contributing to the verdict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_test_attempt_ref: Option<String>,
    /// Attempt refs contributing to the verdict.
    pub evidence_attempt_refs: Vec<String>,
    /// Attempt inputs used to classify the verdict.
    pub evidence_attempts: Vec<FlakyVerdictAttemptInput>,
    /// Number of attempts considered.
    pub observation_window_attempts: u32,
    /// True when release rows must keep the verdict visible as debt.
    pub release_visible: bool,
    /// Export-safe summary.
    pub summary: String,
}

impl FlakyVerdictPacket {
    /// Builds a flaky-verdict packet from quality truth and attempt history.
    pub fn from_quality_and_attempts(
        identity: TestTriageIdentity,
        quality_flaky: Option<&FlakyTruthPacket>,
        attempts: &[&TestAttemptRecord],
    ) -> Self {
        let verdict_state = quality_flaky
            .map(|packet| packet.flaky_verdict_state)
            .or_else(|| attempts.last().map(|attempt| attempt.flaky_verdict_state))
            .unwrap_or(FlakyVerdictState::Unknown);
        let verdict_token = quality_flaky
            .map(|packet| packet.flaky_verdict_token.clone())
            .unwrap_or_else(|| verdict_state.as_str().to_owned());
        let producing_test_attempt_ref = quality_flaky
            .and_then(|packet| packet.producing_test_attempt_ref.clone())
            .or_else(|| {
                attempts
                    .last()
                    .map(|attempt| attempt.test_attempt_id.clone())
            });
        let evidence_attempts = attempts
            .iter()
            .map(|attempt| FlakyVerdictAttemptInput::from_attempt(attempt))
            .collect::<Vec<_>>();
        let evidence_attempt_refs = evidence_attempts
            .iter()
            .map(|attempt| attempt.attempt_ref.clone())
            .collect::<Vec<_>>();
        let observation_window_attempts = evidence_attempts.len() as u32;
        let release_visible = !matches!(verdict_state, FlakyVerdictState::StableAgain);
        let summary = format!(
            "verdict={} attempts={}",
            verdict_token, observation_window_attempts
        );
        Self {
            record_kind: FLAKY_VERDICT_PACKET_RECORD_KIND.to_owned(),
            schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
            flaky_verdict_packet_id: format!(
                "test-flaky-verdict:{}:{}",
                stable_token(&identity.test_session_ref),
                stable_token(&identity.canonical_test_item_ref)
            ),
            identity,
            verdict_state,
            verdict_token,
            producing_test_attempt_ref,
            evidence_attempt_refs,
            evidence_attempts,
            observation_window_attempts,
            release_visible,
            summary,
        }
    }
}

/// State of a snapshot or baseline mutation review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotMutationReviewState {
    /// No snapshot or baseline mutation is present.
    NoMutation,
    /// A preview is required before the mutation can land.
    PreviewRequired,
    /// Preview, actor, policy hook, and rollback checkpoint are ready.
    ReviewReady,
    /// Mutation landed with a grouped rollback checkpoint.
    AppliedWithRollbackCheckpoint,
    /// Mutation is blocked because no preview exists.
    BlockedMissingPreview,
    /// Mutation is blocked because no grouped rollback checkpoint exists.
    BlockedMissingRollbackCheckpoint,
    /// Mutation is blocked by release-bearing policy hooks.
    BlockedReleaseBearingPolicy,
    /// Review state cannot be classified.
    UnknownRequiresReview,
}

impl SnapshotMutationReviewState {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMutation => "no_mutation",
            Self::PreviewRequired => "preview_required",
            Self::ReviewReady => "review_ready",
            Self::AppliedWithRollbackCheckpoint => "applied_with_rollback_checkpoint",
            Self::BlockedMissingPreview => "blocked_missing_preview",
            Self::BlockedMissingRollbackCheckpoint => "blocked_missing_rollback_checkpoint",
            Self::BlockedReleaseBearingPolicy => "blocked_release_bearing_policy",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// One file or artifact preview inside a snapshot mutation review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotFileChangePreview {
    /// Stable preview id.
    pub preview_id: String,
    /// Opaque file or artifact ref.
    pub subject_ref: String,
    /// Baseline artifact ref before the mutation.
    pub before_artifact_ref: String,
    /// Proposed artifact ref after the mutation.
    pub after_artifact_ref: String,
    /// Change class token.
    pub change_class_token: String,
    /// Rendered or text preview ref.
    pub preview_artifact_ref: String,
    /// Export-safe preview summary.
    pub summary: String,
}

/// Reviewable snapshot or baseline mutation operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotMutationReview {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub mutation_review_id: String,
    /// Row identity.
    pub identity: TestTriageIdentity,
    /// Review state.
    pub review_state: SnapshotMutationReviewState,
    /// Stable review-state token.
    pub review_state_token: String,
    /// Actor ref responsible for the mutation review.
    pub actor_ref: String,
    /// File or artifact previews included in the review.
    pub file_change_previews: Vec<SnapshotFileChangePreview>,
    /// True when at least one preview is available.
    pub preview_available: bool,
    /// Grouped rollback checkpoint ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// True when a grouped rollback checkpoint is required to land.
    pub grouped_rollback_checkpoint_required: bool,
    /// True when the row is release-bearing.
    pub release_bearing_scope: bool,
    /// Policy hooks consulted by the review.
    pub policy_hook_refs: Vec<String>,
    /// True when the mutation is allowed to land.
    pub can_land: bool,
    /// Denial reasons preventing the mutation from landing.
    pub denial_reason_tokens: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl SnapshotMutationReview {
    /// Builds a governed snapshot/baseline mutation review.
    pub fn new(
        mutation_review_id: impl Into<String>,
        identity: TestTriageIdentity,
        requested_state: SnapshotMutationReviewState,
        actor_ref: impl Into<String>,
        file_change_previews: Vec<SnapshotFileChangePreview>,
        rollback_checkpoint_ref: Option<String>,
        release_bearing_scope: bool,
        policy_hook_refs: Vec<String>,
        summary: impl Into<String>,
    ) -> Self {
        let preview_available = !file_change_previews.is_empty();
        let grouped_rollback_checkpoint_required = matches!(
            requested_state,
            SnapshotMutationReviewState::ReviewReady
                | SnapshotMutationReviewState::AppliedWithRollbackCheckpoint
        );
        let mut denial_reason_tokens = Vec::new();
        if grouped_rollback_checkpoint_required && !preview_available {
            denial_reason_tokens.push("missing_file_change_preview".to_owned());
        }
        if grouped_rollback_checkpoint_required && rollback_checkpoint_ref.is_none() {
            denial_reason_tokens.push("missing_grouped_rollback_checkpoint".to_owned());
        }
        if release_bearing_scope && policy_hook_refs.is_empty() {
            denial_reason_tokens.push("missing_release_bearing_policy_hook".to_owned());
        }

        let can_land = grouped_rollback_checkpoint_required && denial_reason_tokens.is_empty();
        let review_state = if can_land {
            requested_state
        } else if denial_reason_tokens
            .iter()
            .any(|token| token == "missing_file_change_preview")
        {
            SnapshotMutationReviewState::BlockedMissingPreview
        } else if denial_reason_tokens
            .iter()
            .any(|token| token == "missing_grouped_rollback_checkpoint")
        {
            SnapshotMutationReviewState::BlockedMissingRollbackCheckpoint
        } else if release_bearing_scope && policy_hook_refs.is_empty() {
            SnapshotMutationReviewState::BlockedReleaseBearingPolicy
        } else {
            requested_state
        };

        Self {
            record_kind: SNAPSHOT_MUTATION_REVIEW_RECORD_KIND.to_owned(),
            schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
            mutation_review_id: mutation_review_id.into(),
            identity,
            review_state,
            review_state_token: review_state.as_str().to_owned(),
            actor_ref: actor_ref.into(),
            file_change_previews,
            preview_available,
            rollback_checkpoint_ref,
            grouped_rollback_checkpoint_required,
            release_bearing_scope,
            policy_hook_refs,
            can_land,
            denial_reason_tokens,
            summary: summary.into(),
        }
    }
}

/// Governed test treatment kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQuarantineTreatmentKind {
    /// User or policy mute that suppresses noise but remains release-visible.
    Mute,
    /// Quarantine that narrows execution or result counting.
    Quarantine,
}

impl TestQuarantineTreatmentKind {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mute => "mute",
            Self::Quarantine => "quarantine",
        }
    }
}

/// Governed test quarantine status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQuarantineStatus {
    /// Record is active and inside its expiry window.
    Active,
    /// Record expired and reopened the affected scope for review.
    ExpiredReopened,
    /// Record was resolved and retained as history.
    Resolved,
    /// Record was renewed under a new owner or expiry.
    Renewed,
}

impl TestQuarantineStatus {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiredReopened => "expired_reopened",
            Self::Resolved => "resolved",
            Self::Renewed => "renewed",
        }
    }
}

/// Reason a mute or quarantine exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQuarantineReason {
    /// Recent intermittent behavior is suspected.
    SuspectedFlaky,
    /// Comparable attempts reproduced flaky behavior.
    ReproducedFlaky,
    /// The row is known failing and under investigation.
    KnownFailing,
    /// The row depends on a target environment condition.
    EnvironmentDependent,
    /// Imported provider evidence is incomparable locally.
    ImportedIncomparable,
    /// A local mute suppresses noise but does not hide debt.
    ManualNoiseReduction,
    /// Policy restricts execution or result publication.
    PolicyRestriction,
    /// Reason cannot be classified.
    UnknownRequiresReview,
}

impl TestQuarantineReason {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::KnownFailing => "known_failing",
            Self::EnvironmentDependent => "environment_dependent",
            Self::ImportedIncomparable => "imported_incomparable",
            Self::ManualNoiseReduction => "manual_noise_reduction",
            Self::PolicyRestriction => "policy_restriction",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Scope class for a mute or quarantine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQuarantineScopeClass {
    /// Scope is one canonical test item.
    ExactTestItem,
    /// Scope is a parameterized family.
    ParameterizedFamily,
    /// Scope is a suite or container.
    SuiteOrContainer,
    /// Scope is a selector expression.
    SelectorExpression,
    /// Scope is a target environment.
    TargetEnvironment,
    /// Scope is a release channel.
    ReleaseChannelScope,
    /// Scope cannot be classified.
    UnknownRequiresReview,
}

impl TestQuarantineScopeClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTestItem => "exact_test_item",
            Self::ParameterizedFamily => "parameterized_family",
            Self::SuiteOrContainer => "suite_or_container",
            Self::SelectorExpression => "selector_expression",
            Self::TargetEnvironment => "target_environment",
            Self::ReleaseChannelScope => "release_channel_scope",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Release treatment for mute or quarantine debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestReleaseDebtClass {
    /// Debt blocks release widening or stable promotion.
    ReleaseBlocking,
    /// Claim text must narrow while debt remains active.
    ClaimNarrowingRequired,
    /// Debt cites a governed waiver.
    WaiverLinked,
    /// Debt is informational after recovery.
    InformationalRecovered,
    /// Debt cannot be classified.
    UnknownRequiresReview,
}

impl TestReleaseDebtClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseBlocking => "release_blocking",
            Self::ClaimNarrowingRequired => "claim_narrowing_required",
            Self::WaiverLinked => "waiver_linked",
            Self::InformationalRecovered => "informational_recovered",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Returns true when the debt blocks release widening by default.
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::ReleaseBlocking | Self::ClaimNarrowingRequired | Self::UnknownRequiresReview
        )
    }
}

/// Behavior applied when a mute or quarantine expires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestQuarantineReopenBehavior {
    /// Reopen the affected failing scope on the next attempt.
    ReopenFailure,
    /// Require a stable evidence window before closing.
    StableWindowRequired,
    /// Require manual owner review.
    ManualOwnerReview,
    /// Require renewal before continued suppression.
    RenewalRequired,
    /// Reopen behavior cannot be classified.
    UnknownRequiresReview,
}

impl TestQuarantineReopenBehavior {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReopenFailure => "reopen_failure",
            Self::StableWindowRequired => "stable_window_required",
            Self::ManualOwnerReview => "manual_owner_review",
            Self::RenewalRequired => "renewal_required",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Governed mute or quarantine record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQuarantineRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable record id.
    pub quarantine_record_id: String,
    /// Treatment kind.
    pub treatment_kind: TestQuarantineTreatmentKind,
    /// Stable treatment-kind token.
    pub treatment_kind_token: String,
    /// Current status.
    pub status: TestQuarantineStatus,
    /// Stable status token.
    pub status_token: String,
    /// Owner responsible for the record.
    pub owner_ref: String,
    /// Reason for the treatment.
    pub reason: TestQuarantineReason,
    /// Stable reason token.
    pub reason_token: String,
    /// Scope class.
    pub scope_class: TestQuarantineScopeClass,
    /// Stable scope-class token.
    pub scope_class_token: String,
    /// Opaque refs covered by the scope.
    pub scope_refs: Vec<String>,
    /// Record creation timestamp.
    pub created_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Evidence refs supporting the treatment.
    pub evidence_refs: Vec<String>,
    /// Behavior applied after expiry.
    pub reopen_behavior: TestQuarantineReopenBehavior,
    /// Stable reopen-behavior token.
    pub reopen_behavior_token: String,
    /// Release debt class.
    pub release_debt_class: TestReleaseDebtClass,
    /// Stable release-debt token.
    pub release_debt_token: String,
    /// True when release and support packets must include the record.
    pub release_visible: bool,
    /// Attempt ref reopened when the record expired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reopened_attempt_ref: Option<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl TestQuarantineRecord {
    /// Builds an active governed mute or quarantine record.
    #[allow(clippy::too_many_arguments)]
    pub fn active(
        quarantine_record_id: impl Into<String>,
        treatment_kind: TestQuarantineTreatmentKind,
        owner_ref: impl Into<String>,
        reason: TestQuarantineReason,
        scope_class: TestQuarantineScopeClass,
        scope_refs: Vec<String>,
        created_at: impl Into<String>,
        expires_at: impl Into<String>,
        evidence_refs: Vec<String>,
        reopen_behavior: TestQuarantineReopenBehavior,
        release_debt_class: TestReleaseDebtClass,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TEST_QUARANTINE_RECORD_KIND.to_owned(),
            schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
            quarantine_record_id: quarantine_record_id.into(),
            treatment_kind,
            treatment_kind_token: treatment_kind.as_str().to_owned(),
            status: TestQuarantineStatus::Active,
            status_token: TestQuarantineStatus::Active.as_str().to_owned(),
            owner_ref: owner_ref.into(),
            reason,
            reason_token: reason.as_str().to_owned(),
            scope_class,
            scope_class_token: scope_class.as_str().to_owned(),
            scope_refs,
            created_at: created_at.into(),
            expires_at: expires_at.into(),
            evidence_refs,
            reopen_behavior,
            reopen_behavior_token: reopen_behavior.as_str().to_owned(),
            release_debt_class,
            release_debt_token: release_debt_class.as_str().to_owned(),
            release_visible: true,
            reopened_attempt_ref: None,
            summary: summary.into(),
        }
    }

    /// Returns this record with expiry automatically reopened, when due.
    pub fn evaluated_at(
        &self,
        now: &str,
        reopened_attempt_ref: Option<String>,
    ) -> TestQuarantineRecord {
        if self.status == TestQuarantineStatus::Active && self.expires_at.as_str() <= now {
            let mut reopened = self.clone();
            reopened.status = TestQuarantineStatus::ExpiredReopened;
            reopened.status_token = TestQuarantineStatus::ExpiredReopened.as_str().to_owned();
            reopened.release_debt_class = TestReleaseDebtClass::ReleaseBlocking;
            reopened.release_debt_token = TestReleaseDebtClass::ReleaseBlocking.as_str().to_owned();
            reopened.release_visible = true;
            reopened.reopened_attempt_ref = reopened_attempt_ref;
            reopened.summary = format!(
                "Expired {} reopened for owner review.",
                reopened.treatment_kind_token
            );
            reopened
        } else {
            self.clone()
        }
    }

    /// Returns true when this record applies to a row identity.
    pub fn applies_to(&self, identity: &TestTriageIdentity) -> bool {
        self.scope_refs.iter().any(|scope| {
            scope == &identity.canonical_test_item_ref || scope == &identity.test_session_ref
        })
    }
}

/// Evidence class summarized on a release-facing trust row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestEvidenceTrustClass {
    /// Current local evidence exists.
    LiveLocal,
    /// Evidence is imported only.
    ImportedOnly,
    /// Local and imported evidence are both represented.
    MixedLocalAndImported,
    /// Evidence is stale, missing, or unknown.
    StaleOrUnknown,
}

impl TestEvidenceTrustClass {
    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLocal => "live_local",
            Self::ImportedOnly => "imported_only",
            Self::MixedLocalAndImported => "mixed_local_and_imported",
            Self::StaleOrUnknown => "stale_or_unknown",
        }
    }
}

/// Row-level summary included in a test-trust packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTrustRowSummary {
    /// Row identity.
    pub identity: TestTriageIdentity,
    /// Latest attempt ref considered for the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Watch-state packet ref.
    pub watch_state_packet_ref: String,
    /// Release-facing watch-state token.
    pub watch_state_token: String,
    /// Downgrade-reason tokens for watch state.
    pub watch_downgrade_reason_tokens: Vec<String>,
    /// Flaky-verdict packet ref.
    pub flaky_verdict_packet_ref: String,
    /// Flaky-verdict token.
    pub flaky_verdict_token: String,
    /// Attempt refs backing the flaky verdict.
    pub flaky_evidence_attempt_refs: Vec<String>,
    /// Snapshot mutation reviews applying to the row.
    pub snapshot_mutation_review_refs: Vec<String>,
    /// Mute or quarantine records applying to the row.
    pub quarantine_record_refs: Vec<String>,
    /// Imported-vs-live evidence class.
    pub evidence_trust_class: TestEvidenceTrustClass,
    /// Stable evidence-trust token.
    pub evidence_trust_token: String,
    /// Release-debt tokens applying to the row.
    pub row_debt_tokens: Vec<String>,
    /// True when the row blocks release widening by default.
    pub release_blocking: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Release-facing test-trust packet for a set of claimed beta rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTrustPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub trust_packet_id: String,
    /// Packet generation timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Test-tree projection ref.
    pub tree_projection_ref: String,
    /// Inline-result projection ref.
    pub inline_projection_ref: String,
    /// Quality projection ref.
    pub quality_projection_ref: String,
    /// Watch-state packets included in deterministic row order.
    pub watch_state_packets: Vec<WatchStatePacket>,
    /// Flaky-verdict packets included in deterministic row order.
    pub flaky_verdict_packets: Vec<FlakyVerdictPacket>,
    /// Snapshot/baseline mutation reviews included in the packet.
    pub snapshot_mutation_reviews: Vec<SnapshotMutationReview>,
    /// Mute/quarantine records included in the packet.
    pub quarantine_records: Vec<TestQuarantineRecord>,
    /// Row summaries included in deterministic row order.
    pub row_summaries: Vec<TestTrustRowSummary>,
    /// Number of rows with degraded watch state.
    pub watch_degraded_row_count: u32,
    /// Number of rows with imported-only evidence.
    pub imported_only_row_count: u32,
    /// Number of active mute records.
    pub active_mute_record_count: u32,
    /// Number of visible quarantined scopes.
    pub quarantined_scope_count: u32,
    /// Number of records reopened by expiry.
    pub expired_reopened_count: u32,
    /// Number of snapshot/baseline mutation reviews.
    pub snapshot_mutation_count: u32,
    /// Number of row summaries carrying release-blocking debt.
    pub release_blocking_debt_count: u32,
    /// Export-safe summary lines.
    pub summary_lines: Vec<String>,
}

impl TestTrustPacket {
    /// Builds a release-facing trust packet from beta runner and quality truth.
    pub fn from_projection(
        runner: &TestRunnerBetaProjection,
        quality: &TestQualityProjection,
        snapshot_mutation_reviews: Vec<SnapshotMutationReview>,
        quarantine_records: Vec<TestQuarantineRecord>,
        generated_at: impl Into<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let quarantine_records = quarantine_records
            .into_iter()
            .map(|record| record.evaluated_at(&generated_at, None))
            .collect::<Vec<_>>();

        let mut watch_state_packets = Vec::new();
        let mut flaky_verdict_packets = Vec::new();
        let mut row_summaries = Vec::new();

        for row in &quality.row_truths {
            let identity = TestTriageIdentity::from_quality_row(row);
            let packet = attempt_packet_for_session(runner, &identity.test_session_ref);
            let attempts = packet
                .map(|packet| packet.attempts.iter().collect::<Vec<_>>())
                .unwrap_or_default();
            let latest_attempt = attempts.last().copied();

            let watch_packet = packet
                .map(|packet| WatchStatePacket::from_alpha_packet(identity.clone(), packet))
                .unwrap_or_else(|| missing_watch_packet(identity.clone()));
            let quality_flaky =
                flaky_packet_for_identity(quality, &identity.canonical_test_item_ref);
            let flaky_packet = FlakyVerdictPacket::from_quality_and_attempts(
                identity.clone(),
                quality_flaky,
                &attempts,
            );

            let snapshot_refs = snapshot_mutation_reviews
                .iter()
                .filter(|review| {
                    review.identity.canonical_test_item_ref == identity.canonical_test_item_ref
                })
                .map(|review| review.mutation_review_id.clone())
                .collect::<Vec<_>>();
            let row_quarantine_records = quarantine_records
                .iter()
                .filter(|record| record.applies_to(&identity))
                .collect::<Vec<_>>();
            let quarantine_refs = row_quarantine_records
                .iter()
                .map(|record| record.quarantine_record_id.clone())
                .collect::<Vec<_>>();
            let evidence_trust_class = evidence_trust_class(latest_attempt);
            let mut row_debt_tokens = row_debt_tokens(
                &watch_packet,
                &flaky_packet,
                &snapshot_mutation_reviews,
                &snapshot_refs,
                &row_quarantine_records,
                evidence_trust_class,
            );
            row_debt_tokens.sort();
            row_debt_tokens.dedup();
            let release_blocking = release_blocking_row(
                &snapshot_mutation_reviews,
                &snapshot_refs,
                &row_quarantine_records,
            );
            let row_summary = TestTrustRowSummary {
                identity: identity.clone(),
                latest_attempt_ref: latest_attempt.map(|attempt| attempt.test_attempt_id.clone()),
                watch_state_packet_ref: watch_packet.watch_state_packet_id.clone(),
                watch_state_token: watch_packet.watch_state_token.clone(),
                watch_downgrade_reason_tokens: watch_packet.downgrade_reason_tokens.clone(),
                flaky_verdict_packet_ref: flaky_packet.flaky_verdict_packet_id.clone(),
                flaky_verdict_token: flaky_packet.verdict_token.clone(),
                flaky_evidence_attempt_refs: flaky_packet.evidence_attempt_refs.clone(),
                snapshot_mutation_review_refs: snapshot_refs,
                quarantine_record_refs: quarantine_refs,
                evidence_trust_class,
                evidence_trust_token: evidence_trust_class.as_str().to_owned(),
                row_debt_tokens: row_debt_tokens.clone(),
                release_blocking,
                summary: format!(
                    "test={} watch={} flaky={} evidence={} debt={}",
                    identity.canonical_test_item_ref,
                    watch_packet.watch_state_token,
                    flaky_packet.verdict_token,
                    evidence_trust_class.as_str(),
                    row_debt_tokens.join(",")
                ),
            };

            watch_state_packets.push(watch_packet);
            flaky_verdict_packets.push(flaky_packet);
            row_summaries.push(row_summary);
        }

        let watch_degraded_row_count = watch_state_packets
            .iter()
            .filter(|packet| packet.watch_state.is_degraded())
            .count() as u32;
        let imported_only_row_count = row_summaries
            .iter()
            .filter(|row| row.evidence_trust_class == TestEvidenceTrustClass::ImportedOnly)
            .count() as u32;
        let active_mute_record_count = quarantine_records
            .iter()
            .filter(|record| {
                record.treatment_kind == TestQuarantineTreatmentKind::Mute
                    && record.status == TestQuarantineStatus::Active
            })
            .count() as u32;
        let quarantined_scope_count = quarantine_records
            .iter()
            .filter(|record| {
                record.treatment_kind == TestQuarantineTreatmentKind::Quarantine
                    && record.release_visible
            })
            .count() as u32;
        let expired_reopened_count = quarantine_records
            .iter()
            .filter(|record| record.status == TestQuarantineStatus::ExpiredReopened)
            .count() as u32;
        let snapshot_mutation_count = snapshot_mutation_reviews.len() as u32;
        let release_blocking_debt_count = row_summaries
            .iter()
            .filter(|row| row.release_blocking)
            .count() as u32;
        let summary_lines = row_summaries
            .iter()
            .map(|row| row.summary.clone())
            .collect::<Vec<_>>();

        Self {
            record_kind: TEST_TRUST_PACKET_RECORD_KIND.to_owned(),
            schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
            trust_packet_id: format!(
                "test-trust-packet:{}:{}",
                stable_token(&runner.workspace_id),
                stable_token(&generated_at)
            ),
            generated_at,
            workspace_id: runner.workspace_id.clone(),
            tree_projection_ref: runner.tree.tree_projection_id.clone(),
            inline_projection_ref: runner.inline.inline_projection_id.clone(),
            quality_projection_ref: quality.projection_id.clone(),
            watch_state_packets,
            flaky_verdict_packets,
            snapshot_mutation_reviews,
            quarantine_records,
            row_summaries,
            watch_degraded_row_count,
            imported_only_row_count,
            active_mute_record_count,
            quarantined_scope_count,
            expired_reopened_count,
            snapshot_mutation_count,
            release_blocking_debt_count,
            summary_lines,
        }
    }

    /// Renders deterministic plaintext lines for support and release review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Test trust packet: {}\n", self.trust_packet_id);
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Watch degraded rows: {}\n",
            self.watch_degraded_row_count
        ));
        out.push_str(&format!(
            "Imported-only rows: {}\n",
            self.imported_only_row_count
        ));
        out.push_str(&format!(
            "Active mute records: {}\n",
            self.active_mute_record_count
        ));
        out.push_str(&format!(
            "Quarantined scopes: {}\n",
            self.quarantined_scope_count
        ));
        out.push_str(&format!(
            "Expired reopened records: {}\n",
            self.expired_reopened_count
        ));
        out.push_str(&format!(
            "Snapshot mutations: {}\n",
            self.snapshot_mutation_count
        ));
        out.push_str(&format!(
            "Release-blocking debt rows: {}\n",
            self.release_blocking_debt_count
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

fn release_watch_state(
    controller: &TestWatchController,
    imported_ci_projection_class: ImportedCiProjectionClass,
) -> WatchModeState {
    if imported_ci_projection_class != ImportedCiProjectionClass::NotImportedCi
        && controller.degradation_reason == TestWatchDegradationReason::ImportedReadOnlyEvidence
    {
        return WatchModeState::ImportedOnly;
    }
    match (controller.watch_state, controller.degradation_reason) {
        (TestWatchState::Running, TestWatchDegradationReason::None) => WatchModeState::Live,
        (TestWatchState::Buffering, _) | (TestWatchState::Degraded, _) => WatchModeState::Reduced,
        (TestWatchState::Stale, TestWatchDegradationReason::TargetUnavailable) => {
            WatchModeState::Unavailable
        }
        (TestWatchState::Stale, _) => WatchModeState::Polling,
        (TestWatchState::Paused, _) => WatchModeState::Unavailable,
        (TestWatchState::Running, _) => WatchModeState::Reduced,
    }
}

fn release_watch_downgrade_reasons(
    controller: &TestWatchController,
    imported_ci_projection_class: ImportedCiProjectionClass,
) -> Vec<WatchModeDowngradeReason> {
    if release_watch_state(controller, imported_ci_projection_class) == WatchModeState::Live {
        return vec![WatchModeDowngradeReason::None];
    }
    let reason = match controller.degradation_reason {
        TestWatchDegradationReason::None => WatchModeDowngradeReason::UnknownRequiresReview,
        TestWatchDegradationReason::NotAWatchSession => WatchModeDowngradeReason::NotAWatchSession,
        TestWatchDegradationReason::PartialDiscovery => WatchModeDowngradeReason::PartialDiscovery,
        TestWatchDegradationReason::BufferingBatch => WatchModeDowngradeReason::BoundedBacklog,
        TestWatchDegradationReason::ImportedReadOnlyEvidence => {
            WatchModeDowngradeReason::ImportedReadOnlyEvidence
        }
        TestWatchDegradationReason::SourceOrTargetDrift => {
            WatchModeDowngradeReason::SourceOrTargetDrift
        }
        TestWatchDegradationReason::TargetUnavailable => {
            WatchModeDowngradeReason::TargetUnavailable
        }
    };
    vec![reason]
}

fn missing_watch_packet(identity: TestTriageIdentity) -> WatchStatePacket {
    WatchStatePacket {
        record_kind: WATCH_STATE_PACKET_RECORD_KIND.to_owned(),
        schema_version: TEST_TRIAGE_TRUST_SCHEMA_VERSION,
        watch_state_packet_id: format!(
            "test-watch-state:{}:{}",
            stable_token(&identity.test_session_ref),
            stable_token(&identity.canonical_test_item_ref)
        ),
        identity,
        watch_controller_ref: "watch-controller:missing".to_owned(),
        watch_state: WatchModeState::Unavailable,
        watch_state_token: WatchModeState::Unavailable.as_str().to_owned(),
        downgrade_reasons: vec![WatchModeDowngradeReason::UnknownRequiresReview],
        downgrade_reason_tokens: vec![WatchModeDowngradeReason::UnknownRequiresReview
            .as_str()
            .to_owned()],
        source_watch_state_token: "missing".to_owned(),
        source_degradation_reason_token: "missing".to_owned(),
        latest_attempt_ref: None,
        last_successful_attempt_ref: None,
        buffered_change_count: 0,
        current_truth_claim_allowed: false,
        reconnect_preserves_state: true,
        summary: "Watch state is unavailable because no alpha packet was found.".to_owned(),
    }
}

fn flaky_packet_for_identity<'a>(
    quality: &'a TestQualityProjection,
    canonical_test_item_ref: &str,
) -> Option<&'a FlakyTruthPacket> {
    quality
        .flaky_packets
        .iter()
        .find(|packet| packet.identity.canonical_test_item_ref.as_str() == canonical_test_item_ref)
}

fn attempt_packet_for_session<'a>(
    runner: &'a TestRunnerBetaProjection,
    session_ref: &str,
) -> Option<&'a TestAttemptAlphaPacket> {
    runner
        .attempt_packets
        .iter()
        .find(|packet| packet.session_plan.test_session_id == session_ref)
}

fn evidence_trust_class(latest_attempt: Option<&TestAttemptRecord>) -> TestEvidenceTrustClass {
    match latest_attempt {
        None => TestEvidenceTrustClass::StaleOrUnknown,
        Some(attempt) => match attempt.imported_ci_projection_class {
            ImportedCiProjectionClass::NotImportedCi => {
                if matches!(
                    attempt.result_state,
                    TestAttemptResultState::ImportedFailed | TestAttemptResultState::ImportedStale
                ) {
                    TestEvidenceTrustClass::ImportedOnly
                } else {
                    TestEvidenceTrustClass::LiveLocal
                }
            }
            ImportedCiProjectionClass::AuthoritativeImportedReadOnly
            | ImportedCiProjectionClass::StaleImportedReadOnly => {
                TestEvidenceTrustClass::ImportedOnly
            }
            ImportedCiProjectionClass::FreshLocalReconfirmation => {
                TestEvidenceTrustClass::MixedLocalAndImported
            }
            ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview => {
                TestEvidenceTrustClass::StaleOrUnknown
            }
        },
    }
}

fn row_debt_tokens(
    watch_packet: &WatchStatePacket,
    flaky_packet: &FlakyVerdictPacket,
    snapshot_mutation_reviews: &[SnapshotMutationReview],
    snapshot_refs: &[String],
    quarantine_records: &[&TestQuarantineRecord],
    evidence_trust_class: TestEvidenceTrustClass,
) -> Vec<String> {
    let mut tokens = Vec::new();
    if watch_packet.watch_state.is_degraded() {
        tokens.push("watch_degraded".to_owned());
    }
    if flaky_packet.release_visible {
        tokens.push("flaky_verdict_visible".to_owned());
    }
    if evidence_trust_class == TestEvidenceTrustClass::ImportedOnly {
        tokens.push("imported_only_evidence".to_owned());
    }
    for review in snapshot_mutation_reviews
        .iter()
        .filter(|review| snapshot_refs.contains(&review.mutation_review_id))
    {
        if !review.can_land {
            tokens.push("snapshot_mutation_review_blocked".to_owned());
        }
    }
    for record in quarantine_records {
        tokens.push(record.release_debt_token.clone());
        if record.status == TestQuarantineStatus::ExpiredReopened {
            tokens.push("expired_reopened".to_owned());
        }
    }
    tokens
}

fn release_blocking_row(
    snapshot_mutation_reviews: &[SnapshotMutationReview],
    snapshot_refs: &[String],
    quarantine_records: &[&TestQuarantineRecord],
) -> bool {
    quarantine_records.iter().any(|record| {
        record.release_debt_class.is_blocking()
            || record.status == TestQuarantineStatus::ExpiredReopened
    }) || snapshot_mutation_reviews
        .iter()
        .filter(|review| snapshot_refs.contains(&review.mutation_review_id))
        .any(|review| review.release_bearing_scope && !review.can_land)
}

fn stable_token(input: &str) -> String {
    let mut token = String::with_capacity(input.len());
    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('-') {
            token.push('-');
        }
    }
    token.trim_matches('-').to_owned()
}
