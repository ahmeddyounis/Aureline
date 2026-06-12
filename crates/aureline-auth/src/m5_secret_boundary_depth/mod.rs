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

/// Typed trust, certificate, SSH, or renewal change that blocks a
/// credential-bearing surface until a bounded repair runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryRepairableChangeClass {
    /// The current route no longer chains to a trusted CA.
    CaUntrusted,
    /// The organization CA bundle is stale or revoked for the current route.
    BundleStale,
    /// The pinned control-plane trust root changed unexpectedly.
    PinMismatch,
    /// Trust or credential rotation is required before the route may continue.
    RotationRequired,
    /// The delegated scope, grant, or handle was revoked and must be rebound.
    CredentialRevoked,
    /// The SSH host proof is unknown and requires explicit review.
    SshHostKeyUnknown,
    /// The SSH host proof changed from the last-known-good fingerprint.
    SshHostKeyMismatch,
    /// The route requires a client-certificate binding that is absent.
    ClientCertificateRequired,
    /// The bound client certificate expired and must be renewed.
    ClientCertificateExpired,
    /// A system-browser return path or callback correlation failed.
    BrowserHandoffReturnLost,
    /// A device-code-backed or browser/device-code renewal window elapsed.
    DeviceCodeRenewalRequired,
}

impl SecretBoundaryRepairableChangeClass {
    /// Every required change class in canonical order.
    pub const ALL: [Self; 11] = [
        Self::CaUntrusted,
        Self::BundleStale,
        Self::PinMismatch,
        Self::RotationRequired,
        Self::CredentialRevoked,
        Self::SshHostKeyUnknown,
        Self::SshHostKeyMismatch,
        Self::ClientCertificateRequired,
        Self::ClientCertificateExpired,
        Self::BrowserHandoffReturnLost,
        Self::DeviceCodeRenewalRequired,
    ];

    /// Stable token recorded in packets and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaUntrusted => "ca_untrusted",
            Self::BundleStale => "bundle_stale",
            Self::PinMismatch => "pin_mismatch",
            Self::RotationRequired => "rotation_required",
            Self::CredentialRevoked => "credential_revoked",
            Self::SshHostKeyUnknown => "ssh_host_key_unknown",
            Self::SshHostKeyMismatch => "ssh_host_key_mismatch",
            Self::ClientCertificateRequired => "client_certificate_required",
            Self::ClientCertificateExpired => "client_certificate_expired",
            Self::BrowserHandoffReturnLost => "browser_handoff_return_lost",
            Self::DeviceCodeRenewalRequired => "device_code_renewal_required",
        }
    }
}

/// Closed vocabulary naming the boundary object whose prior-good posture or
/// current drift the repair state is describing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryLastKnownGoodClass {
    /// Platform trust-store descriptor previously admitted the route.
    OsTrustStoreDescriptor,
    /// Organization CA bundle epoch previously admitted the route.
    OrgCaBundleEpoch,
    /// Pinned root set previously admitted the route.
    PinnedControlPlaneRoot,
    /// SSH host proof or fingerprint previously admitted the route.
    SshHostProof,
    /// Client-certificate binding and fingerprint previously admitted the route.
    ClientCertificateBinding,
    /// Device-code session or refresh window previously admitted the route.
    DeviceCodeSession,
    /// Browser-handoff return packet/callback pairing previously admitted the route.
    BrowserHandoffSession,
    /// Remote-vault lineage previously admitted the route.
    RemoteVaultLineage,
    /// Delegated scope or installation grant previously admitted the route.
    DelegatedScopeBinding,
}

impl SecretBoundaryLastKnownGoodClass {
    /// Stable token recorded in packets and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsTrustStoreDescriptor => "os_trust_store_descriptor",
            Self::OrgCaBundleEpoch => "org_ca_bundle_epoch",
            Self::PinnedControlPlaneRoot => "pinned_control_plane_root",
            Self::SshHostProof => "ssh_host_proof",
            Self::ClientCertificateBinding => "client_certificate_binding",
            Self::DeviceCodeSession => "device_code_session",
            Self::BrowserHandoffSession => "browser_handoff_session",
            Self::RemoteVaultLineage => "remote_vault_lineage",
            Self::DelegatedScopeBinding => "delegated_scope_binding",
        }
    }
}

/// Project Doctor probe family that owns the exported finding code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryDoctorProbeFamilyClass {
    /// Network, proxy, CA, certificate, and transport explainability.
    NetworkProxyCaTransport,
    /// Trust, identity, and approval/renewal explainability.
    TrustIdentityPolicy,
    /// Remote, route, and collaboration explainability.
    RemoteRoutesAndCollaboration,
}

impl SecretBoundaryDoctorProbeFamilyClass {
    /// Stable token recorded in packets and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NetworkProxyCaTransport => "network_proxy_ca_transport",
            Self::TrustIdentityPolicy => "trust_identity_policy",
            Self::RemoteRoutesAndCollaboration => "remote_routes_and_collaboration",
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

/// Artifact family that must preserve handle-first metadata while excluding raw
/// secret material in export, import, replay, or restore flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryArtifactFamilyClass {
    /// Portable profile exports and profile handoff packets.
    Profiles,
    /// Workflow-bundle manifests and review/install handoffs.
    WorkflowBundles,
    /// Portable-state packages and restore previews.
    PortableStatePackages,
    /// Recipe exports, shared recipe packs, and replay packets.
    Recipes,
    /// Support bundles and support-export packets.
    SupportBundles,
    /// AI evidence packets, rerun reviews, and support-linked AI exports.
    AiEvidencePackets,
    /// Incident exports and incident-workspace handoff packets.
    IncidentExports,
    /// Offboarding exports and profile-roaming deletion packets.
    OffboardingExports,
}

impl SecretBoundaryArtifactFamilyClass {
    /// Every required artifact family in canonical order.
    pub const ALL: [Self; 8] = [
        Self::Profiles,
        Self::WorkflowBundles,
        Self::PortableStatePackages,
        Self::Recipes,
        Self::SupportBundles,
        Self::AiEvidencePackets,
        Self::IncidentExports,
        Self::OffboardingExports,
    ];

    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Profiles => "profiles",
            Self::WorkflowBundles => "workflow_bundles",
            Self::PortableStatePackages => "portable_state_packages",
            Self::Recipes => "recipes",
            Self::SupportBundles => "support_bundles",
            Self::AiEvidencePackets => "ai_evidence_packets",
            Self::IncidentExports => "incident_exports",
            Self::OffboardingExports => "offboarding_exports",
        }
    }
}

/// Redaction-default material class that a portable export rule must omit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryOmittedMaterialClass {
    /// Raw bearer tokens, API tokens, or literal password values.
    RawTokens,
    /// Private keys, raw SSH keys, and equivalent signing private material.
    PrivateKeys,
    /// Refresh tokens or session-refresh secrets.
    RefreshTokens,
    /// Ambient delegated, forwarded, or service-issued credentials.
    AmbientDelegatedCredentials,
    /// Raw credential-handle ids or vault object ids not safe for export.
    RawHandleIds,
}

impl SecretBoundaryOmittedMaterialClass {
    /// Every required omitted material class.
    pub const ALL: [Self; 5] = [
        Self::RawTokens,
        Self::PrivateKeys,
        Self::RefreshTokens,
        Self::AmbientDelegatedCredentials,
        Self::RawHandleIds,
    ];

    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawTokens => "raw_tokens",
            Self::PrivateKeys => "private_keys",
            Self::RefreshTokens => "refresh_tokens",
            Self::AmbientDelegatedCredentials => "ambient_delegated_credentials",
            Self::RawHandleIds => "raw_handle_ids",
        }
    }
}

/// Artifact-family export rule proving what survives export and how replay or
/// import repairs the missing secret boundary safely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryArtifactExportRule {
    /// Artifact family covered by this rule.
    pub artifact_family: SecretBoundaryArtifactFamilyClass,
    /// Repo-relative ref for the first implementing surface.
    pub source_ref: String,
    /// Export posture the family publishes.
    pub export_posture: SecretBoundaryExportPostureClass,
    /// `true` when credential aliases survive export.
    pub preserves_credential_aliases: bool,
    /// `true` when handle classes survive export.
    pub preserves_handle_classes: bool,
    /// `true` when source labels survive export.
    pub preserves_source_labels: bool,
    /// `true` when a consumer identity or acting-identity class survives export.
    pub preserves_consumer_identity: bool,
    /// Reviewable omission marker shown when raw material is removed.
    pub omission_marker_label: String,
    /// Reviewable export-safety banner shown before export or replay.
    pub omission_banner: String,
    /// Material classes that never cross the default export boundary.
    pub omitted_material_classes: Vec<SecretBoundaryOmittedMaterialClass>,
    /// `true` when import must stop for a typed rebind.
    pub requires_typed_rebind_on_import: bool,
    /// `true` when replay, restore, or rerun must stop for a typed rebind.
    pub requires_typed_rebind_on_replay: bool,
    /// Bounded action the user takes to rebind safely.
    pub rebind_action_label: String,
    /// Export-safe explanation of what failed and why raw material was not reused.
    pub rebind_failure_label: String,
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

/// Typed, repairable blocker caused by trust, certificate, SSH, or
/// browser/device-code renewal drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryRepairableState {
    /// Stable repair-state id safe for export.
    pub repair_state_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// The specific change or failure surfaced to the user.
    pub change_class: SecretBoundaryRepairableChangeClass,
    /// The health state that activates this repair state.
    pub triggering_health_state: SecretBoundaryHealthStateClass,
    /// Exact target ref affected by the drift or renewal failure.
    pub affected_target_ref: String,
    /// Reviewable target label safe for export.
    pub affected_target_label: String,
    /// The last-known-good class that previously admitted the route.
    pub last_known_good_class: SecretBoundaryLastKnownGoodClass,
    /// Export-safe last-known-good summary.
    pub last_known_good_summary: String,
    /// Workflows blocked by this exact state.
    pub blocked_workflows: Vec<SecretBoundaryWorkflowDependency>,
    /// Minimally destructive next action label.
    pub next_action_label: String,
    /// Owner responsible for the repair path.
    pub repair_owner: SecretBoundaryRepairOwnerClass,
    /// Project Doctor probe family that owns the finding.
    pub doctor_probe_family: SecretBoundaryDoctorProbeFamilyClass,
    /// Stable Project Doctor finding code.
    pub doctor_finding_code: String,
    /// Stable repair candidate id.
    pub repair_candidate_id: String,
    /// Stable support-bundle lineage ref.
    pub support_bundle_lineage_ref: String,
}

/// Consumer surface that must project the matrix id and shared vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryConsumerSurface {
    /// Docs or in-product help.
    DocsHelp,
    /// Help/About truth surfaces.
    HelpAbout,
    /// Admin documentation or operator-facing truth surfaces.
    AdminDocs,
    /// Diagnostics or inspector surfaces.
    Diagnostics,
    /// Support-export surfaces.
    SupportExport,
    /// Release or public-truth surfaces.
    ReleasePublicTruth,
}

impl SecretBoundaryConsumerSurface {
    /// Every required consumer surface in canonical order.
    pub const ALL: [Self; 6] = [
        Self::DocsHelp,
        Self::HelpAbout,
        Self::AdminDocs,
        Self::Diagnostics,
        Self::SupportExport,
        Self::ReleasePublicTruth,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelp => "docs_help",
            Self::HelpAbout => "help_about",
            Self::AdminDocs => "admin_docs",
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
        SecretBoundaryHealthStateClass::Healthy | SecretBoundaryHealthStateClass::ExpiringSoon => {
            SecretBoundaryUseAuditResultClass::Used
        }
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
    /// Canonical repairable states for this surface.
    pub repairable_states: Vec<SecretBoundaryRepairableState>,
    /// Exact active repairable state when the current health posture is blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_repair_state: Option<SecretBoundaryRepairableState>,
    /// Per-profile parity rows reused by product and diagnostics.
    pub profile_parity_rows: Vec<SecretBoundaryProfileParityRow>,
    /// Shared export-safety banner.
    pub export_safety_banner: SecretBoundaryExportSafetyBanner,
}

/// Closed lineage-event vocabulary for credential rotation, revoke, rebind,
/// and policy-denied projection rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryLineageEventClass {
    /// The surface needs a credential or lease rotation before work can resume.
    RotationRequired,
    /// The current handle, grant, or delegated authority was revoked.
    CredentialRevoked,
    /// The surface needs a bounded rebind or trust repair.
    RebindRequired,
    /// Policy denied the attempted projection before downstream send/run/publish.
    PolicyDeniedProjection,
    /// Forwarding was intentionally paused at the current boundary.
    ForwardingPaused,
}

impl SecretBoundaryLineageEventClass {
    /// Stable token recorded on derived lineage projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RotationRequired => "rotation_required",
            Self::CredentialRevoked => "credential_revoked",
            Self::RebindRequired => "rebind_required",
            Self::PolicyDeniedProjection => "policy_denied_projection",
            Self::ForwardingPaused => "forwarding_paused",
        }
    }
}

/// Failure dimension surfaced before a credential-bearing action may proceed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryFailureDimensionClass {
    /// Trust, CA, SSH, or certificate verification failed.
    Trust,
    /// Policy or approval scope denied the projection.
    Policy,
    /// The configured credential or delegated scope needs repair.
    Credential,
    /// Network or remote-vault availability blocked the projection.
    Network,
    /// The boundary was paused intentionally and must be resumed explicitly.
    RuntimeHealth,
}

impl SecretBoundaryFailureDimensionClass {
    /// Stable token recorded on derived lineage projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trust => "trust",
            Self::Policy => "policy",
            Self::Credential => "credential",
            Self::Network => "network",
            Self::RuntimeHealth => "runtime_health",
        }
    }
}

/// Severity class used by the derived durable-activity projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryActivitySeverityClass {
    /// Informational continuity item.
    Info,
    /// Warning that needs bounded repair before a blocked workflow can continue.
    Warning,
    /// Blocking failure that stopped a credentialed action before send/run/publish.
    Error,
}

impl SecretBoundaryActivitySeverityClass {
    /// Stable token recorded on derived durable-activity projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

/// Export-safe credential-lineage event derived from one surface state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryLineageEvent {
    /// Stable lineage-event id.
    pub event_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Closed event class.
    pub event_class: SecretBoundaryLineageEventClass,
    /// Failed dimension surfaced before work may continue.
    pub failure_dimension: SecretBoundaryFailureDimensionClass,
    /// Current health posture tied to this event.
    pub health_state: SecretBoundaryHealthStateClass,
    /// Acting identity class bound to the event.
    pub actor_identity: SecretBoundaryActingIdentityClass,
    /// Consumer identity bound to the event.
    pub consumer_identity: SecretBoundaryConsumerIdentityClass,
    /// Reviewable target boundary label.
    pub target_boundary_label: String,
    /// Workflows impacted by the event.
    pub impacted_workflows: Vec<SecretBoundaryWorkflowDependency>,
    /// Narrowest safe next action.
    pub next_safe_action_label: String,
    /// Export-safe continuity note.
    pub local_safe_behavior: String,
    /// Stable reopen target safe for workflow history and durable activity.
    pub durable_reopen_target_ref: String,
    /// Optional Project Doctor finding code.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doctor_finding_code: Option<String>,
    /// Optional repair candidate id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_candidate_id: Option<String>,
    /// Optional support-bundle lineage ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_bundle_lineage_ref: Option<String>,
    /// Redaction-safe event summary.
    pub export_safe_summary: String,
}

/// Workflow-history row derived from one credential-lineage event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryWorkflowHistoryRow {
    /// Stable workflow-history row id.
    pub row_id: String,
    /// Source lineage-event id.
    pub event_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Workflow ref blocked or impacted by the event.
    pub workflow_ref: String,
    /// Workflow label blocked or impacted by the event.
    pub workflow_label: String,
    /// Event class carried into workflow history.
    pub event_class: SecretBoundaryLineageEventClass,
    /// Health posture carried into workflow history.
    pub health_state: SecretBoundaryHealthStateClass,
    /// Narrowest safe next action.
    pub next_safe_action_label: String,
    /// Stable reopen target safe for the workflow-history surface.
    pub durable_reopen_target_ref: String,
    /// Redaction-safe workflow-history summary.
    pub export_safe_summary: String,
}

/// Durable-activity row derived from one credential-lineage event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryActivityRow {
    /// Stable durable row id.
    pub row_id: String,
    /// Stable durable job id.
    pub durable_job_id: String,
    /// Source lineage-event id.
    pub event_id: String,
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Severity surfaced to durable-activity consumers.
    pub severity: SecretBoundaryActivitySeverityClass,
    /// Reviewable activity summary.
    pub summary_label: String,
    /// Stable reopen target safe for the activity center.
    pub durable_reopen_target_ref: String,
    /// Primary repair or reopen action label.
    pub primary_action_label: String,
    /// Redaction-safe activity summary.
    pub export_safe_summary: String,
}

/// Derived credential-lineage bundle that joins workflow history, durable
/// activity, and support export to one secret-boundary state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryLineageBundle {
    /// Shared matrix row id.
    pub matrix_row_id: String,
    /// Shared vocabulary ref.
    pub vocabulary_ref: String,
    /// Export-safe lineage events.
    pub events: Vec<SecretBoundaryLineageEvent>,
    /// Workflow-history rows derived from the events.
    pub workflow_history_rows: Vec<SecretBoundaryWorkflowHistoryRow>,
    /// Durable-activity rows derived from the events.
    pub activity_rows: Vec<SecretBoundaryActivityRow>,
}

impl SecretBoundaryExportSafetyBanner {
    /// Returns the default export-safety banner shared by M5 rows.
    pub fn standard(matrix_row_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            matrix_row_id: matrix_row_id.into(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            summary: summary.into(),
            excluded_exports: vec![
                "profiles".to_owned(),
                "workflow bundles".to_owned(),
                "support bundles".to_owned(),
                "recipes".to_owned(),
                "portable-state packages".to_owned(),
                "ai evidence packets".to_owned(),
                "incident exports".to_owned(),
                "offboarding exports".to_owned(),
            ],
            raw_secret_values_included: false,
        }
    }
}

impl SecretBoundarySurfaceState {
    /// Derives the export-safe lineage bundle for this surface state.
    pub fn lineage_bundle(&self) -> SecretBoundaryLineageBundle {
        let events = derived_lineage_events_from_surface_state(self);
        SecretBoundaryLineageBundle::from_events(
            self.matrix_row_id.clone(),
            self.vocabulary_ref.clone(),
            events,
        )
    }
}

impl SecretBoundaryLineageBundle {
    /// Builds a lineage bundle from explicit lineage events.
    pub fn from_events(
        matrix_row_id: impl Into<String>,
        vocabulary_ref: impl Into<String>,
        events: Vec<SecretBoundaryLineageEvent>,
    ) -> Self {
        let matrix_row_id = matrix_row_id.into();
        let workflow_history_rows = events
            .iter()
            .flat_map(|event| {
                event.impacted_workflows.iter().map(move |workflow| {
                    SecretBoundaryWorkflowHistoryRow {
                        row_id: format!(
                            "workflow_history:{}:{}",
                            event.event_id, workflow.workflow_ref
                        ),
                        event_id: event.event_id.clone(),
                        matrix_row_id: event.matrix_row_id.clone(),
                        workflow_ref: workflow.workflow_ref.clone(),
                        workflow_label: workflow.workflow_label.clone(),
                        event_class: event.event_class,
                        health_state: event.health_state,
                        next_safe_action_label: event.next_safe_action_label.clone(),
                        durable_reopen_target_ref: event.durable_reopen_target_ref.clone(),
                        export_safe_summary: format!(
                            "{} is blocked by {}. {}",
                            workflow.workflow_label,
                            event.target_boundary_label,
                            event.local_safe_behavior
                        ),
                    }
                })
            })
            .collect();
        let activity_rows = events
            .iter()
            .map(|event| SecretBoundaryActivityRow {
                row_id: format!("activity_row:{}", event.event_id),
                durable_job_id: format!("activity_job:{}", event.event_id),
                event_id: event.event_id.clone(),
                matrix_row_id: event.matrix_row_id.clone(),
                severity: activity_severity_for_health(event.health_state),
                summary_label: event.export_safe_summary.clone(),
                durable_reopen_target_ref: event.durable_reopen_target_ref.clone(),
                primary_action_label: event.next_safe_action_label.clone(),
                export_safe_summary: event.export_safe_summary.clone(),
            })
            .collect();
        Self {
            matrix_row_id,
            vocabulary_ref: vocabulary_ref.into(),
            events,
            workflow_history_rows,
            activity_rows,
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
    /// Typed trust/certificate/SSH/renewal repair states for this surface.
    pub repairable_states: Vec<SecretBoundaryRepairableState>,
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
    /// Stable evidence-index ref shared across consuming surfaces.
    pub evidence_index_ref: String,
    /// Qualification-row refs the consumer must project verbatim.
    pub qualification_row_refs: Vec<String>,
    /// Count of qualifications currently held at `qualified_current`.
    pub current_qualification_count: usize,
    /// Count of qualifications automatically narrowed below `qualified_current`.
    pub narrowed_qualification_count: usize,
    /// Export-safe description of how the consumer narrows its copy.
    pub notes: String,
}

/// Qualification label shown on secret-boundary truth surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryQualificationLabel {
    /// Current proof and all required secret-boundary coverage are present.
    QualifiedCurrent,
    /// The profile keeps local-safe continuity but not a live credential claim.
    LimitedLocalContinuity,
    /// The row remains visible for support/review only because current proof or
    /// required coverage is incomplete.
    SupportReviewOnly,
}

impl SecretBoundaryQualificationLabel {
    /// Stable token recorded in qualification rows and summaries.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QualifiedCurrent => "qualified_current",
            Self::LimitedLocalContinuity => "limited_local_continuity",
            Self::SupportReviewOnly => "support_review_only",
        }
    }

    /// Returns true when the label represents a current secret-boundary claim.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::QualifiedCurrent)
    }
}

/// Freshness posture of the proof packet behind one row/profile qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryProofFreshnessClass {
    /// A current checked proof packet backs the row/profile qualification.
    Current,
    /// Proof exists but is outside its freshness window.
    Stale,
    /// No checked proof packet exists for the row/profile qualification.
    Missing,
}

impl SecretBoundaryProofFreshnessClass {
    /// Stable token recorded in qualification rows and summaries.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
        }
    }
}

/// Closed reason vocabulary for a qualification row narrowed below
/// `qualified_current`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretBoundaryQualificationNarrowReason {
    /// No narrowing is required.
    NotNarrowed,
    /// This deployment profile keeps only local-safe continuity for the row.
    ProfileLocalContinuityOnly,
    /// No checked proof packet is available.
    ProofPacketMissing,
    /// The checked proof packet is stale.
    ProofPacketStale,
    /// The row lacks current explicit handle-class proof.
    HandleClassesUnproven,
    /// The row lacks current vault/keychain picker proof.
    VaultPickerUnproven,
    /// The row lacks current delegated-identity proof.
    DelegatedIdentityUnproven,
    /// The row lacks current projection-pause proof.
    ProjectionPauseUnproven,
    /// The row lacks current trust-store drift proof.
    TrustStoreDriftUnproven,
    /// The row lacks current rotation/revoke lineage proof.
    RotationRevokeLineageUnproven,
    /// The row lacks current export-redaction proof.
    ExportRedactionUnproven,
    /// The row lacks current repair-flow proof.
    RepairFlowUnproven,
}

impl SecretBoundaryQualificationNarrowReason {
    /// Stable token recorded in qualification rows and summaries.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::ProfileLocalContinuityOnly => "profile_local_continuity_only",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::HandleClassesUnproven => "handle_classes_unproven",
            Self::VaultPickerUnproven => "vault_picker_unproven",
            Self::DelegatedIdentityUnproven => "delegated_identity_unproven",
            Self::ProjectionPauseUnproven => "projection_pause_unproven",
            Self::TrustStoreDriftUnproven => "trust_store_drift_unproven",
            Self::RotationRevokeLineageUnproven => "rotation_revoke_lineage_unproven",
            Self::ExportRedactionUnproven => "export_redaction_unproven",
            Self::RepairFlowUnproven => "repair_flow_unproven",
        }
    }
}

/// Checked proof packet and evidence refs behind one row/profile qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryQualificationProof {
    /// Stable proof id.
    pub proof_id: String,
    /// Repo-relative ref to the checked proof packet or checked support-export
    /// artifact that grounds the qualification.
    pub packet_ref: String,
    /// Repo-relative ref to the proof schema or contract.
    pub schema_ref: String,
    /// Stable evidence-index ref reused by every consumer surface.
    pub proof_index_ref: String,
    /// UTC capture timestamp of the checked proof.
    pub captured_at: String,
    /// Checked evidence refs supporting the claim.
    pub evidence_refs: Vec<String>,
}

/// One row/profile qualification in the canonical secret-boundary evidence
/// index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundaryProfileQualificationRow {
    /// Stable qualification id.
    pub qualification_row_id: String,
    /// Shared matrix row id this qualification binds to.
    pub matrix_row_id: String,
    /// Stable domain token preserved for release/public-truth consumers.
    pub domain: SecretBoundarySurfaceDomain,
    /// Deployment profile the qualification applies to.
    pub deployment_profile: SecretBoundaryDeploymentProfileClass,
    /// Claimed label ceiling for this row/profile.
    pub claimed_label: SecretBoundaryQualificationLabel,
    /// Actual displayed label after proof and parity checks.
    pub displayed_label: SecretBoundaryQualificationLabel,
    /// Current proof freshness posture.
    pub proof_freshness: SecretBoundaryProofFreshnessClass,
    /// Checked proof packet when current or stale proof exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_packet: Option<SecretBoundaryQualificationProof>,
    /// Stable ref to the bound profile-parity row.
    pub profile_parity_ref: String,
    /// `true` when explicit handle-class proof is present.
    pub handle_classes_verified: bool,
    /// `true` when vault/keychain picker or storage-source proof is present.
    pub vault_picker_verified: bool,
    /// `true` when delegated-identity proof is present where required.
    pub delegated_identity_verified: bool,
    /// `true` when pause/stop/drop projection controls are proven.
    pub projection_pause_verified: bool,
    /// `true` when trust-store drift and typed repair proof is present.
    pub trust_store_drift_verified: bool,
    /// `true` when rotation/revoke lineage proof is present.
    pub rotation_revoke_lineage_verified: bool,
    /// `true` when export-redaction proof is present.
    pub export_redaction_verified: bool,
    /// `true` when typed repair-flow proof is present.
    pub repair_flow_verified: bool,
    /// Closed reason vocabulary for any narrowing.
    pub narrow_reason: SecretBoundaryQualificationNarrowReason,
    /// Export-safe rationale shown by help/admin/diagnostics/public-truth
    /// consumers.
    pub rationale: String,
}

/// Summary for the canonical packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SecretBoundarySummary {
    /// Total number of surface rows.
    pub surface_count: usize,
    /// Total number of row/profile qualification rows.
    pub qualification_count: usize,
    /// Number of qualifications held at `qualified_current`.
    pub current_qualification_count: usize,
    /// Number of qualifications narrowed to `limited_local_continuity`.
    pub limited_local_continuity_count: usize,
    /// Number of qualifications narrowed to `support_review_only`.
    pub support_review_only_count: usize,
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
    /// Repairable change tokens present in the packet.
    pub repairable_change_tokens_present: Vec<String>,
    /// Artifact-family tokens present in the packet.
    pub artifact_family_tokens_present: Vec<String>,
    /// Omitted material tokens present in the packet.
    pub omitted_material_tokens_present: Vec<String>,
    /// Qualification-label tokens present in the packet.
    pub qualification_label_tokens_present: Vec<String>,
    /// Proof-freshness tokens present in the packet.
    pub proof_freshness_tokens_present: Vec<String>,
    /// Qualification narrow-reason tokens present in the packet.
    pub qualification_narrow_reason_tokens_present: Vec<String>,
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
    /// Per-row, per-profile qualification rows forming the canonical evidence
    /// index for current secret-boundary claims.
    pub qualification_rows: Vec<SecretBoundaryProfileQualificationRow>,
    /// Required consumer projections.
    pub consumer_projections: Vec<SecretBoundaryConsumerProjection>,
    /// Cross-artifact export rules for secret-bearing families.
    pub artifact_export_rules: Vec<SecretBoundaryArtifactExportRule>,
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
        let mut repairable_change_tokens: BTreeSet<String> = BTreeSet::new();
        let mut artifact_family_tokens: BTreeSet<String> = BTreeSet::new();
        let mut omitted_material_tokens: BTreeSet<String> = BTreeSet::new();
        let mut qualification_label_tokens: BTreeSet<String> = BTreeSet::new();
        let mut proof_freshness_tokens: BTreeSet<String> = BTreeSet::new();
        let mut qualification_narrow_reason_tokens: BTreeSet<String> = BTreeSet::new();
        let mut current_qualification_count = 0usize;
        let mut limited_local_continuity_count = 0usize;
        let mut support_review_only_count = 0usize;

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
            for repairable_state in &row.repairable_states {
                repairable_change_tokens.insert(repairable_state.change_class.as_str().to_owned());
            }
        }
        for projection in &self.consumer_projections {
            consumer_tokens.insert(projection.surface.as_str().to_owned());
        }
        for qualification in &self.qualification_rows {
            profile_tokens.insert(qualification.deployment_profile.as_str().to_owned());
            qualification_label_tokens.insert(qualification.displayed_label.as_str().to_owned());
            proof_freshness_tokens.insert(qualification.proof_freshness.as_str().to_owned());
            qualification_narrow_reason_tokens
                .insert(qualification.narrow_reason.as_str().to_owned());
            match qualification.displayed_label {
                SecretBoundaryQualificationLabel::QualifiedCurrent => {
                    current_qualification_count += 1;
                }
                SecretBoundaryQualificationLabel::LimitedLocalContinuity => {
                    limited_local_continuity_count += 1;
                }
                SecretBoundaryQualificationLabel::SupportReviewOnly => {
                    support_review_only_count += 1;
                }
            }
        }
        for rule in &self.artifact_export_rules {
            artifact_family_tokens.insert(rule.artifact_family.as_str().to_owned());
            for omitted in &rule.omitted_material_classes {
                omitted_material_tokens.insert(omitted.as_str().to_owned());
            }
        }

        SecretBoundarySummary {
            surface_count: self.surface_rows.len(),
            qualification_count: self.qualification_rows.len(),
            current_qualification_count,
            limited_local_continuity_count,
            support_review_only_count,
            domain_tokens_present: domain_tokens.into_iter().collect(),
            default_credential_mode_tokens_present: mode_tokens.into_iter().collect(),
            projection_mode_tokens_present: projection_mode_tokens.into_iter().collect(),
            consumer_surface_tokens_present: consumer_tokens.into_iter().collect(),
            consumer_identity_tokens_present: consumer_identity_tokens.into_iter().collect(),
            projection_control_tokens_present: projection_control_tokens.into_iter().collect(),
            deployment_profile_tokens_present: profile_tokens.into_iter().collect(),
            projection_parity_tokens_present: parity_tokens.into_iter().collect(),
            health_state_tokens_present: health_tokens.into_iter().collect(),
            repairable_change_tokens_present: repairable_change_tokens.into_iter().collect(),
            artifact_family_tokens_present: artifact_family_tokens.into_iter().collect(),
            omitted_material_tokens_present: omitted_material_tokens.into_iter().collect(),
            qualification_label_tokens_present: qualification_label_tokens.into_iter().collect(),
            proof_freshness_tokens_present: proof_freshness_tokens.into_iter().collect(),
            qualification_narrow_reason_tokens_present: qualification_narrow_reason_tokens
                .into_iter()
                .collect(),
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
        let mut seen_artifact_families = BTreeSet::new();
        for rule in &self.artifact_export_rules {
            seen_artifact_families.insert(rule.artifact_family);
            if !rule.preserves_credential_aliases
                || !rule.preserves_handle_classes
                || !rule.preserves_source_labels
                || !rule.preserves_consumer_identity
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::IncompleteArtifactExportRule(
                        rule.artifact_family,
                    ),
                );
            }
            if rule.omission_marker_label.trim().is_empty()
                || rule.omission_banner.trim().is_empty()
                || rule.rebind_action_label.trim().is_empty()
                || rule.rebind_failure_label.trim().is_empty()
                || (!rule.requires_typed_rebind_on_import && !rule.requires_typed_rebind_on_replay)
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::ArtifactRuleMissingBannerOrRebind(
                        rule.artifact_family,
                    ),
                );
            }
            let omitted = rule
                .omitted_material_classes
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();
            for required in SecretBoundaryOmittedMaterialClass::ALL {
                if !omitted.contains(&required) {
                    violations.push(
                        M5SecretBoundaryDepthViolation::ArtifactRuleMissingOmittedMaterial(
                            rule.artifact_family,
                            required,
                        ),
                    );
                }
            }
        }
        for family in SecretBoundaryArtifactFamilyClass::ALL {
            if !seen_artifact_families.contains(&family) {
                violations.push(M5SecretBoundaryDepthViolation::MissingArtifactExportRule(
                    family,
                ));
            }
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
            if row.repairable_states.is_empty() {
                violations.push(M5SecretBoundaryDepthViolation::MissingRepairableStates(
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

            for repairable_state in &row.repairable_states {
                if repairable_state.matrix_row_id != row.matrix_row_id {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RepairableStateRowIdMismatch(
                            row.matrix_row_id.clone(),
                            repairable_state.repair_state_id.clone(),
                        ),
                    );
                }
                if repairable_state.vocabulary_ref != M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RepairableStateVocabularyDrift(
                            repairable_state.repair_state_id.clone(),
                        ),
                    );
                }
                if repairable_state.affected_target_ref.trim().is_empty()
                    || repairable_state.affected_target_label.trim().is_empty()
                    || repairable_state.last_known_good_summary.trim().is_empty()
                    || repairable_state.next_action_label.trim().is_empty()
                    || repairable_state.doctor_finding_code.trim().is_empty()
                    || repairable_state.repair_candidate_id.trim().is_empty()
                    || repairable_state
                        .support_bundle_lineage_ref
                        .trim()
                        .is_empty()
                {
                    violations.push(M5SecretBoundaryDepthViolation::IncompleteRepairableState(
                        repairable_state.repair_state_id.clone(),
                    ));
                }
                if !repairable_state
                    .doctor_finding_code
                    .starts_with("doctor.finding.")
                {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RepairableStateDoctorFindingInvalid(
                            repairable_state.repair_state_id.clone(),
                        ),
                    );
                }
                if repairable_state.repair_owner != row.repair_owner {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RepairableStateOwnerMismatch(
                            repairable_state.repair_state_id.clone(),
                        ),
                    );
                }
                if !matches!(
                    repairable_state.triggering_health_state,
                    SecretBoundaryHealthStateClass::Expired
                        | SecretBoundaryHealthStateClass::Revoked
                        | SecretBoundaryHealthStateClass::Unavailable
                        | SecretBoundaryHealthStateClass::PolicyBlocked
                        | SecretBoundaryHealthStateClass::RemoteVaultUnavailable
                        | SecretBoundaryHealthStateClass::Missing
                ) {
                    violations.push(
                        M5SecretBoundaryDepthViolation::RepairableStateHealthyTrigger(
                            repairable_state.repair_state_id.clone(),
                        ),
                    );
                }
            }

            let mut seen_profiles = BTreeSet::new();
            for profile_row in &row.profile_parity_rows {
                seen_profiles.insert(profile_row.deployment_profile);
                if profile_row.next_action_label.trim().is_empty()
                    || profile_row.local_safe_behavior.trim().is_empty()
                {
                    violations.push(M5SecretBoundaryDepthViolation::IncompleteProfileParity(
                        row.matrix_row_id.clone(),
                        profile_row.deployment_profile,
                    ));
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
                    violations.push(M5SecretBoundaryDepthViolation::ForwardingPausedParityDrift(
                        row.matrix_row_id.clone(),
                        profile_row.deployment_profile,
                    ));
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

        let row_lookup = self
            .surface_rows
            .iter()
            .map(|row| (row.matrix_row_id.as_str(), row))
            .collect::<std::collections::BTreeMap<_, _>>();
        let mut seen_qualification_ids = BTreeSet::new();
        let mut covered_qualifications = BTreeSet::new();
        let expected_qualification_refs = self
            .qualification_rows
            .iter()
            .map(|row| row.qualification_row_id.clone())
            .collect::<BTreeSet<_>>();
        for qualification in &self.qualification_rows {
            if !seen_qualification_ids.insert(qualification.qualification_row_id.clone()) {
                violations.push(M5SecretBoundaryDepthViolation::DuplicateQualificationRowId(
                    qualification.qualification_row_id.clone(),
                ));
            }
            let Some(surface_row) = row_lookup.get(qualification.matrix_row_id.as_str()) else {
                violations.push(
                    M5SecretBoundaryDepthViolation::QualificationUnknownMatrixRow(
                        qualification.qualification_row_id.clone(),
                    ),
                );
                continue;
            };
            if qualification.domain != surface_row.domain {
                violations.push(M5SecretBoundaryDepthViolation::QualificationDomainMismatch(
                    qualification.qualification_row_id.clone(),
                ));
            }
            let Some(profile_row) = surface_row.profile_parity_rows.iter().find(|profile_row| {
                profile_row.deployment_profile == qualification.deployment_profile
            }) else {
                violations.push(
                    M5SecretBoundaryDepthViolation::QualificationUnknownProfileParity(
                        qualification.qualification_row_id.clone(),
                    ),
                );
                continue;
            };
            let expected_profile_parity_ref = format!(
                "{}:{}",
                surface_row.matrix_row_id,
                qualification.deployment_profile.as_str()
            );
            if qualification.profile_parity_ref != expected_profile_parity_ref {
                violations.push(
                    M5SecretBoundaryDepthViolation::QualificationUnknownProfileParity(
                        qualification.qualification_row_id.clone(),
                    ),
                );
            }
            covered_qualifications.insert((
                qualification.matrix_row_id.clone(),
                qualification.deployment_profile,
            ));

            let current_coverage_complete = qualification.handle_classes_verified
                && qualification.vault_picker_verified
                && qualification.delegated_identity_verified
                && qualification.projection_pause_verified
                && qualification.trust_store_drift_verified
                && qualification.rotation_revoke_lineage_verified
                && qualification.export_redaction_verified
                && qualification.repair_flow_verified;
            let local_continuity_only = matches!(
                profile_row.projection_parity,
                SecretBoundaryProjectionParityClass::Missing
            ) || matches!(
                profile_row.health_state,
                SecretBoundaryHealthStateClass::Missing
                    | SecretBoundaryHealthStateClass::NotConfigured
            );

            if qualification.displayed_label.is_current() {
                if qualification.proof_freshness != SecretBoundaryProofFreshnessClass::Current
                    || qualification.qualification_packet.is_none()
                {
                    violations.push(
                        M5SecretBoundaryDepthViolation::CurrentQualificationMissingCurrentProof(
                            qualification.qualification_row_id.clone(),
                        ),
                    );
                }
                if !current_coverage_complete {
                    violations.push(
                        M5SecretBoundaryDepthViolation::CurrentQualificationMissingCoverage(
                            qualification.qualification_row_id.clone(),
                        ),
                    );
                }
                if local_continuity_only {
                    violations.push(
                        M5SecretBoundaryDepthViolation::CurrentQualificationOnMissingProfile(
                            qualification.qualification_row_id.clone(),
                        ),
                    );
                }
            } else if qualification.narrow_reason
                == SecretBoundaryQualificationNarrowReason::NotNarrowed
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::NarrowedQualificationMissingReason(
                        qualification.qualification_row_id.clone(),
                    ),
                );
            }
        }

        for row in &self.surface_rows {
            for profile in SecretBoundaryDeploymentProfileClass::ALL {
                if !covered_qualifications.contains(&(row.matrix_row_id.clone(), profile)) {
                    violations.push(
                        M5SecretBoundaryDepthViolation::MissingQualificationCoverage(
                            row.matrix_row_id.clone(),
                            profile,
                        ),
                    );
                }
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
            if projection.evidence_index_ref.trim().is_empty()
                || projection.current_qualification_count
                    != self.recompute_summary().current_qualification_count
                || projection.narrowed_qualification_count
                    != self
                        .recompute_summary()
                        .qualification_count
                        .saturating_sub(self.recompute_summary().current_qualification_count)
                || projection
                    .qualification_row_refs
                    .iter()
                    .cloned()
                    .collect::<BTreeSet<_>>()
                    != expected_qualification_refs
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::ConsumerProjectionQualificationDrift(
                        projection.surface,
                    ),
                );
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

        for label in [
            SecretBoundaryQualificationLabel::QualifiedCurrent,
            SecretBoundaryQualificationLabel::LimitedLocalContinuity,
            SecretBoundaryQualificationLabel::SupportReviewOnly,
        ] {
            if !self
                .summary
                .qualification_label_tokens_present
                .iter()
                .any(|token| token == label.as_str())
            {
                violations
                    .push(M5SecretBoundaryDepthViolation::MissingRequiredQualificationLabel(label));
            }
        }

        for freshness in [
            SecretBoundaryProofFreshnessClass::Current,
            SecretBoundaryProofFreshnessClass::Missing,
        ] {
            if !self
                .summary
                .proof_freshness_tokens_present
                .iter()
                .any(|token| token == freshness.as_str())
            {
                violations
                    .push(M5SecretBoundaryDepthViolation::MissingRequiredProofFreshness(freshness));
            }
        }

        for reason in [
            SecretBoundaryQualificationNarrowReason::NotNarrowed,
            SecretBoundaryQualificationNarrowReason::ProfileLocalContinuityOnly,
            SecretBoundaryQualificationNarrowReason::ProofPacketMissing,
        ] {
            if !self
                .summary
                .qualification_narrow_reason_tokens_present
                .iter()
                .any(|token| token == reason.as_str())
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::MissingRequiredQualificationNarrowReason(
                        reason,
                    ),
                );
            }
        }

        for family in SecretBoundaryArtifactFamilyClass::ALL {
            if !self
                .summary
                .artifact_family_tokens_present
                .iter()
                .any(|token| token == family.as_str())
            {
                violations
                    .push(M5SecretBoundaryDepthViolation::MissingRequiredArtifactFamily(family));
            }
        }

        for material in SecretBoundaryOmittedMaterialClass::ALL {
            if !self
                .summary
                .omitted_material_tokens_present
                .iter()
                .any(|token| token == material.as_str())
            {
                violations
                    .push(M5SecretBoundaryDepthViolation::MissingRequiredOmittedMaterial(material));
            }
        }

        if self.summary != self.recompute_summary() {
            violations.push(M5SecretBoundaryDepthViolation::SummaryMismatch);
        }

        for required_state in [
            SecretBoundaryHealthStateClass::Missing,
            SecretBoundaryHealthStateClass::Expired,
            SecretBoundaryHealthStateClass::Revoked,
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

        for change_class in SecretBoundaryRepairableChangeClass::ALL {
            if !self
                .summary
                .repairable_change_tokens_present
                .iter()
                .any(|token| token == change_class.as_str())
            {
                violations.push(
                    M5SecretBoundaryDepthViolation::MissingRequiredRepairableChange(change_class),
                );
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
    /// Qualification rows preserved by support export so support/help/release
    /// consumers share the same current secret-boundary result.
    pub qualification_rows: Vec<SecretBoundaryProfileQualificationRow>,
    /// Consumer evidence-index projections preserved by support export.
    pub consumer_projections: Vec<SecretBoundaryConsumerProjection>,
    /// Artifact-family export rules preserved by support export.
    pub artifact_export_rules: Vec<SecretBoundaryArtifactExportRule>,
    /// Project Doctor finding codes preserved by the export.
    pub doctor_finding_codes: Vec<String>,
    /// Credential-lineage events preserved by the export.
    pub lineage_events: Vec<SecretBoundaryLineageEvent>,
    /// Workflow-history rows preserved by the export.
    pub workflow_history_rows: Vec<SecretBoundaryWorkflowHistoryRow>,
    /// Durable-activity rows preserved by the export.
    pub activity_rows: Vec<SecretBoundaryActivityRow>,
    /// `true` when raw secret values are excluded.
    pub raw_secret_values_excluded: bool,
    /// `true` when raw handle ids are excluded.
    pub raw_handle_ids_excluded: bool,
    /// `true` when the export preserves matrix ids.
    pub matrix_ids_preserved: bool,
    /// `true` when Project Doctor finding linkage is preserved.
    pub project_doctor_lineage_preserved: bool,
    /// `true` when support-bundle lineage refs are preserved.
    pub support_bundle_lineage_preserved: bool,
    /// `true` when workflow-history lineage is preserved.
    pub workflow_history_lineage_preserved: bool,
    /// `true` when durable-activity lineage is preserved.
    pub activity_lineage_preserved: bool,
}

impl SecretBoundarySupportExport {
    /// Builds a metadata-only support export from the packet.
    pub fn from_packet(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        packet: &M5SecretBoundaryDepthPacket,
    ) -> Self {
        let doctor_finding_codes: BTreeSet<_> = packet
            .surface_rows
            .iter()
            .flat_map(|row| row.repairable_states.iter())
            .map(|state| state.doctor_finding_code.clone())
            .collect();
        let lineage_bundles: Vec<_> = packet
            .surface_rows
            .iter()
            .map(lineage_bundle_from_surface_row)
            .collect();
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
            qualification_rows: packet.qualification_rows.clone(),
            consumer_projections: packet.consumer_projections.clone(),
            artifact_export_rules: packet.artifact_export_rules.clone(),
            doctor_finding_codes: doctor_finding_codes.into_iter().collect(),
            lineage_events: lineage_bundles
                .iter()
                .flat_map(|bundle| bundle.events.clone())
                .collect(),
            workflow_history_rows: lineage_bundles
                .iter()
                .flat_map(|bundle| bundle.workflow_history_rows.clone())
                .collect(),
            activity_rows: lineage_bundles
                .iter()
                .flat_map(|bundle| bundle.activity_rows.clone())
                .collect(),
            raw_secret_values_excluded: true,
            raw_handle_ids_excluded: true,
            matrix_ids_preserved: true,
            project_doctor_lineage_preserved: true,
            support_bundle_lineage_preserved: true,
            workflow_history_lineage_preserved: true,
            activity_lineage_preserved: true,
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
    /// Typed repairable states preserved for support diagnosis.
    pub repairable_states: Vec<SecretBoundaryRepairableState>,
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
            repairable_states: row.repairable_states.clone(),
            profile_parity_rows: row.profile_parity_rows.clone(),
        }
    }
}

fn lineage_bundle_from_surface_row(row: &SecretBoundarySurfaceRow) -> SecretBoundaryLineageBundle {
    let events = if let Some(repairable_state) = row.repairable_states.first() {
        vec![lineage_event_from_repairable_state(
            row.matrix_row_id.clone(),
            row.acting_identities
                .first()
                .copied()
                .unwrap_or(SecretBoundaryActingIdentityClass::LocalOnlyHandle),
            row.consumer_identities
                .first()
                .copied()
                .unwrap_or(SecretBoundaryConsumerIdentityClass::LocalWorkflow),
            row.local_safe_behavior.clone(),
            repairable_state,
        )]
    } else {
        Vec::new()
    };
    SecretBoundaryLineageBundle::from_events(
        row.matrix_row_id.clone(),
        M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        events,
    )
}

fn derived_lineage_events_from_surface_state(
    state: &SecretBoundarySurfaceState,
) -> Vec<SecretBoundaryLineageEvent> {
    if let Some(repairable_state) = &state.active_repair_state {
        return vec![lineage_event_from_repairable_state(
            state.matrix_row_id.clone(),
            state.consumer_identity_receipt.actor_identity,
            state.consumer_identity_receipt.consumer_identity,
            state
                .credential_state_row
                .decline_path
                .still_works_summary
                .clone(),
            repairable_state,
        )];
    }

    if let Some(repairable_state) = state.repairable_states.iter().find(|repairable_state| {
        repairable_state.triggering_health_state == state.credential_state_row.health_state
    }) {
        return vec![lineage_event_from_repairable_state(
            state.matrix_row_id.clone(),
            state.consumer_identity_receipt.actor_identity,
            state.consumer_identity_receipt.consumer_identity,
            state
                .credential_state_row
                .decline_path
                .still_works_summary
                .clone(),
            repairable_state,
        )];
    }

    if let Some(event) = generic_lineage_event_from_surface_state(state) {
        return vec![event];
    }

    state
        .repairable_states
        .first()
        .map(|repairable_state| {
            vec![lineage_event_from_repairable_state(
                state.matrix_row_id.clone(),
                state.consumer_identity_receipt.actor_identity,
                state.consumer_identity_receipt.consumer_identity,
                state
                    .credential_state_row
                    .decline_path
                    .still_works_summary
                    .clone(),
                repairable_state,
            )]
        })
        .unwrap_or_default()
}

fn lineage_event_from_repairable_state(
    matrix_row_id: String,
    actor_identity: SecretBoundaryActingIdentityClass,
    consumer_identity: SecretBoundaryConsumerIdentityClass,
    local_safe_behavior: String,
    repairable_state: &SecretBoundaryRepairableState,
) -> SecretBoundaryLineageEvent {
    let event_class = event_class_for_repairable_state(repairable_state);
    let failure_dimension = failure_dimension_for_repairable_state(repairable_state);
    let impacted_count = repairable_state.blocked_workflows.len();
    SecretBoundaryLineageEvent {
        event_id: format!(
            "lineage:{}:{}",
            repairable_state.matrix_row_id,
            repairable_state.change_class.as_str()
        ),
        matrix_row_id,
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        event_class,
        failure_dimension,
        health_state: repairable_state.triggering_health_state,
        actor_identity,
        consumer_identity,
        target_boundary_label: repairable_state.affected_target_label.clone(),
        impacted_workflows: repairable_state.blocked_workflows.clone(),
        next_safe_action_label: repairable_state.next_action_label.clone(),
        local_safe_behavior: local_safe_behavior.clone(),
        durable_reopen_target_ref: repairable_state.repair_state_id.clone(),
        doctor_finding_code: Some(repairable_state.doctor_finding_code.clone()),
        repair_candidate_id: Some(repairable_state.repair_candidate_id.clone()),
        support_bundle_lineage_ref: Some(repairable_state.support_bundle_lineage_ref.clone()),
        export_safe_summary: format!(
            "{} affects {} workflow(s). {}",
            repairable_state.affected_target_label, impacted_count, local_safe_behavior
        ),
    }
}

fn generic_lineage_event_from_surface_state(
    state: &SecretBoundarySurfaceState,
) -> Option<SecretBoundaryLineageEvent> {
    let health_state = state.credential_state_row.health_state;
    let event_class = match health_state {
        SecretBoundaryHealthStateClass::Revoked => {
            SecretBoundaryLineageEventClass::CredentialRevoked
        }
        SecretBoundaryHealthStateClass::PolicyBlocked => {
            SecretBoundaryLineageEventClass::PolicyDeniedProjection
        }
        SecretBoundaryHealthStateClass::ForwardingPaused => {
            SecretBoundaryLineageEventClass::ForwardingPaused
        }
        SecretBoundaryHealthStateClass::Expired => {
            SecretBoundaryLineageEventClass::RotationRequired
        }
        SecretBoundaryHealthStateClass::Missing
        | SecretBoundaryHealthStateClass::Unavailable
        | SecretBoundaryHealthStateClass::RemoteVaultUnavailable
        | SecretBoundaryHealthStateClass::NotConfigured => {
            SecretBoundaryLineageEventClass::RebindRequired
        }
        SecretBoundaryHealthStateClass::Healthy | SecretBoundaryHealthStateClass::ExpiringSoon => {
            return None;
        }
    };
    let failure_dimension = failure_dimension_for_health(health_state);
    let next_safe_action_label = match event_class {
        SecretBoundaryLineageEventClass::CredentialRevoked
        | SecretBoundaryLineageEventClass::RotationRequired => {
            state.credential_state_row.rotate_action_label.clone()
        }
        SecretBoundaryLineageEventClass::PolicyDeniedProjection
        | SecretBoundaryLineageEventClass::RebindRequired => {
            state.credential_state_row.test_action_label.clone()
        }
        SecretBoundaryLineageEventClass::ForwardingPaused => {
            "Resume forwarded projection".to_owned()
        }
    };
    let impacted_workflows = state.credential_state_row.dependent_workflows.clone();
    Some(SecretBoundaryLineageEvent {
        event_id: format!("lineage:{}:{}", state.matrix_row_id, event_class.as_str()),
        matrix_row_id: state.matrix_row_id.clone(),
        vocabulary_ref: state.vocabulary_ref.clone(),
        event_class,
        failure_dimension,
        health_state,
        actor_identity: state.consumer_identity_receipt.actor_identity,
        consumer_identity: state.consumer_identity_receipt.consumer_identity,
        target_boundary_label: state.credential_state_row.target_boundary_label.clone(),
        impacted_workflows: impacted_workflows.clone(),
        next_safe_action_label,
        local_safe_behavior: state
            .credential_state_row
            .decline_path
            .still_works_summary
            .clone(),
        durable_reopen_target_ref: format!("secret_boundary:{}", state.matrix_row_id),
        doctor_finding_code: None,
        repair_candidate_id: None,
        support_bundle_lineage_ref: None,
        export_safe_summary: format!(
            "{} blocks {} workflow(s). {}",
            state.credential_state_row.target_boundary_label,
            impacted_workflows.len(),
            state.credential_state_row.decline_path.still_works_summary
        ),
    })
}

fn event_class_for_repairable_state(
    repairable_state: &SecretBoundaryRepairableState,
) -> SecretBoundaryLineageEventClass {
    match repairable_state.change_class {
        SecretBoundaryRepairableChangeClass::RotationRequired => {
            SecretBoundaryLineageEventClass::RotationRequired
        }
        SecretBoundaryRepairableChangeClass::CredentialRevoked => {
            SecretBoundaryLineageEventClass::CredentialRevoked
        }
        _ if repairable_state.triggering_health_state
            == SecretBoundaryHealthStateClass::PolicyBlocked =>
        {
            SecretBoundaryLineageEventClass::PolicyDeniedProjection
        }
        _ => SecretBoundaryLineageEventClass::RebindRequired,
    }
}

fn failure_dimension_for_repairable_state(
    repairable_state: &SecretBoundaryRepairableState,
) -> SecretBoundaryFailureDimensionClass {
    match repairable_state.change_class {
        SecretBoundaryRepairableChangeClass::CaUntrusted
        | SecretBoundaryRepairableChangeClass::BundleStale
        | SecretBoundaryRepairableChangeClass::PinMismatch
        | SecretBoundaryRepairableChangeClass::SshHostKeyUnknown
        | SecretBoundaryRepairableChangeClass::SshHostKeyMismatch
        | SecretBoundaryRepairableChangeClass::ClientCertificateRequired
        | SecretBoundaryRepairableChangeClass::ClientCertificateExpired => {
            SecretBoundaryFailureDimensionClass::Trust
        }
        SecretBoundaryRepairableChangeClass::BrowserHandoffReturnLost
        | SecretBoundaryRepairableChangeClass::DeviceCodeRenewalRequired
        | SecretBoundaryRepairableChangeClass::RotationRequired
        | SecretBoundaryRepairableChangeClass::CredentialRevoked => {
            SecretBoundaryFailureDimensionClass::Credential
        }
    }
}

fn failure_dimension_for_health(
    health_state: SecretBoundaryHealthStateClass,
) -> SecretBoundaryFailureDimensionClass {
    match health_state {
        SecretBoundaryHealthStateClass::PolicyBlocked => {
            SecretBoundaryFailureDimensionClass::Policy
        }
        SecretBoundaryHealthStateClass::ForwardingPaused => {
            SecretBoundaryFailureDimensionClass::RuntimeHealth
        }
        SecretBoundaryHealthStateClass::Unavailable
        | SecretBoundaryHealthStateClass::RemoteVaultUnavailable => {
            SecretBoundaryFailureDimensionClass::Network
        }
        SecretBoundaryHealthStateClass::Missing
        | SecretBoundaryHealthStateClass::NotConfigured
        | SecretBoundaryHealthStateClass::Expired
        | SecretBoundaryHealthStateClass::Revoked => {
            SecretBoundaryFailureDimensionClass::Credential
        }
        SecretBoundaryHealthStateClass::Healthy | SecretBoundaryHealthStateClass::ExpiringSoon => {
            SecretBoundaryFailureDimensionClass::Credential
        }
    }
}

fn activity_severity_for_health(
    health_state: SecretBoundaryHealthStateClass,
) -> SecretBoundaryActivitySeverityClass {
    match health_state {
        SecretBoundaryHealthStateClass::PolicyBlocked | SecretBoundaryHealthStateClass::Revoked => {
            SecretBoundaryActivitySeverityClass::Error
        }
        SecretBoundaryHealthStateClass::Expired
        | SecretBoundaryHealthStateClass::Unavailable
        | SecretBoundaryHealthStateClass::RemoteVaultUnavailable
        | SecretBoundaryHealthStateClass::Missing
        | SecretBoundaryHealthStateClass::NotConfigured => {
            SecretBoundaryActivitySeverityClass::Warning
        }
        SecretBoundaryHealthStateClass::ForwardingPaused => {
            SecretBoundaryActivitySeverityClass::Info
        }
        SecretBoundaryHealthStateClass::Healthy | SecretBoundaryHealthStateClass::ExpiringSoon => {
            SecretBoundaryActivitySeverityClass::Info
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
    /// A row omitted every repairable state.
    MissingRepairableStates(String),
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
    /// A repairable state drifted from its parent row id.
    RepairableStateRowIdMismatch(String, String),
    /// A repairable state drifted from the shared vocabulary ref.
    RepairableStateVocabularyDrift(String),
    /// A repairable state omitted one or more required fields.
    IncompleteRepairableState(String),
    /// A repairable state declared an invalid doctor finding code.
    RepairableStateDoctorFindingInvalid(String),
    /// A repairable state drifted from the row repair owner.
    RepairableStateOwnerMismatch(String),
    /// A repairable state used a non-blocking triggering health state.
    RepairableStateHealthyTrigger(String),
    /// One required domain had no coverage.
    MissingDomainCoverage(SecretBoundarySurfaceDomain),
    /// A row/profile qualification is missing.
    MissingQualificationCoverage(String, SecretBoundaryDeploymentProfileClass),
    /// A qualification id appeared more than once.
    DuplicateQualificationRowId(String),
    /// A qualification row drifted from the parent domain.
    QualificationDomainMismatch(String),
    /// A qualification row referenced an unknown matrix row.
    QualificationUnknownMatrixRow(String),
    /// A qualification row referenced a missing per-profile parity row.
    QualificationUnknownProfileParity(String),
    /// A qualification row rendered `qualified_current` without current proof.
    CurrentQualificationMissingCurrentProof(String),
    /// A qualification row rendered `qualified_current` while a required proof
    /// coverage dimension was false.
    CurrentQualificationMissingCoverage(String),
    /// A qualification row rendered `qualified_current` on a local-only missing
    /// profile.
    CurrentQualificationOnMissingProfile(String),
    /// A narrowed qualification row omitted its typed narrow reason.
    NarrowedQualificationMissingReason(String),
    /// A consumer projection omitted or mismatched qualification coverage.
    ConsumerProjectionQualificationDrift(SecretBoundaryConsumerSurface),
    /// One required consumer projection was absent.
    MissingConsumerProjection(SecretBoundaryConsumerSurface),
    /// A consumer projection drifted from the matrix id or vocabulary ref.
    ConsumerProjectionDrift(SecretBoundaryConsumerSurface),
    /// One required artifact export family had no rule coverage.
    MissingArtifactExportRule(SecretBoundaryArtifactFamilyClass),
    /// An artifact rule omitted alias, handle-class, source-label, or consumer-identity retention.
    IncompleteArtifactExportRule(SecretBoundaryArtifactFamilyClass),
    /// An artifact rule omitted one required omitted-material class.
    ArtifactRuleMissingOmittedMaterial(
        SecretBoundaryArtifactFamilyClass,
        SecretBoundaryOmittedMaterialClass,
    ),
    /// An artifact rule omitted its banner, omission marker, or typed rebind copy.
    ArtifactRuleMissingBannerOrRebind(SecretBoundaryArtifactFamilyClass),
    /// A required artifact-family token was not represented anywhere in the packet.
    MissingRequiredArtifactFamily(SecretBoundaryArtifactFamilyClass),
    /// A required omitted-material token was not represented anywhere in the packet.
    MissingRequiredOmittedMaterial(SecretBoundaryOmittedMaterialClass),
    /// A required consumer-identity token was not represented anywhere.
    MissingRequiredConsumerIdentity(SecretBoundaryConsumerIdentityClass),
    /// A required projection-control token was not represented anywhere.
    MissingRequiredProjectionControl(SecretBoundaryProjectionControlClass),
    /// A required qualification-label token was not represented anywhere.
    MissingRequiredQualificationLabel(SecretBoundaryQualificationLabel),
    /// A required proof-freshness token was not represented anywhere.
    MissingRequiredProofFreshness(SecretBoundaryProofFreshnessClass),
    /// A required qualification narrow-reason token was not represented
    /// anywhere.
    MissingRequiredQualificationNarrowReason(SecretBoundaryQualificationNarrowReason),
    /// A required health-state token was not represented anywhere in the packet.
    MissingRequiredHealthState(SecretBoundaryHealthStateClass),
    /// A required repairable change token was not represented anywhere in the packet.
    MissingRequiredRepairableChange(SecretBoundaryRepairableChangeClass),
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
            Self::MissingRepairableStates(row) => {
                write!(f, "row {row} must declare at least one repairable state")
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
            Self::RepairableStateRowIdMismatch(row, repair_state) => write!(
                f,
                "repairable state {repair_state} drifted from parent row {row}"
            ),
            Self::RepairableStateVocabularyDrift(repair_state) => write!(
                f,
                "repairable state {repair_state} drifted from the shared vocabulary ref"
            ),
            Self::IncompleteRepairableState(repair_state) => write!(
                f,
                "repairable state {repair_state} must declare target, last-known-good, next-action, doctor, repair, and lineage fields"
            ),
            Self::RepairableStateDoctorFindingInvalid(repair_state) => write!(
                f,
                "repairable state {repair_state} must cite a doctor.finding.* code"
            ),
            Self::RepairableStateOwnerMismatch(repair_state) => write!(
                f,
                "repairable state {repair_state} must preserve the row repair owner"
            ),
            Self::RepairableStateHealthyTrigger(repair_state) => write!(
                f,
                "repairable state {repair_state} must trigger only on a blocked health posture"
            ),
            Self::MissingDomainCoverage(domain) => {
                write!(f, "domain {} is missing from the matrix", domain.as_str())
            }
            Self::MissingQualificationCoverage(row, profile) => write!(
                f,
                "row {row} is missing a qualification for deployment-profile {}",
                profile.as_str()
            ),
            Self::DuplicateQualificationRowId(row) => {
                write!(f, "qualification row id {row} is duplicated")
            }
            Self::QualificationDomainMismatch(row) => write!(
                f,
                "qualification row {row} drifted from the parent row domain"
            ),
            Self::QualificationUnknownMatrixRow(row) => write!(
                f,
                "qualification row {row} referenced an unknown matrix row"
            ),
            Self::QualificationUnknownProfileParity(row) => write!(
                f,
                "qualification row {row} referenced a missing profile-parity row"
            ),
            Self::CurrentQualificationMissingCurrentProof(row) => write!(
                f,
                "qualification row {row} rendered qualified_current without current proof"
            ),
            Self::CurrentQualificationMissingCoverage(row) => write!(
                f,
                "qualification row {row} rendered qualified_current without all required secret-boundary proof coverage"
            ),
            Self::CurrentQualificationOnMissingProfile(row) => write!(
                f,
                "qualification row {row} rendered qualified_current on a local-continuity-only profile"
            ),
            Self::NarrowedQualificationMissingReason(row) => write!(
                f,
                "qualification row {row} narrowed below qualified_current without a typed reason"
            ),
            Self::ConsumerProjectionQualificationDrift(surface) => write!(
                f,
                "consumer projection {} drifted from the shared qualification coverage set",
                surface.as_str()
            ),
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
            Self::MissingArtifactExportRule(family) => write!(
                f,
                "artifact export rule {} is missing from the packet",
                family.as_str()
            ),
            Self::IncompleteArtifactExportRule(family) => write!(
                f,
                "artifact export rule {} must preserve aliases, handle classes, source labels, and consumer identity",
                family.as_str()
            ),
            Self::ArtifactRuleMissingOmittedMaterial(family, material) => write!(
                f,
                "artifact export rule {} must omit {} by default",
                family.as_str(),
                material.as_str()
            ),
            Self::ArtifactRuleMissingBannerOrRebind(family) => write!(
                f,
                "artifact export rule {} must declare an omission banner, omission marker, and typed rebind copy",
                family.as_str()
            ),
            Self::MissingRequiredArtifactFamily(family) => write!(
                f,
                "packet must cover the {} artifact family",
                family.as_str()
            ),
            Self::MissingRequiredOmittedMaterial(material) => write!(
                f,
                "packet must cover the {} omitted material class",
                material.as_str()
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
            Self::MissingRequiredQualificationLabel(label) => write!(
                f,
                "packet must cover the {} qualification label",
                label.as_str()
            ),
            Self::MissingRequiredProofFreshness(freshness) => write!(
                f,
                "packet must cover the {} proof-freshness posture",
                freshness.as_str()
            ),
            Self::MissingRequiredQualificationNarrowReason(reason) => write!(
                f,
                "packet must cover the {} qualification narrow-reason",
                reason.as_str()
            ),
            Self::MissingRequiredHealthState(state) => write!(
                f,
                "packet must cover the {} health state",
                state.as_str()
            ),
            Self::MissingRequiredRepairableChange(change_class) => write!(
                f,
                "packet must cover the {} repairable change",
                change_class.as_str()
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
pub fn seeded_secret_boundary_surface_row(matrix_row_id: &str) -> Option<SecretBoundarySurfaceRow> {
    seeded_m5_secret_boundary_depth_packet()
        .surface_rows
        .into_iter()
        .find(|row| row.matrix_row_id == matrix_row_id)
}

/// Returns the canonical repairable states for one matrix row id.
pub fn seeded_secret_boundary_repairable_states(
    matrix_row_id: &str,
) -> Vec<SecretBoundaryRepairableState> {
    seeded_secret_boundary_surface_row(matrix_row_id)
        .map(|row| row.repairable_states)
        .unwrap_or_default()
}

/// Returns the canonical active repairable state for one matrix row id and
/// triggering health state, if one exists.
pub fn seeded_secret_boundary_active_repair_state(
    matrix_row_id: &str,
    health_state: SecretBoundaryHealthStateClass,
) -> Option<SecretBoundaryRepairableState> {
    seeded_secret_boundary_repairable_states(matrix_row_id)
        .into_iter()
        .find(|state| state.triggering_health_state == health_state)
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

fn repairable_state(
    repair_state_id: &str,
    matrix_row_id: &str,
    change_class: SecretBoundaryRepairableChangeClass,
    triggering_health_state: SecretBoundaryHealthStateClass,
    affected_target_ref: &str,
    affected_target_label: &str,
    last_known_good_class: SecretBoundaryLastKnownGoodClass,
    last_known_good_summary: &str,
    blocked_workflows: Vec<SecretBoundaryWorkflowDependency>,
    next_action_label: &str,
    repair_owner: SecretBoundaryRepairOwnerClass,
    doctor_probe_family: SecretBoundaryDoctorProbeFamilyClass,
    doctor_finding_code: &str,
    repair_candidate_id: &str,
    support_bundle_lineage_ref: &str,
) -> SecretBoundaryRepairableState {
    SecretBoundaryRepairableState {
        repair_state_id: repair_state_id.to_owned(),
        matrix_row_id: matrix_row_id.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        change_class,
        triggering_health_state,
        affected_target_ref: affected_target_ref.to_owned(),
        affected_target_label: affected_target_label.to_owned(),
        last_known_good_class,
        last_known_good_summary: last_known_good_summary.to_owned(),
        blocked_workflows,
        next_action_label: next_action_label.to_owned(),
        repair_owner,
        doctor_probe_family,
        doctor_finding_code: doctor_finding_code.to_owned(),
        repair_candidate_id: repair_candidate_id.to_owned(),
        support_bundle_lineage_ref: support_bundle_lineage_ref.to_owned(),
    }
}

fn workflow_dependency(
    workflow_ref: &str,
    workflow_label: &str,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.to_owned(),
        workflow_label: workflow_label.to_owned(),
    }
}

fn artifact_export_rule(
    artifact_family: SecretBoundaryArtifactFamilyClass,
    source_ref: &str,
    export_posture: SecretBoundaryExportPostureClass,
    omission_marker_label: &str,
    omission_banner: &str,
    requires_typed_rebind_on_import: bool,
    requires_typed_rebind_on_replay: bool,
    rebind_action_label: &str,
    rebind_failure_label: &str,
) -> SecretBoundaryArtifactExportRule {
    SecretBoundaryArtifactExportRule {
        artifact_family,
        source_ref: source_ref.to_owned(),
        export_posture,
        preserves_credential_aliases: true,
        preserves_handle_classes: true,
        preserves_source_labels: true,
        preserves_consumer_identity: true,
        omission_marker_label: omission_marker_label.to_owned(),
        omission_banner: omission_banner.to_owned(),
        omitted_material_classes: SecretBoundaryOmittedMaterialClass::ALL.to_vec(),
        requires_typed_rebind_on_import,
        requires_typed_rebind_on_replay,
        rebind_action_label: rebind_action_label.to_owned(),
        rebind_failure_label: rebind_failure_label.to_owned(),
    }
}

fn evidence_index_ref() -> String {
    "evidence_index:m5.secret_boundary.depth.v1".to_owned()
}

fn qualification_proof(
    row: &SecretBoundarySurfaceRow,
    deployment_profile: SecretBoundaryDeploymentProfileClass,
) -> Option<SecretBoundaryQualificationProof> {
    let proof = match row.matrix_row_id.as_str() {
        "m5.secret.request_workspace.send_http"
        | "m5.secret.request_workspace.history_replay" => Some((
            "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json",
            "schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json",
            vec![
                "crates/aureline-api/src/materialize_versioned_request_workspace_documents_environment_sets_and_auth_source_inspectors/mod.rs".to_owned(),
                "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json".to_owned(),
            ],
        )),
        "m5.secret.database.connection_picker" => Some((
            "artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json",
            "schemas/data/database-statement-safety-and-result-grid-qualification.schema.json",
            vec![
                "crates/aureline-data/src/database_qualification.rs".to_owned(),
                "artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json".to_owned(),
            ],
        )),
        "m5.secret.database.query_history_portability" => Some((
            "artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json",
            "schemas/data/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.schema.json",
            vec![
                "crates/aureline-api/src/ship_query_history_connection_profile_portability_secret_safe_auth_storage_and_mirror_or_offline_truth/mod.rs".to_owned(),
                "artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json".to_owned(),
            ],
        )),
        "m5.secret.preview_route.remote_preview" => Some((
            "artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json",
            "schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json",
            vec![
                "crates/aureline-review/src/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/mod.rs".to_owned(),
                "artifacts/review/m5/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/support_export.json".to_owned(),
            ],
        )),
        "m5.secret.infra_connector.target_context" => Some((
            "artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json",
            "schemas/data/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.schema.json",
            vec![
                "crates/aureline-api/src/implement_connection_browsers_schema_trees_and_target_context_envelopes_for_database_tooling/mod.rs".to_owned(),
                "artifacts/data/m5/implement-connection-browsers-schema-trees-and-target-context-envelopes-for-database-tooling.json".to_owned(),
            ],
        )),
        "m5.secret.companion.session_handoff" => Some((
            "artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json",
            "schemas/companion/add-remote-preview-or-session-handoff-light-remote-edit-and-scoped-collaboration-follow-continuity-on-companio.schema.json",
            vec![
                "crates/aureline-companion/src/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/mod.rs".to_owned(),
                "artifacts/companion/m5/add_remote_preview_or_session_handoff_light_remote_edit_and_scoped_collaboration_follow_continuity_on_companio/support_export.json".to_owned(),
            ],
        )),
        "m5.secret.managed.workspace_runtime" => Some((
            "fixtures/remote/managed_workspace_lifecycle/support_export.json",
            "schemas/remote/managed_workspace_lifecycle.schema.json",
            vec![
                "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs".to_owned(),
                "fixtures/remote/managed_workspace_lifecycle/support_export.json".to_owned(),
            ],
        )),
        "m5.secret.managed.sync_plane" => Some((
            "artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json",
            "schemas/companion/ship-managed-sync-maturity-with-snapshot-classes-conflict-review-device-registry-and-end-to-end-encrypted-storage.schema.json",
            vec![
                "crates/aureline-companion/src/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/mod.rs".to_owned(),
                "artifacts/companion/m5/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/support_export.json".to_owned(),
            ],
        )),
        _ => None,
    }?;

    Some(SecretBoundaryQualificationProof {
        proof_id: format!(
            "secret_boundary_proof:{}:{}",
            row.matrix_row_id,
            deployment_profile.as_str()
        ),
        packet_ref: proof.0.to_owned(),
        schema_ref: proof.1.to_owned(),
        proof_index_ref: format!(
            "{}:{}:{}",
            evidence_index_ref(),
            row.matrix_row_id,
            deployment_profile.as_str()
        ),
        captured_at: "2026-06-12T00:00:00Z".to_owned(),
        evidence_refs: proof.2,
    })
}

fn qualification_narrow_reason_for_gap(
    proof_freshness: SecretBoundaryProofFreshnessClass,
    handle_classes_verified: bool,
    vault_picker_verified: bool,
    delegated_identity_verified: bool,
    projection_pause_verified: bool,
    trust_store_drift_verified: bool,
    rotation_revoke_lineage_verified: bool,
    export_redaction_verified: bool,
    repair_flow_verified: bool,
) -> SecretBoundaryQualificationNarrowReason {
    match proof_freshness {
        SecretBoundaryProofFreshnessClass::Missing => {
            SecretBoundaryQualificationNarrowReason::ProofPacketMissing
        }
        SecretBoundaryProofFreshnessClass::Stale => {
            SecretBoundaryQualificationNarrowReason::ProofPacketStale
        }
        SecretBoundaryProofFreshnessClass::Current => {
            if !handle_classes_verified {
                SecretBoundaryQualificationNarrowReason::HandleClassesUnproven
            } else if !vault_picker_verified {
                SecretBoundaryQualificationNarrowReason::VaultPickerUnproven
            } else if !delegated_identity_verified {
                SecretBoundaryQualificationNarrowReason::DelegatedIdentityUnproven
            } else if !projection_pause_verified {
                SecretBoundaryQualificationNarrowReason::ProjectionPauseUnproven
            } else if !trust_store_drift_verified {
                SecretBoundaryQualificationNarrowReason::TrustStoreDriftUnproven
            } else if !rotation_revoke_lineage_verified {
                SecretBoundaryQualificationNarrowReason::RotationRevokeLineageUnproven
            } else if !export_redaction_verified {
                SecretBoundaryQualificationNarrowReason::ExportRedactionUnproven
            } else if !repair_flow_verified {
                SecretBoundaryQualificationNarrowReason::RepairFlowUnproven
            } else {
                SecretBoundaryQualificationNarrowReason::NotNarrowed
            }
        }
    }
}

fn qualification_row_for_profile(
    row: &SecretBoundarySurfaceRow,
    profile_row: &SecretBoundaryProfileParityRow,
) -> SecretBoundaryProfileQualificationRow {
    let qualification_row_id = format!(
        "qualification:{}:{}",
        row.matrix_row_id,
        profile_row.deployment_profile.as_str()
    );
    let proof = qualification_proof(row, profile_row.deployment_profile);
    let proof_freshness = if proof.is_some() {
        SecretBoundaryProofFreshnessClass::Current
    } else {
        SecretBoundaryProofFreshnessClass::Missing
    };
    let local_continuity_only = matches!(
        profile_row.projection_parity,
        SecretBoundaryProjectionParityClass::Missing
    ) || matches!(
        profile_row.health_state,
        SecretBoundaryHealthStateClass::Missing | SecretBoundaryHealthStateClass::NotConfigured
    );
    let coverage_verified = proof.is_some();
    let narrow_reason = if local_continuity_only {
        SecretBoundaryQualificationNarrowReason::ProfileLocalContinuityOnly
    } else {
        qualification_narrow_reason_for_gap(
            proof_freshness,
            coverage_verified,
            coverage_verified,
            coverage_verified,
            coverage_verified,
            coverage_verified,
            coverage_verified,
            coverage_verified,
            coverage_verified,
        )
    };
    let displayed_label = if local_continuity_only {
        SecretBoundaryQualificationLabel::LimitedLocalContinuity
    } else if narrow_reason == SecretBoundaryQualificationNarrowReason::NotNarrowed {
        SecretBoundaryQualificationLabel::QualifiedCurrent
    } else {
        SecretBoundaryQualificationLabel::SupportReviewOnly
    };
    let rationale = match displayed_label {
        SecretBoundaryQualificationLabel::QualifiedCurrent => format!(
            "{} on {} is backed by a current checked proof packet covering handle classes, storage-source disclosure, delegated identity, projection controls, trust-store drift, rotation/revoke lineage, export redaction, and repair flow.",
            row.title,
            profile_row.deployment_profile.as_str()
        ),
        SecretBoundaryQualificationLabel::LimitedLocalContinuity => format!(
            "{} on {} keeps only the local-safe continuity label because this profile does not claim a live credential path.",
            row.title,
            profile_row.deployment_profile.as_str()
        ),
        SecretBoundaryQualificationLabel::SupportReviewOnly => format!(
            "{} on {} narrows to support-review-only because current secret-boundary proof is missing or incomplete.",
            row.title,
            profile_row.deployment_profile.as_str()
        ),
    };

    SecretBoundaryProfileQualificationRow {
        qualification_row_id,
        matrix_row_id: row.matrix_row_id.clone(),
        domain: row.domain,
        deployment_profile: profile_row.deployment_profile,
        claimed_label: SecretBoundaryQualificationLabel::QualifiedCurrent,
        displayed_label,
        proof_freshness,
        qualification_packet: proof,
        profile_parity_ref: format!(
            "{}:{}",
            row.matrix_row_id,
            profile_row.deployment_profile.as_str()
        ),
        handle_classes_verified: coverage_verified,
        vault_picker_verified: coverage_verified,
        delegated_identity_verified: coverage_verified,
        projection_pause_verified: coverage_verified,
        trust_store_drift_verified: coverage_verified,
        rotation_revoke_lineage_verified: coverage_verified,
        export_redaction_verified: coverage_verified,
        repair_flow_verified: coverage_verified,
        narrow_reason,
        rationale,
    }
}

fn qualification_rows_for_surface_rows(
    surface_rows: &[SecretBoundarySurfaceRow],
) -> Vec<SecretBoundaryProfileQualificationRow> {
    surface_rows
        .iter()
        .flat_map(|row| {
            row.profile_parity_rows
                .iter()
                .map(move |profile_row| qualification_row_for_profile(row, profile_row))
        })
        .collect()
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.request_workspace.send_http.client_certificate_required",
                "m5.secret.request_workspace.send_http",
                SecretBoundaryRepairableChangeClass::ClientCertificateRequired,
                SecretBoundaryHealthStateClass::Missing,
                "target:request_workspace:send_http:mtls_origin",
                "Request workspace mTLS origin",
                SecretBoundaryLastKnownGoodClass::ClientCertificateBinding,
                "The last-known-good request route used the same client-certificate binding and target audience without widening scope.",
                vec![workflow_dependency(
                    "workflow:request.send_http",
                    "Send request with current auth source",
                )],
                "Rebind the exact client certificate for this origin",
                SecretBoundaryRepairOwnerClass::User,
                SecretBoundaryDoctorProbeFamilyClass::NetworkProxyCaTransport,
                "doctor.finding.secret_boundary.request_workspace.client_certificate_required",
                "repair_candidate:secret_boundary.request_workspace.rebind_client_certificate",
                "support.lineage.secret_boundary.request_workspace.send_http.client_certificate_required",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.request_workspace.history_replay.rotation_required",
                "m5.secret.request_workspace.history_replay",
                SecretBoundaryRepairableChangeClass::RotationRequired,
                SecretBoundaryHealthStateClass::Expired,
                "target:request_workspace:history_replay:auth_source",
                "Replayed auth source",
                SecretBoundaryLastKnownGoodClass::DelegatedScopeBinding,
                "The last-known-good replay used the same auth-source alias and scope binding before rotation closed it.",
                vec![workflow_dependency(
                    "workflow:request.history_replay",
                    "Replay stored request metadata",
                )],
                "Refresh the replay auth source without mutating stored history",
                SecretBoundaryRepairOwnerClass::User,
                SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                "doctor.finding.secret_boundary.request_workspace.rotation_required",
                "repair_candidate:secret_boundary.request_workspace.refresh_replay_auth_source",
                "support.lineage.secret_boundary.request_workspace.history_replay.rotation_required",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.database.connection_picker.client_certificate_expired",
                "m5.secret.database.connection_picker",
                SecretBoundaryRepairableChangeClass::ClientCertificateExpired,
                SecretBoundaryHealthStateClass::Expired,
                "target:database.connection_picker:live_session",
                "Live database session",
                SecretBoundaryLastKnownGoodClass::ClientCertificateBinding,
                "The last-known-good session used the same bound database certificate and fingerprint under the current connector policy.",
                vec![
                    workflow_dependency(
                        "workflow:database.connect",
                        "Open live database session",
                    ),
                    workflow_dependency(
                        "workflow:database.schema",
                        "Browse schema and target context",
                    ),
                ],
                "Renew the bound client certificate and retest this connector",
                SecretBoundaryRepairOwnerClass::DataOperator,
                SecretBoundaryDoctorProbeFamilyClass::NetworkProxyCaTransport,
                "doctor.finding.secret_boundary.database.client_certificate_expired",
                "repair_candidate:secret_boundary.database.renew_client_certificate",
                "support.lineage.secret_boundary.database.connection_picker.client_certificate_expired",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.database.query_history_portability.ca_untrusted",
                "m5.secret.database.query_history_portability",
                SecretBoundaryRepairableChangeClass::CaUntrusted,
                SecretBoundaryHealthStateClass::Unavailable,
                "target:database.query_history_portability:replay_origin",
                "Database replay origin",
                SecretBoundaryLastKnownGoodClass::OsTrustStoreDescriptor,
                "The last-known-good replay route chained through the platform trust store and admitted the same replay origin.",
                vec![workflow_dependency(
                    "workflow:database.replay",
                    "Replay stored query metadata",
                )],
                "Inspect the trust source and restore the approved CA path for replay",
                SecretBoundaryRepairOwnerClass::DataOperator,
                SecretBoundaryDoctorProbeFamilyClass::NetworkProxyCaTransport,
                "doctor.finding.secret_boundary.database.ca_untrusted",
                "repair_candidate:secret_boundary.database.restore_ca_path_for_replay",
                "support.lineage.secret_boundary.database.query_history_portability.ca_untrusted",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.provider_model.route_resolution.browser_handoff_return_lost",
                "m5.secret.provider_model.route_resolution",
                SecretBoundaryRepairableChangeClass::BrowserHandoffReturnLost,
                SecretBoundaryHealthStateClass::Expired,
                "target:provider.route_resolution:connected_provider_route",
                "Connected provider route",
                SecretBoundaryLastKnownGoodClass::BrowserHandoffSession,
                "The last-known-good provider route completed through the same browser handoff packet and callback correlation envelope.",
                vec![
                    workflow_dependency("workflow:provider.route.inspect", "Inspect provider route"),
                    workflow_dependency("workflow:provider.route.repair", "Repair provider route auth"),
                ],
                "Retry the exact browser handoff or switch to the bounded device-code fallback",
                SecretBoundaryRepairOwnerClass::ProviderOperator,
                SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                "doctor.finding.secret_boundary.provider.browser_handoff_return_lost",
                "repair_candidate:secret_boundary.provider.retry_browser_handoff",
                "support.lineage.secret_boundary.provider.route_resolution.browser_handoff_return_lost",
            )],
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
                    SecretBoundaryHealthStateClass::Revoked,
                    SecretBoundaryStorageClass::RemoteVault,
                    SecretBoundaryActingIdentityClass::ServiceIssuedAuthority,
                    "Rebind revoked provider scope",
                    "Managed scope review stays available when the delegated scope or grant was revoked.",
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
            repairable_states: vec![
                repairable_state(
                    "repair_state:m5.secret.provider_model.scope_registry.device_code_renewal_required",
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryRepairableChangeClass::DeviceCodeRenewalRequired,
                    SecretBoundaryHealthStateClass::Expired,
                    "target:provider.scope_registry:delegated_scope",
                    "Provider delegated scope",
                    SecretBoundaryLastKnownGoodClass::DeviceCodeSession,
                    "The last-known-good scope repair path used the same device-code session and delegated scope binding.",
                    vec![
                        workflow_dependency("workflow:provider.scope.inspect", "Inspect provider scope"),
                        workflow_dependency("workflow:provider.scope.repair", "Repair scope or delegated identity"),
                    ],
                    "Renew the exact device-code session for this scope without widening authority",
                    SecretBoundaryRepairOwnerClass::ProviderOperator,
                    SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                    "doctor.finding.secret_boundary.provider.device_code_renewal_required",
                    "repair_candidate:secret_boundary.provider.renew_device_code_scope",
                    "support.lineage.secret_boundary.provider.scope_registry.device_code_renewal_required",
                ),
                repairable_state(
                    "repair_state:m5.secret.provider_model.scope_registry.credential_revoked",
                    "m5.secret.provider_model.scope_registry",
                    SecretBoundaryRepairableChangeClass::CredentialRevoked,
                    SecretBoundaryHealthStateClass::Revoked,
                    "target:provider.scope_registry:delegated_scope",
                    "Provider delegated scope",
                    SecretBoundaryLastKnownGoodClass::DelegatedScopeBinding,
                    "The last-known-good scope mutation used the reviewed delegated scope binding and installation-grant lineage before revocation.",
                    vec![
                        workflow_dependency("workflow:provider.scope.inspect", "Inspect provider scope"),
                        workflow_dependency("workflow:provider.scope.repair", "Repair scope or delegated identity"),
                    ],
                    "Rebind the exact delegated scope or installation grant before mutating provider state",
                    SecretBoundaryRepairOwnerClass::ProviderOperator,
                    SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                    "doctor.finding.secret_boundary.provider.credential_revoked",
                    "repair_candidate:secret_boundary.provider.rebind_revoked_scope",
                    "support.lineage.secret_boundary.provider.scope_registry.credential_revoked",
                ),
            ],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.registry.package_auth.bundle_stale",
                "m5.secret.registry.package_auth",
                SecretBoundaryRepairableChangeClass::BundleStale,
                SecretBoundaryHealthStateClass::PolicyBlocked,
                "target:registry.package_auth:primary_registry",
                "Primary package registry",
                SecretBoundaryLastKnownGoodClass::OrgCaBundleEpoch,
                "The last-known-good registry session used the current org-approved CA bundle epoch before it went stale.",
                vec![workflow_dependency("workflow:registry.resolve", "Resolve package metadata")],
                "Refresh the approved registry CA bundle or switch to the named mirror",
                SecretBoundaryRepairOwnerClass::User,
                SecretBoundaryDoctorProbeFamilyClass::NetworkProxyCaTransport,
                "doctor.finding.secret_boundary.registry.bundle_stale",
                "repair_candidate:secret_boundary.registry.refresh_ca_bundle",
                "support.lineage.secret_boundary.registry.package_auth.bundle_stale",
            )],
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
                    SecretBoundaryHealthStateClass::Revoked,
                    SecretBoundaryStorageClass::SessionOnly,
                    SecretBoundaryActingIdentityClass::DelegatedCredential,
                    "Rebind delegated preview session",
                    "Managed preview revocation narrows to metadata-only route history and exact desktop handoff instructions.",
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
            repairable_states: vec![
                repairable_state(
                    "repair_state:m5.secret.preview_route.remote_preview.pin_mismatch",
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryRepairableChangeClass::PinMismatch,
                    SecretBoundaryHealthStateClass::PolicyBlocked,
                    "target:preview_route.remote_preview:public_or_org_route",
                    "Preview route trust root",
                    SecretBoundaryLastKnownGoodClass::PinnedControlPlaneRoot,
                    "The last-known-good preview route used the published pinned control-plane roots for this route class.",
                    vec![
                        workflow_dependency("workflow:preview.route", "Open preview route"),
                        workflow_dependency("workflow:preview.share", "Share or revoke preview route"),
                    ],
                    "Review the rotated route trust root before reopening or sharing this preview",
                    SecretBoundaryRepairOwnerClass::RemoteOperator,
                    SecretBoundaryDoctorProbeFamilyClass::NetworkProxyCaTransport,
                    "doctor.finding.secret_boundary.preview.pin_mismatch",
                    "repair_candidate:secret_boundary.preview.review_rotated_root",
                    "support.lineage.secret_boundary.preview.remote_preview.pin_mismatch",
                ),
                repairable_state(
                    "repair_state:m5.secret.preview_route.remote_preview.credential_revoked",
                    "m5.secret.preview_route.remote_preview",
                    SecretBoundaryRepairableChangeClass::CredentialRevoked,
                    SecretBoundaryHealthStateClass::Revoked,
                    "target:preview_route.remote_preview:delegated_session",
                    "Preview delegated session",
                    SecretBoundaryLastKnownGoodClass::DelegatedScopeBinding,
                    "The last-known-good preview route used the same delegated preview audience and reviewed share scope before revocation.",
                    vec![
                        workflow_dependency("workflow:preview.route", "Open preview route"),
                        workflow_dependency("workflow:preview.share", "Share or revoke preview route"),
                    ],
                    "Rebind the delegated preview session before reopening or sharing this route",
                    SecretBoundaryRepairOwnerClass::RemoteOperator,
                    SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                    "doctor.finding.secret_boundary.preview.credential_revoked",
                    "repair_candidate:secret_boundary.preview.rebind_revoked_session",
                    "support.lineage.secret_boundary.preview.remote_preview.credential_revoked",
                ),
            ],
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
            repairable_states: vec![
                repairable_state(
                    "repair_state:m5.secret.infra_connector.target_context.ssh_host_key_mismatch",
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryRepairableChangeClass::SshHostKeyMismatch,
                    SecretBoundaryHealthStateClass::PolicyBlocked,
                    "target:infra_connector.target_context:ssh_control_plane",
                    "SSH target control plane",
                    SecretBoundaryLastKnownGoodClass::SshHostProof,
                    "The last-known-good target context used the approved SSH fingerprint history for this target identity.",
                    vec![
                        workflow_dependency("workflow:target.inspect", "Inspect target context"),
                        workflow_dependency("workflow:target.connect", "Connect to target context"),
                    ],
                    "Review the changed SSH fingerprint before reconnecting this target",
                    SecretBoundaryRepairOwnerClass::RemoteOperator,
                    SecretBoundaryDoctorProbeFamilyClass::RemoteRoutesAndCollaboration,
                    "doctor.finding.secret_boundary.infra.ssh_host_key_mismatch",
                    "repair_candidate:secret_boundary.infra.review_ssh_fingerprint",
                    "support.lineage.secret_boundary.infra.target_context.ssh_host_key_mismatch",
                ),
                repairable_state(
                    "repair_state:m5.secret.infra_connector.target_context.ssh_host_key_unknown",
                    "m5.secret.infra_connector.target_context",
                    SecretBoundaryRepairableChangeClass::SshHostKeyUnknown,
                    SecretBoundaryHealthStateClass::Missing,
                    "target:infra_connector.target_context:ssh_control_plane",
                    "SSH target control plane",
                    SecretBoundaryLastKnownGoodClass::SshHostProof,
                    "The last-known-good target class requires an approved host-proof record before connection is allowed.",
                    vec![workflow_dependency("workflow:target.connect", "Connect to target context")],
                    "Approve or import the exact SSH host proof for this target",
                    SecretBoundaryRepairOwnerClass::RemoteOperator,
                    SecretBoundaryDoctorProbeFamilyClass::RemoteRoutesAndCollaboration,
                    "doctor.finding.secret_boundary.infra.ssh_host_key_unknown",
                    "repair_candidate:secret_boundary.infra.approve_ssh_host_proof",
                    "support.lineage.secret_boundary.infra.target_context.ssh_host_key_unknown",
                ),
            ],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.companion.session_handoff.browser_handoff_return_lost",
                "m5.secret.companion.session_handoff",
                SecretBoundaryRepairableChangeClass::BrowserHandoffReturnLost,
                SecretBoundaryHealthStateClass::Expired,
                "target:companion.session_handoff:return_path",
                "Companion return path",
                SecretBoundaryLastKnownGoodClass::BrowserHandoffSession,
                "The last-known-good companion handoff returned through the same desktop/browser pairing and callback packet.",
                vec![workflow_dependency("workflow:companion.follow", "Resume companion follow state")],
                "Repeat the desktop/browser return path without widening the companion scope",
                SecretBoundaryRepairOwnerClass::User,
                SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                "doctor.finding.secret_boundary.companion.browser_handoff_return_lost",
                "repair_candidate:secret_boundary.companion.retry_browser_return",
                "support.lineage.secret_boundary.companion.session_handoff.browser_handoff_return_lost",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.managed.workspace_runtime.rotation_required",
                "m5.secret.managed.workspace_runtime",
                SecretBoundaryRepairableChangeClass::RotationRequired,
                SecretBoundaryHealthStateClass::RemoteVaultUnavailable,
                "target:managed.workspace_runtime:remote_vault_lineage",
                "Managed runtime vault lineage",
                SecretBoundaryLastKnownGoodClass::RemoteVaultLineage,
                "The last-known-good runtime resumed through the same remote-vault lineage, delegated scope, and host-proof set.",
                vec![
                    workflow_dependency("workflow:managed.runtime.resume", "Resume managed runtime"),
                    workflow_dependency("workflow:managed.runtime.repair", "Repair managed runtime"),
                ],
                "Rotate the remote-vault lease and revalidate host proof before resume",
                SecretBoundaryRepairOwnerClass::RemoteOperator,
                SecretBoundaryDoctorProbeFamilyClass::RemoteRoutesAndCollaboration,
                "doctor.finding.secret_boundary.managed.rotation_required",
                "repair_candidate:secret_boundary.managed.rotate_remote_vault_lease",
                "support.lineage.secret_boundary.managed.workspace_runtime.rotation_required",
            )],
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
            repairable_states: vec![repairable_state(
                "repair_state:m5.secret.managed.sync_plane.device_code_renewal_required",
                "m5.secret.managed.sync_plane",
                SecretBoundaryRepairableChangeClass::DeviceCodeRenewalRequired,
                SecretBoundaryHealthStateClass::PolicyBlocked,
                "target:managed.sync_plane:service_session",
                "Managed sync-plane session",
                SecretBoundaryLastKnownGoodClass::DeviceCodeSession,
                "The last-known-good managed sync mutation used the same reviewed device-code or browser/device-code renewal window.",
                vec![
                    workflow_dependency("workflow:managed.sync.inspect", "Inspect managed sync posture"),
                    workflow_dependency("workflow:managed.sync.mutate", "Mutate managed sync state"),
                ],
                "Renew the managed sync auth window before mutating remote state",
                SecretBoundaryRepairOwnerClass::ServiceOperator,
                SecretBoundaryDoctorProbeFamilyClass::TrustIdentityPolicy,
                "doctor.finding.secret_boundary.managed.device_code_renewal_required",
                "repair_candidate:secret_boundary.managed.renew_sync_auth_window",
                "support.lineage.secret_boundary.managed.sync_plane.device_code_renewal_required",
            )],
            repair_path: "Reissue the sync-plane credential or complete the browser/device-code return path before mutating managed sync state.".to_owned(),
            local_safe_behavior:
                "Local history, offline packets, and offboarding exports stay available while managed sync auth is repaired."
                    .to_owned(),
        },
    ];

    let qualification_rows = qualification_rows_for_surface_rows(&surface_rows);
    let current_qualification_count = qualification_rows
        .iter()
        .filter(|row| row.displayed_label.is_current())
        .count();
    let narrowed_qualification_count = qualification_rows
        .len()
        .saturating_sub(current_qualification_count);
    let qualification_row_refs = qualification_rows
        .iter()
        .map(|row| row.qualification_row_id.clone())
        .collect::<Vec<_>>();

    let consumer_projections = vec![
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::DocsHelp,
            surface_ref: M5_SECRET_BOUNDARY_DEPTH_DOC_REF.to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs: qualification_row_refs.clone(),
            current_qualification_count,
            narrowed_qualification_count,
            notes: "Docs and help quote the same matrix row ids and credential-mode vocabulary instead of restating connected status.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::HelpAbout,
            surface_ref: "docs/help/m5-secret-boundary-evidence-index.md".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs: qualification_row_refs.clone(),
            current_qualification_count,
            narrowed_qualification_count,
            notes: "Help/About projects the same qualification rows and current-vs-narrowed counts used by diagnostics and release truth.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::AdminDocs,
            surface_ref: "docs/admin/m5-secret-boundary-evidence-index.md".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs: qualification_row_refs.clone(),
            current_qualification_count,
            narrowed_qualification_count,
            notes: "Admin docs reference the same qualification rows, proof freshness, and narrow reasons instead of summarizing secret maturity manually.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::Diagnostics,
            surface_ref: "crates/aureline-shell/src/secret_broker_beta/mod.rs".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs: qualification_row_refs.clone(),
            current_qualification_count,
            narrowed_qualification_count,
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
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs: qualification_row_refs.clone(),
            current_qualification_count,
            narrowed_qualification_count,
            notes: "Support export preserves matrix ids, default modes, export posture, and repair owner while excluding raw secret bodies and raw handle ids.".to_owned(),
        },
        SecretBoundaryConsumerProjection {
            surface: SecretBoundaryConsumerSurface::ReleasePublicTruth,
            surface_ref: "artifacts/security/m5/m5-secret-boundary-depth.md".to_owned(),
            matrix_id: matrix_id.clone(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            shows_matrix_row_ids: true,
            uses_shared_vocabulary: true,
            evidence_index_ref: evidence_index_ref(),
            qualification_row_refs,
            current_qualification_count,
            narrowed_qualification_count,
            notes: "Release/public-truth surfaces publish only the checked matrix id, row ids, and summary vocabulary; they never widen a row with ad hoc prose.".to_owned(),
        },
    ];

    let artifact_export_rules = vec![
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::Profiles,
            "crates/aureline-workspace/src/profiles/mod.rs",
            SecretBoundaryExportPostureClass::AliasOnly,
            "Portable profile exported without raw secret material; rebind required on import.",
            "Sensitive values excluded: profile exports keep credential aliases, handle classes, source labels, and acting identity only.",
            true,
            false,
            "Bind a new credential handle for this profile",
            "Profile import refused to deserialize prior secret material; a fresh handle bind is required.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::WorkflowBundles,
            "crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs",
            SecretBoundaryExportPostureClass::MetadataOnly,
            "Workflow bundle exported with credential metadata only; rebind required before install.",
            "Sensitive values excluded: workflow bundles keep credential aliases, handle classes, source labels, and consumer identity only.",
            true,
            false,
            "Review bundle auth bindings before install",
            "Bundle install refused to reuse exported secret material; review and bind current handles first.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::PortableStatePackages,
            "crates/aureline-workspace/src/state_packages/mod.rs",
            SecretBoundaryExportPostureClass::MetadataOnly,
            "Portable-state package omits raw secret material and live authority; rebind required on restore.",
            "Sensitive values excluded: portable-state packages keep credential aliases, handle classes, source labels, and consumer identity only.",
            true,
            true,
            "Rebind the required authority before restore",
            "Restore paused because the exported package preserved only handle metadata; raw authority did not travel.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::Recipes,
            "crates/aureline-ai/src/implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay/mod.rs",
            SecretBoundaryExportPostureClass::RedactedSupportExport,
            "Recipe export omitted raw credentials; rebind required before replay.",
            "Sensitive values excluded: recipe exports keep credential aliases, handle classes, source labels, and consumer identity only.",
            true,
            true,
            "Rebind recipe credentials before replay",
            "Recipe replay refused to reuse exported secret material; a new handle or delegated grant must be chosen.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::SupportBundles,
            "crates/aureline-support/src/export_review/mod.rs",
            SecretBoundaryExportPostureClass::RedactedSupportExport,
            "Support bundle exported with omission markers for secret material.",
            "Sensitive values excluded: support bundles keep credential aliases, handle classes, source labels, and consumer identity only.",
            false,
            true,
            "Open the typed repair or rebind flow",
            "Support replay keeps the omission visible and requires a fresh bind before any credentialed action resumes.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::AiEvidencePackets,
            "crates/aureline-ai/src/finalize_ai_evidence_packets/mod.rs",
            SecretBoundaryExportPostureClass::RedactedSupportExport,
            "AI evidence packet exported with redaction evidence and typed rebind posture.",
            "Sensitive values excluded: AI evidence packets keep credential aliases, handle classes, source labels, and consumer identity only.",
            false,
            true,
            "Rerun with reviewed credential bindings",
            "Replay is incomplete because the evidence packet preserved only handle metadata; raw credentials were intentionally omitted.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::IncidentExports,
            "crates/aureline-support/src/incident_workspace/mod.rs",
            SecretBoundaryExportPostureClass::RedactedSupportExport,
            "Incident export omitted secret-bearing material and preserved the missing-span marker.",
            "Sensitive values excluded: incident exports keep credential aliases, handle classes, source labels, and consumer identity only.",
            true,
            true,
            "Rebind credentials in the incident workspace",
            "Incident replay stops at a typed rebind step; the original secret material was never serialized into the export.",
        ),
        artifact_export_rule(
            SecretBoundaryArtifactFamilyClass::OffboardingExports,
            "crates/aureline-settings/src/stabilize_profile_sync_snapshot_backup_restore/model.rs",
            SecretBoundaryExportPostureClass::MetadataOnly,
            "Offboarding export omitted secret-bearing material and preserved bind requirements.",
            "Sensitive values excluded: offboarding exports keep credential aliases, handle classes, source labels, and consumer identity only.",
            true,
            false,
            "Bind replacement credentials on the destination device",
            "Offboarding import refused to deserialize prior secret material; only rebind-safe metadata crossed the boundary.",
        ),
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
            "crates/aureline-workspace/src/profiles/mod.rs".to_owned(),
            "crates/aureline-workspace/src/m5_workflow_bundle_manifests/mod.rs".to_owned(),
            "crates/aureline-workspace/src/state_packages/mod.rs".to_owned(),
            "crates/aureline-support/src/export_review/mod.rs".to_owned(),
            "crates/aureline-ai/src/finalize_ai_evidence_packets/mod.rs".to_owned(),
            "crates/aureline-settings/src/stabilize_profile_sync_snapshot_backup_restore/model.rs".to_owned(),
            "crates/aureline-provider/src/account_scope/mod.rs".to_owned(),
            "crates/aureline-provider/src/route_resolution/mod.rs".to_owned(),
            "crates/aureline-review/src/add_remote_preview_route_lifecycle_expiry_target_identity_and_preview_runtime_trust_disclosure/mod.rs".to_owned(),
            "crates/aureline-remote/src/managed_workspace_lifecycle/mod.rs".to_owned(),
        ],
        surface_rows,
        qualification_rows,
        consumer_projections,
        artifact_export_rules,
        summary: SecretBoundarySummary {
            surface_count: 0,
            qualification_count: 0,
            current_qualification_count: 0,
            limited_local_continuity_count: 0,
            support_review_only_count: 0,
            domain_tokens_present: Vec::new(),
            default_credential_mode_tokens_present: Vec::new(),
            projection_mode_tokens_present: Vec::new(),
            consumer_surface_tokens_present: Vec::new(),
            consumer_identity_tokens_present: Vec::new(),
            projection_control_tokens_present: Vec::new(),
            deployment_profile_tokens_present: Vec::new(),
            projection_parity_tokens_present: Vec::new(),
            health_state_tokens_present: Vec::new(),
            repairable_change_tokens_present: Vec::new(),
            artifact_family_tokens_present: Vec::new(),
            omitted_material_tokens_present: Vec::new(),
            qualification_label_tokens_present: Vec::new(),
            proof_freshness_tokens_present: Vec::new(),
            qualification_narrow_reason_tokens_present: Vec::new(),
            raw_secret_values_excluded: false,
            raw_handle_ids_excluded: false,
        },
    };
    packet.summary = packet.recompute_summary();
    packet
}

#[cfg(test)]
mod tests;
