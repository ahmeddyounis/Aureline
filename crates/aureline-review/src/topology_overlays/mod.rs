//! Topology-aware mutation-review overlays and the multi-root mutation guard.
//!
//! Review is the surface where a change becomes a mutation. When the working set
//! spans more than one repository root — a parent and its submodule, a parent and
//! a nested independent repo, or two linked worktrees — an "apply to everything"
//! convenience is a footgun: it lets an ambient bulk mutation cross a repository
//! boundary the user never explicitly opted into.
//!
//! This module overlays the canonical [`aureline_git`] topology truth onto the
//! mutation-review surface. Each [`MutationRootRow`] is derived from the
//! deterministic [`aureline_git::SurfaceTopologyBinding`] that
//! [`aureline_git::TopologyRootDescriptor::project`] produces for the
//! [`aureline_git::TopologyConsumerSurface::Review`] surface, so parent/child repo
//! identity and worktree/root identity stay visible during mutation review, and a
//! root that is not the active one can never be mutated by the same ambient action
//! (the projection denies wrong-root mutation).
//!
//! The [`MultiRootMutationPreview`] is the guard itself: whenever a proposed
//! mutation set touches more than one root, the preview marks it
//! [`auto_apply_blocked`](MultiRootMutationPreview::auto_apply_blocked),
//! [`opt_in_required`](MultiRootMutationPreview::opt_in_required), and requires the
//! [`TopologyOperationScope::ExplicitMultiRootPreviewRequired`] scope. Cross-root
//! bulk mutation stays preview-first and opt-in; it is never the default.
//!
//! The boundary schema is
//! [`schemas/git/review_topology_overlay.schema.json`](../../../../schemas/git/review_topology_overlay.schema.json).
//! The checked-in canonical packet is
//! [`artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json`](../../../../artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json).

#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_git::{
    RepoIdentityKind, SurfaceResultTruth, TopologyConsumerSurface, TopologyOperationScope,
    TopologyRootDescriptor,
};

/// Schema version for [`ReviewTopologyOverlayPacket`].
pub const REVIEW_TOPOLOGY_OVERLAY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ReviewTopologyOverlayPacket`].
pub const REVIEW_TOPOLOGY_OVERLAY_PACKET_RECORD_KIND: &str = "review_topology_overlay_packet";

/// Stable record-kind tag carried by [`MutationRootRow`].
pub const REVIEW_TOPOLOGY_OVERLAY_ROW_RECORD_KIND: &str = "review_topology_overlay_root_row";

/// Stable record-kind tag carried by [`MultiRootMutationPreview`].
pub const REVIEW_TOPOLOGY_OVERLAY_PREVIEW_RECORD_KIND: &str = "review_multi_root_mutation_preview";

/// Stable record-kind tag carried by [`ReviewTopologyOverlaySupportExport`].
pub const REVIEW_TOPOLOGY_OVERLAY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "review_topology_overlay_support_export";

/// Repo-relative path of the boundary schema.
pub const REVIEW_TOPOLOGY_OVERLAY_SCHEMA_REF: &str =
    "schemas/git/review_topology_overlay.schema.json";

/// Repo-relative path of the checked-in canonical packet.
pub const REVIEW_TOPOLOGY_OVERLAY_ARTIFACT_REF: &str =
    "artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json";

/// Reconstruction fields a support export must retain after redaction.
pub const REVIEW_TOPOLOGY_OVERLAY_REQUIRED_RECONSTRUCTION_FIELDS: [&str; 6] = [
    "active_root_ref",
    "touched_root_refs",
    "spans_multiple_roots",
    "explicit_preview_required",
    "auto_apply_blocked",
    "opt_in_required",
];

/// One root in a proposed mutation set, with its repo identity kept visible.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationRootRow {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable row id.
    pub row_id: String,
    /// Root this row describes.
    pub root_ref: String,
    /// Parent/child repo identity kind.
    pub identity_kind: RepoIdentityKind,
    /// Parent root ref, for child roots.
    pub parent_root_ref: Option<String>,
    /// Whether this root is the caller's active root.
    pub is_active_root: bool,
    /// Result truth the review surface renders for this root.
    pub result_truth: SurfaceResultTruth,
    /// Safe mutation scope for this root.
    pub mutation_scope: TopologyOperationScope,
    /// Whether the review surface may mutate this root in the active action.
    pub mutation_allowed: bool,
}

impl MutationRootRow {
    /// Derives the mutation-review row for one root in the active selection.
    pub fn for_descriptor(
        descriptor: &TopologyRootDescriptor,
        active_root_ref: &str,
        row_id: impl Into<String>,
    ) -> Self {
        let binding = descriptor.project(
            TopologyConsumerSurface::Review,
            active_root_ref,
            format!("review-overlay-binding-{}", descriptor.root_id),
        );
        Self {
            record_kind: REVIEW_TOPOLOGY_OVERLAY_ROW_RECORD_KIND.to_owned(),
            row_id: row_id.into(),
            root_ref: descriptor.root_id.clone(),
            identity_kind: descriptor.repo_identity.kind,
            parent_root_ref: descriptor.repo_identity.parent_root_id.clone(),
            is_active_root: descriptor.root_id == active_root_ref,
            result_truth: binding.result_truth,
            mutation_scope: binding.mutation_scope,
            mutation_allowed: binding.mutation_allowed,
        }
    }
}

/// The cross-root preview that gates a bulk mutation.
///
/// When [`spans_multiple_roots`](Self::spans_multiple_roots) is true, the gate
/// fields are all set: a cross-root mutation requires the explicit multi-root
/// preview scope, blocks auto-apply, and requires opt-in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiRootMutationPreview {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable preview id.
    pub preview_id: String,
    /// Caller's active root.
    pub active_root_ref: String,
    /// Distinct roots the proposed mutation set touches, sorted.
    pub touched_root_refs: Vec<String>,
    /// Whether the mutation set touches more than one root.
    pub spans_multiple_roots: bool,
    /// Whether the set crosses a parent/submodule boundary.
    pub crosses_parent_child_boundary: bool,
    /// Whether the set crosses a nested-independent-repo boundary.
    pub crosses_nested_boundary: bool,
    /// Mutation scope the set requires before any apply.
    pub required_scope: TopologyOperationScope,
    /// Whether an explicit multi-root preview must precede mutation.
    pub explicit_preview_required: bool,
    /// Whether auto-apply across roots is blocked.
    pub auto_apply_blocked: bool,
    /// Whether the user must opt in before a cross-root mutation runs.
    pub opt_in_required: bool,
}

impl MultiRootMutationPreview {
    /// Computes the preview a proposed mutation set across descriptors implies.
    pub fn for_descriptors(
        descriptors: &[TopologyRootDescriptor],
        active_root_ref: &str,
        preview_id: impl Into<String>,
    ) -> Self {
        let mut touched: Vec<String> = descriptors
            .iter()
            .map(|descriptor| descriptor.root_id.clone())
            .collect();
        touched.sort();
        touched.dedup();

        let spans_multiple_roots = touched.len() > 1;
        let has_submodule_child = descriptors
            .iter()
            .any(|descriptor| descriptor.repo_identity.kind == RepoIdentityKind::SubmoduleChild);
        let has_nested = descriptors
            .iter()
            .any(|descriptor| descriptor.repo_identity.kind == RepoIdentityKind::NestedIndependent);

        let crosses_parent_child_boundary = spans_multiple_roots && has_submodule_child;
        let crosses_nested_boundary = spans_multiple_roots && has_nested;

        let required_scope = if spans_multiple_roots {
            TopologyOperationScope::ExplicitMultiRootPreviewRequired
        } else {
            TopologyOperationScope::ActiveRootOnly
        };

        Self {
            record_kind: REVIEW_TOPOLOGY_OVERLAY_PREVIEW_RECORD_KIND.to_owned(),
            preview_id: preview_id.into(),
            active_root_ref: active_root_ref.to_owned(),
            touched_root_refs: touched,
            spans_multiple_roots,
            crosses_parent_child_boundary,
            crosses_nested_boundary,
            required_scope,
            explicit_preview_required: spans_multiple_roots,
            auto_apply_blocked: spans_multiple_roots,
            opt_in_required: spans_multiple_roots,
        }
    }
}

/// Redaction-safe support-export projection for a review overlay packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewTopologyOverlaySupportExport {
    /// Record-kind tag.
    pub record_kind: String,
    /// Stable export id.
    pub export_id: String,
    /// Row ids included in the export.
    pub row_refs: Vec<String>,
    /// Preview id included in the export.
    pub preview_ref: String,
    /// Structured fields retained after redaction.
    pub reconstruction_fields: Vec<String>,
    /// True when no raw paths are embedded.
    pub raw_paths_redacted: bool,
    /// True when no raw object bytes are embedded.
    pub raw_object_bytes_redacted: bool,
}

/// Top-level packet binding a mutation-review overlay to its multi-root preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewTopologyOverlayPacket {
    /// Record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp (RFC 3339).
    pub generated_at: String,
    /// Caller's active root.
    pub active_root_ref: String,
    /// Canonical descriptors of the proposed mutation set.
    pub roots: Vec<TopologyRootDescriptor>,
    /// Per-root mutation-review rows.
    pub rows: Vec<MutationRootRow>,
    /// Cross-root mutation preview gating the apply.
    pub preview: MultiRootMutationPreview,
    /// Redaction-safe support-export projection.
    pub support_export: ReviewTopologyOverlaySupportExport,
}

impl ReviewTopologyOverlayPacket {
    /// Builds a packet for a proposed mutation set across descriptors.
    pub fn from_descriptors(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        export_id: impl Into<String>,
        active_root_ref: impl Into<String>,
        roots: Vec<TopologyRootDescriptor>,
    ) -> Self {
        let active_root_ref = active_root_ref.into();
        let rows: Vec<MutationRootRow> = roots
            .iter()
            .map(|descriptor| {
                MutationRootRow::for_descriptor(
                    descriptor,
                    &active_root_ref,
                    format!("review-overlay-{}", descriptor.root_id),
                )
            })
            .collect();
        let preview = MultiRootMutationPreview::for_descriptors(
            &roots,
            &active_root_ref,
            format!("review-overlay-preview:{active_root_ref}"),
        );
        let support_export = ReviewTopologyOverlaySupportExport {
            record_kind: REVIEW_TOPOLOGY_OVERLAY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            export_id: export_id.into(),
            row_refs: rows.iter().map(|row| row.row_id.clone()).collect(),
            preview_ref: preview.preview_id.clone(),
            reconstruction_fields: REVIEW_TOPOLOGY_OVERLAY_REQUIRED_RECONSTRUCTION_FIELDS
                .iter()
                .map(|field| (*field).to_owned())
                .collect(),
            raw_paths_redacted: true,
            raw_object_bytes_redacted: true,
        };
        Self {
            record_kind: REVIEW_TOPOLOGY_OVERLAY_PACKET_RECORD_KIND.to_owned(),
            schema_version: REVIEW_TOPOLOGY_OVERLAY_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            active_root_ref,
            roots,
            rows,
            preview,
            support_export,
        }
    }

    /// Parses a packet from JSON and validates its cross-row invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewTopologyOverlayError`] when the JSON is invalid or the
    /// parsed packet violates the overlay contract.
    pub fn parse_json(input: &str) -> Result<Self, ReviewTopologyOverlayError> {
        let packet: Self = serde_json::from_str(input).map_err(ReviewTopologyOverlayError::Json)?;
        let violations = packet.validate();
        if violations.is_empty() {
            Ok(packet)
        } else {
            Err(ReviewTopologyOverlayError::Validation(violations))
        }
    }

    /// Validates every root, row, preview, and support-export invariant.
    pub fn validate(&self) -> Vec<ReviewTopologyOverlayValidationError> {
        let mut errors = Vec::new();

        if self.record_kind != REVIEW_TOPOLOGY_OVERLAY_PACKET_RECORD_KIND {
            errors.push(ReviewTopologyOverlayValidationError::WrongRecordKind {
                observed: self.record_kind.clone(),
            });
        }
        if self.schema_version != REVIEW_TOPOLOGY_OVERLAY_SCHEMA_VERSION {
            errors.push(ReviewTopologyOverlayValidationError::WrongSchemaVersion {
                observed: self.schema_version,
            });
        }
        if self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.active_root_ref.trim().is_empty()
        {
            errors.push(ReviewTopologyOverlayValidationError::MissingIdentity);
        }
        if self.roots.is_empty() {
            errors.push(ReviewTopologyOverlayValidationError::NoRoots);
        }

        let mut root_ids: HashSet<&str> = HashSet::new();
        for root in &self.roots {
            if !root_ids.insert(root.root_id.as_str()) {
                errors.push(ReviewTopologyOverlayValidationError::DuplicateRootId {
                    root_id: root.root_id.clone(),
                });
            }
            // Keep the embedded descriptors honest about parent/child identity.
            if root.repo_identity.kind.is_child() && root.repo_identity.parent_root_id.is_none() {
                errors.push(ReviewTopologyOverlayValidationError::ChildMissingParent {
                    root_id: root.root_id.clone(),
                });
            }
        }

        if !root_ids.contains(self.active_root_ref.as_str()) {
            errors.push(ReviewTopologyOverlayValidationError::ActiveRootNotInSet {
                active_root_ref: self.active_root_ref.clone(),
            });
        }

        let mut row_ids: HashSet<&str> = HashSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.as_str()) {
                errors.push(ReviewTopologyOverlayValidationError::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            let Some(root) = self.roots.iter().find(|root| root.root_id == row.root_ref) else {
                errors.push(ReviewTopologyOverlayValidationError::UnknownRowRoot {
                    row_id: row.row_id.clone(),
                    root_ref: row.root_ref.clone(),
                });
                continue;
            };
            // The row must equal the deterministic projection of its descriptor.
            let expected =
                MutationRootRow::for_descriptor(root, &self.active_root_ref, row.row_id.clone());
            if &expected != row {
                errors.push(
                    ReviewTopologyOverlayValidationError::RowDoesNotMatchDescriptor {
                        row_id: row.row_id.clone(),
                    },
                );
            }
            // A non-active root can never be mutated by the active ambient action.
            if !row.is_active_root && row.mutation_allowed {
                errors.push(
                    ReviewTopologyOverlayValidationError::CrossRootMutationAllowed {
                        row_id: row.row_id.clone(),
                    },
                );
            }
        }

        // The preview must equal the deterministic computation over the roots.
        let expected_preview = MultiRootMutationPreview::for_descriptors(
            &self.roots,
            &self.active_root_ref,
            self.preview.preview_id.clone(),
        );
        if expected_preview != self.preview {
            errors.push(ReviewTopologyOverlayValidationError::PreviewMismatch);
        }

        // Decisive guardrail: a cross-root mutation set is never ambiently applied.
        if self.preview.spans_multiple_roots
            && (!self.preview.auto_apply_blocked
                || !self.preview.explicit_preview_required
                || !self.preview.opt_in_required
                || self.preview.required_scope
                    != TopologyOperationScope::ExplicitMultiRootPreviewRequired)
        {
            errors.push(ReviewTopologyOverlayValidationError::AmbientBulkMutationNotGuarded);
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
        serde_json::to_string_pretty(self).expect("review topology overlay packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Topology-Aware Mutation Review Overlay\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Active root: `{}`\n", self.active_root_ref));
        out.push_str(&format!(
            "- Touched roots: {} / Cross-root preview required: {}\n\n",
            self.preview.touched_root_refs.len(),
            self.preview.explicit_preview_required,
        ));
        out.push_str("## Roots\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- `{}` ({}{}): truth `{}`, mutate {}{}\n",
                row.root_ref,
                row.identity_kind.as_str(),
                if row.is_active_root { ", active" } else { "" },
                result_truth_token(row.result_truth),
                row.mutation_allowed,
                row.parent_root_ref
                    .as_ref()
                    .map_or(String::new(), |parent| format!(", parent `{parent}`")),
            ));
        }
        out
    }
}

/// Reads and validates the checked-in canonical review overlay packet.
///
/// # Errors
///
/// Returns [`ReviewTopologyOverlayError`] when the checked-in packet fails to
/// parse or violates the overlay contract.
pub fn current_review_topology_overlay_packet(
) -> Result<ReviewTopologyOverlayPacket, ReviewTopologyOverlayError> {
    ReviewTopologyOverlayPacket::parse_json(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/git/m5/git_topology/topology_propagation/review_topology_overlay.json"
    )))
}

fn validate_support_export(
    packet: &ReviewTopologyOverlayPacket,
    row_ids: &HashSet<&str>,
    errors: &mut Vec<ReviewTopologyOverlayValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != REVIEW_TOPOLOGY_OVERLAY_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(ReviewTopologyOverlayValidationError::WrongRecordKind {
            observed: export.record_kind.clone(),
        });
    }
    for row_ref in &export.row_refs {
        if !row_ids.contains(row_ref.as_str()) {
            errors.push(ReviewTopologyOverlayValidationError::UnknownSupportRowRef {
                row_ref: row_ref.clone(),
            });
        }
    }
    if export.preview_ref != packet.preview.preview_id {
        errors.push(
            ReviewTopologyOverlayValidationError::UnknownSupportPreviewRef {
                preview_ref: export.preview_ref.clone(),
            },
        );
    }
    for required in REVIEW_TOPOLOGY_OVERLAY_REQUIRED_RECONSTRUCTION_FIELDS {
        if !export
            .reconstruction_fields
            .iter()
            .any(|field| field == required)
        {
            errors.push(
                ReviewTopologyOverlayValidationError::SupportExportMissingField {
                    field: required.to_string(),
                },
            );
        }
    }
    if !export.raw_paths_redacted || !export.raw_object_bytes_redacted {
        errors.push(ReviewTopologyOverlayValidationError::SupportExportEmbedsRawMaterial);
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

/// Error returned while parsing a review overlay packet.
#[derive(Debug)]
pub enum ReviewTopologyOverlayError {
    /// JSON parsing failed.
    Json(serde_json::Error),
    /// Cross-row validation failed.
    Validation(Vec<ReviewTopologyOverlayValidationError>),
}

impl fmt::Display for ReviewTopologyOverlayError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => {
                write!(
                    formatter,
                    "failed to parse review topology overlay packet JSON: {error}"
                )
            }
            Self::Validation(errors) => {
                write!(
                    formatter,
                    "review topology overlay packet has validation errors: "
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

impl Error for ReviewTopologyOverlayError {}

/// Cross-row validation error for a review overlay packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewTopologyOverlayValidationError {
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
    /// The packet carries no roots.
    NoRoots,
    /// A root id is declared more than once.
    DuplicateRootId {
        /// Duplicated root id.
        root_id: String,
    },
    /// A child root does not name its parent.
    ChildMissingParent {
        /// Root id.
        root_id: String,
    },
    /// The active root is not part of the mutation set.
    ActiveRootNotInSet {
        /// Active root ref.
        active_root_ref: String,
    },
    /// A row id is declared more than once.
    DuplicateRowId {
        /// Duplicated row id.
        row_id: String,
    },
    /// A row references a root not present in the packet.
    UnknownRowRoot {
        /// Row id.
        row_id: String,
        /// Unknown root ref.
        root_ref: String,
    },
    /// A row does not equal the projection of its descriptor.
    RowDoesNotMatchDescriptor {
        /// Row id.
        row_id: String,
    },
    /// A non-active root permits mutation in the active ambient action.
    CrossRootMutationAllowed {
        /// Row id.
        row_id: String,
    },
    /// The preview does not equal the computation over the roots.
    PreviewMismatch,
    /// A cross-root mutation set is not fully guarded against ambient apply.
    AmbientBulkMutationNotGuarded,
    /// A support-export row ref is unknown.
    UnknownSupportRowRef {
        /// Unknown row ref.
        row_ref: String,
    },
    /// The support-export preview ref does not match the packet preview.
    UnknownSupportPreviewRef {
        /// Unknown preview ref.
        preview_ref: String,
    },
    /// The support export omits a required reconstruction field.
    SupportExportMissingField {
        /// Missing reconstruction field.
        field: String,
    },
    /// The support export embeds raw paths or raw object bytes.
    SupportExportEmbedsRawMaterial,
}

impl fmt::Display for ReviewTopologyOverlayValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { observed } => {
                write!(formatter, "unexpected record kind {observed}")
            }
            Self::WrongSchemaVersion { observed } => {
                write!(formatter, "unsupported schema version {observed}")
            }
            Self::MissingIdentity => write!(formatter, "packet is missing identity fields"),
            Self::NoRoots => write!(formatter, "packet carries no roots"),
            Self::DuplicateRootId { root_id } => {
                write!(formatter, "root id {root_id} is declared more than once")
            }
            Self::ChildMissingParent { root_id } => {
                write!(formatter, "child root {root_id} does not name a parent")
            }
            Self::ActiveRootNotInSet { active_root_ref } => write!(
                formatter,
                "active root {active_root_ref} is not part of the mutation set"
            ),
            Self::DuplicateRowId { row_id } => {
                write!(formatter, "row id {row_id} is declared more than once")
            }
            Self::UnknownRowRoot { row_id, root_ref } => write!(
                formatter,
                "overlay row {row_id} references unknown root {root_ref}"
            ),
            Self::RowDoesNotMatchDescriptor { row_id } => write!(
                formatter,
                "overlay row {row_id} does not match its descriptor projection"
            ),
            Self::CrossRootMutationAllowed { row_id } => write!(
                formatter,
                "overlay row {row_id} permits mutation of a non-active root"
            ),
            Self::PreviewMismatch => {
                write!(formatter, "multi-root preview does not match the roots")
            }
            Self::AmbientBulkMutationNotGuarded => write!(
                formatter,
                "cross-root mutation set is not guarded preview-first and opt-in"
            ),
            Self::UnknownSupportRowRef { row_ref } => {
                write!(formatter, "support export references unknown row {row_ref}")
            }
            Self::UnknownSupportPreviewRef { preview_ref } => write!(
                formatter,
                "support export references unknown preview {preview_ref}"
            ),
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
