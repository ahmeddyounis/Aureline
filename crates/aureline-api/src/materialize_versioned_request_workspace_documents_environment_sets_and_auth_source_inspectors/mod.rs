//! Versioned request-workspace documents, environment sets, and auth-source
//! inspector qualification records.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_active_repair_state,
    seeded_secret_boundary_profile_parity_rows, seeded_secret_boundary_repairable_states,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryVaultPickerOption,
    SecretBoundaryVaultPickerState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

/// Supported schema version for request-workspace qualification packets.
pub const REQUEST_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RequestQualificationPacket`].
pub const REQUEST_QUALIFICATION_RECORD_KIND: &str =
    "versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors";

/// Repo-relative path to the checked-in request-workspace qualification packet.
pub const REQUEST_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json";

/// Embedded checked-in packet JSON.
pub const REQUEST_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json"
));

const REQUEST_SEND_MATRIX_ROW_ID: &str = "m5.secret.request_workspace.send_http";
const REQUEST_HISTORY_MATRIX_ROW_ID: &str = "m5.secret.request_workspace.history_replay";

/// Qualification label shown on promoted request-workspace surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl RequestQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Request-workspace surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestSurfaceKind {
    /// Request document editor or viewer.
    RequestEditor,
    /// Environment set picker or layer inspector.
    EnvironmentPicker,
    /// Auth-source inspector surface.
    AuthInspector,
    /// Send / replay / assert action bar.
    SendBar,
    /// Request history and replay row.
    HistoryRow,
    /// Response export or handoff review.
    ExportReview,
    /// Schema snapshot inspector.
    SchemaInspector,
    /// Effective-request inspector before send.
    EffectiveRequestInspector,
}

/// Request document kind for versioned workspace files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestDocumentKind {
    /// HTTP request document.
    HttpRequest,
    /// GraphQL operation document.
    GraphqlOperation,
}

/// Write authority state shown across request surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestWritePosture {
    /// Request is constrained to safe read-only work.
    ReadOnly,
    /// Mutating requests are possible after guardrails and review.
    WriteCapable,
    /// Policy blocks execution or export.
    PolicyBlocked,
}

/// Credential source mode shown without exposing raw secret material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceMode {
    /// No authentication is required.
    NoAuth,
    /// Credential is referenced through the secret broker.
    SecretBrokerHandle,
    /// Identity is delegated from a signed-in session.
    DelegatedIdentity,
    /// Enterprise policy injected the credential reference.
    PolicyInjectedCredential,
    /// Managed workspace or cloud service identity.
    ManagedServiceIdentity,
    /// mTLS or signing certificate reference.
    Mtls,
    /// Imported packet has no live credential.
    ImportedNoLiveAuth,
    /// Auth source is blocked by policy.
    PolicyBlocked,
}

/// Provenance of an auth source or environment value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceProvenance {
    /// Value originates from a checked-in request file.
    RequestFile,
    /// Value originates from workspace or profile defaults.
    WorkspaceDefault,
    /// Value was injected by enterprise policy.
    PolicyInjection,
    /// Value came from an ad hoc user override.
    AdHocOverride,
    /// Value resolves through the secret broker.
    SecretBroker,
}

/// Layer kind for environment set resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentLayerKind {
    /// Layer defined in the request file.
    RequestFile,
    /// Layer from workspace or profile defaults.
    WorkspaceDefault,
    /// Layer injected by policy.
    PolicyInjection,
    /// Ad hoc user override layer.
    AdHocOverride,
    /// Layer resolved from secret broker handle.
    SecretHandle,
}

/// Safe-preview class for response components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSafePreviewClass {
    /// Rendered as a navigable JSON tree.
    JsonTree,
    /// Raw text with no execution rights.
    RawText,
    /// Sanitized rich rendering.
    SanitizedRich,
    /// Metadata only, no body values.
    MetadataOnly,
    /// Digest or hash only.
    DigestOnly,
    /// Summary for large payloads.
    LargePayloadSummary,
    /// Redacted, no values shown.
    Redacted,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestSurfaceGuardSet {
    /// Request document kind and version are visible.
    pub document_kind_visible: bool,
    /// Endpoint identity is visible through opaque, non-secret refs.
    pub endpoint_identity_visible: bool,
    /// Auth source mode is visible without raw secrets.
    pub auth_source_visible: bool,
    /// Environment set layers and provenance are visible.
    pub environment_layers_visible: bool,
    /// Effective request inspector shows value sources before send.
    pub effective_inspector_visible: bool,
    /// Read-only, write-capable, or blocked posture is visible.
    pub write_posture_visible: bool,
    /// Schema snapshot freshness is visible.
    pub schema_freshness_visible: bool,
    /// Response safe-preview class is visible.
    pub response_preview_visible: bool,
    /// Export/copy/handoff redaction posture is explicit.
    pub export_redaction_visible: bool,
}

impl RequestSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.document_kind_visible
            && self.endpoint_identity_visible
            && self.auth_source_visible
            && self.environment_layers_visible
            && self.effective_inspector_visible
            && self.write_posture_visible
            && self.schema_freshness_visible
            && self.response_preview_visible
            && self.export_redaction_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: RequestSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: RequestQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: RequestQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<RequestQualificationProof>,
    /// Visible guard set.
    pub guards: RequestSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One versioned request-workspace document row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestWorkspaceDocumentRow {
    /// Stable document id.
    pub document_id: String,
    /// Document kind.
    pub document_kind: RequestDocumentKind,
    /// Document version.
    pub document_version: RequestDocumentVersion,
    /// Method or operation kind label.
    pub method_kind: String,
    /// Path template.
    pub path_template: String,
    /// Header refs (opaque, non-secret).
    pub header_refs: Vec<String>,
    /// Body ref (opaque, non-secret).
    pub body_ref: String,
    /// Variable refs.
    pub variable_refs: Vec<String>,
    /// Assertion refs.
    pub assertion_refs: Vec<String>,
    /// Collection tags.
    pub collection_tags: Vec<String>,
    /// Environment set ref.
    pub environment_set_ref: String,
    /// Auth source ref.
    pub auth_source_ref: String,
    /// Write posture.
    pub write_posture: RequestWritePosture,
    /// Schema snapshot ref, if any.
    pub schema_snapshot_ref: Option<String>,
    /// Whether the document is diffable and versionable.
    pub diffable: bool,
    /// Whether the document is reusable from CLI and automation.
    pub cli_reusable: bool,
}

/// Document version descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestDocumentVersion {
    /// Major version.
    pub major: u32,
    /// Minor version.
    pub minor: u32,
    /// Patch version.
    pub patch: u32,
}

/// One environment set row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentSetRow {
    /// Stable environment id.
    pub environment_id: String,
    /// Base URL (opaque ref, not raw endpoint internals).
    pub base_url_ref: String,
    /// Layer rows.
    pub layers: Vec<EnvironmentLayerRow>,
    /// Secret handle refs.
    pub secret_handle_refs: Vec<String>,
    /// Effective fingerprint ref.
    pub effective_fingerprint_ref: String,
    /// Whether effective resolution is previewable before send.
    pub previewable: bool,
    /// Whether raw secrets are excluded from portable export.
    pub excludes_raw_secrets_from_export: bool,
}

/// One environment layer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentLayerRow {
    /// Stable layer id.
    pub layer_id: String,
    /// Layer kind.
    pub layer_kind: EnvironmentLayerKind,
    /// Variable keys defined in this layer (not values).
    pub variable_keys: Vec<String>,
    /// Provenance.
    pub provenance: AuthSourceProvenance,
    /// Whether this layer may override lower layers.
    pub overrides_lower: bool,
}

/// One auth-source inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthSourceInspectorRow {
    /// Stable auth source id.
    pub auth_source_id: String,
    /// Auth strategy kind.
    pub auth_mode: AuthSourceMode,
    /// Broker handle refs (opaque).
    pub broker_handle_refs: Vec<String>,
    /// Whether refresh or challenge metadata is visible.
    pub refresh_metadata_visible: bool,
    /// mTLS or signing refs (opaque).
    pub mtls_signing_refs: Vec<String>,
    /// Whether auth source is visible without exposing raw secret material.
    pub visible_without_secret: bool,
    /// Provenance of the auth source.
    pub provenance: AuthSourceProvenance,
    /// Policy-blocked reason, if applicable.
    pub policy_blocked_reason: Option<String>,
}

/// One effective-request inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EffectiveRequestInspectorRow {
    /// Stable inspector row id.
    pub case_id: String,
    /// Whether the inspector shows document-level values.
    pub shows_document_values: bool,
    /// Whether the inspector shows workspace/profile defaults.
    pub shows_workspace_defaults: bool,
    /// Whether the inspector shows policy-injected values.
    pub shows_policy_injected: bool,
    /// Whether the inspector shows ad hoc overrides.
    pub shows_ad_hoc_overrides: bool,
    /// Whether the inspector shows secret-handle resolutions.
    pub shows_secret_handles: bool,
    /// Whether the inspector is visible before send.
    pub visible_before_send: bool,
    /// Whether the inspector preserves provenance in exports.
    pub preserves_provenance_in_export: bool,
}

/// One schema snapshot row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaSnapshotRow {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Source kind (openapi, graphql_introspection, etc.).
    pub source_kind: String,
    /// Digest ref.
    pub digest_ref: String,
    /// Freshness state.
    pub freshness_state: String,
    /// Target endpoint identity ref.
    pub target_endpoint_ref: String,
    /// Whether stale schema is visibly labeled.
    pub stale_labeled: bool,
    /// Whether stale schema may masquerade as live truth.
    pub may_masquerade_as_live: bool,
}

/// Summary counts for a request-workspace qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of request document rows.
    pub document_count: usize,
    /// Number of environment set rows.
    pub environment_set_count: usize,
    /// Number of auth-source inspector rows.
    pub auth_source_count: usize,
    /// Number of effective-request inspector rows.
    pub effective_inspector_count: usize,
    /// Number of schema snapshot rows.
    pub schema_snapshot_count: usize,
}

/// Canonical request-workspace qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<RequestSurfaceQualificationRow>,
    /// Request document rows.
    pub documents: Vec<RequestWorkspaceDocumentRow>,
    /// Environment set rows.
    pub environment_sets: Vec<EnvironmentSetRow>,
    /// Auth-source inspector rows.
    pub auth_sources: Vec<AuthSourceInspectorRow>,
    /// Effective-request inspector rows.
    pub effective_inspectors: Vec<EffectiveRequestInspectorRow>,
    /// Schema snapshot rows.
    pub schema_snapshots: Vec<SchemaSnapshotRow>,
    /// Summary counts.
    pub summary: RequestQualificationSummary,
}

impl RequestQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> RequestQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        RequestQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            document_count: self.documents.len(),
            environment_set_count: self.environment_sets.len(),
            auth_source_count: self.auth_sources.len(),
            effective_inspector_count: self.effective_inspectors.len(),
            schema_snapshot_count: self.schema_snapshots.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<RequestQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != REQUEST_QUALIFICATION_SCHEMA_VERSION {
            violations.push(RequestQualificationViolation::SchemaVersion {
                expected: REQUEST_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REQUEST_QUALIFICATION_RECORD_KIND {
            violations.push(RequestQualificationViolation::RecordKind {
                expected: REQUEST_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::Surface,
        );
        collect_ids(
            self.documents.iter().map(|row| row.document_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::Document,
        );
        collect_ids(
            self.environment_sets
                .iter()
                .map(|row| row.environment_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::EnvironmentSet,
        );
        collect_ids(
            self.auth_sources
                .iter()
                .map(|row| row.auth_source_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::AuthSource,
        );
        collect_ids(
            self.effective_inspectors
                .iter()
                .map(|row| row.case_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::EffectiveInspector,
        );
        collect_ids(
            self.schema_snapshots
                .iter()
                .map(|row| row.snapshot_id.as_str()),
            &mut violations,
            RequestQualificationViolationKind::SchemaSnapshot,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(RequestQualificationViolation::StableSurfaceMissingProof {
                        surface_id: surface.surface_id.clone(),
                    });
                }
                if !surface.guards.all_visible() {
                    violations.push(RequestQualificationViolation::StableSurfaceMissingGuard {
                        surface_id: surface.surface_id.clone(),
                    });
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    RequestQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let document_kinds: BTreeSet<_> =
            self.documents.iter().map(|row| row.document_kind).collect();
        for required_kind in [
            RequestDocumentKind::HttpRequest,
            RequestDocumentKind::GraphqlOperation,
        ] {
            if !document_kinds.contains(&required_kind) {
                violations.push(RequestQualificationViolation::MissingDocumentKind {
                    document_kind: required_kind,
                });
            }
        }

        let write_postures: BTreeSet<_> =
            self.documents.iter().map(|row| row.write_posture).collect();
        for required_posture in [
            RequestWritePosture::ReadOnly,
            RequestWritePosture::WriteCapable,
            RequestWritePosture::PolicyBlocked,
        ] {
            if !write_postures.contains(&required_posture) {
                violations.push(RequestQualificationViolation::MissingWritePosture {
                    write_posture: required_posture,
                });
            }
        }

        for row in &self.documents {
            if row.path_template.is_empty()
                || row.environment_set_ref.is_empty()
                || row.auth_source_ref.is_empty()
                || !row.diffable
            {
                violations.push(
                    RequestQualificationViolation::IncompleteDocumentProjection {
                        document_id: row.document_id.clone(),
                    },
                );
            }
        }

        let auth_modes: BTreeSet<_> = self.auth_sources.iter().map(|row| row.auth_mode).collect();
        for required_mode in [
            AuthSourceMode::NoAuth,
            AuthSourceMode::SecretBrokerHandle,
            AuthSourceMode::DelegatedIdentity,
            AuthSourceMode::PolicyBlocked,
        ] {
            if !auth_modes.contains(&required_mode) {
                violations.push(RequestQualificationViolation::MissingAuthSourceMode {
                    auth_mode: required_mode,
                });
            }
        }
        for row in &self.auth_sources {
            if !row.visible_without_secret {
                violations.push(RequestQualificationViolation::AuthSourceHidesSecret {
                    auth_source_id: row.auth_source_id.clone(),
                });
            }
        }

        let mut all_provenances = BTreeSet::new();
        for row in &self.environment_sets {
            if !(row.previewable && row.excludes_raw_secrets_from_export) {
                violations.push(RequestQualificationViolation::UnsafeEnvironmentSet {
                    environment_id: row.environment_id.clone(),
                });
            }
            for layer in &row.layers {
                all_provenances.insert(layer.provenance);
            }
        }
        for required_provenance in [
            AuthSourceProvenance::RequestFile,
            AuthSourceProvenance::WorkspaceDefault,
            AuthSourceProvenance::SecretBroker,
        ] {
            if !all_provenances.contains(&required_provenance) {
                violations.push(RequestQualificationViolation::MissingEnvironmentLayer {
                    environment_id: "corpus".to_owned(),
                    provenance: required_provenance,
                });
            }
        }

        for row in &self.effective_inspectors {
            if !(row.shows_document_values
                && row.shows_workspace_defaults
                && row.shows_policy_injected
                && row.shows_ad_hoc_overrides
                && row.shows_secret_handles
                && row.visible_before_send
                && row.preserves_provenance_in_export)
            {
                violations.push(
                    RequestQualificationViolation::IncompleteEffectiveInspector {
                        case_id: row.case_id.clone(),
                    },
                );
            }
        }

        for row in &self.schema_snapshots {
            if row.digest_ref.is_empty() || row.freshness_state.is_empty() {
                violations.push(RequestQualificationViolation::IncompleteSchemaSnapshot {
                    snapshot_id: row.snapshot_id.clone(),
                });
            }
            if row.may_masquerade_as_live {
                violations.push(RequestQualificationViolation::StaleSchemaMayMasquerade {
                    snapshot_id: row.snapshot_id.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(RequestQualificationViolation::SummaryMismatch);
        }

        violations
    }

    /// Projects the shared M5 secret-boundary prompt, credential row, picker,
    /// delegated-row, and export banner state for the request-workspace send
    /// and history/replay surfaces.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let Some(primary_document) = self.documents.first() else {
            return Vec::new();
        };

        let send_auth = self
            .auth_sources
            .iter()
            .find(|row| {
                matches!(
                    row.auth_mode,
                    AuthSourceMode::SecretBrokerHandle
                        | AuthSourceMode::DelegatedIdentity
                        | AuthSourceMode::Mtls
                        | AuthSourceMode::ManagedServiceIdentity
                )
            })
            .unwrap_or_else(|| {
                self.auth_sources
                    .first()
                    .expect("request auth source exists")
            });
        let history_auth = self
            .auth_sources
            .iter()
            .find(|row| !matches!(row.auth_mode, AuthSourceMode::PolicyBlocked))
            .unwrap_or(send_auth);

        vec![
            request_surface_state(
                REQUEST_SEND_MATRIX_ROW_ID,
                "Request send",
                primary_document,
                send_auth,
                vec![
                    workflow_dependency(
                        "workflow:request.send",
                        format!(
                            "Send {} {}",
                            primary_document.method_kind, primary_document.path_template
                        ),
                    ),
                    workflow_dependency(
                        "workflow:request.assertions",
                        "Run request assertions".to_owned(),
                    ),
                ],
                "Declining keeps editing, effective-request review, and metadata-only history available.",
            ),
            request_surface_state(
                REQUEST_HISTORY_MATRIX_ROW_ID,
                "Request history and replay",
                primary_document,
                history_auth,
                vec![
                    workflow_dependency(
                        "workflow:request.replay",
                        "Replay request from history".to_owned(),
                    ),
                    workflow_dependency(
                        "workflow:request.diff",
                        "Diff replay inputs before dispatch".to_owned(),
                    ),
                ],
                "Declining keeps history review, diff, and metadata-only export available.",
            ),
        ]
    }
}

fn request_surface_state(
    matrix_row_id: &str,
    requester_label: &str,
    document: &RequestWorkspaceDocumentRow,
    auth_source: &AuthSourceInspectorRow,
    dependent_workflows: Vec<SecretBoundaryWorkflowDependency>,
    decline_summary: &str,
) -> SecretBoundarySurfaceState {
    let credential_mode = request_credential_mode(auth_source.auth_mode);
    let storage_class = request_storage_class(auth_source.auth_mode);
    let projection_mode = request_projection_mode(auth_source.auth_mode);
    let secret_class = request_secret_class(auth_source.auth_mode);
    let health_state = request_health_state(auth_source.auth_mode);
    let actor_identity = request_actor_identity(auth_source.auth_mode);
    let consumer_identity = SecretBoundaryConsumerIdentityClass::LocalWorkflow;
    let target_label = format!("{} {}", document.method_kind, document.path_template);
    let decline_path = SecretBoundaryDeclinePath {
        decline_label: "Continue local-only".to_owned(),
        still_works_summary: decline_summary.to_owned(),
    };
    let delegated_credential_row = request_delegated_row(matrix_row_id, auth_source);
    let audit_result = secret_boundary_use_audit_result_for_health(health_state);
    let available_controls = request_projection_controls(matrix_row_id, auth_source.auth_mode);

    SecretBoundarySurfaceState {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        secret_access_prompt: SecretBoundarySecretAccessPrompt {
            matrix_row_id: matrix_row_id.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            requester_label: requester_label.to_owned(),
            secret_class,
            target_workflow_label: target_label.clone(),
            storage_class,
            credential_mode,
            projection_mode,
            lifetime_label: request_lifetime_label(auth_source.auth_mode).to_owned(),
            expires_at: request_expires_at(auth_source.auth_mode),
            dependent_workflows: dependent_workflows.clone(),
            decline_path: decline_path.clone(),
        },
        credential_state_row: SecretBoundaryCredentialStateRow {
            matrix_row_id: matrix_row_id.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            display_label: format!("{requester_label} credential state"),
            secret_class,
            source_class: credential_mode,
            target_boundary_label: target_label,
            storage_class,
            projection_mode,
            health_state,
            expires_at: request_expires_at(auth_source.auth_mode),
            rotate_action_label: "Rotate and rebind handle".to_owned(),
            revoke_action_label: "Revoke current request auth".to_owned(),
            test_action_label: "Test request auth".to_owned(),
            dependent_workflows,
            decline_path,
        },
        vault_picker: Some(request_picker_state(matrix_row_id, auth_source)),
        delegated_credential_row,
        consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
            format!("{matrix_row_id}:consumer-receipt"),
            matrix_row_id,
            actor_identity,
            consumer_identity,
            "Workspace or provider policy",
            format!("{requester_label} / {}", document.path_template),
            credential_mode,
            projection_mode,
            storage_class,
            audit_result,
        ),
        projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
            format!("{matrix_row_id}:projection-audit"),
            matrix_row_id,
            actor_identity,
            consumer_identity,
            "Workspace or provider policy",
            format!("{requester_label} / {}", document.path_template),
            projection_mode,
            audit_result,
            SecretBoundaryRepairOwnerClass::User,
            available_controls
                .iter()
                .map(|control| control.control_class)
                .collect(),
        ),
        repairable_states: seeded_secret_boundary_repairable_states(matrix_row_id),
        active_repair_state: seeded_secret_boundary_active_repair_state(matrix_row_id, health_state),
        profile_parity_rows: seeded_secret_boundary_profile_parity_rows(matrix_row_id),
        export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
            matrix_row_id,
            "Raw credentials stay excluded from profiles, support bundles, request history exports, and portable workspace artifacts.",
        ),
    }
}

fn workflow_dependency(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn request_picker_state(
    matrix_row_id: &str,
    auth_source: &AuthSourceInspectorRow,
) -> SecretBoundaryVaultPickerState {
    let selected = request_credential_mode(auth_source.auth_mode);
    let options = vec![
        SecretBoundaryVaultPickerOption {
            option_id: format!("{matrix_row_id}:os-store"),
            option_label: "OS credential store".to_owned(),
            source_class: SecretBoundaryCredentialMode::OsStore,
            storage_class: SecretBoundaryStorageClass::OsStore,
            access_scope_label: "Workspace request auth".to_owned(),
            reveal_policy_label: "Handle only by default".to_owned(),
            portability_note: "Exports only aliases and source labels.".to_owned(),
            open_source_of_truth_action_label: "Open keychain detail".to_owned(),
            selectable: matches!(
                selected,
                SecretBoundaryCredentialMode::OsStore | SecretBoundaryCredentialMode::HandleOnly
            ),
        },
        SecretBoundaryVaultPickerOption {
            option_id: format!("{matrix_row_id}:vault"),
            option_label: "Enterprise vault".to_owned(),
            source_class: SecretBoundaryCredentialMode::EnterpriseVault,
            storage_class: SecretBoundaryStorageClass::EnterpriseVault,
            access_scope_label: "Workspace request auth".to_owned(),
            reveal_policy_label: "Vault handle only".to_owned(),
            portability_note: "Portable exports omit raw values.".to_owned(),
            open_source_of_truth_action_label: "Open vault source".to_owned(),
            selectable: true,
        },
        SecretBoundaryVaultPickerOption {
            option_id: format!("{matrix_row_id}:browser"),
            option_label: "Browser or device-code handoff".to_owned(),
            source_class: SecretBoundaryCredentialMode::BrowserHandoff,
            storage_class: SecretBoundaryStorageClass::SessionOnly,
            access_scope_label: "Session-bounded request auth".to_owned(),
            reveal_policy_label: "No raw secret reveal".to_owned(),
            portability_note: "Session expires and re-prompts after sign-out.".to_owned(),
            open_source_of_truth_action_label: "Open handoff detail".to_owned(),
            selectable: true,
        },
    ];

    SecretBoundaryVaultPickerState {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        picker_label: "Request auth source picker".to_owned(),
        options,
    }
}

fn request_delegated_row(
    matrix_row_id: &str,
    auth_source: &AuthSourceInspectorRow,
) -> Option<SecretBoundaryDelegatedCredentialRow> {
    let delegated_use_class = match auth_source.auth_mode {
        AuthSourceMode::DelegatedIdentity => {
            SecretBoundaryDelegatedUseClass::ServiceIssuedDelegatedIdentity
        }
        AuthSourceMode::ManagedServiceIdentity => SecretBoundaryDelegatedUseClass::RemoteVaultFetch,
        _ => return None,
    };

    Some(SecretBoundaryDelegatedCredentialRow {
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        delegated_use_class,
        target_host_or_workspace_label: "Request workspace".to_owned(),
        expires_at: request_expires_at(auth_source.auth_mode),
        policy_owner_label: "Workspace or provider policy".to_owned(),
        projection_controls: request_projection_controls(matrix_row_id, auth_source.auth_mode),
    })
}

fn request_actor_identity(auth_mode: AuthSourceMode) -> SecretBoundaryActingIdentityClass {
    match auth_mode {
        AuthSourceMode::DelegatedIdentity => SecretBoundaryActingIdentityClass::DelegatedCredential,
        AuthSourceMode::ManagedServiceIdentity => {
            SecretBoundaryActingIdentityClass::ServiceIssuedAuthority
        }
        _ => SecretBoundaryActingIdentityClass::LocalOnlyHandle,
    }
}

fn request_projection_controls(
    matrix_row_id: &str,
    auth_mode: AuthSourceMode,
) -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Editing, effective-request review, and metadata-only history remain available locally.";
    let mut controls = vec![SecretBoundaryProjectionControl::new(
        matrix_row_id,
        SecretBoundaryProjectionControlClass::StopUsingSecret,
        "Stop using request auth",
        local_safe_note,
    )];
    match auth_mode {
        AuthSourceMode::DelegatedIdentity => controls.push(SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop delegated request identity",
            local_safe_note,
        )),
        AuthSourceMode::ManagedServiceIdentity => {
            controls.push(SecretBoundaryProjectionControl::new(
                matrix_row_id,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
                "Drop managed request delegate",
                local_safe_note,
            ));
        }
        _ => {}
    }
    controls
}

fn request_secret_class(auth_mode: AuthSourceMode) -> SecretBoundarySecretClass {
    match auth_mode {
        AuthSourceMode::NoAuth | AuthSourceMode::ImportedNoLiveAuth => {
            SecretBoundarySecretClass::SessionScopedSecretInput
        }
        AuthSourceMode::Mtls => SecretBoundarySecretClass::SshOrClientCertMaterial,
        AuthSourceMode::DelegatedIdentity | AuthSourceMode::ManagedServiceIdentity => {
            SecretBoundarySecretClass::CloudDelegatedIdentity
        }
        _ => SecretBoundarySecretClass::CodeHostOrRegistryToken,
    }
}

fn request_credential_mode(auth_mode: AuthSourceMode) -> SecretBoundaryCredentialMode {
    match auth_mode {
        AuthSourceMode::NoAuth => SecretBoundaryCredentialMode::SessionOnly,
        AuthSourceMode::SecretBrokerHandle => SecretBoundaryCredentialMode::HandleOnly,
        AuthSourceMode::DelegatedIdentity => SecretBoundaryCredentialMode::Delegated,
        AuthSourceMode::PolicyInjectedCredential => SecretBoundaryCredentialMode::EnterpriseVault,
        AuthSourceMode::ManagedServiceIdentity => SecretBoundaryCredentialMode::RemoteVaultFetch,
        AuthSourceMode::Mtls => SecretBoundaryCredentialMode::HandleOnly,
        AuthSourceMode::ImportedNoLiveAuth | AuthSourceMode::PolicyBlocked => {
            SecretBoundaryCredentialMode::NotConfigured
        }
    }
}

fn request_storage_class(auth_mode: AuthSourceMode) -> SecretBoundaryStorageClass {
    match auth_mode {
        AuthSourceMode::DelegatedIdentity
        | AuthSourceMode::ManagedServiceIdentity
        | AuthSourceMode::NoAuth => SecretBoundaryStorageClass::SessionOnly,
        AuthSourceMode::PolicyInjectedCredential => SecretBoundaryStorageClass::EnterpriseVault,
        AuthSourceMode::ImportedNoLiveAuth | AuthSourceMode::PolicyBlocked => {
            SecretBoundaryStorageClass::NotConfigured
        }
        _ => SecretBoundaryStorageClass::OsStore,
    }
}

fn request_projection_mode(auth_mode: AuthSourceMode) -> SecretBoundaryProjectionMode {
    match auth_mode {
        AuthSourceMode::DelegatedIdentity => SecretBoundaryProjectionMode::Delegated,
        AuthSourceMode::ManagedServiceIdentity => SecretBoundaryProjectionMode::RemoteVaultFetch,
        AuthSourceMode::Mtls => SecretBoundaryProjectionMode::ClientCert,
        AuthSourceMode::ImportedNoLiveAuth | AuthSourceMode::PolicyBlocked => {
            SecretBoundaryProjectionMode::HandleOnly
        }
        _ => SecretBoundaryProjectionMode::RequestHeader,
    }
}

fn request_health_state(auth_mode: AuthSourceMode) -> SecretBoundaryHealthStateClass {
    match auth_mode {
        AuthSourceMode::PolicyBlocked => SecretBoundaryHealthStateClass::PolicyBlocked,
        AuthSourceMode::ImportedNoLiveAuth => SecretBoundaryHealthStateClass::Missing,
        _ => SecretBoundaryHealthStateClass::Healthy,
    }
}

fn request_lifetime_label(auth_mode: AuthSourceMode) -> &'static str {
    match auth_mode {
        AuthSourceMode::DelegatedIdentity | AuthSourceMode::ManagedServiceIdentity => {
            "Session-scoped delegated identity"
        }
        AuthSourceMode::ImportedNoLiveAuth | AuthSourceMode::PolicyBlocked => "No live credential",
        _ => "Short-lived request projection",
    }
}

fn request_expires_at(auth_mode: AuthSourceMode) -> Option<String> {
    match auth_mode {
        AuthSourceMode::DelegatedIdentity => Some("2026-06-12T18:00:00Z".to_owned()),
        AuthSourceMode::ManagedServiceIdentity => Some("2026-06-12T19:00:00Z".to_owned()),
        _ => None,
    }
}

/// Loads the checked-in request-workspace qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_request_workspace_qualification(
) -> Result<RequestQualificationPacket, serde_json::Error> {
    serde_json::from_str(REQUEST_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Request document rows.
    Document,
    /// Environment set rows.
    EnvironmentSet,
    /// Auth-source rows.
    AuthSource,
    /// Effective-request inspector rows.
    EffectiveInspector,
    /// Schema snapshot rows.
    SchemaSnapshot,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<RequestQualificationViolation>,
    kind: RequestQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(RequestQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for request-workspace qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequestQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: RequestQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required request document kind is missing.
    MissingDocumentKind { document_kind: RequestDocumentKind },
    /// Required write posture is missing.
    MissingWritePosture { write_posture: RequestWritePosture },
    /// Document row does not project truth everywhere.
    IncompleteDocumentProjection { document_id: String },
    /// Required auth source mode is missing.
    MissingAuthSourceMode { auth_mode: AuthSourceMode },
    /// Auth source hides secret material.
    AuthSourceHidesSecret { auth_source_id: String },
    /// Environment set is not previewable or exports raw secrets.
    UnsafeEnvironmentSet { environment_id: String },
    /// Required environment layer provenance is missing.
    MissingEnvironmentLayer {
        environment_id: String,
        provenance: AuthSourceProvenance,
    },
    /// Effective-request inspector does not show all value sources.
    IncompleteEffectiveInspector { case_id: String },
    /// Schema snapshot lacks digest or freshness.
    IncompleteSchemaSnapshot { snapshot_id: String },
    /// Stale schema may masquerade as live truth.
    StaleSchemaMayMasquerade { snapshot_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for RequestQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingDocumentKind { document_kind } => {
                write!(f, "document kind {document_kind:?} is not covered")
            }
            Self::MissingWritePosture { write_posture } => {
                write!(f, "write posture {write_posture:?} is not covered")
            }
            Self::IncompleteDocumentProjection { document_id } => {
                write!(
                    f,
                    "{document_id} does not project document truth everywhere"
                )
            }
            Self::MissingAuthSourceMode { auth_mode } => {
                write!(f, "auth source mode {auth_mode:?} is not covered")
            }
            Self::AuthSourceHidesSecret { auth_source_id } => {
                write!(
                    f,
                    "{auth_source_id} hides secret material instead of using broker handles"
                )
            }
            Self::UnsafeEnvironmentSet { environment_id } => {
                write!(
                    f,
                    "{environment_id} is not previewable or may export raw secrets"
                )
            }
            Self::MissingEnvironmentLayer {
                environment_id,
                provenance,
            } => {
                write!(f, "{environment_id} lacks a {provenance:?} layer")
            }
            Self::IncompleteEffectiveInspector { case_id } => {
                write!(f, "{case_id} does not show all value sources before send")
            }
            Self::IncompleteSchemaSnapshot { snapshot_id } => {
                write!(f, "{snapshot_id} lacks digest or freshness truth")
            }
            Self::StaleSchemaMayMasquerade { snapshot_id } => {
                write!(f, "{snapshot_id} may masquerade stale schema as live truth")
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for RequestQualificationViolation {}
