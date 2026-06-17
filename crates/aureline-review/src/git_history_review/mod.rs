//! Review-surface projection of per-verb history-surgery review sheets.
//!
//! The canonical [`aureline_git`] history-surgery sheets carry one durable object
//! per risky verb (rebase, cherry-pick, revert, reset, patch-apply, force-push),
//! each with exact target truth, the pre-execution gate states, and a derived
//! allow/block/downgrade [`decision`](aureline_git::ReviewDecision). This module
//! projects those sheets onto the surfaces that must *explain* a risky mutation —
//! the review pane, the CLI/headless result packet, the redaction-safe support
//! export, the provider overlay, and AI context — so every surface reads the same
//! decision instead of re-deriving its own.
//!
//! Each [`HistorySurgeryDecisionRow`] restates *why* a mutation was allowed,
//! blocked, or downgraded (the decision outcome, its primary reason, and the
//! gates that contributed) and whether the surface may actually *execute* the
//! mutation. Only a mutation surface may execute, and only when the sheet's
//! decision is [`allowed`](aureline_git::ReviewDecisionOutcome::Allowed): a
//! read-only surface (support export, provider overlay, AI context) always
//! restates the decision but never marks it executable. Because the row is a
//! deterministic projection of its sheet, a stored row can be re-derived and
//! verified against the same Git truth the desktop and CLI surfaces use.
//!
//! The embedded sheets are validated through the Git history-surgery contract, so
//! the review surface never explains a sheet the Git layer would reject, and a
//! provider outage never blocks the local preview/abort/restore truth a row
//! carries forward.
//!
//! The boundary schema is
//! [`schemas/git/git_history_review.schema.json`](../../../../schemas/git/git_history_review.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/history_surgery_review/git_history_review.json`](../../../../artifacts/git/m5/history_surgery_review/git_history_review.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    HistorySurgeryReviewPacket, HistorySurgeryReviewSheet, HistorySurgeryReviewSupportExport,
    HistorySurgeryVerb, ReviewDecisionOutcome, HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND,
    HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS, HISTORY_SURGERY_REVIEW_SCHEMA_VERSION,
    HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
};

/// Schema version for [`GitHistoryReviewPacket`].
pub const GIT_HISTORY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`GitHistoryReviewPacket`].
pub const GIT_HISTORY_REVIEW_PACKET_RECORD_KIND: &str = "git_history_review_packet";

/// Stable record-kind tag carried by [`HistorySurgeryDecisionRow`].
pub const GIT_HISTORY_REVIEW_ROW_RECORD_KIND: &str = "git_history_review_decision_row";

/// Stable record-kind tag carried by [`GitHistoryReviewSupportExport`].
pub const GIT_HISTORY_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str = "git_history_review_support_export";

/// Repo-relative path of the boundary schema.
pub const GIT_HISTORY_REVIEW_SCHEMA_REF: &str = "schemas/git/git_history_review.schema.json";

/// Repo-relative path of the checked-in canonical review packet.
pub const GIT_HISTORY_REVIEW_ARTIFACT_REF: &str =
    "artifacts/git/m5/history_surgery_review/git_history_review.json";

/// Reconstruction fields a support export must retain after redaction.
pub const GIT_HISTORY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 6] = [
    "surface",
    "sheet_ref",
    "verb",
    "decision_outcome",
    "decision_reason",
    "execution_permitted",
];

/// A surface that restates a history-surgery decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryReviewSurface {
    /// Review diff, summary, and history-edit rows.
    Review,
    /// CLI / headless replay or JSON result packets.
    CliHeadless,
    /// Redaction-safe support / export rows.
    SupportExport,
    /// Provider overlay (status, PR, checks) layered over local truth.
    ProviderOverlay,
    /// AI-context assembly and evidence inspectors.
    AiContext,
}

impl HistoryReviewSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Review,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ProviderOverlay,
        Self::AiContext,
    ];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ProviderOverlay => "provider_overlay",
            Self::AiContext => "ai_context",
        }
    }

    /// Whether this surface can actually execute a risky mutation.
    ///
    /// Read and continuity surfaces still restate the decision so the user can
    /// see why a mutation was allowed/blocked/downgraded, but only a mutation
    /// surface may run it.
    pub const fn is_mutation_surface(self) -> bool {
        matches!(self, Self::Review | Self::CliHeadless)
    }
}

/// One surface-facing restatement of a sheet's allow/block/downgrade decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistorySurgeryDecisionRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Surface that renders this row.
    pub surface: HistoryReviewSurface,
    /// Referenced [`HistorySurgeryReviewSheet::sheet_id`].
    pub sheet_ref: String,
    /// Verb carried for surface routing.
    pub verb: HistorySurgeryVerb,
    /// Restated decision outcome.
    pub outcome: ReviewDecisionOutcome,
    /// Restated primary reason token.
    pub primary_reason: String,
    /// Restated contributing-gate tokens.
    pub contributing_gates: Vec<String>,
    /// Restated recovery visibility.
    pub recovery_visible: bool,
    /// Restated offline local-truth availability.
    pub local_truth_available_offline: bool,
    /// True only when this surface may run the mutation now (mutation surface and
    /// an allowed decision).
    pub execution_permitted: bool,
    /// Restated deterministic explanation.
    pub explanation: String,
}

impl HistorySurgeryDecisionRow {
    /// Projects a sheet onto one surface, restating its decision.
    ///
    /// The restatement is a deterministic copy of the sheet's derived decision;
    /// execution is permitted only on a mutation surface with an allowed decision.
    pub fn for_surface_and_sheet(
        surface: HistoryReviewSurface,
        sheet: &HistorySurgeryReviewSheet,
        row_id: impl Into<String>,
    ) -> Self {
        let execution_permitted =
            surface.is_mutation_surface() && sheet.decision.outcome.permits_execution();
        Self {
            record_kind: GIT_HISTORY_REVIEW_ROW_RECORD_KIND.to_owned(),
            row_id: row_id.into(),
            surface,
            sheet_ref: sheet.sheet_id.clone(),
            verb: sheet.verb,
            outcome: sheet.decision.outcome,
            primary_reason: sheet.decision.primary_reason.clone(),
            contributing_gates: sheet.decision.contributing_gates.clone(),
            recovery_visible: sheet.decision.recovery_visible,
            local_truth_available_offline: sheet.decision.local_truth_available_offline,
            execution_permitted,
            explanation: sheet.decision.explanation.clone(),
        }
    }
}

/// Redaction-safe support-export projection for a review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryReviewSupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Row ids included in the export.
    pub row_refs: Vec<String>,
    /// Structured fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw patch/todo bodies are embedded.
    pub raw_patch_bodies_redacted: bool,
    /// True when no raw provider payloads are embedded.
    pub raw_provider_payloads_redacted: bool,
}

/// Top-level packet binding surface decision rows to the sheets they restate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitHistoryReviewPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Repository ref the embedded sheets belong to.
    pub repo_ref: String,
    /// Reviewed history-surgery sheets the rows restate.
    pub sheets: Vec<HistorySurgeryReviewSheet>,
    /// Per-surface decision rows derived from the sheets.
    pub rows: Vec<HistorySurgeryDecisionRow>,
    /// Redaction-safe support-export projection.
    pub support_export: GitHistoryReviewSupportExport,
}

impl GitHistoryReviewPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GitHistoryReviewError`] when the JSON is invalid or the parsed
    /// packet violates the review contract.
    pub fn parse_json(input: &str) -> Result<Self, GitHistoryReviewError> {
        let packet: Self = serde_json::from_str(input).map_err(GitHistoryReviewError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(GitHistoryReviewError::Validation(violations))
        }
    }

    /// Validates every row, embedded sheet, and support-export invariant.
    pub fn validate(&self) -> Vec<GitHistoryReviewValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != GIT_HISTORY_REVIEW_PACKET_RECORD_KIND {
            errors.push(GitHistoryReviewValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != GIT_HISTORY_REVIEW_SCHEMA_VERSION {
            errors.push(GitHistoryReviewValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.repo_ref.trim().is_empty()
        {
            errors.push(GitHistoryReviewValidationError::MissingIdentity);
        }

        // The embedded sheets must themselves pass the Git history-surgery
        // contract, so the review surface never explains a sheet the Git layer
        // would reject.
        let embedded = HistorySurgeryReviewPacket {
            record_kind: HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: HISTORY_SURGERY_REVIEW_SCHEMA_VERSION,
            packet_id: format!("{}::embedded", self.packet_id),
            generated_at: self.generated_at.clone(),
            repo_ref: self.repo_ref.clone(),
            sheets: self.sheets.clone(),
            support_export: HistorySurgeryReviewSupportExport {
                record_kind: HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
                export_id: format!("{}::embedded-export", self.packet_id),
                sheet_refs: self
                    .sheets
                    .iter()
                    .map(|sheet| sheet.sheet_id.clone())
                    .collect(),
                reconstruction_fields: HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
                    .iter()
                    .map(|field| (*field).to_owned())
                    .collect(),
                raw_paths_redacted: true,
                raw_patch_bodies_redacted: true,
                raw_provider_payloads_redacted: true,
            },
        };
        for violation in embedded.validate() {
            errors.push(GitHistoryReviewValidationError::EmbeddedSheetInvalid {
                detail: violation.to_string(),
            });
        }

        let sheets_by_id: std::collections::HashMap<&str, &HistorySurgeryReviewSheet> = self
            .sheets
            .iter()
            .map(|sheet| (sheet.sheet_id.as_str(), sheet))
            .collect();

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if row.record_kind != GIT_HISTORY_REVIEW_ROW_RECORD_KIND {
                errors.push(GitHistoryReviewValidationError::WrongRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(GitHistoryReviewValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }

            let Some(sheet) = sheets_by_id.get(row.sheet_ref.as_str()) else {
                errors.push(GitHistoryReviewValidationError::UnknownSheetRef {
                    row_id: row.row_id.clone(),
                    sheet_ref: row.sheet_ref.clone(),
                });
                continue;
            };

            // The row must equal the deterministic projection of its sheet; this
            // is what proves the same decision drives every surface.
            let expected = HistorySurgeryDecisionRow::for_surface_and_sheet(
                row.surface,
                sheet,
                row.row_id.clone(),
            );
            if &expected != row {
                errors.push(GitHistoryReviewValidationError::RowDoesNotMatchSheet {
                    row_id: row.row_id.clone(),
                });
            }

            // Guardrail: a read-only surface never marks a mutation executable.
            if !row.surface.is_mutation_surface() && row.execution_permitted {
                errors.push(GitHistoryReviewValidationError::ReadOnlySurfaceExecutable {
                    row_id: row.row_id.clone(),
                });
            }

            // Guardrail: execution is permitted only for an allowed decision.
            if row.execution_permitted && row.outcome != ReviewDecisionOutcome::Allowed {
                errors.push(
                    GitHistoryReviewValidationError::ExecutionWithoutAllowedDecision {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        }

        validate_support_export(self, &row_ids, &mut errors);
        errors
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("git history review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# History-Surgery Decision Rows\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Sheets: {} / Rows: {}\n\n",
            self.sheets.len(),
            self.rows.len()
        ));
        out.push_str("## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** → `{}` ({}): {} — {} (executable {})\n",
                row.surface.as_str(),
                row.sheet_ref,
                row.verb.as_str(),
                row.outcome.as_str(),
                row.primary_reason,
                row.execution_permitted,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical review packet.
///
/// # Errors
///
/// Returns [`GitHistoryReviewError`] when the checked-in packet fails to parse or
/// violates the review contract.
pub fn current_git_history_review_packet() -> Result<GitHistoryReviewPacket, GitHistoryReviewError>
{
    GitHistoryReviewPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/history_surgery_review/git_history_review.json"
    )))
}

fn validate_support_export(
    packet: &GitHistoryReviewPacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<GitHistoryReviewValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != GIT_HISTORY_REVIEW_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(GitHistoryReviewValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(GitHistoryReviewValidationError::UnknownSupportRowRef {
                row_ref: row_ref.clone(),
            });
        }
    }
    for required in GIT_HISTORY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(GitHistoryReviewValidationError::SupportExportMissingField {
                field: required.to_string(),
            });
        }
    }
    if !export.raw_paths_redacted
        || !export.raw_patch_bodies_redacted
        || !export.raw_provider_payloads_redacted
    {
        errors.push(GitHistoryReviewValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Error returned while parsing a review packet.
#[derive(Debug)]
pub enum GitHistoryReviewError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<GitHistoryReviewValidationError>),
}

impl fmt::Display for GitHistoryReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse git history review packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "git history review packet has validation errors: "
                )?;
                for (index, error) in errors.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, "; ")?;
                    }
                    write!(formatter, "{error}")?;
                }
                Ok(())
            }
        }
    }
}

impl Error for GitHistoryReviewError {}

/// Cross-row validation error for a review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHistoryReviewValidationError {
    /// A record-kind tag does not match the stable contract.
    WrongRecordKind {
        /// Observed record-kind tag.
        observed: String,
    },
    /// The packet schema version is unsupported.
    WrongSchemaVersion {
        /// Observed schema version.
        observed: u32,
    },
    /// A required identity field is missing.
    MissingIdentity,
    /// An embedded history-surgery sheet fails the Git contract.
    EmbeddedSheetInvalid {
        /// Human-readable detail from the Git validator.
        detail: String,
    },
    /// A row id is declared more than once.
    DuplicateRowId {
        /// Duplicated row id.
        row_id: String,
    },
    /// A row references a sheet not present in the packet.
    UnknownSheetRef {
        /// Row id.
        row_id: String,
        /// Unknown sheet ref.
        sheet_ref: String,
    },
    /// A row does not equal the deterministic projection of its sheet.
    RowDoesNotMatchSheet {
        /// Row id.
        row_id: String,
    },
    /// A read-only surface marks a mutation executable.
    ReadOnlySurfaceExecutable {
        /// Row id.
        row_id: String,
    },
    /// A row permits execution without an allowed decision.
    ExecutionWithoutAllowedDecision {
        /// Row id.
        row_id: String,
    },
    /// A support-export row ref is unknown.
    UnknownSupportRowRef {
        /// Unknown row ref.
        row_ref: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths, bodies, or provider payloads.
    SupportExportEmbedsRawMaterial,
}

impl fmt::Display for GitHistoryReviewValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "packet is missing identity fields"),
            Self::EmbeddedSheetInvalid { detail } => {
                write!(
                    formatter,
                    "embedded history-surgery sheet invalid: {detail}"
                )
            }
            Self::DuplicateRowId { row_id } => {
                write!(formatter, "row id {row_id} is declared more than once")
            }
            Self::UnknownSheetRef { row_id, sheet_ref } => write!(
                formatter,
                "review row {row_id} references unknown sheet {sheet_ref}"
            ),
            Self::RowDoesNotMatchSheet { row_id } => write!(
                formatter,
                "review row {row_id} does not match its sheet projection"
            ),
            Self::ReadOnlySurfaceExecutable { row_id } => write!(
                formatter,
                "read-only review row {row_id} marks a mutation executable"
            ),
            Self::ExecutionWithoutAllowedDecision { row_id } => write!(
                formatter,
                "review row {row_id} permits execution without an allowed decision"
            ),
            Self::UnknownSupportRowRef { row_ref } => {
                write!(formatter, "support export references unknown row {row_ref}")
            }
            Self::SupportExportMissingField { field } => {
                write!(
                    formatter,
                    "support export missing reconstruction field {field}"
                )
            }
            Self::SupportExportEmbedsRawMaterial => write!(
                formatter,
                "support export embeds raw paths, bodies, or provider payloads"
            ),
        }
    }
}
