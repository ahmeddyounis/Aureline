//! API-client, request-workspace, database-tooling, and auth-source inspector contracts.
//!
//! This crate owns the typed records that keep versioned request-workspace
//! documents, layered environment sets, auth-source inspectors, statement-safety
//! classification, write-mode bars, protected-target step-up flows, and request
//! qualification packets attributable and inspectable without depending on
//! hidden shell shortcuts or ad hoc scripts. The request-workspace boundary
//! schema is
//! [`/schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json`](../../../schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`](../../../artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json).
//!
//! This crate also owns the request composer, mutation-review sheets, replay
//! and history lanes, and redaction-safe export qualification records. The
//! composer boundary schema is
//! [`/schemas/data/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.schema.json`](../../../schemas/data/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json`](../../../artifacts/data/m5/implement-the-request-composer-mutation-review-sheets-and-replay-or-history-lanes-with-redaction-safe-export.json).
//!
//! This crate also owns the REST and GraphQL response viewers, assertion
//! panels, timing tabs, and browser-runtime trust class qualification records.
//! The response-viewer boundary schema is
//! [`/schemas/data/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.schema.json`](../../../schemas/data/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json`](../../../artifacts/data/m5/ship-rest-and-graphql-response-viewers-assertions-timing-tabs-and-browser-runtime-trust-classes.json).
//!
//! This crate also owns the connection browsers, schema trees, and
//! target-context envelope qualification records for database tooling. The
//! database-browser boundary schema is
//! [`/schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json`](../../../schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json`](../../../artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json).
//!
//! This crate also owns the result-grid virtualization, typed copy or export,
//! filter and sort state, and row-count boundary truth qualification records.
//! The result-grid boundary schema is
//! [`/schemas/data/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.schema.json`](../../../schemas/data/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json`](../../../artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json).
//!
//! This crate also owns the explain-plan freshness notes, engine-version
//! context, and plan-comparison flow qualification records. The explain-plan
//! boundary schema is
//! [`/schemas/data/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.schema.json`](../../../schemas/data/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json`](../../../artifacts/data/m5/implement-explain-plan-freshness-notes-engine-version-context-and-plan-comparison-flows.json).
//!
//! This crate also owns the request and database result handoff to notebook,
//! chart, AI, and support-export surface qualification records. The handoff
//! boundary schema is
//! [`/schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json`](../../../schemas/data/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json`](../../../artifacts/data/m5/integrate-request-and-database-result-handoff-to-notebook-chart-ai-and-support-export-surfaces.json).
//!
//! This crate also owns the query history, connection-profile portability,
//! secret-safe auth storage, and mirror or offline truth qualification records.
//! The ship-query-history boundary schema is
//! [`/schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json`](../../../schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`](../../../artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json).
//!
//! This crate also owns the certification qualification records for API,
//! database, and browser-runtime workflows with mutation, redaction, and scale
//! drills. The certification boundary schema is
//! [`/schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json`](../../../schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json`](../../../artifacts/data/m5/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.json).
//!
//! This crate also owns the API-collection, contract-source, request-origin,
//! and persisted-operation matrix qualification records. The matrix boundary
//! schema is
//! [`/schemas/data/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.schema.json`](../../../schemas/data/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`](../../../artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json).
//!
//! This crate also owns the operation-collection and request-list view
//! qualification records that render the matrix as a real consumer surface with
//! protocol class, environment, contract/source badge, last-run state, retention
//! mode, provenance, and open-detail/inspect/export actions. The view boundary
//! schema is
//! [`/schemas/data/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.schema.json`](../../../schemas/data/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json`](../../../artifacts/data/m5/implement-operation-collection-and-request-list-views-with-protocol-class-environment-retention-mode-and-contract-or-source-badges.json).
//!
//! This crate also owns the contract freshness banner, imported-snapshot label,
//! and refresh/diff/open-spec flow qualification records that keep GraphQL and
//! other contract-linked requests honest wherever validation or completion
//! depends on a contract snapshot. The freshness-banner boundary schema is
//! [`/schemas/data/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.schema.json`](../../../schemas/data/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json`](../../../artifacts/data/m5/ship-contract-freshness-banners-imported-snapshot-labels-and-refresh-diff-or-open-spec-flows.json).
//!
//! This crate also owns the request-origin truth and rerun drift-review
//! qualification records that make execution origin a first-class fact across
//! the local-desktop, SSH, container, managed-workspace, and browser-companion
//! execution paths, distinguishing rerun-exactly from rerun-with-current-context
//! and enumerating origin changes before dispatch. The origin-truth boundary
//! schema is
//! [`/schemas/data/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.schema.json`](../../../schemas/data/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.json`](../../../artifacts/data/m5/implement-request-origin-truth-for-local-desktop-ssh-container-managed-workspace-and-browser-companion-execution-paths-with-drift-review.json).
//!
//! This crate also owns the persisted-operation detail, hash/id drift-check,
//! contract-version-review, and no-unsafe-fallback send-rule qualification
//! records that make a request's persisted-operation binding a first-class fact:
//! each detail carries the local name, opaque remote id/hash, contract version,
//! breaking-risk note, and open-contract action, and drift, deprecation, or
//! removal blocks the send behind rerun/regenerate/cancel choices instead of a
//! silent raw-text fallback. The persisted-operation boundary schema is
//! [`/schemas/data/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.schema.json`](../../../schemas/data/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.json`](../../../artifacts/data/m5/add-persisted-operation-detail-hash-or-id-drift-checks-contract-version-review-and-no-unsafe-fallback-send-rules.json).
//!
//! This crate also owns the request-history row qualification records that
//! upgrade request history into a governed object model: each history row keeps
//! the timestamp, environment, origin scope, status/result class, assertion
//! state, retention mode, and redaction posture inspectable, keeps metadata-only
//! retention as the safe default so storing redacted or full payloads needs an
//! explicit reviewed selection, and keeps compare and export export-safe so they
//! never widen retention or drop origin/environment identity. The request-history
//! boundary schema is
//! [`/schemas/data/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.schema.json`](../../../schemas/data/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json`](../../../artifacts/data/m5/implement-request-history-rows-with-environment-origin-scope-assertion-state-redaction-or-retention-mode-and-export-safe-compare.json).
//!
//! This crate also owns the auth-sheet, secret-source cue, browser/device-code
//! continuity, and offline or mirror-safe collection-portability qualification
//! records that make API auth configuration and collection portability explicit
//! and honest: each auth sheet states the auth scheme, secret source, token
//! lifetime, browser/device-code state, and policy note without persisting a raw
//! secret; each secret-source cue names where a credential resolves from without
//! exposing it; each continuity row keeps an interrupted browser or device-code
//! flow resumable behind a non-secret verification handle; and each
//! collection-portability row preserves contract source, retention mode, and
//! redaction posture across export/import while labeling contract freshness
//! honestly when a collection reopens offline or from a mirror. The
//! auth/portability boundary schema is
//! [`/schemas/data/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.schema.json`](../../../schemas/data/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json`](../../../artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, raw cookie or
//! token values do not belong in these records. They carry stable IDs, closed
//! posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

#![doc(html_root_url = "https://docs.rs/aureline-api/0.0.0")]

pub mod add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules;
pub mod add_staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions;
pub mod add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows;
pub mod certify_api_database_and_browser_runtime_workflows_with_mutation_redaction_and_scale_drills;
pub mod freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix;
pub mod implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling;
pub mod implement_explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows;
pub mod implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges;
pub mod implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare;
pub mod implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review;
pub mod implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export;
pub mod integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces;
pub mod materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors;
pub mod ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability;
pub mod ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows;
pub mod ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth;
pub mod ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes;
pub mod ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth;

pub use implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export::{
    current_request_composer_qualification, ComposerQualificationLabel,
    ComposerQualificationPacket, ComposerQualificationProof, ComposerQualificationSummary,
    ComposerQualificationViolation, ComposerQualificationViolationKind, ComposerSurfaceGuardSet,
    ComposerSurfaceKind, ComposerSurfaceQualificationRow, ExportRedactionClass, HistoryLaneRow,
    HistoryRetentionPosture, MutationReviewSheetRow, MutationRiskClass, RedactionSafeExportRow,
    ReplayConfigRow, ReplayMode, RequestComposerKind, RequestComposerRow, ResponseStreamState,
    COMPOSER_QUALIFICATION_PACKET_JSON, COMPOSER_QUALIFICATION_PACKET_PATH,
    COMPOSER_QUALIFICATION_RECORD_KIND, COMPOSER_QUALIFICATION_SCHEMA_VERSION,
};

pub use materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors::{
    current_request_workspace_qualification, AuthSourceInspectorRow, AuthSourceMode,
    AuthSourceProvenance, EffectiveRequestInspectorRow, EnvironmentLayerKind, EnvironmentLayerRow,
    EnvironmentSetRow, RequestDocumentKind, RequestDocumentVersion, RequestQualificationLabel,
    RequestQualificationPacket, RequestQualificationProof, RequestQualificationSummary,
    RequestQualificationViolation, RequestQualificationViolationKind, RequestSurfaceGuardSet,
    RequestSurfaceKind, RequestSurfaceQualificationRow, RequestWorkspaceDocumentRow,
    RequestWritePosture, ResponseSafePreviewClass, SchemaSnapshotRow, REQUEST_QUALIFICATION_PACKET_JSON,
    REQUEST_QUALIFICATION_PACKET_PATH, REQUEST_QUALIFICATION_RECORD_KIND,
    REQUEST_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling::{
    current_database_browser_qualification, ConnectionBrowserRow, DatabaseBrowserAuthSourceMode,
    DatabaseBrowserConnectionClass, DatabaseBrowserQualificationLabel,
    DatabaseBrowserQualificationPacket, DatabaseBrowserQualificationProof,
    DatabaseBrowserQualificationSummary, DatabaseBrowserQualificationViolation,
    DatabaseBrowserQualificationViolationKind, DatabaseBrowserRedactionMode,
    DatabaseBrowserResultScope, DatabaseBrowserStatementSafetyClass, DatabaseBrowserSurfaceGuardSet,
    DatabaseBrowserSurfaceKind, DatabaseBrowserSurfaceQualificationRow, DatabaseBrowserTransactionPosture,
    DatabaseBrowserWritePosture, SchemaTreeRow, TargetContextEnvelopeRow,
    DATABASE_BROWSER_QUALIFICATION_PACKET_JSON, DATABASE_BROWSER_QUALIFICATION_PACKET_PATH,
    DATABASE_BROWSER_QUALIFICATION_RECORD_KIND, DATABASE_BROWSER_QUALIFICATION_SCHEMA_VERSION,
};

pub use add_staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions::{
    current_staged_row_mutation_qualification, CheckpointActionRow, CheckpointScope,
    ConcurrencyConflictClass, MutationKind, OptimisticConcurrencyCueRow, RollbackActionRow,
    RollbackScope, StagedRowMutationQualificationLabel, StagedRowMutationQualificationPacket,
    StagedRowMutationQualificationProof, StagedRowMutationQualificationSummary,
    StagedRowMutationQualificationViolation, StagedRowMutationQualificationViolationKind,
    StagedRowMutationSheetRow, StagedRowMutationSurfaceGuardSet, StagedRowMutationSurfaceKind,
    StagedRowMutationSurfaceQualificationRow, STAGED_ROW_MUTATION_QUALIFICATION_PACKET_JSON,
    STAGED_ROW_MUTATION_QUALIFICATION_PACKET_PATH, STAGED_ROW_MUTATION_QUALIFICATION_RECORD_KIND,
    STAGED_ROW_MUTATION_QUALIFICATION_SCHEMA_VERSION,
};

pub use add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows::{
    current_statement_safety_qualification, AmbiguityReasonClass, BlockedReasonClass,
    MultiStatementPostureClass, ObjectImpactClass, ObjectImpactEnvelope,
    PerStatementClassDescriptor, ProtectedTargetStepUpRow, StatementSafetyClass,
    StatementSafetyClassifierRow, StatementSafetyExecutionOrigin,
    StatementSafetyQualificationLabel, StatementSafetyQualificationPacket,
    StatementSafetyQualificationProof, StatementSafetyQualificationSummary,
    StatementSafetyQualificationViolation, StatementSafetyQualificationViolationKind,
    StatementSafetySurfaceGuardSet, StatementSafetySurfaceKind,
    StatementSafetySurfaceQualificationRow, StatementSafetyWritePosture, StepUpKind, StepUpState,
    TransactionContextClass, TransactionEnvelope, WriteModeBarRow,
    STATEMENT_SAFETY_QUALIFICATION_PACKET_JSON, STATEMENT_SAFETY_QUALIFICATION_PACKET_PATH,
    STATEMENT_SAFETY_QUALIFICATION_RECORD_KIND, STATEMENT_SAFETY_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth::{
    current_result_grid_qualification, ColumnProvenanceClass, ColumnTypeClass, ExportFormatClass,
    ExportPostureClass, FilterEvaluationLocus, FilterSortStatePanelRow,
    NotebookHandoffStateClass, ResultGridQualificationLabel, ResultGridQualificationPacket,
    ResultGridQualificationProof, ResultGridQualificationSummary, ResultGridQualificationViolation,
    ResultGridQualificationViolationKind, ResultGridSurfaceGuardSet, ResultGridSurfaceKind,
    ResultGridSurfaceQualificationRow, ResultGridViewerRow, RowCountBoundaryChipRow,
    RowCountTruthClass, TruncationReasonClass, TruncationStateClass, TypedCopyActionRow,
    TypedExportActionRow, TypeCoercionStateClass, VirtualizationPostureClass,
    RESULT_GRID_QUALIFICATION_PACKET_JSON, RESULT_GRID_QUALIFICATION_PACKET_PATH,
    RESULT_GRID_QUALIFICATION_RECORD_KIND, RESULT_GRID_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_explain_plan_freshness_notes_engine_version_context_and_plan_comparison_flows::{
    current_explain_plan_qualification, ComparisonBasis, ComparisonOutcome,
    EngineVersionContextRow, ExplainPlanFreshnessNoteRow, ExplainPlanMode,
    ExplainPlanQualificationLabel, ExplainPlanQualificationPacket, ExplainPlanQualificationProof,
    ExplainPlanQualificationSummary, ExplainPlanQualificationViolation,
    ExplainPlanQualificationViolationKind, ExplainPlanSurfaceGuardSet, ExplainPlanSurfaceKind,
    ExplainPlanSurfaceQualificationRow, FreshnessState, PlanComparisonFlowRow,
    EXPLAIN_PLAN_QUALIFICATION_PACKET_JSON, EXPLAIN_PLAN_QUALIFICATION_PACKET_PATH,
    EXPLAIN_PLAN_QUALIFICATION_RECORD_KIND, EXPLAIN_PLAN_QUALIFICATION_SCHEMA_VERSION,
};

pub use integrate_request_and_database_result_handoff_to_notebook_chart_ai_and_support_export_surfaces::{
    current_handoff_qualification, AiHandoffRow, AiHandoffStateClass, ChartHandoffRow,
    ChartHandoffStateClass, HandoffQualificationLabel, HandoffQualificationPacket,
    HandoffQualificationProof, HandoffQualificationSummary, HandoffQualificationViolation,
    HandoffQualificationViolationKind, HandoffSurfaceGuardSet, HandoffSurfaceKind,
    HandoffSurfaceQualificationRow, NotebookHandoffRow, ResultSetOriginClass,
    SupportExportPostureClass, SupportExportRow,
    HANDOFF_QUALIFICATION_PACKET_JSON, HANDOFF_QUALIFICATION_PACKET_PATH,
    HANDOFF_QUALIFICATION_RECORD_KIND, HANDOFF_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth::{
    current_ship_query_history_qualification, ConnectionProfilePortabilityPosture,
    ConnectionProfilePortabilityRow, MirrorOrOfflineStateClass, MirrorOrOfflineTruthRow,
    QueryHistoryEntryRow, QueryHistoryReplayDriftRisk, QueryHistoryRetentionPosture,
    SecretSafeAuthStorageMode, SecretSafeAuthStorageRow, ShipQueryHistoryQualificationLabel,
    ShipQueryHistoryQualificationPacket, ShipQueryHistoryQualificationProof,
    ShipQueryHistoryQualificationSummary, ShipQueryHistoryQualificationViolation,
    ShipQueryHistoryQualificationViolationKind, ShipQueryHistoryRedactionClass,
    ShipQueryHistorySurfaceGuardSet, ShipQueryHistorySurfaceKind,
    ShipQueryHistorySurfaceQualificationRow,
    SHIP_QUERY_HISTORY_QUALIFICATION_PACKET_JSON, SHIP_QUERY_HISTORY_QUALIFICATION_PACKET_PATH,
    SHIP_QUERY_HISTORY_QUALIFICATION_RECORD_KIND, SHIP_QUERY_HISTORY_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes::{
    current_response_viewer_qualification, AssertionOutcome, AssertionRow, BrowserRuntimeSurfaceKind,
    BrowserRuntimeTrustClass, BrowserRuntimeTrustRow, ResponsePreviewClass,
    ResponseViewerQualificationLabel, ResponseViewerQualificationPacket,
    ResponseViewerQualificationProof, ResponseViewerQualificationSummary,
    ResponseViewerQualificationViolation, ResponseViewerQualificationViolationKind,
    ResponseViewerSurfaceGuardSet, ResponseViewerSurfaceKind, ResponseViewerSurfaceQualificationRow,
    ResponseViewerKind, ResponseViewerRow, TimingPhaseKind, TimingTabRow,
    RESPONSE_VIEWER_QUALIFICATION_PACKET_JSON, RESPONSE_VIEWER_QUALIFICATION_PACKET_PATH,
    RESPONSE_VIEWER_QUALIFICATION_RECORD_KIND, RESPONSE_VIEWER_QUALIFICATION_SCHEMA_VERSION,
};

pub use certify_api_database_and_browser_runtime_workflows_with_mutation_redaction_and_scale_drills::{
    current_certification_qualification, CertificationQualificationLabel,
    CertificationQualificationPacket, CertificationQualificationProof,
    CertificationQualificationSummary, CertificationQualificationViolation,
    CertificationQualificationViolationKind, CertificationSurfaceGuardSet,
    CertificationSurfaceKind, CertificationSurfaceQualificationRow, MutationDrillClass,
    MutationDrillRow, RedactionDrillClass, RedactionDrillRow, ScaleDrillClass, ScaleDrillRow,
    UpstreamPacketRefRow,
    CERTIFICATION_QUALIFICATION_PACKET_JSON, CERTIFICATION_QUALIFICATION_PACKET_PATH,
    CERTIFICATION_QUALIFICATION_RECORD_KIND, CERTIFICATION_QUALIFICATION_SCHEMA_VERSION,
};

pub use freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix::{
    current_api_matrix_qualification, ApiCollectionRow, ApiMatrixQualificationLabel,
    ApiMatrixQualificationPacket, ApiMatrixQualificationProof, ApiMatrixQualificationSummary,
    ApiMatrixQualificationViolation, ApiMatrixQualificationViolationKind, ApiMatrixSurfaceGuardSet,
    ApiMatrixSurfaceKind, ApiMatrixSurfaceQualificationRow, ContractFreshnessState, ContractKind,
    ContractRow, ContractSourceClass, OfflineMirrorBehavior, PersistedOperationBindingRow,
    PersistedOperationBindingState, RequestBindingKind, RequestListRow, RequestOriginDriftState,
    RequestOriginKind, RequestOriginRow, RetentionClassRow, RetentionMode, RetentionScope,
    UpstreamRefRow, API_MATRIX_QUALIFICATION_PACKET_JSON, API_MATRIX_QUALIFICATION_PACKET_PATH,
    API_MATRIX_QUALIFICATION_RECORD_KIND, API_MATRIX_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_operation_collection_and_request_list_views_with_protocol_class_environment_retention_mode_and_contract_or_source_badges::{
    current_request_views_qualification, CollectionViewRow, EnvironmentClass, EnvironmentViewRow,
    LastRunState, ProtocolClass, RequestListViewRow, RequestProvenanceClass,
    RequestViewsQualificationLabel, RequestViewsQualificationPacket, RequestViewsQualificationProof,
    RequestViewsQualificationSummary, RequestViewsQualificationViolation, RequestViewsSurfaceGuardSet,
    RequestViewsSurfaceKind, RequestViewsSurfaceQualificationRow, RequestViewsUpstreamRefRow,
    RequestViewsViolationKind, SavedViewRow, SavedViewVisibility,
    REQUEST_VIEWS_QUALIFICATION_PACKET_JSON, REQUEST_VIEWS_QUALIFICATION_PACKET_PATH,
    REQUEST_VIEWS_QUALIFICATION_RECORD_KIND, REQUEST_VIEWS_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_contract_freshness_banners_imported_snapshot_labels_and_refresh_diff_or_open_spec_flows::{
    current_freshness_banner_qualification, BannerSeverityClass, DiffFlowRow, FreshnessBannerAction,
    FreshnessBannerQualificationLabel, FreshnessBannerQualificationPacket,
    FreshnessBannerQualificationProof, FreshnessBannerQualificationSummary,
    FreshnessBannerQualificationViolation, FreshnessBannerRow, FreshnessBannerSurfaceGuardSet,
    FreshnessBannerSurfaceKind, FreshnessBannerSurfaceQualificationRow,
    FreshnessBannerUpstreamRefRow, FreshnessBannerViolationKind, OpenSpecFlowRow, RefreshFlowRow,
    RefreshMode, SpecTargetKind, FRESHNESS_BANNER_QUALIFICATION_PACKET_JSON,
    FRESHNESS_BANNER_QUALIFICATION_PACKET_PATH, FRESHNESS_BANNER_QUALIFICATION_RECORD_KIND,
    FRESHNESS_BANNER_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_request_origin_truth_for_local_desktop_ssh_container_managed_workspace_and_browser_companion_execution_paths_with_drift_review::{
    current_origin_truth_qualification, OriginChangeKind, OriginChangeRow, OriginExecutionPath,
    OriginTrustBoundaryClass, OriginTruthQualificationLabel, OriginTruthQualificationPacket,
    OriginTruthQualificationProof, OriginTruthQualificationSummary,
    OriginTruthQualificationViolation, OriginTruthSurfaceGuardSet, OriginTruthSurfaceKind,
    OriginTruthSurfaceQualificationRow, OriginTruthUpstreamRefRow, OriginTruthViolationKind,
    RerunReviewMode, RerunReviewSheetRow, ResolvedOriginRow,
    ORIGIN_TRUTH_QUALIFICATION_PACKET_JSON, ORIGIN_TRUTH_QUALIFICATION_PACKET_PATH,
    ORIGIN_TRUTH_QUALIFICATION_RECORD_KIND, ORIGIN_TRUTH_QUALIFICATION_SCHEMA_VERSION,
};

pub use add_persisted_operation_detail_hash_or_id_drift_checks_contract_version_review_and_no_unsafe_fallback_send_rules::{
    current_persisted_operation_qualification, PersistedOpDetailRow, PersistedOpDriftReviewSheetRow,
    PersistedOpQualificationLabel, PersistedOpQualificationPacket, PersistedOpQualificationProof,
    PersistedOpQualificationSummary, PersistedOpQualificationViolation,
    PersistedOpQualificationViolationKind, PersistedOpReviewChoiceRow, PersistedOpSurfaceGuardSet,
    PersistedOpSurfaceKind, PersistedOpSurfaceQualificationRow, PersistedOpUpstreamRefRow,
    PersistedOperationDriftClass, ReviewChoiceKind, SendDecisionClass,
    PERSISTED_OP_QUALIFICATION_PACKET_JSON, PERSISTED_OP_QUALIFICATION_PACKET_PATH,
    PERSISTED_OP_QUALIFICATION_RECORD_KIND, PERSISTED_OP_QUALIFICATION_SCHEMA_VERSION,
};

pub use implement_request_history_rows_with_environment_origin_scope_assertion_state_redaction_or_retention_mode_and_export_safe_compare::{
    current_request_history_qualification, AssertionStateClass, CompareBasisClass,
    HistoryCompareRow, HistoryExportRow, HistoryResultClass, RedactionPostureClass,
    RequestHistoryQualificationLabel, RequestHistoryQualificationPacket,
    RequestHistoryQualificationProof, RequestHistoryQualificationSummary,
    RequestHistoryQualificationViolation, RequestHistoryQualificationViolationKind,
    RequestHistoryRow, RequestHistorySurfaceGuardSet, RequestHistorySurfaceKind,
    RequestHistorySurfaceQualificationRow, RequestHistoryUpstreamRefRow, RetentionSelectionRow,
    REQUEST_HISTORY_QUALIFICATION_PACKET_JSON, REQUEST_HISTORY_QUALIFICATION_PACKET_PATH,
    REQUEST_HISTORY_QUALIFICATION_RECORD_KIND, REQUEST_HISTORY_QUALIFICATION_SCHEMA_VERSION,
};

pub use ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability::{
    current_auth_portability_qualification, AuthPortabilityQualificationLabel,
    AuthPortabilityQualificationPacket, AuthPortabilityQualificationProof,
    AuthPortabilityQualificationSummary, AuthPortabilityQualificationViolation,
    AuthPortabilityQualificationViolationKind, AuthPortabilitySurfaceGuardSet,
    AuthPortabilitySurfaceKind, AuthPortabilitySurfaceQualificationRow,
    AuthPortabilityUpstreamRefRow, AuthSchemeClass, AuthSheetRow, BrowserDeviceCodeContinuityRow,
    BrowserDeviceCodeState, CollectionPortabilityRow, PortabilityDirection, SecretSourceCueRow,
    TokenLifetimeClass, AUTH_PORTABILITY_QUALIFICATION_PACKET_JSON,
    AUTH_PORTABILITY_QUALIFICATION_PACKET_PATH, AUTH_PORTABILITY_QUALIFICATION_RECORD_KIND,
    AUTH_PORTABILITY_QUALIFICATION_SCHEMA_VERSION,
};
