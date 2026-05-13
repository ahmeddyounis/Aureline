//! Rerun-last task and test loop contracts.
//!
//! This module stores the last launch contract for task and test lanes together
//! with the exact [`ExecutionContext`] that produced the attempt. Before a
//! rerun is prepared, callers supply a freshly resolved current context; the
//! loop compares exact-prior target truth with current target truth and marks
//! drift for review before any dispatch can proceed.

use serde::{Deserialize, Serialize};

use crate::discovery::package_scripts::{PackageScriptRerunMode, PackageScriptRunContract};
use crate::discovery::pytest::{PytestRerunMode, PytestRunContract};
use crate::execution_context::{
    ConfidenceLevel, ExecutionContext, ReachabilityState, ScopeClass, SurfaceClass, TargetClass,
    ToolchainClass,
};
use crate::tasks::{TaskEventStream, TaskEventStreamError, TaskWedgeClass};
use crate::TrustState;

/// Schema version for rerun-last loop records.
pub const RERUN_LOOP_SCHEMA_VERSION: u32 = 1;
/// Stable record-kind tag for one keyboard command binding.
pub const RERUN_COMMAND_BINDING_RECORD_KIND: &str = "rerun_command_binding_record";
/// Stable record-kind tag for one remembered last-run launch contract.
pub const RERUN_LAST_LAUNCH_RECORD_KIND: &str = "rerun_last_launch_record";
/// Stable record-kind tag for exact-prior versus current target comparison.
pub const RERUN_TARGET_COMPARISON_RECORD_KIND: &str = "rerun_target_comparison_record";
/// Stable record-kind tag for a prepared rerun-last attempt.
pub const RERUN_PREPARED_ATTEMPT_RECORD_KIND: &str = "rerun_prepared_attempt_record";
/// Stable record-kind tag for a support/export projection of rerun state.
pub const RERUN_SUPPORT_EXPORT_RECORD_KIND: &str = "rerun_support_export_record";

/// Canonical command id for rerunning the last task.
pub const RERUN_LAST_TASK_COMMAND_ID: &str = "cmd:task.rerun_last";
/// Canonical command id for rerunning the last test.
pub const RERUN_LAST_TEST_COMMAND_ID: &str = "cmd:test.rerun_last";

/// Rerun lane addressed by a last-run command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunLane {
    /// Build, package, terminal-backed, or generic task lane.
    Task,
    /// Test-run lane.
    Test,
}

impl RerunLane {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Task => "task",
            Self::Test => "test",
        }
    }

    /// Human-readable label for launch-wedge surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Task => "Task",
            Self::Test => "Test",
        }
    }

    /// Canonical rerun-last command id for this lane.
    pub const fn command_id(self) -> &'static str {
        match self {
            Self::Task => RERUN_LAST_TASK_COMMAND_ID,
            Self::Test => RERUN_LAST_TEST_COMMAND_ID,
        }
    }

    /// Runtime surface expected to resolve the current context.
    pub const fn expected_surface(self) -> SurfaceClass {
        match self {
            Self::Task => SurfaceClass::Task,
            Self::Test => SurfaceClass::Test,
        }
    }

    fn no_prior_reason(self) -> RerunUnavailableReason {
        match self {
            Self::Task => RerunUnavailableReason::NoPriorTask,
            Self::Test => RerunUnavailableReason::NoPriorTest,
        }
    }
}

/// Target mode selected by a rerun-last command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunTargetMode {
    /// Reuse the exact prior execution-context target.
    ExactPriorTarget,
    /// Use the freshly resolved current execution-context target.
    CurrentResolvedTarget,
}

impl RerunTargetMode {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactPriorTarget => "exact_prior_target",
            Self::CurrentResolvedTarget => "current_resolved_target",
        }
    }

    /// Human-readable label for a rerun comparison surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactPriorTarget => "Rerun exactly",
            Self::CurrentResolvedTarget => "Rerun with current context",
        }
    }
}

/// Dispatch state after target comparison and prior-contract lookup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunDispatchState {
    /// The rerun can dispatch without a drift review.
    Ready,
    /// A drift review must be shown before dispatch.
    ReviewRequired,
    /// No dispatch can be prepared.
    Unavailable,
}

impl RerunDispatchState {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReviewRequired => "review_required",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Reason a rerun-last command cannot prepare an attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunUnavailableReason {
    /// No prior task contract has been remembered.
    NoPriorTask,
    /// No prior test contract has been remembered.
    NoPriorTest,
}

impl RerunUnavailableReason {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPriorTask => "no_prior_task",
            Self::NoPriorTest => "no_prior_test",
        }
    }
}

/// Contract kind remembered by the rerun loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunContractKind {
    /// TS/JS package-script run contract.
    PackageScript,
    /// Python pytest run contract.
    Pytest,
}

impl RerunContractKind {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageScript => "package_script",
            Self::Pytest => "pytest",
        }
    }
}

/// One default keyboard route for a rerun command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunKeyboardRoute {
    /// Platform token such as `macos`, `windows`, or `linux`.
    pub platform_class: String,
    /// Human-readable key sequence.
    pub sequence: String,
    /// Source layer that supplied the binding.
    pub source_layer: String,
}

/// Command binding metadata projected to launch-wedge surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunCommandBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Rerun lane addressed by the command.
    pub lane: RerunLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Canonical command id.
    pub command_id: String,
    /// User-visible command label.
    pub label: String,
    /// True when a keybinding route exists.
    pub keyboard_reachable: bool,
    /// Default keyboard routes exposed by the launch wedge.
    pub default_keyboard_routes: Vec<RerunKeyboardRoute>,
    /// UI slots that should expose the command.
    pub ui_slot_tokens: Vec<String>,
}

impl RerunCommandBinding {
    /// Returns the built-in binding metadata for one rerun lane.
    pub fn for_lane(lane: RerunLane) -> Self {
        let (label, macos, other) = match lane {
            RerunLane::Task => ("Rerun Last Task", "Cmd+Alt+R", "Ctrl+Alt+R"),
            RerunLane::Test => ("Rerun Last Test", "Cmd+Alt+T", "Ctrl+Alt+T"),
        };
        Self {
            record_kind: RERUN_COMMAND_BINDING_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            lane,
            lane_token: lane.as_str().to_owned(),
            command_id: lane.command_id().to_owned(),
            label: label.to_owned(),
            keyboard_reachable: true,
            default_keyboard_routes: vec![
                RerunKeyboardRoute {
                    platform_class: "macos".to_owned(),
                    sequence: macos.to_owned(),
                    source_layer: "core_default".to_owned(),
                },
                RerunKeyboardRoute {
                    platform_class: "windows".to_owned(),
                    sequence: other.to_owned(),
                    source_layer: "core_default".to_owned(),
                },
                RerunKeyboardRoute {
                    platform_class: "linux".to_owned(),
                    sequence: other.to_owned(),
                    source_layer: "core_default".to_owned(),
                },
            ],
            ui_slot_tokens: vec![
                "command_palette".to_owned(),
                "keybinding_help".to_owned(),
                "launch_wedge".to_owned(),
            ],
        }
    }
}

/// Returns both built-in rerun-last command bindings.
pub fn built_in_rerun_command_bindings() -> [RerunCommandBinding; 2] {
    [
        RerunCommandBinding::for_lane(RerunLane::Task),
        RerunCommandBinding::for_lane(RerunLane::Test),
    ]
}

/// Run-contract payload stored or prepared by the rerun loop.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "contract_kind", content = "contract", rename_all = "snake_case")]
pub enum RerunRunContract {
    /// TS/JS package-script run contract.
    PackageScript(PackageScriptRunContract),
    /// Python pytest run contract.
    Pytest(PytestRunContract),
}

impl RerunRunContract {
    /// Returns the contract kind.
    pub fn kind(&self) -> RerunContractKind {
        match self {
            Self::PackageScript(_) => RerunContractKind::PackageScript,
            Self::Pytest(_) => RerunContractKind::Pytest,
        }
    }

    /// Returns the stable workspace id.
    pub fn workspace_id(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.workspace_id,
            Self::Pytest(contract) => &contract.workspace_id,
        }
    }

    /// Returns the stable task id.
    pub fn task_id(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.task_id,
            Self::Pytest(contract) => &contract.task_id,
        }
    }

    /// Returns the stable run id.
    pub fn run_id(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.run_id,
            Self::Pytest(contract) => &contract.run_id,
        }
    }

    /// Returns the attempt id.
    pub fn attempt_id(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.attempt_id,
            Self::Pytest(contract) => &contract.attempt_id,
        }
    }

    /// Returns the attempt number.
    pub fn attempt_number(&self) -> u32 {
        match self {
            Self::PackageScript(contract) => contract.attempt_number,
            Self::Pytest(contract) => contract.attempt_number,
        }
    }

    /// Returns the execution-context reference used by the attempt.
    pub fn execution_context_ref(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.execution_context_ref,
            Self::Pytest(contract) => &contract.execution_context_ref,
        }
    }

    /// Returns the target id used by the attempt.
    pub fn target_id(&self) -> &str {
        match self {
            Self::PackageScript(contract) => &contract.target_id,
            Self::Pytest(contract) => &contract.target_id,
        }
    }

    /// Returns the task wedge class used by the attempt.
    pub fn wedge(&self) -> TaskWedgeClass {
        match self {
            Self::PackageScript(contract) => contract.wedge,
            Self::Pytest(contract) => contract.wedge,
        }
    }

    /// Returns a compact label for support exports and wedge rows.
    pub fn display_label(&self) -> String {
        match self {
            Self::PackageScript(contract) => format!("package script `{}`", contract.script.name),
            Self::Pytest(contract) => format!("pytest `{}`", contract.selection.label),
        }
    }

    /// Builds a next-attempt contract for the selected rerun target mode.
    pub fn rerun_with_context(
        &self,
        current_context: &ExecutionContext,
        mode: RerunTargetMode,
        observed_at: &str,
    ) -> Self {
        match self {
            Self::PackageScript(contract) => Self::PackageScript(contract.rerun_with_context(
                current_context,
                match mode {
                    RerunTargetMode::ExactPriorTarget => PackageScriptRerunMode::ExactContext,
                    RerunTargetMode::CurrentResolvedTarget => {
                        PackageScriptRerunMode::CurrentContext
                    }
                },
                observed_at,
            )),
            Self::Pytest(contract) => Self::Pytest(contract.rerun_with_context(
                current_context,
                match mode {
                    RerunTargetMode::ExactPriorTarget => PytestRerunMode::ExactContext,
                    RerunTargetMode::CurrentResolvedTarget => PytestRerunMode::CurrentContext,
                },
                observed_at,
            )),
        }
    }

    /// Builds the canonical task-event stream for the prepared attempt.
    pub fn launch_event_stream(
        &self,
        observed_at: &str,
    ) -> Result<TaskEventStream, TaskEventStreamError> {
        match self {
            Self::PackageScript(contract) => contract.launch_event_stream(observed_at),
            Self::Pytest(contract) => contract.launch_event_stream(observed_at),
        }
    }
}

/// Compact attempt identity stored in preview and support records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunAttemptSummary {
    /// Contract implementation kind.
    pub contract_kind: RerunContractKind,
    /// Stable contract-kind token.
    pub contract_kind_token: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Stable task id.
    pub task_id: String,
    /// Stable run id.
    pub run_id: String,
    /// Attempt id.
    pub attempt_id: String,
    /// Attempt number.
    pub attempt_number: u32,
    /// Execution-context reference.
    pub execution_context_ref: String,
    /// Canonical target id.
    pub target_id: String,
    /// Task wedge class.
    pub wedge: TaskWedgeClass,
    /// Stable task wedge token.
    pub wedge_token: String,
    /// Compact label for the run contract.
    pub display_label: String,
}

impl RerunAttemptSummary {
    /// Projects an attempt summary from a run contract.
    pub fn project(contract: &RerunRunContract) -> Self {
        let contract_kind = contract.kind();
        let wedge = contract.wedge();
        Self {
            contract_kind,
            contract_kind_token: contract_kind.as_str().to_owned(),
            workspace_id: contract.workspace_id().to_owned(),
            task_id: contract.task_id().to_owned(),
            run_id: contract.run_id().to_owned(),
            attempt_id: contract.attempt_id().to_owned(),
            attempt_number: contract.attempt_number(),
            execution_context_ref: contract.execution_context_ref().to_owned(),
            target_id: contract.target_id().to_owned(),
            wedge,
            wedge_token: wedge.as_str().to_owned(),
            display_label: contract.display_label(),
        }
    }
}

/// Remembered last-launch contract plus its exact execution context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunLastLaunch {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Rerun lane this launch updates.
    pub lane: RerunLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Timestamp supplied by the caller.
    pub remembered_at: String,
    /// Prior attempt summary.
    pub prior_attempt: RerunAttemptSummary,
    /// True when the stored context id matches the stored run contract.
    pub exact_context_matches_contract: bool,
    /// Exact execution context used by the prior attempt.
    pub exact_context: ExecutionContext,
    /// Prior run contract payload.
    pub contract: RerunRunContract,
}

impl RerunLastLaunch {
    /// Creates a remembered package-script launch.
    pub fn from_package_script(
        contract: PackageScriptRunContract,
        exact_context: ExecutionContext,
        remembered_at: impl Into<String>,
    ) -> Self {
        let lane = lane_for_wedge(contract.wedge);
        Self::new(
            lane,
            RerunRunContract::PackageScript(contract),
            exact_context,
            remembered_at,
        )
    }

    /// Creates a remembered pytest launch.
    pub fn from_pytest(
        contract: PytestRunContract,
        exact_context: ExecutionContext,
        remembered_at: impl Into<String>,
    ) -> Self {
        Self::new(
            RerunLane::Test,
            RerunRunContract::Pytest(contract),
            exact_context,
            remembered_at,
        )
    }

    fn new(
        lane: RerunLane,
        contract: RerunRunContract,
        exact_context: ExecutionContext,
        remembered_at: impl Into<String>,
    ) -> Self {
        let prior_attempt = RerunAttemptSummary::project(&contract);
        let exact_context_matches_contract =
            exact_context.execution_context_id == prior_attempt.execution_context_ref;
        Self {
            record_kind: RERUN_LAST_LAUNCH_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            lane,
            lane_token: lane.as_str().to_owned(),
            remembered_at: remembered_at.into(),
            prior_attempt,
            exact_context_matches_contract,
            exact_context,
            contract,
        }
    }
}

/// Export-safe target snapshot projected from an execution context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunTargetSnapshot {
    /// Execution-context reference backing this snapshot.
    pub execution_context_ref: String,
    /// Runtime surface class.
    pub surface: SurfaceClass,
    /// Stable surface token.
    pub surface_token: String,
    /// Resolved target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Resolved working directory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    /// Target reachability.
    pub reachability_state: ReachabilityState,
    /// Stable reachability token.
    pub reachability_state_token: String,
    /// True when local-vs-managed boundary chrome is required.
    pub boundary_cue_visible: bool,
    /// Target-confidence level.
    pub target_confidence_level: ConfidenceLevel,
    /// Stable target-confidence token.
    pub target_confidence_level_token: String,
    /// Structured target-confidence reason tokens.
    pub target_confidence_reason_tokens: Vec<String>,
    /// Toolchain class.
    pub toolchain_class: ToolchainClass,
    /// Stable toolchain-class token.
    pub toolchain_class_token: String,
    /// Toolchain id.
    pub toolchain_id: String,
    /// Resolved toolchain version.
    pub resolved_version: String,
    /// Trust state.
    pub trust_state: TrustState,
    /// Stable trust-state token.
    pub trust_state_token: String,
    /// Policy epoch used by the resolver.
    pub policy_epoch: u64,
    /// Workset scope class.
    pub scope_class: ScopeClass,
    /// Stable workset scope token.
    pub scope_class_token: String,
    /// Cache disposition token.
    pub cache_disposition_token: String,
    /// Prebuild reuse state token.
    pub prebuild_reuse_state_token: String,
    /// Opaque prebuild compatibility fingerprint.
    pub prebuild_compatibility_fingerprint: String,
    /// Prebuild invalidation reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_invalidation_reason_token: Option<String>,
    /// Helper/client mixed-version state token.
    pub mixed_version_state_token: String,
    /// Helper/client mixed-version reason token.
    pub mixed_version_reason_token: String,
    /// Degraded field reason tokens.
    pub degraded_field_tokens: Vec<String>,
}

impl RerunTargetSnapshot {
    /// Projects a target snapshot from the canonical runtime context.
    pub fn project(context: &ExecutionContext) -> Self {
        Self {
            execution_context_ref: context.execution_context_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            target_class: context.target_identity.target_class,
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            working_directory: context.target_identity.working_directory.clone(),
            reachability_state: context.target_identity.reachability_state,
            reachability_state_token: context
                .target_identity
                .reachability_state
                .as_str()
                .to_owned(),
            boundary_cue_visible: context.target_identity.local_vs_managed_boundary_visible,
            target_confidence_level: context.target_confidence.level,
            target_confidence_level_token: context.target_confidence.level.as_str().to_owned(),
            target_confidence_reason_tokens: context
                .target_confidence
                .reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            toolchain_class: context.toolchain_identity.toolchain_class,
            toolchain_class_token: context
                .toolchain_identity
                .toolchain_class
                .as_str()
                .to_owned(),
            toolchain_id: context.toolchain_identity.toolchain_id.clone(),
            resolved_version: context.toolchain_identity.resolved_version.clone(),
            trust_state: context.policy_and_trust.trust_state,
            trust_state_token: trust_token(context.policy_and_trust.trust_state).to_owned(),
            policy_epoch: context.policy_and_trust.policy_epoch,
            scope_class: context.workset_scope_class,
            scope_class_token: context.workset_scope_class.as_str().to_owned(),
            cache_disposition_token: context.cache_disposition.as_str().to_owned(),
            prebuild_reuse_state_token: context.prebuild_metadata.reuse_state.as_str().to_owned(),
            prebuild_compatibility_fingerprint: context
                .prebuild_metadata
                .compatibility_fingerprint
                .clone(),
            prebuild_invalidation_reason_token: context
                .prebuild_metadata
                .invalidation_reason
                .map(|reason| reason.as_str().to_owned()),
            mixed_version_state_token: context.mixed_version_drift.state.as_str().to_owned(),
            mixed_version_reason_token: context.mixed_version_drift.reason.as_str().to_owned(),
            degraded_field_tokens: context
                .degraded_fields
                .iter()
                .map(|field| field.reason.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this snapshot points at a reachable target.
    pub fn target_is_reachable(&self) -> bool {
        matches!(self.reachability_state, ReachabilityState::Reachable)
    }
}

/// Difference class for exact-prior versus current target comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunDiffClass {
    /// Both sides carry a token and the tokens differ.
    Changed,
    /// The exact-prior snapshot does not carry the field.
    MissingInExact,
    /// The current snapshot does not carry the field.
    MissingInCurrent,
}

impl RerunDiffClass {
    /// Stable string token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Changed => "changed",
            Self::MissingInExact => "missing_in_exact",
            Self::MissingInCurrent => "missing_in_current",
        }
    }
}

/// One changed field in an exact-prior versus current target comparison.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunDiffRow {
    /// Dotted field path from the execution-context projection.
    pub field_path: String,
    /// Human-readable field label.
    pub label: String,
    /// Exact-prior token when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_value_token: Option<String>,
    /// Current token when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value_token: Option<String>,
    /// Difference class.
    pub diff_class: RerunDiffClass,
    /// Stable difference-class token.
    pub diff_class_token: String,
    /// True when the changed field can alter where work executes.
    pub affects_target_boundary: bool,
    /// True when the changed field must be reviewed before dispatch.
    pub requires_review_before_dispatch: bool,
}

/// Exact-prior versus current target comparison for a rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunTargetComparison {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Exact-prior execution-context reference.
    pub exact_context_ref: String,
    /// Current execution-context reference.
    pub current_context_ref: String,
    /// Exact-prior target snapshot.
    pub exact_target: RerunTargetSnapshot,
    /// Current target snapshot.
    pub current_target: RerunTargetSnapshot,
    /// Changed field rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diff_rows: Vec<RerunDiffRow>,
    /// True when any compared field changed.
    pub has_drift: bool,
    /// True when target identity or boundary changed.
    pub target_or_boundary_changed: bool,
    /// True when the current target is reachable.
    pub current_target_reachable: bool,
    /// True when a drift sheet must be shown before dispatch.
    pub requires_review_before_dispatch: bool,
    /// Stable compact summary for support exports.
    pub summary_headline: String,
}

impl RerunTargetComparison {
    /// Compares the exact-prior context with the current context.
    pub fn compare(exact_context: &ExecutionContext, current_context: &ExecutionContext) -> Self {
        let exact_target = RerunTargetSnapshot::project(exact_context);
        let current_target = RerunTargetSnapshot::project(current_context);
        let mut diff_rows = Vec::new();

        push_diff(
            &mut diff_rows,
            "target_identity.target_class",
            "Target class",
            Some(exact_target.target_class_token.clone()),
            Some(current_target.target_class_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_identity.canonical_target_id",
            "Canonical target id",
            Some(exact_target.canonical_target_id.clone()),
            Some(current_target.canonical_target_id.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_identity.working_directory",
            "Working directory",
            exact_target.working_directory.clone(),
            current_target.working_directory.clone(),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_identity.reachability_state",
            "Target reachability",
            Some(exact_target.reachability_state_token.clone()),
            Some(current_target.reachability_state_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_identity.local_vs_managed_boundary_visible",
            "Boundary cue",
            Some(exact_target.boundary_cue_visible.to_string()),
            Some(current_target.boundary_cue_visible.to_string()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_confidence.level",
            "Target confidence",
            Some(exact_target.target_confidence_level_token.clone()),
            Some(current_target.target_confidence_level_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "target_confidence.reasons",
            "Target confidence reasons",
            Some(join_tokens(&exact_target.target_confidence_reason_tokens)),
            Some(join_tokens(&current_target.target_confidence_reason_tokens)),
            true,
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.toolchain_class",
            "Toolchain class",
            Some(exact_target.toolchain_class_token.clone()),
            Some(current_target.toolchain_class_token.clone()),
            false,
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.toolchain_id",
            "Toolchain id",
            Some(exact_target.toolchain_id.clone()),
            Some(current_target.toolchain_id.clone()),
            false,
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.resolved_version",
            "Resolved version",
            Some(exact_target.resolved_version.clone()),
            Some(current_target.resolved_version.clone()),
            false,
        );
        push_diff(
            &mut diff_rows,
            "policy_and_trust.trust_state",
            "Trust state",
            Some(exact_target.trust_state_token.clone()),
            Some(current_target.trust_state_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "policy_and_trust.policy_epoch",
            "Policy epoch",
            Some(exact_target.policy_epoch.to_string()),
            Some(current_target.policy_epoch.to_string()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "workset_scope_class",
            "Workset scope",
            Some(exact_target.scope_class_token.clone()),
            Some(current_target.scope_class_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "cache_disposition",
            "Cache disposition",
            Some(exact_target.cache_disposition_token.clone()),
            Some(current_target.cache_disposition_token.clone()),
            false,
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.reuse_state",
            "Prebuild reuse",
            Some(exact_target.prebuild_reuse_state_token.clone()),
            Some(current_target.prebuild_reuse_state_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.compatibility_fingerprint",
            "Prebuild compatibility fingerprint",
            Some(exact_target.prebuild_compatibility_fingerprint.clone()),
            Some(current_target.prebuild_compatibility_fingerprint.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.invalidation_reason",
            "Prebuild invalidation",
            exact_target.prebuild_invalidation_reason_token.clone(),
            current_target.prebuild_invalidation_reason_token.clone(),
            true,
        );
        push_diff(
            &mut diff_rows,
            "mixed_version_drift.state",
            "Mixed-version state",
            Some(exact_target.mixed_version_state_token.clone()),
            Some(current_target.mixed_version_state_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "mixed_version_drift.reason",
            "Mixed-version reason",
            Some(exact_target.mixed_version_reason_token.clone()),
            Some(current_target.mixed_version_reason_token.clone()),
            true,
        );
        push_diff(
            &mut diff_rows,
            "degraded_fields",
            "Degraded field reasons",
            Some(join_tokens(&exact_target.degraded_field_tokens)),
            Some(join_tokens(&current_target.degraded_field_tokens)),
            false,
        );

        let target_or_boundary_changed = diff_rows.iter().any(|row| row.affects_target_boundary);
        let requires_review_before_dispatch = diff_rows
            .iter()
            .any(|row| row.requires_review_before_dispatch);
        let summary_headline = if diff_rows.is_empty() {
            format!(
                "Exact prior target matches current target: {}",
                current_target.canonical_target_id
            )
        } else {
            format!(
                "Rerun target drift: {} -> {} ({} changed fields)",
                exact_target.canonical_target_id,
                current_target.canonical_target_id,
                diff_rows.len()
            )
        };

        Self {
            record_kind: RERUN_TARGET_COMPARISON_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            exact_context_ref: exact_target.execution_context_ref.clone(),
            current_context_ref: current_target.execution_context_ref.clone(),
            exact_target,
            current_target: current_target.clone(),
            diff_rows,
            has_drift: requires_review_before_dispatch,
            target_or_boundary_changed,
            current_target_reachable: current_target.target_is_reachable(),
            requires_review_before_dispatch,
            summary_headline,
        }
    }
}

/// Prepared rerun-last attempt for one command invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunPreparedAttempt {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Command binding used to reach this rerun path.
    pub command: RerunCommandBinding,
    /// Selected rerun target mode.
    pub selected_target_mode: RerunTargetMode,
    /// Stable selected mode token.
    pub selected_target_mode_token: String,
    /// Timestamp supplied by the caller.
    pub prepared_at: String,
    /// Dispatch state after comparison.
    pub dispatch_state: RerunDispatchState,
    /// Stable dispatch-state token.
    pub dispatch_state_token: String,
    /// Unavailable reason when no attempt can be prepared.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<RerunUnavailableReason>,
    /// Stable unavailable-reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason_token: Option<String>,
    /// Prior attempt summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_attempt: Option<RerunAttemptSummary>,
    /// Prepared next attempt summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_attempt: Option<RerunAttemptSummary>,
    /// Exact-prior versus current target comparison.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<RerunTargetComparison>,
    /// Prepared run contract payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prepared_contract: Option<RerunRunContract>,
    /// True when dispatch must wait for a visible review.
    pub requires_review_before_dispatch: bool,
    /// True when the rerun command has a keyboard route.
    pub keyboard_reachable: bool,
}

impl RerunPreparedAttempt {
    /// Builds an unavailable rerun preview for an empty lane.
    pub fn unavailable(lane: RerunLane, observed_at: impl Into<String>) -> Self {
        Self::unavailable_with_mode(lane, RerunTargetMode::ExactPriorTarget, observed_at)
    }

    /// Builds an unavailable rerun preview while preserving the selected mode.
    pub fn unavailable_with_mode(
        lane: RerunLane,
        mode: RerunTargetMode,
        observed_at: impl Into<String>,
    ) -> Self {
        let reason = lane.no_prior_reason();
        let dispatch_state = RerunDispatchState::Unavailable;
        let command = RerunCommandBinding::for_lane(lane);
        Self {
            record_kind: RERUN_PREPARED_ATTEMPT_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            command,
            selected_target_mode: mode,
            selected_target_mode_token: mode.as_str().to_owned(),
            prepared_at: observed_at.into(),
            dispatch_state,
            dispatch_state_token: dispatch_state.as_str().to_owned(),
            unavailable_reason: Some(reason),
            unavailable_reason_token: Some(reason.as_str().to_owned()),
            prior_attempt: None,
            next_attempt: None,
            comparison: None,
            prepared_contract: None,
            requires_review_before_dispatch: false,
            keyboard_reachable: true,
        }
    }

    /// Builds a prepared rerun attempt from a remembered launch.
    pub fn prepare(
        launch: &RerunLastLaunch,
        current_context: &ExecutionContext,
        mode: RerunTargetMode,
        observed_at: impl Into<String>,
    ) -> Self {
        let prepared_at = observed_at.into();
        let comparison = RerunTargetComparison::compare(&launch.exact_context, current_context);
        let prepared_contract =
            launch
                .contract
                .rerun_with_context(current_context, mode, &prepared_at);
        let requires_review_before_dispatch = comparison.requires_review_before_dispatch
            || !launch.exact_context_matches_contract
            || current_context.invocation_subject.surface != launch.lane.expected_surface();
        let dispatch_state = if requires_review_before_dispatch {
            RerunDispatchState::ReviewRequired
        } else {
            RerunDispatchState::Ready
        };
        let command = RerunCommandBinding::for_lane(launch.lane);
        Self {
            record_kind: RERUN_PREPARED_ATTEMPT_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            command: command.clone(),
            selected_target_mode: mode,
            selected_target_mode_token: mode.as_str().to_owned(),
            prepared_at,
            dispatch_state,
            dispatch_state_token: dispatch_state.as_str().to_owned(),
            unavailable_reason: None,
            unavailable_reason_token: None,
            prior_attempt: Some(launch.prior_attempt.clone()),
            next_attempt: Some(RerunAttemptSummary::project(&prepared_contract)),
            comparison: Some(comparison),
            prepared_contract: Some(prepared_contract),
            requires_review_before_dispatch,
            keyboard_reachable: command.keyboard_reachable,
        }
    }

    /// Builds the task-event stream for the prepared attempt when available.
    pub fn launch_event_stream(
        &self,
        observed_at: &str,
    ) -> Option<Result<TaskEventStream, TaskEventStreamError>> {
        self.prepared_contract
            .as_ref()
            .map(|contract| contract.launch_event_stream(observed_at))
    }

    /// Renders a compact support/export line.
    pub fn support_line(&self) -> String {
        match (&self.prior_attempt, &self.next_attempt, &self.comparison) {
            (Some(prior), Some(next), Some(comparison)) => format!(
                "{} mode={} state={} prior={} next={} drift={} review={} target={}->{}",
                self.command.command_id,
                self.selected_target_mode_token,
                self.dispatch_state_token,
                prior.attempt_id,
                next.attempt_id,
                comparison.has_drift,
                self.requires_review_before_dispatch,
                comparison.exact_target.canonical_target_id,
                comparison.current_target.canonical_target_id,
            ),
            _ => format!(
                "{} state={} reason={}",
                self.command.command_id,
                self.dispatch_state_token,
                self.unavailable_reason_token.as_deref().unwrap_or("none")
            ),
        }
    }
}

/// In-memory last-task and last-test rerun loop.
#[derive(Debug, Clone, Default)]
pub struct RerunLastLoop {
    last_task: Option<RerunLastLaunch>,
    last_test: Option<RerunLastLaunch>,
}

impl RerunLastLoop {
    /// Creates an empty rerun-last loop.
    pub const fn new() -> Self {
        Self {
            last_task: None,
            last_test: None,
        }
    }

    /// Remembers a package-script attempt in the task or test lane.
    pub fn remember_package_script(
        &mut self,
        contract: PackageScriptRunContract,
        exact_context: ExecutionContext,
        remembered_at: impl Into<String>,
    ) -> RerunLastLaunch {
        let launch = RerunLastLaunch::from_package_script(contract, exact_context, remembered_at);
        self.remember(launch.clone());
        launch
    }

    /// Remembers a pytest attempt in the test lane.
    pub fn remember_pytest(
        &mut self,
        contract: PytestRunContract,
        exact_context: ExecutionContext,
        remembered_at: impl Into<String>,
    ) -> RerunLastLaunch {
        let launch = RerunLastLaunch::from_pytest(contract, exact_context, remembered_at);
        self.remember(launch.clone());
        launch
    }

    /// Remembers an already projected last-launch record.
    pub fn remember(&mut self, launch: RerunLastLaunch) {
        match launch.lane {
            RerunLane::Task => self.last_task = Some(launch),
            RerunLane::Test => self.last_test = Some(launch),
        }
    }

    /// Returns the last remembered launch for one lane.
    pub fn last_launch(&self, lane: RerunLane) -> Option<&RerunLastLaunch> {
        match lane {
            RerunLane::Task => self.last_task.as_ref(),
            RerunLane::Test => self.last_test.as_ref(),
        }
    }

    /// Prepares a rerun-last-task attempt.
    pub fn prepare_last_task(
        &self,
        current_context: &ExecutionContext,
        mode: RerunTargetMode,
        observed_at: impl Into<String>,
    ) -> RerunPreparedAttempt {
        self.prepare(RerunLane::Task, current_context, mode, observed_at)
    }

    /// Prepares a rerun-last-test attempt.
    pub fn prepare_last_test(
        &self,
        current_context: &ExecutionContext,
        mode: RerunTargetMode,
        observed_at: impl Into<String>,
    ) -> RerunPreparedAttempt {
        self.prepare(RerunLane::Test, current_context, mode, observed_at)
    }

    /// Prepares a rerun-last attempt for one lane.
    pub fn prepare(
        &self,
        lane: RerunLane,
        current_context: &ExecutionContext,
        mode: RerunTargetMode,
        observed_at: impl Into<String>,
    ) -> RerunPreparedAttempt {
        match self.last_launch(lane) {
            Some(launch) => {
                RerunPreparedAttempt::prepare(launch, current_context, mode, observed_at)
            }
            None => RerunPreparedAttempt::unavailable_with_mode(lane, mode, observed_at),
        }
    }

    /// Builds a support/export snapshot for task and test rerun state.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        current_task_context: Option<&ExecutionContext>,
        current_test_context: Option<&ExecutionContext>,
    ) -> RerunSupportExport {
        let generated_at = generated_at.into();
        let mut prepared_attempts = Vec::new();
        if let Some(context) = current_task_context {
            prepared_attempts.push(self.prepare_last_task(
                context,
                RerunTargetMode::ExactPriorTarget,
                generated_at.clone(),
            ));
        } else if self.last_task.is_none() {
            prepared_attempts.push(RerunPreparedAttempt::unavailable(
                RerunLane::Task,
                generated_at.clone(),
            ));
        }
        if let Some(context) = current_test_context {
            prepared_attempts.push(self.prepare_last_test(
                context,
                RerunTargetMode::ExactPriorTarget,
                generated_at.clone(),
            ));
        } else if self.last_test.is_none() {
            prepared_attempts.push(RerunPreparedAttempt::unavailable(
                RerunLane::Test,
                generated_at.clone(),
            ));
        }
        RerunSupportExport {
            record_kind: RERUN_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: RERUN_LOOP_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at,
            command_bindings: built_in_rerun_command_bindings().to_vec(),
            prepared_attempts,
        }
    }
}

/// Support/export projection for rerun-last state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RerunSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Support/export id supplied by the caller.
    pub export_id: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Keyboard command bindings available for rerun-last lanes.
    pub command_bindings: Vec<RerunCommandBinding>,
    /// Prepared rerun attempts included in the export.
    pub prepared_attempts: Vec<RerunPreparedAttempt>,
}

impl RerunSupportExport {
    /// Renders stable plaintext lines for support exports.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Rerun support export: {}\n", self.export_id);
        for binding in &self.command_bindings {
            out.push_str(&format!(
                "Command: {} keyboard={}\n",
                binding.command_id, binding.keyboard_reachable
            ));
        }
        for prepared in &self.prepared_attempts {
            out.push_str(&prepared.support_line());
            out.push('\n');
        }
        out
    }
}

fn lane_for_wedge(wedge: TaskWedgeClass) -> RerunLane {
    if wedge == TaskWedgeClass::Test {
        RerunLane::Test
    } else {
        RerunLane::Task
    }
}

fn push_diff(
    rows: &mut Vec<RerunDiffRow>,
    field_path: &str,
    label: &str,
    exact_value_token: Option<String>,
    current_value_token: Option<String>,
    affects_target_boundary: bool,
) {
    let diff_class = match (&exact_value_token, &current_value_token) {
        (Some(exact), Some(current)) if exact == current => return,
        (Some(_), Some(_)) => RerunDiffClass::Changed,
        (None, Some(_)) => RerunDiffClass::MissingInExact,
        (Some(_), None) => RerunDiffClass::MissingInCurrent,
        (None, None) => return,
    };
    rows.push(RerunDiffRow {
        field_path: field_path.to_owned(),
        label: label.to_owned(),
        exact_value_token,
        current_value_token,
        diff_class,
        diff_class_token: diff_class.as_str().to_owned(),
        affects_target_boundary,
        requires_review_before_dispatch: true,
    });
}

fn join_tokens(tokens: &[String]) -> String {
    if tokens.is_empty() {
        "none".to_owned()
    } else {
        tokens.join("|")
    }
}

const fn trust_token(state: TrustState) -> &'static str {
    match state {
        TrustState::Trusted => "trusted",
        TrustState::Restricted => "restricted",
        TrustState::PendingEvaluation => "pending_evaluation",
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use serde::Deserialize;

    use super::*;
    use crate::detectors::node::NodeToolchainDetectorConfig;
    use crate::detectors::python::PythonEnvironmentDetectorConfig;
    use crate::discovery::package_scripts::{
        PackageScriptDiscoverer, PackageScriptDiscovererConfig,
    };
    use crate::discovery::pytest::{PytestDiscoverer, PytestDiscovererConfig};
    use crate::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode,
    };

    fn runtime_fixture_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/runtime")
    }

    fn rerun_fixture(name: &str) -> RerunCaseFixture {
        let path = runtime_fixture_root()
            .join("rerun_exact_vs_current")
            .join(name);
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        serde_json::from_str(&payload).expect("fixture must parse")
    }

    fn baseline_resolver(workspace_id: &str, cwd: &str) -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: workspace_id.to_owned(),
            profile_id: Some("profile:default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some(cwd.to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: format!("caps:{workspace_id}"),
                capsule_hash: format!("sha256:{workspace_id}"),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "rerun-test-resolver".to_owned(),
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

    fn package_discoverer() -> PackageScriptDiscoverer {
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

    fn pytest_discoverer() -> PytestDiscoverer {
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
    fn rerun_last_task_discloses_current_target_drift_before_dispatch() {
        let fixture = rerun_fixture("last_task_current_target_drift.json");
        let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
        let exact_context = package_script_context(&mut resolver, "2026-05-13T18:00:00Z");
        let discovery = package_discoverer().discover_workspace(
            &runtime_fixture_root().join("tsjs_task_discovery_alpha/ready_pnpm"),
            exact_context.clone(),
            "2026-05-13T18:00:01Z",
        );
        let contract = discovery
            .contract_for_script("build")
            .expect("build contract")
            .clone();

        let mut loop_state = RerunLastLoop::new();
        loop_state.remember_package_script(contract.clone(), exact_context, "2026-05-13T18:00:02Z");

        let mut current_request = ExecutionContextRequest::package_script_task_seed(
            "task.run.package_script",
            TrustState::Restricted,
            "2026-05-13T18:00:03Z",
        );
        current_request.override_target_class = Some(TargetClass::ManagedWorkspace);
        current_request.override_working_directory = Some("/srv/web");
        let current_context = resolver.resolve(current_request);

        let prepared = loop_state.prepare_last_task(
            &current_context,
            RerunTargetMode::CurrentResolvedTarget,
            "2026-05-13T18:00:04Z",
        );

        assert_eq!(prepared.command.command_id, fixture.expect.command_id);
        assert!(prepared.keyboard_reachable);
        assert_eq!(prepared.dispatch_state, RerunDispatchState::ReviewRequired);
        assert!(prepared.requires_review_before_dispatch);
        assert_eq!(
            prepared.selected_target_mode_token,
            fixture.expect.selected_target_mode
        );
        let comparison = prepared.comparison.as_ref().expect("comparison");
        assert!(comparison.has_drift);
        assert!(comparison.target_or_boundary_changed);
        for field in &fixture.expect.required_diff_fields {
            assert!(
                comparison
                    .diff_rows
                    .iter()
                    .any(|row| row.field_path == *field),
                "missing diff field {field}"
            );
        }

        let next = prepared.next_attempt.as_ref().expect("next attempt");
        assert_eq!(next.run_id, contract.run_id);
        assert_eq!(next.attempt_number, 2);
        assert_eq!(
            next.target_id,
            current_context.target_identity.canonical_target_id
        );

        let stream = prepared
            .launch_event_stream("2026-05-13T18:00:05Z")
            .expect("prepared contract")
            .expect("stream");
        assert_eq!(stream.events[0].identity.attempt_id, next.attempt_id);
        assert_eq!(
            stream.events[0].identity.execution_context_id,
            current_context.execution_context_id
        );
    }

    #[test]
    fn rerun_last_test_can_prepare_exact_prior_target_with_visible_drift() {
        let fixture = rerun_fixture("last_test_exact_prior_target.json");
        let mut resolver = baseline_resolver("workspace:python", "/workspace/python");
        let exact_context = pytest_context(&mut resolver, "2026-05-13T18:10:00Z");
        let discovery = pytest_discoverer().discover_workspace(
            &runtime_fixture_root().join("python_task_discovery_alpha/ready_uv"),
            exact_context.clone(),
            "2026-05-13T18:10:01Z",
        );
        let contract = discovery
            .contract_for_selector("tests/test_api.py::test_health")
            .expect("test contract")
            .clone();

        let mut loop_state = RerunLastLoop::new();
        loop_state.remember_pytest(contract.clone(), exact_context, "2026-05-13T18:10:02Z");

        let mut current_request = ExecutionContextRequest::test_seed(
            "test.run.pytest",
            TrustState::Trusted,
            "2026-05-13T18:10:03Z",
        );
        current_request.override_target_class = Some(TargetClass::SshRemote);
        current_request.override_working_directory = Some("/srv/python");
        let current_context = resolver.resolve(current_request);

        let prepared = loop_state.prepare_last_test(
            &current_context,
            RerunTargetMode::ExactPriorTarget,
            "2026-05-13T18:10:04Z",
        );

        assert_eq!(prepared.command.command_id, fixture.expect.command_id);
        assert_eq!(
            prepared.selected_target_mode_token,
            fixture.expect.selected_target_mode
        );
        assert_eq!(prepared.dispatch_state, RerunDispatchState::ReviewRequired);
        assert!(prepared.keyboard_reachable);
        let comparison = prepared.comparison.as_ref().expect("comparison");
        assert!(comparison.has_drift);
        assert_eq!(
            comparison.current_target.canonical_target_id,
            current_context.target_identity.canonical_target_id
        );
        for field in &fixture.expect.required_diff_fields {
            assert!(
                comparison
                    .diff_rows
                    .iter()
                    .any(|row| row.field_path == *field),
                "missing diff field {field}"
            );
        }

        let next = prepared.next_attempt.as_ref().expect("next attempt");
        assert_eq!(next.run_id, contract.run_id);
        assert_eq!(next.attempt_number, 2);
        assert_eq!(next.target_id, contract.target_id);
        assert_eq!(next.execution_context_ref, contract.execution_context_ref);
    }

    #[test]
    fn no_prior_test_reports_unavailable_but_keeps_keyboard_command_visible() {
        let mut resolver = baseline_resolver("workspace:python", "/workspace/python");
        let current_context = pytest_context(&mut resolver, "2026-05-13T18:20:00Z");
        let loop_state = RerunLastLoop::new();

        let prepared = loop_state.prepare_last_test(
            &current_context,
            RerunTargetMode::ExactPriorTarget,
            "2026-05-13T18:20:01Z",
        );

        assert_eq!(prepared.dispatch_state, RerunDispatchState::Unavailable);
        assert_eq!(
            prepared.unavailable_reason,
            Some(RerunUnavailableReason::NoPriorTest)
        );
        assert_eq!(prepared.command.command_id, RERUN_LAST_TEST_COMMAND_ID);
        assert!(prepared.keyboard_reachable);
        assert!(prepared.prepared_contract.is_none());
    }

    #[test]
    fn support_export_includes_bindings_and_drift_lines() {
        let mut resolver = baseline_resolver("workspace:web", "/workspace/web");
        let exact_context = package_script_context(&mut resolver, "2026-05-13T18:30:00Z");
        let discovery = package_discoverer().discover_workspace(
            &runtime_fixture_root().join("tsjs_task_discovery_alpha/ready_pnpm"),
            exact_context.clone(),
            "2026-05-13T18:30:01Z",
        );
        let contract = discovery
            .contract_for_script("build")
            .expect("build contract")
            .clone();
        let mut loop_state = RerunLastLoop::new();
        loop_state.remember_package_script(contract, exact_context, "2026-05-13T18:30:02Z");

        let current_context = package_script_context(&mut resolver, "2026-05-13T18:30:03Z");
        let export = loop_state.support_export(
            "support:rerun:01",
            "2026-05-13T18:30:04Z",
            Some(&current_context),
            None,
        );

        assert_eq!(export.record_kind, RERUN_SUPPORT_EXPORT_RECORD_KIND);
        assert_eq!(export.command_bindings.len(), 2);
        assert!(export
            .command_bindings
            .iter()
            .any(|binding| binding.command_id == RERUN_LAST_TASK_COMMAND_ID));
        let rendered = export.render_plaintext();
        assert!(rendered.contains(RERUN_LAST_TASK_COMMAND_ID));
        assert!(rendered.contains(RERUN_LAST_TEST_COMMAND_ID));
    }

    #[derive(Debug, Deserialize)]
    struct RerunCaseFixture {
        expect: RerunCaseExpect,
    }

    #[derive(Debug, Deserialize)]
    struct RerunCaseExpect {
        command_id: String,
        selected_target_mode: String,
        required_diff_fields: Vec<String>,
    }
}
