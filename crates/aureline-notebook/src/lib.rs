//! Retained notebook preview runtime-truth model and canonical document model.
//!
//! This crate carries three typed models:
//!
//! 1. The [`runtime_truth`] module keeps a notebook preview row honest about
//!    notebook identity, kernel/session state, output trust, variable freshness,
//!    restart/reconnect consequences, and debugger-bridge support — so the
//!    chrome row never implies JupyterLab-class maturity through silence.
//!
//! 2. The [`materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability`]
//!    module materializes the canonical `.ipynb` document model, stable cell IDs,
//!    attachments, and no-kernel editability so that `.ipynb` stays canonical,
//!    cell IDs stay durable, and notebook open/search/review flows remain useful
//!    without a selected kernel.
//!
//! 3. The [`implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state`]
//!    module materializes the composed notebook header, kernel bar,
//!    execution-locus chips, and paired-export state that the notebook chrome
//!    consumes. It reuses the closed vocabularies from [`runtime_truth`] and
//!    adds the [`ExecutionLocusChip`] record so execution locus is visible
//!    wherever the user can run, restart, debug, or export.
//!
//! The records and closed vocabularies under [`runtime_truth`] mirror the
//! boundary schemas at `/schemas/notebook/kernel_session_summary.schema.json`
//! and `/schemas/notebook/output_trust_record.schema.json`. Worked fixtures
//! live under `/fixtures/notebook/m3/kernel_output_and_reconnect/`.
//!
//! The records under
//! [`materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability`]
//! mirror the boundary schema at
//! `/schemas/notebook/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/`.
//!
//! The records under
//! [`implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state`]
//! mirror the boundary schema at
//! `/schemas/notebook/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state/`.
//!
//! The records under
//! [`ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows`]
//! mirror the boundary schema at
//! `/schemas/notebook/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows/`.
//!
//! The records project the notebook document / kernel-session / output /
//! widget trust axes already frozen in
//! `/schemas/notebook/notebook_metadata_aureline.schema.json` and the
//! kernels-and-trust matrix at
//! `/artifacts/notebook/kernels_and_trust_matrix.yaml`. This crate does not
//! redefine those vocabularies; it adds the runtime-bearing surface records
//! the preview row needs to render — kernel-bar header, cell-execution row,
//! variable-explorer entry, rich-output trust class, and debugger-bridge state
//! — and stable validators a UI/audit/export pipeline can call against them.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

#![doc(html_root_url = "https://docs.rs/aureline-notebook/0.0.0")]

pub mod implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state;
pub mod materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability;
pub mod runtime_truth;
pub mod ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows;

pub use runtime_truth::{
    CellExecutionDetailRow, CellExecutionFinding, CellExecutionOutcomeClass, CellExecutionRunScope,
    DebuggerBridgeAdapterClass, DebuggerBridgeBreakpointPostureClass, DebuggerBridgeFinding,
    DebuggerBridgeFrameRelationClass, DebuggerBridgeKernelClass, DebuggerBridgeState,
    DebuggerBridgeSupportClass, DebuggerBridgeUnsupportedReasonClass, KernelBarActionClass,
    KernelOriginClass, KernelSelectionState, KernelSessionSummary, KernelSessionSummaryFinding,
    NotebookDirtyStateClass, NotebookDocumentTrustClass, NotebookHeaderBlock,
    NotebookLastSuccessfulRunSummary, NotebookPairedExportPosture, OutputTrustClass,
    OutputTrustFallbackActionClass, OutputTrustHiddenEscalationPosture, OutputTrustRecord,
    OutputTrustRecordFinding, OutputTrustStaleReasonClass, ReconnectReviewConsequenceClass,
    ReconnectReviewKind, ReconnectReviewSheet, ReconnectReviewSheetFinding, RuntimeTruthFinding,
    VariableExplorerEntry, VariableExplorerEntryActionClass, VariableExplorerEntryFinding,
    VariableExplorerFreshnessClass, VariableExplorerTruncationClass,
    CELL_EXECUTION_DETAIL_ROW_RECORD_KIND, DEBUGGER_BRIDGE_STATE_RECORD_KIND,
    KERNEL_SESSION_SUMMARY_RECORD_KIND, NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION,
    OUTPUT_TRUST_RECORD_KIND, RECONNECT_REVIEW_SHEET_RECORD_KIND,
    VARIABLE_EXPLORER_ENTRY_RECORD_KIND,
};

pub use materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability::{
    current_notebook_document_model_packet, DocumentModelFinding, NotebookAttachment,
    NotebookAttachmentFinding, NotebookAttachmentPreviewClass, NotebookCanonicalPreservationClass,
    NotebookCell, NotebookCellFinding, NotebookCellIdStabilityClass, NotebookCellType,
    NotebookDocument, NotebookDocumentFinding, NotebookDocumentModelPacket,
    NotebookDocumentModelPacketFinding, NotebookLocalStateOverlay, NotebookLocalStateOverlayFinding,
    NotebookMetadataSurvivalClass, NotebookNoKernelEditabilityClass,
    NOTEBOOK_ATTACHMENT_RECORD_KIND, NOTEBOOK_CELL_RECORD_KIND, NOTEBOOK_DOCUMENT_MODEL_PACKET_JSON,
    NOTEBOOK_DOCUMENT_MODEL_PACKET_PATH, NOTEBOOK_DOCUMENT_MODEL_PACKET_RECORD_KIND,
    NOTEBOOK_DOCUMENT_MODEL_SCHEMA_VERSION, NOTEBOOK_DOCUMENT_RECORD_KIND,
    NOTEBOOK_LOCAL_STATE_OVERLAY_RECORD_KIND,
};

pub use implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state::{
    current_notebook_header_kernel_bar_packet, ExecutionLocusChip, ExecutionLocusChipClass,
    ExecutionLocusChipFinding, ExecutionLocusChipState, HeaderKernelBarFinding,
    NotebookHeaderKernelBarPacket, NotebookHeaderKernelBarPacketFinding,
    NotebookHeaderKernelBarState, NotebookHeaderKernelBarStateFinding,
    EXECUTION_LOCUS_CHIP_RECORD_KIND, NOTEBOOK_HEADER_KERNEL_BAR_PACKET_JSON,
    NOTEBOOK_HEADER_KERNEL_BAR_PACKET_PATH, NOTEBOOK_HEADER_KERNEL_BAR_PACKET_RECORD_KIND,
    NOTEBOOK_HEADER_KERNEL_BAR_SCHEMA_VERSION, NOTEBOOK_HEADER_KERNEL_BAR_STATE_RECORD_KIND,
};

pub use ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows::{
    current_notebook_cell_chrome_packet, CellChromeActionClass, CellChromeFinding,
    CellChromeStatusClass, DurableExecutionStateRow, DurableExecutionStateRowFinding,
    NotebookCellChrome, NotebookCellChromeFinding, NotebookCellChromePacket,
    NotebookCellChromePacketFinding, RunScopeControl, RunScopeControlFinding,
    RunScopeControlLockReasonClass, NOTEBOOK_CELL_CHROME_PACKET_JSON,
    NOTEBOOK_CELL_CHROME_PACKET_PATH, NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND,
    NOTEBOOK_CELL_CHROME_RECORD_KIND, NOTEBOOK_CELL_CHROME_SCHEMA_VERSION,
    DURABLE_EXECUTION_STATE_ROW_RECORD_KIND, RUN_SCOPE_CONTROL_RECORD_KIND,
};
