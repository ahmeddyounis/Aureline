//! Canonical M5 secret-boundary depth matrix for credential-bearing surfaces.
//!
//! This module freezes one export-safe matrix for the M5 request-workspace,
//! database, provider/model, registry, preview-route, infrastructure-connector,
//! companion-handoff, and managed-surface lanes that can ask for a secret,
//! delegated identity, trust-store exception, or remote-vault fetch.
//!
//! Each [`SecretBoundarySurfaceRow`] names one claimed credential-bearing
//! surface and binds it to:
//!
//! - one stable matrix row id;
//! - its [`SecretBoundarySurfaceDomain`];
//! - the allowed [`SecretBoundaryCredentialMode`] vocabulary shared across
//!   product, docs, diagnostics, and support export;
//! - the explicit [`SecretBoundaryProjectionMode`],
//!   [`SecretBoundaryStorageClass`], and
//!   [`SecretBoundaryActingIdentityClass`] sets that the surface may use;
//! - the trust-store dependencies and repair owner needed to keep failures
//!   typed instead of collapsing into generic downstream errors; and
//! - one export posture and local-safe continuity note so portable state,
//!   support bundles, and release/public-truth packets all quote the same
//!   boundary model.
//!
//! [`seeded_m5_secret_boundary_depth_packet`] builds the canonical packet and
//! [`current_m5_secret_boundary_depth_packet`] loads the checked-in artifact
//! that later surfaces should ingest rather than copying free-text status.
//! [`SecretBoundarySupportExport`] is the first consumer projection: it keeps
//! row ids, shared vocabulary refs, and export posture intact while excluding
//! raw secret values and raw handle ids.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for the M5 secret-boundary depth packet.
pub const M5_SECRET_BOUNDARY_DEPTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_SECRET_BOUNDARY_DEPTH_RECORD_KIND: &str = "m5_secret_boundary_depth_packet";

/// Stable record-kind tag for the support-export projection.
pub const M5_SECRET_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_secret_boundary_depth_support_export";

/// Repo-relative path to the checked-in packet.
pub const M5_SECRET_BOUNDARY_DEPTH_PATH: &str =
    "artifacts/security/m5/m5-secret-boundary-depth.json";

/// Repo-relative path to the boundary schema.
pub const M5_SECRET_BOUNDARY_DEPTH_SCHEMA_REF: &str =
    "schemas/security/m5-secret-boundary-depth.schema.json";

/// Repo-relative path to the contract doc.
pub const M5_SECRET_BOUNDARY_DEPTH_DOC_REF: &str = "docs/security/m5/m5-secret-boundary-depth.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_SECRET_BOUNDARY_DEPTH_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5-secret-boundary-depth";

/// Shared contract ref pinned by every record in this lane.
pub const M5_SECRET_BOUNDARY_DEPTH_SHARED_CONTRACT_REF: &str =
    "security:m5_secret_boundary_depth:v1";

/// Shared vocabulary ref reused by product, docs, diagnostics, and exports.
pub const M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF: &str =
    "docs/security/m5/m5-secret-boundary-depth.md#shared-vocabulary";

/// Embedded checked-in packet JSON.
pub const M5_SECRET_BOUNDARY_DEPTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/security/m5/m5-secret-boundary-depth.json"
));

/// Surface-domain coverage required by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundarySurfaceDomain {
    /// Request-workspace authoring, replay, and send surfaces.
    RequestWorkspaces,
    /// Database tooling and query-history surfaces.
    DatabaseTooling,
    /// Provider, model, and delegated-auth surfaces.
    ProviderModelLanes,
    /// Registry and package-auth surfaces.
    Registries,
    /// Remote preview and browser-handoff routes.
    PreviewRoutes,
    /// Infrastructure and target-context connector surfaces.
    InfraConnectors,
    /// Companion or mobile/browser handoff surfaces.
    CompanionHandoff,
    /// Managed workspace, sync, and service-backed surfaces.
    ManagedSurfaces,
}

impl SecretBoundarySurfaceDomain {
    /// Every required domain in canonical order.
    pub const ALL: [Self; 8] = [
        Self::RequestWorkspaces,
        Self::DatabaseTooling,
        Self::ProviderModelLanes,
        Self::Registries,
        Self::PreviewRoutes,
        Self::InfraConnectors,
        Self::CompanionHandoff,
        Self::ManagedSurfaces,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestWorkspaces => "request_workspaces",
            Self::DatabaseTooling => "database_tooling",
            Self::ProviderModelLanes => "provider_model_lanes",
            Self::Registries => "registries",
            Self::PreviewRoutes => "preview_routes",
            Self::InfraConnectors => "infra_connectors",
            Self::CompanionHandoff => "companion_handoff",
            Self::ManagedSurfaces => "managed_surfaces",
        }
    }
}

/// Shared credential-mode vocabulary pinned by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryCredentialMode {
    /// Credential authority is backed by the operating-system secure store.
    OsStore,
    /// Credential authority is backed by an enterprise vault.
    EnterpriseVault,
    /// Credential authority lives only for the active session.
    SessionOnly,
    /// The consumer receives only a broker handle or callback.
    HandleOnly,
    /// The consumer acts through delegated identity.
    Delegated,
    /// The credential is refreshed through a device-code flow.
    DeviceCode,
    /// The credential is refreshed or acquired through a browser handoff.
    BrowserHandoff,
    /// The runtime fetches a secret from a remote vault on demand.
    RemoteVaultFetch,
    /// No working credential path is configured.
    NotConfigured,
}

impl SecretBoundaryCredentialMode {
    /// Every shared credential mode in canonical order.
    pub const ALL: [Self; 9] = [
        Self::OsStore,
        Self::EnterpriseVault,
        Self::SessionOnly,
        Self::HandleOnly,
        Self::Delegated,
        Self::DeviceCode,
        Self::BrowserHandoff,
        Self::RemoteVaultFetch,
        Self::NotConfigured,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::EnterpriseVault => "enterprise_vault",
            Self::SessionOnly => "session_only",
            Self::HandleOnly => "handle_only",
            Self::Delegated => "delegated",
            Self::DeviceCode => "device_code",
            Self::BrowserHandoff => "browser_handoff",
            Self::RemoteVaultFetch => "remote_vault_fetch",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Projection mode a surface may use when resolving credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryProjectionMode {
    /// The consumer uses a handle or callback only.
    HandleOnly,
    /// The consumer receives delegated authority.
    Delegated,
    /// A browser handoff mediates the credential refresh or acquisition.
    BrowserHandoff,
    /// A remote vault fetch resolves the credential at use time.
    RemoteVaultFetch,
    /// The broker signs or injects a request header for one request path.
    RequestHeader,
    /// The broker lends a file descriptor or pipe.
    FileDescriptor,
    /// The broker binds a client certificate.
    ClientCert,
    /// The broker performs signing without revealing private material.
    SignOnly,
    /// The broker mounts a target-scoped secret view.
    MountRef,
}

impl SecretBoundaryProjectionMode {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HandleOnly => "handle_only",
            Self::Delegated => "delegated",
            Self::BrowserHandoff => "browser_handoff",
            Self::RemoteVaultFetch => "remote_vault_fetch",
            Self::RequestHeader => "request_header",
            Self::FileDescriptor => "file_descriptor",
            Self::ClientCert => "client_cert",
            Self::SignOnly => "sign_only",
            Self::MountRef => "mount_ref",
        }
    }
}

/// Storage boundary class a surface may rely on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryStorageClass {
    /// The operating-system secure store persists the material.
    OsStore,
    /// An enterprise vault persists the material.
    EnterpriseVault,
    /// The material exists only for the active session.
    SessionOnly,
    /// A remote vault persists the material and is fetched at use time.
    RemoteVault,
    /// The surface has no configured storage path.
    NotConfigured,
}

impl SecretBoundaryStorageClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::EnterpriseVault => "enterprise_vault",
            Self::SessionOnly => "session_only",
            Self::RemoteVault => "remote_vault",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Identity class Aureline acts as for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryActingIdentityClass {
    /// A signed-in human account.
    HumanAccount,
    /// An installation, app, or project grant.
    InstallationAppGrant,
    /// A delegated credential on behalf of a user or service.
    DelegatedCredential,
    /// A locally-held credential forwarded to a remote runtime.
    ForwardedLocalCredential,
    /// A local-only handle with no remote identity widening.
    LocalOnlyHandle,
    /// A service-issued authority bounded to a managed plane.
    ServiceIssuedAuthority,
}

impl SecretBoundaryActingIdentityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::InstallationAppGrant => "installation_app_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::ForwardedLocalCredential => "forwarded_local_credential",
            Self::LocalOnlyHandle => "local_only_handle",
            Self::ServiceIssuedAuthority => "service_issued_authority",
        }
    }
}

/// Trust-store dependency a surface must disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryTrustStoreDependencyClass {
    /// The operating-system trust store must admit the route.
    OsStore,
    /// An imported organization CA bundle is required.
    OrgCaBundle,
    /// A pinned control-plane trust root is required.
    PinnedControlPlane,
    /// SSH known-host proof is required.
    KnownHosts,
    /// A vault reference or vault trust root is required.
    VaultRef,
}

impl SecretBoundaryTrustStoreDependencyClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsStore => "os_store",
            Self::OrgCaBundle => "org_ca_bundle",
            Self::PinnedControlPlane => "pinned_control_plane",
            Self::KnownHosts => "known_hosts",
            Self::VaultRef => "vault_ref",
        }
    }
}

/// Export posture for a credential-bearing surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryExportPostureClass {
    /// Metadata only; no secret bodies or raw handle ids.
    MetadataOnly,
    /// Alias and failure metadata only.
    AliasOnly,
    /// Redacted support export only.
    RedactedSupportExport,
    /// Release/public truth may publish only a summary row.
    ReleaseSummaryOnly,
}

impl SecretBoundaryExportPostureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::AliasOnly => "alias_only",
            Self::RedactedSupportExport => "redacted_support_export",
            Self::ReleaseSummaryOnly => "release_summary_only",
        }
    }
}

/// Owner responsible for the typed repair path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryRepairOwnerClass {
    /// The local user can repair the issue.
    User,
    /// An admin or policy owner must repair the issue.
    Admin,
    /// The provider or registry owner must repair the issue.
    ProviderOperator,
    /// The remote or managed-runtime owner must repair the issue.
    RemoteOperator,
    /// The database or connector owner must repair the issue.
    DataOperator,
    /// The managed service operator must repair the issue.
    ServiceOperator,
}

impl SecretBoundaryRepairOwnerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
            Self::ProviderOperator => "provider_operator",
            Self::RemoteOperator => "remote_operator",
            Self::DataOperator => "data_operator",
            Self::ServiceOperator => "service_operator",
        }
    }
}

/// Consumer surface that must project the matrix id and shared vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryConsumerSurface {
    /// Docs or in-product help.
    DocsHelp,
    /// Diagnostics or inspector surfaces.
    Diagnostics,
    /// Support-export surfaces.
    SupportExport,
    /// Release or public-truth surfaces.
    ReleasePublicTruth,
}

impl SecretBoundaryConsumerSurface {
    /// Every required consumer surface in canonical order.
    pub const ALL: [Self; 4] = [
        Self::DocsHelp,
        Self::Diagnostics,
        Self::SupportExport,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelp => "docs_help",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::ReleasePublicTruth => "release_public_truth",
        }
    }
}

/// One credential-bearing surface row in the M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySurfaceRow {
    /// Stable matrix row id.
    pub matrix_row_id: String,
    /// Human-readable row title.
    pub title: String,
    /// Domain covered by the row.
    pub domain: SecretBoundarySurfaceDomain,
    /// Repo-relative ref of the first consuming implementation or packet.
    pub surface_ref: String,
    /// Allowed shared credential modes for the surface.
    pub allowed_credential_modes: Vec<SecretBoundaryCredentialMode>,
    /// Default credential mode the surface should prefer.
    pub default_credential_mode: SecretBoundaryCredentialMode,
    /// Allowed projection modes for the surface.
    pub projection_modes: Vec<SecretBoundaryProjectionMode>,
    /// Allowed storage classes for the surface.
    pub storage_classes: Vec<SecretBoundaryStorageClass>,
    /// Acting identities the surface may assume.
    pub acting_identities: Vec<SecretBoundaryActingIdentityClass>,
    /// Trust-store dependencies the surface must disclose.
    pub trust_store_dependencies: Vec<SecretBoundaryTrustStoreDependencyClass>,
    /// Export posture for the surface.
    pub export_posture: SecretBoundaryExportPostureClass,
    /// Owner of the typed repair path.
    pub repair_owner: SecretBoundaryRepairOwnerClass,
    /// Export-safe explanation of the repair path.
    pub repair_path: String,
    /// Export-safe explanation of what remains usable without approval.
    pub local_safe_behavior: String,
}

/// Consumer projection that must quote the shared matrix and vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryConsumerProjection {
    /// Consumer surface token.
    pub surface: SecretBoundaryConsumerSurface,
    /// Repo-relative ref of the consumer surface.
    pub surface_ref: String,
    /// Matrix id the consumer must show.
    pub matrix_id: String,
    /// Shared vocabulary ref the consumer must reuse.
    pub vocabulary_ref: String,
    /// `true` when the consumer projects row ids rather than free-text only.
    pub shows_matrix_row_ids: bool,
    /// `true` when the consumer reuses the packet vocabulary verbatim.
    pub uses_shared_vocabulary: bool,
    /// Export-safe description of how the consumer narrows its copy.
    pub notes: String,
}

/// Summary for the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySummary {
    /// Total number of surface rows.
    pub surface_count: usize,
    /// Domain tokens present in the matrix.
    pub domain_tokens_present: Vec<String>,
    /// Default credential-mode tokens present in the matrix.
    pub default_credential_mode_tokens_present: Vec<String>,
    /// Consumer surface tokens present in the packet.
    pub consumer_surface_tokens_present: Vec<String>,
    /// `true` when the packet excludes raw secret bodies.
    pub raw_secret_values_excluded: bool,
    /// `true` when the packet excludes raw handle ids.
    pub raw_handle_ids_excluded: bool,
}

/// Canonical M5 secret-boundary depth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5SecretBoundaryDepthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Stable matrix id shared by product, docs, diagnostics, and exports.
    pub matrix_id: String,
    /// Reviewer-facing label.
    pub label: String,
    /// UTC mint timestamp.
    pub minted_at: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Contract doc ref.
    pub doc_ref: String,
    /// Fixture directory ref.
    pub fixture_dir: String,
    /// Source contracts and implementation refs this packet unifies.
    pub source_contract_refs: Vec<String>,
    /// Surface rows governed by the packet.
    pub surface_rows: Vec<SecretBoundarySurfaceRow>,
    /// Required consumer projections.
    pub consumer_projections: Vec<SecretBoundaryConsumerProjection>,
    /// Recomputed packet summary.
    pub summary: SecretBoundarySummary,
}

impl M5SecretBoundaryDepthPacket {
    /// Recomputes the packet summary from the row and consumer contents.
    pub fn recompute_summary(&self) -> SecretBoundarySummary {
        let mut domain_tokens: BTreeSet<String> = BTreeSet::new();
        let mut mode_tokens: BTreeSet<String> = BTreeSet::new();
        let mut consumer_tokens: BTreeSet<String> = BTreeSet::new();

        for row in &self.surface_rows {
            domain_tokens.insert(row.domain.as_str().to_owned());
            mode_tokens.insert(row.default_credential_mode.as_str().to_owned());
        }
        for projection in &self.consumer_projections {
            consumer_tokens.insert(projection.surface.as_str().to_owned());
        }

        SecretBoundarySummary {
            surface_count: self.surface_rows.len(),
            domain_tokens_present: domain_tokens.into_iter().collect(),
            default_credential_mode_tokens_present: mode_tokens.into_iter().collect(),
            consumer_surface_tokens_present: consumer_tokens.into_iter().collect(),
            raw_secret_values_excluded: true,
            raw_handle_ids_excluded: true,
        }
    }

    /// Validates this packet against the frozen M5 secret-boundary rules.
    pub fn validate(&self) -> Vec<M5SecretBoundaryDepthViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SECRET_BOUNDARY_DEPTH_RECORD_KIND {
            violations.push(M5SecretBoundaryDepthViolation::RecordKindMismatch);
        }
        if self.schema_version != M5_SECRET_BOUNDARY_DEPTH_SCHEMA_VERSION {
            violations.push(M5SecretBoundaryDepthViolation::SchemaVersionMismatch);
        }
        if self.shared_contract_ref != M5_SECRET_BOUNDARY_DEPTH_SHARED_CONTRACT_REF {
            violations.push(M5SecretBoundaryDepthViolation::SharedContractRefMismatch);
        }
        if self.schema_ref != M5_SECRET_BOUNDARY_DEPTH_SCHEMA_REF {
            violations.push(M5SecretBoundaryDepthViolation::SchemaRefMismatch);
        }
        if self.doc_ref != M5_SECRET_BOUNDARY_DEPTH_DOC_REF {
            violations.push(M5SecretBoundaryDepthViolation::DocRefMismatch);
        }
        if self.fixture_dir != M5_SECRET_BOUNDARY_DEPTH_FIXTURE_DIR {
            violations.push(M5SecretBoundaryDepthViolation::FixtureDirMismatch);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5SecretBoundaryDepthViolation::MissingSourceContracts);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_domains = BTreeSet::new();
        for row in &self.surface_rows {
            if !row_ids.insert(row.matrix_row_id.clone()) {
                violations.push(M5SecretBoundaryDepthViolation::DuplicateMatrixRowId(
                    row.matrix_row_id.clone(),
                ));
            }
            seen_domains.insert(row.domain);

            if !row
                .allowed_credential_modes
                .contains(&row.default_credential_mode)
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::DefaultCredentialModeNotAllowed(
                        row.matrix_row_id.clone(),
                    ),
                );
            }
            if row.projection_modes.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingProjectionModes(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.storage_classes.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingStorageClasses(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.acting_identities.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingActingIdentities(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.trust_store_dependencies.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingTrustDependencies(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.repair_path.trim().is_empty() || row.local_safe_behavior.trim().is_empty() {
                violations.push(
                    M5SecretBoundaryDepthViolation::MissingRepairOrContinuityNote(
                        row.matrix_row_id.clone(),
                    ),
                );
            }
            if row.default_credential_mode == SecretBoundaryCredentialMode::NotConfigured
                && row.repair_owner == SecretBoundaryRepairOwnerClass::User
                && !row.local_safe_behavior.to_lowercase().contains("metadata")
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::NotConfiguredWithoutLocalSafeDisclosure(
                        row.matrix_row_id.clone(),
                    ),
                );
            }
        }

        for domain in SecretBoundarySurfaceDomain::ALL {
            if !seen_domains.contains(&domain) {
                violations.push(M5SecretBoundaryDepthViolation::MissingDomainCoverage(
                    domain,
                ));
            }
        }

        let mut seen_consumers = BTreeSet::new();
        for projection in &self.consumer_projections {
            seen_consumers.insert(projection.surface);
            if projection.matrix_id != self.matrix_id
                || projection.vocabulary_ref != M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF
                || !projection.shows_matrix_row_ids
                || !projection.uses_shared_vocabulary
            {
                violations.push(M5SecretBoundaryDepthViolation::ConsumerProjectionDrift(
                    projection.surface,
                ));
            }
        }
        for surface in SecretBoundaryConsumerSurface::ALL {
            if !seen_consumers.contains(&surface) {
                violations.push(M5SecretBoundaryDepthViolation::MissingConsumerProjection(
                    surface,
                ));
            }
        }

        if self.summary != self.recompute_summary() {
            violations.push(M5SecretBoundaryDepthViolation::SummaryMismatch);
        }

        violations
    }
}

/// Metadata-only support export derived from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Source packet id.
    pub packet_id: String,
    /// Shared matrix id.
    pub matrix_id: String,
    /// UTC generation time.
    pub generated_at: String,
    /// Shared vocabulary ref reused by the export.
    pub vocabulary_ref: String,
    /// Support-export row summaries.
    pub rows: Vec<SecretBoundarySupportExportRow>,
    /// `true` when raw secret values are excluded.
    pub raw_secret_values_excluded: bool,
    /// `true` when raw handle ids are excluded.
    pub raw_handle_ids_excluded: bool,
    /// `true` when the export preserves matrix ids.
    pub matrix_ids_preserved: bool,
}

impl SecretBoundarySupportExport {
    /// Builds a metadata-only support export from the packet.
    pub fn from_packet(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        packet: &M5SecretBoundaryDepthPacket,
    ) -> Self {
        Self {
            record_kind: M5_SECRET_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SECRET_BOUNDARY_DEPTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id: packet.packet_id.clone(),
            matrix_id: packet.matrix_id.clone(),
            generated_at: generated_at.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            rows: packet
                .surface_rows
                .iter()
                .map(SecretBoundarySupportExportRow::from_surface_row)
                .collect(),
            raw_secret_values_excluded: true,
            raw_handle_ids_excluded: true,
            matrix_ids_preserved: true,
        }
    }
}

/// Metadata-only row emitted into the support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySupportExportRow {
    /// Matrix row id preserved from the packet.
    pub matrix_row_id: String,
    /// Domain token.
    pub domain_token: String,
    /// Default credential-mode token.
    pub default_credential_mode_token: String,
    /// Export posture token.
    pub export_posture_token: String,
    /// Repair owner token.
    pub repair_owner_token: String,
}

impl SecretBoundarySupportExportRow {
    /// Builds one support-export row from the canonical surface row.
    pub fn from_surface_row(row: &SecretBoundarySurfaceRow) -> Self {
        Self {
            matrix_row_id: row.matrix_row_id.clone(),
            domain_token: row.domain.as_str().to_owned(),
            default_credential_mode_token: row.default_credential_mode.as_str().to_owned(),
            export_posture_token: row.export_posture.as_str().to_owned(),
            repair_owner_token: row.repair_owner.as_str().to_owned(),
        }
    }
}

/// Validation failures for the M5 secret-boundary depth packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SecretBoundaryDepthViolation {
    /// The packet record kind drifted from the frozen constant.
    RecordKindMismatch,
    /// The schema version drifted from the frozen constant.
    SchemaVersionMismatch,
    /// The shared contract ref drifted from the frozen constant.
    SharedContractRefMismatch,
    /// The schema ref drifted from the frozen constant.
    SchemaRefMismatch,
    /// The doc ref drifted from the frozen constant.
    DocRefMismatch,
    /// The fixture-dir ref drifted from the frozen constant.
    FixtureDirMismatch,
    /// No source contracts were declared.
    MissingSourceContracts,
    /// A matrix row id appeared more than once.
    DuplicateMatrixRowId(String),
    /// The default credential mode was not present in the allowed set.
    DefaultCredentialModeNotAllowed(String),
    /// A row omitted every projection mode.
    MissingProjectionModes(String),
    /// A row omitted every storage class.
    MissingStorageClasses(String),
    /// A row omitted every acting identity.
    MissingActingIdentities(String),
    /// A row omitted every trust-store dependency.
    MissingTrustDependencies(String),
    /// A row omitted the repair path or local-safe continuity note.
    MissingRepairOrContinuityNote(String),
    /// A `not_configured` row omitted a local-safe metadata path.
    NotConfiguredWithoutLocalSafeDisclosure(String),
    /// One required domain had no coverage.
    MissingDomainCoverage(SecretBoundarySurfaceDomain),
    /// One required consumer projection was absent.
    MissingConsumerProjection(SecretBoundaryConsumerSurface),
    /// A consumer projection drifted from the matrix id or vocabulary ref.
    ConsumerProjectionDrift(SecretBoundaryConsumerSurface),
    /// The checked summary diverged from the recomputed summary.
    SummaryMismatch,
}

impl fmt::Display for M5SecretBoundaryDepthViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecordKindMismatch => write!(f, "packet record_kind drifted"),
            Self::SchemaVersionMismatch => write!(f, "packet schema_version drifted"),
            Self::SharedContractRefMismatch => write!(f, "packet shared_contract_ref drifted"),
            Self::SchemaRefMismatch => write!(f, "packet schema_ref drifted"),
            Self::DocRefMismatch => write!(f, "packet doc_ref drifted"),
            Self::FixtureDirMismatch => write!(f, "packet fixture_dir drifted"),
            Self::MissingSourceContracts => write!(f, "packet must cite source contracts"),
            Self::DuplicateMatrixRowId(row) => write!(f, "matrix row id {row} is duplicated"),
            Self::DefaultCredentialModeNotAllowed(row) => write!(
                f,
                "row {row} default_credential_mode must appear in allowed_credential_modes"
            ),
            Self::MissingProjectionModes(row) => {
                write!(f, "row {row} must declare at least one projection mode")
            }
            Self::MissingStorageClasses(row) => {
                write!(f, "row {row} must declare at least one storage class")
            }
            Self::MissingActingIdentities(row) => {
                write!(f, "row {row} must declare at least one acting identity")
            }
            Self::MissingTrustDependencies(row) => {
                write!(
                    f,
                    "row {row} must declare at least one trust-store dependency"
                )
            }
            Self::MissingRepairOrContinuityNote(row) => write!(
                f,
                "row {row} must declare both repair_path and local_safe_behavior"
            ),
            Self::NotConfiguredWithoutLocalSafeDisclosure(row) => write!(
                f,
                "row {row} uses not_configured without a local-safe metadata disclosure"
            ),
            Self::MissingDomainCoverage(domain) => {
                write!(f, "domain {} is missing from the matrix", domain.as_str())
            }
            Self::MissingConsumerProjection(surface) => write!(
                f,
                "consumer projection {} is missing from the packet",
                surface.as_str()
            ),
            Self::ConsumerProjectionDrift(surface) => write!(
                f,
                "consumer projection {} drifted from matrix_id or vocabulary_ref",
                surface.as_str()
            ),
            Self::SummaryMismatch => write!(f, "packet summary diverges from recomputed summary"),
        }
    }
}

impl Error for M5SecretBoundaryDepthViolation {}

/// Loads the embedded canonical M5 secret-boundary depth packet.
///
/// # Errors
///
/// Returns a deserialization error if the embedded JSON no longer parses into
/// the typed packet.
pub fn current_m5_secret_boundary_depth_packet(
) -> Result<M5SecretBoundaryDepthPacket, serde_json::Error> {
    serde_json::from_str(M5_SECRET_BOUNDARY_DEPTH_JSON)
}

/// Validates a canonical M5 secret-boundary depth packet.
pub fn validate_m5_secret_boundary_depth_packet(
    packet: &M5SecretBoundaryDepthPacket,
) -> Vec<M5SecretBoundaryDepthViolation> {
    packet.validate()
}

/// Builds the canonical seeded M5 secret-boundary depth packet.
pub fn seeded_m5_secret_boundary_depth_packet() -> M5SecretBoundaryDepthPacket {
    let packet_id = "m5-secret-boundary-depth:2026-06-12".to_owned();
    let matrix_id = "m5.secret_boundary.depth.v1".to_owned();

    let surface_rows = vec![
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.request_workspace.send_http".to_owned(),
            title: "Request workspace HTTP send".to_owned(),
            domain: SecretBoundarySurfaceDomain::RequestWorkspaces,
            surface_ref: "crates/aureline-runtime/src/request_workspace/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::RequestHeader,
                SecretBoundaryProjectionMode::ClientCert,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            export_posture: SecretBoundaryExportPostureClass::RedactedSupportExport,
            repair_owner: SecretBoundaryRepairOwnerClass::User,
            repair_path: "Rebind the broker handle or complete browser/device-code auth before send."
                .to_owned(),
            local_safe_behavior:
                "Edit request files, inspect the effective request, and export metadata-only history without live credentials."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.request_workspace.history_replay".to_owned(),
            title: "Request workspace history and replay".to_owned(),
            domain: SecretBoundarySurfaceDomain::RequestWorkspaces,
            surface_ref: "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::RequestHeader,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            export_posture: SecretBoundaryExportPostureClass::MetadataOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::User,
            repair_path: "Rebind the replayed auth source before dispatch when the prior handle expired or was revoked.".to_owned(),
            local_safe_behavior:
                "Replay review, diff, and history inspection remain available as metadata-only actions."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.database.connection_picker".to_owned(),
            title: "Database connection picker and live session".to_owned(),
            domain: SecretBoundarySurfaceDomain::DatabaseTooling,
            surface_ref: "crates/aureline-data/src/database_qualification.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::FileDescriptor,
                SecretBoundaryProjectionMode::ClientCert,
                SecretBoundaryProjectionMode::SignOnly,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            export_posture: SecretBoundaryExportPostureClass::AliasOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::DataOperator,
            repair_path: "Rotate the connection-scoped handle, certificate binding, or delegated session before reconnect.".to_owned(),
            local_safe_behavior:
                "Statement review, schema inspection, and imported-result browsing stay available without a live credential."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.database.query_history_portability".to_owned(),
            title: "Database query history portability and replay".to_owned(),
            domain: SecretBoundarySurfaceDomain::DatabaseTooling,
            surface_ref: "artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::FileDescriptor,
                SecretBoundaryProjectionMode::RemoteVaultFetch,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::RemoteVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            export_posture: SecretBoundaryExportPostureClass::MetadataOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::DataOperator,
            repair_path: "Re-resolve the connection alias or remote-vault reference before replaying a live query.".to_owned(),
            local_safe_behavior:
                "History review, portability diff, and redacted exports remain usable without reconnect."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.provider_model.route_resolution".to_owned(),
            title: "Provider/model route resolution".to_owned(),
            domain: SecretBoundarySurfaceDomain::ProviderModelLanes,
            surface_ref: "crates/aureline-provider/src/route_resolution/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::Delegated,
            projection_modes: vec![
                SecretBoundaryProjectionMode::Delegated,
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryProjectionMode::RequestHeader,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::InstallationAppGrant,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            export_posture: SecretBoundaryExportPostureClass::RedactedSupportExport,
            repair_owner: SecretBoundaryRepairOwnerClass::ProviderOperator,
            repair_path: "Re-issue the delegated grant, renew the browser/device-code session, or narrow the route to a local-safe path.".to_owned(),
            local_safe_behavior:
                "Account-free local work and cached provider metadata remain available while auth is repaired."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.provider_model.scope_registry".to_owned(),
            title: "Provider scope registry and delegated identity".to_owned(),
            domain: SecretBoundarySurfaceDomain::ProviderModelLanes,
            surface_ref: "crates/aureline-provider/src/account_scope/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::Delegated,
            projection_modes: vec![
                SecretBoundaryProjectionMode::Delegated,
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryProjectionMode::RemoteVaultFetch,
                SecretBoundaryProjectionMode::RequestHeader,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::RemoteVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::InstallationAppGrant,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            export_posture: SecretBoundaryExportPostureClass::RedactedSupportExport,
            repair_owner: SecretBoundaryRepairOwnerClass::ProviderOperator,
            repair_path: "Repair the exact delegated scope, installation grant, or remote-vault lineage that drifted; do not widen to generic connected.".to_owned(),
            local_safe_behavior:
                "Scope inspection, drift review, and local draft queues remain available without live mutation authority."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.registry.package_auth".to_owned(),
            title: "Registry auth, install, and publish".to_owned(),
            domain: SecretBoundarySurfaceDomain::Registries,
            surface_ref: "docs/help/deps/package-mutation-and-registry-review.md".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::RequestHeader,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::InstallationAppGrant,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            export_posture: SecretBoundaryExportPostureClass::AliasOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::User,
            repair_path: "Rebind the registry handle or refresh the delegated token before install or publish.".to_owned(),
            local_safe_behavior:
                "Dependency review, lockfile diff, and local-only resolution remain available without registry credentials."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.preview_route.remote_preview".to_owned(),
            title: "Remote preview route and provider handoff".to_owned(),
            domain: SecretBoundarySurfaceDomain::PreviewRoutes,
            surface_ref: "crates/aureline-review/src/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::Delegated,
            projection_modes: vec![
                SecretBoundaryProjectionMode::Delegated,
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryProjectionMode::RemoteVaultFetch,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::SessionOnly,
                SecretBoundaryStorageClass::RemoteVault,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            export_posture: SecretBoundaryExportPostureClass::ReleaseSummaryOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::RemoteOperator,
            repair_path: "Revalidate the preview route trust or delegated session before reopening a remote preview.".to_owned(),
            local_safe_behavior:
                "Expired previews narrow to metadata-only route history and exact desktop handoff instructions."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.infra_connector.target_context".to_owned(),
            title: "Infrastructure connector target context".to_owned(),
            domain: SecretBoundarySurfaceDomain::InfraConnectors,
            surface_ref: "crates/aureline-api/src/implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::HandleOnly,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::HandleOnly,
            projection_modes: vec![
                SecretBoundaryProjectionMode::HandleOnly,
                SecretBoundaryProjectionMode::ClientCert,
                SecretBoundaryProjectionMode::SignOnly,
                SecretBoundaryProjectionMode::MountRef,
                SecretBoundaryProjectionMode::RemoteVaultFetch,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::RemoteVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::KnownHosts,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            export_posture: SecretBoundaryExportPostureClass::RedactedSupportExport,
            repair_owner: SecretBoundaryRepairOwnerClass::RemoteOperator,
            repair_path: "Repair SSH host proof, client-certificate binding, or vault trust before reconnecting the target context.".to_owned(),
            local_safe_behavior:
                "Manifest inspection, drift review, and policy explanation stay local-safe when live connector auth is blocked."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.companion.session_handoff".to_owned(),
            title: "Companion session handoff".to_owned(),
            domain: SecretBoundarySurfaceDomain::CompanionHandoff,
            surface_ref: "crates/aureline-companion/src/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::BrowserHandoff,
            projection_modes: vec![
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryProjectionMode::Delegated,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::LocalOnlyHandle,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            export_posture: SecretBoundaryExportPostureClass::MetadataOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::User,
            repair_path: "Complete the desktop/browser return path again before resuming the companion handoff.".to_owned(),
            local_safe_behavior:
                "Read-only follow state and handoff descriptors stay available without reviving a live companion credential."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.managed.workspace_runtime".to_owned(),
            title: "Managed workspace runtime".to_owned(),
            domain: SecretBoundarySurfaceDomain::ManagedSurfaces,
            surface_ref: "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::RemoteVaultFetch,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::RemoteVaultFetch,
            projection_modes: vec![
                SecretBoundaryProjectionMode::RemoteVaultFetch,
                SecretBoundaryProjectionMode::Delegated,
                SecretBoundaryProjectionMode::MountRef,
                SecretBoundaryProjectionMode::SignOnly,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::RemoteVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::KnownHosts,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            export_posture: SecretBoundaryExportPostureClass::RedactedSupportExport,
            repair_owner: SecretBoundaryRepairOwnerClass::RemoteOperator,
            repair_path: "Repair the remote-vault lineage, delegated authority, or host proof before resuming managed runtime actions.".to_owned(),
            local_safe_behavior:
                "Local editing and bounded state inspection remain available when managed credential repair is pending."
                    .to_owned(),
        },
        SecretBoundarySurfaceRow {
            matrix_row_id: "m5.secret.managed.sync_plane".to_owned(),
            title: "Managed sync and offboarding control plane".to_owned(),
            domain: SecretBoundarySurfaceDomain::ManagedSurfaces,
            surface_ref: "crates/aureline-companion/src/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/mod.rs".to_owned(),
            allowed_credential_modes: vec![
                SecretBoundaryCredentialMode::OsStore,
                SecretBoundaryCredentialMode::EnterpriseVault,
                SecretBoundaryCredentialMode::Delegated,
                SecretBoundaryCredentialMode::DeviceCode,
                SecretBoundaryCredentialMode::BrowserHandoff,
                SecretBoundaryCredentialMode::NotConfigured,
            ],
            default_credential_mode: SecretBoundaryCredentialMode::Delegated,
            projection_modes: vec![
                SecretBoundaryProjectionMode::Delegated,
                SecretBoundaryProjectionMode::BrowserHandoff,
                SecretBoundaryProjectionMode::HandleOnly,
            ],
            storage_classes: vec![
                SecretBoundaryStorageClass::OsStore,
                SecretBoundaryStorageClass::EnterpriseVault,
                SecretBoundaryStorageClass::SessionOnly,
            ],
            acting_identities: vec![
                SecretBoundaryActingIdentityClass::HumanAccount,
                SecretBoundaryActingIdentityClass::DelegatedCredential,
                SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
            ],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            export_posture: SecretBoundaryExportPostureClass::ReleaseSummaryOnly,
            repair_owner: SecretBoundaryRepairOwnerClass::ServiceOperator,
            repair_path: "Reissue the sync-plane credential or complete the browser/device-code return path before mutating managed sync state.".to_owned(),
            local_safe_behavior:
                "Local history, offline packets, and offboarding exports stay available while managed sync auth is repaired."
                    .to_owned(),
        },
    ];

    let consumer_projections = vec![
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::DocsHelp,
            surface_ref: M5_SECRET_BOUNDARY_DEPTH_DOC_REF.to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            notes: "Docs and help quote the same matrix row ids and credential-mode vocabulary instead of restating connected status.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::Diagnostics,
            surface_ref: "crates/aureline-shell/src/secret_broker_beta/mod.rs".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            notes: "Diagnostics project the row id, acting identity, and repair owner before showing downstream failure details.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::SupportExport,
            surface_ref: "fixtures/security/m5/m5-secret-boundary-depth/support_export.json"
                .to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            notes: "Support export preserves matrix ids, default modes, export posture, and repair owner while excluding raw secret bodies and raw handle ids.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::ReleasePublicTruth,
            surface_ref: "artifacts/security/m5/m5-secret-boundary-depth.md".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            notes: "Release/public-truth surfaces publish only the checked matrix id, row ids, and summary vocabulary; they never widen a row with ad hoc prose.".to_owned(),
        },
    ];

    let mut packet = M5SecretBoundaryDepthPacket {
        record_kind: M5_SECRET_BOUNDARY_DEPTH_RECORD_KIND.to_owned(),
        schema_version: M5_SECRET_BOUNDARY_DEPTH_SCHEMA_VERSION,
        packet_id,
        matrix_id,
        label: "M5 Secret Boundary Depth Matrix".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
        shared_contract_ref: M5_SECRET_BOUNDARY_DEPTH_SHARED_CONTRACT_REF.to_owned(),
        schema_ref: M5_SECRET_BOUNDARY_DEPTH_SCHEMA_REF.to_owned(),
        doc_ref: M5_SECRET_BOUNDARY_DEPTH_DOC_REF.to_owned(),
        fixture_dir: M5_SECRET_BOUNDARY_DEPTH_FIXTURE_DIR.to_owned(),
        source_contract_refs: vec![
            "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json".to_owned(),
            "artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json".to_owned(),
            "crates/aureline-provider/src/account_scope/mod.rs".to_owned(),
            "crates/aureline-provider/src/route_resolution/mod.rs".to_owned(),
            "crates/aureline-review/src/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/mod.rs".to_owned(),
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs".to_owned(),
        ],
        surface_rows,
        consumer_projections,
        summary: SecretBoundarySummary {
            surface_count: 0,
            domain_tokens_present: Vec::new(),
            default_credential_mode_tokens_present: Vec::new(),
            consumer_surface_tokens_present: Vec::new(),
            raw_secret_values_excluded: false,
            raw_handle_ids_excluded: false,
        },
    };
    packet.summary = packet.recompute_summary();
    packet
}

#[cfg(test)]
mod tests;
