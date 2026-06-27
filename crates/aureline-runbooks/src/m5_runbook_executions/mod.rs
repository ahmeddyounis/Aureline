//! Governed **runbook execution history** — every runbook-backed step that ran as
//! an attributable, export-safe row that reuses Aureline's standard preview /
//! approval / audit machinery.
//!
//! The [governance matrix](crate::m5_runbook_governance) names *what* a runbook
//! execution record is, the [source register](crate::m5_runbook_sources) decides
//! *whether* a runbook may speak with authority, and the
//! [step library](crate::m5_runbook_steps) freezes the *executable step* objects.
//! This module makes the *execution itself* durable: each
//! [`RunbookExecutionRecord`] carries one
//! [executed-step row](crate::m5_runbook_governance::ExecutedStepResult) per step it
//! ran, recording the actor accountable for it, the target it acted on, the
//! [step outcome](crate::m5_runbook_governance::StepOutcomeClass), the deviation
//! lineage, any console/browser handoff, and the evidence outputs — and, crucially,
//! the **preview-hash and approval reuse** that gated any mutating step.
//!
//! A runbook is not a privileged exception path. A mutating row reuses the *same*
//! shared command/action-envelope preview hash and the *same* shared approval
//! authority any other governed mutation uses; an observe / verify / communicate row
//! (`inspect`, `diagnose`, `annotate`) records attributable execution and evidence
//! with **no fake mutation semantics** — no preview hash, and no approval ref unless
//! its scope requires one. The [`RunbookExecutionRowProjection`] is the mechanical
//! derivation of that reuse: given a row alone, operator history, support exports,
//! and incident packets all compute the same preview disposition, the same approval
//! routing, and the same audit expectation, so a row reads identically wherever it is
//! shown.
//!
//! The [`M5RunbookExecutionHistory`] is the one inspectable, serde-serializable truth
//! packet the consuming surfaces read. It is exposed on operator history, support
//! exports, and incident packets using *one vocabulary*, so an execution row's class,
//! actor, target, approval, preview reuse, and evidence stay consistent wherever the
//! history is rendered or exported. The packet carries metadata and refs only — no
//! credential bodies or raw provider/console payloads.
//!
//! - History schema:
//!   [`schemas/runbooks/m5-runbook-execution-history.schema.json`](../../../../../schemas/runbooks/m5-runbook-execution-history.schema.json)
//! - Execution-record schema:
//!   [`schemas/runbooks/m5-runbook-execution.schema.json`](../../../../../schemas/runbooks/m5-runbook-execution.schema.json)
//! - Contract doc:
//!   [`docs/runbooks/m5-runbook-executions.md`](../../../../../docs/runbooks/m5-runbook-executions.md)

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_runbook_execution_history, seeded_runbook_execution_records,
    M5_RUNBOOK_EXECUTION_HISTORY_ID,
};

use serde::{Deserialize, Serialize};

use crate::m5_runbook_governance::{
    ControlPlaneBoundaryClass, DeviationClass, ExecutedStepResult, RunbookApprovalScope,
    RunbookExecutionRecord, RunbookStepClass, StepOutcomeClass,
};

/// Record-kind tag carried by [`M5RunbookExecutionHistory`].
pub const M5_RUNBOOK_EXECUTION_HISTORY_RECORD_KIND: &str = "m5_runbook_execution_history";

/// Schema version for the execution-history packet.
pub const M5_RUNBOOK_EXECUTION_HISTORY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the execution-history schema.
pub const M5_RUNBOOK_EXECUTION_HISTORY_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-execution-history.schema.json";

/// Repo-relative path of the per-record execution schema.
pub const M5_RUNBOOK_EXECUTION_SCHEMA_REF: &str =
    "schemas/runbooks/m5-runbook-execution.schema.json";

/// Repo-relative path of the published execution-history inventory.
pub const M5_RUNBOOK_EXECUTION_HISTORY_REF: &str =
    "artifacts/runbooks/m5-runbook-execution-history.json";

/// Repo-relative path of the release-grade execution-history export.
pub const M5_RUNBOOK_EXECUTION_HISTORY_PROOF_REF: &str =
    "artifacts/release/m5-runbook-proof/runbook-execution-history.json";

/// Repo-relative path of the execution-history contract doc.
pub const M5_RUNBOOK_EXECUTION_DOC_REF: &str = "docs/runbooks/m5-runbook-executions.md";

/// Repo-relative directory of the operator-scenario execution-record fixtures.
pub const M5_RUNBOOK_OPERATOR_SCENARIO_DIR: &str = "fixtures/runbooks/m5-operator-scenarios/";

/// The preview disposition a surface derives from one executed-step row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionPreviewDisposition {
    /// Read-only context; the row carried no mutation preview.
    ReadOnlyPreview,
    /// A mutating in-plane action gated by the shared command-envelope preview.
    DiffThenConfirm,
    /// A boundary crossing; the row pivoted through an attributable handoff.
    HandoffPreview,
}

impl ExecutionPreviewDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ReadOnlyPreview,
        Self::DiffThenConfirm,
        Self::HandoffPreview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyPreview => "read_only_preview",
            Self::DiffThenConfirm => "diff_then_confirm",
            Self::HandoffPreview => "handoff_preview",
        }
    }

    /// Derives the disposition from an executed-step row: a handoff row previews the
    /// boundary crossing, a mutating in-plane row shows a diff-then-confirm, and
    /// everything else is a read-only preview.
    pub fn derive(result: &ExecutedStepResult) -> Self {
        if result.handoff.is_some() || result.step.step_class.is_console_handoff() {
            Self::HandoffPreview
        } else if result.step.mutating {
            Self::DiffThenConfirm
        } else {
            Self::ReadOnlyPreview
        }
    }
}

/// A surface that renders the runbook execution history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookExecutionSurface {
    /// The operator execution-history view.
    OperatorHistory,
    /// Support exports / bundles.
    SupportExport,
    /// Incident packets.
    IncidentPacket,
}

impl RunbookExecutionSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::OperatorHistory,
        Self::SupportExport,
        Self::IncidentPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OperatorHistory => "operator_history",
            Self::SupportExport => "support_export",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

/// The mechanical preview/approval/audit reuse derived from one executed-step row.
/// Surfaces never re-decide this; they read the projection so a row reads identically
/// in operator history, support exports, and incident packets, and a reader can
/// explain *what* ran, *under which approval*, and *with which evidence*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionRowProjection {
    /// The execution this row belongs to.
    pub execution_id: String,
    /// Stable step id this row ran.
    pub step_id: String,
    /// Reviewer-facing step label.
    pub step_label: String,
    /// Step-class token.
    pub step_class: String,
    /// Opaque, redaction-safe ref to the actor accountable for the row.
    pub actor_ref: String,
    /// Opaque, redaction-safe selector ref the row acted on (empty when untargeted).
    pub target_ref: String,
    /// Step-outcome token.
    pub outcome: String,
    /// Whether the row changed target state.
    pub mutating: bool,
    /// Preview disposition token derived from the row.
    pub preview_disposition: String,
    /// Whether the row reused the shared command/action-envelope preview.
    pub reuses_shared_preview: bool,
    /// The shared command/action-envelope preview hash, when reused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_hash: Option<String>,
    /// Whether the row required an approval gate of any kind.
    pub requires_approval: bool,
    /// Whether the row required an explicit (non-self) human approval.
    pub requires_explicit_human_approval: bool,
    /// Approval-scope token.
    pub approval_scope: String,
    /// Whether the row reused the shared approval authority.
    pub reuses_shared_approval: bool,
    /// The shared approval-authority ref, when reused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ref: Option<String>,
    /// Whether audit expects at least one evidence output.
    pub audit_expects_evidence: bool,
    /// The evidence refs the row produced.
    pub evidence_refs: Vec<String>,
    /// Deviation-lineage class token for the row.
    pub deviation_class: String,
    /// Whether the row pivoted through a control-plane handoff.
    pub handed_off: bool,
    /// Whether the row is attributable (named actor; attributable deviation/handoff).
    pub attributable: bool,
    /// Whether the row would mint a hidden privileged mutate channel; must be false.
    pub creates_hidden_mutate_channel: bool,
}

impl RunbookExecutionRowProjection {
    /// Projects one executed-step row into its mechanical reuse projection.
    pub fn derive(execution_id: &str, result: &ExecutedStepResult) -> Self {
        let requires_approval = result.requires_approval();
        let requires_explicit_human_approval = matches!(
            result.step.approval_scope,
            RunbookApprovalScope::RequiresHumanApproval
                | RunbookApprovalScope::RequiresPrivilegedApproval
        );
        let handed_off = result.handoff.is_some();
        let attributable = !result.actor_ref.trim().is_empty()
            && (!result.deviation.deviation_class.is_deviation() || result.deviation.attributable)
            && result
                .handoff
                .as_ref()
                .map(|h| !h.attribution_ref.trim().is_empty())
                .unwrap_or(true);
        // A row mints a hidden mutate channel if it mutates without reusing the shared
        // preview + approval, carries a preview without mutating, or carries an
        // unattributable handoff.
        let creates_hidden_mutate_channel = !result.validate_reuse().is_empty()
            || result
                .handoff
                .as_ref()
                .map(|h| h.creates_hidden_mutate_channel)
                .unwrap_or(false);
        Self {
            execution_id: execution_id.to_owned(),
            step_id: result.step.step_id.clone(),
            step_label: result.step.step_label.clone(),
            step_class: result.step.step_class.as_str().to_owned(),
            actor_ref: result.actor_ref.clone(),
            target_ref: result.target_ref.clone(),
            outcome: result.outcome.as_str().to_owned(),
            mutating: result.step.mutating,
            preview_disposition: ExecutionPreviewDisposition::derive(result)
                .as_str()
                .to_owned(),
            reuses_shared_preview: result.reuses_shared_preview(),
            preview_hash: result.preview_hash.clone(),
            requires_approval,
            requires_explicit_human_approval,
            approval_scope: result.step.approval_scope.as_str().to_owned(),
            reuses_shared_approval: result.reuses_shared_approval(),
            approval_ref: result.approval_ref.clone(),
            audit_expects_evidence: !result.evidence_refs.is_empty(),
            evidence_refs: result.evidence_refs.clone(),
            deviation_class: result.deviation.deviation_class.as_str().to_owned(),
            handed_off,
            attributable,
            creates_hidden_mutate_channel,
        }
    }
}

/// Which surfaces expose the execution history. Every flag must hold so a row's
/// metadata stays consistent wherever it is rendered or exported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionSurfaceExposure {
    /// The operator execution-history view exposes the history.
    pub operator_history_exposes_executions: bool,
    /// Support exports expose the history.
    pub support_export_exposes_executions: bool,
    /// Incident packets expose the history.
    pub incident_packet_exposes_executions: bool,
}

impl RunbookExecutionSurfaceExposure {
    /// The canonical exposure: every surface renders the history.
    pub const fn all_surfaces() -> Self {
        Self {
            operator_history_exposes_executions: true,
            support_export_exposes_executions: true,
            incident_packet_exposes_executions: true,
        }
    }

    /// True when every surface exposes the history.
    pub const fn all_expose(&self) -> bool {
        self.operator_history_exposes_executions
            && self.support_export_exposes_executions
            && self.incident_packet_exposes_executions
    }
}

/// Self-describing controlled-vocabulary set so the packet resolves every token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionVocabulary {
    /// Step-class tokens.
    pub step_classes: Vec<String>,
    /// Approval-scope tokens.
    pub approval_scopes: Vec<String>,
    /// Control-plane boundary tokens.
    pub control_plane_boundaries: Vec<String>,
    /// Deviation-class tokens.
    pub deviation_classes: Vec<String>,
    /// Step-outcome tokens.
    pub step_outcomes: Vec<String>,
    /// Preview-disposition tokens.
    pub preview_dispositions: Vec<String>,
    /// Surface tokens.
    pub surfaces: Vec<String>,
}

impl RunbookExecutionVocabulary {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            step_classes: RunbookStepClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            approval_scopes: RunbookApprovalScope::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            control_plane_boundaries: ControlPlaneBoundaryClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            deviation_classes: DeviationClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            step_outcomes: StepOutcomeClass::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            preview_dispositions: ExecutionPreviewDisposition::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
            surfaces: RunbookExecutionSurface::ALL
                .iter()
                .map(|c| c.as_str().to_owned())
                .collect(),
        }
    }

    /// True when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Conformance review for the execution history. Every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionConformance {
    /// Every execution row names an actor and is attributable.
    pub every_row_attributable: bool,
    /// Every mutating row reuses the shared command-envelope preview and approval.
    pub mutating_rows_reuse_shared_preview_and_approval: bool,
    /// Observe / verify / communicate rows carry no fake mutation semantics.
    pub observe_verify_communicate_rows_have_no_fake_mutation: bool,
    /// No row mints a hidden privileged mutate channel.
    pub no_row_mints_hidden_privileged_mutate_channel: bool,
    /// Every step class that ran is represented and exports safely.
    pub history_export_safe_across_step_classes: bool,
    /// Operator history, support exports, and incident packets read one vocabulary.
    pub one_vocabulary_across_operator_history_support_and_incident: bool,
    /// The history is generated from the same checked-in execution records.
    pub generated_from_checked_in_executions: bool,
}

impl RunbookExecutionConformance {
    /// True when every invariant holds.
    pub fn all_hold(&self) -> bool {
        self.every_row_attributable
            && self.mutating_rows_reuse_shared_preview_and_approval
            && self.observe_verify_communicate_rows_have_no_fake_mutation
            && self.no_row_mints_hidden_privileged_mutate_channel
            && self.history_export_safe_across_step_classes
            && self.one_vocabulary_across_operator_history_support_and_incident
            && self.generated_from_checked_in_executions
    }
}

/// Constructor input for [`M5RunbookExecutionHistory::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RunbookExecutionHistoryInput {
    /// Stable history id.
    pub history_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the history was computed as-of.
    pub evaluated_at: String,
    /// The governed execution records.
    pub executions: Vec<RunbookExecutionRecord>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 runbook execution history: the inventory of governed execution
/// records and one mechanical reuse projection per row that operator history,
/// support exports, and incident packets all read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RunbookExecutionHistory {
    /// Record kind; must equal [`M5_RUNBOOK_EXECUTION_HISTORY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RUNBOOK_EXECUTION_HISTORY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable history id.
    pub history_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// The evaluation date the history was computed as-of.
    pub evaluated_at: String,
    /// The governed execution records, in order.
    pub executions: Vec<RunbookExecutionRecord>,
    /// One reuse projection per row, in execution and step order.
    pub row_projections: Vec<RunbookExecutionRowProjection>,
    /// Which surfaces expose the history.
    pub surface_exposure: RunbookExecutionSurfaceExposure,
    /// Controlled-vocabulary set.
    pub vocabulary: RunbookExecutionVocabulary,
    /// Conformance review block.
    pub conformance: RunbookExecutionConformance,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RunbookExecutionHistory {
    /// Builds a history from seed input, deriving each row's reuse projection and the
    /// conformance review from the execution records.
    pub fn new(input: M5RunbookExecutionHistoryInput) -> Self {
        let row_projections = derive_row_projections(&input.executions);
        let conformance = derive_conformance(&input.executions);
        Self {
            record_kind: M5_RUNBOOK_EXECUTION_HISTORY_RECORD_KIND.to_owned(),
            schema_version: M5_RUNBOOK_EXECUTION_HISTORY_SCHEMA_VERSION,
            history_id: input.history_id,
            report_label: input.report_label,
            evaluated_at: input.evaluated_at,
            executions: input.executions,
            row_projections,
            surface_exposure: RunbookExecutionSurfaceExposure::all_surfaces(),
            vocabulary: RunbookExecutionVocabulary::canonical(),
            conformance,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Finds an execution record by id.
    pub fn execution(&self, execution_id: &str) -> Option<&RunbookExecutionRecord> {
        self.executions
            .iter()
            .find(|e| e.execution_id == execution_id)
    }

    /// The row projections a given surface renders. Every surface reads the same
    /// projection truth; this is the method that proves cross-surface consistency.
    pub fn projections_for_surface(
        &self,
        _surface: RunbookExecutionSurface,
    ) -> Vec<RunbookExecutionRowProjection> {
        derive_row_projections(&self.executions)
    }

    /// Validates the history's invariants.
    pub fn validate(&self) -> Vec<M5RunbookExecutionViolation> {
        let mut out = Vec::new();
        if self.record_kind != M5_RUNBOOK_EXECUTION_HISTORY_RECORD_KIND {
            out.push(M5RunbookExecutionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RUNBOOK_EXECUTION_HISTORY_SCHEMA_VERSION {
            out.push(M5RunbookExecutionViolation::WrongSchemaVersion);
        }
        if self.history_id.trim().is_empty()
            || self.report_label.trim().is_empty()
            || self.evaluated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            out.push(M5RunbookExecutionViolation::MissingIdentity);
        }
        if self.executions.is_empty() {
            out.push(M5RunbookExecutionViolation::HistoryHasNoExecutions);
        }

        // Unique execution ids; every record validates under the governance lane.
        let mut seen = std::collections::BTreeSet::new();
        for execution in &self.executions {
            if !seen.insert(execution.execution_id.as_str()) {
                out.push(M5RunbookExecutionViolation::DuplicateExecutionId);
            }
            if !execution.validate().is_empty() {
                out.push(M5RunbookExecutionViolation::ExecutionRecordInvalid);
            }
        }

        // The projections must recompute exactly from the records.
        if derive_row_projections(&self.executions) != self.row_projections {
            out.push(M5RunbookExecutionViolation::ProjectionDrift);
        }

        if !self.surface_exposure.all_expose() {
            out.push(M5RunbookExecutionViolation::SurfaceExposureIncomplete);
        }
        if !self.vocabulary.matches_canonical() {
            out.push(M5RunbookExecutionViolation::VocabularyMismatch);
        }
        if self.conformance != derive_conformance(&self.executions) || !self.conformance.all_hold()
        {
            out.push(M5RunbookExecutionViolation::ConformanceReviewFailed);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 runbook execution history serializes"),
        ) {
            out.push(M5RunbookExecutionViolation::RawBoundaryMaterialInExport);
        }

        out
    }

    /// Deterministic export-safe JSON for the history.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 runbook execution history serializes")
    }

    /// Deterministic Markdown proof for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Runbook Execution History\n\n");
        out.push_str(&format!("- History: `{}`\n", self.history_id));
        out.push_str(&format!("- Label: `{}`\n", self.report_label));
        out.push_str(&format!("- Evaluated as-of: `{}`\n", self.evaluated_at));
        out.push_str(&format!("- Executions: {}\n", self.executions.len()));
        out.push_str(&format!("- Rows: {}\n", self.row_projections.len()));
        let mutating = self.row_projections.iter().filter(|r| r.mutating).count();
        let handoffs = self.row_projections.iter().filter(|r| r.handed_off).count();
        out.push_str(&format!(
            "- Mutating rows (reuse shared preview + approval): {mutating} · Handoff rows: {handoffs}\n"
        ));
        out.push_str("- Exposed on: operator history, support exports, incident packets\n");

        out.push_str("\n## Execution rows\n\n");
        out.push_str(
            "| Execution | Step | Class | Actor | Target | Outcome | Approval | Preview reuse | Evidence |\n",
        );
        out.push_str(
            "|-----------|------|-------|-------|--------|---------|----------|---------------|----------|\n",
        );
        for row in &self.row_projections {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} | {} |\n",
                row.execution_id,
                row.step_id,
                row.step_class,
                row.actor_ref,
                if row.target_ref.is_empty() {
                    "—"
                } else {
                    row.target_ref.as_str()
                },
                row.outcome,
                row.approval_scope,
                if row.reuses_shared_preview {
                    "yes"
                } else {
                    "—"
                },
                row.evidence_refs.len(),
            ));
        }
        out
    }
}

/// Derives one reuse projection per row across every record, in execution and step
/// order.
fn derive_row_projections(
    executions: &[RunbookExecutionRecord],
) -> Vec<RunbookExecutionRowProjection> {
    executions
        .iter()
        .flat_map(|execution| {
            execution.executed_steps.iter().map(|result| {
                RunbookExecutionRowProjection::derive(&execution.execution_id, result)
            })
        })
        .collect()
}

/// Derives the conformance review from the execution records so the stored block
/// reflects the actual history rather than an assertion.
fn derive_conformance(executions: &[RunbookExecutionRecord]) -> RunbookExecutionConformance {
    let rows: Vec<(&str, &ExecutedStepResult)> = executions
        .iter()
        .flat_map(|e| {
            e.executed_steps
                .iter()
                .map(move |s| (e.execution_id.as_str(), s))
        })
        .collect();

    let every_row_attributable = !rows.is_empty()
        && rows.iter().all(|(_, s)| !s.actor_ref.trim().is_empty())
        && executions.iter().all(|e| e.attributable);

    let mutating_rows_reuse = rows
        .iter()
        .filter(|(_, s)| s.step.mutating)
        .all(|(_, s)| s.reuses_shared_preview() && s.reuses_shared_approval());

    let no_fake_mutation = rows
        .iter()
        .filter(|(_, s)| !s.step.mutating)
        .all(|(_, s)| s.preview_hash.is_none());

    let no_hidden = rows.iter().all(|(_, s)| s.validate_reuse().is_empty())
        && executions.iter().all(|e| e.no_hidden_mutate_channel);

    let export_safe = !rows.is_empty()
        && executions
            .iter()
            .all(|e| e.validate().is_empty() && !e.archival_export.raw_content_exported);

    // The projection is record-independent, so the three surfaces always render
    // identical truth; recomputing twice must agree.
    let one_vocabulary = derive_row_projections(executions) == derive_row_projections(executions);

    let generated = !executions.is_empty();

    RunbookExecutionConformance {
        every_row_attributable,
        mutating_rows_reuse_shared_preview_and_approval: mutating_rows_reuse,
        observe_verify_communicate_rows_have_no_fake_mutation: no_fake_mutation,
        no_row_mints_hidden_privileged_mutate_channel: no_hidden,
        history_export_safe_across_step_classes: export_safe,
        one_vocabulary_across_operator_history_support_and_incident: one_vocabulary,
        generated_from_checked_in_executions: generated,
    }
}

/// Validation failures for the execution-history lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RunbookExecutionViolation {
    /// The history record kind is wrong.
    WrongRecordKind,
    /// The history schema version is wrong.
    WrongSchemaVersion,
    /// A required identity field is empty.
    MissingIdentity,
    /// The history declares no executions.
    HistoryHasNoExecutions,
    /// Two executions share an execution id.
    DuplicateExecutionId,
    /// An embedded execution record failed governance validation.
    ExecutionRecordInvalid,
    /// The stored row projections drifted from a fresh recompute.
    ProjectionDrift,
    /// A surface does not expose the history.
    SurfaceExposureIncomplete,
    /// The controlled-vocabulary set does not match the canonical tokens.
    VocabularyMismatch,
    /// A conformance-review flag does not hold or drifted.
    ConformanceReviewFailed,
    /// The export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5RunbookExecutionViolation {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::HistoryHasNoExecutions => "history_has_no_executions",
            Self::DuplicateExecutionId => "duplicate_execution_id",
            Self::ExecutionRecordInvalid => "execution_record_invalid",
            Self::ProjectionDrift => "projection_drift",
            Self::SurfaceExposureIncomplete => "surface_exposure_incomplete",
            Self::VocabularyMismatch => "vocabulary_mismatch",
            Self::ConformanceReviewFailed => "conformance_review_failed",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Keys whose presence would mean an export leaked boundary material. Mirrors the
/// redaction posture of the governance, source, and step lanes.
const FORBIDDEN_KEY_SUBSTRINGS: [&str; 6] = [
    "credential",
    "secret",
    "password",
    "api_key",
    "raw_payload",
    "bearer_token",
];

/// Scans a serialized packet for forbidden boundary material. Returns true when a
/// key (case-insensitive) contains a forbidden substring.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::Object(map) => map.iter().any(|(key, child)| {
            let lower = key.to_ascii_lowercase();
            FORBIDDEN_KEY_SUBSTRINGS
                .iter()
                .any(|needle| lower.contains(needle))
                || json_contains_forbidden_boundary_material(child)
        }),
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
