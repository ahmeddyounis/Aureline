//! Topology-aware review sheets for the search, review, blame, and AI lanes.
//!
//! When a repository is sparse, shallow, partial, submodule-backed, or
//! pointer-backed, the search, review, blame, and AI lanes used to fall back to a
//! generic empty result or a generic error. This module replaces that fallback
//! with an explicit, lane-facing **review sheet row**: each row names the exact
//! topology scope limit ([`ScopeLimitLabel`]) and, when a reviewed remediation
//! exists, advertises which [`TopologyActionKind`] would widen, deepen,
//! initialize, or hydrate the missing content.
//!
//! The rows are *advisory*. A search, review, or AI lane may recommend widening
//! scope, but it never mutates state to do so: every row carries
//! `mutation_applied = false`, and the reviewed [`TopologyActionSheet`] it points
//! at keeps its own approval and no-wrong-root guards. The lane only surfaces the
//! limit and the offer; the user (or a separately reviewed action) decides.
//!
//! The embedded sheets are the canonical [`TopologyActionSheet`] objects produced
//! by [`aureline_git::topology_actions`], so the review lanes recommend the exact
//! same reviewed action the Git surfaces do — there is one topology-action truth,
//! not a search copy and a review copy that can drift.
//!
//! The boundary schema is
//! [`schemas/git/git_topology_review.schema.json`](../../../../schemas/git/git_topology_review.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/git_topology/git_topology_review.json`](../../../../artifacts/git/m5/git_topology/git_topology_review.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    SurfaceResultTruth, TopologyActionKind, TopologyActionReviewPacket, TopologyActionSheet,
    TopologyActionSupportExport, WrongRootGuard, TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS,
    TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND, TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION,
    TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND,
};

/// Schema version for [`GitTopologyReviewPacket`].
pub const GIT_TOPOLOGY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`GitTopologyReviewPacket`].
pub const GIT_TOPOLOGY_REVIEW_PACKET_RECORD_KIND: &str = "git_topology_review_packet";

/// Stable record-kind tag carried by [`TopologyReviewSheetRow`].
pub const GIT_TOPOLOGY_REVIEW_ROW_RECORD_KIND: &str = "git_topology_review_sheet_row";

/// Stable record-kind tag carried by [`GitTopologyReviewSupportExport`].
pub const GIT_TOPOLOGY_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "git_topology_review_support_export";

/// Repo-relative path of the boundary schema.
pub const GIT_TOPOLOGY_REVIEW_SCHEMA_REF: &str = "schemas/git/git_topology_review.schema.json";

/// Repo-relative path of the checked-in canonical review packet.
pub const GIT_TOPOLOGY_REVIEW_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/git_topology_review.json";

/// Reconstruction fields a support export must retain after redaction.
pub const GIT_TOPOLOGY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 5] = [
    "lane",
    "sheet_ref",
    "scope_limit_label",
    "recommended_action",
    "mutation_applied",
];

/// A lane that renders topology-aware review sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyReviewLane {
    /// Code/content search results and zero-result rows.
    Search,
    /// Review diff, summary, and publish rows.
    Review,
    /// Blame and file-history rows.
    Blame,
    /// AI-context assembly and evidence inspectors.
    Ai,
}

impl TopologyReviewLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 4] = [Self::Search, Self::Review, Self::Blame, Self::Ai];

    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Review => "review",
            Self::Blame => "blame",
            Self::Ai => "ai",
        }
    }
}

/// Explicit, lane-facing label for a topology scope limit.
///
/// A lane shows one of these instead of a generic empty or error state, so a user
/// always learns *why* content is missing rather than seeing "no results".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeLimitLabel {
    /// Content exists but is outside the active sparse/workset slice.
    OmittedOutsideSlice,
    /// A known object is referenced but not materialized locally.
    Unfetched,
    /// A submodule child is not initialized.
    Uninitialized,
    /// History or blame stopped at a shallow boundary.
    ShallowBounded,
    /// Only Git LFS pointer metadata is available.
    PointerOnly,
    /// The lane targeted a root other than the content's owning root.
    WrongTargetRoot,
    /// The content belongs to a nested independent root.
    NestedBoundary,
}

impl ScopeLimitLabel {
    /// Stable token used by fixtures, schemas, and support packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OmittedOutsideSlice => "omitted_outside_slice",
            Self::Unfetched => "unfetched",
            Self::Uninitialized => "uninitialized",
            Self::ShallowBounded => "shallow_bounded",
            Self::PointerOnly => "pointer_only",
            Self::WrongTargetRoot => "wrong_target_root",
            Self::NestedBoundary => "nested_boundary",
        }
    }

    /// Derives the lane-facing label a sheet implies.
    ///
    /// A wrong-root guarded sheet surfaces the wrong-root/nested boundary; an
    /// in-scope sheet surfaces the partial state it repairs.
    pub fn for_sheet(sheet: &TopologyActionSheet) -> Self {
        match sheet.wrong_root_guard {
            WrongRootGuard::RetargetRequiredWrongRoot => return Self::WrongTargetRoot,
            WrongRootGuard::BlockedNestedBoundary => return Self::NestedBoundary,
            WrongRootGuard::TargetMatchesAuthoritativeRoot => {}
        }
        match sheet.object_scope.pre_action_truth {
            SurfaceResultTruth::OutsideCurrentSlice => Self::OmittedOutsideSlice,
            SurfaceResultTruth::NotFetched => Self::Unfetched,
            SurfaceResultTruth::Uninitialized => Self::Uninitialized,
            SurfaceResultTruth::ShallowBoundary => Self::ShallowBounded,
            SurfaceResultTruth::PointerOnly => Self::PointerOnly,
            SurfaceResultTruth::NestedRoot => Self::NestedBoundary,
            SurfaceResultTruth::WrongTargetRoot => Self::WrongTargetRoot,
            // Generated/excluded, unavailable, and complete states never produce a
            // remediation sheet, so they cannot reach this path; default honestly.
            _ => Self::Unfetched,
        }
    }
}

/// One topology-aware review row for a lane over a single remediation sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyReviewSheetRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Lane that renders this row.
    pub lane: TopologyReviewLane,
    /// Referenced [`TopologyActionSheet::sheet_id`].
    pub sheet_ref: String,
    /// Explicit scope-limit label shown in place of a generic empty/error state.
    pub scope_limit_label: ScopeLimitLabel,
    /// Reviewed remediation the lane recommends, if any. Advisory only.
    pub recommended_action: Option<TopologyActionKind>,
    /// Always false: a lane never mutates state to widen scope implicitly.
    pub mutation_applied: bool,
    /// Always true: the lane suppresses the generic empty/error fallback.
    pub generic_state_suppressed: bool,
}

impl TopologyReviewSheetRow {
    /// Builds the review row a lane renders for a remediation sheet.
    ///
    /// The row recommends the sheet's action verb only when the sheet is in scope
    /// (not wrong-root guarded); a wrong-root sheet surfaces the limit and asks the
    /// user to retarget rather than recommending an action against the wrong root.
    pub fn for_lane_and_sheet(
        lane: TopologyReviewLane,
        sheet: &TopologyActionSheet,
        row_id: impl Into<String>,
    ) -> Self {
        let scope_limit_label = ScopeLimitLabel::for_sheet(sheet);
        let recommended_action = if sheet.wrong_root_guard.blocks() {
            None
        } else {
            Some(sheet.action_kind)
        };
        Self {
            record_kind: GIT_TOPOLOGY_REVIEW_ROW_RECORD_KIND.to_owned(),
            row_id: row_id.into(),
            lane,
            sheet_ref: sheet.sheet_id.clone(),
            scope_limit_label,
            recommended_action,
            mutation_applied: false,
            generic_state_suppressed: true,
        }
    }
}

/// Redaction-safe support-export projection for a review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitTopologyReviewSupportExport {
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
    /// True when no raw object bytes are embedded.
    pub raw_object_bytes_redacted: bool,
}

/// Top-level packet binding lane review rows to the sheets they review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GitTopologyReviewPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Reviewed remediation sheets the rows point at.
    pub action_sheets: Vec<TopologyActionSheet>,
    /// Per-lane topology-aware review rows.
    pub rows: Vec<TopologyReviewSheetRow>,
    /// Redaction-safe support-export projection.
    pub support_export: GitTopologyReviewSupportExport,
}

impl GitTopologyReviewPacket {
    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`GitTopologyReviewError`] when the JSON is invalid or the parsed
    /// packet violates the review contract.
    pub fn parse_json(input: &str) -> Result<Self, GitTopologyReviewError> {
        let packet: Self = serde_json::from_str(input).map_err(GitTopologyReviewError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(GitTopologyReviewError::Validation(violations))
        }
    }

    /// Validates every row, embedded sheet, and support-export invariant.
    pub fn validate(&self) -> Vec<GitTopologyReviewValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != GIT_TOPOLOGY_REVIEW_PACKET_RECORD_KIND {
            errors.push(GitTopologyReviewValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != GIT_TOPOLOGY_REVIEW_SCHEMA_VERSION {
            errors.push(GitTopologyReviewValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            errors.push(GitTopologyReviewValidationError::MissingIdentity);
        }

        // The embedded sheets must themselves be a valid action-review packet, so
        // the review lanes never recommend an action that would not pass the Git
        // contract's no-wrong-root and reviewed-network guards.
        let embedded = TopologyActionReviewPacket {
            record_kind: TOPOLOGY_ACTION_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: TOPOLOGY_ACTION_REVIEW_SCHEMA_VERSION,
            packet_id: format!("{}::embedded", self.packet_id),
            generated_at: self.generated_at.clone(),
            sheets: self.action_sheets.clone(),
            support_export: TopologyActionSupportExport {
                record_kind: TOPOLOGY_ACTION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
                export_id: format!("{}::embedded-export", self.packet_id),
                sheet_refs: self
                    .action_sheets
                    .iter()
                    .map(|sheet| sheet.sheet_id.clone())
                    .collect(),
                action_kinds: self
                    .action_sheets
                    .iter()
                    .map(|sheet| sheet.action_kind)
                    .collect(),
                reconstruction_fields: TOPOLOGY_ACTION_REQUIRED_RECONSTRUCTION_FIELDS
                    .iter()
                    .map(|field| (*field).to_owned())
                    .collect(),
                raw_paths_redacted: true,
                raw_object_bytes_redacted: true,
            },
        };
        for violation in embedded.validate() {
            errors.push(GitTopologyReviewValidationError::EmbeddedSheetInvalid {
                detail: violation.to_string(),
            });
        }

        let sheets_by_id: std::collections::HashMap<&str, &TopologyActionSheet> = self
            .action_sheets
            .iter()
            .map(|sheet| (sheet.sheet_id.as_str(), sheet))
            .collect();

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if row.record_kind != GIT_TOPOLOGY_REVIEW_ROW_RECORD_KIND {
                errors.push(GitTopologyReviewValidationError::WrongRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(GitTopologyReviewValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }

            // A lane never mutates implicitly, and never hides behind a generic
            // empty/error state.
            if row.mutation_applied {
                errors.push(GitTopologyReviewValidationError::MutationAppliedInLane {
                    row_id: row.row_id.clone(),
                });
            }
            if !row.generic_state_suppressed {
                errors.push(
                    GitTopologyReviewValidationError::GenericStateNotSuppressed {
                        row_id: row.row_id.clone(),
                    },
                );
            }

            let Some(sheet) = sheets_by_id.get(row.sheet_ref.as_str()) else {
                errors.push(GitTopologyReviewValidationError::UnknownSheetRef {
                    row_id: row.row_id.clone(),
                    sheet_ref: row.sheet_ref.clone(),
                });
                continue;
            };

            // The label must reflect the reviewed sheet, not a generic guess.
            let expected_label = ScopeLimitLabel::for_sheet(sheet);
            if row.scope_limit_label != expected_label {
                errors.push(GitTopologyReviewValidationError::LabelMismatch {
                    row_id: row.row_id.clone(),
                    expected: expected_label,
                    observed: row.scope_limit_label,
                });
            }

            // A recommendation is advisory and must match the reviewed verb; a
            // wrong-root sheet recommends nothing (the user must retarget).
            match row.recommended_action {
                Some(action) => {
                    if sheet.wrong_root_guard.blocks() {
                        errors.push(
                            GitTopologyReviewValidationError::RecommendationAcrossWrongRoot {
                                row_id: row.row_id.clone(),
                            },
                        );
                    } else if action != sheet.action_kind {
                        errors.push(GitTopologyReviewValidationError::RecommendationMismatch {
                            row_id: row.row_id.clone(),
                        });
                    }
                }
                None => {}
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
        serde_json::to_string_pretty(self).expect("git topology review packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology-Aware Review Sheets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Sheets: {} / Rows: {}\n\n",
            self.action_sheets.len(),
            self.rows.len()
        ));
        out.push_str("## Rows\n\n");
        for row in &self.rows {
            let recommend = row
                .recommended_action
                .map_or("none", |action| action.as_str());
            out.push_str(&format!(
                "- **{}** → `{}`: limit `{}`, recommend `{}`, mutates {}\n",
                row.lane.as_str(),
                row.sheet_ref,
                row.scope_limit_label.as_str(),
                recommend,
                row.mutation_applied,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical review packet.
///
/// # Errors
///
/// Returns [`GitTopologyReviewError`] when the checked-in packet fails to parse or
/// violates the review contract.
pub fn current_git_topology_review_packet(
) -> Result<GitTopologyReviewPacket, GitTopologyReviewError> {
    GitTopologyReviewPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/git_topology_review.json"
    )))
}

fn validate_support_export(
    packet: &GitTopologyReviewPacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<GitTopologyReviewValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != GIT_TOPOLOGY_REVIEW_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(GitTopologyReviewValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(GitTopologyReviewValidationError::UnknownSupportRowRef {
                row_ref: row_ref.clone(),
            });
        }
    }
    for required in GIT_TOPOLOGY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                GitTopologyReviewValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(GitTopologyReviewValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Error returned while parsing a review packet.
#[derive(Debug)]
pub enum GitTopologyReviewError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<GitTopologyReviewValidationError>),
}

impl fmt::Display for GitTopologyReviewError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse git topology review packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "git topology review packet has validation errors: "
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

impl Error for GitTopologyReviewError {}

/// Cross-row validation error for a review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitTopologyReviewValidationError {
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
    /// An embedded action sheet fails the Git action-review contract.
    EmbeddedSheetInvalid {
        /// Human-readable detail from the Git validator.
        detail: String,
    },
    /// A row id is declared more than once.
    DuplicateRowId {
        /// Duplicated row id.
        row_id: String,
    },
    /// A lane row mutates state to widen scope implicitly.
    MutationAppliedInLane {
        /// Row id.
        row_id: String,
    },
    /// A lane row falls back to a generic empty/error state.
    GenericStateNotSuppressed {
        /// Row id.
        row_id: String,
    },
    /// A row references a sheet not present in the packet.
    UnknownSheetRef {
        /// Row id.
        row_id: String,
        /// Unknown sheet ref.
        sheet_ref: String,
    },
    /// A row's scope-limit label does not match the reviewed sheet.
    LabelMismatch {
        /// Row id.
        row_id: String,
        /// Expected label.
        expected: ScopeLimitLabel,
        /// Observed label.
        observed: ScopeLimitLabel,
    },
    /// A row recommends an action against a wrong-root sheet.
    RecommendationAcrossWrongRoot {
        /// Row id.
        row_id: String,
    },
    /// A row recommends an action that does not match the reviewed verb.
    RecommendationMismatch {
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
    /// The support export embeds raw paths or raw object bytes.
    SupportExportEmbedsRawMaterial,
}

impl fmt::Display for GitTopologyReviewValidationError {
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
                write!(formatter, "embedded action sheet invalid: {detail}")
            }
            Self::DuplicateRowId { row_id } => {
                write!(formatter, "row id {row_id} is declared more than once")
            }
            Self::MutationAppliedInLane { row_id } => {
                write!(formatter, "review row {row_id} mutates state implicitly")
            }
            Self::GenericStateNotSuppressed { row_id } => write!(
                formatter,
                "review row {row_id} falls back to a generic empty/error state"
            ),
            Self::UnknownSheetRef { row_id, sheet_ref } => write!(
                formatter,
                "review row {row_id} references unknown sheet {sheet_ref}"
            ),
            Self::LabelMismatch {
                row_id,
                expected,
                observed,
            } => write!(
                formatter,
                "review row {row_id} label {} does not match sheet label {}",
                observed.as_str(),
                expected.as_str()
            ),
            Self::RecommendationAcrossWrongRoot { row_id } => write!(
                formatter,
                "review row {row_id} recommends an action across a wrong root"
            ),
            Self::RecommendationMismatch { row_id } => write!(
                formatter,
                "review row {row_id} recommendation does not match the reviewed verb"
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
            Self::SupportExportEmbedsRawMaterial => {
                write!(
                    formatter,
                    "support export embeds raw paths or raw object bytes"
                )
            }
        }
    }
}
