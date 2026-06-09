//! Canonical `.ipynb` document model, stable cell IDs, attachments, and no-kernel editability.
//!
//! This module materializes the typed document model that keeps the notebook
//! surface honest about canonical source truth, cell identity, attachment
//! survival, and editing without a kernel. The records and closed vocabularies
//! here mirror the boundary schema at
//! `/schemas/notebook/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.schema.json`
//! and reuse the preservation, stability, and survival axes already frozen in
//! `/schemas/notebook/notebook_metadata_aureline.schema.json`.
//!
//! The module exposes:
//!
//! - the [`NotebookDocument`] record that binds notebook identity, nbformat
//!   version, canonical-preservation class, cell-id stability class, metadata
//!   survival class, and the ordered cell list;
//! - the [`NotebookCell`] record that carries a stable cell id, cell type,
//!   source ref, metadata survival class, attachment refs, and execution refs;
//! - the [`NotebookAttachment`] record that carries owner cell ref, MIME class,
//!   digest, size, and preview class so attachments are never silently
//!   externalized;
//! - the [`NotebookLocalStateOverlay`] record that carries selection, fold
//!   state, scroll anchors, and pinned viewers so UI convenience state never
//!   rewrites canonical notebook structure;
//! - the [`NotebookDocumentModelPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw attachment bytes, and
//! raw URLs MUST NOT appear on any record carried here. Only opaque handles
//! and closed-vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every document-model record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookDocument`] payloads.
pub const NOTEBOOK_DOCUMENT_RECORD_KIND: &str = "notebook_document";

/// Stable record-kind tag for serialized [`NotebookCell`] payloads.
pub const NOTEBOOK_CELL_RECORD_KIND: &str = "notebook_cell";

/// Stable record-kind tag for serialized [`NotebookAttachment`] payloads.
pub const NOTEBOOK_ATTACHMENT_RECORD_KIND: &str = "notebook_attachment";

/// Stable record-kind tag for serialized [`NotebookLocalStateOverlay`] payloads.
pub const NOTEBOOK_LOCAL_STATE_OVERLAY_RECORD_KIND: &str = "notebook_local_state_overlay";

/// Stable record-kind tag for the checked-in [`NotebookDocumentModelPacket`].
pub const NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND: &str = "notebook_document_model_packet";

/// Repo-relative path to the checked-in document-model packet JSON.
pub const NOTEBOOK_DOCUMENT_MODEL_PACKET_PATH: &str =
    "artifacts/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.json";

/// Embedded checked-in document-model packet JSON.
pub const NOTEBOOK_DOCUMENT_MODEL_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Cell-type class. Mirrors the official Jupyter nbformat vocabulary so
    /// the document model never invents ad hoc cell kinds.
    NotebookCellType {
        Code => "code",
        Markdown => "markdown",
        Raw => "raw",
    }
);

closed_vocab!(
    /// Canonical-preservation class. Pins whether the on-disk `.ipynb` is
    /// canonical, an export-only form with no round trip, or a format with no
    /// paired text representation.
    NotebookCanonicalPreservationClass {
        CanonicalIpynbPreserved => "canonical_ipynb_preserved",
        ExportOnlyNoRoundTrip => "export_only_no_round_trip",
        NoPairedTextRepresentation => "no_paired_text_representation",
    }
);

closed_vocab!(
    /// Cell-id stability class. Names whether stable cell IDs are required,
    /// minted on first save, or unavailable (forcing raw-JSON fallback).
    NotebookCellIdStabilityClass {
        StableCellIdRequired => "stable_cell_id_required",
        StableCellIdMintedOnFirstSave => "stable_cell_id_minted_on_first_save",
        CellIdStabilityNotAvailableRawJsonFallbackOnly => "cell_id_stability_not_available_raw_json_fallback_only",
    }
);

closed_vocab!(
    /// Metadata-survival class. Names which metadata namespaces must survive
    /// open/save/diff/merge. Mirrors the survival axis frozen in the metadata
    /// schema.
    NotebookMetadataSurvivalClass {
        SurvivalRequiredJupyterAndAurelineNamespaces => "survival_required_jupyter_and_aureline_namespaces",
        SurvivalRequiredVendorNamespaces => "survival_required_vendor_namespaces",
        SurvivalRecommendedUnknownVendorNamespaces => "survival_recommended_unknown_vendor_namespaces",
    }
);

closed_vocab!(
    /// No-kernel editability class. Names the editability posture when no
    /// compatible kernel is available. The notebook must always remain editable,
    /// searchable, and reviewable; this class says how degraded the preview is.
    NotebookNoKernelEditabilityClass {
        EditableSearchableReviewable => "editable_searchable_reviewable",
        EditableReadonlyKernelRequiredForExecution => "editable_readonly_kernel_required_for_execution",
        EditableWithDegradedPreview => "editable_with_degraded_preview",
    }
);

closed_vocab!(
    /// Attachment-preview class. Names how an attachment is previewed in the
    /// chrome row without exposing raw bytes.
    NotebookAttachmentPreviewClass {
        InlinePreview => "inline_preview",
        ThumbnailPreview => "thumbnail_preview",
        IconPreview => "icon_preview",
        NoPreview => "no_preview",
    }
);

/// Generic finding shape used by every document-model validator. Mirrors the
/// finding shapes other Aureline crates expose so a single review/audit/
/// support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentModelFinding {
    /// Stable check id (e.g. `notebook_document.nbformat_major_positive`).
    pub check_id: String,
    /// Subject row id (record id, document id, cell id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl DocumentModelFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookDocument`].
pub type NotebookDocumentFinding = DocumentModelFinding;

/// Typed validation finding for a [`NotebookCell`].
pub type NotebookCellFinding = DocumentModelFinding;

/// Typed validation finding for a [`NotebookAttachment`].
pub type NotebookAttachmentFinding = DocumentModelFinding;

/// Typed validation finding for a [`NotebookLocalStateOverlay`].
pub type NotebookLocalStateOverlayFinding = DocumentModelFinding;

/// Typed validation finding for a [`NotebookDocumentModelPacket`].
pub type NotebookDocumentModelPacketFinding = DocumentModelFinding;

/// Canonical notebook attachment record. Attachments preserve unknown
/// namespaces and are not silently externalized.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookAttachment {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_document_model_schema_version: u32,
    /// Stable opaque attachment id.
    pub attachment_id: String,
    /// Opaque ref to the owning cell.
    pub owner_cell_ref: String,
    /// MIME class token (e.g. `image/png`, `application/json`). This is a
    /// closed-vocabulary token, not raw bytes.
    pub mime_class: String,
    /// Content digest (e.g. `sha256:...`).
    pub digest: String,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Preview class for the chrome row.
    pub preview_class: NotebookAttachmentPreviewClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookAttachment {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookAttachmentFinding> {
        let mut findings = Vec::new();
        let subject = self.attachment_id.as_str();

        if self.record_kind != NOTEBOOK_ATTACHMENT_RECORD_KIND {
            findings.push(NotebookAttachmentFinding::new(
                "notebook_attachment.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_ATTACHMENT_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_document_model_schema_version != NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION {
            findings.push(NotebookAttachmentFinding::new(
                "notebook_attachment.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION}, found {}",
                    self.notebook_document_model_schema_version
                ),
            ));
        }

        if self.mime_class.trim().is_empty() {
            findings.push(NotebookAttachmentFinding::new(
                "notebook_attachment.mime_class_required",
                subject,
                "mime_class must be non-empty",
            ));
        }
        if self.digest.trim().is_empty() {
            findings.push(NotebookAttachmentFinding::new(
                "notebook_attachment.digest_required",
                subject,
                "digest must be non-empty",
            ));
        }

        findings
    }
}

/// Canonical notebook cell record. Stable cell IDs survive reorder, diff,
/// review, and comment anchoring where the format allows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCell {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_document_model_schema_version: u32,
    /// Stable opaque cell id. Identity survives save, diff, merge, re-order,
    /// and re-materialization.
    pub cell_id: String,
    /// Opaque notebook-document id this cell belongs to.
    pub document_id_ref: String,
    /// Cell-type class.
    pub cell_type: NotebookCellType,
    /// Opaque ref to the cell source body. Raw source bytes MUST NOT appear
    /// inline on this record.
    pub cell_source_ref: String,
    /// Metadata survival class for this cell.
    pub cell_metadata_survival_class: NotebookMetadataSurvivalClass,
    /// Opaque refs to [`NotebookAttachment`] records bound to this cell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachment_refs: Vec<String>,
    /// Opaque list of vendor-namespace ids detected on the cell metadata; raw
    /// vendor bodies MUST NOT appear.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unknown_vendor_namespaces_present: Vec<String>,
    /// Opaque ref to the last cell-execution record for this cell.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_cell_execution_id_ref: Option<String>,
    /// Opaque refs to output-lineage records bound to this cell.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_lineage_refs: Vec<String>,
    /// Whether the cell is collapsed in the UI. This is ephemeral chrome state
    /// carried on the cell record for convenience, not canonical structure.
    #[serde(default)]
    pub collapsed: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCell {
    /// Returns typed truth-rule findings; an empty vector means the cell is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCellFinding> {
        let mut findings = Vec::new();
        let subject = self.cell_id.as_str();

        if self.record_kind != NOTEBOOK_CELL_RECORD_KIND {
            findings.push(NotebookCellFinding::new(
                "notebook_cell.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_CELL_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_document_model_schema_version != NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION {
            findings.push(NotebookCellFinding::new(
                "notebook_cell.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION}, found {}",
                    self.notebook_document_model_schema_version
                ),
            ));
        }

        if self.cell_source_ref.trim().is_empty() {
            findings.push(NotebookCellFinding::new(
                "notebook_cell.cell_source_ref_required",
                subject,
                "cell_source_ref must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook local-state overlay. UI convenience state that MUST NOT rewrite
/// canonical notebook structure without an explicit save/apply path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookLocalStateOverlay {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_document_model_schema_version: u32,
    /// Stable opaque overlay id.
    pub overlay_id: String,
    /// Opaque notebook-document id this overlay belongs to.
    pub document_id_ref: String,
    /// Opaque ref to the currently selected cell id, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_cell_id_ref: Option<String>,
    /// Opaque refs to cells whose output is collapsed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_collapsed_cell_id_refs: Vec<String>,
    /// Opaque refs to cells whose source is folded.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_folded_cell_id_refs: Vec<String>,
    /// Opaque scroll-anchor cell id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_anchor_cell_id_ref: Option<String>,
    /// Opaque refs to pinned viewer cell ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pinned_viewer_cell_id_refs: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookLocalStateOverlay {
    /// Returns typed truth-rule findings; an empty vector means the overlay is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookLocalStateOverlayFinding> {
        let mut findings = Vec::new();
        let subject = self.overlay_id.as_str();

        if self.record_kind != NOTEBOOK_LOCAL_STATE_OVERLAY_RECORD_KIND {
            findings.push(NotebookLocalStateOverlayFinding::new(
                "notebook_local_state_overlay.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_LOCAL_STATE_OVERLAY_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_document_model_schema_version != NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION {
            findings.push(NotebookLocalStateOverlayFinding::new(
                "notebook_local_state_overlay.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION}, found {}",
                    self.notebook_document_model_schema_version
                ),
            ));
        }

        findings
    }
}

/// Canonical notebook document record. The `.ipynb` stays canonical; this
/// record carries the typed identity, format version, preservation posture,
/// cell list, and local-state overlay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocument {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_document_model_schema_version: u32,
    /// Stable opaque notebook-document id.
    pub document_id: String,
    /// Opaque VFS path-identity token for the document.
    pub document_path_token_ref: String,
    /// Notebook URI for display and export.
    pub document_uri: String,
    /// nbformat major version read from the document.
    pub nbformat_major: u32,
    /// nbformat minor version read from the document.
    pub nbformat_minor: u32,
    /// Canonical-preservation class.
    pub canonical_preservation_class: NotebookCanonicalPreservationClass,
    /// Cell-id stability class.
    pub cell_id_stability_class: NotebookCellIdStabilityClass,
    /// Metadata-survival class.
    pub metadata_survival_class: NotebookMetadataSurvivalClass,
    /// No-kernel editability class.
    pub no_kernel_editability_class: NotebookNoKernelEditabilityClass,
    /// Opaque ref to the document-trust state record.
    pub document_trust_state_ref: String,
    /// Opaque ref to the workspace-trust state record.
    pub workspace_trust_state_ref: String,
    /// Paired-text export posture.
    pub paired_text_export_posture_class: crate::NotebookPairedExportPosture,
    /// Opaque paired-export ref; non-null only when the posture is a derived
    /// class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paired_text_export_ref: Option<String>,
    /// Ordered cell records.
    pub cells: Vec<NotebookCell>,
    /// Local-state overlay.
    pub local_state_overlay: NotebookLocalStateOverlay,
    /// Cell-order digest for change detection.
    pub cell_order_digest: String,
    /// Metadata namespace inventory.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata_namespace_inventory: Vec<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDocument {
    /// Returns typed truth-rule findings; an empty vector means the document is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDocumentFinding> {
        let mut findings = Vec::new();
        let subject = self.document_id.as_str();

        if self.record_kind != NOTEBOOK_DOCUMENT_RECORD_KIND {
            findings.push(NotebookDocumentFinding::new(
                "notebook_document.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DOCUMENT_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_document_model_schema_version != NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION {
            findings.push(NotebookDocumentFinding::new(
                "notebook_document.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION}, found {}",
                    self.notebook_document_model_schema_version
                ),
            ));
        }

        if self.nbformat_major == 0 && self.nbformat_minor == 0 {
            findings.push(NotebookDocumentFinding::new(
                "notebook_document.nbformat_major_positive",
                subject,
                "nbformat_major and nbformat_minor cannot both be zero",
            ));
        }

        let mut seen_cell_ids = std::collections::BTreeSet::new();
        for cell in &self.cells {
            if !seen_cell_ids.insert(cell.cell_id.clone()) {
                findings.push(NotebookDocumentFinding::new(
                    "notebook_document.duplicate_cell_id",
                    subject,
                    format!("duplicate cell_id '{}' in document cell list", cell.cell_id),
                ));
            }
            if cell.document_id_ref != self.document_id {
                findings.push(NotebookDocumentFinding::new(
                    "notebook_document.cell_document_mismatch",
                    subject,
                    format!(
                        "cell {} claims document_id_ref '{}', expected '{}'",
                        cell.cell_id, cell.document_id_ref, self.document_id
                    ),
                ));
            }
        }

        if self.cells.is_empty() {
            findings.push(NotebookDocumentFinding::new(
                "notebook_document.empty_cells",
                subject,
                "notebook document must contain at least one cell",
            ));
        }

        match self.canonical_preservation_class {
            NotebookCanonicalPreservationClass::CanonicalIpynbPreserved => {}
            NotebookCanonicalPreservationClass::ExportOnlyNoRoundTrip => {
                findings.push(NotebookDocumentFinding::new(
                    "notebook_document.export_only_warning",
                    subject,
                    "export_only_no_round_trip: document cannot be saved back to canonical .ipynb",
                ));
            }
            NotebookCanonicalPreservationClass::NoPairedTextRepresentation => {
                findings.push(NotebookDocumentFinding::new(
                    "notebook_document.no_paired_text_warning",
                    subject,
                    "no_paired_text_representation: paired text export is not available for this document",
                ));
            }
        }

        match self.paired_text_export_posture_class {
            crate::NotebookPairedExportPosture::NotApplicable => {
                if self.paired_text_export_ref.is_some() {
                    findings.push(NotebookDocumentFinding::new(
                        "notebook_document.paired_export_ref_not_applicable",
                        subject,
                        "paired_text_export_not_applicable must not carry a paired_text_export_ref",
                    ));
                }
            }
            crate::NotebookPairedExportPosture::DerivedNotebookToScript
            | crate::NotebookPairedExportPosture::DerivedNotebookToMarkdown => {
                if self.paired_text_export_ref.is_none() {
                    findings.push(NotebookDocumentFinding::new(
                        "notebook_document.paired_export_ref_required",
                        subject,
                        "derived paired-export postures must carry a paired_text_export_ref",
                    ));
                }
            }
        }

        if self.local_state_overlay.document_id_ref != self.document_id {
            findings.push(NotebookDocumentFinding::new(
                "notebook_document.overlay_document_mismatch",
                subject,
                format!(
                    "local_state_overlay claims document_id_ref '{}', expected '{}'",
                    self.local_state_overlay.document_id_ref, self.document_id
                ),
            ));
        }

        findings
    }
}

/// Checked-in document-model packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDocumentModelPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: cell types.
    pub cell_types: Vec<NotebookCellType>,
    /// Closed vocabulary: canonical preservation classes.
    pub canonical_preservation_classes: Vec<NotebookCanonicalPreservationClass>,
    /// Closed vocabulary: cell-id stability classes.
    pub cell_id_stability_classes: Vec<NotebookCellIdStabilityClass>,
    /// Closed vocabulary: metadata survival classes.
    pub metadata_survival_classes: Vec<NotebookMetadataSurvivalClass>,
    /// Closed vocabulary: no-kernel editability classes.
    pub no_kernel_editability_classes: Vec<NotebookNoKernelEditabilityClass>,
    /// Closed vocabulary: attachment preview classes.
    pub attachment_preview_classes: Vec<NotebookAttachmentPreviewClass>,
    /// Worked example documents.
    pub example_documents: Vec<NotebookDocument>,
    /// Worked example cells.
    pub example_cells: Vec<NotebookCell>,
    /// Worked example attachments.
    pub example_attachments: Vec<NotebookAttachment>,
    /// Worked example overlays.
    pub example_overlays: Vec<NotebookLocalStateOverlay>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDocumentModelPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookDocumentModelPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND, self.record_kind
                ),
            ));
        }

        if self.cell_types.len() != NotebookCellType::ALL.len() {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.cell_types_coverage",
                subject,
                "cell_types must list every variant",
            ));
        }
        if self.canonical_preservation_classes.len()
            != NotebookCanonicalPreservationClass::ALL.len()
        {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.canonical_preservation_classes_coverage",
                subject,
                "canonical_preservation_classes must list every variant",
            ));
        }
        if self.cell_id_stability_classes.len() != NotebookCellIdStabilityClass::ALL.len() {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.cell_id_stability_classes_coverage",
                subject,
                "cell_id_stability_classes must list every variant",
            ));
        }
        if self.metadata_survival_classes.len() != NotebookMetadataSurvivalClass::ALL.len() {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.metadata_survival_classes_coverage",
                subject,
                "metadata_survival_classes must list every variant",
            ));
        }
        if self.no_kernel_editability_classes.len() != NotebookNoKernelEditabilityClass::ALL.len() {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.no_kernel_editability_classes_coverage",
                subject,
                "no_kernel_editability_classes must list every variant",
            ));
        }
        if self.attachment_preview_classes.len() != NotebookAttachmentPreviewClass::ALL.len() {
            findings.push(NotebookDocumentModelPacketFinding::new(
                "notebook_document_model_packet.attachment_preview_classes_coverage",
                subject,
                "attachment_preview_classes must list every variant",
            ));
        }

        for doc in &self.example_documents {
            findings.extend(doc.validate().into_iter().map(|f| {
                NotebookDocumentModelPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for cell in &self.example_cells {
            findings.extend(cell.validate().into_iter().map(|f| {
                NotebookDocumentModelPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for attachment in &self.example_attachments {
            findings.extend(attachment.validate().into_iter().map(|f| {
                NotebookDocumentModelPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for overlay in &self.example_overlays {
            findings.extend(overlay.validate().into_iter().map(|f| {
                NotebookDocumentModelPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in document-model packet JSON.
pub fn current_notebook_document_model_packet(
) -> Result<NotebookDocumentModelPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_DOCUMENT_MODEL_PACKET_JSON)
}

impl NotebookCellType {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Code, Self::Markdown, Self::Raw];
}

impl NotebookCanonicalPreservationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::CanonicalIpynbPreserved,
        Self::ExportOnlyNoRoundTrip,
        Self::NoPairedTextRepresentation,
    ];
}

impl NotebookCellIdStabilityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::StableCellIdRequired,
        Self::StableCellIdMintedOnFirstSave,
        Self::CellIdStabilityNotAvailableRawJsonFallbackOnly,
    ];
}

impl NotebookMetadataSurvivalClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::SurvivalRequiredJupyterAndAurelineNamespaces,
        Self::SurvivalRequiredVendorNamespaces,
        Self::SurvivalRecommendedUnknownVendorNamespaces,
    ];
}

impl NotebookNoKernelEditabilityClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::EditableSearchableReviewable,
        Self::EditableReadonlyKernelRequiredForExecution,
        Self::EditableWithDegradedPreview,
    ];
}

impl NotebookAttachmentPreviewClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::InlinePreview,
        Self::ThumbnailPreview,
        Self::IconPreview,
        Self::NoPreview,
    ];
}

#[cfg(test)]
mod tests;
