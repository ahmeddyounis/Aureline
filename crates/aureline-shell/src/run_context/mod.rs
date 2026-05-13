//! Shared execution-context summary for launch-capable entry surfaces.
//!
//! Terminal, task, test, debug-prep, and AI-tool entry points all need the
//! same pre-dispatch answer: where work would run, which runtime would be
//! used, which resolver input won, whether prebuild or helper drift is in
//! play, and how an exact rerun differs from the current environment. This
//! module is a shell-side projection over
//! [`aureline_runtime::ExecutionContext`]. It owns no target, toolchain,
//! trust, or fallback truth of its own.

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    CacheDisposition, ConfidenceLevel, ExecutionContext, PrebuildInvalidationReason,
    PrebuildReuseState, ResolverInputField, ResolverInputSource, ScopeClass, SurfaceClass,
    TargetClass, ToolchainClass, TrustState,
};

/// Stable record-kind tag for [`RunContextSummary`] payloads.
pub const RUN_CONTEXT_SUMMARY_RECORD_KIND: &str = "run_context_summary_record";
/// Stable record-kind tag for [`RunContextComparison`] payloads.
pub const RUN_CONTEXT_COMPARISON_RECORD_KIND: &str = "run_context_comparison_record";
/// Stable record-kind tag for [`ExecutionEntrySurface`] payloads.
pub const EXECUTION_ENTRY_SURFACE_RECORD_KIND: &str = "execution_entry_surface_record";
/// Stable record-kind tag for [`ExecutionEntryTruthSnapshot`] payloads.
pub const EXECUTION_ENTRY_TRUTH_SNAPSHOT_RECORD_KIND: &str =
    "execution_entry_truth_snapshot_record";
/// Schema version for shell execution-entry truth payloads.
pub const RUN_CONTEXT_SCHEMA_VERSION: u32 = 1;

/// Launch-capable shell entry point that renders execution-context truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionEntryPoint {
    /// Bottom-panel terminal entry.
    Terminal,
    /// Task runner entry.
    Task,
    /// Test runner entry.
    Test,
    /// Debug preparation entry.
    DebugPrep,
    /// AI tool-call entry.
    AiTool,
}

impl ExecutionEntryPoint {
    /// Stable string token recorded on the entry surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::Task => "task",
            Self::Test => "test",
            Self::DebugPrep => "debug_prep",
            Self::AiTool => "ai_tool",
        }
    }

    /// Human-readable label for the entry surface.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Terminal => "Terminal",
            Self::Task => "Task",
            Self::Test => "Test",
            Self::DebugPrep => "Debug prep",
            Self::AiTool => "AI tool",
        }
    }

    /// Runtime surface class that should back this entry point.
    pub const fn expected_surface(self) -> SurfaceClass {
        match self {
            Self::Terminal => SurfaceClass::Terminal,
            Self::Task => SurfaceClass::Task,
            Self::Test => SurfaceClass::Test,
            Self::DebugPrep => SurfaceClass::Debug,
            Self::AiTool => SurfaceClass::AiToolCall,
        }
    }

    /// Map a runtime surface onto the corresponding shell entry point.
    pub const fn from_surface_class(surface: SurfaceClass) -> Option<Self> {
        match surface {
            SurfaceClass::Terminal => Some(Self::Terminal),
            SurfaceClass::Task => Some(Self::Task),
            SurfaceClass::Test => Some(Self::Test),
            SurfaceClass::Debug => Some(Self::DebugPrep),
            SurfaceClass::AiToolCall => Some(Self::AiTool),
            _ => None,
        }
    }
}

/// One resolver-input decision projected into an entry summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContextInputDecisionSummary {
    pub field: ResolverInputField,
    pub field_token: String,
    pub winning_source: ResolverInputSource,
    pub winning_source_token: String,
    pub resolved_value_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicting_source_tokens: Vec<String>,
}

/// Compact, export-safe summary every launch-capable entry surface renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContextSummary {
    pub record_kind: String,
    pub schema_version: u32,
    pub execution_context_ref: String,
    pub provenance_record_ref: String,
    pub resolver_version: String,
    pub workspace_id: String,
    pub surface: SurfaceClass,
    pub surface_token: String,
    pub target_class: TargetClass,
    pub target_class_token: String,
    pub canonical_target_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
    pub boundary_cue_visible: bool,
    pub target_confidence_level: ConfidenceLevel,
    pub target_confidence_level_token: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub target_confidence_reason_tokens: Vec<String>,
    pub toolchain_class: ToolchainClass,
    pub toolchain_class_token: String,
    pub toolchain_id: String,
    pub resolved_version: String,
    pub trust_state: TrustState,
    pub trust_state_token: String,
    pub identity_mode_token: String,
    pub policy_epoch: u64,
    pub scope_class: ScopeClass,
    pub scope_class_token: String,
    pub cache_disposition: CacheDisposition,
    pub cache_disposition_token: String,
    pub prebuild_reuse_state: PrebuildReuseState,
    pub prebuild_reuse_state_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_snapshot_ref: Option<String>,
    pub prebuild_compatibility_fingerprint: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_invalidation_reason: Option<PrebuildInvalidationReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prebuild_invalidation_reason_token: Option<String>,
    pub mixed_version_state_token: String,
    pub mixed_version_reason_token: String,
    pub client_protocol: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub helper_protocol: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub degraded_field_tokens: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub explanation_reason_code_tokens: Vec<String>,
    pub input_decisions: Vec<RunContextInputDecisionSummary>,
}

impl RunContextSummary {
    /// Project one summary from the canonical runtime context.
    pub fn project(context: &ExecutionContext) -> Self {
        Self {
            record_kind: RUN_CONTEXT_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: RUN_CONTEXT_SCHEMA_VERSION,
            execution_context_ref: context.execution_context_id.clone(),
            provenance_record_ref: context.provenance.provenance_record_id.clone(),
            resolver_version: context.provenance.resolver_version.clone(),
            workspace_id: context.invocation_subject.workspace_id.clone(),
            surface: context.invocation_subject.surface,
            surface_token: context.invocation_subject.surface.as_str().to_owned(),
            target_class: context.target_identity.target_class,
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            canonical_target_id: context.target_identity.canonical_target_id.clone(),
            working_directory: context.target_identity.working_directory.clone(),
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
            identity_mode_token: context.policy_and_trust.identity_mode.as_str().to_owned(),
            policy_epoch: context.policy_and_trust.policy_epoch,
            scope_class: context.workset_scope_class,
            scope_class_token: context.workset_scope_class.as_str().to_owned(),
            cache_disposition: context.cache_disposition,
            cache_disposition_token: context.cache_disposition.as_str().to_owned(),
            prebuild_reuse_state: context.prebuild_metadata.reuse_state,
            prebuild_reuse_state_token: context.prebuild_metadata.reuse_state.as_str().to_owned(),
            prebuild_snapshot_ref: context.prebuild_metadata.snapshot_ref.clone(),
            prebuild_compatibility_fingerprint: context
                .prebuild_metadata
                .compatibility_fingerprint
                .clone(),
            prebuild_invalidation_reason: context.prebuild_metadata.invalidation_reason,
            prebuild_invalidation_reason_token: context
                .prebuild_metadata
                .invalidation_reason
                .map(|reason| reason.as_str().to_owned()),
            mixed_version_state_token: context.mixed_version_drift.state.as_str().to_owned(),
            mixed_version_reason_token: context.mixed_version_drift.reason.as_str().to_owned(),
            client_protocol: context.mixed_version_drift.client_protocol.clone(),
            helper_protocol: context.mixed_version_drift.helper_protocol.clone(),
            degraded_field_tokens: context
                .degraded_fields
                .iter()
                .map(|field| field.reason.as_str().to_owned())
                .collect(),
            explanation_reason_code_tokens: context
                .explanations
                .iter()
                .map(|explanation| explanation.reason_code.as_str().to_owned())
                .collect(),
            input_decisions: context
                .provenance
                .input_decisions
                .iter()
                .map(|decision| RunContextInputDecisionSummary {
                    field: decision.field,
                    field_token: decision.field.as_str().to_owned(),
                    winning_source: decision.winning_source,
                    winning_source_token: decision.winning_source.as_str().to_owned(),
                    resolved_value_token: decision.resolved_value_token.clone(),
                    conflicting_source_tokens: decision
                        .conflicting_sources
                        .iter()
                        .map(|source| source.as_str().to_owned())
                        .collect(),
                })
                .collect(),
        }
    }

    /// True when the summary carries an unresolved or degraded field marker.
    pub fn has_degraded_fields(&self) -> bool {
        !self.degraded_field_tokens.is_empty()
    }

    /// Render a stable one-line summary suitable for copy and support export.
    pub fn summary_line(&self) -> String {
        format!(
            "surface={}; target={}; toolchain={}; trust={}; prebuild={}; helper={}; confidence={}",
            self.surface_token,
            self.target_class_token,
            self.toolchain_class_token,
            self.trust_state_token,
            self.prebuild_reuse_state_token,
            self.mixed_version_state_token,
            self.target_confidence_level_token,
        )
    }
}

/// Difference class for exact-rerun versus current-context comparisons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunContextDiffClass {
    /// Both contexts carry the same token for this field.
    Unchanged,
    /// Both contexts carry a token and the tokens differ.
    Changed,
    /// The exact-rerun context does not carry the field.
    MissingInExact,
    /// The current context does not carry the field.
    MissingInCurrent,
}

impl RunContextDiffClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::Changed => "changed",
            Self::MissingInExact => "missing_in_exact",
            Self::MissingInCurrent => "missing_in_current",
        }
    }
}

/// One visible difference between exact-rerun and current contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContextDiffRow {
    pub field_path: String,
    pub label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_value_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_value_token: Option<String>,
    pub diff_class: RunContextDiffClass,
    pub diff_class_token: String,
    pub requires_review_before_dispatch: bool,
}

/// Exact-rerun versus current-context comparison shown before dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunContextComparison {
    pub record_kind: String,
    pub schema_version: u32,
    pub exact_context_ref: String,
    pub current_context_ref: String,
    pub exact_summary: RunContextSummary,
    pub current_summary: RunContextSummary,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diff_rows: Vec<RunContextDiffRow>,
    pub has_drift: bool,
    pub requires_review_before_dispatch: bool,
}

impl RunContextComparison {
    /// Compare a prior exact-run context with the current environment.
    pub fn compare(exact: &ExecutionContext, current: &ExecutionContext) -> Self {
        let exact_summary = RunContextSummary::project(exact);
        let current_summary = RunContextSummary::project(current);
        Self::from_summaries(exact_summary, current_summary)
    }

    /// Compare two already-projected summaries.
    pub fn from_summaries(
        exact_summary: RunContextSummary,
        current_summary: RunContextSummary,
    ) -> Self {
        let mut diff_rows = Vec::new();
        push_diff(
            &mut diff_rows,
            "target_identity.target_class",
            "Target class",
            Some(exact_summary.target_class_token.clone()),
            Some(current_summary.target_class_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "target_identity.canonical_target_id",
            "Canonical target id",
            Some(exact_summary.canonical_target_id.clone()),
            Some(current_summary.canonical_target_id.clone()),
        );
        push_diff(
            &mut diff_rows,
            "target_identity.working_directory",
            "Working directory",
            exact_summary.working_directory.clone(),
            current_summary.working_directory.clone(),
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.toolchain_class",
            "Toolchain class",
            Some(exact_summary.toolchain_class_token.clone()),
            Some(current_summary.toolchain_class_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.toolchain_id",
            "Toolchain id",
            Some(exact_summary.toolchain_id.clone()),
            Some(current_summary.toolchain_id.clone()),
        );
        push_diff(
            &mut diff_rows,
            "toolchain_identity.resolved_version",
            "Resolved version",
            Some(exact_summary.resolved_version.clone()),
            Some(current_summary.resolved_version.clone()),
        );
        push_diff(
            &mut diff_rows,
            "policy_and_trust.trust_state",
            "Trust state",
            Some(exact_summary.trust_state_token.clone()),
            Some(current_summary.trust_state_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "policy_and_trust.policy_epoch",
            "Policy epoch",
            Some(exact_summary.policy_epoch.to_string()),
            Some(current_summary.policy_epoch.to_string()),
        );
        push_diff(
            &mut diff_rows,
            "workset_scope_class",
            "Workset scope",
            Some(exact_summary.scope_class_token.clone()),
            Some(current_summary.scope_class_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "cache_disposition",
            "Cache disposition",
            Some(exact_summary.cache_disposition_token.clone()),
            Some(current_summary.cache_disposition_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.reuse_state",
            "Prebuild reuse",
            Some(exact_summary.prebuild_reuse_state_token.clone()),
            Some(current_summary.prebuild_reuse_state_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.compatibility_fingerprint",
            "Prebuild compatibility fingerprint",
            Some(exact_summary.prebuild_compatibility_fingerprint.clone()),
            Some(current_summary.prebuild_compatibility_fingerprint.clone()),
        );
        push_diff(
            &mut diff_rows,
            "prebuild_metadata.invalidation_reason",
            "Prebuild invalidation",
            exact_summary.prebuild_invalidation_reason_token.clone(),
            current_summary.prebuild_invalidation_reason_token.clone(),
        );
        push_diff(
            &mut diff_rows,
            "mixed_version_drift.state",
            "Mixed-version state",
            Some(exact_summary.mixed_version_state_token.clone()),
            Some(current_summary.mixed_version_state_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "mixed_version_drift.reason",
            "Mixed-version reason",
            Some(exact_summary.mixed_version_reason_token.clone()),
            Some(current_summary.mixed_version_reason_token.clone()),
        );
        push_diff(
            &mut diff_rows,
            "degraded_fields",
            "Degraded field reasons",
            Some(join_tokens(&exact_summary.degraded_field_tokens)),
            Some(join_tokens(&current_summary.degraded_field_tokens)),
        );

        let requires_review_before_dispatch = diff_rows
            .iter()
            .any(|row| row.requires_review_before_dispatch);

        Self {
            record_kind: RUN_CONTEXT_COMPARISON_RECORD_KIND.to_owned(),
            schema_version: RUN_CONTEXT_SCHEMA_VERSION,
            exact_context_ref: exact_summary.execution_context_ref.clone(),
            current_context_ref: current_summary.execution_context_ref.clone(),
            exact_summary,
            current_summary,
            has_drift: !diff_rows.is_empty(),
            requires_review_before_dispatch,
            diff_rows,
        }
    }
}

/// Shell entry surface that consumes a [`RunContextSummary`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEntrySurface {
    pub record_kind: String,
    pub schema_version: u32,
    pub entry_point: ExecutionEntryPoint,
    pub entry_point_token: String,
    pub entry_point_label: String,
    pub context_summary: RunContextSummary,
    pub surface_matches_context: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_rerun_comparison: Option<RunContextComparison>,
    pub requires_review_before_dispatch: bool,
}

impl ExecutionEntrySurface {
    /// Project an entry surface from the canonical context.
    pub fn project(entry_point: ExecutionEntryPoint, context: &ExecutionContext) -> Self {
        Self::project_with_exact_rerun(entry_point, context, None)
    }

    /// Project an entry surface and attach an exact-rerun comparison.
    pub fn project_with_exact_rerun(
        entry_point: ExecutionEntryPoint,
        current_context: &ExecutionContext,
        exact_context: Option<&ExecutionContext>,
    ) -> Self {
        let context_summary = RunContextSummary::project(current_context);
        let surface_matches_context =
            current_context.invocation_subject.surface == entry_point.expected_surface();
        let exact_rerun_comparison =
            exact_context.map(|exact| RunContextComparison::compare(exact, current_context));
        let requires_review_before_dispatch = !surface_matches_context
            || context_summary.has_degraded_fields()
            || exact_rerun_comparison
                .as_ref()
                .map(|comparison| comparison.requires_review_before_dispatch)
                .unwrap_or(false);

        Self {
            record_kind: EXECUTION_ENTRY_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RUN_CONTEXT_SCHEMA_VERSION,
            entry_point,
            entry_point_token: entry_point.as_str().to_owned(),
            entry_point_label: entry_point.label().to_owned(),
            context_summary,
            surface_matches_context,
            exact_rerun_comparison,
            requires_review_before_dispatch,
        }
    }

    /// Render a stable plaintext block for support export.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!("Execution entry: {}\n", self.entry_point_token));
        out.push_str(&format!(
            "Context: {}\n",
            self.context_summary.execution_context_ref
        ));
        out.push_str(&format!(
            "Summary: {}\n",
            self.context_summary.summary_line()
        ));
        if let Some(comparison) = &self.exact_rerun_comparison {
            out.push_str(&format!(
                "Exact rerun drift: {} (review: {})\n",
                comparison.has_drift, comparison.requires_review_before_dispatch
            ));
            for row in &comparison.diff_rows {
                out.push_str(&format!(
                    "  - {}: {:?} vs {:?} [{}]\n",
                    row.field_path,
                    row.exact_value_token,
                    row.current_value_token,
                    row.diff_class_token
                ));
            }
        }
        out
    }
}

/// Snapshot containing all launch-capable entry surfaces for a workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionEntryTruthSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub workspace_id: String,
    pub entries: Vec<ExecutionEntrySurface>,
    pub all_entries_share_summary_shape: bool,
    pub any_entry_requires_review_before_dispatch: bool,
}

impl ExecutionEntryTruthSnapshot {
    /// Build a workspace snapshot from projected entry surfaces.
    pub fn from_entries(
        workspace_id: impl Into<String>,
        entries: Vec<ExecutionEntrySurface>,
    ) -> Self {
        let all_entries_share_summary_shape = entries.iter().all(|entry| {
            entry.context_summary.record_kind == RUN_CONTEXT_SUMMARY_RECORD_KIND
                && entry.context_summary.schema_version == RUN_CONTEXT_SCHEMA_VERSION
        });
        let any_entry_requires_review_before_dispatch = entries
            .iter()
            .any(|entry| entry.requires_review_before_dispatch);

        Self {
            record_kind: EXECUTION_ENTRY_TRUTH_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: RUN_CONTEXT_SCHEMA_VERSION,
            workspace_id: workspace_id.into(),
            entries,
            all_entries_share_summary_shape,
            any_entry_requires_review_before_dispatch,
        }
    }

    /// Locate the surface for one entry point.
    pub fn entry(&self, entry_point: ExecutionEntryPoint) -> Option<&ExecutionEntrySurface> {
        self.entries
            .iter()
            .find(|entry| entry.entry_point == entry_point)
    }
}

fn push_diff(
    rows: &mut Vec<RunContextDiffRow>,
    field_path: &str,
    label: &str,
    exact_value_token: Option<String>,
    current_value_token: Option<String>,
) {
    let diff_class = match (&exact_value_token, &current_value_token) {
        (Some(exact), Some(current)) if exact == current => RunContextDiffClass::Unchanged,
        (Some(_), Some(_)) => RunContextDiffClass::Changed,
        (None, Some(_)) => RunContextDiffClass::MissingInExact,
        (Some(_), None) => RunContextDiffClass::MissingInCurrent,
        (None, None) => RunContextDiffClass::Unchanged,
    };

    if diff_class == RunContextDiffClass::Unchanged {
        return;
    }

    rows.push(RunContextDiffRow {
        field_path: field_path.to_owned(),
        label: label.to_owned(),
        exact_value_token,
        current_value_token,
        diff_class,
        diff_class_token: diff_class.as_str().to_owned(),
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
mod tests;
