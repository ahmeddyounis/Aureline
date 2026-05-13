//! Package-script discovery and run contracts for TS/JS workspaces.
//!
//! This module reads `package.json#scripts`, combines the findings with the
//! canonical execution-context resolver and the Node detector, and emits
//! package-manager run contracts that project into the shared task-event
//! stream. The contract never wraps scripts in `sh -c`, `cmd /C`, or a
//! terminal-only command line; it records a package-manager program plus argv
//! and lets the package manager own the script lifecycle it already defines.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::detectors::node::{
    NodePackageManagerKind, NodeToolchainDetection, NodeToolchainDetector,
    NodeToolchainDetectorConfig, NodeToolchainResolutionState, NodeToolchainSourceKind,
};
use crate::execution_context::{DegradedFieldReason, ExecutionContext, ReachabilityState};
use crate::provenance::ExecutionEventProvenance;
use crate::tasks::{
    RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskBlockReason, TaskEvent,
    TaskEventConfidence, TaskEventIdentity, TaskEventKind, TaskEventPayload, TaskEventProvenance,
    TaskEventRedactionClass, TaskEventSourceKind, TaskEventStream, TaskEventStreamError,
    TaskShellProjection, TaskStateClass, TaskSupportExport, TaskWedgeClass,
    RAW_TASK_EVENT_ENVELOPE_RECORD_KIND, TASK_EVENT_RECORD_KIND, TASK_EVENT_SCHEMA_VERSION,
};
use crate::TrustState;

/// Schema version for [`PackageScriptDiscovery`] and
/// [`PackageScriptRunContract`] records.
pub const PACKAGE_SCRIPT_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a package-script discovery result.
pub const PACKAGE_SCRIPT_DISCOVERY_RECORD_KIND: &str = "package_script_discovery_record";

/// Stable record-kind tag for one package-script run contract.
pub const PACKAGE_SCRIPT_RUN_CONTRACT_RECORD_KIND: &str = "package_script_run_contract_record";

/// Stable implementation version recorded in discovery reports and task
/// event provenance.
pub const PACKAGE_SCRIPT_DISCOVERER_VERSION: &str = "package_scripts.discovery.alpha.v1";

/// Configuration for [`PackageScriptDiscoverer`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PackageScriptDiscovererConfig {
    /// Node detector configuration used before script launch contracts are
    /// materialized.
    pub node_detector: NodeToolchainDetectorConfig,
    /// Workspace revision captured by the caller, when known.
    pub workspace_revision: Option<String>,
}

/// Read-only package-script discoverer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageScriptDiscoverer {
    config: PackageScriptDiscovererConfig,
}

impl PackageScriptDiscoverer {
    /// Creates a discoverer with caller-provided detector facts.
    pub fn new(config: PackageScriptDiscovererConfig) -> Self {
        Self { config }
    }

    /// Creates a discoverer with no caller-provided runtime facts.
    pub fn default_read_only() -> Self {
        Self::new(PackageScriptDiscovererConfig::default())
    }

    /// Discovers package scripts for a workspace root.
    ///
    /// The caller supplies a freshly resolved [`ExecutionContext`]. The
    /// discoverer attaches a read-only Node detector report to that context,
    /// parses `package.json` with `serde_json`, and returns descriptors plus
    /// run contracts for the bounded TS/JS launch-wedge script set.
    pub fn discover_workspace(
        &self,
        workspace_root: &Path,
        context: ExecutionContext,
        discovered_at: &str,
    ) -> PackageScriptDiscovery {
        let node_detection = NodeToolchainDetector::new(self.config.node_detector.clone())
            .detect_workspace(workspace_root, discovered_at);
        let mut context = context.with_node_toolchain_detection(node_detection.clone());
        let runtime_status = PackageScriptRuntimeStatus::from_detection(&node_detection);
        let read = read_package_scripts(workspace_root);

        let workspace_id = context.invocation_subject.workspace_id.clone();
        let workspace_root_ref = workspace_root.display().to_string();
        let package_manifest_ref =
            if matches!(read.manifest_state, PackageScriptManifestState::Missing) {
                None
            } else {
                Some("package.json".to_owned())
            };

        let scripts = read
            .scripts
            .iter()
            .map(|(name, body)| PackageScriptDescriptor::from_script(name, body))
            .collect::<Vec<_>>();

        let contract_scripts = scripts
            .iter()
            .filter(|script| script.runnable_in_launch_wedge)
            .cloned()
            .collect::<Vec<_>>();
        let run_contracts = build_run_contracts(
            &workspace_id,
            &context,
            &runtime_status,
            &scripts,
            &contract_scripts,
            &self.config.workspace_revision,
        );
        let discovery_state = discovery_state_for(
            read.manifest_state,
            scripts.is_empty(),
            run_contracts.is_empty(),
        );
        if matches!(
            discovery_state,
            PackageScriptDiscoveryState::ManifestUnreadable
                | PackageScriptDiscoveryState::ManifestInvalid
                | PackageScriptDiscoveryState::ScriptsFieldInvalid
        ) {
            context.degraded_fields.push(crate::DegradedFieldRecord {
                field_path: "package_scripts.package_json".to_owned(),
                reason: DegradedFieldReason::ProvenanceGap,
                repair_hook_ref: Some("doctor.repair.package_scripts".to_owned()),
            });
        }

        let honesty_marker_present = runtime_status.has_missing_or_blocked_runtime()
            || run_contracts
                .iter()
                .any(|contract| contract.readiness.requires_honesty_marker())
            || !context.degraded_fields.is_empty()
            || matches!(
                discovery_state,
                PackageScriptDiscoveryState::ManifestUnreadable
                    | PackageScriptDiscoveryState::ManifestInvalid
                    | PackageScriptDiscoveryState::ScriptsFieldInvalid
                    | PackageScriptDiscoveryState::NoRunnableScripts
            );

        PackageScriptDiscovery {
            record_kind: PACKAGE_SCRIPT_DISCOVERY_RECORD_KIND.to_owned(),
            schema_version: PACKAGE_SCRIPT_DISCOVERY_SCHEMA_VERSION,
            discoverer_version: PACKAGE_SCRIPT_DISCOVERER_VERSION.to_owned(),
            workspace_id,
            workspace_root_ref,
            package_manifest_ref,
            discovered_at: discovered_at.to_owned(),
            discovery_state,
            manifest_error: read.error,
            runtime_status,
            execution_context: context,
            scripts,
            run_contracts,
            honesty_marker_present,
        }
    }
}

/// Top-level package-script discovery record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptDiscovery {
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
    /// Manifest reference when `package.json` was present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_manifest_ref: Option<String>,
    /// Timestamp supplied by the caller.
    pub discovered_at: String,
    /// Overall discovery state.
    pub discovery_state: PackageScriptDiscoveryState,
    /// Manifest read or parse error, when the state carries one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_error: Option<String>,
    /// Runtime status derived from the Node detector.
    pub runtime_status: PackageScriptRuntimeStatus,
    /// Canonical execution context with the Node detector report attached.
    pub execution_context: ExecutionContext,
    /// All scripts found in `package.json#scripts`.
    pub scripts: Vec<PackageScriptDescriptor>,
    /// Runnable contracts for the bounded TS/JS launch-wedge subset.
    pub run_contracts: Vec<PackageScriptRunContract>,
    /// True when discovery or at least one contract must render an honesty
    /// marker before launch.
    pub honesty_marker_present: bool,
}

impl PackageScriptDiscovery {
    /// Returns the run contract for a script name, if it is in the bounded
    /// launch-wedge set.
    pub fn contract_for_script(&self, script_name: &str) -> Option<&PackageScriptRunContract> {
        self.run_contracts
            .iter()
            .find(|contract| contract.script.name == script_name)
    }

    /// Returns shell projections for all launchable contracts.
    ///
    /// This is the first runtime consumer: package-script contracts are
    /// projected through the canonical task-event stream rather than a
    /// package-script-only event grammar.
    pub fn shell_projections(&self, observed_at: &str) -> Vec<TaskShellProjection> {
        self.run_contracts
            .iter()
            .flat_map(|contract| match contract.launch_event_stream(observed_at) {
                Ok(stream) => stream.shell_projection(),
                Err(_) => Vec::new(),
            })
            .collect()
    }

    /// Returns support exports for all launchable contracts.
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
}

/// Overall package-script discovery state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptDiscoveryState {
    /// At least one launch-wedge run contract was produced.
    Ready,
    /// No `package.json` manifest was present at the workspace root.
    NoPackageManifest,
    /// The manifest could not be read.
    ManifestUnreadable,
    /// The manifest was not valid JSON.
    ManifestInvalid,
    /// The manifest did not define a usable `scripts` object.
    ScriptsFieldInvalid,
    /// The manifest was present but did not define any scripts.
    NoScripts,
    /// Scripts were present, but none are in the bounded launch-wedge set.
    NoRunnableScripts,
}

impl PackageScriptDiscoveryState {
    /// Stable string token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::NoPackageManifest => "no_package_manifest",
            Self::ManifestUnreadable => "manifest_unreadable",
            Self::ManifestInvalid => "manifest_invalid",
            Self::ScriptsFieldInvalid => "scripts_field_invalid",
            Self::NoScripts => "no_scripts",
            Self::NoRunnableScripts => "no_runnable_scripts",
        }
    }
}

/// Runtime status derived from the embedded Node detector report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptRuntimeStatus {
    /// Node runtime resolution state.
    pub node_resolution_state: NodeToolchainResolutionState,
    /// Package-manager resolution state.
    pub package_manager_resolution_state: NodeToolchainResolutionState,
    /// Resolved Node token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_value_token: Option<String>,
    /// Resolved package-manager token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_manager_value_token: Option<String>,
    /// Resolved package-manager kind when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_manager_kind: Option<NodePackageManagerKind>,
    /// Missing, ambiguous, or unsupported runtime states that must be visible
    /// on discovery rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing_runtime_states: Vec<PackageScriptMissingRuntimeState>,
}

impl PackageScriptRuntimeStatus {
    /// Builds a runtime status from a Node detector report.
    pub fn from_detection(detection: &NodeToolchainDetection) -> Self {
        let node_value_token = detection
            .node_runtime
            .resolved_requirement
            .as_ref()
            .map(|value| format!("node@{value}"));
        let package_manager_value_token = package_manager_value_token(detection);
        let mut missing_runtime_states = Vec::new();
        push_runtime_state(
            &mut missing_runtime_states,
            detection.node_runtime.resolution_state,
            PackageScriptMissingRuntimeState::NodeRuntimeMissing,
            PackageScriptMissingRuntimeState::NodeRuntimeAmbiguous,
            PackageScriptMissingRuntimeState::NodeRuntimeUnsupported,
        );
        push_runtime_state(
            &mut missing_runtime_states,
            detection.package_manager.resolution_state,
            PackageScriptMissingRuntimeState::PackageManagerMissing,
            PackageScriptMissingRuntimeState::PackageManagerAmbiguous,
            PackageScriptMissingRuntimeState::PackageManagerUnsupported,
        );

        Self {
            node_resolution_state: detection.node_runtime.resolution_state,
            package_manager_resolution_state: detection.package_manager.resolution_state,
            node_value_token,
            package_manager_value_token,
            package_manager_kind: detection.package_manager.kind,
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
pub enum PackageScriptMissingRuntimeState {
    /// Node runtime could not be resolved.
    NodeRuntimeMissing,
    /// Node runtime pins are ambiguous.
    NodeRuntimeAmbiguous,
    /// Node runtime resolved outside the supported contract.
    NodeRuntimeUnsupported,
    /// Package-manager runner could not be resolved.
    PackageManagerMissing,
    /// Package-manager pins are ambiguous.
    PackageManagerAmbiguous,
    /// Package-manager runner is outside the TS/JS launch-wedge contract.
    PackageManagerUnsupported,
}

impl PackageScriptMissingRuntimeState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodeRuntimeMissing => "node_runtime_missing",
            Self::NodeRuntimeAmbiguous => "node_runtime_ambiguous",
            Self::NodeRuntimeUnsupported => "node_runtime_unsupported",
            Self::PackageManagerMissing => "package_manager_missing",
            Self::PackageManagerAmbiguous => "package_manager_ambiguous",
            Self::PackageManagerUnsupported => "package_manager_unsupported",
        }
    }
}

/// Source kind for a discovered package script.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptSourceKind {
    /// `package.json#scripts`.
    PackageJsonScripts,
}

impl PackageScriptSourceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageJsonScripts => "package_json_scripts",
        }
    }
}

/// Workspace source reference for one package script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptSource {
    /// Source kind.
    pub source_kind: PackageScriptSourceKind,
    /// Manifest path relative to the workspace root.
    pub manifest_ref: String,
    /// JSON pointer within the manifest.
    pub json_pointer: String,
    /// Reviewable source reference combining manifest and JSON pointer.
    pub source_ref: String,
}

/// One script discovered in `package.json#scripts`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptDescriptor {
    /// Stable script id inside the workspace.
    pub script_id: String,
    /// Script name from `package.json#scripts`.
    pub name: String,
    /// Script body from the manifest.
    pub script_body: String,
    /// Source information for this script.
    pub source: PackageScriptSource,
    /// True when this script is in the bounded TS/JS launch-wedge set.
    pub runnable_in_launch_wedge: bool,
    /// Task wedge the script maps to when runnable.
    pub wedge: TaskWedgeClass,
}

impl PackageScriptDescriptor {
    fn from_script(name: &str, body: &str) -> Self {
        let json_pointer = format!("/scripts/{}", json_pointer_segment(name));
        Self {
            script_id: format!("package-script:{}", stable_token(name)),
            name: name.to_owned(),
            script_body: body.to_owned(),
            source: PackageScriptSource {
                source_kind: PackageScriptSourceKind::PackageJsonScripts,
                manifest_ref: "package.json".to_owned(),
                source_ref: format!("package.json#{json_pointer}"),
                json_pointer,
            },
            runnable_in_launch_wedge: is_launch_wedge_script(name),
            wedge: classify_script(name),
        }
    }
}

/// Package-manager runner for a script contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptRunner {
    /// Runner family.
    pub kind: NodePackageManagerKind,
    /// Program name to execute directly.
    pub program: String,
    /// Version token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// How Aureline dispatches a package script.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptShellMode {
    /// Directly spawn the package-manager executable with argv.
    DirectProcess,
}

impl PackageScriptShellMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectProcess => "direct_process",
        }
    }
}

/// Related lifecycle hook that the package manager may run around a script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptLifecycleHook {
    /// Hook script name, such as `prebuild`.
    pub name: String,
    /// Source reference for the hook.
    pub source_ref: String,
}

/// Process dispatch record for one package-script attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptDispatch {
    /// Program to spawn directly.
    pub program: String,
    /// Argument vector passed to the program.
    pub args: Vec<String>,
    /// Working directory copied from the execution context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Shell mode used by Aureline.
    pub shell_mode: PackageScriptShellMode,
    /// True because the package manager, not Aureline shell glue, owns
    /// lifecycle-hook semantics.
    pub package_manager_owns_script_lifecycle: bool,
    /// Adjacent lifecycle hooks disclosed before launch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_lifecycle_hooks: Vec<PackageScriptLifecycleHook>,
}

/// Launch readiness for one package-script run contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptLaunchReadiness {
    /// The contract can launch without extra honesty markers.
    Ready,
    /// The contract can launch, but fallback or ambient runtime truth must
    /// remain visible.
    ReadyWithHonestyMarker,
    /// The contract cannot safely launch.
    Blocked,
}

impl PackageScriptLaunchReadiness {
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

/// Blocker on a package-script run contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptBlockReason {
    /// Workspace trust has not been accepted.
    PendingTrust,
    /// Node runtime is missing.
    MissingNodeRuntime,
    /// Node runtime is ambiguous.
    AmbiguousNodeRuntime,
    /// Node runtime is unsupported by the TS/JS launch wedge.
    UnsupportedNodeRuntime,
    /// Package manager is missing.
    MissingPackageManager,
    /// Package manager is ambiguous.
    AmbiguousPackageManager,
    /// Package manager is unsupported by the TS/JS launch wedge.
    UnsupportedPackageManager,
    /// Context target is not reachable.
    TargetUnavailable,
    /// Policy or trust blocked an activator.
    PolicyOrTrustBlocked,
    /// Another degraded execution-context field blocks dispatch.
    DegradedExecutionContext,
}

impl PackageScriptBlockReason {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingTrust => "pending_trust",
            Self::MissingNodeRuntime => "missing_node_runtime",
            Self::AmbiguousNodeRuntime => "ambiguous_node_runtime",
            Self::UnsupportedNodeRuntime => "unsupported_node_runtime",
            Self::MissingPackageManager => "missing_package_manager",
            Self::AmbiguousPackageManager => "ambiguous_package_manager",
            Self::UnsupportedPackageManager => "unsupported_package_manager",
            Self::TargetUnavailable => "target_unavailable",
            Self::PolicyOrTrustBlocked => "policy_or_trust_blocked",
            Self::DegradedExecutionContext => "degraded_execution_context",
        }
    }
}

/// Non-blocking honesty marker for a package-script contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptWarningClass {
    /// Node came from captured ambient PATH facts rather than a workspace pin.
    AmbientNodeRuntime,
    /// Package manager fell back to npm.
    PackageManagerFallback,
    /// The execution context carries a toolchain fallback marker.
    ToolchainFallback,
}

impl PackageScriptWarningClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbientNodeRuntime => "ambient_node_runtime",
            Self::PackageManagerFallback => "package_manager_fallback",
            Self::ToolchainFallback => "toolchain_fallback",
        }
    }
}

/// Rerun mode selected for a package-script contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageScriptRerunMode {
    /// Preserve the original execution-context reference.
    ExactContext,
    /// Use the current freshly resolved execution context.
    CurrentContext,
}

impl PackageScriptRerunMode {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactContext => "exact_context",
            Self::CurrentContext => "current_context",
        }
    }
}

/// Rerun lineage attached to a package-script attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptRerunLineage {
    /// Rerun mode.
    pub mode: PackageScriptRerunMode,
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

/// Launch and rerun contract for one package script.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageScriptRunContract {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub run_contract_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Stable task id for this script.
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
    /// Script descriptor.
    pub script: PackageScriptDescriptor,
    /// Task wedge classification.
    pub wedge: TaskWedgeClass,
    /// Package-manager runner when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runner: Option<PackageScriptRunner>,
    /// Direct-process dispatch when runnable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dispatch: Option<PackageScriptDispatch>,
    /// Launch readiness.
    pub readiness: PackageScriptLaunchReadiness,
    /// Blocking reasons.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blockers: Vec<PackageScriptBlockReason>,
    /// Non-blocking honesty markers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<PackageScriptWarningClass>,
    /// Workspace revision captured by the caller, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_revision: Option<String>,
    /// Rerun lineage when this contract represents a retry/rerun attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_lineage: Option<PackageScriptRerunLineage>,
}

impl PackageScriptRunContract {
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
                lifecycle_reason: Some("package script queued".to_owned()),
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
                    unblock_ref: Some("doctor.repair.node_toolchain".to_owned()),
                },
                observed_at,
            ))?;
        } else {
            stream.append(self.event(
                sequence_for_attempt(self.attempt_number, 2),
                TaskEventKind::TaskStarted,
                TaskStateClass::Running,
                TaskEventPayload::Lifecycle {
                    lifecycle_reason: Some("package-manager runner started".to_owned()),
                    exit_status: None,
                },
                observed_at,
            ))?;
        }
        Ok(stream)
    }

    /// Builds a rerun contract from a freshly resolved current context.
    ///
    /// The method keeps the run id stable, increments the attempt identity,
    /// updates target/cwd from the supplied context when
    /// [`PackageScriptRerunMode::CurrentContext`] is selected, and records
    /// exact-vs-current context drift in [`PackageScriptRerunLineage`].
    pub fn rerun_with_context(
        &self,
        current_context: &ExecutionContext,
        mode: PackageScriptRerunMode,
        observed_at: &str,
    ) -> Self {
        let mut next = self.clone();
        let previous_attempt_id = self.attempt_id.clone();
        let previous_execution_context_ref = self.execution_context_ref.clone();
        next.attempt_number = self.attempt_number.saturating_add(1);
        next.attempt_id = format!("attempt:{}:{}", self.task_id, next.attempt_number);

        if mode == PackageScriptRerunMode::CurrentContext {
            next.execution_context_ref = current_context.execution_context_id.clone();
            next.target_id = current_context.target_identity.canonical_target_id.clone();
            next.context_provenance = ExecutionEventProvenance::from_context(current_context);
            if let Some(dispatch) = &mut next.dispatch {
                dispatch.working_directory =
                    current_context.target_identity.working_directory.clone();
            }
        }

        next.rerun_lineage = Some(PackageScriptRerunLineage {
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
        let confidence = if self.readiness == PackageScriptLaunchReadiness::Ready {
            TaskEventConfidence::High
        } else if self.readiness == PackageScriptLaunchReadiness::ReadyWithHonestyMarker {
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
                source_adapter_id: "adapter:package_scripts".to_owned(),
                adapter_version: PACKAGE_SCRIPT_DISCOVERER_VERSION.to_owned(),
                workspace_revision: self.workspace_revision.clone(),
                confidence,
                context_provenance: Some(self.context_provenance.clone()),
            },
            raw_envelope: self.raw_envelope(&event_id, source_kind, event_kind, observed_at),
        }
    }

    fn event_summary(&self, event_kind: TaskEventKind, state: TaskStateClass) -> String {
        format!(
            "Package script `{}` {} ({})",
            safe_script_label(&self.script.name),
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
            "script_name": self.script.name,
            "script_source_ref": self.script.source.source_ref,
            "readiness": self.readiness.as_str(),
            "dispatch_program": dispatch_program,
            "dispatch_args": dispatch_args,
            "shell_mode": self.dispatch.as_ref().map(|dispatch| dispatch.shell_mode.as_str()),
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
            adapter_origin_event_id: format!("package-script:{event_id}"),
            redaction_class: TaskEventRedactionClass::MetadataSafeDefault,
            retention_state: RawEnvelopeRetentionState::RetainedInlineRedacted,
            payload_digest: digest_token(&retained_payload.to_string()),
            retained_payload: Some(retained_payload),
            retained_at: observed_at.to_owned(),
            reconstruction_fields: vec![
                "script_name".to_owned(),
                "script_source_ref".to_owned(),
                "readiness".to_owned(),
                "dispatch_program".to_owned(),
                "dispatch_args".to_owned(),
                "shell_mode".to_owned(),
            ],
        }
    }

    fn task_block_reason(&self) -> TaskBlockReason {
        match self.blockers.first().copied() {
            Some(PackageScriptBlockReason::PendingTrust) => TaskBlockReason::TrustReview,
            Some(PackageScriptBlockReason::PolicyOrTrustBlocked) => TaskBlockReason::PolicyReview,
            Some(PackageScriptBlockReason::TargetUnavailable) => TaskBlockReason::TargetUnavailable,
            _ => TaskBlockReason::DependencyMissing,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PackageScriptManifestState {
    Present,
    Missing,
    Unreadable,
    Invalid,
    ScriptsInvalid,
}

#[derive(Debug, Clone)]
struct PackageScriptRead {
    manifest_state: PackageScriptManifestState,
    scripts: BTreeMap<String, String>,
    error: Option<String>,
}

fn read_package_scripts(workspace_root: &Path) -> PackageScriptRead {
    let path = workspace_root.join("package.json");
    let payload = match fs::read_to_string(&path) {
        Ok(payload) => payload,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return PackageScriptRead {
                manifest_state: PackageScriptManifestState::Missing,
                scripts: BTreeMap::new(),
                error: None,
            };
        }
        Err(err) => {
            return PackageScriptRead {
                manifest_state: PackageScriptManifestState::Unreadable,
                scripts: BTreeMap::new(),
                error: Some(format!("package.json could not be read: {err}")),
            };
        }
    };

    let value: Value = match serde_json::from_str(&payload) {
        Ok(value) => value,
        Err(err) => {
            return PackageScriptRead {
                manifest_state: PackageScriptManifestState::Invalid,
                scripts: BTreeMap::new(),
                error: Some(format!("package.json could not be parsed: {err}")),
            };
        }
    };

    let Some(scripts_value) = value.get("scripts") else {
        return PackageScriptRead {
            manifest_state: PackageScriptManifestState::Present,
            scripts: BTreeMap::new(),
            error: None,
        };
    };
    let Some(scripts_object) = scripts_value.as_object() else {
        return PackageScriptRead {
            manifest_state: PackageScriptManifestState::ScriptsInvalid,
            scripts: BTreeMap::new(),
            error: Some("package.json#scripts must be a JSON object".to_owned()),
        };
    };

    let scripts = scripts_object
        .iter()
        .filter_map(|(name, value)| {
            value
                .as_str()
                .map(|body| (name.to_owned(), body.to_owned()))
        })
        .collect::<BTreeMap<_, _>>();

    PackageScriptRead {
        manifest_state: PackageScriptManifestState::Present,
        scripts,
        error: None,
    }
}

fn build_run_contracts(
    workspace_id: &str,
    context: &ExecutionContext,
    runtime_status: &PackageScriptRuntimeStatus,
    all_scripts: &[PackageScriptDescriptor],
    contract_scripts: &[PackageScriptDescriptor],
    workspace_revision: &Option<String>,
) -> Vec<PackageScriptRunContract> {
    let runner = runner_from_status(runtime_status);
    contract_scripts
        .iter()
        .map(|script| {
            let blockers = blockers_for(context, runtime_status, runner.as_ref());
            let warnings = warnings_for(context, runtime_status);
            let readiness = if blockers.is_empty() {
                if warnings.is_empty() {
                    PackageScriptLaunchReadiness::Ready
                } else {
                    PackageScriptLaunchReadiness::ReadyWithHonestyMarker
                }
            } else {
                PackageScriptLaunchReadiness::Blocked
            };
            let task_id = format!("task:package-script:{}", stable_token(&script.name));
            let run_id = format!("run:{task_id}");
            let dispatch = runner
                .as_ref()
                .filter(|_| !readiness.is_blocked())
                .map(|runner| dispatch_for(script, runner, context, all_scripts));
            PackageScriptRunContract {
                record_kind: PACKAGE_SCRIPT_RUN_CONTRACT_RECORD_KIND.to_owned(),
                schema_version: PACKAGE_SCRIPT_DISCOVERY_SCHEMA_VERSION,
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
                script: script.clone(),
                wedge: script.wedge,
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

fn dispatch_for(
    script: &PackageScriptDescriptor,
    runner: &PackageScriptRunner,
    context: &ExecutionContext,
    all_scripts: &[PackageScriptDescriptor],
) -> PackageScriptDispatch {
    PackageScriptDispatch {
        program: runner.program.clone(),
        args: vec!["run".to_owned(), script.name.clone()],
        working_directory: context.target_identity.working_directory.clone(),
        shell_mode: PackageScriptShellMode::DirectProcess,
        package_manager_owns_script_lifecycle: true,
        related_lifecycle_hooks: lifecycle_hooks_for(script, all_scripts),
    }
}

fn lifecycle_hooks_for(
    script: &PackageScriptDescriptor,
    all_scripts: &[PackageScriptDescriptor],
) -> Vec<PackageScriptLifecycleHook> {
    let hook_names = [
        format!("pre{}", script.name),
        format!("post{}", script.name),
    ];
    hook_names
        .iter()
        .filter_map(|name| {
            all_scripts
                .iter()
                .find(|candidate| candidate.name == *name)
                .map(|candidate| PackageScriptLifecycleHook {
                    name: candidate.name.clone(),
                    source_ref: candidate.source.source_ref.clone(),
                })
        })
        .collect()
}

fn runner_from_status(status: &PackageScriptRuntimeStatus) -> Option<PackageScriptRunner> {
    let kind = status.package_manager_kind?;
    if !kind.is_launch_wedge_supported() {
        return None;
    }
    Some(PackageScriptRunner {
        kind,
        program: kind.as_str().to_owned(),
        version: status
            .package_manager_value_token
            .as_deref()
            .and_then(|value| value.split_once('@').map(|(_, version)| version.to_owned())),
    })
}

fn blockers_for(
    context: &ExecutionContext,
    status: &PackageScriptRuntimeStatus,
    runner: Option<&PackageScriptRunner>,
) -> Vec<PackageScriptBlockReason> {
    let mut blockers = Vec::new();
    if context.policy_and_trust.trust_state == TrustState::PendingEvaluation {
        blockers.push(PackageScriptBlockReason::PendingTrust);
    }
    match status.node_resolution_state {
        NodeToolchainResolutionState::Missing => {
            blockers.push(PackageScriptBlockReason::MissingNodeRuntime)
        }
        NodeToolchainResolutionState::Ambiguous => {
            blockers.push(PackageScriptBlockReason::AmbiguousNodeRuntime)
        }
        NodeToolchainResolutionState::Unsupported => {
            blockers.push(PackageScriptBlockReason::UnsupportedNodeRuntime)
        }
        NodeToolchainResolutionState::Resolved | NodeToolchainResolutionState::Fallback => {}
    }
    match status.package_manager_resolution_state {
        NodeToolchainResolutionState::Missing => {
            blockers.push(PackageScriptBlockReason::MissingPackageManager)
        }
        NodeToolchainResolutionState::Ambiguous => {
            blockers.push(PackageScriptBlockReason::AmbiguousPackageManager)
        }
        NodeToolchainResolutionState::Unsupported => {
            blockers.push(PackageScriptBlockReason::UnsupportedPackageManager)
        }
        NodeToolchainResolutionState::Resolved | NodeToolchainResolutionState::Fallback => {}
    }
    if status.package_manager_kind.is_some() && runner.is_none() {
        blockers.push(PackageScriptBlockReason::UnsupportedPackageManager);
    }
    if context.target_identity.reachability_state != ReachabilityState::Reachable {
        blockers.push(PackageScriptBlockReason::TargetUnavailable);
    }
    for field in &context.degraded_fields {
        if field.field_path.starts_with("node_toolchain_detection.") {
            continue;
        }
        match field.reason {
            DegradedFieldReason::ToolchainFallback => {}
            DegradedFieldReason::ActivatorBlockedByTrust
            | DegradedFieldReason::ActivatorBlockedByPolicy => {
                blockers.push(PackageScriptBlockReason::PolicyOrTrustBlocked)
            }
            DegradedFieldReason::TrustStateUnresolved => {
                blockers.push(PackageScriptBlockReason::PendingTrust)
            }
            _ => blockers.push(PackageScriptBlockReason::DegradedExecutionContext),
        }
    }
    dedupe_blockers(blockers)
}

fn warnings_for(
    context: &ExecutionContext,
    status: &PackageScriptRuntimeStatus,
) -> Vec<PackageScriptWarningClass> {
    let mut warnings = Vec::new();
    if context
        .node_toolchain_detection
        .as_ref()
        .and_then(|detection| detection.node_runtime.winning_source)
        == Some(NodeToolchainSourceKind::AmbientPath)
    {
        warnings.push(PackageScriptWarningClass::AmbientNodeRuntime);
    }
    if status.package_manager_resolution_state == NodeToolchainResolutionState::Fallback {
        warnings.push(PackageScriptWarningClass::PackageManagerFallback);
    }
    if context.degraded_fields.iter().any(|field| {
        field.reason == DegradedFieldReason::ToolchainFallback
            && field.field_path.starts_with("node_toolchain_detection.")
    }) {
        warnings.push(PackageScriptWarningClass::ToolchainFallback);
    }
    dedupe_warnings(warnings)
}

fn dedupe_blockers(blockers: Vec<PackageScriptBlockReason>) -> Vec<PackageScriptBlockReason> {
    let mut out = Vec::new();
    for blocker in blockers {
        if !out.contains(&blocker) {
            out.push(blocker);
        }
    }
    out
}

fn dedupe_warnings(warnings: Vec<PackageScriptWarningClass>) -> Vec<PackageScriptWarningClass> {
    let mut out = Vec::new();
    for warning in warnings {
        if !out.contains(&warning) {
            out.push(warning);
        }
    }
    out
}

fn package_manager_value_token(detection: &NodeToolchainDetection) -> Option<String> {
    let kind = detection.package_manager.kind?;
    Some(match &detection.package_manager.version {
        Some(version) if !version.is_empty() => format!("{}@{version}", kind.as_str()),
        _ => kind.as_str().to_owned(),
    })
}

fn push_runtime_state(
    states: &mut Vec<PackageScriptMissingRuntimeState>,
    state: NodeToolchainResolutionState,
    missing: PackageScriptMissingRuntimeState,
    ambiguous: PackageScriptMissingRuntimeState,
    unsupported: PackageScriptMissingRuntimeState,
) {
    match state {
        NodeToolchainResolutionState::Missing => states.push(missing),
        NodeToolchainResolutionState::Ambiguous => states.push(ambiguous),
        NodeToolchainResolutionState::Unsupported => states.push(unsupported),
        NodeToolchainResolutionState::Resolved | NodeToolchainResolutionState::Fallback => {}
    }
}

fn discovery_state_for(
    manifest_state: PackageScriptManifestState,
    no_scripts: bool,
    no_contracts: bool,
) -> PackageScriptDiscoveryState {
    match manifest_state {
        PackageScriptManifestState::Missing => PackageScriptDiscoveryState::NoPackageManifest,
        PackageScriptManifestState::Unreadable => PackageScriptDiscoveryState::ManifestUnreadable,
        PackageScriptManifestState::Invalid => PackageScriptDiscoveryState::ManifestInvalid,
        PackageScriptManifestState::ScriptsInvalid => {
            PackageScriptDiscoveryState::ScriptsFieldInvalid
        }
        PackageScriptManifestState::Present if no_scripts => PackageScriptDiscoveryState::NoScripts,
        PackageScriptManifestState::Present if no_contracts => {
            PackageScriptDiscoveryState::NoRunnableScripts
        }
        PackageScriptManifestState::Present => PackageScriptDiscoveryState::Ready,
    }
}

fn is_launch_wedge_script(name: &str) -> bool {
    matches!(
        name,
        "build" | "test" | "typecheck" | "lint" | "dev" | "start" | "check"
    ) || name.starts_with("test:")
        || name.starts_with("build:")
        || name.starts_with("lint:")
        || name.starts_with("typecheck:")
}

fn classify_script(name: &str) -> TaskWedgeClass {
    if name == "test" || name.starts_with("test:") {
        TaskWedgeClass::Test
    } else if name == "dev" || name == "start" {
        TaskWedgeClass::Terminal
    } else if name == "build"
        || name.starts_with("build:")
        || name == "typecheck"
        || name.starts_with("typecheck:")
        || name == "lint"
        || name.starts_with("lint:")
        || name == "check"
    {
        TaskWedgeClass::Build
    } else {
        TaskWedgeClass::Generic
    }
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

fn safe_script_label(raw: &str) -> String {
    raw.chars()
        .filter(|ch| !ch.is_control())
        .collect::<String>()
}

fn json_pointer_segment(raw: &str) -> String {
    raw.replace('~', "~0").replace('/', "~1")
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
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        TargetClass, ToolchainClass,
    };

    fn fixture_root(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/tsjs_task_discovery_alpha")
            .join(name)
    }

    fn baseline_resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "workspace:web".to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace/web".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:workspace:web".to_owned(),
                capsule_hash: "sha256:web".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "test-resolver".to_owned(),
        })
    }

    fn package_script_context(
        resolver: &mut ExecutionContextResolver,
        observed_at: &str,
    ) -> ExecutionContext {
        resolver.resolve(ExecutionContextRequest::package_script_task_seed(
            "task.run.package_script",
            TrustState::Trusted,
            observed_at,
        ))
    }

    fn discoverer_with_ambient() -> PackageScriptDiscoverer {
        PackageScriptDiscoverer::new(PackageScriptDiscovererConfig {
            node_detector: NodeToolchainDetectorConfig {
                ambient_node_version: Some("22.11.0".to_owned()),
                ambient_npm_version: Some("10.9.0".to_owned()),
                ambient_pnpm_version: Some("9.15.4".to_owned()),
                ..NodeToolchainDetectorConfig::default()
            },
            workspace_revision: Some("rev:web".to_owned()),
        })
    }

    #[test]
    fn ready_pnpm_workspace_discovers_bounded_scripts_and_direct_process_contracts() {
        let mut resolver = baseline_resolver();
        let context = package_script_context(&mut resolver, "2026-05-13T16:20:00Z");
        assert_eq!(
            context.toolchain_identity.toolchain_class,
            ToolchainClass::PackageManagerRunner
        );

        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_pnpm"),
            context,
            "2026-05-13T16:20:01Z",
        );

        assert_eq!(discovery.record_kind, PACKAGE_SCRIPT_DISCOVERY_RECORD_KIND);
        assert_eq!(
            discovery.discovery_state,
            PackageScriptDiscoveryState::Ready
        );
        assert_eq!(
            discovery.runtime_status.package_manager_kind,
            Some(NodePackageManagerKind::Pnpm)
        );
        assert!(!discovery.runtime_status.has_missing_or_blocked_runtime());
        assert!(discovery
            .scripts
            .iter()
            .any(|script| script.name == "prebuild" && !script.runnable_in_launch_wedge));

        let build = discovery
            .contract_for_script("build")
            .expect("build contract");
        assert_eq!(build.readiness, PackageScriptLaunchReadiness::Ready);
        assert_eq!(build.wedge, TaskWedgeClass::Build);
        assert_eq!(
            build.runner.as_ref().map(|runner| runner.program.as_str()),
            Some("pnpm")
        );
        let dispatch = build.dispatch.as_ref().expect("ready dispatch");
        assert_eq!(dispatch.program, "pnpm");
        assert_eq!(dispatch.args, vec!["run".to_owned(), "build".to_owned()]);
        assert_eq!(dispatch.shell_mode, PackageScriptShellMode::DirectProcess);
        assert!(dispatch.package_manager_owns_script_lifecycle);
        assert!(dispatch
            .related_lifecycle_hooks
            .iter()
            .any(|hook| hook.name == "prebuild"));
        assert!(!dispatch.args.iter().any(|arg| arg == "sh" || arg == "-c"));

        let test = discovery
            .contract_for_script("test")
            .expect("test contract");
        assert_eq!(test.wedge, TaskWedgeClass::Test);
        assert_eq!(test.script.source.source_ref, "package.json#/scripts/test");
    }

    #[test]
    fn launch_contract_projects_into_canonical_task_stream_and_support_export() {
        let mut resolver = baseline_resolver();
        let context = package_script_context(&mut resolver, "2026-05-13T16:21:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_pnpm"),
            context,
            "2026-05-13T16:21:01Z",
        );
        let contract = discovery
            .contract_for_script("test:unit")
            .expect("test:unit contract");

        let stream = contract
            .launch_event_stream("2026-05-13T16:21:02Z")
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
            "support-export:package-script:test-unit",
            "2026-05-13T16:21:03Z",
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
        let context = package_script_context(&mut resolver, "2026-05-13T16:22:00Z");
        let discovery = PackageScriptDiscoverer::default_read_only().discover_workspace(
            &fixture_root("missing_node_runtime"),
            context,
            "2026-05-13T16:22:01Z",
        );

        assert_eq!(
            discovery.discovery_state,
            PackageScriptDiscoveryState::Ready
        );
        assert!(discovery
            .runtime_status
            .missing_runtime_states
            .contains(&PackageScriptMissingRuntimeState::NodeRuntimeMissing));
        assert!(discovery
            .runtime_status
            .missing_runtime_states
            .contains(&PackageScriptMissingRuntimeState::PackageManagerMissing));
        assert!(discovery.honesty_marker_present);

        let build = discovery
            .contract_for_script("build")
            .expect("build contract");
        assert_eq!(build.readiness, PackageScriptLaunchReadiness::Blocked);
        assert!(build
            .blockers
            .contains(&PackageScriptBlockReason::MissingNodeRuntime));
        assert!(build
            .blockers
            .contains(&PackageScriptBlockReason::MissingPackageManager));
        assert!(!build
            .blockers
            .contains(&PackageScriptBlockReason::UnsupportedPackageManager));
        assert!(build.dispatch.is_none());

        let stream = build
            .launch_event_stream("2026-05-13T16:22:02Z")
            .expect("blocked stream");
        assert_eq!(stream.events[1].event_kind, TaskEventKind::TaskBlocked);
        assert_eq!(stream.events[1].state_after, TaskStateClass::Blocked);
        assert_eq!(
            stream
                .state_for_task(&build.task_id)
                .expect("task state")
                .current_state,
            TaskStateClass::Blocked
        );
        assert!(stream.shell_projection()[1].needs_attention);
    }

    #[test]
    fn unsupported_package_manager_keeps_script_source_but_blocks_dispatch() {
        let mut resolver = baseline_resolver();
        let context = package_script_context(&mut resolver, "2026-05-13T16:23:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("unsupported_yarn"),
            context,
            "2026-05-13T16:23:01Z",
        );

        let test = discovery
            .contract_for_script("test")
            .expect("test contract");
        assert_eq!(
            discovery.runtime_status.package_manager_kind,
            Some(NodePackageManagerKind::Yarn)
        );
        assert!(test
            .blockers
            .contains(&PackageScriptBlockReason::UnsupportedPackageManager));
        assert_eq!(test.script.source.source_ref, "package.json#/scripts/test");
        assert!(test.dispatch.is_none());
    }

    #[test]
    fn rerun_contract_preserves_run_identity_and_records_context_drift() {
        let mut resolver = baseline_resolver();
        let original_context = package_script_context(&mut resolver, "2026-05-13T16:24:00Z");
        let discovery = discoverer_with_ambient().discover_workspace(
            &fixture_root("ready_pnpm"),
            original_context,
            "2026-05-13T16:24:01Z",
        );
        let build = discovery
            .contract_for_script("build")
            .expect("build contract");

        let mut current_request = ExecutionContextRequest::package_script_task_seed(
            "task.run.package_script",
            TrustState::Trusted,
            "2026-05-13T16:24:02Z",
        );
        current_request.override_target_class = Some(TargetClass::SshRemote);
        current_request.override_working_directory = Some("/srv/web");
        let current_context = resolver.resolve(current_request);

        let rerun = build.rerun_with_context(
            &current_context,
            PackageScriptRerunMode::CurrentContext,
            "2026-05-13T16:24:03Z",
        );
        assert_eq!(rerun.run_id, build.run_id);
        assert_ne!(rerun.attempt_id, build.attempt_id);
        assert_eq!(rerun.attempt_number, 2);
        assert_eq!(
            rerun
                .dispatch
                .as_ref()
                .and_then(|dispatch| dispatch.working_directory.as_deref()),
            Some("/srv/web")
        );
        let lineage = rerun.rerun_lineage.as_ref().expect("rerun lineage");
        assert_eq!(lineage.mode, PackageScriptRerunMode::CurrentContext);
        assert!(lineage.context_changed);
        assert_eq!(
            lineage.previous_execution_context_ref,
            build.execution_context_ref
        );
        assert_eq!(
            lineage.current_execution_context_ref,
            current_context.execution_context_id
        );
        assert!(rerun.context_provenance.matches_context(&current_context));

        let initial_stream = build
            .launch_event_stream("2026-05-13T16:24:04Z")
            .expect("initial stream");
        let rerun_stream = rerun
            .launch_event_stream("2026-05-13T16:24:05Z")
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
