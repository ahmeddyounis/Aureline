//! Support-export consumer for mutation-journal beta entries.
//!
//! This module is the first non-state consumer of the
//! [`aureline_reactive_state::mutation_journal`] projection. It folds
//! the checked-in mutation-journal corpus into a typed support-export
//! envelope so the local-first support pipeline can quote the same
//! source lane, actor, authority, recovery class, replayability state,
//! affected paths, and downgrade label the in-product chrome renders,
//! without re-reading raw diffs and without forcing raw payload
//! capture.
//!
//! The envelope is metadata-safe by construction: it only ever holds
//! tokens drawn from the closed mutation-journal vocabularies, the
//! grouped entry's repo-relative path refs, and the safety baseline
//! the journal case declared.

use std::error::Error;
use std::fmt;

use aureline_reactive_state::mutation_journal::{
    current_mutation_journal_corpus, ActorClass, AttributionState, AuthorityClass,
    ConsumerSurface, DowngradeLabel, EntryKind, MutationJournalCase, MutationJournalCorpus,
    MutationJournalEvaluator, MutationJournalReport, MutationJournalValidationReport,
    OpenGapClass, RecoveryClass, ReplayabilityState, SourceLane, MUTATION_JOURNAL_DOC_REF,
    MUTATION_JOURNAL_REPORT_REF, MUTATION_JOURNAL_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export envelope row.
pub const MUTATION_JOURNAL_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "mutation_journal_support_export_row";

/// Stable record-kind tag for the support-export envelope itself.
pub const MUTATION_JOURNAL_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "mutation_journal_support_export_envelope";

/// One row of the support-export envelope. Quotes the closed
/// mutation-journal tokens, the grouped entry's repo-relative paths,
/// and the open-gap classes so the support pipeline can explain what
/// changed without re-reading raw diffs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalSupportExportRow {
    pub record_kind: String,
    pub entry_id: String,
    pub consumer_surface: ConsumerSurface,
    pub source_lane: SourceLane,
    pub actor_class: ActorClass,
    pub authority_class: AuthorityClass,
    pub entry_kind: EntryKind,
    pub group_size: u32,
    pub affected_paths: Vec<String>,
    pub recovery_class: RecoveryClass,
    pub attribution_state: AttributionState,
    pub replayability_state: ReplayabilityState,
    pub downgrade_label: DowngradeLabel,
    pub open_gap_classes: Vec<OpenGapClass>,
}

impl MutationJournalSupportExportRow {
    fn from_case(case: &MutationJournalCase) -> Self {
        let mut open_gap_classes: Vec<OpenGapClass> =
            case.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(OpenGapClass::None);
        }
        Self {
            record_kind: MUTATION_JOURNAL_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            entry_id: case.entry_id.clone(),
            consumer_surface: case.consumer_surface,
            source_lane: case.source_lane,
            actor_class: case.actor_class,
            authority_class: case.authority_class,
            entry_kind: case.entry_kind,
            group_size: case.group_size,
            affected_paths: case.affected_paths.clone(),
            recovery_class: case.recovery_class,
            attribution_state: case.attribution_state,
            replayability_state: case.replayability_state,
            downgrade_label: case.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Support-export envelope folded from the checked-in mutation-journal
/// corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalSupportExportEnvelope {
    pub record_kind: String,
    pub envelope_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub rows: Vec<MutationJournalSupportExportRow>,
}

impl MutationJournalSupportExportEnvelope {
    /// Builds the envelope by folding the [`MutationJournalReport`]
    /// matrix rows into one support-export row per case.
    pub fn from_report(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        report: &MutationJournalReport,
        corpus: &MutationJournalCorpus,
    ) -> Self {
        let mut rows: Vec<MutationJournalSupportExportRow> = corpus
            .entries
            .iter()
            .map(|entry| MutationJournalSupportExportRow::from_case(&entry.case))
            .collect();
        rows.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));
        Self {
            record_kind: MUTATION_JOURNAL_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: report.doc_ref.clone(),
            schema_ref: report.schema_ref.clone(),
            report_ref: MUTATION_JOURNAL_REPORT_REF.to_owned(),
            raw_payload_excluded: report.raw_payload_excluded,
            raw_private_material_excluded: report.raw_private_material_excluded,
            ambient_authority_excluded: report.ambient_authority_excluded,
            rows,
        }
    }

    /// Returns true when the envelope is metadata-safe.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == MUTATION_JOURNAL_DOC_REF
            && self.schema_ref == MUTATION_JOURNAL_SCHEMA_REF
            && self.report_ref == MUTATION_JOURNAL_REPORT_REF
            && !self.rows.is_empty()
    }
}

/// Error returned when the support-export pipeline cannot compile a
/// mutation-journal envelope from the checked-in corpus.
#[derive(Debug)]
pub enum MutationJournalSupportExportError {
    CorpusParse(serde_yaml::Error),
    CorpusValidation(MutationJournalValidationReport),
}

impl fmt::Display for MutationJournalSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorpusParse(err) => write!(f, "mutation journal corpus parse: {err}"),
            Self::CorpusValidation(report) => {
                write!(f, "mutation journal corpus invalid: {report}")
            }
        }
    }
}

impl Error for MutationJournalSupportExportError {}

impl From<serde_yaml::Error> for MutationJournalSupportExportError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::CorpusParse(err)
    }
}

impl From<MutationJournalValidationReport> for MutationJournalSupportExportError {
    fn from(err: MutationJournalValidationReport) -> Self {
        Self::CorpusValidation(err)
    }
}

/// Compiles the support-export envelope from the checked-in
/// mutation-journal corpus. Support pipelines call this to load
/// grouped-write attribution and recovery truth into a metadata-safe
/// envelope without re-reading the underlying diff.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<MutationJournalSupportExportEnvelope, MutationJournalSupportExportError> {
    let envelope_id = envelope_id.into();
    let captured_at = captured_at.into();
    let corpus = current_mutation_journal_corpus()?;
    let report = MutationJournalEvaluator::new().report(
        format!("{envelope_id}:report"),
        captured_at.clone(),
        &corpus,
    )?;
    Ok(MutationJournalSupportExportEnvelope::from_report(
        envelope_id,
        captured_at,
        &report,
        &corpus,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_envelope_round_trip() {
        let envelope = compile_support_export_envelope(
            "envelope:mutation_journal:test",
            "2026-05-16T10:30:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert!(!envelope.rows.is_empty());

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: MutationJournalSupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }
}
