//! Topology-aware AI context assembly.
//!
//! When AI assembles context for a repository, flattening topology is a
//! correctness hazard, not just a UX one: an unfetched object, a pointer-only
//! asset, or an uninitialized submodule that is silently treated as "the file is
//! empty / absent" makes the model reason over a lie, and merging a parent and a
//! child repository into one undifferentiated context erases the boundary the
//! user is actually working across.
//!
//! This module propagates the canonical [`aureline_git`] topology truth into the
//! AI-context surface. Each [`AiContextSliceRow`] is derived from the
//! deterministic [`SurfaceTopologyBinding`] that
//! [`aureline_git::TopologyRootDescriptor::project`] produces for the
//! [`TopologyConsumerSurface::AiContext`] surface, so AI context reads the *same*
//! boundary search, review, blame, and support/export read.
//!
//! Two invariants protect the model:
//!
//! * [`AiContextSliceRow::admit_body_to_prompt`] tracks the binding's
//!   body-export gate, so only complete, in-scope, hydrated content is ever
//!   admitted as authoritative prompt material — a pointer-only or unfetched
//!   slice is named, not pasted in as if it were the file.
//! * [`AiContextSliceRow::crosses_repo_boundary`] stays visible whenever the slice
//!   belongs to a different root than the active one, so the assembled context
//!   never folds a parent and a child repository into one scope.
//!
//! Each limited slice also carries the reviewed remediation verb
//! ([`TopologyActionKind`]) the user could take, the same verb the Git action
//! sheets, search scope, and review lanes surface. The rows are read-only:
//! assembling context never mutates state.
//!
//! The boundary schema is
//! [`schemas/git/ai_topology_context.schema.json`](../../../../schemas/git/ai_topology_context.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/git_topology/topology_propagation/ai_topology_context.json`](../../../../artifacts/git/m5/git_topology/topology_propagation/ai_topology_context.json).

#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    SurfaceResultTruth, SurfaceTopologyBinding, TopologyActionKind, TopologyConsumerSurface,
};

/// Schema version for [`AiTopologyContextPacket`].
pub const AI_TOPOLOGY_CONTEXT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`AiTopologyContextPacket`].
pub const AI_TOPOLOGY_CONTEXT_PACKET_RECORD_KIND: &str = "ai_topology_context_packet";

/// Stable record-kind tag carried by [`AiContextSliceRow`].
pub const AI_TOPOLOGY_CONTEXT_ROW_RECORD_KIND: &str = "ai_topology_context_slice_row";

/// Stable record-kind tag carried by [`AiTopologyContextSupportExport`].
pub const AI_TOPOLOGY_CONTEXT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "ai_topology_context_support_export";

/// Repo-relative path of the boundary schema.
pub const AI_TOPOLOGY_CONTEXT_SCHEMA_REF: &str = "schemas/git/ai_topology_context.schema.json";

/// Repo-relative path of the checked-in canonical packet.
pub const AI_TOPOLOGY_CONTEXT_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/topology_propagation/ai_topology_context.json";

/// Reconstruction fields a support export must retain after redaction.
pub const AI_TOPOLOGY_CONTEXT_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 6] = [
    "binding_ref",
    "result_truth",
    "content_is_authoritative",
    "admit_body_to_prompt",
    "crosses_repo_boundary",
    "remediation_action",
];

/// The reviewed remediation verb an AI-context limit calls for, if any.
///
/// The mapping mirrors `aureline_git::topology_actions` exactly, so an AI-context
/// slice surfaces the same widen/deepen/initialize/hydrate verb the Git action
/// sheets, search scope, and review lanes surface.
#[must_use]
pub fn ai_remediation_for(truth: SurfaceResultTruth) -> Option<TopologyActionKind> {
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

/// One topology-aware AI-context slice derived from a surface binding.
///
/// A context-assembly pass renders one row per repository root contributing to
/// the prompt. The row carries the explicit topology truth so the model never
/// reasons over a topology gap as if it were genuine absence, and never crosses a
/// repository boundary it cannot see.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiContextSliceRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Referenced [`SurfaceTopologyBinding::binding_id`].
    pub binding_ref: String,
    /// Topology result truth copied from the binding.
    pub result_truth: SurfaceResultTruth,
    /// Whether this slice is complete, in-scope truth the model may treat as
    /// authoritative. True only when the result truth is
    /// [`SurfaceResultTruth::Complete`]; every topology-limited slice keeps this
    /// false so the model treats it as a disclosed partial, not as absence.
    pub content_is_authoritative: bool,
    /// Whether the slice body may be admitted into the prompt as authoritative
    /// material. Tracks the binding's body-export gate, so pointer-only and
    /// unfetched slices are named rather than pasted in.
    pub admit_body_to_prompt: bool,
    /// Whether this slice belongs to a different root than the active one, kept
    /// visible so context assembly never folds parent and child repos into one.
    pub crosses_repo_boundary: bool,
    /// Reviewed remediation verb the slice advertises, if any. Advisory only.
    pub remediation_action: Option<TopologyActionKind>,
    /// Root that actually owns the content.
    pub authoritative_root_ref: String,
}

impl AiContextSliceRow {
    /// Derives the AI-context slice row for one surface binding.
    pub fn for_binding(binding: &SurfaceTopologyBinding, row_id: impl Into<String>) -> Self {
        let complete = matches!(binding.result_truth, SurfaceResultTruth::Complete);
        Self {
            record_kind: AI_TOPOLOGY_CONTEXT_ROW_RECORD_KIND.to_owned(),
            row_id: row_id.into(),
            binding_ref: binding.binding_id.clone(),
            result_truth: binding.result_truth,
            content_is_authoritative: complete,
            admit_body_to_prompt: binding.body_export_allowed,
            crosses_repo_boundary: binding.active_root_ref != binding.authoritative_root_ref,
            remediation_action: ai_remediation_for(binding.result_truth),
            authoritative_root_ref: binding.authoritative_root_ref.clone(),
        }
    }
}

/// Redaction-safe support-export projection for an AI-context packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTopologyContextSupportExport {
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

/// Top-level packet binding AI-context slices to the surface bindings they read.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiTopologyContextPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Canonical AI-context surface bindings the rows are derived from.
    pub bindings: Vec<SurfaceTopologyBinding>,
    /// Per-root topology-aware AI-context slices.
    pub rows: Vec<AiContextSliceRow>,
    /// Redaction-safe support-export projection.
    pub support_export: AiTopologyContextSupportExport,
}

impl AiTopologyContextPacket {
    /// Builds a packet from the AI-context bindings of a topology map.
    ///
    /// Only [`TopologyConsumerSurface::AiContext`] bindings are consumed; the row
    /// order follows the binding order.
    pub fn from_ai_context_bindings(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        export_id: impl Into<String>,
        bindings: Vec<SurfaceTopologyBinding>,
    ) -> Self {
        let rows: Vec<AiContextSliceRow> = bindings
            .iter()
            .map(|binding| {
                AiContextSliceRow::for_binding(
                    binding,
                    format!("ai-context-{}", binding.binding_id),
                )
            })
            .collect();
        let support_export = AiTopologyContextSupportExport {
            record_kind: AI_TOPOLOGY_CONTEXT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            export_id: export_id.into(),
            row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
            reconstruction_fields: AI_TOPOLOGY_CONTEXT_REQUIRED_RECONSTRUCTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            raw_paths_redacted: true,
            raw_object_bytes_redacted: true,
        };
        Self {
            record_kind: AI_TOPOLOGY_CONTEXT_PACKET_RECORD_KIND.to_owned(),
            schema_version: AI_TOPOLOGY_CONTEXT_SCHEMA_VERSION,
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
    /// Returns [`AiTopologyContextError`] when the JSON is invalid or the parsed
    /// packet violates the AI-context contract.
    pub fn parse_json(input: &str) -> Result<Self, AiTopologyContextError> {
        let packet: Self = serde_json::from_str(input).map_err(AiTopologyContextError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(AiTopologyContextError::Validation(violations))
        }
    }

    /// Validates every binding, row, and support-export invariant.
    pub fn validate(&self) -> Vec<AiTopologyContextValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != AI_TOPOLOGY_CONTEXT_PACKET_RECORD_KIND {
            errors.push(AiTopologyContextValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != AI_TOPOLOGY_CONTEXT_SCHEMA_VERSION {
            errors.push(AiTopologyContextValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            errors.push(AiTopologyContextValidationError::MissingIdentity);
        }

        let mut binding_ids: HashSet<&str> = HashSet::new();
        let mut bindings_by_id: HashMap<&str, &SurfaceTopologyBinding> = HashMap::new();
        for binding in &self.bindings {
            if !binding_ids.insert(binding.binding_id.as_str()) {
                errors.push(AiTopologyContextValidationError::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if binding.surface != TopologyConsumerSurface::AiContext {
                errors.push(AiTopologyContextValidationError::BindingWrongSurface {
                    binding_id: binding.binding_id.clone(),
                    surface: binding.surface.as_str().to_owned(),
                });
            }
            bindings_by_id.insert(binding.binding_id.as_str(), binding);
        }

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if row.record_kind != AI_TOPOLOGY_CONTEXT_ROW_RECORD_KIND {
                errors.push(AiTopologyContextValidationError::WrongRecordKind {
                    observed: row.record_kind.clone(),
                });
            }
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(AiTopologyContextValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }

            let Some(binding) = bindings_by_id.get(row.binding_ref.as_str()) else {
                errors.push(AiTopologyContextValidationError::UnknownBindingRef {
                    row_id: row.row_id.clone(),
                    binding_ref: row.binding_ref.clone(),
                });
                continue;
            };

            // The row must equal the deterministic derivation of its binding.
            let expected = AiContextSliceRow::for_binding(binding, row.row_id.clone());
            if &expected != row {
                errors.push(AiTopologyContextValidationError::RowDoesNotMatchBinding {
                    row_id: row.row_id.clone(),
                });
            }

            // Decisive guardrail: a topology-limited slice never enters the prompt
            // as authoritative material.
            if (row.admit_body_to_prompt || row.content_is_authoritative)
                && !matches!(row.result_truth, SurfaceResultTruth::Complete)
            {
                errors.push(
                    AiTopologyContextValidationError::LimitedSliceAdmittedAsTruth {
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
        serde_json::to_string_pretty(self).expect("ai topology context packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology-Aware AI Context\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Bindings: {} / Rows: {}\n\n",
            self.bindings.len(),
            self.rows.len()
        ));
        out.push_str("## Slices\n\n");
        for row in &self.rows {
            let remediate = row
                .remediation_action
                .map_or("none", TopologyActionKind::as_str);
            out.push_str(&format!(
                "- `{}` → owner `{}`: truth `{}`, admit-body {}, cross-root {}, remediate `{}`\n",
                row.binding_ref,
                row.authoritative_root_ref,
                result_truth_token(row.result_truth),
                row.admit_body_to_prompt,
                row.crosses_repo_boundary,
                remediate,
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical AI-context packet.
///
/// # Errors
///
/// Returns [`AiTopologyContextError`] when the checked-in packet fails to parse or
/// violates the AI-context contract.
pub fn current_ai_topology_context_packet(
) -> Result<AiTopologyContextPacket, AiTopologyContextError> {
    AiTopologyContextPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/topology_propagation/ai_topology_context.json"
    )))
}

fn validate_support_export(
    packet: &AiTopologyContextPacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<AiTopologyContextValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != AI_TOPOLOGY_CONTEXT_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(AiTopologyContextValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(AiTopologyContextValidationError::UnknownSupportRowRef {
                row_ref: row_ref.clone(),
            });
        }
    }
    for required in AI_TOPOLOGY_CONTEXT_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                AiTopologyContextValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(AiTopologyContextValidationError::SupportExportEmbedsRawMaterial);
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

/// Error returned while parsing an AI-context packet.
#[derive(Debug)]
pub enum AiTopologyContextError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<AiTopologyContextValidationError>),
}

impl fmt::Display for AiTopologyContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse ai topology context packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "ai topology context packet has validation errors: "
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

impl Error for AiTopologyContextError {}

/// Cross-row validation error for an AI-context packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiTopologyContextValidationError {
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
    /// A binding is not an AI-context projection.
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
    /// A topology-limited slice would enter the prompt as authoritative material.
    LimitedSliceAdmittedAsTruth {
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

impl fmt::Display for AiTopologyContextValidationError {
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
                "binding {binding_id} projects surface {surface}, not ai_context"
            ),
            Self::DuplicateRowId { row_id } => {
                write!(formatter, "row id {row_id} is declared more than once")
            }
            Self::UnknownBindingRef {
                row_id,
                binding_ref,
            } => write!(
                formatter,
                "ai context row {row_id} references unknown binding {binding_ref}"
            ),
            Self::RowDoesNotMatchBinding { row_id } => write!(
                formatter,
                "ai context row {row_id} does not match its binding derivation"
            ),
            Self::LimitedSliceAdmittedAsTruth { row_id } => write!(
                formatter,
                "ai context row {row_id} admits a topology-limited slice as authoritative truth"
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
