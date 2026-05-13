use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticBusSnapshot, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticProviderAvailabilityRow, DiagnosticSeverityClass,
    DiagnosticSourceFamily, DiagnosticSurfaceClass, DiagnosticSurfaceProjection,
};
use crate::lsp_router::{
    CompletenessClass, FaultDomainId, FreshnessClass, HealthState, LocalityClass, ProviderKind,
    RedactionClass, ScopeClaimClass, ScopeLimitClass, SupportClass,
};

use super::super::records::{
    PythonInterpreterReadinessClass, PythonInterpreterSelectionStateClass, PythonWorkspaceContext,
};

/// Integer schema version for Python quality alpha records.
pub type PythonQualityAlphaSchemaVersion = u32;

/// Schema version used by Python quality alpha records.
pub const PYTHON_QUALITY_ALPHA_SCHEMA_VERSION: PythonQualityAlphaSchemaVersion = 1;

/// Quality-tool lane covered by the Python launch wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualityToolKindClass {
    /// Formatter adapter such as Black or Ruff format.
    Formatter,
    /// Linter adapter such as Ruff.
    Linter,
    /// Test adapter such as a pytest adapter.
    TestAdapter,
}

impl PythonQualityToolKindClass {
    /// Returns the diagnostic source family this quality-tool lane emits.
    pub const fn source_family(self) -> DiagnosticSourceFamily {
        match self {
            Self::Formatter | Self::Linter => DiagnosticSourceFamily::LinterFormatterStyle,
            Self::TestAdapter => DiagnosticSourceFamily::RuntimeTestOrDebug,
        }
    }

    /// Returns the router provider kind for this quality-tool lane.
    pub const fn provider_kind(self) -> ProviderKind {
        match self {
            Self::Formatter => ProviderKind::FormatterAdapter,
            Self::Linter => ProviderKind::LinterAdapter,
            Self::TestAdapter => ProviderKind::TestAdapter,
        }
    }
}

/// Trigger that admitted a formatter, linter, or test-adapter quality flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualityTriggerClass {
    /// Explicit user command.
    ManualCommand,
    /// Save participant or format-on-save path.
    OnSave,
    /// Test discovery request.
    TestDiscovery,
    /// Test run request.
    TestRun,
    /// Rerun requested from execution-plane state.
    ExecutionPlaneRerun,
}

/// Normalized quality action attached to a task hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualityActionClass {
    /// Format the current document.
    FormatDocument,
    /// Format an explicit range when a formatter can safely support it.
    FormatRange,
    /// Run lint checks without applying edits.
    LintCheck,
    /// Build a lint autofix preview.
    LintAutofixPreview,
    /// Discover tests for the current Python scope.
    TestDiscovery,
    /// Run tests for the current Python scope.
    TestRun,
}

/// Fix-safety label for a quality action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualitySafetyClass {
    /// Whitespace, trivia, or formatting-only change.
    TriviaSafe,
    /// Single-file syntax-safe change.
    LocalSyntaxSafe,
    /// Single-file semantic change.
    SemanticLocal,
    /// Cross-file semantic change.
    CrossFileSemantic,
    /// Generated or protected source is involved.
    GeneratedOrProtected,
    /// Adapter cannot prove the mutation scope.
    UnknownOrUnstable,
    /// The action is read-only and does not mutate source.
    ReadOnlyNoMutation,
}

/// Preview posture required before a quality action may mutate source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualityPreviewRequirementClass {
    /// No preview is required.
    NotRequired,
    /// Inline summary is enough.
    InlineSummary,
    /// Structured diff is required.
    StructuredDiff,
    /// Batch scope preview is required.
    BatchScopePreview,
    /// Generated or protected path review is required.
    GeneratedOrProtectedPreviewRequired,
    /// Manual review is required because safety cannot be proven.
    ManualReviewRequired,
}

impl PythonQualityPreviewRequirementClass {
    /// Returns true when rerun must route through review before apply.
    pub const fn requires_preview(self) -> bool {
        !matches!(self, Self::NotRequired)
    }
}

/// Rerun posture exposed by execution-plane hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonQualityRerunPostureClass {
    /// The execution plane may rerun this hook directly.
    RerunnableFromExecutionPlane,
    /// The execution plane may rerun after preview or scope review.
    RerunnableAfterPreviewReview,
    /// Python interpreter selection or readiness blocks this hook.
    BlockedInterpreterUnavailable,
    /// The tool is missing, unavailable, or lacks the requested capability.
    BlockedToolUnavailable,
    /// Policy blocks this hook.
    BlockedByPolicy,
    /// The hook preserves evidence but cannot rerun the tool.
    InspectOnlyEvidence,
}

impl PythonQualityRerunPostureClass {
    /// Returns true when execution-plane surfaces may offer a rerun action.
    pub const fn is_runnable(self) -> bool {
        matches!(
            self,
            Self::RerunnableFromExecutionPlane | Self::RerunnableAfterPreviewReview
        )
    }

    /// Returns true when a rerun first needs preview or scope review.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::RerunnableAfterPreviewReview)
    }
}

/// Provider state for one Python quality tool.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualityToolStatusRow {
    /// Quality-tool lane.
    pub tool_kind_class: PythonQualityToolKindClass,
    /// Provider id used by diagnostics and task hooks.
    pub provider_id: String,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Provider support posture.
    pub support_class: SupportClass,
    /// Provider health.
    pub health_state: HealthState,
    /// Provider freshness.
    pub freshness_class: FreshnessClass,
    /// Scope claimed by the provider.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits explaining partiality.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Locality where the tool ran or would run.
    pub locality_class: LocalityClass,
    /// Fault domain that owns restart or unavailable accounting.
    pub fault_domain_id: FaultDomainId,
    /// Opaque tool reference.
    pub tool_ref: String,
    /// Opaque tool version reference, when known.
    pub tool_version_ref: Option<String>,
    /// Effective quality-profile ref applied to this tool.
    pub effective_quality_profile_ref: String,
    /// Router or execution decision ref, when one admitted this tool state.
    pub router_decision_ref: Option<String>,
    /// Export-safe provider summary.
    pub summary: String,
}

impl PythonQualityToolStatusRow {
    /// Projects this tool state into the shared diagnostic-provider row.
    pub fn diagnostic_provider_row(&self) -> DiagnosticProviderAvailabilityRow {
        DiagnosticProviderAvailabilityRow {
            provider_id: self.provider_id.clone(),
            provider_display_label: self.provider_display_label.clone(),
            source_family: self.tool_kind_class.source_family(),
            provider_kind: self.tool_kind_class.provider_kind(),
            support_class: self.support_class,
            health_state: self.health_state,
            freshness_class: self.freshness_class,
            scope_claim_class: self.scope_claim_class,
            completeness_class: self.completeness_class,
            scope_limit_classes: self.scope_limit_classes.clone(),
            locality_class: self.locality_class,
            fault_domain_id: self.fault_domain_id,
            restart_strike_count: 0,
            quarantine_ref: None,
            router_decision_ref: self.router_decision_ref.clone(),
            summary: self.summary.clone(),
        }
    }

    /// Returns true when the tool is unavailable for its claimed quality lane.
    pub const fn is_unavailable(&self) -> bool {
        matches!(
            self.health_state,
            HealthState::PolicyBlocked
                | HealthState::CapabilityMissing
                | HealthState::CrashLoopQuarantined
                | HealthState::Unavailable
        ) || matches!(
            self.completeness_class,
            CompletenessClass::UnavailableForClaimedScope
        ) || matches!(self.support_class, SupportClass::Unsupported)
    }

    /// Returns true when downstream surfaces must disclose degraded state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.health_state.requires_disclosure()
            || self.freshness_class != FreshnessClass::AuthoritativeLive
            || self.completeness_class != CompletenessClass::CompleteForClaimedScope
            || !self.scope_limit_classes.is_empty()
            || matches!(
                self.support_class,
                SupportClass::FallbackOnly | SupportClass::InspectOnly | SupportClass::Unsupported
            )
    }
}

/// Fixture seed for one normalized Python quality diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualityDiagnosticSeed {
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Stable source descriptor id.
    pub source_descriptor_id: String,
    /// Quality-tool lane that produced or preserved the finding.
    pub tool_kind_class: PythonQualityToolKindClass,
    /// Diagnostic source family.
    pub source_family: DiagnosticSourceFamily,
    /// Plane of evidence behind the finding.
    pub evidence_plane_class: DiagnosticEvidencePlaneClass,
    /// Origin of the evidence copy.
    pub origin_class: DiagnosticOriginClass,
    /// Opaque producer or tool reference.
    pub producer_ref: String,
    /// Opaque producer version reference, when known.
    pub producer_version_ref: Option<String>,
    /// Provider id linked to the finding.
    pub provider_id: Option<String>,
    /// Provider support posture.
    pub support_class: SupportClass,
    /// Evidence locality.
    pub locality_class: LocalityClass,
    /// Normalized severity.
    pub severity_class: DiagnosticSeverityClass,
    /// Opaque rule id reference, when available.
    pub rule_id_ref: Option<String>,
    /// Opaque category reference, when available.
    pub category_ref: Option<String>,
    /// Freshness class.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Source or current epoch reference.
    pub epoch_ref: Option<String>,
    /// Invalidation or stale reason reference, when known.
    pub invalidation_ref: Option<String>,
    /// Scope claimed by the finding.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Target, workset, run, or file scope reference.
    pub target_ref: String,
    /// Stable anchor family id.
    pub anchor_family_id: String,
    /// Current anchor reference, when safe to show.
    pub current_anchor_ref: Option<String>,
    /// Opaque path or structured object reference, when known.
    pub path_ref: Option<String>,
    /// Current anchor remap state.
    pub remap_state_class: crate::diagnostics::DiagnosticAnchorRemapStateClass,
    /// Export-safe source summary.
    pub source_summary: String,
    /// Export-safe freshness summary.
    pub freshness_summary: String,
    /// Export-safe scope summary.
    pub scope_summary: String,
    /// Export-safe anchor summary.
    pub anchor_summary: String,
    /// Export-safe diagnostic summary.
    pub diagnostic_summary: String,
}

/// Fixture seed for one execution-plane quality task hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualityTaskHookSeed {
    /// Stable task hook id.
    pub task_hook_id: String,
    /// Quality-tool lane this hook reruns.
    pub tool_kind_class: PythonQualityToolKindClass,
    /// Quality action this hook executes.
    pub action_class: PythonQualityActionClass,
    /// Trigger that admitted the hook.
    pub trigger_class: PythonQualityTriggerClass,
    /// Provider id used to resolve current health.
    pub provider_id: String,
    /// Stable command id.
    pub canonical_command_id: String,
    /// Stable command verb.
    pub canonical_verb: String,
    /// Target, workset, run, or file scope ref.
    pub target_ref: String,
    /// Build or test target id, when known.
    pub build_target_id: Option<String>,
    /// Task-event trace ref, when a trace exists.
    pub task_event_trace_ref: Option<String>,
    /// Normalized task-event refs emitted or expected from this hook.
    pub normalized_task_event_refs: Vec<String>,
    /// Diagnostic refs this hook can refresh.
    pub source_diagnostic_refs: Vec<String>,
    /// Preview requirement for this hook.
    pub preview_requirement_class: PythonQualityPreviewRequirementClass,
    /// Fix-safety label for this hook.
    pub safety_class: PythonQualitySafetyClass,
    /// Export-safe hook summary.
    pub summary: String,
}

/// Execution-plane hook exposed by Python quality flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualityExecutionTaskHook {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonQualityAlphaSchemaVersion,
    /// Stable task hook id.
    pub task_hook_id: String,
    /// Quality-tool lane this hook reruns.
    pub tool_kind_class: PythonQualityToolKindClass,
    /// Quality action this hook executes.
    pub action_class: PythonQualityActionClass,
    /// Trigger that admitted the hook.
    pub trigger_class: PythonQualityTriggerClass,
    /// Provider id used to resolve current health.
    pub provider_id: String,
    /// Stable command id.
    pub canonical_command_id: String,
    /// Stable command verb.
    pub canonical_verb: String,
    /// Execution-context id anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Python interpreter ref used to admit the hook.
    pub interpreter_ref: String,
    /// Interpreter selection state used to admit the hook.
    pub interpreter_selection_state_class: PythonInterpreterSelectionStateClass,
    /// Interpreter readiness used to admit the hook.
    pub interpreter_readiness_class: PythonInterpreterReadinessClass,
    /// Target, workset, run, or file scope ref.
    pub target_ref: String,
    /// Build or test target id, when known.
    pub build_target_id: Option<String>,
    /// Diagnostic collection this hook can refresh.
    pub diagnostic_collection_id: String,
    /// Diagnostic refs this hook can refresh.
    pub source_diagnostic_refs: Vec<String>,
    /// Task-event trace ref, when a trace exists.
    pub task_event_trace_ref: Option<String>,
    /// Normalized task-event refs emitted or expected from this hook.
    pub normalized_task_event_refs: Vec<String>,
    /// Rerun posture exposed to execution-plane consumers.
    pub rerun_posture_class: PythonQualityRerunPostureClass,
    /// Preview requirement for this hook.
    pub preview_requirement_class: PythonQualityPreviewRequirementClass,
    /// Fix-safety label for this hook.
    pub safety_class: PythonQualitySafetyClass,
    /// Provider health at the time the hook was built.
    pub provider_health_class: HealthState,
    /// Provider freshness at the time the hook was built.
    pub provider_freshness_class: FreshnessClass,
    /// Provider support posture at the time the hook was built.
    pub support_class: SupportClass,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe hook summary.
    pub export_safe_summary: String,
}

impl PythonQualityExecutionTaskHook {
    /// Stable record-kind tag for quality task hooks.
    pub const RECORD_KIND: &'static str = "python_quality_execution_task_hook";

    /// Returns true when execution-plane surfaces may offer this rerun.
    pub const fn is_runnable(&self) -> bool {
        self.rerun_posture_class.is_runnable()
    }

    /// Returns true when rerun first needs preview or scope review.
    pub const fn requires_preview(&self) -> bool {
        self.rerun_posture_class.requires_preview()
    }
}

/// Fixture-backed Python quality seed snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualitySeedSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonQualityAlphaSchemaVersion,
    /// Requested language id.
    pub language_id: String,
    /// Workspace, interpreter, scope, and policy context.
    pub workspace_context: PythonWorkspaceContext,
    /// Effective quality-profile ref applied to all rows.
    pub effective_quality_profile_ref: String,
    /// Export-safe quality-profile summary.
    pub quality_profile_summary: String,
    /// Provider state for formatter, linter, and test-adapter lanes.
    pub tool_rows: Vec<PythonQualityToolStatusRow>,
    /// Diagnostic seeds emitted into the shared diagnostic bus.
    pub diagnostic_seeds: Vec<PythonQualityDiagnosticSeed>,
    /// Execution-plane hook seeds linked to quality actions.
    pub task_hook_seeds: Vec<PythonQualityTaskHookSeed>,
    /// Capture timestamp used by deterministic fixtures.
    pub captured_at: String,
    /// Export-safe summary for support and review.
    pub export_safe_summary: String,
}

impl PythonQualitySeedSnapshot {
    /// Stable record-kind tag for fixture-backed Python quality seeds.
    pub const RECORD_KIND: &'static str = "python_quality_alpha_seed_snapshot";

    /// Returns the tool row for the requested provider id.
    pub fn tool(&self, provider_id: &str) -> Option<&PythonQualityToolStatusRow> {
        self.tool_rows
            .iter()
            .find(|tool| tool.provider_id == provider_id)
    }
}

/// Request for a deterministic Python quality snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonQualitySnapshotRequest {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Diagnostic collection id for normalized findings.
    pub collection_id: String,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Aggregate counts used by Python quality consumers.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PythonQualityAggregateCounts {
    /// Normalized diagnostic count.
    pub normalized_diagnostic_count: usize,
    /// Quality-tool provider count.
    pub tool_count: usize,
    /// Tools requiring degraded disclosure.
    pub degraded_tool_count: usize,
    /// Tools unavailable for their lane.
    pub unavailable_tool_count: usize,
    /// Execution-plane task hook count.
    pub task_hook_count: usize,
    /// Hooks execution-plane surfaces may rerun.
    pub runnable_hook_count: usize,
    /// Hooks blocked from rerun.
    pub blocked_hook_count: usize,
    /// Hooks blocked because interpreter truth is unavailable.
    pub interpreter_blocked_hook_count: usize,
    /// Hooks that require preview before rerun or apply.
    pub preview_required_hook_count: usize,
}

impl PythonQualityAggregateCounts {
    /// Builds aggregate counts from diagnostics, tool rows, and task hooks.
    pub fn from_parts(
        diagnostics: &DiagnosticBusSnapshot,
        tools: &[PythonQualityToolStatusRow],
        hooks: &[PythonQualityExecutionTaskHook],
    ) -> Self {
        Self {
            normalized_diagnostic_count: diagnostics.aggregate_counts.total_count,
            tool_count: tools.len(),
            degraded_tool_count: tools
                .iter()
                .filter(|tool| tool.requires_degraded_disclosure())
                .count(),
            unavailable_tool_count: tools.iter().filter(|tool| tool.is_unavailable()).count(),
            task_hook_count: hooks.len(),
            runnable_hook_count: hooks.iter().filter(|hook| hook.is_runnable()).count(),
            blocked_hook_count: hooks.iter().filter(|hook| !hook.is_runnable()).count(),
            interpreter_blocked_hook_count: hooks
                .iter()
                .filter(|hook| {
                    hook.rerun_posture_class
                        == PythonQualityRerunPostureClass::BlockedInterpreterUnavailable
                })
                .count(),
            preview_required_hook_count: hooks
                .iter()
                .filter(|hook| hook.requires_preview())
                .count(),
        }
    }
}

/// Canonical Python quality snapshot emitted for consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualitySnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonQualityAlphaSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Requested language id.
    pub language_id: String,
    /// Workspace, interpreter, scope, and policy context.
    pub workspace_context: PythonWorkspaceContext,
    /// Effective quality-profile ref applied to all rows.
    pub effective_quality_profile_ref: String,
    /// Export-safe quality-profile summary.
    pub quality_profile_summary: String,
    /// Shared diagnostic bus snapshot containing normalized quality findings.
    pub diagnostic_bus_snapshot: DiagnosticBusSnapshot,
    /// Provider state for formatter, linter, and test-adapter lanes.
    pub tool_rows: Vec<PythonQualityToolStatusRow>,
    /// Execution-plane hooks linked to quality actions.
    pub task_hooks: Vec<PythonQualityExecutionTaskHook>,
    /// Aggregate counts for compact surfaces.
    pub aggregate_counts: PythonQualityAggregateCounts,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe snapshot summary.
    pub export_safe_summary: String,
}

impl PythonQualitySnapshot {
    /// Stable record-kind tag for Python quality snapshots.
    pub const RECORD_KIND: &'static str = "python_quality_alpha_snapshot";

    /// Returns true when quality consumers must show degraded labels.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.workspace_context
            .interpreter_context
            .requires_disclosure()
            || self.diagnostic_bus_snapshot.requires_degraded_disclosure()
            || self
                .tool_rows
                .iter()
                .any(PythonQualityToolStatusRow::requires_degraded_disclosure)
            || self.task_hooks.iter().any(|hook| !hook.is_runnable())
    }

    /// Builds the execution-plane projection that consumes diagnostic and hook truth together.
    pub fn execution_plane_projection(
        &self,
        surface_class: DiagnosticSurfaceClass,
        captured_at: impl Into<String>,
    ) -> PythonQualityExecutionPlaneProjection {
        let captured_at = captured_at.into();
        let diagnostic_projection = self
            .diagnostic_bus_snapshot
            .surface_projection(surface_class, captured_at.clone());
        let runnable_task_hook_ids = self
            .task_hooks
            .iter()
            .filter(|hook| hook.is_runnable())
            .map(|hook| hook.task_hook_id.clone())
            .collect::<Vec<_>>();
        let blocked_task_hook_ids = self
            .task_hooks
            .iter()
            .filter(|hook| !hook.is_runnable())
            .map(|hook| hook.task_hook_id.clone())
            .collect::<Vec<_>>();
        let preview_required_task_hook_ids = self
            .task_hooks
            .iter()
            .filter(|hook| hook.requires_preview())
            .map(|hook| hook.task_hook_id.clone())
            .collect::<Vec<_>>();
        let provider_availability_refs = self
            .tool_rows
            .iter()
            .map(|tool| tool.provider_id.clone())
            .collect::<Vec<_>>();

        PythonQualityExecutionPlaneProjection {
            record_kind: PythonQualityExecutionPlaneProjection::RECORD_KIND.into(),
            schema_version: PYTHON_QUALITY_ALPHA_SCHEMA_VERSION,
            projection_id: format!(
                "python:quality:execution-plane:{}",
                sanitize_id(&self.snapshot_id)
            ),
            snapshot_id: self.snapshot_id.clone(),
            surface_class,
            diagnostic_projection,
            runnable_task_hook_ids,
            blocked_task_hook_ids,
            preview_required_task_hook_ids,
            provider_availability_refs,
            execution_context_id: self.workspace_context.execution_context_id.clone(),
            interpreter_ref: self
                .workspace_context
                .interpreter_context
                .interpreter_ref
                .clone(),
            interpreter_selection_state_class: self
                .workspace_context
                .interpreter_context
                .selection_state_class,
            interpreter_readiness_class: self.workspace_context.interpreter_context.readiness_class,
            disclosure_required: self.requires_degraded_disclosure(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at,
            export_safe_summary: format!(
                "Python quality projection exposes {} diagnostics and {} runnable hooks.",
                self.aggregate_counts.normalized_diagnostic_count,
                self.aggregate_counts.runnable_hook_count
            ),
        }
    }
}

/// Execution-plane projection over quality diagnostics and rerun hooks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonQualityExecutionPlaneProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonQualityAlphaSchemaVersion,
    /// Stable projection id.
    pub projection_id: String,
    /// Source quality snapshot id.
    pub snapshot_id: String,
    /// Diagnostic surface consuming this projection.
    pub surface_class: DiagnosticSurfaceClass,
    /// Diagnostic bus projection for the same surface.
    pub diagnostic_projection: DiagnosticSurfaceProjection,
    /// Task hooks that may be rerun.
    pub runnable_task_hook_ids: Vec<String>,
    /// Task hooks blocked from rerun.
    pub blocked_task_hook_ids: Vec<String>,
    /// Task hooks requiring preview or scope review before rerun.
    pub preview_required_task_hook_ids: Vec<String>,
    /// Provider ids linked to this projection.
    pub provider_availability_refs: Vec<String>,
    /// Execution-context id all hooks bind to.
    pub execution_context_id: String,
    /// Python interpreter ref projected for execution-plane consumers.
    pub interpreter_ref: String,
    /// Interpreter selection state projected for execution-plane consumers.
    pub interpreter_selection_state_class: PythonInterpreterSelectionStateClass,
    /// Interpreter readiness projected for execution-plane consumers.
    pub interpreter_readiness_class: PythonInterpreterReadinessClass,
    /// Whether the projection must show degraded labels.
    pub disclosure_required: bool,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe projection summary.
    pub export_safe_summary: String,
}

impl PythonQualityExecutionPlaneProjection {
    /// Stable record-kind tag for Python quality execution-plane projections.
    pub const RECORD_KIND: &'static str = "python_quality_execution_plane_projection";
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
