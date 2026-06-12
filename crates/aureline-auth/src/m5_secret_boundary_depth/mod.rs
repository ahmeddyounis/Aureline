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

/// Concrete consumer identity that used a credential handle, delegated grant,
/// or remote-vault projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryConsumerIdentityClass {
    /// A local request, task, or workflow initiated the secret use directly.
    LocalWorkflow,
    /// A remote helper or helper-backed runtime consumed the projected
    /// authority.
    RemoteHelper,
    /// A registry client consumed the projected authority.
    RegistryClient,
    /// A database connector or live database session consumed the projected
    /// authority.
    DatabaseConnector,
    /// A preview publisher or remote-preview route consumer used the projected
    /// authority.
    PreviewPublisher,
    /// A cluster or infrastructure connector consumed the projected authority.
    ClusterConnector,
    /// A companion handoff or companion follow surface consumed the projected
    /// authority.
    CompanionHandoff,
    /// A service-issued delegate consumed the authority on the user's behalf.
    ServiceIssuedDelegate,
}

impl SecretBoundaryConsumerIdentityClass {
    /// Every required consumer identity in canonical order.
    pub const ALL: [Self; 8] = [
        Self::LocalWorkflow,
        Self::RemoteHelper,
        Self::RegistryClient,
        Self::DatabaseConnector,
        Self::PreviewPublisher,
        Self::ClusterConnector,
        Self::CompanionHandoff,
        Self::ServiceIssuedDelegate,
    ];

    /// Stable token recorded in the packet and support/export projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkflow => "local_workflow",
            Self::RemoteHelper => "remote_helper",
            Self::RegistryClient => "registry_client",
            Self::DatabaseConnector => "database_connector",
            Self::PreviewPublisher => "preview_publisher",
            Self::ClusterConnector => "cluster_connector",
            Self::CompanionHandoff => "companion_handoff",
            Self::ServiceIssuedDelegate => "service_issued_delegate",
        }
    }
}

/// Bounded control that stops projection or delegated use without deleting
/// unrelated local state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryProjectionControlClass {
    /// Pause forwarding of a local credential into a remote boundary.
    PauseForwarding,
    /// Stop using the current secret or handle for downstream actions.
    StopUsingSecret,
    /// Drop the delegated identity while preserving local continuity.
    DropDelegatedIdentity,
}

impl SecretBoundaryProjectionControlClass {
    /// Every required projection control in canonical order.
    pub const ALL: [Self; 3] = [
        Self::PauseForwarding,
        Self::StopUsingSecret,
        Self::DropDelegatedIdentity,
    ];

    /// Stable token recorded in the packet and support/export projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PauseForwarding => "pause_forwarding",
            Self::StopUsingSecret => "stop_using_secret",
            Self::DropDelegatedIdentity => "drop_delegated_identity",
        }
    }
}

/// Handle-safe outcome recorded by consumer receipts and projection-mode audit
/// rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryUseAuditResultClass {
    /// The consumer used the projected authority successfully.
    Used,
    /// Forwarding is intentionally paused.
    ForwardingPaused,
    /// Secret use was intentionally stopped.
    SecretUseStopped,
    /// Delegated identity use was intentionally dropped.
    DelegatedIdentityDropped,
    /// The authority expired before use could continue.
    Expired,
    /// The authority was revoked before use could continue.
    Revoked,
    /// Policy blocked the attempted use.
    PolicyBlocked,
    /// The backing store, vault, or trust path was unavailable.
    Unavailable,
    /// No usable source was configured for the attempted use.
    NotConfigured,
}

impl SecretBoundaryUseAuditResultClass {
    /// Stable token recorded in consumer receipts and support/export
    /// projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Used => "used",
            Self::ForwardingPaused => "forwarding_paused",
            Self::SecretUseStopped => "secret_use_stopped",
            Self::DelegatedIdentityDropped => "delegated_identity_dropped",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Stable secret-class vocabulary reused by prompts, credential rows, pickers,
/// delegated rows, and export-safe summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundarySecretClass {
    /// AI-provider or model-routing token.
    AiProviderToken,
    /// Code-host, package-registry, or release-publish token.
    CodeHostOrRegistryToken,
    /// Database credential or warehouse session.
    DatabaseCredential,
    /// SSH material or client-certificate binding.
    SshOrClientCertMaterial,
    /// Cloud, remote, or service-issued delegated identity.
    CloudDelegatedIdentity,
    /// Session-scoped secret material entered for one operation.
    SessionScopedSecretInput,
}

impl SecretBoundarySecretClass {
    /// Stable token recorded in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiProviderToken => "ai_provider_token",
            Self::CodeHostOrRegistryToken => "code_host_or_registry_token",
            Self::DatabaseCredential => "database_credential",
            Self::SshOrClientCertMaterial => "ssh_or_client_cert_material",
            Self::CloudDelegatedIdentity => "cloud_delegated_identity",
            Self::SessionScopedSecretInput => "session_scoped_secret_input",
        }
    }
}

/// Health or expiry posture surfaced on a credential-state row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryHealthStateClass {
    /// Credential is healthy and current.
    Healthy,
    /// Credential is healthy but within a renew/rotate horizon.
    ExpiringSoon,
    /// Credential expired and must be renewed.
    Expired,
    /// Credential or handle was revoked.
    Revoked,
    /// Store, trust root, or source is unavailable.
    Unavailable,
    /// Policy or scope now blocks the credential.
    PolicyBlocked,
    /// A forwarded local credential was intentionally paused.
    ForwardingPaused,
    /// A remote vault or remote-vault-backed authority could not be reached.
    RemoteVaultUnavailable,
    /// No usable credential or handle is currently configured.
    Missing,
    /// No usable source exists.
    NotConfigured,
}

impl SecretBoundaryHealthStateClass {
    /// Stable token recorded in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::ExpiringSoon => "expiring_soon",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
            Self::ForwardingPaused => "forwarding_paused",
            Self::RemoteVaultUnavailable => "remote_vault_unavailable",
            Self::Missing => "missing",
            Self::NotConfigured => "not_configured",
        }
    }
}

/// Maps a credential-health posture onto the handle-safe audit result used by
/// receipts and projection-mode audit rows.
pub const fn secret_boundary_use_audit_result_for_health(
    health_state: SecretBoundaryHealthStateClass,
) -> SecretBoundaryUseAuditResultClass {
    match health_state {
        SecretBoundaryHealthStateClass::Healthy
        | SecretBoundaryHealthStateClass::ExpiringSoon => SecretBoundaryUseAuditResultClass::Used,
        SecretBoundaryHealthStateClass::Expired => SecretBoundaryUseAuditResultClass::Expired,
        SecretBoundaryHealthStateClass::Revoked => SecretBoundaryUseAuditResultClass::Revoked,
        SecretBoundaryHealthStateClass::Unavailable
        | SecretBoundaryHealthStateClass::RemoteVaultUnavailable
        | SecretBoundaryHealthStateClass::Missing => SecretBoundaryUseAuditResultClass::Unavailable,
        SecretBoundaryHealthStateClass::PolicyBlocked => {
            SecretBoundaryUseAuditResultClass::PolicyBlocked
        }
        SecretBoundaryHealthStateClass::ForwardingPaused => {
            SecretBoundaryUseAuditResultClass::ForwardingPaused
        }
        SecretBoundaryHealthStateClass::NotConfigured => {
            SecretBoundaryUseAuditResultClass::NotConfigured
        }
    }
}

/// Deployment profile that must preserve the same credential-state semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryDeploymentProfileClass {
    /// Local desktop execution.
    LocalDesktop,
    /// SSH, container, or remote-helper execution.
    SshOrContainer,
    /// Managed workspace or managed service execution.
    ManagedWorkspace,
    /// Mirror-only or offline continuity execution.
    MirrorOffline,
}

impl SecretBoundaryDeploymentProfileClass {
    /// Every required deployment profile in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalDesktop,
        Self::SshOrContainer,
        Self::ManagedWorkspace,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDesktop => "local_desktop",
            Self::SshOrContainer => "ssh_or_container",
            Self::ManagedWorkspace => "managed_workspace",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Projection-parity class a surface exposes across deployment profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryProjectionParityClass {
    /// A local broker handle or OS-store reference is used.
    LocalHandle,
    /// A local credential is forwarded into a remote runtime.
    ForwardedLocalCredential,
    /// A remote vault is fetched on demand.
    RemoteVaultFetch,
    /// A visibly degraded session-only secret is used.
    SessionOnlySecret,
    /// A delegated identity or service-issued authority is used.
    DelegatedIdentity,
    /// No usable credential path is configured.
    Missing,
}

impl SecretBoundaryProjectionParityClass {
    /// Stable token recorded in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHandle => "local_handle",
            Self::ForwardedLocalCredential => "forwarded_local_credential",
            Self::RemoteVaultFetch => "remote_vault_fetch",
            Self::SessionOnlySecret => "session_only_secret",
            Self::DelegatedIdentity => "delegated_identity",
            Self::Missing => "missing",
        }
    }
}

/// Per-profile parity state carried by the matrix and projected by consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryProfileParityRow {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Deployment profile covered by this row.
    pub deployment_profile: SecretBoundaryDeploymentProfileClass,
    /// Projection parity the profile uses.
    pub projection_parity: SecretBoundaryProjectionParityClass,
    /// Current state named for the profile.
    pub health_state: SecretBoundaryHealthStateClass,
    /// Storage class used by the profile.
    pub storage_class: SecretBoundaryStorageClass,
    /// Acting identity class used by the profile.
    pub acting_identity: SecretBoundaryActingIdentityClass,
    /// Bounded next action shown to the user.
    pub next_action_label: String,
    /// Export-safe note describing what still works in this profile.
    pub local_safe_behavior: String,
}

/// Delegated-credential posture that must remain visible across local, remote,
/// vault, and service-issued flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryDelegatedUseClass {
    /// Consumer uses a local broker handle only.
    LocalSecretHandle,
    /// Consumer forwards a local credential into a remote runtime.
    ForwardedLocalCredential,
    /// Consumer fetches the credential from a remote vault on demand.
    RemoteVaultFetch,
    /// Consumer acts through a service-issued delegated identity.
    ServiceIssuedDelegatedIdentity,
}

impl SecretBoundaryDelegatedUseClass {
    /// Stable token recorded in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSecretHandle => "local_secret_handle",
            Self::ForwardedLocalCredential => "forwarded_local_credential",
            Self::RemoteVaultFetch => "remote_vault_fetch",
            Self::ServiceIssuedDelegatedIdentity => "service_issued_delegated_identity",
        }
    }
}

/// One workflow that depends on a credential-bearing surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryWorkflowDependency {
    /// Stable workflow ref safe for export.
    pub workflow_ref: String,
    /// User-facing workflow label safe for export.
    pub workflow_label: String,
}

/// Decline-path truth shown on prompts and rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryDeclinePath {
    /// Reviewable decline label.
    pub decline_label: String,
    /// Export-safe summary of what remains available after decline.
    pub still_works_summary: String,
}

/// Typed control shown on delegated, forwarded, remote-vault, and companion
/// rows so users can stop projection without deleting unrelated local state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryProjectionControl {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Control class.
    pub control_class: SecretBoundaryProjectionControlClass,
    /// Reviewable user action label.
    pub action_label: String,
    /// Export-safe note describing what remains local after the control runs.
    pub preserved_local_state: String,
}

impl SecretBoundaryProjectionControl {
    /// Builds one standard projection control bound to the shared vocabulary.
    pub fn new(
        matrix_row_id: impl Into<String>,
        control_class: SecretBoundaryProjectionControlClass,
        action_label: impl Into<String>,
        preserved_local_state: impl Into<String>,
    ) -> Self {
        Self {
            matrix_row_id: matrix_row_id.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            control_class,
            action_label: action_label.into(),
            preserved_local_state: preserved_local_state.into(),
        }
    }
}

/// Shared secret-access prompt shown before a credential-bearing action uses
/// a handle, delegated identity, browser handoff, or vault fetch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySecretAccessPrompt {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Requesting actor label.
    pub requester_label: String,
    /// Credential or identity class requested.
    pub secret_class: SecretBoundarySecretClass,
    /// Target workflow label.
    pub target_workflow_label: String,
    /// Storage class the prompt will use.
    pub storage_class: SecretBoundaryStorageClass,
    /// Credential mode the prompt will use.
    pub credential_mode: SecretBoundaryCredentialMode,
    /// Projection mode the prompt will use.
    pub projection_mode: SecretBoundaryProjectionMode,
    /// Reviewable lifetime label.
    pub lifetime_label: String,
    /// Optional expiry timestamp or duration label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Workflows that depend on this approval.
    pub dependent_workflows: Vec<SecretBoundaryWorkflowDependency>,
    /// Decline-path truth.
    pub decline_path: SecretBoundaryDeclinePath,
}

/// Shared credential-state row projected by M5 surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryCredentialStateRow {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Reviewable row label.
    pub display_label: String,
    /// Credential or identity class in use.
    pub secret_class: SecretBoundarySecretClass,
    /// Source class shown to the user.
    pub source_class: SecretBoundaryCredentialMode,
    /// Target boundary label.
    pub target_boundary_label: String,
    /// Storage class shown to the user.
    pub storage_class: SecretBoundaryStorageClass,
    /// Projection mode shown to the user.
    pub projection_mode: SecretBoundaryProjectionMode,
    /// Health or expiry posture.
    pub health_state: SecretBoundaryHealthStateClass,
    /// Optional expiry timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Rotate affordance label.
    pub rotate_action_label: String,
    /// Revoke affordance label.
    pub revoke_action_label: String,
    /// Test or validate affordance label.
    pub test_action_label: String,
    /// Workflows that depend on this credential state.
    pub dependent_workflows: Vec<SecretBoundaryWorkflowDependency>,
    /// Export-safe decline/local-safe continuity note.
    pub decline_path: SecretBoundaryDeclinePath,
}

/// One selectable vault, keychain, session-only, or delegated source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryVaultPickerOption {
    /// Stable option id.
    pub option_id: String,
    /// Reviewable option label.
    pub option_label: String,
    /// Source class offered by the picker.
    pub source_class: SecretBoundaryCredentialMode,
    /// Backing storage class.
    pub storage_class: SecretBoundaryStorageClass,
    /// Access-scope label.
    pub access_scope_label: String,
    /// Reveal-policy label.
    pub reveal_policy_label: String,
    /// Portability/export note.
    pub portability_note: String,
    /// Source-of-truth/open detail affordance.
    pub open_source_of_truth_action_label: String,
    /// Whether this option is currently selectable.
    pub selectable: bool,
}

/// Shared vault/keychain picker state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryVaultPickerState {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Picker title.
    pub picker_label: String,
    /// Available picker options.
    pub options: Vec<SecretBoundaryVaultPickerOption>,
}

/// Shared delegated-credential row projected by remote, provider, and managed
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryDelegatedCredentialRow {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Delegated-use class.
    pub delegated_use_class: SecretBoundaryDelegatedUseClass,
    /// Target host or workspace label.
    pub target_host_or_workspace_label: String,
    /// Optional expiry timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Policy or operator owner label.
    pub policy_owner_label: String,
    /// Bounded controls that pause forwarding, stop secret use, or drop
    /// delegated identity without deleting unrelated local state.
    pub projection_controls: Vec<SecretBoundaryProjectionControl>,
}

/// Receipt that records which actor and which consumer used a handle-safe
/// authority boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryConsumerIdentityReceipt {
    /// Stable receipt id safe for export.
    pub receipt_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Actor class Aureline acted as.
    pub actor_identity: SecretBoundaryActingIdentityClass,
    /// Consumer identity that used the authority.
    pub consumer_identity: SecretBoundaryConsumerIdentityClass,
    /// Issuer, broker, or operator label safe for export.
    pub issuer_label: String,
    /// Target boundary label safe for export.
    pub target_boundary_label: String,
    /// Credential mode in use.
    pub credential_mode: SecretBoundaryCredentialMode,
    /// Projection mode in use.
    pub projection_mode: SecretBoundaryProjectionMode,
    /// Storage class in use.
    pub storage_class: SecretBoundaryStorageClass,
    /// Handle-safe result of the use.
    pub result: SecretBoundaryUseAuditResultClass,
}

impl SecretBoundaryConsumerIdentityReceipt {
    /// Builds one consumer-identity receipt bound to the shared vocabulary.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        receipt_id: impl Into<String>,
        matrix_row_id: impl Into<String>,
        actor_identity: SecretBoundaryActingIdentityClass,
        consumer_identity: SecretBoundaryConsumerIdentityClass,
        issuer_label: impl Into<String>,
        target_boundary_label: impl Into<String>,
        credential_mode: SecretBoundaryCredentialMode,
        projection_mode: SecretBoundaryProjectionMode,
        storage_class: SecretBoundaryStorageClass,
        result: SecretBoundaryUseAuditResultClass,
    ) -> Self {
        Self {
            receipt_id: receipt_id.into(),
            matrix_row_id: matrix_row_id.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            actor_identity,
            consumer_identity,
            issuer_label: issuer_label.into(),
            target_boundary_label: target_boundary_label.into(),
            credential_mode,
            projection_mode,
            storage_class,
            result,
        }
    }
}

/// Audit row that records the active projection mode, consumer identity, and
/// available bounded stop/pause/drop controls for one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryProjectionModeAudit {
    /// Stable audit row id safe for export.
    pub audit_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Actor class Aureline acted as.
    pub actor_identity: SecretBoundaryActingIdentityClass,
    /// Consumer identity that used the authority.
    pub consumer_identity: SecretBoundaryConsumerIdentityClass,
    /// Issuer, broker, or operator label safe for export.
    pub issuer_label: String,
    /// Target boundary label safe for export.
    pub target_boundary_label: String,
    /// Projection mode currently in effect.
    pub projection_mode: SecretBoundaryProjectionMode,
    /// Handle-safe result of the current or most recent projection.
    pub result: SecretBoundaryUseAuditResultClass,
    /// Repair owner responsible when the projection cannot continue.
    pub repair_owner: SecretBoundaryRepairOwnerClass,
    /// Bounded controls available on this projection.
    pub available_controls: Vec<SecretBoundaryProjectionControlClass>,
}

impl SecretBoundaryProjectionModeAudit {
    /// Builds one projection-mode audit row bound to the shared vocabulary.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        audit_id: impl Into<String>,
        matrix_row_id: impl Into<String>,
        actor_identity: SecretBoundaryActingIdentityClass,
        consumer_identity: SecretBoundaryConsumerIdentityClass,
        issuer_label: impl Into<String>,
        target_boundary_label: impl Into<String>,
        projection_mode: SecretBoundaryProjectionMode,
        result: SecretBoundaryUseAuditResultClass,
        repair_owner: SecretBoundaryRepairOwnerClass,
        available_controls: Vec<SecretBoundaryProjectionControlClass>,
    ) -> Self {
        Self {
            audit_id: audit_id.into(),
            matrix_row_id: matrix_row_id.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            actor_identity,
            consumer_identity,
            issuer_label: issuer_label.into(),
            target_boundary_label: target_boundary_label.into(),
            projection_mode,
            result,
            repair_owner,
            available_controls,
        }
    }
}

/// Export-safety banner shared by M5 credential-bearing surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryExportSafetyBanner {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Banner summary text.
    pub summary: String,
    /// Export classes from which raw values are excluded.
    pub excluded_exports: Vec<String>,
    /// `false` when raw secret material is excluded.
    pub raw_secret_values_included: bool,
}

/// Shared secret-boundary state bundle projected by the first consuming M5
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySurfaceState {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Shared secret-access prompt.
    pub secret_access_prompt: SecretBoundarySecretAccessPrompt,
    /// Shared credential-state row.
    pub credential_state_row: SecretBoundaryCredentialStateRow,
    /// Optional shared picker state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vault_picker: Option<SecretBoundaryVaultPickerState>,
    /// Optional delegated-credential row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegated_credential_row: Option<SecretBoundaryDelegatedCredentialRow>,
    /// Consumer-identity receipt for the active or most recent handle-safe use.
    pub consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt,
    /// Projection-mode audit for the active or most recent handle-safe use.
    pub projection_mode_audit: SecretBoundaryProjectionModeAudit,
    /// Per-profile parity rows reused by product and diagnostics.
    pub profile_parity_rows: Vec<SecretBoundaryProfileParityRow>,
    /// Shared export-safety banner.
    pub export_safety_banner: SecretBoundaryExportSafetyBanner,
}

impl SecretBoundaryExportSafetyBanner {
    /// Returns the default export-safety banner shared by M5 rows.
    pub fn standard(
        matrix_row_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            matrix_row_id: matrix_row_id.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            summary: summary.into(),
            excluded_exports: vec![
                "profiles".to_owned(),
                "support bundles".to_owned(),
                "handoff packets".to_owned(),
                "recipes".to_owned(),
                "portable workspace exports".to_owned(),
            ],
            raw_secret_values_included: false,
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
    /// Concrete consumers that may use the projected authority.
    pub consumer_identities: Vec<SecretBoundaryConsumerIdentityClass>,
    /// Trust-store dependencies the surface must disclose.
    pub trust_store_dependencies: Vec<SecretBoundaryTrustStoreDependencyClass>,
    /// Bounded stop/pause/drop controls the surface must preserve.
    pub projection_control_classes: Vec<SecretBoundaryProjectionControlClass>,
    /// Deployment-profile parity rows for the surface.
    pub profile_parity_rows: Vec<SecretBoundaryProfileParityRow>,
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
    /// Projection-mode tokens present in the matrix.
    pub projection_mode_tokens_present: Vec<String>,
    /// Consumer surface tokens present in the packet.
    pub consumer_surface_tokens_present: Vec<String>,
    /// Concrete consumer-identity tokens present in the matrix.
    pub consumer_identity_tokens_present: Vec<String>,
    /// Projection-control tokens present in the matrix.
    pub projection_control_tokens_present: Vec<String>,
    /// Deployment profile tokens present in the packet.
    pub deployment_profile_tokens_present: Vec<String>,
    /// Projection-parity tokens present in the packet.
    pub projection_parity_tokens_present: Vec<String>,
    /// Health-state tokens present in the packet.
    pub health_state_tokens_present: Vec<String>,
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
        let mut projection_mode_tokens: BTreeSet<String> = BTreeSet::new();
        let mut consumer_tokens: BTreeSet<String> = BTreeSet::new();
        let mut consumer_identity_tokens: BTreeSet<String> = BTreeSet::new();
        let mut projection_control_tokens: BTreeSet<String> = BTreeSet::new();
        let mut profile_tokens: BTreeSet<String> = BTreeSet::new();
        let mut parity_tokens: BTreeSet<String> = BTreeSet::new();
        let mut health_tokens: BTreeSet<String> = BTreeSet::new();

        for row in &self.surface_rows {
            domain_tokens.insert(row.domain.as_str().to_owned());
            mode_tokens.insert(row.default_credential_mode.as_str().to_owned());
            for projection_mode in &row.projection_modes {
                projection_mode_tokens.insert(projection_mode.as_str().to_owned());
            }
            for consumer_identity in &row.consumer_identities {
                consumer_identity_tokens.insert(consumer_identity.as_str().to_owned());
            }
            for control in &row.projection_control_classes {
                projection_control_tokens.insert(control.as_str().to_owned());
            }
            for profile_row in &row.profile_parity_rows {
                profile_tokens.insert(profile_row.deployment_profile.as_str().to_owned());
                parity_tokens.insert(profile_row.projection_parity.as_str().to_owned());
                health_tokens.insert(profile_row.health_state.as_str().to_owned());
            }
        }
        for projection in &self.consumer_projections {
            consumer_tokens.insert(projection.surface.as_str().to_owned());
        }

        SecretBoundarySummary {
            surface_count: self.surface_rows.len(),
            domain_tokens_present: domain_tokens.into_iter().collect(),
            default_credential_mode_tokens_present: mode_tokens.into_iter().collect(),
            projection_mode_tokens_present: projection_mode_tokens.into_iter().collect(),
            consumer_surface_tokens_present: consumer_tokens.into_iter().collect(),
            consumer_identity_tokens_present: consumer_identity_tokens.into_iter().collect(),
            projection_control_tokens_present: projection_control_tokens.into_iter().collect(),
            deployment_profile_tokens_present: profile_tokens.into_iter().collect(),
            projection_parity_tokens_present: parity_tokens.into_iter().collect(),
            health_state_tokens_present: health_tokens.into_iter().collect(),
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
            if row.consumer_identities.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingConsumerIdentities(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.trust_store_dependencies.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingTrustDependencies(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.projection_control_classes.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingProjectionControls(
                    row.matrix_row_id.clone(),
                ));
            }
            if row.profile_parity_rows.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingProfileParity(
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
            if row
                .acting_identities
                .contains(&SecretBoundaryActingIdentityClass::ForwardedLocalCredential)
                && !row
                    .projection_control_classes
                    .contains(&SecretBoundaryProjectionControlClass::PauseForwarding)
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::ForwardedIdentityMissingPauseControl(
                        row.matrix_row_id.clone(),
                    ),
                );
            }
            if row.acting_identities.iter().any(|identity| {
                matches!(
                    identity,
                    SecretBoundaryActingIdentityClass::DelegatedCredential
                        | SecretBoundaryActingIdentityClass::ServiceIssuedAuthority
                )
            }) && !row.projection_control_classes.iter().any(|control| {
                matches!(
                    control,
                    SecretBoundaryProjectionControlClass::StopUsingSecret
                        | SecretBoundaryProjectionControlClass::DropDelegatedIdentity
                )
            }) {
                violations.push(
                    M5SecretBoundaryDepthViolation::DelegatedIdentityMissingStopOrDropControl(
                        row.matrix_row_id.clone(),
                    ),
                );
            }

            let mut seen_profiles = BTreeSet::new();
            for profile_row in &row.profile_parity_rows {
                seen_profiles.insert(profile_row.deployment_profile);
                if profile_row.next_action_label.trim().is_empty()
                    || profile_row.local_safe_behavior.trim().is_empty()
                {
                    violations.push(
                        M5SecretBoundaryDepthViolation::IncompleteProfileParity(
                            row.matrix_row_id.clone(),
                            profile_row.deployment_profile,
                        ),
                    );
                }
                if profile_row.matrix_row_id != row.matrix_row_id {
                    violations.push(M5SecretBoundaryDepthViolation::ProfileParityRowIdMismatch(
                        row.matrix_row_id.clone(),
                        profile_row.deployment_profile,
                    ));
                }
                if profile_row.projection_parity == SecretBoundaryProjectionParityClass::Missing
                    && profile_row.health_state != SecretBoundaryHealthStateClass::Missing
                {
                    violations.push(M5SecretBoundaryDepthViolation::MissingParityStateDrift(
                        row.matrix_row_id.clone(),
                        profile_row.deployment_profile,
                    ));
                }
                if profile_row.health_state == SecretBoundaryHealthStateClass::ForwardingPaused
                    && profile_row.projection_parity
                        != SecretBoundaryProjectionParityClass::ForwardedLocalCredential
                {
                    violations.push(
                        M5SecretBoundaryDepthViolation::ForwardingPausedParityDrift(
                            row.matrix_row_id.clone(),
                            profile_row.deployment_profile,
                        ),
                    );
                }
                if profile_row.health_state
                    == SecretBoundaryHealthStateClass::RemoteVaultUnavailable
                    && profile_row.projection_parity
                        != SecretBoundaryProjectionParityClass::RemoteVaultFetch
                {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RemoteVaultUnavailableParityDrift(
                            row.matrix_row_id.clone(),
                            profile_row.deployment_profile,
                        ),
                    );
                }
            }
            for profile in SecretBoundaryDeploymentProfileClass::ALL {
                if !seen_profiles.contains(&profile) {
                    violations.push(M5SecretBoundaryDepthViolation::MissingProfileCoverage(
                        row.matrix_row_id.clone(),
                        profile,
                    ));
                }
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

        for identity in SecretBoundaryConsumerIdentityClass::ALL {
            if !self
                .summary
                .consumer_identity_tokens_present
                .iter()
                .any(|token| token == identity.as_str())
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::MissingRequiredConsumerIdentity(identity),
                );
            }
        }

        for control in SecretBoundaryProjectionControlClass::ALL {
            if !self
                .summary
                .projection_control_tokens_present
                .iter()
                .any(|token| token == control.as_str())
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::MissingRequiredProjectionControl(control),
                );
            }
        }

        if self.summary != self.recompute_summary() {
            violations.push(M5SecretBoundaryDepthViolation::SummaryMismatch);
        }

        for required_state in [
            SecretBoundaryHealthStateClass::Missing,
            SecretBoundaryHealthStateClass::Expired,
            SecretBoundaryHealthStateClass::PolicyBlocked,
            SecretBoundaryHealthStateClass::ForwardingPaused,
            SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
        ] {
            if !self
                .summary
                .health_state_tokens_present
                .iter()
                .any(|token| token == required_state.as_str())
            {
                violations.push(M5SecretBoundaryDepthViolation::MissingRequiredHealthState(
                    required_state,
                ));
            }
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
    /// Projection-mode tokens preserved for lineage/audit review.
    pub projection_mode_tokens: Vec<String>,
    /// Consumer-identity tokens preserved for lineage/audit review.
    pub consumer_identity_tokens: Vec<String>,
    /// Projection-control tokens preserved for lineage/audit review.
    pub projection_control_tokens: Vec<String>,
    /// Export posture token.
    pub export_posture_token: String,
    /// Repair owner token.
    pub repair_owner_token: String,
    /// Per-profile parity rows preserved for diagnostics/support.
    pub profile_parity_rows: Vec<SecretBoundaryProfileParityRow>,
}

impl SecretBoundarySupportExportRow {
    /// Builds one support-export row from the canonical surface row.
    pub fn from_surface_row(row: &SecretBoundarySurfaceRow) -> Self {
        Self {
            matrix_row_id: row.matrix_row_id.clone(),
            domain_token: row.domain.as_str().to_owned(),
            default_credential_mode_token: row.default_credential_mode.as_str().to_owned(),
            projection_mode_tokens: row
                .projection_modes
                .iter()
                .map(|mode| mode.as_str().to_owned())
                .collect(),
            consumer_identity_tokens: row
                .consumer_identities
                .iter()
                .map(|identity| identity.as_str().to_owned())
                .collect(),
            projection_control_tokens: row
                .projection_control_classes
                .iter()
                .map(|control| control.as_str().to_owned())
                .collect(),
            export_posture_token: row.export_posture.as_str().to_owned(),
            repair_owner_token: row.repair_owner.as_str().to_owned(),
            profile_parity_rows: row.profile_parity_rows.clone(),
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
    /// A row omitted every consumer identity.
    MissingConsumerIdentities(String),
    /// A row omitted every trust-store dependency.
    MissingTrustDependencies(String),
    /// A row omitted every projection control class.
    MissingProjectionControls(String),
    /// A row omitted per-profile parity.
    MissingProfileParity(String),
    /// A row omitted one required deployment profile.
    MissingProfileCoverage(String, SecretBoundaryDeploymentProfileClass),
    /// A per-profile row omitted a bounded next action or continuity note.
    IncompleteProfileParity(String, SecretBoundaryDeploymentProfileClass),
    /// A per-profile row did not preserve the parent matrix row id.
    ProfileParityRowIdMismatch(String, SecretBoundaryDeploymentProfileClass),
    /// A `missing` parity row drifted from the `missing` health state.
    MissingParityStateDrift(String, SecretBoundaryDeploymentProfileClass),
    /// A `forwarding_paused` state was not paired with forwarded-local parity.
    ForwardingPausedParityDrift(String, SecretBoundaryDeploymentProfileClass),
    /// A `remote_vault_unavailable` state was not paired with remote-vault parity.
    RemoteVaultUnavailableParityDrift(String, SecretBoundaryDeploymentProfileClass),
    /// A row omitted the repair path or local-safe continuity note.
    MissingRepairOrContinuityNote(String),
    /// A `not_configured` row omitted a local-safe metadata path.
    NotConfiguredWithoutLocalSafeDisclosure(String),
    /// A forwarded credential path omitted the pause-forwarding control.
    ForwardedIdentityMissingPauseControl(String),
    /// A delegated or service-issued path omitted a stop/drop control.
    DelegatedIdentityMissingStopOrDropControl(String),
    /// One required domain had no coverage.
    MissingDomainCoverage(SecretBoundarySurfaceDomain),
    /// One required consumer projection was absent.
    MissingConsumerProjection(SecretBoundaryConsumerSurface),
    /// A consumer projection drifted from the matrix id or vocabulary ref.
    ConsumerProjectionDrift(SecretBoundaryConsumerSurface),
    /// A required consumer-identity token was not represented anywhere.
    MissingRequiredConsumerIdentity(SecretBoundaryConsumerIdentityClass),
    /// A required projection-control token was not represented anywhere.
    MissingRequiredProjectionControl(SecretBoundaryProjectionControlClass),
    /// A required health-state token was not represented anywhere in the packet.
    MissingRequiredHealthState(SecretBoundaryHealthStateClass),
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
            Self::MissingConsumerIdentities(row) => {
                write!(f, "row {row} must declare at least one consumer identity")
            }
            Self::MissingTrustDependencies(row) => {
                write!(
                    f,
                    "row {row} must declare at least one trust-store dependency"
                )
            }
            Self::MissingProjectionControls(row) => {
                write!(f, "row {row} must declare at least one projection control")
            }
            Self::MissingProfileParity(row) => {
                write!(f, "row {row} must declare per-profile parity rows")
            }
            Self::MissingProfileCoverage(row, profile) => write!(
                f,
                "row {row} is missing deployment-profile coverage for {}",
                profile.as_str()
            ),
            Self::IncompleteProfileParity(row, profile) => write!(
                f,
                "row {row} profile {} must declare a bounded next action and continuity note",
                profile.as_str()
            ),
            Self::ProfileParityRowIdMismatch(row, profile) => write!(
                f,
                "row {row} profile {} drifted from the parent matrix_row_id",
                profile.as_str()
            ),
            Self::MissingParityStateDrift(row, profile) => write!(
                f,
                "row {row} profile {} uses missing parity without missing state",
                profile.as_str()
            ),
            Self::ForwardingPausedParityDrift(row, profile) => write!(
                f,
                "row {row} profile {} uses forwarding_paused without forwarded-local parity",
                profile.as_str()
            ),
            Self::RemoteVaultUnavailableParityDrift(row, profile) => write!(
                f,
                "row {row} profile {} uses remote_vault_unavailable without remote-vault parity",
                profile.as_str()
            ),
            Self::MissingRepairOrContinuityNote(row) => write!(
                f,
                "row {row} must declare both repair_path and local_safe_behavior"
            ),
            Self::NotConfiguredWithoutLocalSafeDisclosure(row) => write!(
                f,
                "row {row} uses not_configured without a local-safe metadata disclosure"
            ),
            Self::ForwardedIdentityMissingPauseControl(row) => write!(
                f,
                "row {row} forwards local credentials without a pause_forwarding control"
            ),
            Self::DelegatedIdentityMissingStopOrDropControl(row) => write!(
                f,
                "row {row} uses delegated/service-issued identity without stop/drop control"
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
            Self::MissingRequiredConsumerIdentity(identity) => write!(
                f,
                "packet must cover the {} consumer identity",
                identity.as_str()
            ),
            Self::MissingRequiredProjectionControl(control) => write!(
                f,
                "packet must cover the {} projection control",
                control.as_str()
            ),
            Self::MissingRequiredHealthState(state) => write!(
                f,
                "packet must cover the {} health state",
                state.as_str()
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

/// Returns the canonical per-profile parity rows for one matrix row id.
pub fn seeded_secret_boundary_profile_parity_rows(
    matrix_row_id: &str,
) -> Vec<SecretBoundaryProfileParityRow> {
    seeded_m5_secret_boundary_depth_packet()
        .surface_rows
        .into_iter()
        .find(|row| row.matrix_row_id == matrix_row_id)
        .map(|row| row.profile_parity_rows)
        .unwrap_or_default()
}

/// Returns the canonical surface row for one matrix row id.
pub fn seeded_secret_boundary_surface_row(
    matrix_row_id: &str,
) -> Option<SecretBoundarySurfaceRow> {
    seeded_m5_secret_boundary_depth_packet()
        .surface_rows
        .into_iter()
        .find(|row| row.matrix_row_id == matrix_row_id)
}

fn profile_parity_row(
    matrix_row_id: &str,
    deployment_profile: SecretBoundaryDeploymentProfileClass,
    projection_parity: SecretBoundaryProjectionParityClass,
    health_state: SecretBoundaryHealthStateClass,
    storage_class: SecretBoundaryStorageClass,
    acting_identity: SecretBoundaryActingIdentityClass,
    next_action_label: &str,
    local_safe_behavior: &str,
) -> SecretBoundaryProfileParityRow {
    SecretBoundaryProfileParityRow {
        matrix_row_id: matrix_row_id.to_owned(),
        deployment_profile,
        projection_parity,
        health_state,
        storage_class,
        acting_identity,
        next_action_label: next_action_label.to_owned(),
        local_safe_behavior: local_safe_behavior.to_owned(),
    }
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::LocalWorkflow],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            projection_control_classes: vec![SecretBoundaryProjectionControlClass::StopUsingSecret],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.request_workspace.send_http",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Test local request handle",
                    "Request editing, effective-request review, and metadata-only history stay available locally.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.send_http",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded request credential",
                    "Forwarding pauses without forcing paste-a-token fallback; request files and metadata-only replay remain available.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.send_http",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Renew managed request session",
                    "Managed send keeps the delegated boundary explicit while local request authoring continues.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.send_http",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open metadata-only request history",
                    "Mirror/offline mode keeps request review, diff, and metadata exports available without implying a live credential path.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::LocalWorkflow],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            projection_control_classes: vec![SecretBoundaryProjectionControlClass::StopUsingSecret],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.request_workspace.history_replay",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Retest replay handle",
                    "Local replay keeps history, diff, and request evidence review available.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.history_replay",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Re-enter session-only replay secret",
                    "Replay review stays available when a session-only credential expires in a remote shell or container.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.history_replay",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Refresh managed replay approval",
                    "Managed replay keeps delegated authority separate from the saved request history.",
                ),
                profile_parity_row(
                    "m5.secret.request_workspace.history_replay",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Use one-time replay secret",
                    "Mirror/offline replay can use a bounded session-only secret without persisting raw auth into the history packet.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::DatabaseConnector],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.database.connection_picker",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Test local database handle",
                    "Schema inspection, statement review, and imported-result browsing stay available locally.",
                ),
                profile_parity_row(
                    "m5.secret.database.connection_picker",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded database credential",
                    "Remote/container sessions pause forwarding explicitly instead of asking for a raw password paste.",
                ),
                profile_parity_row(
                    "m5.secret.database.connection_picker",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::RemoteVaultFetch,
                    SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Rebind managed vault route",
                    "Managed database sessions narrow to inspect-only when the remote vault is unavailable.",
                ),
                profile_parity_row(
                    "m5.secret.database.connection_picker",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open imported result or schema snapshot",
                    "Mirror/offline continuity keeps query review and imported snapshots available without claiming a reconnectable credential.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::DatabaseConnector],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.database.query_history_portability",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Validate local query-history alias",
                    "Local query-history review and portability diff remain available.",
                ),
                profile_parity_row(
                    "m5.secret.database.query_history_portability",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Refresh session-only replay secret",
                    "A session-only replay secret can expire without losing the redacted history packet.",
                ),
                profile_parity_row(
                    "m5.secret.database.query_history_portability",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::RemoteVaultFetch,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Inspect managed vault lineage",
                    "Managed portability keeps remote-vault lineage explicit and still excludes raw values from exports.",
                ),
                profile_parity_row(
                    "m5.secret.database.query_history_portability",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Replay with one-time session input",
                    "Mirror/offline replay can accept bounded session input while preserving the same redaction and export rules.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::ServiceIssuedDelegate],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.provider_model.route_resolution",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::HumanAccount,
                    "Retest local provider route",
                    "Local provider metadata and draft queues remain available when live routing is closed.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.route_resolution",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded provider credential",
                    "Remote helper routing pauses forwarded credentials explicitly and keeps publish-later fallback visible.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.route_resolution",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Reissue delegated provider grant",
                    "Managed routes keep delegated authority explicit rather than flattening to a generic connected badge.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.route_resolution",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Continue with cached provider metadata",
                    "Mirror/offline provider review stays metadata-only until a live route is re-established.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::ServiceIssuedDelegate],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::HumanAccount,
                    "Inspect provider scope lineage",
                    "Local scope inspection and draft fallback remain available without widening authority.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded scope credential",
                    "Forwarded scope credentials pause explicitly when the remote boundary changes.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::RemoteVaultFetch,
                    SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Repair remote-vault scope lineage",
                    "Managed scope review stays available even when the remote vault path is unavailable.",
                ),
                profile_parity_row(
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Enter a one-time recovery scope",
                    "Offline scope repair can use a bounded session-only secret while keeping durable local drafts separate.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::RegistryClient],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.registry.package_auth",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Test local registry handle",
                    "Dependency review, lockfile diff, and local resolution stay available on desktop.",
                ),
                profile_parity_row(
                    "m5.secret.registry.package_auth",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded registry credential",
                    "Remote package actions keep the forwarded boundary explicit instead of asking for a pasted token.",
                ),
                profile_parity_row(
                    "m5.secret.registry.package_auth",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::PolicyBlocked,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Review managed publish policy",
                    "Managed registry routes can be blocked by policy while dependency inspection remains available.",
                ),
                profile_parity_row(
                    "m5.secret.registry.package_auth",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Use offline cache or mirror review",
                    "Mirror/offline package review stays honest about cache-only state and never implies live publish authority.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::PreviewPublisher],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Refresh desktop preview session",
                    "Local preview keeps exact desktop handoff and route review available.",
                ),
                profile_parity_row(
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded preview credential",
                    "Remote preview pauses forwarded credentials explicitly while preserving route history and revocation details.",
                ),
                profile_parity_row(
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Renew delegated preview session",
                    "Managed preview expiry narrows to metadata-only route history and exact desktop handoff instructions.",
                ),
                profile_parity_row(
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open prior route evidence",
                    "Mirror/offline mode keeps exported route evidence and revocation history available without a live preview path.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::ClusterConnector],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::KnownHosts,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Retest local connector handle",
                    "Manifest inspection, trust review, and safe handoff stay available on desktop.",
                ),
                profile_parity_row(
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded target credential",
                    "Forwarding pauses explicitly across SSH/container boundaries while target manifests remain inspectable.",
                ),
                profile_parity_row(
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::RemoteVaultFetch,
                    SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Repair target vault trust",
                    "Managed target contexts keep trust and repair lineage explicit when the remote vault path is unavailable.",
                ),
                profile_parity_row(
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open cached target manifest",
                    "Mirror/offline target review stays local-safe and never implies a reachable remote control plane.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::CompanionHandoff],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.companion.session_handoff",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Refresh companion handoff session",
                    "Read-only follow state and exact desktop handoff stay available on the local device.",
                ),
                profile_parity_row(
                    "m5.secret.companion.session_handoff",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded companion credential",
                    "Companion handoff keeps remote/local identity boundaries visible when forwarding pauses.",
                ),
                profile_parity_row(
                    "m5.secret.companion.session_handoff",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Renew companion delegated session",
                    "Managed companion handoff expiry keeps read-only follow state and handoff descriptors visible.",
                ),
                profile_parity_row(
                    "m5.secret.companion.session_handoff",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open saved handoff descriptor",
                    "Offline companion review stays bounded to saved descriptors and never implies a live relay.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::RemoteHelper],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::KnownHosts,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
                SecretBoundaryTrustStoreDependencyClass::VaultRef,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.managed.workspace_runtime",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open local-safe continuation",
                    "Local editing remains available when the managed runtime is absent.",
                ),
                profile_parity_row(
                    "m5.secret.managed.workspace_runtime",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded runtime credential",
                    "Remote helper execution pauses forwarding explicitly instead of falling back to an unmanaged token copy.",
                ),
                profile_parity_row(
                    "m5.secret.managed.workspace_runtime",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::RemoteVaultFetch,
                    SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Repair managed runtime vault lineage",
                    "Managed runtime actions narrow to local-safe continuation when the remote vault or host proof path is unavailable.",
                ),
                profile_parity_row(
                    "m5.secret.managed.workspace_runtime",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::SessionOnlySecret,
                    SecretBoundaryHealthStateClass::Expired,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Recreate managed runtime from mirror evidence",
                    "Mirror/offline mode keeps the last validated local mirror and honest recreate guidance rather than implying a resumable managed credential.",
                ),
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
            consumer_identities: vec![SecretBoundaryConsumerIdentityClass::ServiceIssuedDelegate],
            trust_store_dependencies: vec![
                SecretBoundaryTrustStoreDependencyClass::OsStore,
                SecretBoundaryTrustStoreDependencyClass::PinnedControlPlane,
                SecretBoundaryTrustStoreDependencyClass::OrgCaBundle,
            ],
            projection_control_classes: vec![
                SecretBoundaryProjectionControlClass::PauseForwarding,
                SecretBoundaryProjectionControlClass::StopUsingSecret,
                SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            ],
            profile_parity_rows: vec![
                profile_parity_row(
                    "m5.secret.managed.sync_plane",
                    SecretBoundaryDeploymentProfileClass::LocalDesktop,
                    SecretBoundaryProjectionParityClass::LocalHandle,
                    SecretBoundaryHealthStateClass::Healthy,
                    SecretBoundaryStorageClass::OsStore,
                    SecretBoundaryActingIdentityClass::HumanAccount,
                    "Inspect local sync posture",
                    "Local history, offline packets, and offboarding exports stay available on desktop.",
                ),
                profile_parity_row(
                    "m5.secret.managed.sync_plane",
                    SecretBoundaryDeploymentProfileClass::SshOrContainer,
                    SecretBoundaryProjectionParityClass::ForwardedLocalCredential,
                    SecretBoundaryHealthStateClass::ForwardingPaused,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ForwardedLocalCredential,
                    "Resume forwarded sync credential",
                    "Forwarded sync credentials pause explicitly across remote helpers while local history remains intact.",
                ),
                profile_parity_row(
                    "m5.secret.managed.sync_plane",
                    SecretBoundaryDeploymentProfileClass::ManagedWorkspace,
                    SecretBoundaryProjectionParityClass::DelegatedIdentity,
                    SecretBoundaryHealthStateClass::PolicyBlocked,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Review managed sync policy",
                    "Managed sync can be blocked by policy while local offboarding and export paths remain available.",
                ),
                profile_parity_row(
                    "m5.secret.managed.sync_plane",
                    SecretBoundaryDeploymentProfileClass::MirrorOffline,
                    SecretBoundaryProjectionParityClass::Missing,
                    SecretBoundaryHealthStateClass::Missing,
                    SecretBoundaryStorageClass::NotConfigured,
                    SecretBoundaryActingIdentityClass::LocalOnlyHandle,
                    "Open offline sync packet",
                    "Mirror/offline continuity keeps local history and redacted offboarding packets available without a live sync-plane credential.",
                ),
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
            projection_mode_tokens_present: Vec::new(),
            consumer_surface_tokens_present: Vec::new(),
            consumer_identity_tokens_present: Vec::new(),
            projection_control_tokens_present: Vec::new(),
            deployment_profile_tokens_present: Vec::new(),
            projection_parity_tokens_present: Vec::new(),
            health_state_tokens_present: Vec::new(),
            raw_secret_values_excluded: false,
            raw_handle_ids_excluded: false,
        },
    };
    packet.summary = packet.recompute_summary();
    packet
}

#[cfg(test)]
mod tests;
