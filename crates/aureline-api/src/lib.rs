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
//! Raw endpoint URLs, raw secrets, raw credential bodies, raw cookie or
//! token values do not belong in these records. They carry stable IDs, closed
//! posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

#![doc(html_root_url = "https://docs.rs/aureline-api/0.0.0")]

pub mod add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows;
pub mod implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling;
pub mod implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export;
pub mod materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors;
pub mod ship_rest_and_graphql_response_viewers_assertions_timing_tabs_and_browser_runtime_trust_classes;

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

pub use add_the_statement_safety_classifier_write_mode_bar_and_protected_target_step_up_flows::{
    current_statement_safety_qualification, AmbiguityReasonClass, BlockedReasonClass,
    MultiStatementPostureClass, ObjectImpactClass, ObjectImpactEnvelope, PerStatementClassDescriptor,
    ProtectedTargetStepUpRow, StatementSafetyClass, StatementSafetyClassifierRow,
    StatementSafetyExecutionOrigin, StatementSafetyQualificationLabel, StatementSafetyQualificationPacket,
    StatementSafetyQualificationProof, StatementSafetyQualificationSummary, StatementSafetyQualificationViolation,
    StatementSafetyQualificationViolationKind, StatementSafetySurfaceGuardSet, StatementSafetySurfaceKind,
    StatementSafetySurfaceQualificationRow, StatementSafetyWritePosture, StepUpKind, StepUpState,
    TransactionContextClass, TransactionEnvelope, WriteModeBarRow,
    STATEMENT_SAFETY_QUALIFICATION_PACKET_JSON, STATEMENT_SAFETY_QUALIFICATION_PACKET_PATH,
    STATEMENT_SAFETY_QUALIFICATION_RECORD_KIND, STATEMENT_SAFETY_QUALIFICATION_SCHEMA_VERSION,
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
