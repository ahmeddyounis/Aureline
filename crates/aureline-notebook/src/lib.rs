//! Retained notebook preview runtime-truth model and canonical document model.
//!
//! This crate carries four typed models:
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
//! 4. The [`materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization`]
//!    module materializes the viewer-lane and virtualization layer that sits
//!    between the output trust record and the rendered notebook surface. It
//!    produces [`NotebookOutputViewerLane`] records and
//!    [`LargeOutputVirtualizationRecord`] records so the chrome never silently
//!    escalates trust, never freezes on heavy outputs, and always shows the user
//!    why an output is rendered inline, virtualized, opened in detail, or blocked.
//!
//! 5. The [`implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces`]
//!    module materializes save-operation, repair-action, and round-trip
//!    assertion records so that metadata, attachments, and unknown namespaces
//!    never disappear silently on open/save/import/export cycles.
//!
//! 6. The [`implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors`]
//!    module materializes kernel discovery, typed [`Kernelspec`] records,
//!    [`InterpreterResolution`] records, [`EnvironmentFingerprint`] records,
//!    and [`KernelDiscoveryEntry`] candidates so the kernel-selection layer
//!    never presents opaque or unvalidated kernel identities.
//!
//! 7. The [`implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export`]
//!    module materializes the composed variable-explorer surface that the
//!    notebook chrome consumes to render the variable panel, freshness labels,
//!    truncation notices, and typed-export actions. It produces
//!    [`NotebookVariableExplorer`] records and [`VariableExplorerTypedExport`]
//!    records so the explorer never implies durable project facts and never
//!    silently broadens capture on export.
//!
//! 8. The [`add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation`]
//!    module materializes the typed records that keep notebook search, outline,
//!    breadcrumb, and deep-link navigation honest about cell identity, scope,
//!    and degraded state. It produces [`NotebookSearchQuery`] records,
//!    [`NotebookOutlineItem`] records, [`NotebookBreadcrumb`] records, and
//!    [`NotebookCellTarget`] records so search, outline, breadcrumbs, and
//!    navigation remain useful without a selected kernel and never silently
//!    fall back to a different cell.
//!
//! 9. The [`ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories`]
//!    module materializes the typed bridge records that connect notebook
//!    execution to the canonical task-event stream, the activity-center
//!    chronology, and session-restore history. It produces [`NotebookTaskEvent`]
//!    records, [`NotebookActivityCenterRow`] records, and
//!    [`NotebookRestoreSafeHistory`] records so notebook work is observable,
//!    reviewable, and recoverable on the same contracts as build, test, and
//!    debug work.
//!
//! 10. The [`seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet`]
//!     module seeds the bounded fixture set and heavy-output corpus that keep
//!     M5 notebook round-trip and output-truth claims evidence-based. It produces
//!     [`NotebookRoundTripFixture`] records, [`HeavyOutputCorpusEntry`] records,
//!     and the [`NotebookSupportPacket`] checked-in artifact that downstream
//!     docs, help, CI, and support surfaces ingest instead of cloning status text.
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
//! [`materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization`]
//! mirror the boundary schema at
//! `/schemas/notebook/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/`.
//!
//! The records under
//! [`ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows`]
//! mirror the boundary schema at
//! `/schemas/notebook/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows/`.
//!
//! The records under
//! [`implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces`]
//! mirror the boundary schema at
//! `/schemas/notebook/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces/`.
//!
//! The records under
//! [`implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors`]
//! mirror the boundary schema at
//! `/schemas/notebook/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors/`.
//!
//! The records under
//! [`implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export`]
//! mirror the boundary schema at
//! `/schemas/notebook/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export/`.
//!
//! The records under
//! [`add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation`]
//! mirror the boundary schema at
//! `/schemas/notebook/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation/`.
//!
//! The records under
//! [`ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories`]
//! mirror the boundary schema at
//! `/schemas/notebook/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories/`.
//!
//! The records under
//! [`seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet`]
//! mirror the boundary schema at
//! `/schemas/notebook/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.schema.json`.
//! Worked fixtures live under
//! `/fixtures/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet/`.
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

pub mod add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation;
pub mod implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors;
pub mod implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues;
pub mod implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces;
pub mod implement_the_notebook_header_kernel_bar_execution_locus_chips_and_paired_export_state;
pub mod implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export;
pub mod materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization;
pub mod materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability;
pub mod runtime_truth;
pub mod seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet;
pub mod ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories;
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

pub use implement_notebook_save_repair_and_round_trip_safety_for_metadata_attachments_and_unknown_namespaces::{
    current_notebook_save_repair_round_trip_packet, NotebookAttachmentPreservationClass,
    NotebookMetadataPreservationClass, NotebookRepairAction, NotebookRepairActionFinding,
    NotebookRepairConsequenceClass, NotebookRepairKindClass, NotebookRoundTripAssertion,
    NotebookRoundTripAssertionFinding, NotebookRoundTripAssertionKindClass,
    NotebookRoundTripResultClass, NotebookSaveKindClass, NotebookSaveOperation,
    NotebookSaveOperationFinding, NotebookSaveRepairFinding, NotebookSaveRepairRoundTripPacket,
    NotebookSaveRepairRoundTripPacketFinding, NotebookUnknownNamespacePreservationClass,
    NOTEBOOK_REPAIR_ACTION_RECORD_KIND, NOTEBOOK_ROUND_TRIP_ASSERTION_RECORD_KIND,
    NOTEBOOK_SAVE_OPERATION_RECORD_KIND, NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_JSON,
    NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_PATH, NOTEBOOK_SAVE_REPAIR_ROUND_TRIP_PACKET_RECORD_KIND,
    NOTEBOOK_SAVE_REPAIR_SCHEMA_VERSION,
};

pub use implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors::{
    current_kernel_discovery_packet, EnvironmentFingerprint, EnvironmentFingerprintFinding,
    EnvironmentFingerprintFreshnessClass, InterpreterManagerClass, InterpreterResolution,
    InterpreterResolutionFinding, KernelDiscoveryAvailabilityClass,
    KernelDiscoveryCompatibilityClass, KernelDiscoveryEntry, KernelDiscoveryEntryFinding,
    KernelDiscoveryFinding, KernelDiscoveryPacket, KernelDiscoveryPacketFinding,
    Kernelspec, KernelspecDiscoverySourceClass, KernelspecFinding,
    ENVIRONMENT_FINGERPRINT_RECORD_KIND, INTERPRETER_RESOLUTION_RECORD_KIND,
    KERNEL_DISCOVERY_ENTRY_RECORD_KIND, KERNEL_DISCOVERY_PACKET_JSON,
    KERNEL_DISCOVERY_PACKET_PATH, KERNEL_DISCOVERY_PACKET_RECORD_KIND,
    KERNELSPEC_RECORD_KIND, NOTEBOOK_KERNEL_DISCOVERY_SCHEMA_VERSION,
};

pub use materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization::{
    current_notebook_output_viewer_packet, LargeOutputVirtualizationRecord, NotebookOutputViewerLane,
    NotebookOutputViewerPacket, NotebookOutputViewerPacketFinding, OutputViewerFinding,
    OutputViewerLaneClass, OutputSizeBucket, OutputVirtualizationStateClass,
    LARGE_OUTPUT_VIRTUALIZATION_RECORD_KIND, NOTEBOOK_OUTPUT_VIEWER_LANE_RECORD_KIND,
    NOTEBOOK_OUTPUT_VIEWER_PACKET_JSON, NOTEBOOK_OUTPUT_VIEWER_PACKET_PATH,
    NOTEBOOK_OUTPUT_VIEWER_PACKET_RECORD_KIND, NOTEBOOK_OUTPUT_VIEWER_SCHEMA_VERSION,
};

pub use implement_the_notebook_variable_explorer_live_or_snapshot_or_stale_labels_and_typed_export::{
    current_notebook_variable_explorer_packet, NotebookVariableExplorer,
    NotebookVariableExplorerPacket, NotebookVariableExplorerPacketFinding,
    VariableExplorerExportFormatClass, VariableExplorerExportPostureClass,
    VariableExplorerExportScopeClass, VariableExplorerFilterClass,
    VariableExplorerFinding, VariableExplorerSortClass, VariableExplorerTypedExport,
    VariableExplorerTypedExportFinding,
    NOTEBOOK_VARIABLE_EXPLORER_PACKET_JSON, NOTEBOOK_VARIABLE_EXPLORER_PACKET_PATH,
    NOTEBOOK_VARIABLE_EXPLORER_PACKET_RECORD_KIND, NOTEBOOK_VARIABLE_EXPLORER_RECORD_KIND,
    NOTEBOOK_VARIABLE_EXPLORER_SCHEMA_VERSION, VARIABLE_EXPLORER_TYPED_EXPORT_RECORD_KIND,
};

pub use add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation::{
    current_notebook_search_outline_navigation_packet, NotebookBreadcrumb,
    NotebookBreadcrumbClass, NotebookBreadcrumbFinding, NotebookCellTarget,
    NotebookCellTargetClass, NotebookCellTargetFinding, NotebookOutlineItem,
    NotebookOutlineItemClass, NotebookOutlineItemFinding, NotebookSearchMatchClass,
    NotebookSearchOutlineNavigationPacket, NotebookSearchOutlineNavigationPacketFinding,
    NotebookSearchQuery, NotebookSearchQueryFinding, NotebookSearchScopeClass,
    NotebookScrollBehaviorClass, SearchOutlineNavigationFinding,
    NOTEBOOK_BREADCRUMB_RECORD_KIND, NOTEBOOK_CELL_TARGET_RECORD_KIND,
    NOTEBOOK_OUTLINE_ITEM_RECORD_KIND, NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_JSON,
    NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_PATH,
    NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND,
    NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION, NOTEBOOK_SEARCH_QUERY_RECORD_KIND,
};

pub use ship_notebook_activity_integration_with_task_event_model_activity_center_and_restore_safe_histories::{
    current_notebook_activity_integration_packet, ActivityIntegrationFinding,
    NotebookActivityAction, NotebookActivityActorKind, NotebookActivityCenterRow,
    NotebookActivityCenterRowFinding, NotebookActivityFollowUpState,
    NotebookActivityFreshnessClass, NotebookActivityIntegrationPacket,
    NotebookActivityIntegrationPacketFinding, NotebookActivityObjectKind,
    NotebookActivityOutcome, NotebookActivitySourceClass, NotebookActivitySurfaceClass,
    NotebookRestoreClass, NotebookRestorePosture, NotebookRestoreSafeHistory,
    NotebookRestoreSafeHistoryFinding, NotebookTaskEvent, NotebookTaskEventFinding,
    NotebookTaskEventKind, NotebookTaskStateClass, NOTEBOOK_ACTIVITY_CENTER_ROW_RECORD_KIND,
    NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_JSON, NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_PATH,
    NOTEBOOK_ACTIVITY_INTEGRATION_PACKET_RECORD_KIND, NOTEBOOK_ACTIVITY_INTEGRATION_SCHEMA_VERSION,
    NOTEBOOK_RESTORE_SAFE_HISTORY_RECORD_KIND, NOTEBOOK_TASK_EVENT_RECORD_KIND,
};

pub use implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues::{
    current_notebook_debugger_support_packet, BreakpointAffordance, BreakpointAffordanceClass,
    BreakpointAffordanceFinding, BreakpointAffordancePostureClass, DebuggerSupportFinding,
    DebuggerSupportStateClass, NotebookDebuggerSupportPacket, NotebookDebuggerSupportPacketFinding,
    NotebookDebuggerSupportState, NotebookDebuggerSupportStateFinding, UnsupportedStateCue,
    UnsupportedStateCueClass, UnsupportedStateCueFinding, NOTEBOOK_DEBUGGER_SUPPORT_PACKET_JSON,
    NOTEBOOK_DEBUGGER_SUPPORT_PACKET_PATH, NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND,
    NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION, NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND,
    BREAKPOINT_AFFORDANCE_RECORD_KIND, UNSUPPORTED_STATE_CUE_RECORD_KIND,
};

pub use seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet::{
    current_notebook_support_packet, HeavyOutputCorpusEntry, HeavyOutputCorpusEntryFinding,
    HeavyOutputCorpusSizeBucketClass, HeavyOutputCorpusTrustImplicationClass,
    HeavyOutputCorpusVirtualizationClass, NotebookRoundTripFixture,
    NotebookRoundTripFixtureFinding, NotebookRoundTripFixtureKindClass,
    NotebookSupportFinding, NotebookSupportPacket, NotebookSupportPacketCoverageClass,
    NotebookSupportPacketFinding, HEAVY_OUTPUT_CORPUS_ENTRY_RECORD_KIND,
    NOTEBOOK_ROUND_TRIP_FIXTURE_RECORD_KIND, NOTEBOOK_SUPPORT_PACKET_JSON,
    NOTEBOOK_SUPPORT_PACKET_PATH, NOTEBOOK_SUPPORT_PACKET_RECORD_KIND,
    NOTEBOOK_SUPPORT_SCHEMA_VERSION,
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
