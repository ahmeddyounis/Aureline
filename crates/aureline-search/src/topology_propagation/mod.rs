//! Topology-aware search-scope propagation.
//!
//! Search used to answer a query with a flat result list and a single
//! zero-result state. When a repository is sparse, partially cloned, shallow,
//! submodule-backed, pointer-backed, or generated/vendored, that flattening lies:
//! "no results" silently conflates *content that is outside the active slice*,
//! *objects that were never fetched*, *a submodule that is not initialized*, and
//! *content that genuinely does not exist*.
//!
//! This module propagates the canonical [`aureline_git`] topology truth into the
//! search-scope surface. Each [`SearchScopeRow`] is derived from the deterministic
//! [`SurfaceTopologyBinding`] that
//! [`aureline_git::TopologyRootDescriptor::project`] produces for the
//! [`TopologyConsumerSurface::SearchScope`] surface, so search reads the *same*
//! boundary the Git status, review, blame, AI-context, and support/export surfaces
//! read — there is one topology truth, not a search copy that can drift.
//!
//! The decisive invariant is [`SearchScopeRow::zero_results_means_absent`]: it is
//! true only when the binding's result truth is [`SurfaceResultTruth::Complete`].
//! For every topology-limited root, search reports the explicit limit and the
//! reviewed remediation verb ([`TopologyActionKind`]) that would widen, deepen,
//! initialize, or hydrate the missing content, and never asserts that a topology
//! gap means the content is absent.
//!
//! The rows are advisory and read-only: surfacing a limit and an offer never
//! mutates state. The boundary schema is
//! [`schemas/git/search_topology_scope.schema.json`](../../../../schemas/git/search_topology_scope.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/git_topology/topology_propagation/search_topology_scope.json`](../../../../artifacts/git/m5/git_topology/topology_propagation/search_topology_scope.json).

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    SurfaceResultTruth, SurfaceTopologyBinding, TopologyActionKind, TopologyConsumerSurface,
};

/// Schema version for [`SearchTopologyScopePacket`].
pub const SEARCH_TOPOLOGY_SCOPE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`SearchTopologyScopePacket`].
pub const SEARCH_TOPOLOGY_SCOPE_PACKET_RECORD_KIND: &str = "search_topology_scope_packet";

/// Stable record-kind tag carried by [`SearchScopeRow`].
pub const SEARCH_TOPOLOGY_SCOPE_ROW_RECORD_KIND: &str = "search_topology_scope_row";

/// Stable record-kind tag carried by [`SearchTopologyScopeSupportExport`].
pub const SEARCH_TOPOLOGY_SCOPE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "search_topology_scope_support_export";

/// Repo-relative path of the boundary schema.
pub const SEARCH_TOPOLOGY_SCOPE_SCHEMA_REF: &str = "schemas/git/search_topology_scope.schema.json";

/// Repo-relative path of the checked-in canonical packet.
pub const SEARCH_TOPOLOGY_SCOPE_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/topology_propagation/search_topology_scope.json";

/// Reconstruction fields a support export must retain after redaction.
pub const SEARCH_TOPOLOGY_SCOPE_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 5] = [
    "binding_ref",
    "result_truth",
    "zero_results_means_absent",
    "remediation_action",
    "authoritative_root_ref",
];

/// The reviewed remediation verb a search-scope limit calls for, if any.
///
/// The mapping mirrors `aureline_git::topology_actions` exactly, so a search row
/// recommends the same widen/deepen/initialize/hydrate verb the Git action sheets
/// and the review lanes do. Complete, wrong-root, nested, generated/vendor, and
/// unavailable truths carry no remediation: a user retargets, opens the child
/// root, or accepts the boundary rather than widening the active slice.
#[must_use]
pub fn search_remediation_for(truth: SurfaceResultTruth) -> Option<TopologyActionKind> {
    match truth {
        SurfaceResultTruth::OutsideCurrentSlice => Some(TopologyActionKind::Widen),
        SurfaceResultTruth::ShallowBoundary => Some(TopologyActionKind::Deepen),
        SurfaceResultTruth::Uninitialized => Some(TopologyActionKind::Initialize),
        SurfaceResultTruth::PointerOnly | SurfaceResultTruth::NotFetched => {
            Some(TopologyActionKind::Hydrate)
        }
        SurfaceResultTruth::Complete
        | SurfaceResultTruth::NestedRoot
        | SurfaceResultTruth::GeneratedOrExcluded
        | SurfaceResultTruth::WrongTargetRoot
        | SurfaceResultTruth::Unavailable => None,
    }
}

/// One topology-aware search-scope row derived from a surface binding.
///
/// A search lane renders one row per repository root in scope. The row carries the
/// explicit topology truth instead of folding every limit into a single empty
/// result, so a user always learns *why* a root contributes no matches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchScopeRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Referenced [`SurfaceTopologyBinding::binding_id`].
    pub binding_ref: String,
    /// Topology result truth copied from the binding.
    pub result_truth: SurfaceResultTruth,
    /// Whether a zero-result answer for this root may be reported as genuine
    /// absence. True only when the result truth is
    /// [`SurfaceResultTruth::Complete`]; every topology-limited root keeps this
    /// false so search never treats omitted or unfetched content as absent.
    pub zero_results_means_absent: bool,
    /// Reviewed remediation verb the lane advertises, if any. Advisory only.
    pub remediation_action: Option<TopologyActionKind>,
    /// Root that actually owns the content, kept visible so a cross-root row
    /// points the user at the owning root rather than flattening the boundary.
    pub authoritative_root_ref: String,
}

impl SearchScopeRow {
    /// Derives the search-scope row for one surface binding.
    pub fn for_binding(binding: &SurfaceTopologyBinding, row_id: impl Into<String>) -> Self {
        let complete = matches!(binding.result_truth, SurfaceResultTruth::Complete);
        Self {
            record_kind: SEARCH_TOPOLOGY_SCOPE_ROW_RECORD_KIND.to_owned(),
            row_id: row_id.into(),
            binding_ref: binding.binding_id.clone(),
            result_truth: binding.result_truth,
            zero_results_means_absent: complete,
            remediation_action: search_remediation_for(binding.result_truth),
            authoritative_root_ref: binding.authoritative_root_ref.clone(),
        }
    }
}

/// Redaction-safe support-export projection for a search-scope packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchTopologyScopeSupportExport {
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

/// Top-level packet binding search-scope rows to the surface bindings they read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchTopologyScopePacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Canonical search-scope surface bindings the rows are derived from.
    pub bindings: Vec<SurfaceTopologyBinding>,
    /// Per-root topology-aware search-scope rows.
    pub rows: Vec<SearchScopeRow>,
    /// Redaction-safe support-export projection.
    pub support_export: SearchTopologyScopeSupportExport,
}

impl SearchTopologyScopePacket {
    /// Builds a packet from the search-scope bindings of a topology map.
    ///
    /// Only [`TopologyConsumerSurface::SearchScope`] bindings are consumed; the
    /// row order follows the binding order, so a deterministic map yields a
    /// deterministic packet.
    pub fn from_search_bindings(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        export_id: impl Into<String>,
        bindings: Vec<SurfaceTopologyBinding>,
    ) -> Self {
        let rows: Vec<SearchScopeRow> = bindings
            .iter()
            .map(|binding| {
                SearchScopeRow::for_binding(binding, format!("search-scope-{}", binding.binding_id))
            })
            .collect();
        let support_export = SearchTopologyScopeSupportExport {
            record_kind: SEARCH_TOPOLOGY_SCOPE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            export_id: export_id.into(),
            row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
            reconstruction_fields: SEARCH_TOPOLOGY_SCOPE_REQUIRED_RECONSTRUCTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            raw_paths_redacted: true,
            raw_object_bytes_redacted: true,
        };
        Self {
            record_kind: SEARCH_TOPOLOGY_SCOPE_PACKET_RECORD_KIND.to_owned(),
            schema_version: SEARCH_TOPOLOGY_SCOPE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            bindings,
            rows,
            support_export,
        }
    }

    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`SearchTopologyScopeError`] when the JSON is invalid or the parsed
    /// packet violates the search-scope contract.
    pub fn parse_json(input: &str) -> Result<Self, SearchTopologyScopeError> {
        let packet: Self = serde_json::from_str(input).map_err(SearchTopologyScopeError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(SearchTopologyScopeError::Validation(violations))
        }
    }

    /// Validates every binding, row, and support-export invariant.
    ///
    /// Returns every violation found rather than stopping at the first.
    pub fn validate(&self) -> Vec<SearchTopologyScopeValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != SEARCH_TOPOLOGY_SCOPE_PACKET_RECORD_KIND {
            errors.push(SearchTopologyScopeValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != SEARCH_TOPOLOGY_SCOPE_SCHEMA_VERSION {
            errors.push(SearchTopologyScopeValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            errors.push(SearchTopologyScopeValidationError::MissingIdentity);
        }

        let mut binding_ids: HashSet<&str> = HashSet::new();
        let mut bindings_by_id: HashMap<&str, &SurfaceTopologyBinding> = HashMap::new();
        for binding in &self.bindings {
            if !binding_ids.insert(binding.binding_id.as_str()) {
                errors.push(SearchTopologyScopeValidationError::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // Every binding must be a search-scope projection; a stray surface
            // would mean the packet is reading another surface's truth.
            if binding.surface != TopologyConsumerSurface::SearchScope {
                errors.push(SearchTopologyScopeValidationError::BindingWrongSurface {
                    binding_id: binding.binding_id.clone(),
                    surface: binding.surface.as_str().to_owned(),
                });
            }
            bindings_by_id.insert(binding.binding_id.as_str(), binding);
        }

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if row.record_kind != SEARCH_TOPOLOGY_SCOPE_ROW_RECORD_KIND {
                errors.push(SearchTopologyScopeValidationError::WrongRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(SearchTopologyScopeValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }

            let Some(binding) = bindings_by_id.get(row.binding_ref.as_str()) else {
                errors.push(SearchTopologyScopeValidationError::UnknownBindingRef {
                    row_id: row.row_id.clone(),
                    binding_ref: row.binding_ref.clone(),
                });
                continue;
            };

            // The row must equal the deterministic derivation of its binding; this
            // is what proves search reads the same topology truth every surface does.
            let expected = SearchScopeRow::for_binding(binding, row.row_id.clone());
            if &expected != row {
                errors.push(SearchTopologyScopeValidationError::RowDoesNotMatchBinding {
                    row_id: row.row_id.clone(),
                });
            }

            // Decisive guardrail: a topology-limited root never lets search assert
            // genuine absence.
            if row.zero_results_means_absent
                && !matches!(row.result_truth, SurfaceResultTruth::Complete)
            {
                errors.push(SearchTopologyScopeValidationError::SilentAbsenceOverLimit {
                    row_id: row.row_id.clone(),
                });
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
        serde_json::to_string_pretty(self).expect("search topology scope packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology-Aware Search Scope\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Bindings: {} / Rows: {}\n\n",
            self.bindings.len(),
            self.rows.len()
        ));
        out.push_str("## Rows\n\n");
        for row in &self.rows {
            let remediate = row
                .remediation_action
                .map_or("none", TopologyActionKind::as_str);
            out.push_str(&format!(
                "- `{}` → owner `{}`: truth `{}`, absent-if-empty {}, remediate `{}`\n",
                row.binding_ref,
                row.authoritative_root_ref,
                result_truth_token(row.result_truth),
                row.zero_results_means_absent,
                remediate,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical search-scope packet.
///
/// # Errors
///
/// Returns [`SearchTopologyScopeError`] when the checked-in packet fails to parse
/// or violates the search-scope contract.
pub fn current_search_topology_scope_packet(
) -> Result<SearchTopologyScopePacket, SearchTopologyScopeError> {
    SearchTopologyScopePacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/topology_propagation/search_topology_scope.json"
    )))
}

fn validate_support_export(
    packet: &SearchTopologyScopePacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<SearchTopologyScopeValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != SEARCH_TOPOLOGY_SCOPE_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(SearchTopologyScopeValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(SearchTopologyScopeValidationError::UnknownSupportRowRef {
                row_ref: row_ref.clone(),
            });
        }
    }
    for required in SEARCH_TOPOLOGY_SCOPE_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                SearchTopologyScopeValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(SearchTopologyScopeValidationError::SupportExportEmbedsRawMaterial);
    }
}

/// Stable token for a [`SurfaceResultTruth`] reused from the topology packet.
fn result_truth_token(truth: SurfaceResultTruth) -> &'static str {
    match truth {
        SurfaceResultTruth::Complete => "complete",
        SurfaceResultTruth::OutsideCurrentSlice => "outside_current_slice",
        SurfaceResultTruth::NotFetched => "not_fetched",
        SurfaceResultTruth::ShallowBoundary => "shallow_boundary",
        SurfaceResultTruth::Uninitialized => "uninitialized",
        SurfaceResultTruth::NestedRoot => "nested_root",
        SurfaceResultTruth::PointerOnly => "pointer_only",
        SurfaceResultTruth::GeneratedOrExcluded => "generated_or_excluded",
        SurfaceResultTruth::WrongTargetRoot => "wrong_target_root",
        SurfaceResultTruth::Unavailable => "unavailable",
    }
}

/// Error returned while parsing a search-scope packet.
#[derive(Debug)]
pub enum SearchTopologyScopeError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<SearchTopologyScopeValidationError>),
}

impl fmt::Display for SearchTopologyScopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse search topology scope packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "search topology scope packet has validation errors: "
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

impl Error for SearchTopologyScopeError {}

/// Cross-row validation error for a search-scope packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchTopologyScopeValidationError {
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
    /// A binding id is declared more than once.
    DuplicateBindingId {
        /// Duplicated binding id.
        binding_id: String,
    },
    /// A binding is not a search-scope projection.
    BindingWrongSurface {
        /// Binding id.
        binding_id: String,
        /// Observed surface token.
        surface: String,
    },
    /// A row id is declared more than once.
    DuplicateRowId {
        /// Duplicated row id.
        row_id: String,
    },
    /// A row references a binding not present in the packet.
    UnknownBindingRef {
        /// Row id.
        row_id: String,
        /// Unknown binding ref.
        binding_ref: String,
    },
    /// A row does not equal the derivation of its binding.
    RowDoesNotMatchBinding {
        /// Row id.
        row_id: String,
    },
    /// A topology-limited row would let search assert genuine absence.
    SilentAbsenceOverLimit {
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

impl fmt::Display for SearchTopologyScopeValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "packet is missing identity fields"),
            Self::DuplicateBindingId { binding_id } => {
                write!(
                    formatter,
                    "binding id {binding_id} is declared more than once"
                )
            }
            Self::BindingWrongSurface {
                binding_id,
                surface,
            } => write!(
                formatter,
                "binding {binding_id} projects surface {surface}, not search_scope"
            ),
            Self::DuplicateRowId { row_id } => {
                write!(formatter, "row id {row_id} is declared more than once")
            }
            Self::UnknownBindingRef {
                row_id,
                binding_ref,
            } => write!(
                formatter,
                "search row {row_id} references unknown binding {binding_ref}"
            ),
            Self::RowDoesNotMatchBinding { row_id } => write!(
                formatter,
                "search row {row_id} does not match its binding derivation"
            ),
            Self::SilentAbsenceOverLimit { row_id } => write!(
                formatter,
                "search row {row_id} would treat a topology-limited root as absent"
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
