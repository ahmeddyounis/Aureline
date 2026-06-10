//! Explain-plan freshness notes, engine-version context, and plan-comparison flow
//! qualification records.
//!
//! This module owns the typed records that keep explain-plan freshness notes,
//! engine-version context, and plan-comparison flows inspectable and attributable
//! without depending on hidden shell shortcuts or ad hoc scripts. The boundary
//! schema is
//! [`/schemas/data/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.schema.json`](../../../schemas/data/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json`](../../../artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json).
//!
//! Raw plan payloads, raw engine versions, raw hostnames, and raw connection
//! strings do not belong in these records. They carry stable IDs, closed
//! posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for explain-plan qualification packets.
pub const EXPLAIN_PLAN_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ExplainPlanQualificationPacket`].
pub const EXPLAIN_PLAN_QUALIFICATION_RECORD_KIND: &str =
    "explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows";

/// Repo-relative path to the checked-in explain-plan qualification packet.
pub const EXPLAIN_PLAN_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json";

/// Embedded checked-in packet JSON.
pub const EXPLAIN_PLAN_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json"
));

/// Qualification label shown on promoted explain-plan surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainPlanQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl ExplainPlanQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Explain-plan surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainPlanSurfaceKind {
    /// Freshness note panel or row shown with an explain plan.
    ExplainPlanFreshnessNote,
    /// Engine-version context chip or panel shown with a plan.
    EngineVersionContext,
    /// Plan-comparison flow for baseline versus compared plans.
    PlanComparisonFlow,
}

/// Explain-plan capture mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainPlanMode {
    /// Estimated plan only; statement did not execute.
    Estimated,
    /// Actual plan captured from execution.
    Actual,
    /// Imported plan with captured freshness.
    ImportedStale,
}

/// Freshness state of a captured or displayed plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Plan is current relative to the captured engine version.
    Current,
    /// Plan is stale and should be labeled.
    Stale,
    /// Freshness cannot be determined.
    Unknown,
}

/// Basis on which two plans are being compared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonBasis {
    /// Estimated plan compared against actual execution plan.
    EstimatedVsActual,
    /// Same query captured at different times.
    SameQueryDifferentTime,
    /// Different queries being compared.
    DifferentQuery,
    /// Imported plan compared against a live plan.
    ImportedVsLive,
}

/// Outcome of a plan comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOutcome {
    /// Plans are equivalent within the comparison basis.
    Equivalent,
    /// Plans diverge and the difference is visible.
    Divergent,
    /// Comparison is inconclusive due to missing context.
    Inconclusive,
    /// Rollback or re-evaluation is recommended.
    RollbackRecommended,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanSurfaceGuardSet {
    /// Engine version is visible without exposing raw host details.
    pub engine_version_visible: bool,
    /// Freshness note is visible on the plan surface.
    pub freshness_note_visible: bool,
    /// Plan mode (estimated/actual/imported) is visible.
    pub plan_mode_visible: bool,
    /// Stale imported plans are visibly labeled.
    pub stale_import_labeled: bool,
    /// Comparison basis is visible before diff is shown.
    pub comparison_basis_visible: bool,
    /// Plan diff is visible when comparison is divergent.
    pub comparison_diff_visible: bool,
    /// Rollback or re-eval recommendation is visible when applicable.
    pub comparison_rollback_visible: bool,
    /// Downgrade behavior on mismatch is disclosed.
    pub downgrade_on_mismatch_visible: bool,
}

impl ExplainPlanSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.engine_version_visible
            && self.freshness_note_visible
            && self.plan_mode_visible
            && self.stale_import_labeled
            && self.comparison_basis_visible
            && self.comparison_diff_visible
            && self.comparison_rollback_visible
            && self.downgrade_on_mismatch_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: ExplainPlanSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: ExplainPlanQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: ExplainPlanQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<ExplainPlanQualificationProof>,
    /// Visible guard set.
    pub guards: ExplainPlanSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One explain-plan freshness note row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanFreshnessNoteRow {
    /// Stable note id.
    pub note_id: String,
    /// Plan mode.
    pub plan_mode: ExplainPlanMode,
    /// Engine/version ref.
    pub engine_version_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Freshness state.
    pub freshness_state: FreshnessState,
    /// Whether stale plans are visibly labeled.
    pub stale_labeled: bool,
    /// Whether this plan implies statement execution.
    pub implies_execution: bool,
    /// Whether the note is visible in UI.
    pub visible_in_ui: bool,
}

/// One engine-version context row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineVersionContextRow {
    /// Stable context id.
    pub context_id: String,
    /// Engine family label.
    pub engine_family: String,
    /// Version ref.
    pub version_ref: String,
    /// Whether version mismatch is visibly flagged.
    pub version_mismatch_visible: bool,
    /// Whether context is visible in the plan view.
    pub context_visible_in_plan: bool,
    /// Whether context is visible in the comparison view.
    pub context_visible_in_comparison: bool,
    /// Whether the context is visible in UI.
    pub visible_in_ui: bool,
}

/// One plan-comparison flow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanComparisonFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Comparison basis.
    pub comparison_basis: ComparisonBasis,
    /// Baseline plan ref.
    pub baseline_plan_ref: String,
    /// Compared plan ref.
    pub compared_plan_ref: String,
    /// Outcome of the comparison.
    pub outcome: ComparisonOutcome,
    /// Whether diff is visible when divergent.
    pub diff_visible: bool,
    /// Whether rollback recommendation is visible when applicable.
    pub rollback_visible: bool,
    /// Whether downgrade on mismatch is enforced.
    pub downgrade_on_mismatch: bool,
    /// Whether the flow is visible in UI.
    pub visible_in_ui: bool,
}

/// Summary counts for an explain-plan qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of freshness note rows.
    pub freshness_note_count: usize,
    /// Number of engine-version context rows.
    pub engine_version_context_count: usize,
    /// Number of plan-comparison flow rows.
    pub plan_comparison_flow_count: usize,
}

/// Canonical explain-plan qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainPlanQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<ExplainPlanSurfaceQualificationRow>,
    /// Freshness note rows.
    pub freshness_notes: Vec<ExplainPlanFreshnessNoteRow>,
    /// Engine-version context rows.
    pub engine_version_contexts: Vec<EngineVersionContextRow>,
    /// Plan-comparison flow rows.
    pub plan_comparison_flows: Vec<PlanComparisonFlowRow>,
    /// Summary counts.
    pub summary: ExplainPlanQualificationSummary,
}

impl ExplainPlanQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> ExplainPlanQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        ExplainPlanQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            freshness_note_count: self.freshness_notes.len(),
            engine_version_context_count: self.engine_version_contexts.len(),
            plan_comparison_flow_count: self.plan_comparison_flows.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<ExplainPlanQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != EXPLAIN_PLAN_QUALIFICATION_SCHEMA_VERSION {
            violations.push(ExplainPlanQualificationViolation::SchemaVersion {
                expected: EXPLAIN_PLAN_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXPLAIN_PLAN_QUALIFICATION_RECORD_KIND {
            violations.push(ExplainPlanQualificationViolation::RecordKind {
                expected: EXPLAIN_PLAN_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            ExplainPlanQualificationViolationKind::Surface,
        );
        collect_ids(
            self.freshness_notes.iter().map(|row| row.note_id.as_str()),
            &mut violations,
            ExplainPlanQualificationViolationKind::FreshnessNote,
        );
        collect_ids(
            self.engine_version_contexts
                .iter()
                .map(|row| row.context_id.as_str()),
            &mut violations,
            ExplainPlanQualificationViolationKind::EngineVersionContext,
        );
        collect_ids(
            self.plan_comparison_flows
                .iter()
                .map(|row| row.flow_id.as_str()),
            &mut violations,
            ExplainPlanQualificationViolationKind::PlanComparisonFlow,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        ExplainPlanQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        ExplainPlanQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    ExplainPlanQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let plan_modes: BTreeSet<_> = self
            .freshness_notes
            .iter()
            .map(|row| row.plan_mode)
            .collect();
        for required_mode in [
            ExplainPlanMode::Estimated,
            ExplainPlanMode::Actual,
            ExplainPlanMode::ImportedStale,
        ] {
            if !plan_modes.contains(&required_mode) {
                violations.push(ExplainPlanQualificationViolation::MissingExplainPlanMode {
                    plan_mode: required_mode,
                });
            }
        }
        for row in &self.freshness_notes {
            if row.engine_version_ref.is_empty() || row.captured_at.is_empty() {
                violations.push(
                    ExplainPlanQualificationViolation::IncompleteFreshnessNote {
                        note_id: row.note_id.clone(),
                    },
                );
            }
            if row.plan_mode == ExplainPlanMode::Estimated && row.implies_execution {
                violations.push(
                    ExplainPlanQualificationViolation::EstimatedPlanImpliesExecution {
                        note_id: row.note_id.clone(),
                    },
                );
            }
            if row.plan_mode == ExplainPlanMode::ImportedStale && !row.stale_labeled {
                violations.push(ExplainPlanQualificationViolation::StalePlanUnlabeled {
                    note_id: row.note_id.clone(),
                });
            }
            if !row.visible_in_ui {
                violations.push(
                    ExplainPlanQualificationViolation::FreshnessNoteNotVisibleInUi {
                        note_id: row.note_id.clone(),
                    },
                );
            }
        }

        let freshness_states: BTreeSet<_> = self
            .freshness_notes
            .iter()
            .map(|row| row.freshness_state)
            .collect();
        for required_state in [FreshnessState::Current, FreshnessState::Stale, FreshnessState::Unknown] {
            if !freshness_states.contains(&required_state) {
                violations.push(ExplainPlanQualificationViolation::MissingFreshnessState {
                    freshness_state: required_state,
                });
            }
        }

        for row in &self.engine_version_contexts {
            if row.engine_family.is_empty() || row.version_ref.is_empty() {
                violations.push(
                    ExplainPlanQualificationViolation::IncompleteEngineVersionContext {
                        context_id: row.context_id.clone(),
                    },
                );
            }
            if !row.context_visible_in_plan && !row.context_visible_in_comparison {
                violations.push(
                    ExplainPlanQualificationViolation::EngineVersionContextNotVisible {
                        context_id: row.context_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ExplainPlanQualificationViolation::EngineVersionContextNotVisibleInUi {
                        context_id: row.context_id.clone(),
                    },
                );
            }
        }

        let comparison_bases: BTreeSet<_> = self
            .plan_comparison_flows
            .iter()
            .map(|row| row.comparison_basis)
            .collect();
        for required_basis in [
            ComparisonBasis::EstimatedVsActual,
            ComparisonBasis::SameQueryDifferentTime,
            ComparisonBasis::DifferentQuery,
            ComparisonBasis::ImportedVsLive,
        ] {
            if !comparison_bases.contains(&required_basis) {
                violations.push(ExplainPlanQualificationViolation::MissingComparisonBasis {
                    comparison_basis: required_basis,
                });
            }
        }

        let comparison_outcomes: BTreeSet<_> = self
            .plan_comparison_flows
            .iter()
            .map(|row| row.outcome)
            .collect();
        for required_outcome in [
            ComparisonOutcome::Equivalent,
            ComparisonOutcome::Divergent,
            ComparisonOutcome::Inconclusive,
            ComparisonOutcome::RollbackRecommended,
        ] {
            if !comparison_outcomes.contains(&required_outcome) {
                violations.push(ExplainPlanQualificationViolation::MissingComparisonOutcome {
                    comparison_outcome: required_outcome,
                });
            }
        }

        for row in &self.plan_comparison_flows {
            if row.baseline_plan_ref.is_empty() || row.compared_plan_ref.is_empty() {
                violations.push(
                    ExplainPlanQualificationViolation::IncompletePlanComparisonFlow {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
            if row.outcome == ComparisonOutcome::Divergent && !row.diff_visible {
                violations.push(
                    ExplainPlanQualificationViolation::DivergentComparisonDiffHidden {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
            if row.outcome == ComparisonOutcome::RollbackRecommended && !row.rollback_visible {
                violations.push(
                    ExplainPlanQualificationViolation::RollbackRecommendationHidden {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
            if !row.visible_in_ui {
                violations.push(
                    ExplainPlanQualificationViolation::PlanComparisonFlowNotVisibleInUi {
                        flow_id: row.flow_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ExplainPlanQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in explain-plan qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_explain_plan_qualification() -> Result<ExplainPlanQualificationPacket, serde_json::Error> {
    serde_json::from_str(EXPLAIN_PLAN_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplainPlanQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Freshness note rows.
    FreshnessNote,
    /// Engine-version context rows.
    EngineVersionContext,
    /// Plan-comparison flow rows.
    PlanComparisonFlow,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<ExplainPlanQualificationViolation>,
    kind: ExplainPlanQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(ExplainPlanQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for explain-plan qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExplainPlanQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: ExplainPlanQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required explain-plan mode is missing.
    MissingExplainPlanMode { plan_mode: ExplainPlanMode },
    /// Freshness note lacks engine/version or capture timestamp.
    IncompleteFreshnessNote { note_id: String },
    /// Estimated plan incorrectly implies execution.
    EstimatedPlanImpliesExecution { note_id: String },
    /// Stale imported plan is not labeled.
    StalePlanUnlabeled { note_id: String },
    /// Freshness note is not visible in UI.
    FreshnessNoteNotVisibleInUi { note_id: String },
    /// Required freshness state is missing.
    MissingFreshnessState { freshness_state: FreshnessState },
    /// Engine-version context lacks engine family or version ref.
    IncompleteEngineVersionContext { context_id: String },
    /// Engine-version context is not visible in plan or comparison.
    EngineVersionContextNotVisible { context_id: String },
    /// Engine-version context is not visible in UI.
    EngineVersionContextNotVisibleInUi { context_id: String },
    /// Required comparison basis is missing.
    MissingComparisonBasis { comparison_basis: ComparisonBasis },
    /// Required comparison outcome is missing.
    MissingComparisonOutcome { comparison_outcome: ComparisonOutcome },
    /// Plan-comparison flow lacks baseline or compared plan ref.
    IncompletePlanComparisonFlow { flow_id: String },
    /// Divergent comparison hides the diff.
    DivergentComparisonDiffHidden { flow_id: String },
    /// Rollback recommendation is hidden when applicable.
    RollbackRecommendationHidden { flow_id: String },
    /// Plan-comparison flow is not visible in UI.
    PlanComparisonFlowNotVisibleInUi { flow_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for ExplainPlanQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingExplainPlanMode { plan_mode } => {
                write!(f, "explain-plan mode {plan_mode:?} is not covered")
            }
            Self::IncompleteFreshnessNote { note_id } => {
                write!(f, "{note_id} lacks plan engine/version or freshness truth")
            }
            Self::EstimatedPlanImpliesExecution { note_id } => {
                write!(f, "{note_id} makes an estimated plan imply execution")
            }
            Self::StalePlanUnlabeled { note_id } => {
                write!(f, "{note_id} does not label a stale imported plan")
            }
            Self::FreshnessNoteNotVisibleInUi { note_id } => {
                write!(f, "{note_id} is not visible in UI")
            }
            Self::MissingFreshnessState { freshness_state } => {
                write!(f, "freshness state {freshness_state:?} is not covered")
            }
            Self::IncompleteEngineVersionContext { context_id } => {
                write!(f, "{context_id} lacks engine family or version ref")
            }
            Self::EngineVersionContextNotVisible { context_id } => {
                write!(f, "{context_id} is not visible in plan or comparison")
            }
            Self::EngineVersionContextNotVisibleInUi { context_id } => {
                write!(f, "{context_id} is not visible in UI")
            }
            Self::MissingComparisonBasis { comparison_basis } => {
                write!(f, "comparison basis {comparison_basis:?} is not covered")
            }
            Self::MissingComparisonOutcome { comparison_outcome } => {
                write!(f, "comparison outcome {comparison_outcome:?} is not covered")
            }
            Self::IncompletePlanComparisonFlow { flow_id } => {
                write!(f, "{flow_id} lacks baseline or compared plan ref")
            }
            Self::DivergentComparisonDiffHidden { flow_id } => {
                write!(f, "{flow_id} hides diff on a divergent comparison")
            }
            Self::RollbackRecommendationHidden { flow_id } => {
                write!(f, "{flow_id} hides rollback recommendation when applicable")
            }
            Self::PlanComparisonFlowNotVisibleInUi { flow_id } => {
                write!(f, "{flow_id} is not visible in UI")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for ExplainPlanQualificationViolation {}
