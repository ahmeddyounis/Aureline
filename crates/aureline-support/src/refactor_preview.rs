//! Support-export projection for launch-language refactor preview evidence.
//!
//! The projection folds the checked-in
//! [`aureline_language::refactor_preview`] corpus into metadata-safe support
//! rows. It preserves the same green, downgraded, and unsupported row states,
//! fallback labels, validation states, rollback handles, local-history group
//! refs, and mutation-journal refs that the language preview validator checks.

use std::error::Error;
use std::fmt;

use aureline_language::refactor_preview::{
    current_refactor_preview_corpus, RefactorClass, RefactorConfidenceClass,
    RefactorCorpusRowState, RefactorPreviewBetaReport, RefactorPreviewCorpus,
    RefactorPreviewCorpusValidationReport, RefactorPreviewEvaluator,
    RefactorRollbackDrillOutcomeClass, RefactorRuntimeConditionClass, RefactorSemanticSourceClass,
    RefactorSupportClaimClass, RefactorValidationStateClass, REFACTOR_PREVIEW_BETA_DOC_REF,
    REFACTOR_PREVIEW_SCHEMA_REF, REFACTOR_VALIDATION_RESULT_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one support-export row.
pub const REFACTOR_PREVIEW_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "language_refactor_preview_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const REFACTOR_PREVIEW_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "language_refactor_preview_support_export_envelope";

/// One metadata-safe support row for a refactor preview corpus entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewSupportExportRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Preview id.
    pub preview_id: String,
    /// Launch-language row ref.
    pub language_row_ref: String,
    /// Refactor class.
    pub refactor_class: RefactorClass,
    /// Support claim posture.
    pub support_claim_class: RefactorSupportClaimClass,
    /// Runtime condition.
    pub runtime_condition_class: RefactorRuntimeConditionClass,
    /// Shiproom row state.
    pub corpus_row_state: RefactorCorpusRowState,
    /// Source class used to assemble the preview.
    pub semantic_source_class: RefactorSemanticSourceClass,
    /// Confidence tier.
    pub confidence_class: RefactorConfidenceClass,
    /// Fallback or source label rendered for the row.
    pub fallback_label: String,
    /// Affected file count.
    pub affected_file_count: u32,
    /// Affected symbol count.
    pub affected_symbol_count: u32,
    /// Validation state.
    pub validation_state_class: RefactorValidationStateClass,
    /// Rollback drill outcome.
    pub rollback_drill_outcome_class: RefactorRollbackDrillOutcomeClass,
    /// Rollback handle ref when available.
    pub rollback_handle_ref: Option<String>,
    /// Local-history group ref when available.
    pub local_history_group_ref: Option<String>,
    /// Mutation-journal group ref when available.
    pub mutation_journal_ref: Option<String>,
    /// Support-export ref shared with the language validation result.
    pub support_export_ref: String,
}

impl RefactorPreviewSupportExportRow {
    fn from_entry(entry: &aureline_language::refactor_preview::RefactorPreviewCorpusEntry) -> Self {
        let preview = &entry.preview;
        let result = &entry.validation_result;
        Self {
            record_kind: REFACTOR_PREVIEW_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            preview_id: preview.preview_id.clone(),
            language_row_ref: preview.language_row_ref.clone(),
            refactor_class: preview.refactor_class,
            support_claim_class: preview.support_claim_class,
            runtime_condition_class: preview.runtime_condition_class,
            corpus_row_state: result.corpus_row_state,
            semantic_source_class: preview.semantic_source_class,
            confidence_class: preview.confidence_class,
            fallback_label: preview.fallback_label.label.clone(),
            affected_file_count: preview.target_set.affected_file_count,
            affected_symbol_count: preview.target_set.affected_symbol_count,
            validation_state_class: result.validation_state_class,
            rollback_drill_outcome_class: result.rollback_drill_outcome_class,
            rollback_handle_ref: preview.rollback_handle.rollback_handle_ref.clone(),
            local_history_group_ref: preview.rollback_handle.local_history_group_ref.clone(),
            mutation_journal_ref: preview.rollback_handle.mutation_journal_ref.clone(),
            support_export_ref: result.support_export_ref.clone(),
        }
    }
}

/// Metadata-safe envelope for refactor preview support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefactorPreviewSupportExportEnvelope {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Envelope id.
    pub envelope_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Documentation ref shared with the language report.
    pub doc_ref: String,
    /// Preview schema ref.
    pub preview_schema_ref: String,
    /// Validation-result schema ref.
    pub validation_result_schema_ref: String,
    /// True when raw patches and source bodies are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// Language corpus report folded into the export.
    pub report: RefactorPreviewBetaReport,
    /// One support row per corpus entry.
    pub rows: Vec<RefactorPreviewSupportExportRow>,
}

impl RefactorPreviewSupportExportEnvelope {
    /// Builds the support envelope from a validated language corpus report.
    pub fn from_report(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        report: RefactorPreviewBetaReport,
        corpus: &RefactorPreviewCorpus,
    ) -> Self {
        let mut rows = corpus
            .entries
            .iter()
            .map(RefactorPreviewSupportExportRow::from_entry)
            .collect::<Vec<_>>();
        rows.sort_by(|a, b| a.preview_id.cmp(&b.preview_id));
        Self {
            record_kind: REFACTOR_PREVIEW_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REFACTOR_PREVIEW_BETA_DOC_REF.to_owned(),
            preview_schema_ref: REFACTOR_PREVIEW_SCHEMA_REF.to_owned(),
            validation_result_schema_ref: REFACTOR_VALIDATION_RESULT_SCHEMA_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            report,
            rows,
        }
    }

    /// Returns true when the envelope is metadata-safe and quotes the corpus.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.doc_ref == REFACTOR_PREVIEW_BETA_DOC_REF
            && self.preview_schema_ref == REFACTOR_PREVIEW_SCHEMA_REF
            && self.validation_result_schema_ref == REFACTOR_VALIDATION_RESULT_SCHEMA_REF
            && self.report.is_export_safe()
            && !self.rows.is_empty()
            && self.rows.len() == self.report.rows.len()
    }
}

/// Error returned when the support export cannot compile.
#[derive(Debug)]
pub enum RefactorPreviewSupportExportError {
    /// Corpus YAML could not be parsed.
    CorpusParse(serde_yaml::Error),
    /// Corpus failed language validation.
    CorpusValidation(RefactorPreviewCorpusValidationReport),
}

impl fmt::Display for RefactorPreviewSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorpusParse(err) => write!(f, "refactor preview corpus parse: {err}"),
            Self::CorpusValidation(report) => {
                write!(f, "refactor preview corpus invalid: {report}")
            }
        }
    }
}

impl Error for RefactorPreviewSupportExportError {}

impl From<serde_yaml::Error> for RefactorPreviewSupportExportError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::CorpusParse(err)
    }
}

impl From<RefactorPreviewCorpusValidationReport> for RefactorPreviewSupportExportError {
    fn from(err: RefactorPreviewCorpusValidationReport) -> Self {
        Self::CorpusValidation(err)
    }
}

/// Compiles the support-export envelope from the checked-in language corpus.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<RefactorPreviewSupportExportEnvelope, RefactorPreviewSupportExportError> {
    let envelope_id = envelope_id.into();
    let captured_at = captured_at.into();
    let corpus = current_refactor_preview_corpus()?;
    let report = RefactorPreviewEvaluator::new().report(
        format!("{envelope_id}:report"),
        captured_at.clone(),
        &corpus,
    )?;
    Ok(RefactorPreviewSupportExportEnvelope::from_report(
        envelope_id,
        captured_at,
        report,
        &corpus,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_envelope_round_trips() {
        let envelope = compile_support_export_envelope(
            "envelope:language-refactor-preview:test",
            "2026-05-18T10:00:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: RefactorPreviewSupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope deserializes");
        assert_eq!(parsed, envelope);
    }
}
