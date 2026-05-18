//! Retained notebook preview runtime-truth model.
//!
//! This crate carries the typed truth model that keeps a notebook preview row
//! honest about notebook identity, kernel/session state, output trust, variable
//! freshness, restart/reconnect consequences, and debugger-bridge support — so
//! the chrome row never implies JupyterLab-class maturity through silence.
//!
//! The records and closed vocabularies live under [`runtime_truth`] and mirror
//! the boundary schemas at
//! `/schemas/notebook/kernel_session_summary.schema.json` and
//! `/schemas/notebook/output_trust_record.schema.json`. Worked fixtures live
//! under `/fixtures/notebook/m3/kernel_output_and_reconnect/`.
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

pub mod runtime_truth;

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
