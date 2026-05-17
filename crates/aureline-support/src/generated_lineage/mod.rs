//! Support-export consumer for generated-artifact lineage packets.
//!
//! This module is the first non-state consumer of the
//! [`aureline_reactive_state::generated_lineage`] projection. It folds
//! the checked-in lineage corpus into a typed support-export envelope
//! so the local-first support pipeline can quote the same artifact
//! family, lineage class, drift state, default edit posture, generator
//! identity, and canonical source refs the in-product chrome renders,
//! without re-running generators and without forcing raw payload
//! capture.
//!
//! The envelope is metadata-safe by construction: it only ever holds
//! tokens drawn from the closed lineage-packet vocabularies, the
//! generator identity, the source refs back to the canonical local
//! files, and the safety baseline the lineage packet declared.

use std::error::Error;
use std::fmt;

use aureline_reactive_state::generated_lineage::{
    current_generated_artifact_lineage_corpus, ArtifactFamily, DefaultEditPosture, DriftState,
    GeneratedArtifactLineageCorpus, GeneratedArtifactLineageEvaluator,
    GeneratedArtifactLineagePacket, GeneratedArtifactLineageReport,
    GeneratedArtifactLineageValidationReport, GeneratorIdentity, LineageClass,
    LineageConsumerSurface, LineageDowngradeLabel, LineageOpenGapClass, LineageSourceRef,
    GENERATED_ARTIFACT_LINEAGE_DOC_REF, GENERATED_ARTIFACT_LINEAGE_REPORT_REF,
    GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for the support-export envelope row.
pub const GENERATED_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "generated_lineage_support_export_row";

/// Stable record-kind tag for the support-export envelope itself.
pub const GENERATED_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "generated_lineage_support_export_envelope";

/// One row of the support-export envelope. Quotes the closed-vocabulary
/// tokens the lineage packet declared so the support pipeline can audit
/// generated-artifact lineage without re-running generators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedLineageSupportExportRow {
    pub record_kind: String,
    pub packet_id: String,
    pub consumer_surface: LineageConsumerSurface,
    pub artifact_family: ArtifactFamily,
    pub artifact_ref: String,
    pub workspace_id: String,
    pub lineage_class: LineageClass,
    pub drift_state: DriftState,
    pub default_edit_posture: DefaultEditPosture,
    pub downgrade_label: LineageDowngradeLabel,
    pub generator_identity: GeneratorIdentity,
    pub source_refs: Vec<LineageSourceRef>,
    pub open_gap_classes: Vec<LineageOpenGapClass>,
}

impl GeneratedLineageSupportExportRow {
    fn from_packet(packet: &GeneratedArtifactLineagePacket) -> Self {
        let mut open_gap_classes: Vec<LineageOpenGapClass> =
            packet.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(LineageOpenGapClass::None);
        }
        Self {
            record_kind: GENERATED_LINEAGE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            packet_id: packet.packet_id.clone(),
            consumer_surface: packet.consumer_surface,
            artifact_family: packet.artifact_family,
            artifact_ref: packet.artifact_ref.clone(),
            workspace_id: packet.workspace_id.clone(),
            lineage_class: packet.lineage_class,
            drift_state: packet.drift_state,
            default_edit_posture: packet.default_edit_posture,
            downgrade_label: packet.downgrade_label,
            generator_identity: packet.generator_identity.clone(),
            source_refs: packet.source_refs.clone(),
            open_gap_classes,
        }
    }
}

/// Support-export envelope folded from the checked-in lineage corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedLineageSupportExportEnvelope {
    pub record_kind: String,
    pub envelope_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub rows: Vec<GeneratedLineageSupportExportRow>,
}

impl GeneratedLineageSupportExportEnvelope {
    /// Builds the envelope by folding the [`GeneratedArtifactLineageReport`]
    /// matrix rows into one support-export row per packet.
    pub fn from_report(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        report: &GeneratedArtifactLineageReport,
        corpus: &GeneratedArtifactLineageCorpus,
    ) -> Self {
        let mut rows: Vec<GeneratedLineageSupportExportRow> = corpus
            .entries
            .iter()
            .map(|entry| GeneratedLineageSupportExportRow::from_packet(&entry.packet))
            .collect();
        rows.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));
        Self {
            record_kind: GENERATED_LINEAGE_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: report.doc_ref.clone(),
            schema_ref: report.schema_ref.clone(),
            report_ref: GENERATED_ARTIFACT_LINEAGE_REPORT_REF.to_owned(),
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
            && self.doc_ref == GENERATED_ARTIFACT_LINEAGE_DOC_REF
            && self.schema_ref == GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF
            && self.report_ref == GENERATED_ARTIFACT_LINEAGE_REPORT_REF
            && !self.rows.is_empty()
    }
}

/// Error returned when the support-export pipeline cannot compile a
/// lineage envelope from the checked-in corpus.
#[derive(Debug)]
pub enum GeneratedLineageSupportExportError {
    CorpusParse(serde_yaml::Error),
    CorpusValidation(GeneratedArtifactLineageValidationReport),
}

impl fmt::Display for GeneratedLineageSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CorpusParse(err) => write!(f, "generated lineage corpus parse: {err}"),
            Self::CorpusValidation(report) => {
                write!(f, "generated lineage corpus invalid: {report}")
            }
        }
    }
}

impl Error for GeneratedLineageSupportExportError {}

impl From<serde_yaml::Error> for GeneratedLineageSupportExportError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::CorpusParse(err)
    }
}

impl From<GeneratedArtifactLineageValidationReport> for GeneratedLineageSupportExportError {
    fn from(err: GeneratedArtifactLineageValidationReport) -> Self {
        Self::CorpusValidation(err)
    }
}

/// Compiles the support-export envelope from the checked-in
/// generated-artifact lineage corpus. Support pipelines call this to
/// load lineage truth into a metadata-safe envelope without re-running
/// generators.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<GeneratedLineageSupportExportEnvelope, GeneratedLineageSupportExportError> {
    let envelope_id = envelope_id.into();
    let captured_at = captured_at.into();
    let corpus = current_generated_artifact_lineage_corpus()?;
    let report = GeneratedArtifactLineageEvaluator::new().report(
        format!("{envelope_id}:report"),
        captured_at.clone(),
        &corpus,
    )?;
    Ok(GeneratedLineageSupportExportEnvelope::from_report(
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
            "envelope:generated_lineage:test",
            "2026-05-16T10:00:00Z",
        )
        .expect("envelope compiles");
        assert!(envelope.is_export_safe());
        assert!(!envelope.rows.is_empty());

        let json = serde_json::to_string(&envelope).expect("envelope serializes");
        let parsed: GeneratedLineageSupportExportEnvelope =
            serde_json::from_str(&json).expect("envelope round-trips");
        assert_eq!(parsed, envelope);
    }
}
