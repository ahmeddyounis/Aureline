//! Pytest discovery and run contracts for Python workspaces.
//!
//! This module reads pytest-compatible test files, combines the findings with
//! the canonical execution-context resolver and the Python environment
//! detector, and emits direct runner contracts that project into the shared
//! task-event stream. The contract stores a runner program plus argv and a
//! stable pytest selector; it does not rely on shell-only command text or ask
//! users to re-enter selectors for reruns.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::detectors::python::{
    PythonEnvironmentDetection, PythonEnvironmentDetector, PythonEnvironmentDetectorConfig,
    PythonEnvironmentManagerKind, PythonEnvironmentResolutionState, PythonEnvironmentSourceKind,
};
use crate::execution_context::{
    DegradedFieldReason, DegradedFieldRecord, ExecutionContext, ReachabilityState,
};
use crate::provenance::ExecutionEventProvenance;
use crate::tasks::{
    RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskBlockReason, TaskEvent,
    TaskEventConfidence, TaskEventIdentity, TaskEventKind, TaskEventPayload, TaskEventProvenance,
    TaskEventRedactionClass, TaskEventSourceKind, TaskEventStream, TaskEventStreamError,
    TaskShellProjection, TaskStateClass, TaskSupportExport, TaskWedgeClass,
    RAW_TASK_EVENT_ENVELOPE_RECORD_KIND, TASK_EVENT_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
};
use crate::tests::{TestAttemptAlphaPacket, TestAttemptSupportExport, TestWatchState};
use crate::TrustState;

/// Schema version for [`PytestDiscovery`] and [`PytestRunContract`] records.
pub const PYTEST_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a pytest discovery result.
pub const PYTEST_DISCOVERY_RECORD_KIND: &str = "pytest_discovery_record";

/// Stable record-kind tag for one pytest run contract.
pub const PYTEST_RUN_CONTRACT_RECORD_KIND: &str = "pytest_run_contract_record";

/// Stable implementation version recorded in discovery reports and task
/// event provenance.
pub const PYTEST_DISCOVERER_VERSION: &str = "pytest.discovery.alpha.v1";

/// Configuration for [`PytestDiscoverer`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PytestDiscovererConfig {
    /// Python detector configuration used before pytest run contracts are
    /// materialized.
    pub python_detector: PythonEnvironmentDetectorConfig,
    /// Workspace revision captured by the caller, when known.
    pub workspace_revision: Option<String>,
}

/// Read-only pytest discoverer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PytestDiscoverer {
    config: PytestDiscovererConfig,
}

impl PytestDiscoverer {
    /// Creates a discoverer with caller-provided detector facts.
    pub fn new(config: PytestDiscovererConfig) -> Self {
        Self { config }
    }

    /// Creates a discoverer with no caller-provided runtime facts.
    pub fn default_read_only() -> Self {
        Self::new(PytestDiscovererConfig::default())
    }

    /// Discovers pytest tests for a workspace root.
    ///
    /// The caller supplies a freshly resolved [`ExecutionContext`]. The
    /// discoverer attaches a read-only Python detector report to that context,
    /// performs bounded static pytest test-file discovery, and returns test
    /// descriptors plus run contracts that use the shared task-event model.
    pub fn discover_workspace(
        &self,
        workspace_root: &Path,
        context: ExecutionContext,
        discovered_at: &str,
    ) -> PytestDiscovery {
        let python_detection = PythonEnvironmentDetector::new(self.config.python_detector.clone())
            .detect_workspace(workspace_root, discovered_at);
        let mut context = context.with_python_environment_detection(python_detection.clone());
        let runtime_status = PytestRuntimeStatus::from_detection(&python_detection);
        let read = read_pytest_workspace(workspace_root);

        let workspace_id = context.invocation_subject.workspace_id.clone();
        let workspace_root_ref = workspace_root.display().to_string();
        let pytest_config_refs = pytest_config_refs(workspace_root);

        if matches!(
            read.discovery_state,
            PytestDiscoveryState::WorkspaceUnreadable | PytestDiscoveryState::Partial
        ) {
            context.degraded_fields.push(DegradedFieldRecord {
                field_path: "pytest_discovery.test_files".to_owned(),
                reason: DegradedFieldReason::ProvenanceGap,
                repair_hook_ref: Some("doctor.repair.pytest_discovery".to_owned()),
            });
        }

        let run_contracts = build_run_contracts(
            &workspace_id,
            &context,
            &runtime_status,
            &read.discovery_state,
            &read.test_files,
            &read.test_items,
            &self.config.workspace_revision,
        );
        let honesty_marker_present = runtime_status.has_missing_or_blocked_runtime()
            || !context.degraded_fields.is_empty()
            || !read.issues.is_empty()
            || run_contracts
                .iter()
                .any(|contract| contract.readiness.requires_honesty_marker())
            || matches!(
                read.discovery_state,
                PytestDiscoveryState::NoTestsDiscovered
                    | PytestDiscoveryState::Partial
                    | PytestDiscoveryState::WorkspaceUnreadable
            );

        PytestDiscovery {
            record_kind: PYTEST_DISCOVERY_RECORD_KIND.to_owned(),
            schema_version: PYTEST_DISCOVERY_SCHEMA_VERSION,
            discoverer_version: PYTEST_DISCOVERER_VERSION.to_owned(),
            workspace_id,
            workspace_root_ref,
            pytest_config_refs,
            discovered_at: discovered_at.to_owned(),
            discovery_state: read.discovery_state,
            runtime_status,
            execution_context: context,
            test_files: read.test_files,
            test_items: read.test_items,
            issues: read.issues,
            run_contracts,
            honesty_marker_present,
        }
    }
}

/// Top-level pytest discovery record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestDiscovery {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Discoverer implementation version.
    pub discoverer_version: String,
    /// Workspace id copied from the execution context.
    pub workspace_id: String,
    /// Workspace root inspected by the discoverer.
    pub workspace_root_ref: String,
    /// Pytest config files found at the workspace root.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pytest_config_refs: Vec<String>,
    /// Timestamp supplied by the caller.
    pub discovered_at: String,
    /// Overall discovery state.
    pub discovery_state: PytestDiscoveryState,
    /// Runtime status derived from the Python detector.
    pub runtime_status: PytestRuntimeStatus,
    /// Canonical execution context with the Python detector report attached.
    pub execution_context: ExecutionContext,
    /// Pytest-compatible files considered by this discovery pass.
    pub test_files: Vec<PytestTestFileDescriptor>,
    /// Test items discovered from pytest-compatible files.
    pub test_items: Vec<PytestTestDescriptor>,
    /// Discovery issues that must remain visible.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub issues: Vec<PytestDiscoveryIssue>,
    /// Runnable contracts for discovered pytest selections.
    pub run_contracts: Vec<PytestRunContract>,
    /// True when discovery or at least one contract must render an honesty
    /// marker before launch.
    pub honesty_marker_present: bool,
}

impl PytestDiscovery {
    /// Returns the all-discovered-tests contract, if one was produced.
    pub fn all_tests_contract(&self) -> Option<&PytestRunContract> {
        self.run_contracts
            .iter()
            .find(|contract| contract.selection.kind == PytestSelectionKind::AllDiscovered)
    }

    /// Returns the run contract for a pytest node id.
    pub fn contract_for_selector(&self, selector: &str) -> Option<&PytestRunContract> {
        self.run_contracts.iter().find(|contract| {
            contract.selection.selector.as_deref() == Some(selector)
                || contract
                    .selection
                    .test_item_id
                    .as_deref()
                    .map(|item_id| item_id == selector)
                    .unwrap_or(false)
        })
    }

    /// Returns shell projections for all pytest run contracts.
    ///
    /// This is the first runtime consumer: pytest contracts are projected
    /// through the canonical task-event stream rather than a Python-only event
    /// grammar.
    pub fn shell_projections(&self, observed_at: &str) -> Vec<TaskShellProjection> {
        self.run_contracts
            .iter()
            .flat_map(|contract| match contract.launch_event_stream(observed_at) {
                Ok(stream) => stream.shell_projection(),
                Err(_) => Vec::new(),
            })
            .collect()
    }

    /// Returns support exports for all pytest run contracts.
    pub fn support_exports(&self, generated_at: &str) -> Vec<TaskSupportExport> {
        self.run_contracts
            .iter()
            .filter_map(|contract| {
                let stream = contract.launch_event_stream(generated_at).ok()?;
                Some(stream.support_export(
                    format!("support-export:{}", contract.run_contract_id),
                    generated_at.to_owned(),
                ))
            })
            .collect()
    }

    /// Returns test-attempt alpha packets for all pytest run contracts.
    ///
    /// This is the first runtime consumer for the test session/attempt alpha
    /// model: pytest launch-wedge rows expose identity, session plan, attempt
    /// ledger, watch state, imported-CI projection, and support/export state
    /// from the same run contract that feeds task events.
    pub fn test_attempt_alpha_packets(&self, generated_at: &str) -> Vec<TestAttemptAlphaPacket> {
        self.run_contracts
            .iter()
            .map(|contract| {
                TestAttemptAlphaPacket::from_pytest_contract(
                    contract,
                    &self.execution_context,
                    generated_at,
                )
            })
            .collect()
    }

    /// Returns watch-mode test-attempt alpha packets for all pytest run contracts.
    pub fn test_attempt_alpha_watch_packets(
        &self,
        watch_state: TestWatchState,
        generated_at: &str,
    ) -> Vec<TestAttemptAlphaPacket> {
        self.run_contracts
            .iter()
            .map(|contract| {
                TestAttemptAlphaPacket::from_pytest_watch_contract(
                    contract,
                    &self.execution_context,
                    watch_state,
                    generated_at,
                )
            })
            .collect()
    }

    /// Returns support/export projections from test-attempt alpha packets.
    pub fn test_attempt_alpha_support_exports(
        &self,
        generated_at: &str,
    ) -> Vec<TestAttemptSupportExport> {
        self.test_attempt_alpha_packets(generated_at)
            .into_iter()
            .map(|packet| packet.support_export)
            .collect()
    }
}

/// Overall pytest discovery state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestDiscoveryState {
    /// At least one pytest item and run contract was produced.
    Ready,
    /// No pytest-compatible test file was discovered.
    NoTestsDiscovered,
    /// The workspace root could not be read.
    WorkspaceUnreadable,
    /// At least one file could not be inspected, but discovery yielded usable
    /// results.
    Partial,
}

impl PytestDiscoveryState {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::NoTestsDiscovered => "no_tests_discovered",
            Self::WorkspaceUnreadable => "workspace_unreadable",
            Self::Partial => "partial",
        }
    }
}

/// Runtime status derived from the embedded Python detector report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestRuntimeStatus {
    /// Python interpreter resolution state.
    pub interpreter_resolution_state: PythonEnvironmentResolutionState,
    /// Environment-manager resolution state.
    pub environment_manager_resolution_state: PythonEnvironmentResolutionState,
    /// Resolved Python token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python_value_token: Option<String>,
    /// Resolved interpreter reference when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpreter_ref: Option<String>,
    /// Resolved environment-manager token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_manager_value_token: Option<String>,
    /// Resolved environment-manager kind when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_manager_kind: Option<PythonEnvironmentManagerKind>,
    /// Missing, ambiguous, or unsupported runtime states that must be visible
    /// on discovery rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_runtime_states: Vec<PytestMissingRuntimeState>,
}

impl PytestRuntimeStatus {
    /// Builds a runtime status from a Python detector report.
    pub fn from_detection(detection: &PythonEnvironmentDetection) -> Self {
        let python_value_token = python_value_token(detection);
        let interpreter_ref = detection.interpreter.interpreter_ref.clone();
        let environment_manager_value_token = environment_manager_value_token(detection);
        let mut missing_runtime_states = Vec::new();
        push_runtime_state(
            &mut missing_runtime_states,
            detection.interpreter.resolution_state,
            PytestMissingRuntimeState::PythonInterpreterMissing,
            PytestMissingRuntimeState::PythonInterpreterAmbiguous,
            PytestMissingRuntimeState::PythonInterpreterUnsupported,
        );
        push_runtime_state(
            &mut missing_runtime_states,
            detection.environment_manager.resolution_state,
            PytestMissingRuntimeState::EnvironmentManagerMissing,
            PytestMissingRuntimeState::EnvironmentManagerAmbiguous,
            PytestMissingRuntimeState::EnvironmentManagerUnsupported,
        );

        Self {
            interpreter_resolution_state: detection.interpreter.resolution_state,
            environment_manager_resolution_state: detection.environment_manager.resolution_state,
            python_value_token,
            interpreter_ref,
            environment_manager_value_token,
            environment_manager_kind: detection.environment_manager.kind,
            missing_runtime_states,
        }
    }

    /// True when a launch must block or render a prominent runtime honesty
    /// marker.
    pub fn has_missing_or_blocked_runtime(&self) -> bool {
        !self.missing_runtime_states.is_empty()
    }
}

/// Missing or blocked runtime state that discovery must disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestMissingRuntimeState {
    /// Python interpreter could not be resolved.
    PythonInterpreterMissing,
    /// Python interpreter pins are ambiguous.
    PythonInterpreterAmbiguous,
    /// Python interpreter resolved outside the Python launch-wedge contract.
    PythonInterpreterUnsupported,
    /// Environment manager could not be resolved.
    EnvironmentManagerMissing,
    /// Environment manager pins are ambiguous.
    EnvironmentManagerAmbiguous,
    /// Environment manager is outside the Python launch-wedge contract.
    EnvironmentManagerUnsupported,
}

impl PytestMissingRuntimeState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PythonInterpreterMissing => "python_interpreter_missing",
            Self::PythonInterpreterAmbiguous => "python_interpreter_ambiguous",
            Self::PythonInterpreterUnsupported => "python_interpreter_unsupported",
            Self::EnvironmentManagerMissing => "environment_manager_missing",
            Self::EnvironmentManagerAmbiguous => "environment_manager_ambiguous",
            Self::EnvironmentManagerUnsupported => "environment_manager_unsupported",
        }
    }
}

/// Discovery issue class for pytest static inspection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestDiscoveryIssueKind {
    /// Workspace root could not be read.
    WorkspaceUnreadable,
    /// A pytest-compatible file could not be read.
    TestFileUnreadable,
}

impl PytestDiscoveryIssueKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceUnreadable => "workspace_unreadable",
            Self::TestFileUnreadable => "test_file_unreadable",
        }
    }
}

/// Discovery issue that must be surfaced before launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestDiscoveryIssue {
    /// Issue class.
    pub issue_kind: PytestDiscoveryIssueKind,
    /// Workspace-relative source reference when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Export-safe issue summary.
    pub summary: String,
}

/// Source kind for a discovered pytest item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestSourceKind {
    /// Pytest-compatible Python source file.
    PytestFile,
}

impl PytestSourceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PytestFile => "pytest_file",
        }
    }
}

/// Test item kind discovered by the alpha pytest scanner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestTestKind {
    /// Top-level pytest function.
    Function,
    /// Pytest method inside a `Test*` class.
    Method,
}

impl PytestTestKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Method => "method",
        }
    }
}

/// One pytest-compatible source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestTestFileDescriptor {
    /// Stable file id inside the workspace.
    pub file_id: String,
    /// Workspace-relative path.
    pub relative_path: String,
    /// Reviewable source reference.
    pub source_ref: String,
    /// Number of test items discovered in the file.
    pub test_count: u32,
}

/// One pytest item discovered from a Python source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestTestDescriptor {
    /// Stable test item id inside the workspace.
    pub test_item_id: String,
    /// Pytest node id used as the runner selector.
    pub node_id: String,
    /// Export-safe display label.
    pub display_label: String,
    /// Test item kind.
    pub kind: PytestTestKind,
    /// Source kind.
    pub source_kind: PytestSourceKind,
    /// Workspace-relative source file.
    pub source_file_ref: String,
    /// One-based line number.
    pub line_number: u32,
    /// Reviewable source reference.
    pub source_ref: String,
    /// Task wedge the item maps to.
    pub wedge: TaskWedgeClass,
}

/// Pytest runner invocation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestInvocationMode {
    /// `uv run pytest`.
    UvRun,
    /// `poetry run pytest`.
    PoetryRun,
    /// `<python> -m pytest`.
    PythonModule,
}

impl PytestInvocationMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UvRun => "uv_run",
            Self::PoetryRun => "poetry_run",
            Self::PythonModule => "python_module",
        }
    }
}

/// Pytest runner for a run contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestRunner {
    /// Environment-manager family when one resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_manager_kind: Option<PythonEnvironmentManagerKind>,
    /// Program name or interpreter path to execute directly.
    pub program: String,
    /// Version token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Interpreter reference when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpreter_ref: Option<String>,
    /// Invocation mode.
    pub invocation_mode: PytestInvocationMode,
}

/// Selection kind for one pytest run contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestSelectionKind {
    /// All items discovered in the current discovery pass.
    AllDiscovered,
    /// One discovered pytest node id.
    DiscoveredItem,
}

impl PytestSelectionKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllDiscovered => "all_discovered",
            Self::DiscoveredItem => "discovered_item",
        }
    }
}

/// Pytest selection preserved on a run contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestRunSelection {
    /// Selection kind.
    pub kind: PytestSelectionKind,
    /// Pytest node id when this contract targets one discovered item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Matching test item id when this contract targets one discovered item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_item_id: Option<String>,
    /// Export-safe label for picker rows and support exports.
    pub label: String,
    /// Source refs covered by this selection.
    pub source_refs: Vec<String>,
}

/// Process dispatch record for one pytest attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestDispatch {
    /// Program to spawn directly.
    pub program: String,
    /// Argument vector passed to the program.
    pub args: Vec<String>,
    /// Working directory copied from the execution context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Invocation mode used by Aureline.
    pub invocation_mode: PytestInvocationMode,
    /// Pytest selector preserved for reruns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// Source refs covered by this dispatch.
    pub source_refs: Vec<String>,
}

/// Launch readiness for one pytest run contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestLaunchReadiness {
    /// The contract can launch without extra honesty markers.
    Ready,
    /// The contract can launch, but fallback or ambient runtime truth must
    /// remain visible.
    ReadyWithHonestyMarker,
    /// The contract cannot safely launch.
    Blocked,
}

impl PytestLaunchReadiness {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReadyWithHonestyMarker => "ready_with_honesty_marker",
            Self::Blocked => "blocked",
        }
    }

    /// True when a surface must render an honesty marker.
    pub const fn requires_honesty_marker(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// True when dispatch is blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// Blocker on a pytest run contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestBlockReason {
    /// Workspace trust has not been accepted.
    PendingTrust,
    /// Python interpreter is missing.
    MissingPythonInterpreter,
    /// Python interpreter pins are ambiguous.
    AmbiguousPythonInterpreter,
    /// Python interpreter is unsupported by the Python launch wedge.
    UnsupportedPythonInterpreter,
    /// Environment manager is missing.
    MissingEnvironmentManager,
    /// Environment manager pins are ambiguous.
    AmbiguousEnvironmentManager,
    /// Environment manager is unsupported by the Python launch wedge.
    UnsupportedEnvironmentManager,
    /// Context target is not reachable.
    TargetUnavailable,
    /// Policy or trust blocked an activator.
    PolicyOrTrustBlocked,
    /// Another degraded execution-context field blocks dispatch.
    DegradedExecutionContext,
}

impl PytestBlockReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingTrust => "pending_trust",
            Self::MissingPythonInterpreter => "missing_python_interpreter",
            Self::AmbiguousPythonInterpreter => "ambiguous_python_interpreter",
            Self::UnsupportedPythonInterpreter => "unsupported_python_interpreter",
            Self::MissingEnvironmentManager => "missing_environment_manager",
            Self::AmbiguousEnvironmentManager => "ambiguous_environment_manager",
            Self::UnsupportedEnvironmentManager => "unsupported_environment_manager",
            Self::TargetUnavailable => "target_unavailable",
            Self::PolicyOrTrustBlocked => "policy_or_trust_blocked",
            Self::DegradedExecutionContext => "degraded_execution_context",
        }
    }
}

/// Non-blocking honesty marker for a pytest contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestWarningClass {
    /// Python came from captured ambient PATH facts rather than a workspace pin.
    AmbientPythonInterpreter,
    /// Environment manager fell back to the detector default.
    EnvironmentManagerFallback,
    /// The execution context carries a toolchain fallback marker.
    ToolchainFallback,
    /// Discovery was partial because at least one source could not be read.
    PartialDiscovery,
}

impl PytestWarningClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbientPythonInterpreter => "ambient_python_interpreter",
            Self::EnvironmentManagerFallback => "environment_manager_fallback",
            Self::ToolchainFallback => "toolchain_fallback",
            Self::PartialDiscovery => "partial_discovery",
        }
    }
}

/// Rerun mode selected for a pytest contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PytestRerunMode {
    /// Preserve the original execution-context reference.
    ExactContext,
    /// Use the current freshly resolved execution context.
    CurrentContext,
}

impl PytestRerunMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactContext => "exact_context",
            Self::CurrentContext => "current_context",
        }
    }
}

/// Rerun lineage attached to a pytest attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestRerunLineage {
    /// Rerun mode.
    pub mode: PytestRerunMode,
    /// Previous attempt id.
    pub previous_attempt_id: String,
    /// Context used by the previous attempt.
    pub previous_execution_context_ref: String,
    /// Context used by the new attempt.
    pub current_execution_context_ref: String,
    /// True when the new attempt uses a different context id.
    pub context_changed: bool,
    /// Timestamp supplied by the rerun caller.
    pub rerun_observed_at: String,
}

/// Launch and rerun contract for one pytest selection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PytestRunContract {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub run_contract_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Stable task id for this pytest selection.
    pub task_id: String,
    /// Stable run id across attempts.
    pub run_id: String,
    /// Attempt id for this dispatch.
    pub attempt_id: String,
    /// Attempt number within the run.
    pub attempt_number: u32,
    /// Task-event stream id.
    pub task_stream_id: String,
    /// Trace id for task-event projections.
    pub trace_id: String,
    /// Execution-context id used by this attempt.
    pub execution_context_ref: String,
    /// Target id copied from the execution context.
    pub target_id: String,
    /// Redaction-safe execution-context provenance copied from the resolved context.
    pub context_provenance: ExecutionEventProvenance,
    /// Pytest selection.
    pub selection: PytestRunSelection,
    /// Task wedge classification.
    pub wedge: TaskWedgeClass,
    /// Runner when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runner: Option<PytestRunner>,
    /// Direct-process dispatch when runnable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatch: Option<PytestDispatch>,
    /// Launch readiness.
    pub readiness: PytestLaunchReadiness,
    /// Blocking reasons.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blockers: Vec<PytestBlockReason>,
    /// Non-blocking honesty markers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<PytestWarningClass>,
    /// Workspace revision captured by the caller, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
    /// Rerun lineage when this contract represents a retry or rerun attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_lineage: Option<PytestRerunLineage>,
}

impl PytestRunContract {
    /// Builds canonical task events for this attempt.
    ///
    /// Ready contracts emit queued and started events. Blocked contracts emit
    /// queued and blocked events, preserving the same task identity and raw
    /// envelope retention path.
    pub fn launch_event_stream(
        &self,
        observed_at: &str,
    ) -> Result<TaskEventStream, TaskEventStreamError> {
        let mut stream = TaskEventStream::new(
            self.task_stream_id.clone(),
            self.workspace_id.clone(),
            self.trace_id.clone(),
        );
        stream.append(self.event(
            sequence_for_attempt(self.attempt_number, 1),
            TaskEventKind::TaskQueued,
            TaskStateClass::Queued,
            TaskEventPayload::Lifecycle {
                lifecycle_reason: Some("pytest selection queued".to_owned()),
                exit_status: None,
            },
            observed_at,
        ))?;

        if self.readiness.is_blocked() {
            stream.append(self.event(
                sequence_for_attempt(self.attempt_number, 2),
                TaskEventKind::TaskBlocked,
                TaskStateClass::Blocked,
                TaskEventPayload::Blocked {
                    reason: self.task_block_reason(),
                    unblock_ref: Some("doctor.repair.python_environment".to_owned()),
                },
                observed_at,
            ))?;
        } else {
            stream.append(self.event(
                sequence_for_attempt(self.attempt_number, 2),
                TaskEventKind::TaskStarted,
                TaskStateClass::Running,
                TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("pytest runner started".to_owned()),
                    exit_status: None,
                },
                observed_at,
            ))?;
        }
        Ok(stream)
    }

    /// Builds a rerun contract from a freshly resolved current context.
    ///
    /// The method keeps the run id and pytest selector stable, increments the
    /// attempt identity, updates target and cwd from the supplied context when
    /// [`PytestRerunMode::CurrentContext`] is selected, and records
    /// exact-vs-current context drift in [`PytestRerunLineage`].
    pub fn rerun_with_context(
        &self,
        current_context: &ExecutionContext,
        mode: PytestRerunMode,
        observed_at: &str,
    ) -> Self {
        let mut next = self.clone();
        let previous_attempt_id = self.attempt_id.clone();
        let previous_execution_context_ref = self.execution_context_ref.clone();
        next.attempt_number = self.attempt_number.saturating_add(1);
        next.attempt_id = format!("attempt:{}:{}", self.task_id, next.attempt_number);

        if mode == PytestRerunMode::CurrentContext {
            next.execution_context_ref = current_context.execution_context_id.clone();
            next.target_id = current_context.target_identity.canonical_target_id.clone();
            next.context_provenance = ExecutionEventProvenance::from_context(current_context);
            if let Some(dispatch) = &mut next.dispatch {
                dispatch.working_directory =
                    current_context.target_identity.working_directory.clone();
            }
        }

        next.rerun_lineage = Some(PytestRerunLineage {
            mode,
            previous_attempt_id,
            previous_execution_context_ref: previous_execution_context_ref.clone(),
            current_execution_context_ref: next.execution_context_ref.clone(),
            context_changed: previous_execution_context_ref != next.execution_context_ref,
            rerun_observed_at: observed_at.to_owned(),
        });
        next
    }

    fn event(
        &self,
        sequence: u64,
        event_kind: TaskEventKind,
        state_after: TaskStateClass,
        payload: TaskEventPayload,
        observed_at: &str,
    ) -> TaskEvent {
        let event_id = format!(
            "event:{}:{}:{}",
            self.attempt_id,
            event_kind.as_str(),
            sequence
        );
        let source_kind = if self.readiness.is_blocked() {
            TaskEventSourceKind::StructuredOutput
        } else {
            TaskEventSourceKind::Native
        };
        let confidence = if self.readiness == PytestLaunchReadiness::Ready {
            TaskEventConfidence::High
        } else if self.readiness == PytestLaunchReadiness::ReadyWithHonestyMarker {
            TaskEventConfidence::MediumHigh
        } else {
            TaskEventConfidence::Medium
        };
        let identity = TaskEventIdentity {
            task_id: self.task_id.clone(),
            run_id: self.run_id.clone(),
            attempt_id: self.attempt_id.clone(),
            workspace_id: self.workspace_id.clone(),
            trace_id: self.trace_id.clone(),
            execution_context_id: self.execution_context_ref.clone(),
            target_id: self.target_id.clone(),
            wedge: self.wedge,
        };
        TaskEvent {
            record_kind: TASK_EVENT_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            event_id: event_id.clone(),
            stream_id: self.task_stream_id.clone(),
            stream_sequence: sequence,
            identity,
            event_kind,
            state_after,
            occurred_at: observed_at.to_owned(),
            summary: self.event_summary(event_kind, state_after),
            payload,
            provenance: TaskEventProvenance {
                source_kind,
                source_adapter_id: "adapter:pytest".to_owned(),
                adapter_version: PYTEST_DISCOVERER_VERSION.to_owned(),
                workspace_revision: self.workspace_revision.clone(),
                confidence,
                context_provenance: Some(self.context_provenance.clone()),
            },
            raw_envelope: self.raw_envelope(&event_id, source_kind, event_kind, observed_at),
        }
    }

    fn event_summary(&self, event_kind: TaskEventKind, state: TaskStateClass) -> String {
        format!(
            "Pytest selection `{}` {} ({})",
            safe_label(&self.selection.label),
            event_kind.as_str(),
            state.as_str()
        )
    }

    fn raw_envelope(
        &self,
        event_id: &str,
        source_kind: TaskEventSourceKind,
        event_kind: TaskEventKind,
        observed_at: &str,
    ) -> RawTaskEventEnvelope {
        let dispatch_program = self
            .dispatch
            .as_ref()
            .map(|dispatch| dispatch.program.clone());
        let dispatch_args = self
            .dispatch
            .as_ref()
            .map(|dispatch| dispatch.args.clone())
            .unwrap_or_default();
        let retained_payload = serde_json::json!({
            "event_kind": event_kind.as_str(),
            "selection_kind": self.selection.kind.as_str(),
            "selector": self.selection.selector,
            "test_item_id": self.selection.test_item_id,
            "selection_source_refs": self.selection.source_refs,
            "readiness": self.readiness.as_str(),
            "dispatch_program": dispatch_program,
            "dispatch_args": dispatch_args,
            "invocation_mode": self.dispatch.as_ref().map(|dispatch| dispatch.invocation_mode.as_str()),
            "blockers": self.blockers.iter().map(|reason| reason.as_str()).collect::<Vec<_>>(),
            "warnings": self.warnings.iter().map(|warning| warning.as_str()).collect::<Vec<_>>(),
        });
        RawTaskEventEnvelope {
            record_kind: RAW_TASK_EVENT_ENVELOPE_RECORD_KIND.to_owned(),
            raw_envelope_ref: format!("raw:{event_id}"),
            task_id: self.task_id.clone(),
            workspace_id: self.workspace_id.clone(),
            trace_id: self.trace_id.clone(),
            source_kind,
            adapter_origin_event_id: format!("pytest:{event_id}"),
            redaction_class: TaskEventRedactionClass::MetadataSafeDefault,
            retention_state: RawEnvelopeRetentionState::RetainedInlineRedacted,
            payload_digest: digest_token(&retained_payload.to_string()),
            retained_payload: Some(retained_payload),
            retained_at: observed_at.to_owned(),
            reconstruction_fields: vec![
                "selection_kind".to_owned(),
                "selector".to_owned(),
                "test_item_id".to_owned(),
                "readiness".to_owned(),
                "dispatch_program".to_owned(),
                "dispatch_args".to_owned(),
                "invocation_mode".to_owned(),
            ],
        }
    }

    fn task_block_reason(&self) -> TaskBlockReason {
        match self.blockers.first().copied() {
            Some(PytestBlockReason::PendingTrust) => TaskBlockReason::TrustReview,
            Some(PytestBlockReason::PolicyOrTrustBlocked) => TaskBlockReason::PolicyReview,
            Some(PytestBlockReason::TargetUnavailable) => TaskBlockReason::TargetUnavailable,
            _ => TaskBlockReason::DependencyMissing,
        }
    }
}

#[derive(Debug, Clone)]
struct PytestWorkspaceRead {
    discovery_state: PytestDiscoveryState,
    test_files: Vec<PytestTestFileDescriptor>,
    test_items: Vec<PytestTestDescriptor>,
    issues: Vec<PytestDiscoveryIssue>,
}

fn read_pytest_workspace(workspace_root: &Path) -> PytestWorkspaceRead {
    let mut paths = Vec::new();
    let mut issues = Vec::new();
    match collect_test_files(workspace_root, &mut paths) {
        Ok(()) => {}
        Err(err) => {
            return PytestWorkspaceRead {
                discovery_state: PytestDiscoveryState::WorkspaceUnreadable,
                test_files: Vec::new(),
                test_items: Vec::new(),
                issues: vec![PytestDiscoveryIssue {
                    issue_kind: PytestDiscoveryIssueKind::WorkspaceUnreadable,
                    source_ref: None,
                    summary: format!("workspace root could not be read: {err}"),
                }],
            };
        }
    }
    paths.sort_by_key(|path| relative_path(workspace_root, path));

    let mut test_files = Vec::new();
    let mut test_items = Vec::new();
    for path in paths {
        let rel = relative_path(workspace_root, &path);
        let payload = match fs::read_to_string(&path) {
            Ok(payload) => payload,
            Err(err) => {
                issues.push(PytestDiscoveryIssue {
                    issue_kind: PytestDiscoveryIssueKind::TestFileUnreadable,
                    source_ref: Some(rel.clone()),
                    summary: format!("{rel} could not be read: {err}"),
                });
                continue;
            }
        };
        let mut items = parse_pytest_file(&rel, &payload);
        let test_count = items.len() as u32;
        test_files.push(PytestTestFileDescriptor {
            file_id: format!("pytest-file:{}", stable_token(&rel)),
            relative_path: rel.clone(),
            source_ref: rel,
            test_count,
        });
        test_items.append(&mut items);
    }

    let discovery_state = if !issues.is_empty() {
        PytestDiscoveryState::Partial
    } else if test_items.is_empty() {
        PytestDiscoveryState::NoTestsDiscovered
    } else {
        PytestDiscoveryState::Ready
    };

    PytestWorkspaceRead {
        discovery_state,
        test_files,
        test_items,
        issues,
    }
}

fn collect_test_files(current: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if should_skip_dir(&path) {
                continue;
            }
            collect_test_files(&path, out)?;
        } else if file_type.is_file() && is_pytest_file(&path) {
            out.push(path);
        }
    }
    Ok(())
}

fn should_skip_dir(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    matches!(
        name,
        ".git"
            | ".hg"
            | ".svn"
            | ".venv"
            | "venv"
            | "env"
            | "__pycache__"
            | ".mypy_cache"
            | ".pytest_cache"
            | "node_modules"
            | "target"
            | "dist"
            | "build"
    )
}

fn is_pytest_file(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.ends_with(".py")
        && (file_name.starts_with("test_") || file_name.ends_with("_test.py"))
}

fn parse_pytest_file(rel_path: &str, payload: &str) -> Vec<PytestTestDescriptor> {
    let mut items = Vec::new();
    let mut current_class: Option<(String, usize)> = None;
    for (idx, raw_line) in payload.lines().enumerate() {
        let indent = leading_spaces(raw_line);
        let trimmed = raw_line.trim_start();
        if let Some((_, class_indent)) = &current_class {
            if indent <= *class_indent
                && !trimmed.is_empty()
                && !trimmed.starts_with('#')
                && !trimmed.starts_with('@')
                && !trimmed.starts_with("class ")
            {
                current_class = None;
            }
        }
        if let Some(class_name) = parse_class_name(trimmed) {
            current_class = Some((class_name, indent));
            continue;
        }
        let Some(function_name) = parse_test_function_name(trimmed) else {
            continue;
        };
        let line_number = (idx + 1) as u32;
        let (kind, node_id, label) = match &current_class {
            Some((class_name, class_indent)) if indent > *class_indent => (
                PytestTestKind::Method,
                format!("{rel_path}::{class_name}::{function_name}"),
                format!("{class_name}::{function_name}"),
            ),
            _ => (
                PytestTestKind::Function,
                format!("{rel_path}::{function_name}"),
                function_name.clone(),
            ),
        };
        items.push(PytestTestDescriptor {
            test_item_id: format!("pytest:{}", stable_token(&node_id)),
            node_id,
            display_label: label,
            kind,
            source_kind: PytestSourceKind::PytestFile,
            source_file_ref: rel_path.to_owned(),
            line_number,
            source_ref: format!("{rel_path}:{line_number}"),
            wedge: TaskWedgeClass::Test,
        });
    }
    items
}

fn build_run_contracts(
    workspace_id: &str,
    context: &ExecutionContext,
    runtime_status: &PytestRuntimeStatus,
    discovery_state: &PytestDiscoveryState,
    test_files: &[PytestTestFileDescriptor],
    test_items: &[PytestTestDescriptor],
    workspace_revision: &Option<String>,
) -> Vec<PytestRunContract> {
    if test_items.is_empty() {
        return Vec::new();
    }
    let runner = runner_from_status(runtime_status);
    let mut selections = Vec::new();
    selections.push(PytestRunSelection {
        kind: PytestSelectionKind::AllDiscovered,
        selector: None,
        test_item_id: None,
        label: "all discovered pytest tests".to_owned(),
        source_refs: test_files
            .iter()
            .map(|file| file.source_ref.clone())
            .collect(),
    });
    selections.extend(test_items.iter().map(|item| PytestRunSelection {
        kind: PytestSelectionKind::DiscoveredItem,
        selector: Some(item.node_id.clone()),
        test_item_id: Some(item.test_item_id.clone()),
        label: item.display_label.clone(),
        source_refs: vec![item.source_ref.clone()],
    }));

    selections
        .into_iter()
        .map(|selection| {
            let blockers = blockers_for(context, runtime_status, runner.as_ref());
            let warnings = warnings_for(context, runtime_status, discovery_state);
            let readiness = if blockers.is_empty() {
                if warnings.is_empty() {
                    PytestLaunchReadiness::Ready
                } else {
                    PytestLaunchReadiness::ReadyWithHonestyMarker
                }
            } else {
                PytestLaunchReadiness::Blocked
            };
            let task_token = match &selection.selector {
                Some(selector) => stable_token(selector),
                None => "all".to_owned(),
            };
            let task_id = format!("task:pytest:{task_token}");
            let run_id = format!("run:{task_id}");
            let dispatch = runner
                .as_ref()
                .filter(|_| !readiness.is_blocked())
                .map(|runner| dispatch_for(&selection, runner, context));
            PytestRunContract {
                record_kind: PYTEST_RUN_CONTRACT_RECORD_KIND.to_owned(),
                schema_version: PYTEST_DISCOVERY_SCHEMA_VERSION,
                run_contract_id: format!("contract:{task_id}"),
                workspace_id: workspace_id.to_owned(),
                task_id: task_id.clone(),
                run_id: run_id.clone(),
                attempt_id: format!("attempt:{task_id}:1"),
                attempt_number: 1,
                task_stream_id: format!("stream:{run_id}"),
                trace_id: format!("trace:{run_id}"),
                execution_context_ref: context.execution_context_id.clone(),
                target_id: context.target_identity.canonical_target_id.clone(),
                context_provenance: ExecutionEventProvenance::from_context(context),
                selection,
                wedge: TaskWedgeClass::Test,
                runner: runner.clone(),
                dispatch,
                readiness,
                blockers,
                warnings,
                workspace_revision: workspace_revision.clone(),
                rerun_lineage: None,
            }
        })
        .collect()
}

fn runner_from_status(status: &PytestRuntimeStatus) -> Option<PytestRunner> {
    let kind = status.environment_manager_kind?;
    if !kind.is_launch_wedge_supported() {
        return None;
    }
    match kind {
        PythonEnvironmentManagerKind::Uv => Some(PytestRunner {
            environment_manager_kind: Some(kind),
            program: "uv".to_owned(),
            version: version_from_token(status.environment_manager_value_token.as_deref()),
            interpreter_ref: status.interpreter_ref.clone(),
            invocation_mode: PytestInvocationMode::UvRun,
        }),
        PythonEnvironmentManagerKind::Poetry => Some(PytestRunner {
            environment_manager_kind: Some(kind),
            program: "poetry".to_owned(),
            version: version_from_token(status.environment_manager_value_token.as_deref()),
            interpreter_ref: status.interpreter_ref.clone(),
            invocation_mode: PytestInvocationMode::PoetryRun,
        }),
        PythonEnvironmentManagerKind::Venv => Some(PytestRunner {
            environment_manager_kind: Some(kind),
            program: status
                .interpreter_ref
                .clone()
                .unwrap_or_else(|| "python".to_owned()),
            version: version_from_token(status.python_value_token.as_deref()),
            interpreter_ref: status.interpreter_ref.clone(),
            invocation_mode: PytestInvocationMode::PythonModule,
        }),
        PythonEnvironmentManagerKind::Conda | PythonEnvironmentManagerKind::Unknown => None,
    }
}

fn dispatch_for(
    selection: &PytestRunSelection,
    runner: &PytestRunner,
    context: &ExecutionContext,
) -> PytestDispatch {
    let mut args = match runner.invocation_mode {
        PytestInvocationMode::UvRun => vec!["run".to_owned(), "pytest".to_owned()],
        PytestInvocationMode::PoetryRun => vec!["run".to_owned(), "pytest".to_owned()],
        PytestInvocationMode::PythonModule => vec!["-m".to_owned(), "pytest".to_owned()],
    };
    if let Some(selector) = &selection.selector {
        args.push(selector.clone());
    }
    PytestDispatch {
        program: runner.program.clone(),
        args,
        working_directory: context.target_identity.working_directory.clone(),
        invocation_mode: runner.invocation_mode,
        selector: selection.selector.clone(),
        source_refs: selection.source_refs.clone(),
    }
}

fn blockers_for(
    context: &ExecutionContext,
    status: &PytestRuntimeStatus,
    runner: Option<&PytestRunner>,
) -> Vec<PytestBlockReason> {
    let mut blockers = Vec::new();
    if context.policy_and_trust.trust_state == TrustState::PendingEvaluation {
        blockers.push(PytestBlockReason::PendingTrust);
    }
    match status.interpreter_resolution_state {
        PythonEnvironmentResolutionState::Missing => {
            blockers.push(PytestBlockReason::MissingPythonInterpreter)
        }
        PythonEnvironmentResolutionState::Ambiguous => {
            blockers.push(PytestBlockReason::AmbiguousPythonInterpreter)
        }
        PythonEnvironmentResolutionState::Unsupported => {
            blockers.push(PytestBlockReason::UnsupportedPythonInterpreter)
        }
        PythonEnvironmentResolutionState::Resolved | PythonEnvironmentResolutionState::Fallback => {
        }
    }
    match status.environment_manager_resolution_state {
        PythonEnvironmentResolutionState::Missing => {
            blockers.push(PytestBlockReason::MissingEnvironmentManager)
        }
        PythonEnvironmentResolutionState::Ambiguous => {
            blockers.push(PytestBlockReason::AmbiguousEnvironmentManager)
        }
        PythonEnvironmentResolutionState::Unsupported => {
            blockers.push(PytestBlockReason::UnsupportedEnvironmentManager)
        }
        PythonEnvironmentResolutionState::Resolved | PythonEnvironmentResolutionState::Fallback => {
        }
    }
    if status.environment_manager_kind.is_some() && runner.is_none() {
        blockers.push(PytestBlockReason::UnsupportedEnvironmentManager);
    }
    if context.target_identity.reachability_state != ReachabilityState::Reachable {
        blockers.push(PytestBlockReason::TargetUnavailable);
    }
    for field in &context.degraded_fields {
        if field
            .field_path
            .starts_with("python_environment_detection.")
        {
            continue;
        }
        match field.reason {
            DegradedFieldReason::ToolchainFallback => {}
            DegradedFieldReason::ActivatorBlockedByTrust
            | DegradedFieldReason::ActivatorBlockedByPolicy => {
                blockers.push(PytestBlockReason::PolicyOrTrustBlocked)
            }
            DegradedFieldReason::TrustStateUnresolved => {
                blockers.push(PytestBlockReason::PendingTrust)
            }
            _ => blockers.push(PytestBlockReason::DegradedExecutionContext),
        }
    }
    dedupe_blockers(blockers)
}

fn warnings_for(
    context: &ExecutionContext,
    status: &PytestRuntimeStatus,
    discovery_state: &PytestDiscoveryState,
) -> Vec<PytestWarningClass> {
    let mut warnings = Vec::new();
    if context
        .python_environment_detection
        .as_ref()
        .and_then(|detection| detection.interpreter.winning_source)
        == Some(PythonEnvironmentSourceKind::AmbientPath)
    {
        warnings.push(PytestWarningClass::AmbientPythonInterpreter);
    }
    if status.environment_manager_resolution_state == PythonEnvironmentResolutionState::Fallback {
        warnings.push(PytestWarningClass::EnvironmentManagerFallback);
    }
    if *discovery_state == PytestDiscoveryState::Partial {
        warnings.push(PytestWarningClass::PartialDiscovery);
    }
    if context.degraded_fields.iter().any(|field| {
        field.reason == DegradedFieldReason::ToolchainFallback
            && field
                .field_path
                .starts_with("python_environment_detection.")
    }) {
        warnings.push(PytestWarningClass::ToolchainFallback);
    }
    dedupe_warnings(warnings)
}

fn dedupe_blockers(blockers: Vec<PytestBlockReason>) -> Vec<PytestBlockReason> {
    let mut out = Vec::new();
    for blocker in blockers {
        if !out.contains(&blocker) {
            out.push(blocker);
        }
    }
    out
}

fn dedupe_warnings(warnings: Vec<PytestWarningClass>) -> Vec<PytestWarningClass> {
    let mut out = Vec::new();
    for warning in warnings {
        if !out.contains(&warning) {
            out.push(warning);
        }
    }
    out
}

fn push_runtime_state(
    states: &mut Vec<PytestMissingRuntimeState>,
    state: PythonEnvironmentResolutionState,
    missing: PytestMissingRuntimeState,
    ambiguous: PytestMissingRuntimeState,
    unsupported: PytestMissingRuntimeState,
) {
    match state {
        PythonEnvironmentResolutionState::Missing => states.push(missing),
        PythonEnvironmentResolutionState::Ambiguous => states.push(ambiguous),
        PythonEnvironmentResolutionState::Unsupported => states.push(unsupported),
        PythonEnvironmentResolutionState::Resolved | PythonEnvironmentResolutionState::Fallback => {
        }
    }
}

fn python_value_token(detection: &PythonEnvironmentDetection) -> Option<String> {
    detection
        .interpreter
        .resolved_requirement
        .as_deref()
        .map(
            |version| match detection.interpreter.interpreter_ref.as_deref() {
                Some(interpreter_ref) if !interpreter_ref.is_empty() => {
                    format!("python@{version} ({interpreter_ref})")
                }
                _ => format!("python@{version}"),
            },
        )
        .or_else(|| detection.interpreter.interpreter_ref.clone())
}

fn environment_manager_value_token(detection: &PythonEnvironmentDetection) -> Option<String> {
    let kind = detection.environment_manager.kind?;
    let base = match &detection.environment_manager.version {
        Some(version) if !version.is_empty() => format!("{}@{version}", kind.as_str()),
        _ => kind.as_str().to_owned(),
    };
    Some(match &detection.environment_manager.environment_ref {
        Some(environment_ref) if !environment_ref.is_empty() => {
            format!("{base} ({environment_ref})")
        }
        _ => base,
    })
}

fn version_from_token(token: Option<&str>) -> Option<String> {
    let token = token?;
    let (_, version) = token.split_once('@')?;
    Some(
        version
            .split_once(' ')
            .map(|(version, _)| version)
            .unwrap_or(version)
            .to_owned(),
    )
}

fn pytest_config_refs(workspace_root: &Path) -> Vec<String> {
    let mut refs = Vec::new();
    let pyproject = workspace_root.join("pyproject.toml");
    if fs::read_to_string(&pyproject)
        .map(|payload| payload.contains("[tool.pytest.ini_options]"))
        .unwrap_or(false)
    {
        refs.push("pyproject.toml#tool.pytest.ini_options".to_owned());
    }
    for rel in ["pytest.ini", "tox.ini", "setup.cfg"] {
        if workspace_root.join(rel).is_file() {
            refs.push(rel.to_owned());
        }
    }
    refs
}

fn relative_path(workspace_root: &Path, path: &Path) -> String {
    path.strip_prefix(workspace_root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

fn leading_spaces(line: &str) -> usize {
    line.chars().take_while(|ch| *ch == ' ').count()
}

fn parse_class_name(trimmed: &str) -> Option<String> {
    let rest = trimmed.strip_prefix("class ")?;
    let ident = take_identifier(rest);
    if ident.starts_with("Test") {
        Some(ident)
    } else {
        None
    }
}

fn parse_test_function_name(trimmed: &str) -> Option<String> {
    let rest = trimmed
        .strip_prefix("def ")
        .or_else(|| trimmed.strip_prefix("async def "))?;
    let ident = take_identifier(rest);
    if ident.starts_with("test_") {
        Some(ident)
    } else {
        None
    }
}

fn take_identifier(raw: &str) -> String {
    raw.chars()
        .take_while(|ch| ch.is_ascii_alphanumeric() || *ch == '_')
        .collect()
}

fn sequence_for_attempt(attempt_number: u32, offset: u64) -> u64 {
    u64::from(attempt_number.saturating_sub(1)) * 100 + offset
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

fn safe_label(raw: &str) -> String {
    raw.chars()
        .filter(|ch| !ch.is_control())
        .collect::<String>()
}

fn digest_token(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sha256:{hash:064x}")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        TargetClass, ToolchainClass,
    };

    fn fixture_root(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/python_task_discovery_alpha")
            .join(name)
    }

    fn baseline_resolver() -> ExecutionContextResolver {
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
            resolver_version: "test-resolver".to_owned(),
        })
    }

    fn pytest_context(
        resolver: &mut ExecutionContextResolver,
        observed_at: &str,
    ) -> ExecutionContext {
        resolver.resolve(ExecutionContextRequest::test_seed(
            "test.run.pytest",
            TrustState::Trusted,
            observed_at,
        ))
    }

    fn discoverer_with_ambient() -> PytestDiscoverer {
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

    #[test]
    fn ready_uv_workspace_discovers_pytest_items_and_direct_contracts() {
        let mut resolver = baseline_resolver();
        let context = pytest_context(&mut resolver, "2026-05-13T17:00:00Z");
        assert_eq!(
            context.toolchain_identity.toolchain_class,
            ToolchainClass::TestRunnerRuntime
        );

        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_uv"),
            context,
            "2026-05-13T17:00:01Z",
        );

        assert_eq!(discovery.record_kind, PYTEST_DISCOVERY_RECORD_KIND);
        assert_eq!(discovery.discovery_state, PytestDiscoveryState::Ready);
        assert_eq!(
            discovery.runtime_status.environment_manager_kind,
            Some(PythonEnvironmentManagerKind::Uv)
        );
        assert!(!discovery.runtime_status.has_missing_or_blocked_runtime());
        assert!(discovery
            .pytest_config_refs
            .contains(&"pyproject.toml#tool.pytest.ini_options".to_owned()));
        assert!(discovery
            .test_items
            .iter()
            .any(|item| item.node_id == "tests/test_api.py::test_health"));
        assert!(discovery
            .test_items
            .iter()
            .any(|item| item.node_id == "tests/test_api.py::TestBilling::test_invoice_total"));

        let all = discovery.all_tests_contract().expect("all tests contract");
        assert_eq!(all.readiness, PytestLaunchReadiness::Ready);
        let dispatch = all.dispatch.as_ref().expect("ready dispatch");
        assert_eq!(dispatch.program, "uv");
        assert_eq!(dispatch.args, vec!["run".to_owned(), "pytest".to_owned()]);
        assert_eq!(dispatch.invocation_mode, PytestInvocationMode::UvRun);
        assert!(dispatch.selector.is_none());

        let item = discovery
            .contract_for_selector("tests/test_api.py::test_health")
            .expect("item contract");
        assert_eq!(item.wedge, TaskWedgeClass::Test);
        assert_eq!(
            item.dispatch.as_ref().map(|dispatch| dispatch.args.clone()),
            Some(vec![
                "run".to_owned(),
                "pytest".to_owned(),
                "tests/test_api.py::test_health".to_owned()
            ])
        );
    }

    #[test]
    fn launch_contract_projects_into_canonical_task_stream_and_support_export() {
        let mut resolver = baseline_resolver();
        let context = pytest_context(&mut resolver, "2026-05-13T17:01:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_uv"),
            context,
            "2026-05-13T17:01:01Z",
        );
        let contract = discovery
            .contract_for_selector("tests/test_api.py::TestBilling::test_invoice_total")
            .expect("test contract");

        let stream = contract
            .launch_event_stream("2026-05-13T17:01:02Z")
            .expect("task stream");
        assert_eq!(stream.events.len(), 2);
        assert_eq!(stream.events[0].event_kind, TaskEventKind::TaskQueued);
        assert_eq!(stream.events[1].event_kind, TaskEventKind::TaskStarted);
        assert_eq!(stream.events[1].state_after, TaskStateClass::Running);
        assert_eq!(stream.events[1].identity.wedge, TaskWedgeClass::Test);
        assert_eq!(
            stream.events[1]
                .provenance
                .context_provenance
                .as_ref()
                .map(|provenance| provenance.context_provenance_id.as_str()),
            Some(contract.context_provenance.context_provenance_id.as_str())
        );
        assert!(stream.events[1]
            .raw_envelope
            .matches_event(&stream.events[1]));

        let shell = stream.shell_projection();
        assert_eq!(shell.len(), 2);
        assert_eq!(
            shell[1].execution_context_ref,
            contract.execution_context_ref
        );
        assert_eq!(shell[1].state_after, TaskStateClass::Running);

        let support = stream.support_export(
            "support-export:pytest:test-invoice-total",
            "2026-05-13T17:01:03Z",
        );
        assert_eq!(support.events.len(), 2);
        assert_eq!(support.raw_envelopes.len(), 2);
        assert_eq!(support.context_provenance.len(), 1);
        assert_eq!(support.context_provenance_events.len(), 1);
        assert_eq!(
            support.events[1].context_provenance_ref.as_deref(),
            Some(contract.context_provenance.context_provenance_id.as_str())
        );
        assert_eq!(support.events[1].task_id, contract.task_id);
    }

    #[test]
    fn missing_runtime_is_disclosed_and_blocks_launch_with_task_event() {
        let mut resolver = baseline_resolver();
        let context = pytest_context(&mut resolver, "2026-05-13T17:02:00Z");
        let discovery = PytestDiscoverer::default_read_only().discover_workspace(
            &fixture_root("missing_python_runtime"),
            context,
            "2026-05-13T17:02:01Z",
        );

        assert_eq!(discovery.discovery_state, PytestDiscoveryState::Ready);
        assert!(discovery
            .runtime_status
            .missing_runtime_states
            .contains(&PytestMissingRuntimeState::PythonInterpreterMissing));
        assert!(discovery
            .runtime_status
            .missing_runtime_states
            .contains(&PytestMissingRuntimeState::EnvironmentManagerMissing));
        assert!(discovery.honesty_marker_present);

        let contract = discovery
            .contract_for_selector("tests/test_smoke.py::test_smoke")
            .expect("smoke contract");
        assert_eq!(contract.readiness, PytestLaunchReadiness::Blocked);
        assert!(contract
            .blockers
            .contains(&PytestBlockReason::MissingPythonInterpreter));
        assert!(contract
            .blockers
            .contains(&PytestBlockReason::MissingEnvironmentManager));
        assert!(contract.dispatch.is_none());

        let stream = contract
            .launch_event_stream("2026-05-13T17:02:02Z")
            .expect("blocked stream");
        assert_eq!(stream.events[1].event_kind, TaskEventKind::TaskBlocked);
        assert_eq!(stream.events[1].state_after, TaskStateClass::Blocked);
        assert_eq!(
            stream
                .state_for_task(&contract.task_id)
                .expect("task state")
                .current_state,
            TaskStateClass::Blocked
        );
        assert!(stream.shell_projection()[1].needs_attention);
    }

    #[test]
    fn unsupported_environment_manager_keeps_test_source_but_blocks_dispatch() {
        let mut resolver = baseline_resolver();
        let context = pytest_context(&mut resolver, "2026-05-13T17:03:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("unsupported_conda"),
            context,
            "2026-05-13T17:03:01Z",
        );

        let contract = discovery
            .contract_for_selector("tests/test_conda.py::test_conda_only")
            .expect("conda test contract");
        assert_eq!(
            discovery.runtime_status.environment_manager_kind,
            Some(PythonEnvironmentManagerKind::Conda)
        );
        assert!(contract
            .blockers
            .contains(&PytestBlockReason::UnsupportedEnvironmentManager));
        assert_eq!(
            contract.selection.source_refs,
            vec!["tests/test_conda.py:1".to_owned()]
        );
        assert!(contract.dispatch.is_none());
    }

    #[test]
    fn rerun_contract_preserves_selector_and_records_context_drift() {
        let mut resolver = baseline_resolver();
        let original_context = pytest_context(&mut resolver, "2026-05-13T17:04:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_uv"),
            original_context,
            "2026-05-13T17:04:01Z",
        );
        let contract = discovery
            .contract_for_selector("tests/test_api.py::test_health")
            .expect("test contract");

        let mut current_request = ExecutionContextRequest::test_seed(
            "test.run.pytest",
            TrustState::Trusted,
            "2026-05-13T17:04:02Z",
        );
        current_request.override_target_class = Some(TargetClass::SshRemote);
        current_request.override_working_directory = Some("/srv/python");
        let current_context = resolver.resolve(current_request);

        let rerun = contract.rerun_with_context(
            &current_context,
            PytestRerunMode::CurrentContext,
            "2026-05-13T17:04:03Z",
        );
        assert_eq!(rerun.run_id, contract.run_id);
        assert_ne!(rerun.attempt_id, contract.attempt_id);
        assert_eq!(rerun.attempt_number, 2);
        assert_eq!(rerun.selection.selector, contract.selection.selector);
        assert_eq!(
            rerun
                .dispatch
                .as_ref()
                .and_then(|dispatch| dispatch.working_directory.as_deref()),
            Some("/srv/python")
        );
        let lineage = rerun.rerun_lineage.as_ref().expect("rerun lineage");
        assert_eq!(lineage.mode, PytestRerunMode::CurrentContext);
        assert!(lineage.context_changed);
        assert_eq!(
            lineage.previous_execution_context_ref,
            contract.execution_context_ref
        );
        assert_eq!(
            lineage.current_execution_context_ref,
            current_context.execution_context_id
        );
        assert!(rerun.context_provenance.matches_context(&current_context));

        let initial_stream = contract
            .launch_event_stream("2026-05-13T17:04:04Z")
            .expect("initial stream");
        let rerun_stream = rerun
            .launch_event_stream("2026-05-13T17:04:05Z")
            .expect("rerun stream");
        assert!(rerun_stream.events[0].stream_sequence > initial_stream.events[1].stream_sequence);
        assert_eq!(
            rerun_stream.events[0].identity.run_id,
            initial_stream.events[0].identity.run_id
        );
        assert_ne!(
            rerun_stream.events[0].identity.attempt_id,
            initial_stream.events[0].identity.attempt_id
        );
    }
}
