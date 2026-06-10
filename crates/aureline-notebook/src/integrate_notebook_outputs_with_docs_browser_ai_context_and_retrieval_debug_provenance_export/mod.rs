//! Integrate notebook outputs with docs, browser, AI context, and retrieval-debug
//! provenance export.
//!
//! This module materializes the typed records that keep notebook output
//! integration honest across four consumer surfaces — documentation, browser,
//! AI context, and retrieval-debug provenance export — so that each surface
//! knows what it is receiving, whether the output is live runtime state or
//! captured output, and what downgrades or redactions apply before the
//! transfer. The records and closed vocabularies here mirror the boundary
//! schema at
//! `/schemas/notebook/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.schema.json`
//! and reuse the output-trust and runtime-boundary vocabulary already frozen
//! in `/schemas/notebook/output_trust_record.schema.json` and
//! `/crates/aureline-notebook/src/runtime_truth`.
//!
//! The module exposes:
//!
//! - the [`NotebookOutputDocIntegration`] record that carries a notebook
//!   output’s doc-surface posture, cell-aware anchor state, and freshness so
//!   documentation never silently flattens a live notebook into a static
//!   screenshot;
//! - the [`NotebookOutputBrowserIntegration`] record that carries a notebook
//!   output’s browser-surface posture, output-trust class reference, and
//!   runtime-boundary disclosure so the browser surface never conflates
//!   inspected live output with captured rendered output;
//! - the [`NotebookOutputAiContextIntegration`] record that carries a notebook
//!   output’s AI-context posture, redaction explanation, context scope, and
//!   token-budget impact so AI assistance never silently ingests stale or
//!   redacted output without labelling it;
//! - the [`NotebookOutputRetrievalDebugProvenanceExport`] record that carries
//!   a retrieval-debug export’s posture, provenance field list, export format,
//!   and debug-session reference so provenance exports remain local-first,
//!   human-readable, and explicitly scoped;
//! - the [`NotebookOutputIntegrationPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every output-integration record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookOutputDocIntegration`]
/// payloads.
pub const NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND: &str = "notebook_output_doc_integration";

/// Stable record-kind tag for serialized [`NotebookOutputBrowserIntegration`]
/// payloads.
pub const NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND: &str =
    "notebook_output_browser_integration";

/// Stable record-kind tag for serialized
/// [`NotebookOutputAiContextIntegration`] payloads.
pub const NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND: &str =
    "notebook_output_ai_context_integration";

/// Stable record-kind tag for serialized
/// [`NotebookOutputRetrievalDebugProvenanceExport`] payloads.
pub const NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND: &str =
    "notebook_output_retrieval_debug_provenance_export";

/// Stable record-kind tag for the checked-in [`NotebookOutputIntegrationPacket`].
pub const NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND: &str = "notebook_output_integration_packet";

/// Repo-relative path to the checked-in output-integration packet JSON.
pub const NOTEBOOK_OUTPUT_INTEGRATION_PACKET_PATH: &str =
    "artifacts/notebook/m5/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.json";

/// Embedded checked-in output-integration packet JSON.
pub const NOTEBOOK_OUTPUT_INTEGRATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export.json"
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
    /// Doc-posture class. Names how a notebook output is integrated into a
    /// documentation surface so docs never silently flatten a live notebook
    /// into a static screenshot.
    NotebookOutputDocPostureClass {
        Embedded => "embedded",
        Linked => "linked",
        Snapshot => "snapshot",
        Stale => "stale",
        Archived => "archived",
    }
);

closed_vocab!(
    /// Browser-posture class. Names how a notebook output is integrated into
    /// a browser surface so the browser never conflates inspected live output
    /// with captured rendered output.
    NotebookOutputBrowserPostureClass {
        Inspected => "inspected",
        Rendered => "rendered",
        Sandboxed => "sandboxed",
        Blocked => "blocked",
        Degraded => "degraded",
    }
);

closed_vocab!(
    /// AI-context-posture class. Names how a notebook output is integrated
    /// into an AI context surface so AI assistance never silently ingests
    /// stale or redacted output without labelling it.
    NotebookOutputAiContextPostureClass {
        Included => "included",
        Redacted => "redacted",
        Summarized => "summarized",
        Excluded => "excluded",
        Degraded => "degraded",
    }
);

closed_vocab!(
    /// Retrieval-debug-posture class. Names how provenance is exported for
    /// retrieval debugging so exports remain local-first, human-readable, and
    /// explicitly scoped.
    NotebookOutputRetrievalDebugPostureClass {
        FullProvenance => "full_provenance",
        SummaryOnly => "summary_only",
        Redacted => "redacted",
        Degraded => "degraded",
    }
);

closed_vocab!(
    /// Runtime-boundary-disclosure class. Names the explicit boundary between
    /// live runtime state and captured output so every consumer surface knows
    /// what it is receiving.
    NotebookOutputRuntimeBoundaryDisclosureClass {
        LiveRuntime => "live_runtime",
        CapturedOutput => "captured_output",
        Degraded => "degraded",
    }
);

closed_vocab!(
    /// Context-scope class. Names the scope of notebook output included in an
    /// AI context integration so consumers never silently broaden the scope.
    NotebookOutputContextScopeClass {
        Cell => "cell",
        Output => "output",
        Notebook => "notebook",
        Selection => "selection",
    }
);

closed_vocab!(
    /// Provenance-field class. Names the fields that may appear in a
    /// retrieval-debug provenance export so the export explicitly lists what
    /// provenance is included.
    NotebookOutputProvenanceFieldClass {
        ExecutionId => "execution_id",
        EnvironmentFingerprint => "environment_fingerprint",
        DatasetLineage => "dataset_lineage",
        CellSourceVersion => "cell_source_version",
        OutputTrustClass => "output_trust_class",
        Timestamp => "timestamp",
        KernelSessionId => "kernel_session_id",
    }
);

closed_vocab!(
    /// Provenance-format class. Names the export formats available for
    /// retrieval-debug provenance exports.
    NotebookOutputProvenanceFormatClass {
        Json => "json",
        Yaml => "yaml",
        Packet => "packet",
    }
);

/// Generic finding shape used by every output-integration validator. Mirrors
/// the finding shapes other Aureline crates expose so a single
/// review/audit/support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputIntegrationFinding {
    /// Stable check id (e.g. `notebook_output_doc_integration.document_id_ref_required`).
    pub check_id: String,
    /// Subject row id (record id, integration id, document id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl OutputIntegrationFinding {
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

/// Typed validation finding for a [`NotebookOutputDocIntegration`].
pub type NotebookOutputDocIntegrationFinding = OutputIntegrationFinding;

/// Typed validation finding for a [`NotebookOutputBrowserIntegration`].
pub type NotebookOutputBrowserIntegrationFinding = OutputIntegrationFinding;

/// Typed validation finding for a [`NotebookOutputAiContextIntegration`].
pub type NotebookOutputAiContextIntegrationFinding = OutputIntegrationFinding;

/// Typed validation finding for a [`NotebookOutputRetrievalDebugProvenanceExport`].
pub type NotebookOutputRetrievalDebugProvenanceExportFinding = OutputIntegrationFinding;

/// Typed validation finding for a [`NotebookOutputIntegrationPacket`].
pub type NotebookOutputIntegrationPacketFinding = OutputIntegrationFinding;

/// Notebook output doc-integration record. Carries a notebook output’s
/// doc-surface posture, cell-aware anchor state, and freshness so
/// documentation never silently flattens a live notebook into a static
/// screenshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputDocIntegration {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_integration_schema_version: u32,
    /// Stable opaque doc-integration id.
    pub doc_integration_id: String,
    /// Opaque ref to the notebook document.
    pub document_id_ref: String,
    /// Opaque ref to the cell whose output is integrated.
    pub cell_id_ref: String,
    /// Opaque ref to the output block being integrated.
    pub output_block_ref: String,
    /// Opaque ref to the consuming doc surface.
    pub doc_surface_ref: String,
    /// Doc-posture class.
    pub doc_posture: NotebookOutputDocPostureClass,
    /// Whether the doc surface keeps stable cell-aware anchors.
    pub cell_aware_anchor: bool,
    /// Runtime-boundary disclosure.
    pub runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputDocIntegration {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookOutputDocIntegrationFinding> {
        let mut findings = Vec::new();
        let subject = self.doc_integration_id.as_str();

        if self.record_kind != NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_DOC_INTEGRATION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_integration_schema_version != NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_output_integration_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.output_block_ref.trim().is_empty() {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.output_block_ref_required",
                subject,
                "output_block_ref must be non-empty",
            ));
        }
        if self.doc_surface_ref.trim().is_empty() {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.doc_surface_ref_required",
                subject,
                "doc_surface_ref must be non-empty",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookOutputDocIntegrationFinding::new(
                "notebook_output_doc_integration.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook output browser-integration record. Carries a notebook output’s
/// browser-surface posture, output-trust class reference, and runtime-boundary
/// disclosure so the browser surface never conflates inspected live output
/// with captured rendered output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputBrowserIntegration {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_integration_schema_version: u32,
    /// Stable opaque browser-integration id.
    pub browser_integration_id: String,
    /// Opaque ref to the notebook document.
    pub document_id_ref: String,
    /// Opaque ref to the cell whose output is integrated.
    pub cell_id_ref: String,
    /// Opaque ref to the output block being integrated.
    pub output_block_ref: String,
    /// Opaque ref to the consuming browser surface.
    pub browser_surface_ref: String,
    /// Browser-posture class.
    pub browser_posture: NotebookOutputBrowserPostureClass,
    /// Opaque ref to the output-trust class record for this output.
    pub output_trust_class_ref: String,
    /// Runtime-boundary disclosure.
    pub runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputBrowserIntegration {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookOutputBrowserIntegrationFinding> {
        let mut findings = Vec::new();
        let subject = self.browser_integration_id.as_str();

        if self.record_kind != NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_BROWSER_INTEGRATION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_integration_schema_version != NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_output_integration_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.output_block_ref.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.output_block_ref_required",
                subject,
                "output_block_ref must be non-empty",
            ));
        }
        if self.browser_surface_ref.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.browser_surface_ref_required",
                subject,
                "browser_surface_ref must be non-empty",
            ));
        }
        if self.output_trust_class_ref.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.output_trust_class_ref_required",
                subject,
                "output_trust_class_ref must be non-empty",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookOutputBrowserIntegrationFinding::new(
                "notebook_output_browser_integration.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook output AI-context-integration record. Carries a notebook output’s
/// AI-context posture, redaction explanation, context scope, and token-budget
/// impact so AI assistance never silently ingests stale or redacted output
/// without labelling it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputAiContextIntegration {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_integration_schema_version: u32,
    /// Stable opaque AI-context-integration id.
    pub ai_context_integration_id: String,
    /// Opaque ref to the notebook document.
    pub document_id_ref: String,
    /// Opaque ref to the cell whose output is integrated.
    pub cell_id_ref: String,
    /// Opaque ref to the output block being integrated.
    pub output_block_ref: String,
    /// Opaque ref to the consuming AI surface.
    pub ai_surface_ref: String,
    /// AI-context-posture class.
    pub ai_context_posture: NotebookOutputAiContextPostureClass,
    /// Export-safe explanation when ai_context_posture is
    /// [`NotebookOutputAiContextPostureClass::Redacted`] or
    /// [`NotebookOutputAiContextPostureClass::Degraded`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redaction_explanation: Option<String>,
    /// Context-scope class.
    pub context_scope: NotebookOutputContextScopeClass,
    /// Runtime-boundary disclosure.
    pub runtime_boundary_disclosure: NotebookOutputRuntimeBoundaryDisclosureClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputAiContextIntegration {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookOutputAiContextIntegrationFinding> {
        let mut findings = Vec::new();
        let subject = self.ai_context_integration_id.as_str();

        if self.record_kind != NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_AI_CONTEXT_INTEGRATION_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_integration_schema_version != NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_output_integration_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.output_block_ref.trim().is_empty() {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.output_block_ref_required",
                subject,
                "output_block_ref must be non-empty",
            ));
        }
        if self.ai_surface_ref.trim().is_empty() {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.ai_surface_ref_required",
                subject,
                "ai_surface_ref must be non-empty",
            ));
        }

        if (self.ai_context_posture == NotebookOutputAiContextPostureClass::Redacted
            || self.ai_context_posture == NotebookOutputAiContextPostureClass::Degraded)
            && self.redaction_explanation.is_none()
        {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.redaction_explanation_required",
                subject,
                "redaction_explanation must be Some when ai_context_posture is redacted or degraded",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookOutputAiContextIntegrationFinding::new(
                "notebook_output_ai_context_integration.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Notebook output retrieval-debug provenance-export record. Carries a
/// retrieval-debug export’s posture, provenance field list, export format, and
/// debug-session reference so provenance exports remain local-first,
/// human-readable, and explicitly scoped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputRetrievalDebugProvenanceExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_output_integration_schema_version: u32,
    /// Stable opaque provenance-export id.
    pub provenance_export_id: String,
    /// Opaque ref to the notebook document.
    pub document_id_ref: String,
    /// Opaque ref to the cell whose output is exported.
    pub cell_id_ref: String,
    /// Opaque ref to the output block being exported.
    pub output_block_ref: String,
    /// Opaque ref to the retrieval query that triggered this export.
    pub retrieval_query_ref: String,
    /// Retrieval-debug-posture class.
    pub export_posture: NotebookOutputRetrievalDebugPostureClass,
    /// Provenance fields included in this export.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub provenance_fields: Vec<NotebookOutputProvenanceFieldClass>,
    /// Provenance-format class.
    pub export_format: NotebookOutputProvenanceFormatClass,
    /// Opaque ref to the debug session.
    pub debug_session_ref: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputRetrievalDebugProvenanceExport {
    /// Returns typed truth-rule findings; an empty vector means the record is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookOutputRetrievalDebugProvenanceExportFinding> {
        let mut findings = Vec::new();
        let subject = self.provenance_export_id.as_str();

        if self.record_kind != NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_RETRIEVAL_DEBUG_PROVENANCE_EXPORT_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_output_integration_schema_version != NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.notebook_output_integration_schema_version
                ),
            ));
        }

        if self.document_id_ref.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.document_id_ref_required",
                subject,
                "document_id_ref must be non-empty",
            ));
        }
        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }
        if self.output_block_ref.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.output_block_ref_required",
                subject,
                "output_block_ref must be non-empty",
            ));
        }
        if self.retrieval_query_ref.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.retrieval_query_ref_required",
                subject,
                "retrieval_query_ref must be non-empty",
            ));
        }
        if self.debug_session_ref.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.debug_session_ref_required",
                subject,
                "debug_session_ref must be non-empty",
            ));
        }

        if self.export_posture == NotebookOutputRetrievalDebugPostureClass::FullProvenance
            && self.provenance_fields.is_empty()
        {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.provenance_fields_required",
                subject,
                "provenance_fields must not be empty when export_posture is full_provenance",
            ));
        }

        if self.summary.trim().is_empty() {
            findings.push(NotebookOutputRetrievalDebugProvenanceExportFinding::new(
                "notebook_output_retrieval_debug_provenance_export.summary_required",
                subject,
                "summary must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in output-integration packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputIntegrationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: doc posture classes.
    pub doc_posture_classes: Vec<NotebookOutputDocPostureClass>,
    /// Closed vocabulary: browser posture classes.
    pub browser_posture_classes: Vec<NotebookOutputBrowserPostureClass>,
    /// Closed vocabulary: AI context posture classes.
    pub ai_context_posture_classes: Vec<NotebookOutputAiContextPostureClass>,
    /// Closed vocabulary: retrieval-debug posture classes.
    pub retrieval_debug_posture_classes: Vec<NotebookOutputRetrievalDebugPostureClass>,
    /// Closed vocabulary: runtime-boundary disclosure classes.
    pub runtime_boundary_disclosure_classes: Vec<NotebookOutputRuntimeBoundaryDisclosureClass>,
    /// Closed vocabulary: context scope classes.
    pub context_scope_classes: Vec<NotebookOutputContextScopeClass>,
    /// Closed vocabulary: provenance field classes.
    pub provenance_field_classes: Vec<NotebookOutputProvenanceFieldClass>,
    /// Closed vocabulary: provenance format classes.
    pub provenance_format_classes: Vec<NotebookOutputProvenanceFormatClass>,
    /// Worked example doc integrations.
    pub example_doc_integrations: Vec<NotebookOutputDocIntegration>,
    /// Worked example browser integrations.
    pub example_browser_integrations: Vec<NotebookOutputBrowserIntegration>,
    /// Worked example AI context integrations.
    pub example_ai_context_integrations: Vec<NotebookOutputAiContextIntegration>,
    /// Worked example retrieval-debug provenance exports.
    pub example_retrieval_debug_provenance_exports: Vec<NotebookOutputRetrievalDebugProvenanceExport>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutputIntegrationPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookOutputIntegrationPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_OUTPUT_INTEGRATION_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTPUT_INTEGRATION_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.doc_posture_classes.len() != NotebookOutputDocPostureClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.doc_posture_classes_coverage",
                subject,
                "doc_posture_classes must list every variant",
            ));
        }
        if self.browser_posture_classes.len() != NotebookOutputBrowserPostureClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.browser_posture_classes_coverage",
                subject,
                "browser_posture_classes must list every variant",
            ));
        }
        if self.ai_context_posture_classes.len() != NotebookOutputAiContextPostureClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.ai_context_posture_classes_coverage",
                subject,
                "ai_context_posture_classes must list every variant",
            ));
        }
        if self.retrieval_debug_posture_classes.len() != NotebookOutputRetrievalDebugPostureClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.retrieval_debug_posture_classes_coverage",
                subject,
                "retrieval_debug_posture_classes must list every variant",
            ));
        }
        if self.runtime_boundary_disclosure_classes.len() != NotebookOutputRuntimeBoundaryDisclosureClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.runtime_boundary_disclosure_classes_coverage",
                subject,
                "runtime_boundary_disclosure_classes must list every variant",
            ));
        }
        if self.context_scope_classes.len() != NotebookOutputContextScopeClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.context_scope_classes_coverage",
                subject,
                "context_scope_classes must list every variant",
            ));
        }
        if self.provenance_field_classes.len() != NotebookOutputProvenanceFieldClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.provenance_field_classes_coverage",
                subject,
                "provenance_field_classes must list every variant",
            ));
        }
        if self.provenance_format_classes.len() != NotebookOutputProvenanceFormatClass::ALL.len() {
            findings.push(NotebookOutputIntegrationPacketFinding::new(
                "notebook_output_integration_packet.provenance_format_classes_coverage",
                subject,
                "provenance_format_classes must list every variant",
            ));
        }

        for rec in &self.example_doc_integrations {
            findings.extend(rec.validate().into_iter().map(|f| {
                NotebookOutputIntegrationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for rec in &self.example_browser_integrations {
            findings.extend(rec.validate().into_iter().map(|f| {
                NotebookOutputIntegrationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for rec in &self.example_ai_context_integrations {
            findings.extend(rec.validate().into_iter().map(|f| {
                NotebookOutputIntegrationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for rec in &self.example_retrieval_debug_provenance_exports {
            findings.extend(rec.validate().into_iter().map(|f| {
                NotebookOutputIntegrationPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Parses the checked-in output-integration packet JSON.
pub fn current_notebook_output_integration_packet(
) -> Result<NotebookOutputIntegrationPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_OUTPUT_INTEGRATION_PACKET_JSON)
}

impl NotebookOutputDocPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Embedded,
        Self::Linked,
        Self::Snapshot,
        Self::Stale,
        Self::Archived,
    ];
}

impl NotebookOutputBrowserPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Inspected,
        Self::Rendered,
        Self::Sandboxed,
        Self::Blocked,
        Self::Degraded,
    ];
}

impl NotebookOutputAiContextPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Included,
        Self::Redacted,
        Self::Summarized,
        Self::Excluded,
        Self::Degraded,
    ];
}

impl NotebookOutputRetrievalDebugPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullProvenance,
        Self::SummaryOnly,
        Self::Redacted,
        Self::Degraded,
    ];
}

impl NotebookOutputRuntimeBoundaryDisclosureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::LiveRuntime, Self::CapturedOutput, Self::Degraded];
}

impl NotebookOutputContextScopeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Cell, Self::Output, Self::Notebook, Self::Selection];
}

impl NotebookOutputProvenanceFieldClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExecutionId,
        Self::EnvironmentFingerprint,
        Self::DatasetLineage,
        Self::CellSourceVersion,
        Self::OutputTrustClass,
        Self::Timestamp,
        Self::KernelSessionId,
    ];
}

impl NotebookOutputProvenanceFormatClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 3] = [Self::Json, Self::Yaml, Self::Packet];
}

#[cfg(test)]
mod tests;
