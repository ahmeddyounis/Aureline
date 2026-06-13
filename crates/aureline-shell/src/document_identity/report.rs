//! Headless report for document-identity disclosures across shell surfaces.
//!
//! The report is the cross-surface inventory that ties together the
//! notebook lane, retained preview wedges, archive/review artifacts,
//! provider drafts, and export artifacts. It is intentionally
//! metadata-only and export-safe: each row points at a surface and
//! carries the shared [`super::DocumentIdentityDisclosure`] rather than
//! duplicating raw content or runtime state.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::document_identity::{
    DocumentFamilyClass, DocumentIdentityDisclosure, DocumentLabelClass, RootClassDisclosure,
    SaveTargetClass, WritePostureClass, DOCUMENT_IDENTITY_DISCLOSURE_SCHEMA_VERSION,
};
use crate::notebook_alpha::NotebookAlphaLaneRecord;
use crate::preview_truth::{seeded_notebook_preview_truth, seeded_structured_config_preview_truth};

/// Stable record-kind tag for [`DocumentIdentityReport`].
pub const DOCUMENT_IDENTITY_REPORT_RECORD_KIND: &str = "document_identity_report_record";

/// Stable record-kind tag for [`DocumentIdentitySupportExport`].
pub const DOCUMENT_IDENTITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "document_identity_support_export_record";

/// Schema version for report and support-export packets.
pub const DOCUMENT_IDENTITY_REPORT_SCHEMA_VERSION: u32 =
    DOCUMENT_IDENTITY_DISCLOSURE_SCHEMA_VERSION;

const REQUIRED_CONSUMERS: [&str; 5] = [
    "open",
    "review",
    "support_export",
    "docs_help",
    "cli_headless",
];

const NOTEBOOK_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/notebook/notebook_trust_diff_alpha/protected_trust_diff_repair_export.yaml"
));

/// One report row mapping a surface to a shared disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentityReportRow {
    /// Stable row id.
    pub row_id: String,
    /// Stable surface ref.
    pub surface_ref: String,
    /// Consumers that must reuse this disclosure verbatim.
    pub consumer_surfaces: Vec<String>,
    /// Shared document-identity disclosure.
    pub disclosure: DocumentIdentityDisclosure,
}

/// Cross-surface report for current document-identity disclosures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentityReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable report id.
    pub report_id: String,
    /// Deterministic generation timestamp used by the headless inspector.
    pub generated_at: String,
    /// Rows in deterministic order.
    pub rows: Vec<DocumentIdentityReportRow>,
}

/// Export-safe support projection for the report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentitySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source report id.
    pub report_id: String,
    /// Row ids preserved in the export.
    pub row_ids: Vec<String>,
    /// All identity tokens preserved in the export.
    pub identity_tokens: Vec<String>,
    /// Deterministic plaintext summary.
    pub plaintext_summary: String,
}

/// Validation finding emitted by [`DocumentIdentityReport::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentIdentityReportFinding {
    /// Row id the finding applies to, or `report` for top-level findings.
    pub row_id: String,
    /// Stable field or rule ref.
    pub field: String,
    /// Human-readable finding message.
    pub message: String,
}

#[derive(Debug, Deserialize)]
struct NotebookFixtureEnvelope {
    lane: NotebookAlphaLaneRecord,
}

impl DocumentIdentityReport {
    /// Validates row completeness, consumer parity, and label coverage.
    pub fn validate(&self) -> Result<(), Vec<DocumentIdentityReportFinding>> {
        let mut findings = Vec::new();

        if self.record_kind != DOCUMENT_IDENTITY_REPORT_RECORD_KIND {
            findings.push(finding(
                "report",
                "record_kind",
                "record_kind must match the report contract",
            ));
        }
        if self.schema_version != DOCUMENT_IDENTITY_REPORT_SCHEMA_VERSION {
            findings.push(finding(
                "report",
                "schema_version",
                "schema_version must match the report contract",
            ));
        }
        if self.rows.is_empty() {
            findings.push(finding("report", "rows", "at least one row is required"));
        }

        let mut row_ids = BTreeSet::new();
        let mut labels = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.as_str()) {
                findings.push(finding(
                    &row.row_id,
                    "row_id",
                    "row_id must be unique across the report",
                ));
            }
            for field in row.disclosure.missing_fields() {
                findings.push(finding(
                    &row.row_id,
                    field,
                    "disclosure field must be populated",
                ));
            }
            for required in REQUIRED_CONSUMERS {
                if !row
                    .consumer_surfaces
                    .iter()
                    .any(|surface| surface == required)
                {
                    findings.push(finding(
                        &row.row_id,
                        "consumer_surfaces",
                        format!("missing required consumer {required}"),
                    ));
                }
            }
            labels.extend(
                row.disclosure
                    .document_labels
                    .iter()
                    .map(|label| label.as_str()),
            );
        }

        for required in [
            DocumentLabelClass::VirtualDocument,
            DocumentLabelClass::GeneratedDocument,
            DocumentLabelClass::ArchiveDocument,
            DocumentLabelClass::ProviderBackedTransient,
            DocumentLabelClass::OverlayProjection,
            DocumentLabelClass::ExportArtifact,
        ] {
            if !labels.contains(required.as_str()) {
                findings.push(finding(
                    "report",
                    "label_coverage",
                    format!("missing required label coverage for {}", required.as_str()),
                ));
            }
        }

        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Returns an export-safe support projection for the report.
    pub fn support_export(&self) -> DocumentIdentitySupportExport {
        let mut identity_tokens = Vec::new();
        let mut row_ids = Vec::new();
        for row in &self.rows {
            row_ids.push(row.row_id.clone());
            identity_tokens.extend(row.disclosure.identity_tokens());
        }

        DocumentIdentitySupportExport {
            record_kind: DOCUMENT_IDENTITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCUMENT_IDENTITY_REPORT_SCHEMA_VERSION,
            report_id: self.report_id.clone(),
            row_ids,
            identity_tokens,
            plaintext_summary: self.render_plaintext(),
        }
    }

    /// Renders a deterministic plaintext report.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "Document identity report: {} @ {}\n",
            self.report_id, self.generated_at
        ));
        for row in &self.rows {
            out.push_str(&format!("row={} surface={}\n", row.row_id, row.surface_ref));
            for line in row.disclosure.render_plaintext_lines() {
                out.push_str(&format!("  - {line}\n"));
            }
        }
        out
    }

    /// Renders one compact line per row for CLI/headless inspection.
    pub fn compact_lines(&self) -> Vec<String> {
        self.rows
            .iter()
            .map(|row| {
                format!(
                    "{} | {} | {} | {} | {} | {}",
                    row.row_id,
                    row.surface_ref,
                    row.disclosure.document_family.as_str(),
                    row.disclosure.root_class.as_str(),
                    row.disclosure.document_label_tokens.join(","),
                    row.disclosure.write_posture.as_str()
                )
            })
            .collect()
    }
}

/// Builds the seeded report used by the headless inspector and tests.
pub fn seeded_document_identity_report() -> DocumentIdentityReport {
    let notebook_fixture: NotebookFixtureEnvelope =
        serde_yaml::from_str(NOTEBOOK_FIXTURE).expect("notebook fixture parses for report");
    let notebook_preview = seeded_notebook_preview_truth("workspace:report");
    let structured_config = seeded_structured_config_preview_truth("workspace:report");

    DocumentIdentityReport {
        record_kind: DOCUMENT_IDENTITY_REPORT_RECORD_KIND.to_owned(),
        schema_version: DOCUMENT_IDENTITY_REPORT_SCHEMA_VERSION,
        report_id: "document-identity-report:alpha".to_owned(),
        generated_at: "2026-06-12T00:00:00Z".to_owned(),
        rows: vec![
            DocumentIdentityReportRow {
                row_id: "row:notebook_document".to_owned(),
                surface_ref: "shell:notebook_alpha:document".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: notebook_fixture
                    .lane
                    .normalized()
                    .document
                    .identity_disclosure,
            },
            DocumentIdentityReportRow {
                row_id: "row:notebook_preview".to_owned(),
                surface_ref: "shell:preview_truth:notebook".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: notebook_preview.identity_disclosure,
            },
            DocumentIdentityReportRow {
                row_id: "row:structured_config_preview".to_owned(),
                surface_ref: "shell:preview_truth:structured_config".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: structured_config.identity_disclosure,
            },
            DocumentIdentityReportRow {
                row_id: "row:review_archive".to_owned(),
                surface_ref: "shell:review_archive:artifact".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: manual_archive_disclosure(),
            },
            DocumentIdentityReportRow {
                row_id: "row:provider_draft".to_owned(),
                surface_ref: "shell:provider_draft:transient".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: manual_provider_draft_disclosure(),
            },
            DocumentIdentityReportRow {
                row_id: "row:export_artifact".to_owned(),
                surface_ref: "shell:export_artifact:generated".to_owned(),
                consumer_surfaces: REQUIRED_CONSUMERS.into_iter().map(str::to_owned).collect(),
                disclosure: manual_export_artifact_disclosure(),
            },
        ],
    }
}

fn manual_archive_disclosure() -> DocumentIdentityDisclosure {
    DocumentIdentityDisclosure {
        record_kind: String::new(),
        schema_version: 0,
        document_family: DocumentFamilyClass::ReviewArtifact,
        document_family_token: String::new(),
        root_class: RootClassDisclosure::Archive,
        root_class_token: String::new(),
        document_labels: vec![DocumentLabelClass::ArchiveDocument],
        document_label_tokens: Vec::new(),
        presentation_path: "archive://review-bundle/pr-1842/comments.md".to_owned(),
        logical_identity_ref: "logical:review_archive:pr-1842:comments".to_owned(),
        canonical_target: "bundle:review:pr-1842:comments".to_owned(),
        canonical_target_hint: Some(
            "Review artifact is backed by a sealed bundle entry, not a writable workspace file."
                .to_owned(),
        ),
        alias_status: super::AliasStatusClass::ArchiveProjection,
        alias_status_token: String::new(),
        alias_status_label:
            "Presentation path is an archive-backed projection, not the durable inner source."
                .to_owned(),
        save_target_class: SaveTargetClass::ArchiveArtifact,
        save_target_class_token: String::new(),
        save_target_label:
            "Current surface is archive-backed; extract or reopen the backing artifact for durable edits."
                .to_owned(),
        write_posture: WritePostureClass::InspectOnly,
        write_posture_token: String::new(),
        write_posture_label: "Inspect only".to_owned(),
        backing_source_label: "Review bundle artifact".to_owned(),
        freshness_label: "Cached snapshot".to_owned(),
        docs_help_ref: "help:review:archive_identity".to_owned(),
    }
    .normalized()
}

fn manual_provider_draft_disclosure() -> DocumentIdentityDisclosure {
    DocumentIdentityDisclosure {
        record_kind: String::new(),
        schema_version: 0,
        document_family: DocumentFamilyClass::ProviderDraft,
        document_family_token: String::new(),
        root_class: RootClassDisclosure::Virtual,
        root_class_token: String::new(),
        document_labels: vec![
            DocumentLabelClass::VirtualDocument,
            DocumentLabelClass::ProviderBackedTransient,
        ],
        document_label_tokens: Vec::new(),
        presentation_path: "provider://drafts/payments/release-note".to_owned(),
        logical_identity_ref: "logical:provider_draft:payments:release-note".to_owned(),
        canonical_target: "provider:draft:payments:release-note".to_owned(),
        canonical_target_hint: Some(
            "Provider draft is transient until promoted into a durable repository or export target."
                .to_owned(),
        ),
        alias_status: super::AliasStatusClass::ProviderAlias,
        alias_status_token: String::new(),
        alias_status_label:
            "Presentation path is provider-backed and may resolve through a provider alias."
                .to_owned(),
        save_target_class: SaveTargetClass::VirtualProjection,
        save_target_class_token: String::new(),
        save_target_label: "Current surface is a virtual projection; promote or export before durable write.".to_owned(),
        write_posture: WritePostureClass::PromoteBeforeSave,
        write_posture_token: String::new(),
        write_posture_label: "Promote draft before save".to_owned(),
        backing_source_label: "Provider-managed transient draft".to_owned(),
        freshness_label: "Provider live draft".to_owned(),
        docs_help_ref: "help:provider:draft_identity".to_owned(),
    }
    .normalized()
}

fn manual_export_artifact_disclosure() -> DocumentIdentityDisclosure {
    DocumentIdentityDisclosure {
        record_kind: String::new(),
        schema_version: 0,
        document_family: DocumentFamilyClass::ExportArtifact,
        document_family_token: String::new(),
        root_class: RootClassDisclosure::Generated,
        root_class_token: String::new(),
        document_labels: vec![
            DocumentLabelClass::GeneratedDocument,
            DocumentLabelClass::ExportArtifact,
        ],
        document_label_tokens: Vec::new(),
        presentation_path: "generated://exports/payments/snapshot.csv".to_owned(),
        logical_identity_ref: "logical:export_artifact:payments:snapshot_csv".to_owned(),
        canonical_target: "export:payments:snapshot.csv".to_owned(),
        canonical_target_hint: Some(
            "CSV snapshot is generated from a query result and is not the canonical source object."
                .to_owned(),
        ),
        alias_status: super::AliasStatusClass::Projection,
        alias_status_token: String::new(),
        alias_status_label: "Presentation path is a projection over another source of truth."
            .to_owned(),
        save_target_class: SaveTargetClass::ExportArtifact,
        save_target_class_token: String::new(),
        save_target_label:
            "Durable output is an export artifact produced from another source of truth."
                .to_owned(),
        write_posture: WritePostureClass::ExportBeforeWrite,
        write_posture_token: String::new(),
        write_posture_label: "Export before durable write".to_owned(),
        backing_source_label: "Generated export bundle".to_owned(),
        freshness_label: "Generated on demand".to_owned(),
        docs_help_ref: "help:export:artifact_identity".to_owned(),
    }
    .normalized()
}

fn finding(
    row_id: impl Into<String>,
    field: impl Into<String>,
    message: impl Into<String>,
) -> DocumentIdentityReportFinding {
    DocumentIdentityReportFinding {
        row_id: row_id.into(),
        field: field.into(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_report_validates_and_covers_required_labels() {
        let report = seeded_document_identity_report();
        report.validate().expect("seeded report must validate");
        assert_eq!(report.rows.len(), 6);
    }

    #[test]
    fn support_export_preserves_identity_tokens() {
        let export = seeded_document_identity_report().support_export();
        assert!(export
            .identity_tokens
            .iter()
            .any(|token| token == "label:archive_document"));
        assert!(export
            .identity_tokens
            .iter()
            .any(|token| token == "label:provider_backed_transient"));
        assert!(export
            .identity_tokens
            .iter()
            .any(|token| token == "label:overlay_projection"));
    }

    #[test]
    fn plaintext_mentions_root_save_target_and_labels() {
        let text = seeded_document_identity_report().render_plaintext();
        assert!(text.contains("row=row:notebook_document"));
        assert!(text.contains("labels=[durable_local_file]"));
        assert!(text.contains("labels=[generated_document,overlay_projection]"));
        assert!(text.contains("labels=[archive_document]"));
        assert!(text.contains("labels=[virtual_document,provider_backed_transient]"));
        assert!(text.contains("save_target=export_artifact"));
    }
}
