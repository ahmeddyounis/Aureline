//! Notebook result comparison, baseline selection, and confounder visibility.
//!
//! This module materializes the typed records that keep notebook review,
//! experiment lineage, and reproducibility claims honest about result
//! comparison, baseline selection, and confounder visibility. The records and
//! closed vocabularies here mirror the boundary schema at
//! `/schemas/notebook/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.schema.json`
//! and reuse the experiment lineage and diff vocabularies already frozen in
//! `/schemas/notebook/implement_experiment_run_identities_environment_fingerprints_dataset_cards_and_artifact_lineage.schema.json`
//! and `/schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookResultComparison`] record that carries a cell-aware
//!   comparison between a baseline run and a current run, with mode, scope,
//!   outcome, and confounder refs so reviewers never silently assume code is
//!   the only variable;
//! - the [`NotebookBaselineSelection`] record that names how the baseline was
//!   chosen, its source, state, and pinning actor so the comparison boundary
//!   is explicit;
//! - the [`NotebookConfounderVisibility`] record that surfaces potential
//!   confounders (environment drift, dependency changes, dataset changes,
//!   hardware differences, kernel restarts, runtime parameter changes) with
//!   visibility classes so users know why results may differ;
//! - the [`NotebookResultComparisonPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every result-comparison record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookResultComparison`] payloads.
pub const NOTEBOOK_RESULT_COMPARISON_RECORD_KIND: &str = "notebook_result_comparison";

/// Stable record-kind tag for serialized [`NotebookBaselineSelection`] payloads.
pub const NOTEBOOK_BASELINE_SELECTION_RECORD_KIND: &str = "notebook_baseline_selection";

/// Stable record-kind tag for serialized [`NotebookConfounderVisibility`] payloads.
pub const NOTEBOOK_CONFOUNDER_VISIBILITY_RECORD_KIND: &str = "notebook_confounder_visibility";

/// Stable record-kind tag for the checked-in [`NotebookResultComparisonPacket`].
pub const NOTEBOOK_RESULT_COMPARISON_PACKET_RECORD_KIND: &str = "notebook_result_comparison_packet";

/// Repo-relative path to the checked-in result-comparison packet JSON.
pub const NOTEBOOK_RESULT_COMPARISON_PACKET_PATH: &str =
    "artifacts/notebook/m5/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.json";

/// Embedded checked-in result-comparison packet JSON.
pub const NOTEBOOK_RESULT_COMPARISON_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Comparison-mode class. Names whether the comparison is cell-aware,
    /// output-aware, summary-only, or forced to raw JSON fallback.
    NotebookComparisonMode {
        CellAware => "cell_aware",
        OutputAware => "output_aware",
        SummaryOnly => "summary_only",
        RawJsonFallback => "raw_json_fallback",
    }
);

closed_vocab!(
    /// Comparison-outcome class. Names the result of comparing baseline and
    /// current runs.
    NotebookComparisonOutcomeClass {
        Equivalent => "equivalent",
        Different => "different",
        BaselineMissing => "baseline_missing",
        CurrentMissing => "current_missing",
        Incomparable => "incomparable",
    }
);

closed_vocab!(
    /// Comparison-scope class. Names whether the comparison covers the full
    /// notebook, a selection of cells, or only the active cell.
    NotebookComparisonScopeClass {
        FullNotebook => "full_notebook",
        SelectedCells => "selected_cells",
        ActiveCell => "active_cell",
    }
);

closed_vocab!(
    /// Baseline-source class. Names how the baseline run was chosen.
    NotebookBaselineSourceClass {
        LastSuccessfulRun => "last_successful_run",
        PinnedExperiment => "pinned_experiment",
        TaggedCommit => "tagged_commit",
        ManualUpload => "manual_upload",
        WorkspaceSnapshot => "workspace_snapshot",
    }
);

closed_vocab!(
    /// Baseline-selection-state class. Names the lifecycle state of the
    /// baseline selection.
    NotebookBaselineSelectionState {
        Selected => "selected",
        Stale => "stale",
        Unavailable => "unavailable",
        Ambiguous => "ambiguous",
        ExplicitNone => "explicit_none",
    }
);

closed_vocab!(
    /// Confounder-class class. Names the kind of confounder that may explain
    /// a result difference independent of source changes.
    NotebookConfounderClass {
        EnvironmentDrift => "environment_drift",
        DependencyChange => "dependency_change",
        DatasetChange => "dataset_change",
        HardwareDifference => "hardware_difference",
        KernelRestart => "kernel_restart",
        RuntimeParameterChange => "runtime_parameter_change",
        Unspecified => "unspecified",
    }
);

closed_vocab!(
    /// Confounder-visibility class. Names whether the confounder is visible
    /// to the reviewer, suppressed, unknown, or not applicable.
    NotebookConfounderVisibilityClass {
        Visible => "visible",
        Suppressed => "suppressed",
        Unknown => "unknown",
        NotApplicable => "not_applicable",
    }
);

/// Generic finding shape used by every result-comparison validator. Mirrors
/// the finding shapes other Aureline crates expose so a single review/audit/
/// support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultComparisonFinding {
    /// Stable check id (e.g. `notebook_result_comparison.baseline_run_ref_required`).
    pub check_id: String,
    /// Subject row id (record id, comparison id, baseline id, confounder id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl ResultComparisonFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookResultComparison`].
pub type NotebookResultComparisonFinding = ResultComparisonFinding;

/// Typed validation finding for a [`NotebookBaselineSelection`].
pub type NotebookBaselineSelectionFinding = ResultComparisonFinding;

/// Typed validation finding for a [`NotebookConfounderVisibility`].
pub type NotebookConfounderVisibilityFinding = ResultComparisonFinding;

/// Typed validation finding for a [`NotebookResultComparisonPacket`].
pub type NotebookResultComparisonPacketFinding = ResultComparisonFinding;

/// Notebook result-comparison record. Carries a cell-aware comparison between
/// a baseline run and a current run, with mode, scope, outcome, and confounder
/// refs so reviewers never silently assume code is the only variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookResultComparison {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_result_comparison_schema_version: u32,
    /// Stable opaque comparison id.
    pub comparison_id: String,
    /// Opaque ref to the notebook document that owns this comparison.
    pub document_id_ref: String,
    /// Opaque ref to the baseline run.
    pub baseline_run_ref: String,
    /// Opaque ref to the current run.
    pub current_run_ref: String,
    /// Comparison-mode class.
    pub comparison_mode: NotebookComparisonMode,
    /// Comparison-scope class.
    pub comparison_scope: NotebookComparisonScopeClass,
    /// Comparison-outcome class.
    pub outcome_class: NotebookComparisonOutcomeClass,
    /// Opaque refs to [`NotebookConfounderVisibility`] records that explain
    /// detected differences.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub confounder_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookResultComparison {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookResultComparisonFinding> {
        let mut findings = Vec::new();
        let subject = self.comparison_id.as_str();

        if self.record_kind != NOTEBOOK_RESULT_COMPARISON_RECORD_KIND {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_RESULT_COMPARISON_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_result_comparison_schema_version
            != NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION
        {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION}, found {}",
                    self.notebook_result_comparison_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.baseline_run_ref.trim().is_empty() {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.baseline_run_ref_required",
                subject,
                "baseline_run_ref must be non-empty",
            ));
        }
        if self.current_run_ref.trim().is_empty() {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.current_run_ref_required",
                subject,
                "current_run_ref must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        if self.outcome_class == NotebookComparisonOutcomeClass::Different
            && self.confounder_refs.is_empty()
        {
            findings.push(NotebookResultComparisonFinding::new(
                "notebook_result_comparison.confounder_refs_required_when_different",
                subject,
                "confounder_refs must not be empty when outcome_class is different",
            ));
        }

        findings
    }
}

/// Notebook baseline-selection record. Names how the baseline was chosen,
/// its source, state, and pinning actor so the comparison boundary is explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookBaselineSelection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_result_comparison_schema_version: u32,
    /// Stable opaque selection id.
    pub selection_id: String,
    /// Opaque ref to the notebook document that owns this selection.
    pub document_id_ref: String,
    /// Baseline-source class.
    pub baseline_source: NotebookBaselineSourceClass,
    /// Baseline-selection-state class.
    pub selection_state: NotebookBaselineSelectionState,
    /// Opaque ref to the baseline run, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub baseline_run_ref: Option<String>,
    /// Opaque ref to the actor that pinned this baseline, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_by_actor_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookBaselineSelection {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookBaselineSelectionFinding> {
        let mut findings = Vec::new();
        let subject = self.selection_id.as_str();

        if self.record_kind != NOTEBOOK_BASELINE_SELECTION_RECORD_KIND {
            findings.push(NotebookBaselineSelectionFinding::new(
                "notebook_baseline_selection.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_BASELINE_SELECTION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_result_comparison_schema_version
            != NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION
        {
            findings.push(NotebookBaselineSelectionFinding::new(
                "notebook_baseline_selection.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION}, found {}",
                    self.notebook_result_comparison_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookBaselineSelectionFinding::new(
                "notebook_baseline_selection.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }

        match self.selection_state {
            NotebookBaselineSelectionState::Selected | NotebookBaselineSelectionState::Stale => {
                if self
                    .baseline_run_ref
                    .as_ref()
                    .map(|s| s.trim().is_empty())
                    .unwrap_or(true)
                {
                    findings.push(NotebookBaselineSelectionFinding::new(
                        "notebook_baseline_selection.baseline_run_ref_required_when_selected_or_stale",
                        subject,
                        "baseline_run_ref must be non-empty when selection_state is selected or stale",
                    ));
                }
            }
            NotebookBaselineSelectionState::Unavailable
            | NotebookBaselineSelectionState::ExplicitNone => {
                if self.baseline_run_ref.is_some() {
                    findings.push(NotebookBaselineSelectionFinding::new(
                        "notebook_baseline_selection.baseline_run_ref_must_be_none_when_unavailable_or_explicit_none",
                        subject,
                        "baseline_run_ref must be None when selection_state is unavailable or explicit_none",
                    ));
                }
            }
            NotebookBaselineSelectionState::Ambiguous => {
                // baseline_run_ref may be present or absent.
            }
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookBaselineSelectionFinding::new(
                "notebook_baseline_selection.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook confounder-visibility record. Surfaces potential confounders that
/// may explain a result difference independent of source changes, with
/// visibility classes so users know why results may differ.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookConfounderVisibility {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_result_comparison_schema_version: u32,
    /// Stable opaque confounder id.
    pub confounder_id: String,
    /// Opaque ref to the notebook document that owns this confounder.
    pub document_id_ref: String,
    /// Confounder class.
    pub confounder_class: NotebookConfounderClass,
    /// Confounder-visibility class.
    pub visibility_class: NotebookConfounderVisibilityClass,
    /// Opaque refs to evidence records (e.g. environment fingerprint, dataset
    /// card) that support this confounder.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookConfounderVisibility {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookConfounderVisibilityFinding> {
        let mut findings = Vec::new();
        let subject = self.confounder_id.as_str();

        if self.record_kind != NOTEBOOK_CONFOUNDER_VISIBILITY_RECORD_KIND {
            findings.push(NotebookConfounderVisibilityFinding::new(
                "notebook_confounder_visibility.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CONFOUNDER_VISIBILITY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_result_comparison_schema_version
            != NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION
        {
            findings.push(NotebookConfounderVisibilityFinding::new(
                "notebook_confounder_visibility.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION}, found {}",
                    self.notebook_result_comparison_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookConfounderVisibilityFinding::new(
                "notebook_confounder_visibility.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.summary.trim().is_empty() {
            findings.push(NotebookConfounderVisibilityFinding::new(
                "notebook_confounder_visibility.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in result-comparison/baseline/confounder packet that downstream
/// surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookResultComparisonPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: comparison modes.
    pub comparison_modes: Vec<NotebookComparisonMode>,
    /// Closed vocabulary: comparison outcomes.
    pub comparison_outcomes: Vec<NotebookComparisonOutcomeClass>,
    /// Closed vocabulary: comparison scopes.
    pub comparison_scopes: Vec<NotebookComparisonScopeClass>,
    /// Closed vocabulary: baseline sources.
    pub baseline_sources: Vec<NotebookBaselineSourceClass>,
    /// Closed vocabulary: selection states.
    pub selection_states: Vec<NotebookBaselineSelectionState>,
    /// Closed vocabulary: confounder classes.
    pub confounder_classes: Vec<NotebookConfounderClass>,
    /// Closed vocabulary: visibility classes.
    pub visibility_classes: Vec<NotebookConfounderVisibilityClass>,
    /// Worked example comparisons.
    pub example_comparisons: Vec<NotebookResultComparison>,
    /// Worked example baseline selections.
    pub example_baselines: Vec<NotebookBaselineSelection>,
    /// Worked example confounder visibility records.
    pub example_confounders: Vec<NotebookConfounderVisibility>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookResultComparisonPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookResultComparisonPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RESULT_COMPARISON_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_RESULT_COMPARISON_PACKET_RECORD_KIND {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_RESULT_COMPARISON_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.comparison_modes.len() != NotebookComparisonMode::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.comparison_modes_coverage",
                subject,
                "comparison_modes must list every variant",
            ));
        }
        if self.comparison_outcomes.len() != NotebookComparisonOutcomeClass::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.comparison_outcomes_coverage",
                subject,
                "comparison_outcomes must list every variant",
            ));
        }
        if self.comparison_scopes.len() != NotebookComparisonScopeClass::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.comparison_scopes_coverage",
                subject,
                "comparison_scopes must list every variant",
            ));
        }
        if self.baseline_sources.len() != NotebookBaselineSourceClass::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.baseline_sources_coverage",
                subject,
                "baseline_sources must list every variant",
            ));
        }
        if self.selection_states.len() != NotebookBaselineSelectionState::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.selection_states_coverage",
                subject,
                "selection_states must list every variant",
            ));
        }
        if self.confounder_classes.len() != NotebookConfounderClass::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.confounder_classes_coverage",
                subject,
                "confounder_classes must list every variant",
            ));
        }
        if self.visibility_classes.len() != NotebookConfounderVisibilityClass::ALL.len() {
            findings.push(NotebookResultComparisonPacketFinding::new(
                "notebook_result_comparison_packet.visibility_classes_coverage",
                subject,
                "visibility_classes must list every variant",
            ));
        }

        for comparison in &self.example_comparisons {
            findings.extend(comparison.validate().into_iter().map(|f| {
                NotebookResultComparisonPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for baseline in &self.example_baselines {
            findings.extend(baseline.validate().into_iter().map(|f| {
                NotebookResultComparisonPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for confounder in &self.example_confounders {
            findings.extend(confounder.validate().into_iter().map(|f| {
                NotebookResultComparisonPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in result-comparison packet JSON.
pub fn current_notebook_result_comparison_packet(
) -> Result<NotebookResultComparisonPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_RESULT_COMPARISON_PACKET_JSON)
}

impl NotebookComparisonMode {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CellAware,
        Self::OutputAware,
        Self::SummaryOnly,
        Self::RawJsonFallback,
    ];
}

impl NotebookComparisonOutcomeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Equivalent,
        Self::Different,
        Self::BaselineMissing,
        Self::CurrentMissing,
        Self::Incomparable,
    ];
}

impl NotebookComparisonScopeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::FullNotebook, Self::SelectedCells, Self::ActiveCell];
}

impl NotebookBaselineSourceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LastSuccessfulRun,
        Self::PinnedExperiment,
        Self::TaggedCommit,
        Self::ManualUpload,
        Self::WorkspaceSnapshot,
    ];
}

impl NotebookBaselineSelectionState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Selected,
        Self::Stale,
        Self::Unavailable,
        Self::Ambiguous,
        Self::ExplicitNone,
    ];
}

impl NotebookConfounderClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::EnvironmentDrift,
        Self::DependencyChange,
        Self::DatasetChange,
        Self::HardwareDifference,
        Self::KernelRestart,
        Self::RuntimeParameterChange,
        Self::Unspecified,
    ];
}

impl NotebookConfounderVisibilityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Visible,
        Self::Suppressed,
        Self::Unknown,
        Self::NotApplicable,
    ];
}

#[cfg(test)]
mod tests;
