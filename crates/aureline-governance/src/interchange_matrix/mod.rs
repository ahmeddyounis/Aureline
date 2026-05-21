//! Typed standards/interchange matrix.
//!
//! The canonical register at `artifacts/governance/standards_matrix.yaml`
//! records every industry standard Aureline reuses, mirrors, extends, or
//! declines, and two markdown pages project it for reviewers. This module
//! promotes the beta interchange posture into a typed, machine-consumable
//! matrix: one [`InterchangeMatrixRow`] per standard or interchange format,
//! carrying the support posture, import/export expectation, version range,
//! deviation policy, owner, and evidence paths.
//!
//! The matrix is generated from the register by
//! `ci/check_beta_interchange_matrix.py` into
//! `artifacts/governance/standards_interchange_matrix_beta.json` and embedded
//! here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI. The markdown pages remain the human view; that gate keeps
//! the docs and this machine matrix from drifting.
//!
//! Support posture, import expectation, and export expectation are closed enums.
//! A row that lacks a posture, or carries a token outside the closed
//! vocabulary, fails to deserialize rather than passing silently.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported machine-matrix schema version.
pub const STANDARDS_INTERCHANGE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the machine matrix.
pub const STANDARDS_INTERCHANGE_MATRIX_RECORD_KIND: &str = "standards_interchange_matrix";

/// Repo-relative path to the generated machine matrix.
pub const STANDARDS_INTERCHANGE_MATRIX_PATH: &str =
    "artifacts/governance/standards_interchange_matrix_beta.json";

/// Embedded generated machine matrix JSON.
pub const STANDARDS_INTERCHANGE_MATRIX_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/standards_interchange_matrix_beta.json"
));

/// How Aureline relates to a named standard at this milestone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPosture {
    /// Aureline consumes and emits the standard verbatim, within the pinned range.
    StandardShapedImportAndExport,
    /// Aureline emits the standard but does not consume it.
    StandardShapedExportOnly,
    /// Aureline consumes the standard but does not emit it.
    StandardShapedImportOnly,
    /// Aureline owns a custom contract today with a documented bridge to the standard.
    CustomButMirrorable,
    /// Aureline owns a custom contract today; a bridge lane is reserved.
    CustomWithBridgePlanned,
    /// Seat reserved; a stub or placeholder this milestone, not a live claim.
    StandardDeferredPlaceholder,
    /// Deliberately not adopted at this milestone; the row keeps the audit trail.
    StandardDeclinedWithRationale,
}

impl SupportPosture {
    /// Every posture, in register-declaration order.
    pub const ALL: [Self; 7] = [
        Self::StandardShapedImportAndExport,
        Self::StandardShapedExportOnly,
        Self::StandardShapedImportOnly,
        Self::CustomButMirrorable,
        Self::CustomWithBridgePlanned,
        Self::StandardDeferredPlaceholder,
        Self::StandardDeclinedWithRationale,
    ];

    /// Stable token recorded in the register, matrix, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StandardShapedImportAndExport => "standard_shaped_import_and_export",
            Self::StandardShapedExportOnly => "standard_shaped_export_only",
            Self::StandardShapedImportOnly => "standard_shaped_import_only",
            Self::CustomButMirrorable => "custom_but_mirrorable",
            Self::CustomWithBridgePlanned => "custom_with_bridge_planned",
            Self::StandardDeferredPlaceholder => "standard_deferred_placeholder",
            Self::StandardDeclinedWithRationale => "standard_declined_with_rationale",
        }
    }

    /// True for a posture that is not a live interoperability claim and so may
    /// carry no evidence path beyond the reserved-seat note.
    pub const fn is_placeholder(self) -> bool {
        matches!(
            self,
            Self::StandardDeferredPlaceholder | Self::StandardDeclinedWithRationale
        )
    }

    /// True when a row with this posture may be cited as a live interoperability
    /// claim (it is a standard-shaped posture, not a bridge or placeholder).
    pub const fn is_claim_bearing(self) -> bool {
        matches!(
            self,
            Self::StandardShapedImportAndExport
                | Self::StandardShapedExportOnly
                | Self::StandardShapedImportOnly
        )
    }

    /// True for a custom contract with a mirrorable or planned bridge to the
    /// standard. A bridge posture is not a conformance claim.
    pub const fn is_bridge(self) -> bool {
        matches!(
            self,
            Self::CustomButMirrorable | Self::CustomWithBridgePlanned
        )
    }
}

/// What Aureline consumes in a standard's shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportExpectation {
    /// Consumption is mandatory for the surface.
    Required,
    /// Consumption is supported.
    Supported,
    /// Consumption is best-effort only.
    BestEffort,
    /// No import path is planned this milestone.
    NonePlanned,
    /// Import is deferred to a later milestone.
    DeferredToLaterMilestone,
}

impl ImportExpectation {
    /// Stable token recorded in the register and matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Supported => "supported",
            Self::BestEffort => "best_effort",
            Self::NonePlanned => "none_planned",
            Self::DeferredToLaterMilestone => "deferred_to_later_milestone",
        }
    }
}

/// What Aureline emits in a standard's shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportExpectation {
    /// Emission is mandatory for the surface.
    Required,
    /// Emission is supported.
    Supported,
    /// Emission is best-effort only.
    BestEffort,
    /// Only a placeholder stub is emitted; not a conformance claim.
    PlaceholderStubOnly,
    /// No export path is planned this milestone.
    NonePlanned,
    /// Export is deferred to a later milestone.
    DeferredToLaterMilestone,
}

impl ExportExpectation {
    /// Stable token recorded in the register and matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Supported => "supported",
            Self::BestEffort => "best_effort",
            Self::PlaceholderStubOnly => "placeholder_stub_only",
            Self::NonePlanned => "none_planned",
            Self::DeferredToLaterMilestone => "deferred_to_later_milestone",
        }
    }
}

/// One standards/interchange matrix row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterchangeMatrixRow {
    /// Stable register row id, e.g. `standard.sarif`.
    pub row_id: String,
    /// Human-readable standard or interchange surface name.
    pub standard_surface: String,
    /// Domain class the row governs.
    pub domain: String,
    /// How Aureline relates to the standard at this milestone.
    pub support_posture: SupportPosture,
    /// What Aureline consumes in the standard's shape.
    pub import_expectation: ImportExpectation,
    /// What Aureline emits in the standard's shape.
    pub export_expectation: ExportExpectation,
    /// Preferred-standard version range Aureline treats as canonical.
    pub version_range: String,
    /// Posture committed to when the standard is not adopted verbatim.
    pub deviation_policy: String,
    /// Owning lane for the posture.
    pub owner_lane: String,
    /// Directly responsible individual for the posture.
    pub owner_dri: String,
    /// Compatibility-window class for the pinned version.
    pub compatibility_window_class: String,
    /// Minimum-evidence path classes that prove the posture.
    pub evidence_paths: Vec<String>,
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InterchangeMatrixSummary {
    /// Total number of rows.
    pub total_rows: usize,
    /// Rows whose posture is a live standard-shaped interoperability claim.
    pub standard_shaped_rows: usize,
    /// Rows with a custom-but-mirrorable or custom-with-bridge-planned posture.
    pub bridge_rows: usize,
    /// Rows whose posture is a deferred placeholder or declined-with-rationale.
    pub deferred_or_declined_rows: usize,
    /// Number of distinct domains covered.
    pub domains_covered: usize,
}

/// The typed standards/interchange matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StandardsInterchangeMatrix {
    /// Machine-matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix identifier.
    pub matrix_id: String,
    /// UTC timestamp the matrix was generated.
    pub generated_at: String,
    /// Canonical register this matrix projects.
    pub source_register_ref: String,
    /// Markdown human-view pages that must agree with this matrix.
    pub human_view_refs: Vec<String>,
    /// Closed support-posture vocabulary.
    pub support_posture_classes: Vec<SupportPosture>,
    /// Closed import-expectation vocabulary.
    pub import_expectation_classes: Vec<ImportExpectation>,
    /// Closed export-expectation vocabulary.
    pub export_expectation_classes: Vec<ExportExpectation>,
    /// Closed deviation-policy vocabulary.
    pub deviation_policy_classes: Vec<String>,
    /// Matrix rows.
    pub rows: Vec<InterchangeMatrixRow>,
    /// Summary counts.
    pub summary: InterchangeMatrixSummary,
}

impl StandardsInterchangeMatrix {
    /// Returns the row registered for `row_id`.
    pub fn row(&self, row_id: &str) -> Option<&InterchangeMatrixRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Returns the rows whose posture is a live interoperability claim.
    pub fn claim_bearing_rows(&self) -> Vec<&InterchangeMatrixRow> {
        self.rows
            .iter()
            .filter(|row| row.support_posture.is_claim_bearing())
            .collect()
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<InterchangeMatrixViolation> {
        let mut violations = Vec::new();

        if self.schema_version != STANDARDS_INTERCHANGE_MATRIX_SCHEMA_VERSION {
            violations.push(InterchangeMatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != STANDARDS_INTERCHANGE_MATRIX_RECORD_KIND {
            violations.push(InterchangeMatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        if self.rows.is_empty() {
            violations.push(InterchangeMatrixViolation::EmptyMatrix);
        }

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            for (field, value) in [
                ("row_id", &row.row_id),
                ("standard_surface", &row.standard_surface),
                ("domain", &row.domain),
                ("version_range", &row.version_range),
                ("deviation_policy", &row.deviation_policy),
                ("owner_lane", &row.owner_lane),
                ("owner_dri", &row.owner_dri),
                (
                    "compatibility_window_class",
                    &row.compatibility_window_class,
                ),
            ] {
                if value.trim().is_empty() {
                    violations.push(InterchangeMatrixViolation::EmptyField {
                        row_id: row.row_id.clone(),
                        field_name: field,
                    });
                }
            }

            if !is_valid_row_id(&row.row_id) {
                violations.push(InterchangeMatrixViolation::InvalidRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !seen.insert(row.row_id.clone()) {
                violations.push(InterchangeMatrixViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            if !row.support_posture.is_placeholder() && row.evidence_paths.is_empty() {
                violations.push(InterchangeMatrixViolation::MissingEvidence {
                    row_id: row.row_id.clone(),
                });
            }
        }

        let expected = InterchangeMatrixSummary {
            total_rows: self.rows.len(),
            standard_shaped_rows: self
                .rows
                .iter()
                .filter(|row| row.support_posture.is_claim_bearing())
                .count(),
            bridge_rows: self
                .rows
                .iter()
                .filter(|row| row.support_posture.is_bridge())
                .count(),
            deferred_or_declined_rows: self
                .rows
                .iter()
                .filter(|row| row.support_posture.is_placeholder())
                .count(),
            domains_covered: self
                .rows
                .iter()
                .map(|row| row.domain.as_str())
                .collect::<BTreeSet<_>>()
                .len(),
        };
        if self.summary != expected {
            violations.push(InterchangeMatrixViolation::SummaryMismatch);
        }

        violations
    }
}

fn is_valid_row_id(row_id: &str) -> bool {
    if let Some(rest) = row_id.strip_prefix("standard.") {
        !rest.is_empty()
            && rest
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_')
    } else {
        false
    }
}

/// A validation violation for the standards/interchange matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterchangeMatrixViolation {
    /// The matrix carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the matrix.
        actual: u32,
    },
    /// The matrix carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the matrix.
        actual: String,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// A row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// A row id is not shaped like `standard.<token>`.
    InvalidRowId {
        /// Offending row id.
        row_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row id.
        row_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A non-placeholder row lists no evidence path.
    MissingEvidence {
        /// Row id.
        row_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for InterchangeMatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::DuplicateRowId { row_id } => write!(f, "duplicate matrix row id {row_id}"),
            Self::InvalidRowId { row_id } => {
                write!(
                    f,
                    "matrix row id {row_id} is not shaped like standard.<token>"
                )
            }
            Self::EmptyField { row_id, field_name } => {
                write!(f, "matrix row {row_id} has empty field {field_name}")
            }
            Self::MissingEvidence { row_id } => write!(
                f,
                "matrix row {row_id} is a non-placeholder posture with no evidence path"
            ),
            Self::SummaryMismatch => write!(f, "matrix summary counts disagree with the rows"),
        }
    }
}

impl Error for InterchangeMatrixViolation {}

/// Loads the embedded generated machine matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`StandardsInterchangeMatrix`] — including when a row carries a support
/// posture, import expectation, or export expectation outside the closed
/// vocabularies.
pub fn current_standards_interchange_matrix(
) -> Result<StandardsInterchangeMatrix, serde_json::Error> {
    serde_json::from_str(STANDARDS_INTERCHANGE_MATRIX_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_matrix_parses_and_validates() {
        let matrix = current_standards_interchange_matrix().expect("matrix parses");
        assert_eq!(
            matrix.schema_version,
            STANDARDS_INTERCHANGE_MATRIX_SCHEMA_VERSION
        );
        assert_eq!(matrix.record_kind, STANDARDS_INTERCHANGE_MATRIX_RECORD_KIND);
        assert_eq!(matrix.validate(), Vec::new());
        assert!(!matrix.rows.is_empty());
    }

    #[test]
    fn every_row_carries_a_posture_and_expectations() {
        let matrix = current_standards_interchange_matrix().expect("matrix parses");
        // Deserialization already rejects an unknown or missing posture; this
        // asserts each row exposes a posture/import/export from the closed sets.
        for row in &matrix.rows {
            assert!(
                SupportPosture::ALL.contains(&row.support_posture),
                "{}",
                row.row_id
            );
            assert!(
                !row.support_posture.as_str().is_empty()
                    && !row.import_expectation.as_str().is_empty()
                    && !row.export_expectation.as_str().is_empty(),
                "{}",
                row.row_id
            );
        }
    }

    #[test]
    fn summary_counts_match_rows() {
        let matrix = current_standards_interchange_matrix().expect("matrix parses");
        assert_eq!(matrix.summary.total_rows, matrix.rows.len());
        assert_eq!(
            matrix.summary.standard_shaped_rows,
            matrix.claim_bearing_rows().len()
        );
        // The three posture tiers partition every row exactly.
        assert_eq!(
            matrix.summary.standard_shaped_rows
                + matrix.summary.bridge_rows
                + matrix.summary.deferred_or_declined_rows,
            matrix.rows.len()
        );
    }

    #[test]
    fn known_rows_resolve_to_expected_postures() {
        let matrix = current_standards_interchange_matrix().expect("matrix parses");

        let sarif = matrix.row("standard.sarif").expect("sarif row exists");
        assert_eq!(
            sarif.support_posture,
            SupportPosture::StandardShapedImportAndExport
        );
        assert!(sarif.support_posture.is_claim_bearing());
        assert_eq!(sarif.import_expectation, ImportExpectation::Supported);
        assert_eq!(sarif.export_expectation, ExportExpectation::Supported);

        let otel = matrix
            .row("standard.opentelemetry")
            .expect("opentelemetry row exists");
        assert_eq!(
            otel.support_posture,
            SupportPosture::CustomWithBridgePlanned
        );
        assert!(!otel.support_posture.is_claim_bearing());
        assert_eq!(
            otel.export_expectation,
            ExportExpectation::PlaceholderStubOnly
        );
    }

    #[test]
    fn validate_flags_a_row_without_a_posture_evidence() {
        let mut matrix = current_standards_interchange_matrix().expect("matrix parses");
        // Force a claim-bearing row to drop its evidence: a row that asserts a
        // posture but proves nothing must be rejected.
        let row = matrix
            .rows
            .iter_mut()
            .find(|row| row.support_posture.is_claim_bearing())
            .expect("a claim-bearing row exists");
        let row_id = row.row_id.clone();
        row.evidence_paths.clear();
        assert!(matrix
            .validate()
            .contains(&InterchangeMatrixViolation::MissingEvidence { row_id }));
    }

    #[test]
    fn validate_flags_a_duplicate_row() {
        let mut matrix = current_standards_interchange_matrix().expect("matrix parses");
        let first = matrix.rows[0].clone();
        matrix.rows.push(first.clone());
        assert!(matrix
            .validate()
            .contains(&InterchangeMatrixViolation::DuplicateRowId {
                row_id: first.row_id
            }));
    }
}
