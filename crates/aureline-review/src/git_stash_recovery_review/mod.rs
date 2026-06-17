//! Review-surface projection of per-verb stash/recovery review sheets.
//!
//! The canonical [`aureline_git`] stash/recovery sheets carry one durable object per
//! verb (stash apply, pop, drop, create-branch, reflog-restore, checkpoint-restore),
//! each with exact target truth, the pre-execution gate states, the reflog/checkpoint
//! restore surface, and a derived allow/block/downgrade
//! [`decision`](aureline_git::StashRecoveryDecision). This module projects those
//! sheets onto the surfaces that must *explain and reach* a stash or recovery verb —
//! the review pane (which also backs the Git history view and the command palette),
//! the CLI/headless result packet, the redaction-safe support export, the provider
//! overlay, and AI context — so every surface reads the same decision instead of
//! re-deriving its own.
//!
//! Each [`StashRecoveryDecisionRow`] restates *why* a verb was allowed, blocked, or
//! downgraded (the decision outcome, its primary reason, and the gates that
//! contributed) and whether the surface may actually *execute* it. Only a mutation
//! surface may execute, and only when the sheet's decision is
//! [`allowed`](aureline_git::StashRecoveryOutcome::Allowed): a read-only surface
//! (support export, provider overlay, AI context) always restates the decision but
//! never marks it executable. Because the row is a deterministic projection of its
//! sheet, a stored row can be re-derived and verified against the same Git truth the
//! desktop and CLI surfaces use.
//!
//! The embedded sheets are validated through the Git stash/recovery contract, so the
//! review surface never explains a sheet the Git layer would reject, and a provider
//! outage never blocks the local preview/continue/abort/inspect/restore truth a row
//! carries forward.
//!
//! The boundary schema is
//! [`schemas/git/git_stash_recovery_review.schema.json`](../../../../schemas/git/git_stash_recovery_review.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/stash_recovery/git_stash_recovery_review.json`](../../../../artifacts/git/m5/stash_recovery/git_stash_recovery_review.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    StashRecoveryOutcome, StashRecoveryPacket, StashRecoverySheet, StashRecoverySupportExport,
    StashRecoveryVerb, STASH_RECOVERY_PACKET_RECORD_KIND,
    STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS, STASH_RECOVERY_SCHEMA_VERSION,
    STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND,
};

/// Schema version for [`GitStashRecoveryReviewPacket`].
pub const GIT_STASH_RECOVERY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`GitStashRecoveryReviewPacket`].
pub const GIT_STASH_RECOVERY_REVIEW_PACKET_RECORD_KIND: &str = "git_stash_recovery_review_packet";

/// Stable record-kind tag carried by [`StashRecoveryDecisionRow`].
pub const GIT_STASH_RECOVERY_REVIEW_ROW_RECORD_KIND: &str =
    "git_stash_recovery_review_decision_row";

/// Stable record-kind tag carried by [`GitStashRecoveryReviewSupportExport`].
pub const GIT_STASH_RECOVERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "git_stash_recovery_review_support_export";

/// Repo-relative path of the boundary schema.
pub const GIT_STASH_RECOVERY_REVIEW_SCHEMA_REF: &str =
    "schemas/git/git_stash_recovery_review.schema.json";

/// Repo-relative path of the checked-in canonical review packet.
pub const GIT_STASH_RECOVERY_REVIEW_ARTIFACT_REF: &str =
    "artifacts/git/m5/stash_recovery/git_stash_recovery_review.json";

/// Reconstruction fields a support export must retain after redaction.
pub const GIT_STASH_RECOVERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 6] = [
    "surface",
    "sheet_ref",
    "verb",
    "decision_outcome",
    "decision_reason",
    "execution_permitted",
];

/// A surface that restates a stash/recovery decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StashRecoveryReviewSurface {
    /// Review pane, Git history rows, and command palette (the in-product mutation
    /// entry points share this surface).
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

impl StashRecoveryReviewSurface {
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

    /// Whether this surface can actually execute a stash or recovery verb.
    ///
    /// Read and continuity surfaces still restate the decision so the user can see
    /// why a verb was allowed/blocked/downgraded, but only a mutation surface may
    /// run it.
    pub const fn is_mutation_surface(self) -> bool {
        matches!(self, Self::Review | Self::CliHeadless)
    }
}

/// One surface-facing restatement of a sheet's allow/block/downgrade decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StashRecoveryDecisionRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Surface that renders this row.
    pub surface: StashRecoveryReviewSurface,
    /// Referenced [`StashRecoverySheet::sheet_id`].
    pub sheet_ref: String,
    /// Verb carried for surface routing.
    pub verb: StashRecoveryVerb,
    /// Restated decision outcome.
    pub outcome: StashRecoveryOutcome,
    /// Restated primary reason token.
    pub primary_reason: String,
    /// Restated contributing-gate tokens.
    pub contributing_gates: Vec<String>,
    /// Restated recovery visibility.
    pub recovery_visible: bool,
    /// Restated offline local-truth availability.
    pub local_truth_available_offline: bool,
    /// True only when this surface may run the verb now (mutation surface and an
    /// allowed decision).
    pub execution_permitted: bool,
    /// Restated deterministic explanation.
    pub explanation: String,
}

impl StashRecoveryDecisionRow {
    /// Projects a sheet onto one surface, restating its decision.
    ///
    /// The restatement is a deterministic copy of the sheet's derived decision;
    /// execution is permitted only on a mutation surface with an allowed decision.
    pub fn for_surface_and_sheet(
        surface: StashRecoveryReviewSurface,
        sheet: &StashRecoverySheet,
        row_id: impl Into<String>,
    ) -> Self {
        let execution_permitted =
            surface.is_mutation_surface() && sheet.decision.outcome.permits_execution();
        Self {
            record_kind: GIT_STASH_RECOVERY_REVIEW_ROW_RECORD_KIND.to_owned(),
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
pub struct GitStashRecoveryReviewSupportExport {
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
    /// True when no raw patch/diff bodies are embedded.
    pub raw_patch_bodies_redacted: bool,
    /// True when no raw provider payloads are embedded.
    pub raw_provider_payloads_redacted: bool,
}

/// Top-level packet binding surface decision rows to the sheets they restate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitStashRecoveryReviewPacket {
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
    /// Reviewed stash/recovery sheets the rows restate.
    pub sheets: Vec<StashRecoverySheet>,
    /// Per-surface decision rows derived from the sheets.
    pub rows: Vec<StashRecoveryDecisionRow>,
    /// Redaction-safe support-export projection.
    pub support_export: GitStashRecoveryReviewSupportExport,
}

impl GitStashRecoveryReviewPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GitStashRecoveryReviewError`] when the JSON is invalid or the parsed
    /// packet violates the review contract.
    pub fn parse_json(input: &str) -> Result<Self, GitStashRecoveryReviewError> {
        let packet: Self =
            serde_json::from_str(input).map_err(GitStashRecoveryReviewError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(GitStashRecoveryReviewError::Validation(violations))
        }
    }

    /// Validates every row, embedded sheet, and support-export invariant.
    pub fn validate(&self) -> Vec<GitStashRecoveryReviewValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != GIT_STASH_RECOVERY_REVIEW_PACKET_RECORD_KIND {
            errors.push(GitStashRecoveryReviewValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != GIT_STASH_RECOVERY_REVIEW_SCHEMA_VERSION {
            errors.push(GitStashRecoveryReviewValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.repo_ref.trim().is_empty()
        {
            errors.push(GitStashRecoveryReviewValidationError::MissingIdentity);
        }

        // The embedded sheets must themselves pass the Git stash/recovery contract,
        // so the review surface never explains a sheet the Git layer would reject.
        let embedded = StashRecoveryPacket {
            record_kind: STASH_RECOVERY_PACKET_RECORD_KIND.to_owned(),
            schema_version: STASH_RECOVERY_SCHEMA_VERSION,
            packet_id: format!("{}::embedded", self.packet_id),
            generated_at: self.generated_at.clone(),
            repo_ref: self.repo_ref.clone(),
            sheets: self.sheets.clone(),
            support_export: StashRecoverySupportExport {
                record_kind: STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
                export_id: format!("{}::embedded-export", self.packet_id),
                sheet_refs: self
                    .sheets
                    .iter()
                    .map(|sheet| sheet.sheet_id.clone())
                    .collect(),
                reconstruction_fields: STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS
                    .iter()
                    .map(|field| (*field).to_owned())
                    .collect(),
                raw_paths_redacted: true,
                raw_patch_bodies_redacted: true,
                raw_provider_payloads_redacted: true,
            },
        };
        for violation in embedded.validate() {
            errors.push(
                GitStashRecoveryReviewValidationError::EmbeddedSheetInvalid {
                    detail: violation.to_string(),
                },
            );
        }

        let sheets_by_id: std::collections::HashMap<&str, &StashRecoverySheet> = self
            .sheets
            .iter()
            .map(|sheet| (sheet.sheet_id.as_str(), sheet))
            .collect();

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if row.record_kind != GIT_STASH_RECOVERY_REVIEW_ROW_RECORD_KIND {
                errors.push(GitStashRecoveryReviewValidationError::WrongRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(GitStashRecoveryReviewValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }

            let Some(sheet) = sheets_by_id.get(row.sheet_ref.as_str()) else {
                errors.push(GitStashRecoveryReviewValidationError::UnknownSheetRef {
                    row_id: row.row_id.clone(),
                    sheet_ref: row.sheet_ref.clone(),
                });
                continue;
            };

            // The row must equal the deterministic projection of its sheet; this is
            // what proves the same decision drives every surface.
            let expected = StashRecoveryDecisionRow::for_surface_and_sheet(
                row.surface,
                sheet,
                row.row_id.clone(),
            );
            if &expected != row {
                errors.push(
                    GitStashRecoveryReviewValidationError::RowDoesNotMatchSheet {
                        row_id: row.row_id.clone(),
                    },
                );
            }

            // Guardrail: a read-only surface never marks a verb executable.
            if !row.surface.is_mutation_surface() && row.execution_permitted {
                errors.push(
                    GitStashRecoveryReviewValidationError::ReadOnlySurfaceExecutable {
                        row_id: row.row_id.clone(),
                    },
                );
            }

            // Guardrail: execution is permitted only for an allowed decision.
            if row.execution_permitted && row.outcome != StashRecoveryOutcome::Allowed {
                errors.push(
                    GitStashRecoveryReviewValidationError::ExecutionWithoutAllowedDecision {
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
        serde_json::to_string_pretty(self).expect("git stash recovery review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Stash & Recovery Decision Rows\n\n");
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
/// Returns [`GitStashRecoveryReviewError`] when the checked-in packet fails to parse
/// or violates the review contract.
pub fn current_git_stash_recovery_review_packet(
) -> Result<GitStashRecoveryReviewPacket, GitStashRecoveryReviewError> {
    GitStashRecoveryReviewPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/stash_recovery/git_stash_recovery_review.json"
    )))
}

fn validate_support_export(
    packet: &GitStashRecoveryReviewPacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<GitStashRecoveryReviewValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != GIT_STASH_RECOVERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(GitStashRecoveryReviewValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(
                GitStashRecoveryReviewValidationError::UnknownSupportRowRef {
                    row_ref: row_ref.clone(),
                },
            );
        }
    }
    for required in GIT_STASH_RECOVERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                GitStashRecoveryReviewValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted
        || !export.raw_patch_bodies_redacted
        || !export.raw_provider_payloads_redacted
    {
        errors.push(GitStashRecoveryReviewValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Error returned while parsing a review packet.
#[derive(Debug)]
pub enum GitStashRecoveryReviewError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<GitStashRecoveryReviewValidationError>),
}

impl fmt::Display for GitStashRecoveryReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse git stash recovery review packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "git stash recovery review packet has validation errors: "
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

impl Error for GitStashRecoveryReviewError {}

/// Cross-row validation error for a review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitStashRecoveryReviewValidationError {
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
    /// An embedded stash/recovery sheet fails the Git contract.
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
    /// A read-only surface marks a verb executable.
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

impl fmt::Display for GitStashRecoveryReviewValidationError {
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
                write!(formatter, "embedded stash/recovery sheet invalid: {detail}")
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
                "read-only review row {row_id} marks a verb executable"
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
