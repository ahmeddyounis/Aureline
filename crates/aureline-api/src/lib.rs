//! API-client, request-workspace, and auth-source inspector contracts.
//!
//! This crate owns the typed records that keep versioned request-workspace
//! documents, layered environment sets, auth-source inspectors, and request
//! qualification packets attributable and inspectable without depending on
//! hidden shell shortcuts or ad hoc scripts. The request-workspace boundary
//! schema is
//! [`/schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json`](../../../schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`](../../../artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json).
//!
//! Raw endpoint URLs, raw secrets, raw credential bodies, and raw cookie or
//! token values do not belong in these records. They carry stable IDs, closed
//! posture vocabularies, and reviewable summaries that UI, CLI, export,
//! support, and public-proof surfaces can ingest safely.

#![doc(html_root_url = "https://docs.rs/aureline-api/0.0.0")]

pub mod materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors;

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
