//! Beta finalize layer for the test runner.
//!
//! This module pins the closed set of beta test-runner frameworks (today,
//! pytest only) and projects the alpha test-attempt records into one shared
//! identity across three consumer surfaces:
//!
//! - the **test tree** (workspace → file → test-case rows) used by the shell
//!   to discover and run tests;
//! - the **inline editor result** rendered next to the test-case in the editor
//!   pane;
//! - the **rerun-last** lane carried by [`crate::rerun::RerunLastLoop`].
//!
//! The beta promise is that every claimed beta row exposes one canonical
//! `canonical_test_item_ref`, one `selector_ref`, and one
//! `latest_attempt_ref` across the tree row, the inline result row, and the
//! rerun-last command. Artifacts produced by an attempt carry a typed
//! [`TestArtifactKind`] and quote the same identity, so support exports can
//! correlate evidence end to end.
//!
//! The beta layer never widens framework support beyond the
//! [`TestRunnerBetaCoverageManifest`] claims; consumer rows that touch a
//! framework outside the manifest are reported as unclaimed instead of being
//! silently rendered.
//!
//! The canonical alpha test-attempt definitions still live in
//! [`crate::tests`]; this module reads them at the beta finalize boundary so
//! the shell, the activity center, support exports, and AI / human review
//! consumers project the same identity grammar.
//!
//! The machine-readable boundary lives at
//! [`/schemas/testing/test_runner_beta.schema.json`](../../../../schemas/testing/test_runner_beta.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/test_runner_beta.md`](../../../../docs/runtime/m3/test_runner_beta.md).

use serde::{Deserialize, Serialize};

use crate::discovery::pytest::{
    PytestDiscovery, PytestRunContract, PytestSelectionKind, PytestTestDescriptor,
    PytestTestFileDescriptor,
};
use crate::execution_context::ExecutionContext;
use crate::rerun::{RerunLane, RerunLastLaunch, RerunPreparedAttempt, RerunRunContract};
use crate::tasks::TaskWedgeClass;
use crate::tests::{
    TestAttemptAlphaPacket, TestAttemptRecord, TestConsumerSurface, TEST_ATTEMPT_ALPHA_SCHEMA_VERSION,
};

/// Schema-version tag the beta layer republishes so downstream consumers can
/// compare against the boundary schema without reading the alpha module first.
pub const TEST_RUNNER_BETA_SCHEMA_VERSION: u32 = TEST_ATTEMPT_ALPHA_SCHEMA_VERSION;

/// Stable record-kind tag for the beta coverage manifest.
pub const TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "test_runner_beta_coverage_manifest_record";

/// Stable record-kind tag for one test-tree projection.
pub const TEST_RUNNER_BETA_TREE_PROJECTION_RECORD_KIND: &str =
    "test_runner_beta_tree_projection_record";

/// Stable record-kind tag for one test-tree row.
pub const TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND: &str = "test_runner_beta_tree_row_record";

/// Stable record-kind tag for one inline editor result projection.
pub const TEST_RUNNER_BETA_INLINE_PROJECTION_RECORD_KIND: &str =
    "test_runner_beta_inline_projection_record";

/// Stable record-kind tag for one inline editor result row.
pub const TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND: &str = "test_runner_beta_inline_row_record";

/// Stable record-kind tag for one structured artifact identity row.
pub const TEST_RUNNER_BETA_ARTIFACT_IDENTITY_RECORD_KIND: &str =
    "test_runner_beta_artifact_identity_record";

/// Stable record-kind tag for one rerun-last parity row.
pub const TEST_RUNNER_BETA_RERUN_PARITY_RECORD_KIND: &str =
    "test_runner_beta_rerun_parity_record";

/// Stable record-kind tag for the beta support-export packet.
pub const TEST_RUNNER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "test_runner_beta_support_export_record";

/// Frameworks claimed by the beta test-runner manifest.
///
/// The vocabulary is closed; widening it is a beta-policy change that MUST
/// update the canonical schema, the reviewer doc, and the coverage manifest
/// fixture together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestRunnerBetaFramework {
    /// Python pytest framework.
    Pytest,
}

impl TestRunnerBetaFramework {
    /// All frameworks the beta lane currently claims.
    pub const ALL: [Self; 1] = [Self::Pytest];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pytest => "pytest",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pytest => "pytest",
        }
    }

    /// Task wedge that carries this framework's events.
    pub const fn wedge(self) -> TaskWedgeClass {
        match self {
            Self::Pytest => TaskWedgeClass::Test,
        }
    }

    /// Rerun lane that carries this framework's last-launch contract.
    pub const fn rerun_lane(self) -> RerunLane {
        match self {
            Self::Pytest => RerunLane::Test,
        }
    }
}

/// Closed kind vocabulary for [`TestArtifactIdentity`].
///
/// Every artifact emitted by an attempt and surfaced on a beta row MUST map to
/// one of these tokens; widening the vocabulary is a schema change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestArtifactKind {
    /// Structured run report (xunit / pytest report style).
    RunReport,
    /// Structured coverage report.
    CoverageReport,
    /// Snapshot or golden diff produced by the attempt.
    SnapshotDiff,
    /// Captured stdout / stderr slice retained on a governed artifact rail.
    LogSlice,
    /// Retained raw adapter envelope reference.
    RawEventEnvelope,
    /// Debug trace recorded from a test attempt.
    DebugTrace,
    /// AI-generated suggestion attached to the attempt.
    AiSuggestion,
}

impl TestArtifactKind {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunReport => "run_report",
            Self::CoverageReport => "coverage_report",
            Self::SnapshotDiff => "snapshot_diff",
            Self::LogSlice => "log_slice",
            Self::RawEventEnvelope => "raw_event_envelope",
            Self::DebugTrace => "debug_trace",
            Self::AiSuggestion => "ai_suggestion",
        }
    }

    fn classify_artifact_ref(reference: &str) -> Self {
        let lowered = reference.to_ascii_lowercase();
        if lowered.contains("raw-event") || lowered.contains("raw_event") {
            Self::RawEventEnvelope
        } else if lowered.contains("coverage") {
            Self::CoverageReport
        } else if lowered.contains("snapshot") {
            Self::SnapshotDiff
        } else if lowered.contains("debug") || lowered.contains("trace") {
            Self::DebugTrace
        } else if lowered.contains("log") {
            Self::LogSlice
        } else if lowered.contains("ai") {
            Self::AiSuggestion
        } else {
            Self::RunReport
        }
    }
}

/// Closed kind vocabulary for the rows in a [`TestTreeProjection`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestTreeRowKind {
    /// Workspace root row that anchors the tree.
    WorkspaceRoot,
    /// One pytest-compatible source file row.
    TestFile,
    /// One discovered test case row (function or method).
    TestCase,
}

impl TestTreeRowKind {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceRoot => "workspace_root",
            Self::TestFile => "test_file",
            Self::TestCase => "test_case",
        }
    }
}

/// Closed agreement-state vocabulary for [`TestRunnerBetaRerunParity`].
///
/// A beta row may only mark `RowsAgree` when the test-tree case row, the
/// inline editor row, and the rerun-last prepared attempt all carry the same
/// canonical test-item identity AND the same test session reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestRunnerBetaParityState {
    /// Tree row, inline row, and rerun-last command agree on identity.
    RowsAgree,
    /// Rerun-last has no remembered launch for this lane yet.
    RerunLaneUnset,
    /// At least one consumer surface disagrees on identity; review required.
    SurfaceDisagreementRequiresReview,
}

impl TestRunnerBetaParityState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RowsAgree => "rows_agree",
            Self::RerunLaneUnset => "rerun_lane_unset",
            Self::SurfaceDisagreementRequiresReview => {
                "surface_disagreement_requires_review"
            }
        }
    }
}

/// One coverage row of the [`TestRunnerBetaCoverageManifest`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaCoverageRow {
    /// Beta framework.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Reviewer-facing framework label.
    pub framework_label: String,
    /// Task wedge that carries this framework's events.
    pub wedge: TaskWedgeClass,
    /// Stable wedge token.
    pub wedge_token: String,
    /// Rerun lane that carries this framework's last-launch contract.
    pub rerun_lane: RerunLane,
    /// Stable rerun-lane token.
    pub rerun_lane_token: String,
    /// Canonical rerun-last command id.
    pub rerun_last_command_id: String,
    /// Consumer surfaces this framework projects into.
    pub consumer_surfaces: Vec<TestConsumerSurface>,
    /// Stable consumer-surface tokens.
    pub consumer_surface_tokens: Vec<String>,
    /// Artifact kinds the framework is allowed to publish.
    pub claimed_artifact_kinds: Vec<TestArtifactKind>,
    /// Stable artifact-kind tokens.
    pub claimed_artifact_kind_tokens: Vec<String>,
}

impl TestRunnerBetaCoverageRow {
    /// Builds the canonical coverage row for one framework.
    pub fn canonical(framework: TestRunnerBetaFramework) -> Self {
        let consumer_surfaces = vec![
            TestConsumerSurface::LaunchWedge,
            TestConsumerSurface::TestTree,
            TestConsumerSurface::EditorInline,
            TestConsumerSurface::CliOutput,
            TestConsumerSurface::SupportExport,
        ];
        let claimed_artifact_kinds = vec![
            TestArtifactKind::RunReport,
            TestArtifactKind::CoverageReport,
            TestArtifactKind::SnapshotDiff,
            TestArtifactKind::LogSlice,
            TestArtifactKind::RawEventEnvelope,
            TestArtifactKind::DebugTrace,
            TestArtifactKind::AiSuggestion,
        ];
        let consumer_surface_tokens = consumer_surfaces
            .iter()
            .map(|surface| surface.as_str().to_owned())
            .collect();
        let claimed_artifact_kind_tokens = claimed_artifact_kinds
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        let wedge = framework.wedge();
        let rerun_lane = framework.rerun_lane();
        Self {
            framework,
            framework_token: framework.as_str().to_owned(),
            framework_label: framework.label().to_owned(),
            wedge,
            wedge_token: wedge.as_str().to_owned(),
            rerun_lane,
            rerun_lane_token: rerun_lane.as_str().to_owned(),
            rerun_last_command_id: rerun_lane.command_id().to_owned(),
            consumer_surfaces,
            consumer_surface_tokens,
            claimed_artifact_kinds,
            claimed_artifact_kind_tokens,
        }
    }
}

/// Coverage manifest pinning the canonical beta test-runner frameworks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical alpha test-attempt model.
    pub test_attempt_schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Canonical framework coverage rows.
    pub frameworks: Vec<TestRunnerBetaCoverageRow>,
}

impl TestRunnerBetaCoverageManifest {
    /// Builds the canonical beta coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            test_attempt_schema_version: TEST_RUNNER_BETA_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            frameworks: TestRunnerBetaFramework::ALL
                .into_iter()
                .map(TestRunnerBetaCoverageRow::canonical)
                .collect(),
        }
    }

    /// Returns the canonical row for one framework, if present.
    pub fn row_for_framework(
        &self,
        framework: TestRunnerBetaFramework,
    ) -> Option<&TestRunnerBetaCoverageRow> {
        self.frameworks
            .iter()
            .find(|row| row.framework == framework)
    }

    /// True when every claimed framework's wedge appears in the supplied set.
    /// Use this to verify the beta row never implies a wedge / framework that
    /// the manifest does not claim.
    pub fn covers_wedges(&self, wedges: &[TaskWedgeClass]) -> bool {
        for wedge in wedges {
            if !self
                .frameworks
                .iter()
                .any(|row| row.wedge == *wedge)
            {
                return false;
            }
        }
        true
    }
}

/// One row in a [`TestTreeProjection`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTreeRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable row id.
    pub tree_row_id: String,
    /// Row kind.
    pub row_kind: TestTreeRowKind,
    /// Stable row-kind token.
    pub row_kind_token: String,
    /// Parent row id when the row is not the workspace root.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_row_id: Option<String>,
    /// Reviewer-facing display label.
    pub display_label: String,
    /// Workspace-relative source file ref when the row is bound to a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_file_ref: Option<String>,
    /// One-based line number when the row is bound to a test case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_number: Option<u32>,
    /// Canonical test-item ref shared with the inline row and the rerun-last
    /// contract; only present on case rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub canonical_test_item_ref: Option<String>,
    /// Selector ref shared with the inline row and the rerun-last contract;
    /// only present on case rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector_ref: Option<String>,
    /// Stable identity-stability token; only present on case rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identity_stability_token: Option<String>,
    /// Latest attempt ref for this row when an attempt has been recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Latest result-state token when an attempt has been recorded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_result_state_token: Option<String>,
    /// Test session ref the row participates in.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_session_ref: Option<String>,
    /// Run contract id the row dispatches to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_contract_ref: Option<String>,
    /// Canonical rerun-last command id when the row is a case row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_last_command_id: Option<String>,
    /// Child row ids; populated for workspace and file rows.
    pub child_row_ids: Vec<String>,
    /// Artifact identity refs attached to the row.
    pub artifact_identity_refs: Vec<String>,
}

/// Test-tree projection for one beta framework run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTreeProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical alpha test-attempt model.
    pub test_attempt_schema_version: u32,
    /// Stable projection id.
    pub tree_projection_id: String,
    /// Workspace id copied from the discovery context.
    pub workspace_id: String,
    /// Beta framework that produced the projection.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Discovery state token.
    pub discovery_state_token: String,
    /// Execution-context ref used by the discovery pass.
    pub execution_context_ref: String,
    /// Resolved canonical target id.
    pub target_id: String,
    /// Workspace-root row id.
    pub root_row_id: String,
    /// Tree rows in deterministic order (root, files, cases under their files).
    pub rows: Vec<TestTreeRow>,
    /// Honesty-marker flag inherited from discovery.
    pub honesty_marker_present: bool,
}

impl TestTreeProjection {
    /// Returns the row for a given canonical test-item ref, if any.
    pub fn case_row_for_test_item(&self, canonical_test_item_ref: &str) -> Option<&TestTreeRow> {
        self.rows.iter().find(|row| {
            row.row_kind == TestTreeRowKind::TestCase
                && row.canonical_test_item_ref.as_deref() == Some(canonical_test_item_ref)
        })
    }
}

/// One inline editor result row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineTestResultRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable inline row id.
    pub inline_row_id: String,
    /// Canonical test-item ref shared with the tree case row and the rerun-
    /// last contract.
    pub canonical_test_item_ref: String,
    /// Selector ref shared with the tree case row and the rerun-last contract.
    pub selector_ref: String,
    /// Workspace-relative source file ref the inline marker decorates.
    pub source_file_ref: String,
    /// One-based line number the inline marker decorates.
    pub line_number: u32,
    /// Stable identity-stability token.
    pub identity_stability_token: String,
    /// Test session ref the inline result belongs to.
    pub test_session_ref: String,
    /// Latest attempt ref the inline result reflects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Latest result-state token the inline result reflects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_result_state_token: Option<String>,
    /// Tree-row ref this inline marker mirrors (parity invariant).
    pub tree_row_ref: String,
    /// Artifact identity refs attached to this row.
    pub artifact_identity_refs: Vec<String>,
    /// Reviewer-facing summary line.
    pub summary: String,
}

/// Inline editor result projection for one beta framework run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InlineTestResultProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical alpha test-attempt model.
    pub test_attempt_schema_version: u32,
    /// Stable projection id.
    pub inline_projection_id: String,
    /// Workspace id copied from the discovery context.
    pub workspace_id: String,
    /// Beta framework that produced the projection.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Execution-context ref used by the discovery pass.
    pub execution_context_ref: String,
    /// Resolved canonical target id.
    pub target_id: String,
    /// Tree-projection ref the inline rows mirror.
    pub tree_projection_ref: String,
    /// Inline rows in deterministic order.
    pub rows: Vec<InlineTestResultRow>,
}

impl InlineTestResultProjection {
    /// Returns the inline row for a given canonical test-item ref, if any.
    pub fn row_for_test_item(&self, canonical_test_item_ref: &str) -> Option<&InlineTestResultRow> {
        self.rows
            .iter()
            .find(|row| row.canonical_test_item_ref == canonical_test_item_ref)
    }
}

/// Structured artifact identity row carried across beta consumer surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestArtifactIdentity {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable artifact id.
    pub artifact_identity_id: String,
    /// Artifact kind.
    pub artifact_kind: TestArtifactKind,
    /// Stable artifact-kind token.
    pub artifact_kind_token: String,
    /// Underlying artifact ref retained on a governed artifact rail.
    pub artifact_ref: String,
    /// Test session ref the artifact belongs to.
    pub test_session_ref: String,
    /// Test attempt ref that produced the artifact.
    pub test_attempt_ref: String,
    /// Canonical test-item refs the artifact covers.
    pub canonical_test_item_refs: Vec<String>,
    /// Selector ref the artifact was produced for.
    pub selector_ref: String,
    /// Execution-context ref used by the producing attempt.
    pub execution_context_ref: String,
    /// Resolved canonical target id at attempt open.
    pub target_id: String,
    /// Beta framework that produced the artifact.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Producing run id.
    pub producing_run_ref: String,
    /// Producing attempt id (execution-rail ref).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producing_execution_attempt_ref: Option<String>,
    /// Identity-stability token of the producing test-item identity.
    pub identity_stability_token: String,
}

/// Rerun-last parity row binding the tree, inline, and rerun-last surfaces.
///
/// This is the proof object that the three consumer surfaces agree on the
/// same canonical test-item ref, the same test session, and the same lane.
/// Beta rows MUST attach a parity row before claiming "users can rerun from
/// any surface."
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaRerunParity {
    /// Stable record kind.
    pub record_kind: String,
    /// Stable parity id.
    pub parity_id: String,
    /// Beta framework the parity row applies to.
    pub framework: TestRunnerBetaFramework,
    /// Stable framework token.
    pub framework_token: String,
    /// Test session ref the parity row pins.
    pub test_session_ref: String,
    /// Canonical test-item ref the parity row pins.
    pub canonical_test_item_ref: String,
    /// Tree-row ref for the case row.
    pub tree_row_ref: String,
    /// Inline-row ref the parity row pins.
    pub inline_row_ref: String,
    /// Latest test-attempt ref the surfaces agree on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Canonical rerun-last command id.
    pub rerun_last_command_id: String,
    /// Stable rerun-lane token.
    pub rerun_lane_token: String,
    /// Prior attempt id remembered by the rerun-last loop, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_last_prior_attempt_id: Option<String>,
    /// Prepared rerun-attempt id projected by the rerun-last loop, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_last_prepared_attempt_id: Option<String>,
    /// Stable rerun dispatch-state token, if a rerun has been prepared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_dispatch_state_token: Option<String>,
    /// Agreement state.
    pub agreement_state: TestRunnerBetaParityState,
    /// Stable agreement-state token.
    pub agreement_state_token: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl TestRunnerBetaRerunParity {
    /// Builds a parity row from the three consumer surfaces.
    pub fn evaluate(
        framework: TestRunnerBetaFramework,
        tree_row: &TestTreeRow,
        inline_row: &InlineTestResultRow,
        last_launch: Option<&RerunLastLaunch>,
        prepared: Option<&RerunPreparedAttempt>,
    ) -> Self {
        let canonical_test_item_ref = tree_row
            .canonical_test_item_ref
            .clone()
            .unwrap_or_else(|| inline_row.canonical_test_item_ref.clone());
        let tree_session_ref = tree_row.test_session_ref.clone();
        let inline_session_ref = inline_row.test_session_ref.clone();
        let test_session_ref = tree_session_ref
            .clone()
            .unwrap_or_else(|| inline_session_ref.clone());
        let mut agreement = TestRunnerBetaParityState::RowsAgree;

        if tree_row.canonical_test_item_ref.as_deref() != Some(&canonical_test_item_ref) {
            agreement = TestRunnerBetaParityState::SurfaceDisagreementRequiresReview;
        }
        if inline_row.canonical_test_item_ref != canonical_test_item_ref {
            agreement = TestRunnerBetaParityState::SurfaceDisagreementRequiresReview;
        }
        if let Some(tree_session) = tree_session_ref.as_deref() {
            if tree_session != inline_session_ref {
                agreement = TestRunnerBetaParityState::SurfaceDisagreementRequiresReview;
            }
        }
        if inline_row.tree_row_ref != tree_row.tree_row_id {
            agreement = TestRunnerBetaParityState::SurfaceDisagreementRequiresReview;
        }

        let lane = framework.rerun_lane();
        let mut rerun_last_prior_attempt_id = None;
        let mut rerun_last_prepared_attempt_id = None;
        let mut rerun_dispatch_state_token = None;

        match (last_launch, prepared) {
            (None, _) => {
                if !matches!(agreement, TestRunnerBetaParityState::SurfaceDisagreementRequiresReview)
                {
                    agreement = TestRunnerBetaParityState::RerunLaneUnset;
                }
            }
            (Some(launch), prepared_opt) => {
                if launch.lane != lane {
                    agreement = TestRunnerBetaParityState::SurfaceDisagreementRequiresReview;
                }
                rerun_last_prior_attempt_id = Some(launch.prior_attempt.attempt_id.clone());
                if let Some(prepared) = prepared_opt {
                    rerun_dispatch_state_token = Some(prepared.dispatch_state_token.clone());
                    rerun_last_prepared_attempt_id =
                        prepared.next_attempt.as_ref().map(|next| next.attempt_id.clone());
                }
            }
        }

        let agreement_state_token = agreement.as_str().to_owned();
        let summary = match agreement {
            TestRunnerBetaParityState::RowsAgree => format!(
                "Tree, inline, and {} command agree on test {} in session {}.",
                lane.command_id(),
                canonical_test_item_ref,
                test_session_ref
            ),
            TestRunnerBetaParityState::RerunLaneUnset => format!(
                "Tree and inline agree on test {} in session {}; {} has no remembered launch yet.",
                canonical_test_item_ref,
                test_session_ref,
                lane.command_id()
            ),
            TestRunnerBetaParityState::SurfaceDisagreementRequiresReview => format!(
                "Surface disagreement for test {} in session {}; review required before rerun.",
                canonical_test_item_ref, test_session_ref
            ),
        };

        Self {
            record_kind: TEST_RUNNER_BETA_RERUN_PARITY_RECORD_KIND.to_owned(),
            parity_id: format!(
                "test-runner-beta-parity:{}:{}",
                stable_token(&test_session_ref),
                stable_token(&canonical_test_item_ref)
            ),
            framework,
            framework_token: framework.as_str().to_owned(),
            test_session_ref,
            canonical_test_item_ref,
            tree_row_ref: tree_row.tree_row_id.clone(),
            inline_row_ref: inline_row.inline_row_id.clone(),
            latest_attempt_ref: tree_row
                .latest_attempt_ref
                .clone()
                .or_else(|| inline_row.latest_attempt_ref.clone()),
            rerun_last_command_id: lane.command_id().to_owned(),
            rerun_lane_token: lane.as_str().to_owned(),
            rerun_last_prior_attempt_id,
            rerun_last_prepared_attempt_id,
            rerun_dispatch_state_token,
            agreement_state: agreement,
            agreement_state_token,
            summary,
        }
    }
}

/// Beta resolver entry point that turns a discovery + alpha-attempt fan-out
/// into the three beta consumer projections plus the parity rows.
#[derive(Debug, Clone, PartialEq)]
pub struct TestRunnerBetaProjection {
    /// Beta framework that produced the projection.
    pub framework: TestRunnerBetaFramework,
    /// Workspace id.
    pub workspace_id: String,
    /// Test-tree projection.
    pub tree: TestTreeProjection,
    /// Inline editor result projection.
    pub inline: InlineTestResultProjection,
    /// Structured artifact identities for every artifact ref the alpha
    /// attempts published.
    pub artifact_identities: Vec<TestArtifactIdentity>,
    /// Parity rows binding tree, inline, and rerun-last across every case.
    pub parity_rows: Vec<TestRunnerBetaRerunParity>,
    /// Underlying alpha test-attempt packets retained for replay.
    pub attempt_packets: Vec<TestAttemptAlphaPacket>,
}

impl TestRunnerBetaProjection {
    /// Projects a pytest discovery + execution context onto the beta surfaces.
    ///
    /// `last_launch` and `prepared_rerun` are the rerun-last loop facts the
    /// caller resolved against the current context. They are used to fill in
    /// the parity rows.
    pub fn from_pytest_discovery(
        discovery: &PytestDiscovery,
        execution_context: &ExecutionContext,
        attempt_packets: Vec<TestAttemptAlphaPacket>,
        last_launch: Option<&RerunLastLaunch>,
        prepared_rerun: Option<&RerunPreparedAttempt>,
        generated_at: &str,
    ) -> Self {
        let framework = TestRunnerBetaFramework::Pytest;
        let workspace_id = discovery.workspace_id.clone();
        let tree = build_pytest_tree(framework, discovery, &attempt_packets, generated_at);
        let inline = build_pytest_inline_projection(
            framework,
            discovery,
            &tree,
            &attempt_packets,
            generated_at,
        );
        let artifact_identities = build_pytest_artifact_identities(framework, &attempt_packets);
        let parity_rows = build_pytest_parity_rows(
            framework,
            &tree,
            &inline,
            last_launch,
            prepared_rerun,
        );
        let _ = execution_context;
        Self {
            framework,
            workspace_id,
            tree,
            inline,
            artifact_identities,
            parity_rows,
            attempt_packets,
        }
    }

    /// Returns true when every parity row carries the `rows_agree` state.
    pub fn parity_rows_all_agree(&self) -> bool {
        self.parity_rows
            .iter()
            .all(|row| row.agreement_state == TestRunnerBetaParityState::RowsAgree)
    }

    /// Returns true when no parity row is in a review-required state.
    pub fn parity_rows_safe_to_dispatch(&self) -> bool {
        self.parity_rows
            .iter()
            .all(|row| row.agreement_state != TestRunnerBetaParityState::SurfaceDisagreementRequiresReview)
    }
}

/// Beta support-export packet binding manifest, projections, artifact rows,
/// parity rows, and the underlying alpha packet refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestRunnerBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical alpha test-attempt model.
    pub test_attempt_schema_version: u32,
    /// Stable export id.
    pub support_export_id: String,
    /// Export timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Coverage manifest at export time.
    pub coverage_manifest: TestRunnerBetaCoverageManifest,
    /// Test-tree projection refs included in the export.
    pub tree_projection_refs: Vec<String>,
    /// Inline editor projection refs included in the export.
    pub inline_projection_refs: Vec<String>,
    /// Structured artifact identities included in the export.
    pub artifact_identities: Vec<TestArtifactIdentity>,
    /// Parity rows included in the export.
    pub parity_rows: Vec<TestRunnerBetaRerunParity>,
    /// Alpha test-attempt packet ids included in the export.
    pub attempt_packet_refs: Vec<String>,
    /// Underlying alpha support-export ids included for support replay.
    pub attempt_support_export_refs: Vec<String>,
    /// Reviewer-facing summary lines.
    pub summary_lines: Vec<String>,
}

impl TestRunnerBetaSupportExport {
    /// Builds a support-export packet for one beta projection.
    pub fn from_projection(
        projection: &TestRunnerBetaProjection,
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let coverage_manifest =
            TestRunnerBetaCoverageManifest::canonical(manifest_id, generated_at.clone());
        let tree_projection_refs = vec![projection.tree.tree_projection_id.clone()];
        let inline_projection_refs = vec![projection.inline.inline_projection_id.clone()];
        let attempt_packet_refs = projection
            .attempt_packets
            .iter()
            .map(|packet| packet.packet_id.clone())
            .collect();
        let attempt_support_export_refs = projection
            .attempt_packets
            .iter()
            .map(|packet| packet.support_export.support_export_id.clone())
            .collect();
        let summary_lines = projection
            .parity_rows
            .iter()
            .map(|row| {
                format!(
                    "framework={} session={} test={} agreement={} command={}",
                    row.framework_token,
                    row.test_session_ref,
                    row.canonical_test_item_ref,
                    row.agreement_state_token,
                    row.rerun_last_command_id
                )
            })
            .collect();
        Self {
            record_kind: TEST_RUNNER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            test_attempt_schema_version: TEST_RUNNER_BETA_SCHEMA_VERSION,
            support_export_id: format!(
                "test-runner-beta-support:{}:{}",
                stable_token(&projection.workspace_id),
                stable_token(&generated_at)
            ),
            generated_at,
            workspace_id: projection.workspace_id.clone(),
            coverage_manifest,
            tree_projection_refs,
            inline_projection_refs,
            artifact_identities: projection.artifact_identities.clone(),
            parity_rows: projection.parity_rows.clone(),
            attempt_packet_refs,
            attempt_support_export_refs,
            summary_lines,
        }
    }

    /// Renders deterministic plaintext lines for support exports.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Test runner support export: {}\n", self.support_export_id);
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Frameworks: {}\n",
            self.coverage_manifest
                .frameworks
                .iter()
                .map(|row| row.framework_token.clone())
                .collect::<Vec<_>>()
                .join(",")
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

fn build_pytest_tree(
    framework: TestRunnerBetaFramework,
    discovery: &PytestDiscovery,
    attempt_packets: &[TestAttemptAlphaPacket],
    generated_at: &str,
) -> TestTreeProjection {
    let workspace_id = discovery.workspace_id.clone();
    let target_id = discovery
        .execution_context
        .target_identity
        .canonical_target_id
        .clone();
    let execution_context_ref = discovery.execution_context.execution_context_id.clone();
    let projection_id = format!(
        "test-tree:{}:{}",
        stable_token(&workspace_id),
        stable_token(generated_at)
    );
    let root_row_id = format!("test-tree-row:{}:root", stable_token(&workspace_id));

    let mut rows: Vec<TestTreeRow> = Vec::new();
    let mut child_root_ids: Vec<String> = Vec::new();

    for file in &discovery.test_files {
        let file_row_id = format!(
            "test-tree-row:{}:file:{}",
            stable_token(&workspace_id),
            stable_token(&file.file_id)
        );
        child_root_ids.push(file_row_id.clone());
        let mut child_case_ids: Vec<String> = Vec::new();

        let case_descriptors = pytest_cases_in_file(discovery, file);
        for descriptor in case_descriptors {
            let canonical = canonical_test_item_ref(descriptor);
            let selector_ref = format!("selector:pytest:{}", stable_token(&descriptor.node_id));
            let case_row_id = format!(
                "test-tree-row:{}:case:{}",
                stable_token(&workspace_id),
                stable_token(&descriptor.test_item_id)
            );
            child_case_ids.push(case_row_id.clone());

            let attempt = latest_attempt_for_canonical(attempt_packets, &canonical);
            let session_ref = attempt
                .map(|att| att.parent_test_session_ref.clone())
                .or_else(|| {
                    matching_packet_session(attempt_packets, &canonical)
                        .map(|packet| packet.session_plan.test_session_id.clone())
                });
            let identity_stability_token = matching_packet_session(attempt_packets, &canonical)
                .map(|packet| packet.identity_projection.identity_stability_token.clone());
            let run_contract_ref = matching_run_contract(discovery, &canonical)
                .map(|contract| contract.run_contract_id.clone());

            let artifact_identity_refs = artifact_refs_for_canonical(attempt_packets, &canonical)
                .into_iter()
                .map(|art_ref| {
                    artifact_identity_id(&workspace_id, &canonical, &art_ref)
                })
                .collect();

            rows.push(TestTreeRow {
                record_kind: TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND.to_owned(),
                tree_row_id: case_row_id,
                row_kind: TestTreeRowKind::TestCase,
                row_kind_token: TestTreeRowKind::TestCase.as_str().to_owned(),
                parent_row_id: Some(file_row_id.clone()),
                display_label: descriptor.display_label.clone(),
                source_file_ref: Some(descriptor.source_ref.clone()),
                line_number: Some(descriptor.line_number),
                canonical_test_item_ref: Some(canonical.clone()),
                selector_ref: Some(selector_ref),
                identity_stability_token,
                latest_attempt_ref: attempt.map(|att| att.test_attempt_id.clone()),
                latest_result_state_token: attempt.map(|att| att.result_state_token.clone()),
                test_session_ref: session_ref,
                run_contract_ref,
                rerun_last_command_id: Some(framework.rerun_lane().command_id().to_owned()),
                child_row_ids: Vec::new(),
                artifact_identity_refs,
            });
        }

        rows.push(TestTreeRow {
            record_kind: TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND.to_owned(),
            tree_row_id: file_row_id.clone(),
            row_kind: TestTreeRowKind::TestFile,
            row_kind_token: TestTreeRowKind::TestFile.as_str().to_owned(),
            parent_row_id: Some(root_row_id.clone()),
            display_label: file.relative_path.clone(),
            source_file_ref: Some(file.source_ref.clone()),
            line_number: None,
            canonical_test_item_ref: None,
            selector_ref: None,
            identity_stability_token: None,
            latest_attempt_ref: None,
            latest_result_state_token: None,
            test_session_ref: None,
            run_contract_ref: None,
            rerun_last_command_id: None,
            child_row_ids: child_case_ids,
            artifact_identity_refs: Vec::new(),
        });
    }

    let root_row = TestTreeRow {
        record_kind: TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND.to_owned(),
        tree_row_id: root_row_id.clone(),
        row_kind: TestTreeRowKind::WorkspaceRoot,
        row_kind_token: TestTreeRowKind::WorkspaceRoot.as_str().to_owned(),
        parent_row_id: None,
        display_label: discovery.workspace_root_ref.clone(),
        source_file_ref: None,
        line_number: None,
        canonical_test_item_ref: None,
        selector_ref: None,
        identity_stability_token: None,
        latest_attempt_ref: None,
        latest_result_state_token: None,
        test_session_ref: None,
        run_contract_ref: None,
        rerun_last_command_id: None,
        child_row_ids: child_root_ids,
        artifact_identity_refs: Vec::new(),
    };

    let mut ordered = Vec::with_capacity(rows.len() + 1);
    ordered.push(root_row);
    ordered.extend(rows);

    TestTreeProjection {
        record_kind: TEST_RUNNER_BETA_TREE_PROJECTION_RECORD_KIND.to_owned(),
        test_attempt_schema_version: TEST_RUNNER_BETA_SCHEMA_VERSION,
        tree_projection_id: projection_id,
        workspace_id,
        framework,
        framework_token: framework.as_str().to_owned(),
        discovery_state_token: discovery.discovery_state.as_str().to_owned(),
        execution_context_ref,
        target_id,
        root_row_id,
        rows: ordered,
        honesty_marker_present: discovery.honesty_marker_present,
    }
}

fn build_pytest_inline_projection(
    framework: TestRunnerBetaFramework,
    discovery: &PytestDiscovery,
    tree: &TestTreeProjection,
    attempt_packets: &[TestAttemptAlphaPacket],
    generated_at: &str,
) -> InlineTestResultProjection {
    let workspace_id = discovery.workspace_id.clone();
    let target_id = discovery
        .execution_context
        .target_identity
        .canonical_target_id
        .clone();
    let execution_context_ref = discovery.execution_context.execution_context_id.clone();
    let projection_id = format!(
        "test-inline:{}:{}",
        stable_token(&workspace_id),
        stable_token(generated_at)
    );

    let mut rows: Vec<InlineTestResultRow> = Vec::new();
    for descriptor in &discovery.test_items {
        let canonical = canonical_test_item_ref(descriptor);
        let selector_ref = format!("selector:pytest:{}", stable_token(&descriptor.node_id));
        let attempt = latest_attempt_for_canonical(attempt_packets, &canonical);
        let session_ref = attempt
            .map(|att| att.parent_test_session_ref.clone())
            .or_else(|| {
                matching_packet_session(attempt_packets, &canonical)
                    .map(|packet| packet.session_plan.test_session_id.clone())
            })
            .unwrap_or_else(|| format!("test-session:unknown:{}", stable_token(&canonical)));
        let identity_stability_token = matching_packet_session(attempt_packets, &canonical)
            .map(|packet| packet.identity_projection.identity_stability_token.clone())
            .unwrap_or_else(|| "identity_unknown_requires_review".to_owned());
        let tree_row_ref = tree
            .case_row_for_test_item(&canonical)
            .map(|row| row.tree_row_id.clone())
            .unwrap_or_else(|| format!("test-tree-row:{}:case:unknown", stable_token(&workspace_id)));
        let artifact_identity_refs = artifact_refs_for_canonical(attempt_packets, &canonical)
            .into_iter()
            .map(|art_ref| artifact_identity_id(&workspace_id, &canonical, &art_ref))
            .collect();
        let summary = match attempt.map(|att| att.result_state_token.as_str()) {
            Some(token) => format!("{} -> {}", descriptor.display_label, token),
            None => format!("{} -> not yet run", descriptor.display_label),
        };
        rows.push(InlineTestResultRow {
            record_kind: TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND.to_owned(),
            inline_row_id: format!(
                "test-inline-row:{}:case:{}",
                stable_token(&workspace_id),
                stable_token(&descriptor.test_item_id)
            ),
            canonical_test_item_ref: canonical,
            selector_ref,
            source_file_ref: descriptor.source_ref.clone(),
            line_number: descriptor.line_number,
            identity_stability_token,
            test_session_ref: session_ref,
            latest_attempt_ref: attempt.map(|att| att.test_attempt_id.clone()),
            latest_result_state_token: attempt.map(|att| att.result_state_token.clone()),
            tree_row_ref,
            artifact_identity_refs,
            summary,
        });
    }

    InlineTestResultProjection {
        record_kind: TEST_RUNNER_BETA_INLINE_PROJECTION_RECORD_KIND.to_owned(),
        test_attempt_schema_version: TEST_RUNNER_BETA_SCHEMA_VERSION,
        inline_projection_id: projection_id,
        workspace_id,
        framework,
        framework_token: framework.as_str().to_owned(),
        execution_context_ref,
        target_id,
        tree_projection_ref: tree.tree_projection_id.clone(),
        rows,
    }
}

fn build_pytest_artifact_identities(
    framework: TestRunnerBetaFramework,
    attempt_packets: &[TestAttemptAlphaPacket],
) -> Vec<TestArtifactIdentity> {
    let mut identities: Vec<TestArtifactIdentity> = Vec::new();
    for packet in attempt_packets {
        for attempt in &packet.attempts {
            for art_ref in &attempt.artifact_refs {
                identities.push(artifact_identity_for_attempt(
                    framework,
                    attempt,
                    art_ref,
                    TestArtifactKind::classify_artifact_ref(art_ref),
                    &packet.identity_projection.identity_stability_token,
                ));
            }
            for raw_ref in &attempt.raw_event_refs {
                identities.push(artifact_identity_for_attempt(
                    framework,
                    attempt,
                    raw_ref,
                    TestArtifactKind::RawEventEnvelope,
                    &packet.identity_projection.identity_stability_token,
                ));
            }
        }
    }
    identities
}

fn build_pytest_parity_rows(
    framework: TestRunnerBetaFramework,
    tree: &TestTreeProjection,
    inline: &InlineTestResultProjection,
    last_launch: Option<&RerunLastLaunch>,
    prepared_rerun: Option<&RerunPreparedAttempt>,
) -> Vec<TestRunnerBetaRerunParity> {
    let mut parity_rows = Vec::new();
    for inline_row in &inline.rows {
        let Some(tree_row) = tree.case_row_for_test_item(&inline_row.canonical_test_item_ref)
        else {
            continue;
        };
        let lane = framework.rerun_lane();
        let row_lane_match = last_launch
            .map(|launch| launch.lane == lane)
            .unwrap_or(true);
        let row_specific_launch = if row_lane_match { last_launch } else { None };
        let row_specific_prepared = if row_lane_match { prepared_rerun } else { None };
        let row_specific_launch = row_specific_launch.filter(|launch| {
            matches!(
                &launch.contract,
                RerunRunContract::Pytest(contract) if contract.selection.test_item_id.as_deref()
                    == Some(&inline_row.canonical_test_item_ref)
                    || contract.selection.label == inline_row.canonical_test_item_ref
            )
        });
        let row_specific_prepared = if row_specific_launch.is_some() {
            row_specific_prepared
        } else {
            None
        };
        parity_rows.push(TestRunnerBetaRerunParity::evaluate(
            framework,
            tree_row,
            inline_row,
            row_specific_launch,
            row_specific_prepared,
        ));
    }
    parity_rows
}

fn pytest_cases_in_file<'a>(
    discovery: &'a PytestDiscovery,
    file: &PytestTestFileDescriptor,
) -> Vec<&'a PytestTestDescriptor> {
    discovery
        .test_items
        .iter()
        .filter(|descriptor| descriptor.source_file_ref == file.relative_path)
        .collect()
}

fn matching_packet_session<'a>(
    attempt_packets: &'a [TestAttemptAlphaPacket],
    canonical_test_item_ref: &str,
) -> Option<&'a TestAttemptAlphaPacket> {
    attempt_packets.iter().find(|packet| {
        packet
            .session_plan
            .canonical_test_item_refs
            .iter()
            .any(|item| item == canonical_test_item_ref)
            || packet
                .identity_projection
                .canonical_test_item_ref
                .as_deref()
                == Some(canonical_test_item_ref)
    })
}

fn latest_attempt_for_canonical<'a>(
    attempt_packets: &'a [TestAttemptAlphaPacket],
    canonical_test_item_ref: &str,
) -> Option<&'a TestAttemptRecord> {
    matching_packet_session(attempt_packets, canonical_test_item_ref)
        .and_then(|packet| packet.attempts.last())
}

fn matching_run_contract<'a>(
    discovery: &'a PytestDiscovery,
    canonical_test_item_ref: &str,
) -> Option<&'a PytestRunContract> {
    discovery.run_contracts.iter().find(|contract| {
        contract
            .selection
            .test_item_id
            .as_deref()
            .map(|item_id| item_id == canonical_test_item_ref)
            .unwrap_or(false)
            && matches!(contract.selection.kind, PytestSelectionKind::DiscoveredItem)
    })
}

fn artifact_refs_for_canonical(
    attempt_packets: &[TestAttemptAlphaPacket],
    canonical_test_item_ref: &str,
) -> Vec<String> {
    let mut refs: Vec<String> = Vec::new();
    if let Some(packet) = matching_packet_session(attempt_packets, canonical_test_item_ref) {
        for attempt in &packet.attempts {
            for art in &attempt.artifact_refs {
                if !refs.iter().any(|existing| existing == art) {
                    refs.push(art.clone());
                }
            }
            for raw in &attempt.raw_event_refs {
                if !refs.iter().any(|existing| existing == raw) {
                    refs.push(raw.clone());
                }
            }
        }
    }
    refs
}

fn artifact_identity_id(
    workspace_id: &str,
    canonical_test_item_ref: &str,
    artifact_ref: &str,
) -> String {
    format!(
        "test-artifact-identity:{}:{}:{}",
        stable_token(workspace_id),
        stable_token(canonical_test_item_ref),
        stable_token(artifact_ref)
    )
}

fn artifact_identity_for_attempt(
    framework: TestRunnerBetaFramework,
    attempt: &TestAttemptRecord,
    artifact_ref: &str,
    kind: TestArtifactKind,
    identity_stability_token: &str,
) -> TestArtifactIdentity {
    let canonical_test_item_refs = attempt.canonical_test_item_refs.clone();
    let canonical_for_id = canonical_test_item_refs
        .first()
        .cloned()
        .unwrap_or_else(|| attempt.parent_test_session_ref.clone());
    TestArtifactIdentity {
        record_kind: TEST_RUNNER_BETA_ARTIFACT_IDENTITY_RECORD_KIND.to_owned(),
        artifact_identity_id: artifact_identity_id(
            &attempt.parent_test_session_ref,
            &canonical_for_id,
            artifact_ref,
        ),
        artifact_kind: kind,
        artifact_kind_token: kind.as_str().to_owned(),
        artifact_ref: artifact_ref.to_owned(),
        test_session_ref: attempt.parent_test_session_ref.clone(),
        test_attempt_ref: attempt.test_attempt_id.clone(),
        canonical_test_item_refs,
        selector_ref: attempt.selector_ref.clone(),
        execution_context_ref: attempt.execution_context_ref.clone(),
        target_id: attempt.target_id.clone(),
        framework,
        framework_token: framework.as_str().to_owned(),
        producing_run_ref: attempt
            .execution_attempt_ref
            .clone()
            .unwrap_or_else(|| attempt.test_attempt_id.clone()),
        producing_execution_attempt_ref: attempt.execution_attempt_ref.clone(),
        identity_stability_token: identity_stability_token.to_owned(),
    }
}

fn canonical_test_item_ref(descriptor: &PytestTestDescriptor) -> String {
    descriptor.test_item_id.clone()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_manifest_carries_pytest_framework_with_test_wedge() {
        let manifest = TestRunnerBetaCoverageManifest::canonical(
            "test-runner-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert_eq!(
            manifest.record_kind,
            TEST_RUNNER_BETA_COVERAGE_MANIFEST_RECORD_KIND
        );
        assert_eq!(
            manifest.test_attempt_schema_version,
            TEST_RUNNER_BETA_SCHEMA_VERSION
        );
        let pytest = manifest
            .row_for_framework(TestRunnerBetaFramework::Pytest)
            .expect("pytest row");
        assert_eq!(pytest.framework_token, "pytest");
        assert_eq!(pytest.wedge, TaskWedgeClass::Test);
        assert_eq!(pytest.rerun_lane, RerunLane::Test);
        assert_eq!(pytest.rerun_last_command_id, "cmd:test.rerun_last");
        assert!(pytest.consumer_surface_tokens.contains(&"test_tree".to_owned()));
        assert!(pytest.consumer_surface_tokens.contains(&"editor_inline".to_owned()));
        assert!(pytest.claimed_artifact_kind_tokens.contains(&"run_report".to_owned()));
        assert!(pytest.claimed_artifact_kind_tokens.contains(&"raw_event_envelope".to_owned()));
    }

    #[test]
    fn coverage_manifest_does_not_claim_unsupported_wedges() {
        let manifest = TestRunnerBetaCoverageManifest::canonical(
            "test-runner-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert!(manifest.covers_wedges(&[TaskWedgeClass::Test]));
        assert!(!manifest.covers_wedges(&[TaskWedgeClass::Build]));
    }

    #[test]
    fn artifact_kind_classification_routes_keywords() {
        assert_eq!(
            TestArtifactKind::classify_artifact_ref("artifact:coverage:42"),
            TestArtifactKind::CoverageReport
        );
        assert_eq!(
            TestArtifactKind::classify_artifact_ref("artifact:test-attempt:run"),
            TestArtifactKind::RunReport
        );
        assert_eq!(
            TestArtifactKind::classify_artifact_ref("artifact:snapshot:001"),
            TestArtifactKind::SnapshotDiff
        );
        assert_eq!(
            TestArtifactKind::classify_artifact_ref("raw-event:test-attempt:trace"),
            TestArtifactKind::RawEventEnvelope
        );
    }

    #[test]
    fn parity_evaluator_marks_rows_agree_on_matching_identities() {
        let tree_row = TestTreeRow {
            record_kind: TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND.to_owned(),
            tree_row_id: "test-tree-row:case:health".to_owned(),
            row_kind: TestTreeRowKind::TestCase,
            row_kind_token: TestTreeRowKind::TestCase.as_str().to_owned(),
            parent_row_id: Some("test-tree-row:file:test_api".to_owned()),
            display_label: "test_health".to_owned(),
            source_file_ref: Some("tests/test_api.py".to_owned()),
            line_number: Some(7),
            canonical_test_item_ref: Some("pytest:tests_test_api_test_health".to_owned()),
            selector_ref: Some("selector:pytest:tests_test_api_test_health".to_owned()),
            identity_stability_token: Some("stable".to_owned()),
            latest_attempt_ref: Some("test-attempt:1".to_owned()),
            latest_result_state_token: Some("passed".to_owned()),
            test_session_ref: Some("test-session:health".to_owned()),
            run_contract_ref: None,
            rerun_last_command_id: Some("cmd:test.rerun_last".to_owned()),
            child_row_ids: Vec::new(),
            artifact_identity_refs: Vec::new(),
        };
        let inline_row = InlineTestResultRow {
            record_kind: TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND.to_owned(),
            inline_row_id: "test-inline-row:case:health".to_owned(),
            canonical_test_item_ref: "pytest:tests_test_api_test_health".to_owned(),
            selector_ref: "selector:pytest:tests_test_api_test_health".to_owned(),
            source_file_ref: "tests/test_api.py".to_owned(),
            line_number: 7,
            identity_stability_token: "stable".to_owned(),
            test_session_ref: "test-session:health".to_owned(),
            latest_attempt_ref: Some("test-attempt:1".to_owned()),
            latest_result_state_token: Some("passed".to_owned()),
            tree_row_ref: "test-tree-row:case:health".to_owned(),
            artifact_identity_refs: Vec::new(),
            summary: "test_health -> passed".to_owned(),
        };
        let parity = TestRunnerBetaRerunParity::evaluate(
            TestRunnerBetaFramework::Pytest,
            &tree_row,
            &inline_row,
            None,
            None,
        );
        assert_eq!(
            parity.agreement_state,
            TestRunnerBetaParityState::RerunLaneUnset
        );
        assert_eq!(parity.tree_row_ref, "test-tree-row:case:health");
        assert_eq!(parity.inline_row_ref, "test-inline-row:case:health");
        assert_eq!(parity.rerun_last_command_id, "cmd:test.rerun_last");
    }

    #[test]
    fn parity_evaluator_marks_disagreement_when_inline_pointer_drifts() {
        let tree_row = TestTreeRow {
            record_kind: TEST_RUNNER_BETA_TREE_ROW_RECORD_KIND.to_owned(),
            tree_row_id: "test-tree-row:case:health".to_owned(),
            row_kind: TestTreeRowKind::TestCase,
            row_kind_token: TestTreeRowKind::TestCase.as_str().to_owned(),
            parent_row_id: None,
            display_label: "test_health".to_owned(),
            source_file_ref: None,
            line_number: None,
            canonical_test_item_ref: Some("pytest:tests_test_api_test_health".to_owned()),
            selector_ref: None,
            identity_stability_token: None,
            latest_attempt_ref: None,
            latest_result_state_token: None,
            test_session_ref: Some("test-session:health".to_owned()),
            run_contract_ref: None,
            rerun_last_command_id: None,
            child_row_ids: Vec::new(),
            artifact_identity_refs: Vec::new(),
        };
        let inline_row = InlineTestResultRow {
            record_kind: TEST_RUNNER_BETA_INLINE_ROW_RECORD_KIND.to_owned(),
            inline_row_id: "test-inline-row:case:health".to_owned(),
            canonical_test_item_ref: "pytest:tests_test_api_test_health".to_owned(),
            selector_ref: "selector:pytest:tests_test_api_test_health".to_owned(),
            source_file_ref: "tests/test_api.py".to_owned(),
            line_number: 7,
            identity_stability_token: "stable".to_owned(),
            test_session_ref: "test-session:health".to_owned(),
            latest_attempt_ref: None,
            latest_result_state_token: None,
            tree_row_ref: "test-tree-row:case:OTHER".to_owned(),
            artifact_identity_refs: Vec::new(),
            summary: "drifted".to_owned(),
        };
        let parity = TestRunnerBetaRerunParity::evaluate(
            TestRunnerBetaFramework::Pytest,
            &tree_row,
            &inline_row,
            None,
            None,
        );
        assert_eq!(
            parity.agreement_state,
            TestRunnerBetaParityState::SurfaceDisagreementRequiresReview
        );
    }

    #[test]
    fn coverage_manifest_round_trips_through_serde() {
        let manifest = TestRunnerBetaCoverageManifest::canonical(
            "test-runner-beta:test",
            "2026-05-15T00:00:00Z",
        );
        let json = serde_json::to_string(&manifest).expect("serialize manifest");
        let round: TestRunnerBetaCoverageManifest =
            serde_json::from_str(&json).expect("deserialize manifest");
        assert_eq!(round, manifest);
    }
}
