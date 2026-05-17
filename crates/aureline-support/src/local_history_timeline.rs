//! Support-export consumer for local-history compare and restore timeline rows.
//!
//! This module folds the checked-in local-history timeline corpus into a
//! metadata-safe support-export envelope. The rows quote the same fidelity
//! labels and action vocabulary as the timeline projection, so support export
//! cannot relabel evidence-only recovery as a live restore.

use std::error::Error;
use std::fmt;

use aureline_history::local_history::timeline::{
    current_local_history_timeline_corpus, LocalHistoryTimelineActionAvailability,
    LocalHistoryTimelineActionClass, LocalHistoryTimelineCase, LocalHistoryTimelineCorpus,
    LocalHistoryTimelineEvaluator, LocalHistoryTimelineFidelityLabel, LocalHistoryTimelineReport,
    LocalHistoryTimelineRestoreLevel, LocalHistoryTimelineValidationReport,
    LOCAL_HISTORY_TIMELINE_DOC_REF, LOCAL_HISTORY_TIMELINE_REPORT_REF,
    LOCAL_HISTORY_TIMELINE_SCHEMA_REF,
};
use aureline_history::ActorLineageClass;
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export timeline row.
pub const LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "local_history_timeline_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "local_history_timeline_support_export_envelope";

/// One metadata-safe support-export row for a local-history timeline entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineSupportExportRow {
    /// Stable discriminator for this row.
    pub record_kind: String,
    /// Timeline row id.
    pub row_id: String,
    /// Source local-history entry ref.
    pub source_entry_ref: String,
    /// Optional local-history group ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_group_ref: Option<String>,
    /// Actor lineage class rendered on the timeline.
    pub actor_lineage_class: ActorLineageClass,
    /// Shared row fidelity label.
    pub fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Restore level paired with the fidelity label.
    pub restore_level: LocalHistoryTimelineRestoreLevel,
    /// Compare action fidelity label.
    pub compare_fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Restore action fidelity label.
    pub restore_fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Export action fidelity label.
    pub export_fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Compare action availability.
    pub compare_action_availability: LocalHistoryTimelineActionAvailability,
    /// Restore action availability.
    pub restore_action_availability: LocalHistoryTimelineActionAvailability,
    /// Export action availability.
    pub export_action_availability: LocalHistoryTimelineActionAvailability,
    /// Optional support-export ref on the export action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_export_ref: Option<String>,
    /// True only when the original row claimed a live session resumed.
    pub live_session_resumed: bool,
    /// True only when the original row claimed a privileged run resumed.
    pub privileged_run_resumed: bool,
    /// True when the row's support projection excludes raw payloads.
    pub raw_payload_excluded: bool,
    /// True when the row's support projection excludes private material.
    pub raw_private_material_excluded: bool,
    /// True when the row's support projection excludes live authority.
    pub live_authority_excluded: bool,
}

impl LocalHistoryTimelineSupportExportRow {
    fn from_case(case: &LocalHistoryTimelineCase) -> Self {
        let row = &case.timeline_row;
        let compare = action_for(case, LocalHistoryTimelineActionClass::Compare);
        let restore = action_for(case, LocalHistoryTimelineActionClass::Restore);
        let export = action_for(case, LocalHistoryTimelineActionClass::Export);
        Self {
            record_kind: LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            row_id: row.row_id.clone(),
            source_entry_ref: row.source_entry_ref.clone(),
            source_group_ref: row.source_group_ref.clone(),
            actor_lineage_class: row.actor_lineage_class,
            fidelity_label: row.fidelity_label,
            restore_level: row.restore_level,
            compare_fidelity_label: compare.fidelity_label,
            restore_fidelity_label: restore.fidelity_label,
            export_fidelity_label: export.fidelity_label,
            compare_action_availability: compare.availability_class,
            restore_action_availability: restore.availability_class,
            export_action_availability: export.availability_class,
            support_export_ref: export.support_export_ref.clone(),
            live_session_resumed: row.no_rerun_guard.live_session_resumed,
            privileged_run_resumed: row.no_rerun_guard.privileged_run_resumed,
            raw_payload_excluded: row.support_export.raw_payload_excluded,
            raw_private_material_excluded: row.support_export.raw_private_material_excluded,
            live_authority_excluded: row.support_export.live_authority_excluded,
        }
    }

    /// Returns true when support export can include this row without overclaiming restore.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.live_authority_excluded
            && !self.live_session_resumed
            && !self.privileged_run_resumed
            && self.compare_fidelity_label == self.fidelity_label
            && self.restore_fidelity_label == self.fidelity_label
            && self.export_fidelity_label == self.fidelity_label
            && self.support_export_ref.is_some()
    }
}

/// Support-export envelope folded from the local-history timeline corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineSupportExportEnvelope {
    /// Stable discriminator for this envelope.
    pub record_kind: String,
    /// Opaque envelope id.
    pub envelope_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Baseline report ref.
    pub report_ref: String,
    /// True when raw local-history payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when live authority or privilege handles are excluded.
    pub live_authority_excluded: bool,
    /// Support-export rows.
    pub rows: Vec<LocalHistoryTimelineSupportExportRow>,
}

impl LocalHistoryTimelineSupportExportEnvelope {
    /// Builds a support-export envelope from a validated report and corpus.
    pub fn from_report(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        report: &LocalHistoryTimelineReport,
        corpus: &LocalHistoryTimelineCorpus,
    ) -> Self {
        let mut rows: Vec<LocalHistoryTimelineSupportExportRow> = corpus
            .entries
            .iter()
            .map(|entry| LocalHistoryTimelineSupportExportRow::from_case(&entry.case))
            .collect();
        rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));
        Self {
            record_kind: LOCAL_HISTORY_TIMELINE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: report.doc_ref.clone(),
            schema_ref: report.schema_ref.clone(),
            report_ref: LOCAL_HISTORY_TIMELINE_REPORT_REF.to_owned(),
            raw_payload_excluded: report.raw_payload_excluded,
            raw_private_material_excluded: report.raw_private_material_excluded,
            live_authority_excluded: report.live_authority_excluded,
            rows,
        }
    }

    /// Returns true when the envelope is metadata-safe and vocabulary-aligned.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.live_authority_excluded
            && self.doc_ref == LOCAL_HISTORY_TIMELINE_DOC_REF
            && self.schema_ref == LOCAL_HISTORY_TIMELINE_SCHEMA_REF
            && self.report_ref == LOCAL_HISTORY_TIMELINE_REPORT_REF
            && !self.rows.is_empty()
            && self.rows.iter().all(|row| row.is_export_safe())
    }
}

/// Error returned when compiling the local-history timeline support envelope fails.
#[derive(Debug)]
pub enum LocalHistoryTimelineSupportExportError {
    /// The checked-in YAML corpus did not parse.
    CorpusParse(serde_yaml::Error),
    /// The checked-in corpus failed timeline validation.
    CorpusValidation(LocalHistoryTimelineValidationReport),
}

impl fmt::Display for LocalHistoryTimelineSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorpusParse(err) => write!(f, "local-history timeline corpus parse: {err}"),
            Self::CorpusValidation(report) => {
                write!(f, "local-history timeline corpus invalid: {report}")
            }
        }
    }
}

impl Error for LocalHistoryTimelineSupportExportError {}

impl From<serde_yaml::Error> for LocalHistoryTimelineSupportExportError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::CorpusParse(err)
    }
}

impl From<LocalHistoryTimelineValidationReport> for LocalHistoryTimelineSupportExportError {
    fn from(err: LocalHistoryTimelineValidationReport) -> Self {
        Self::CorpusValidation(err)
    }
}

/// Compiles the support-export envelope from the checked-in timeline corpus.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<LocalHistoryTimelineSupportExportEnvelope, LocalHistoryTimelineSupportExportError> {
    let envelope_id = envelope_id.into();
    let captured_at = captured_at.into();
    let corpus = current_local_history_timeline_corpus()?;
    let report = LocalHistoryTimelineEvaluator::new().report(
        format!("{envelope_id}:report"),
        captured_at.clone(),
        &corpus,
    )?;
    Ok(LocalHistoryTimelineSupportExportEnvelope::from_report(
        envelope_id,
        captured_at,
        &report,
        &corpus,
    ))
}

fn action_for(
    case: &LocalHistoryTimelineCase,
    class: LocalHistoryTimelineActionClass,
) -> &aureline_history::LocalHistoryTimelineAction {
    case.timeline_row
        .actions
        .iter()
        .find(|action| action.action_class == class)
        .expect("validated local-history timeline rows carry every required action")
}
