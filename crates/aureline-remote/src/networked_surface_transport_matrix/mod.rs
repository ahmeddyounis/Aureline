//! Freeze the transport-policy, endpoint-class, route-choice, and
//! trust-material matrix for every new network-capable surface.
//!
//! The newer networked surfaces — AI inference gateways, documentation and
//! in-product browser fetchers, generic request/API clients, database and
//! cloud connectors, extension and model registry reads, companion device
//! handoffs, provider mutation lanes, sync and offboarding traffic, and the
//! richer remote preview routes — must all resolve through **one** shared
//! transport-governance vocabulary instead of per-feature proxy or
//! certificate folklore.
//!
//! This module produces a stable matrix proof packet
//! ([`NetworkedSurfaceTransportMatrixPage`]) that names every such surface and
//! freezes, per surface:
//!
//! - the **origin scope** ([`OriginScopeClass`]) — who owns the endpoint,
//! - the **endpoint class** ([`EndpointClass`]) — what kind of endpoint it is,
//! - the **egress class** ([`EgressClass`]) — where traffic is allowed to go,
//! - the **route choice** ([`RouteChoiceClass`]) — how traffic physically
//!   travels (direct, system proxy, manual proxy, PAC-resolved, mirror-first,
//!   or offline; proxy precedence is PAC → manual → system),
//! - the **auth posture** ([`AuthPostureClass`]) — the handle-only credential
//!   shape presented to the endpoint,
//! - the **trust material** ([`TrustMaterialClass`]) — which trust input
//!   anchors host proof,
//! - the **denial vocabulary** ([`DenialReasonClass`]) — the closed set of
//!   typed reasons a request on this surface may be refused, and
//! - the **mirror/offline behavior** ([`MirrorOfflineBehaviorClass`]) — what
//!   the surface does when the primary route is unavailable.
//!
//! The stable claim holds when **all** of the following conditions are
//! verified simultaneously for every covered surface:
//!
//! 1. All required surfaces are covered.
//! 2. No raw private material is present on any record.
//! 3. No surface permits a silent fall-through from a mirror-only or
//!    deny-all profile to the public internet.
//! 4. Any surface that allows offline-deferred or replay queuing restricts
//!    those queues to explicitly idempotent actions.
//! 5. Every surface preserves local-core continuity.
//! 6. Every surface declares trust material and a non-empty trust-proof ref.
//! 7. Every surface declares a non-empty denial vocabulary.
//! 8. Every surface carries fully-typed endpoint, egress, route, and auth
//!    classifications.
//! 9. Every surface whose egress class requires a policy epoch carries a
//!    last-known-good policy epoch ref.
//! 10. Every surface's qualification proof is fresh (or stale only within an
//!     accepted grace window).
//!
//! Three conditions force [`MatrixQualificationClass::Withdrawn`] immediately
//! and cannot be overridden:
//!
//! - a record with `raw_private_material_excluded: false`
//!   ([`MatrixNarrowReasonClass::RawPrivateMaterialExposed`]),
//! - a record with `no_silent_public_fallback: false`
//!   ([`MatrixNarrowReasonClass::SilentPublicFallbackAllowed`]),
//! - a record that queues non-idempotent actions for offline replay
//!   ([`MatrixNarrowReasonClass::NonIdempotentReplayQueued`]).
//!
//! A missing required surface narrows to [`MatrixQualificationClass::Preview`]
//! rather than `Beta` because the coverage gap prevents any verifiable claim
//! for that surface. Stale-beyond-window proofs and the remaining condition
//! gaps narrow to `Beta`, which lets release and support tooling detect and
//! automatically narrow stale or under-qualified rows before publication.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language summary sentences, and
//! opaque refs only. Raw endpoint URLs, raw hostnames, raw ports, raw
//! credentials, raw bearer/session tokens, raw cookie jars, raw private
//! certificate bytes, raw SSH private material, and raw PAC bodies stay
//! outside the support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/network/networked-surface-transport-matrix.md`
//! - Artifact: `artifacts/network/networked-surface-transport-matrix.md`
//! - Schema: `schemas/network/networked_surface_transport_matrix.schema.json`
//! - Contract ref: [`NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF: &str =
    "remote:networked_surface_transport_matrix:v1";

/// Record-kind tag for [`NetworkedSurfaceTransportMatrixPage`] payloads.
pub const NETWORKED_SURFACE_MATRIX_PAGE_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_page_record";

/// Record-kind tag for [`NetworkedSurfaceRecord`] payloads.
pub const NETWORKED_SURFACE_MATRIX_SURFACE_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_surface_record";

/// Record-kind tag for [`NetworkedSurfaceMatrixRow`] payloads.
pub const NETWORKED_SURFACE_MATRIX_ROW_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_row_record";

/// Record-kind tag for [`NetworkedSurfaceMatrixDefect`] payloads.
pub const NETWORKED_SURFACE_MATRIX_DEFECT_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_defect_record";

/// Record-kind tag for [`NetworkedSurfaceMatrixSummary`] payloads.
pub const NETWORKED_SURFACE_MATRIX_SUMMARY_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_summary_record";

/// Record-kind tag for [`NetworkedSurfaceMatrixSupportExport`] payloads.
pub const NETWORKED_SURFACE_MATRIX_SUPPORT_EXPORT_RECORD_KIND: &str =
    "remote_networked_surface_transport_matrix_support_export_record";

/// Repo-relative path of the stable doc for this matrix.
pub const NETWORKED_SURFACE_MATRIX_DOC_REF: &str =
    "docs/network/networked-surface-transport-matrix.md";

/// Repo-relative path of the artifact summary for this matrix.
pub const NETWORKED_SURFACE_MATRIX_ARTIFACT_REF: &str =
    "artifacts/network/networked-surface-transport-matrix.md";

/// Repo-relative path of the canonical evidence index this matrix is bound
/// into for the closeout certification lane.
pub const NETWORKED_SURFACE_MATRIX_EVIDENCE_INDEX_REF: &str =
    "artifacts/release/m5/xt12-evidence-index.md";

/// All required network-capable surfaces in canonical order.
pub const REQUIRED_SURFACES: [SurfaceClass; 9] = [
    SurfaceClass::AiGateway,
    SurfaceClass::DocsBrowserFetcher,
    SurfaceClass::RequestApiClient,
    SurfaceClass::DatabaseCloudConnector,
    SurfaceClass::RegistryRead,
    SurfaceClass::CompanionHandoff,
    SurfaceClass::ProviderMutation,
    SurfaceClass::SyncOffboarding,
    SurfaceClass::RemotePreviewRoute,
];

// ---------------------------------------------------------------------------
// Surface vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the network-capable surfaces this matrix freezes.
///
/// Each variant maps to a family of outbound traffic that newer networked
/// features introduce and that must resolve through the shared
/// transport-governance vocabulary rather than a private proxy or trust stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// AI inference gateway; model requests, completions, and embeddings.
    AiGateway,
    /// Documentation content fetch and in-product browser fetchers.
    DocsBrowserFetcher,
    /// Generic request/REST/API client lane for user- or feature-driven calls.
    RequestApiClient,
    /// Database and cloud data connectors.
    DatabaseCloudConnector,
    /// Extension and model registry reads (browse and metadata fetch).
    RegistryRead,
    /// Companion device handoff traffic.
    CompanionHandoff,
    /// Provider write/mutation lanes (pull requests, issue edits, status posts).
    ProviderMutation,
    /// Sync and offboarding traffic.
    SyncOffboarding,
    /// Richer remote preview routes.
    RemotePreviewRoute,
}

impl SurfaceClass {
    /// Stable closed-vocabulary token recorded in records, schemas, and
    /// exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiGateway => "ai_gateway",
            Self::DocsBrowserFetcher => "docs_browser_fetcher",
            Self::RequestApiClient => "request_api_client",
            Self::DatabaseCloudConnector => "database_cloud_connector",
            Self::RegistryRead => "registry_read",
            Self::CompanionHandoff => "companion_handoff",
            Self::ProviderMutation => "provider_mutation",
            Self::SyncOffboarding => "sync_offboarding",
            Self::RemotePreviewRoute => "remote_preview_route",
        }
    }

    /// Human-readable surface label safe for UI and exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AiGateway => "AI inference gateway",
            Self::DocsBrowserFetcher => "Docs / browser fetcher",
            Self::RequestApiClient => "Request / API client",
            Self::DatabaseCloudConnector => "Database / cloud connector",
            Self::RegistryRead => "Registry read",
            Self::CompanionHandoff => "Companion handoff",
            Self::ProviderMutation => "Provider mutation",
            Self::SyncOffboarding => "Sync / offboarding",
            Self::RemotePreviewRoute => "Remote preview route",
        }
    }
}

// ---------------------------------------------------------------------------
// Origin scope vocabulary
// ---------------------------------------------------------------------------

/// Ownership scope of the endpoint a surface contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginScopeClass {
    /// First-party endpoint operated by the product.
    FirstParty,
    /// Third-party endpoint not operated by the product or the tenant.
    ThirdParty,
    /// Endpoint configured by the user (custom base URL, self-hosted host).
    UserConfigured,
    /// Endpoint controlled by an enterprise/managed tenant administrator.
    ManagedTenant,
    /// Loopback / on-device endpoint with no network egress.
    LoopbackLocal,
}

impl OriginScopeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::ThirdParty => "third_party",
            Self::UserConfigured => "user_configured",
            Self::ManagedTenant => "managed_tenant",
            Self::LoopbackLocal => "loopback_local",
        }
    }
}

// ---------------------------------------------------------------------------
// Endpoint class vocabulary
// ---------------------------------------------------------------------------

/// What kind of endpoint a surface contacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointClass {
    /// AI inference gateway endpoint.
    InferenceGateway,
    /// Static content origin (docs packs, fetched pages, assets).
    ContentOrigin,
    /// Generic REST/HTTP API endpoint.
    RestApi,
    /// Database or data-store endpoint.
    DataStore,
    /// Artifact, extension, or model registry endpoint.
    ArtifactRegistry,
    /// Peer device endpoint reached during a companion handoff.
    PeerDevice,
    /// Version-control / provider host endpoint.
    VcsHost,
    /// Sync service endpoint.
    SyncService,
    /// Remote preview origin endpoint.
    PreviewOrigin,
}

impl EndpointClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InferenceGateway => "inference_gateway",
            Self::ContentOrigin => "content_origin",
            Self::RestApi => "rest_api",
            Self::DataStore => "data_store",
            Self::ArtifactRegistry => "artifact_registry",
            Self::PeerDevice => "peer_device",
            Self::VcsHost => "vcs_host",
            Self::SyncService => "sync_service",
            Self::PreviewOrigin => "preview_origin",
        }
    }
}

// ---------------------------------------------------------------------------
// Egress class vocabulary
// ---------------------------------------------------------------------------

/// Where a surface is permitted to send traffic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EgressClass {
    /// Egress to the public internet is permitted (subject to route choice).
    PublicInternet,
    /// Egress is restricted to a managed/tenant-controlled endpoint.
    ManagedEndpoint,
    /// Egress is restricted to a declared signed mirror only.
    MirrorOnly,
    /// Egress is restricted to loopback / on-device only.
    LoopbackOnly,
    /// No egress is permitted; the surface is air-gapped.
    AirGapped,
}

impl EgressClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicInternet => "public_internet",
            Self::ManagedEndpoint => "managed_endpoint",
            Self::MirrorOnly => "mirror_only",
            Self::LoopbackOnly => "loopback_only",
            Self::AirGapped => "air_gapped",
        }
    }

    /// Returns `true` when this egress class requires a last-known-good
    /// policy epoch ref so the route's governing policy is traceable.
    pub const fn requires_policy_epoch_ref(self) -> bool {
        matches!(
            self,
            Self::PublicInternet | Self::ManagedEndpoint | Self::MirrorOnly
        )
    }

    /// Returns `true` when this egress class confines traffic such that a
    /// silent fall-through to the public internet would be a guardrail
    /// violation (mirror-only, loopback-only, or air-gapped).
    pub const fn is_confined(self) -> bool {
        matches!(
            self,
            Self::MirrorOnly | Self::LoopbackOnly | Self::AirGapped
        )
    }
}

// ---------------------------------------------------------------------------
// Route choice vocabulary
// ---------------------------------------------------------------------------

/// How traffic physically travels for a surface.
///
/// Proxy resolution precedence is PAC → manual → system: a PAC-resolved route
/// wins over a manually-pinned proxy, which wins over the platform system
/// proxy. No surface may ship a private proxy stack or a hidden direct-connect
/// retry outside this vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteChoiceClass {
    /// Direct connection with no proxy.
    Direct,
    /// Routed through the platform system proxy.
    SystemProxy,
    /// Routed through a manually-configured or policy-pinned proxy.
    ManualProxy,
    /// Routed through a proxy resolved by a PAC script.
    PacResolved,
    /// Traffic directed exclusively to a declared signed mirror.
    MirrorFirst,
    /// No outbound traffic; offline.
    Offline,
}

impl RouteChoiceClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::SystemProxy => "system_proxy",
            Self::ManualProxy => "manual_proxy",
            Self::PacResolved => "pac_resolved",
            Self::MirrorFirst => "mirror_first",
            Self::Offline => "offline",
        }
    }

    /// Returns `true` when this route choice traverses a proxy.
    pub const fn is_proxied(self) -> bool {
        matches!(
            self,
            Self::SystemProxy | Self::ManualProxy | Self::PacResolved
        )
    }
}

// ---------------------------------------------------------------------------
// Auth posture vocabulary
// ---------------------------------------------------------------------------

/// The handle-only credential shape a surface presents to its endpoint.
///
/// Every variant denotes a *handle* or reference; no raw credential bytes are
/// ever recorded. Bearer tokens, session cookies, API keys, client
/// certificate bytes, and SSH private material are referenced by opaque
/// broker handle only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthPostureClass {
    /// No credential is presented.
    Anonymous,
    /// A bearer token presented by broker handle.
    BearerTokenHandle,
    /// A delegated OAuth credential presented by broker handle.
    OauthDelegatedHandle,
    /// An API key presented by broker handle.
    ApiKeyHandle,
    /// A client certificate presented by broker handle.
    ClientCertificateHandle,
    /// An SSH key presented by broker handle.
    SshKeyHandle,
    /// A session cookie presented by broker handle.
    SessionCookieHandle,
}

impl AuthPostureClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Anonymous => "anonymous",
            Self::BearerTokenHandle => "bearer_token_handle",
            Self::OauthDelegatedHandle => "oauth_delegated_handle",
            Self::ApiKeyHandle => "api_key_handle",
            Self::ClientCertificateHandle => "client_certificate_handle",
            Self::SshKeyHandle => "ssh_key_handle",
            Self::SessionCookieHandle => "session_cookie_handle",
        }
    }
}

// ---------------------------------------------------------------------------
// Trust material vocabulary
// ---------------------------------------------------------------------------

/// Which trust input anchors host proof for a surface.
///
/// Every variant denotes a trust *input by reference*; no raw certificate
/// bytes, raw CA bundles, or raw known-hosts material are ever recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustMaterialClass {
    /// The platform/OS system trust store.
    SystemTrustStore,
    /// A pinned certificate authority presented by handle.
    PinnedCaHandle,
    /// A managed/tenant trust bundle presented by handle.
    ManagedTrustBundle,
    /// A signed-mirror root presented by handle.
    MirrorRootHandle,
    /// An SSH known-hosts record presented by handle.
    SshKnownHostsHandle,
    /// No TLS is involved (loopback only); trust is the on-device boundary.
    NoTlsLoopback,
}

impl TrustMaterialClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SystemTrustStore => "system_trust_store",
            Self::PinnedCaHandle => "pinned_ca_handle",
            Self::ManagedTrustBundle => "managed_trust_bundle",
            Self::MirrorRootHandle => "mirror_root_handle",
            Self::SshKnownHostsHandle => "ssh_known_hosts_handle",
            Self::NoTlsLoopback => "no_tls_loopback",
        }
    }
}

// ---------------------------------------------------------------------------
// Denial reason vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary for the typed reasons a request on a surface may be
/// refused.
///
/// Each surface declares the subset of denial reasons it can emit so that UI,
/// CLI, diagnostics, and support exports quote the same denial token rather
/// than parsing a free-form error string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialReasonClass {
    /// A transport policy rule blocks the request.
    PolicyBlocked,
    /// The request would exceed the surface's permitted egress class.
    EgressClassForbidden,
    /// No trust proof is available for the endpoint.
    TrustProofMissing,
    /// The trust proof is present but expired.
    TrustProofExpired,
    /// The auth posture presented was rejected by the endpoint or policy.
    AuthPostureRejected,
    /// The selected proxy is unreachable.
    ProxyUnreachable,
    /// The declared mirror root does not match the served mirror.
    MirrorRootMismatch,
    /// The route is offline and no fallback is available.
    OfflineNoFallback,
    /// A non-idempotent action was rejected from an offline/replay queue.
    NonIdempotentReplayRejected,
}

impl DenialReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocked => "policy_blocked",
            Self::EgressClassForbidden => "egress_class_forbidden",
            Self::TrustProofMissing => "trust_proof_missing",
            Self::TrustProofExpired => "trust_proof_expired",
            Self::AuthPostureRejected => "auth_posture_rejected",
            Self::ProxyUnreachable => "proxy_unreachable",
            Self::MirrorRootMismatch => "mirror_root_mismatch",
            Self::OfflineNoFallback => "offline_no_fallback",
            Self::NonIdempotentReplayRejected => "non_idempotent_replay_rejected",
        }
    }
}

// ---------------------------------------------------------------------------
// Mirror/offline behavior vocabulary
// ---------------------------------------------------------------------------

/// What a surface does when its primary route is unavailable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflineBehaviorClass {
    /// Route to a declared signed mirror; deny rather than reach the public
    /// internet when the mirror is unavailable.
    MirrorFirstThenDeny,
    /// Serve previously-cached content; no live egress is attempted.
    CachedOffline,
    /// Operate within a declared offline-grace window on a validated bundle.
    OfflineGrace,
    /// Deny all traffic for this surface.
    DenyAll,
    /// Continue local-core functionality only; the surface is unavailable.
    LocalCoreOnly,
}

impl MirrorOfflineBehaviorClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorFirstThenDeny => "mirror_first_then_deny",
            Self::CachedOffline => "cached_offline",
            Self::OfflineGrace => "offline_grace",
            Self::DenyAll => "deny_all",
            Self::LocalCoreOnly => "local_core_only",
        }
    }
}

// ---------------------------------------------------------------------------
// Proof freshness vocabulary
// ---------------------------------------------------------------------------

/// Freshness of a surface row's qualification proof.
///
/// Release and support tooling set this token from the row's `proof_as_of`
/// against the surface's freshness window. An expired proof narrows the row to
/// beta so stale claims cannot be published as stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofFreshnessClass {
    /// Proof is current and within its freshness window.
    Fresh,
    /// Proof is stale but within an accepted grace window.
    StaleWithinWindow,
    /// Proof has expired beyond its freshness window.
    ExpiredBeyondWindow,
}

impl ProofFreshnessClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::StaleWithinWindow => "stale_within_window",
            Self::ExpiredBeyondWindow => "expired_beyond_window",
        }
    }

    /// Returns `true` when this freshness class still supports a claim.
    pub const fn is_usable(self) -> bool {
        matches!(self, Self::Fresh | Self::StaleWithinWindow)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual
/// surface rows.
///
/// The tier is derived, not asserted: it is computed by comparing the audit
/// defect list against the stability conditions. A caller may never bump a row
/// to `Stable` without a clean audit and complete surface coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixQualificationClass {
    /// All stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required surface has no record; the coverage gap prevents a beta
    /// claim for the missing surface.
    Preview,
    /// A hard guardrail was violated; the packet is withdrawn immediately and
    /// cannot be overridden.
    Withdrawn,
}

impl MatrixQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns `true` when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Returns `true` when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or surface row was narrowed below
/// [`MatrixQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A record carries `raw_private_material_excluded: false`; withdraws the
    /// packet immediately.
    RawPrivateMaterialExposed,
    /// A record permits a silent fall-through to the public internet from a
    /// confined egress class; withdraws the packet immediately.
    SilentPublicFallbackAllowed,
    /// A record queues non-idempotent actions for offline/replay; withdraws
    /// the packet immediately.
    NonIdempotentReplayQueued,
    /// A required surface is not covered by any record; narrows to preview.
    RequiredSurfaceMissing,
    /// A surface does not preserve local-core continuity.
    LocalCoreContinuityNotPreserved,
    /// A surface does not declare trust material or a trust-proof ref.
    TrustMaterialUndeclared,
    /// A surface declares no denial vocabulary.
    DenialVocabularyMissing,
    /// A surface is missing one of its endpoint/egress/route/auth
    /// classifications.
    TransportClassificationIncomplete,
    /// A surface whose egress class requires a policy epoch is missing the
    /// last-known-good policy epoch ref.
    PolicyEpochRefMissing,
    /// A surface's qualification proof has expired beyond its freshness
    /// window.
    ProofStaleBeyondWindow,
}

impl MatrixNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::RawPrivateMaterialExposed => "raw_private_material_exposed",
            Self::SilentPublicFallbackAllowed => "silent_public_fallback_allowed",
            Self::NonIdempotentReplayQueued => "non_idempotent_replay_queued",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::LocalCoreContinuityNotPreserved => "local_core_continuity_not_preserved",
            Self::TrustMaterialUndeclared => "trust_material_undeclared",
            Self::DenialVocabularyMissing => "denial_vocabulary_missing",
            Self::TransportClassificationIncomplete => "transport_classification_incomplete",
            Self::PolicyEpochRefMissing => "policy_epoch_ref_missing",
            Self::ProofStaleBeyondWindow => "proof_stale_beyond_window",
        }
    }

    /// Returns `true` when this reason is a hard guardrail that withdraws the
    /// packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::RawPrivateMaterialExposed
                | Self::SilentPublicFallbackAllowed
                | Self::NonIdempotentReplayQueued
        )
    }

    /// Returns `true` when this reason narrows to preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(self, Self::RequiredSurfaceMissing)
    }
}

// ---------------------------------------------------------------------------
// Surface record (per-surface)
// ---------------------------------------------------------------------------

/// Per-surface transport-governance truth record.
///
/// Each record freezes the origin scope, endpoint class, egress class, route
/// choice, auth posture, trust material, denial vocabulary, and mirror/offline
/// behavior for a single network-capable surface, together with the guardrail
/// flags and the policy-epoch and trust-proof refs that make the surface
/// qualifiable.
///
/// No raw endpoint URLs, raw hostnames, raw ports, raw credentials, raw
/// bearer/session tokens, raw cookie jars, raw private certificate bytes, raw
/// SSH private material, or raw PAC bodies may appear on this record. Only
/// closed-vocabulary tokens, opaque refs, and plain-language summary sentences
/// cross the export boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable surface identifier.
    pub surface: SurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Origin ownership scope for this surface.
    pub origin_scope: OriginScopeClass,
    /// Stable token for [`Self::origin_scope`].
    pub origin_scope_token: String,
    /// Endpoint class this surface contacts.
    pub endpoint_class: EndpointClass,
    /// Stable token for [`Self::endpoint_class`].
    pub endpoint_class_token: String,
    /// Egress class permitted for this surface.
    pub egress_class: EgressClass,
    /// Stable token for [`Self::egress_class`].
    pub egress_class_token: String,
    /// Route choice for this surface.
    pub route_choice: RouteChoiceClass,
    /// Stable token for [`Self::route_choice`].
    pub route_choice_token: String,
    /// Handle-only auth posture presented to the endpoint.
    pub auth_posture: AuthPostureClass,
    /// Stable token for [`Self::auth_posture`].
    pub auth_posture_token: String,
    /// Trust input anchoring host proof for this surface.
    pub trust_material: TrustMaterialClass,
    /// Stable token for [`Self::trust_material`].
    pub trust_material_token: String,
    /// Opaque ref to the trust proof evidence for this surface. Required; an
    /// empty ref narrows the surface to beta.
    pub trust_proof_ref: String,
    /// Closed-vocabulary denial reasons this surface can emit. Required to be
    /// non-empty.
    pub denial_vocabulary: Vec<DenialReasonClass>,
    /// Stable tokens for [`Self::denial_vocabulary`].
    pub denial_vocabulary_tokens: Vec<String>,
    /// Mirror/offline behavior when the primary route is unavailable.
    pub mirror_offline_behavior: MirrorOfflineBehaviorClass,
    /// Stable token for [`Self::mirror_offline_behavior`].
    pub mirror_offline_behavior_token: String,
    /// Opaque ref to the last-known-good policy epoch governing this surface.
    /// Present for egress classes that require it; `None` otherwise.
    pub policy_epoch_ref: Option<String>,
    /// `true` when local-core editing continues regardless of this surface's
    /// availability.
    pub local_core_continuity_preserved: bool,
    /// `true` when no silent fall-through to the public internet is permitted
    /// from a confined egress class. Must be `true` for the stable claim.
    pub no_silent_public_fallback: bool,
    /// `true` when this surface may queue actions for offline-deferred replay.
    pub offline_deferral_allowed: bool,
    /// `true` when only explicitly idempotent actions may enter this surface's
    /// offline/replay queues. Must be `true` whenever
    /// [`Self::offline_deferral_allowed`] is `true`.
    pub replay_idempotent_only: bool,
    /// UTC instant the qualification proof for this surface was captured.
    pub proof_as_of: String,
    /// Freshness of the qualification proof for this surface.
    pub proof_freshness: ProofFreshnessClass,
    /// Stable token for [`Self::proof_freshness`].
    pub proof_freshness_token: String,
    /// `true` when no raw endpoint URL, raw credential, raw private key, raw
    /// cookie, or raw PAC content is present on this record. Must be `true`
    /// for the stable claim.
    pub raw_private_material_excluded: bool,
    /// Plain-language summary safe for UI, support export, and diagnostics.
    pub summary: String,
}

impl NetworkedSurfaceRecord {
    /// Construct a surface record, filling in all token fields from the typed
    /// enum values.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        surface: SurfaceClass,
        origin_scope: OriginScopeClass,
        endpoint_class: EndpointClass,
        egress_class: EgressClass,
        route_choice: RouteChoiceClass,
        auth_posture: AuthPostureClass,
        trust_material: TrustMaterialClass,
        trust_proof_ref: impl Into<String>,
        denial_vocabulary: Vec<DenialReasonClass>,
        mirror_offline_behavior: MirrorOfflineBehaviorClass,
        policy_epoch_ref: Option<impl Into<String>>,
        local_core_continuity_preserved: bool,
        no_silent_public_fallback: bool,
        offline_deferral_allowed: bool,
        replay_idempotent_only: bool,
        proof_as_of: impl Into<String>,
        proof_freshness: ProofFreshnessClass,
        summary: impl Into<String>,
    ) -> Self {
        let denial_vocabulary_tokens = denial_vocabulary
            .iter()
            .map(|d| d.as_str().to_owned())
            .collect();
        Self {
            record_kind: NETWORKED_SURFACE_MATRIX_SURFACE_RECORD_KIND.to_owned(),
            schema_version: NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            surface,
            surface_token: surface.as_str().to_owned(),
            origin_scope,
            origin_scope_token: origin_scope.as_str().to_owned(),
            endpoint_class,
            endpoint_class_token: endpoint_class.as_str().to_owned(),
            egress_class,
            egress_class_token: egress_class.as_str().to_owned(),
            route_choice,
            route_choice_token: route_choice.as_str().to_owned(),
            auth_posture,
            auth_posture_token: auth_posture.as_str().to_owned(),
            trust_material,
            trust_material_token: trust_material.as_str().to_owned(),
            trust_proof_ref: trust_proof_ref.into(),
            denial_vocabulary,
            denial_vocabulary_tokens,
            mirror_offline_behavior,
            mirror_offline_behavior_token: mirror_offline_behavior.as_str().to_owned(),
            policy_epoch_ref: policy_epoch_ref.map(Into::into),
            local_core_continuity_preserved,
            no_silent_public_fallback,
            offline_deferral_allowed,
            replay_idempotent_only,
            proof_as_of: proof_as_of.into(),
            proof_freshness,
            proof_freshness_token: proof_freshness.as_str().to_owned(),
            raw_private_material_excluded: true,
            summary: summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Matrix snapshot (aggregate of all surface records)
// ---------------------------------------------------------------------------

/// Aggregate of all surface transport-governance records.
///
/// The snapshot carries one [`NetworkedSurfaceRecord`] per network-capable
/// surface. A snapshot missing any required surface causes the matrix proof
/// packet to narrow to `Preview`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceMatrixSnapshot {
    /// All surface records in the snapshot.
    pub records: Vec<NetworkedSurfaceRecord>,
}

impl NetworkedSurfaceMatrixSnapshot {
    /// Returns the record for the given surface, if present.
    pub fn record_for_surface(&self, surface: SurfaceClass) -> Option<&NetworkedSurfaceRecord> {
        self.records.iter().find(|r| r.surface == surface)
    }

    /// Returns the set of surface tokens covered by this snapshot.
    pub fn covered_surface_tokens(&self) -> BTreeSet<&str> {
        self.records
            .iter()
            .map(|r| r.surface_token.as_str())
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Matrix row (per-surface stability row)
// ---------------------------------------------------------------------------

/// Stability qualification for one surface in the matrix proof packet.
///
/// Each row is derived from a single [`NetworkedSurfaceRecord`] in the
/// snapshot. The qualification is computed from the record against the
/// stability conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceMatrixRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface token for this row.
    pub surface_token: String,
    /// Origin scope token from the surface record.
    pub origin_scope_token: String,
    /// Endpoint class token from the surface record.
    pub endpoint_class_token: String,
    /// Egress class token from the surface record.
    pub egress_class_token: String,
    /// Route choice token from the surface record.
    pub route_choice_token: String,
    /// Auth posture token from the surface record.
    pub auth_posture_token: String,
    /// Trust material token from the surface record.
    pub trust_material_token: String,
    /// Mirror/offline behavior token from the surface record.
    pub mirror_offline_behavior_token: String,
    /// Denial vocabulary tokens from the surface record.
    pub denial_vocabulary_tokens: Vec<String>,
    /// `true` when local-core continuity is preserved.
    pub local_core_continuity_preserved: bool,
    /// `true` when no silent public fall-through is permitted.
    pub no_silent_public_fallback: bool,
    /// `true` when offline/replay queuing is restricted to idempotent actions
    /// (always `true` when offline deferral is disallowed).
    pub replay_idempotent_only: bool,
    /// `true` when a policy epoch ref is present.
    pub policy_epoch_present: bool,
    /// Proof freshness token from the surface record.
    pub proof_freshness_token: String,
    /// `true` when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate banner emitted with the matrix page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct NetworkedSurfaceMatrixSummary {
    /// Total row count (one row per surface in the snapshot).
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Surface tokens covered by the snapshot.
    pub surfaces_covered: Vec<String>,
    /// Number of surfaces with explicit local-core continuity preservation.
    pub local_core_continuity_preserved_count: usize,
    /// Number of surfaces with a present policy epoch ref.
    pub policy_epoch_present_count: usize,
    /// Number of surfaces with a fresh (or grace-window) proof.
    pub usable_proof_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl NetworkedSurfaceMatrixSummary {
    fn from_rows(
        rows: &[NetworkedSurfaceMatrixRow],
        snapshot: &NetworkedSurfaceMatrixSnapshot,
        defects: &[NetworkedSurfaceMatrixDefect],
    ) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
        }
        // The overall tier is derived from the full defect list, not just the
        // per-row qualifications, so a missing required surface (which has no
        // row) still narrows the page to preview.
        let has_withdrawal = defects
            .iter()
            .any(|d| d.narrow_reason.is_withdrawal_reason());
        let has_preview = defects.iter().any(|d| d.narrow_reason.is_preview_reason());
        let overall = if has_withdrawal || withdrawn > 0 {
            MatrixQualificationClass::Withdrawn
        } else if has_preview || preview > 0 {
            MatrixQualificationClass::Preview
        } else if !defects.is_empty() || beta > 0 {
            MatrixQualificationClass::Beta
        } else {
            MatrixQualificationClass::Stable
        };
        let surfaces_covered: Vec<String> = snapshot
            .records
            .iter()
            .map(|r| r.surface_token.clone())
            .collect();
        let local_core_continuity_preserved_count = rows
            .iter()
            .filter(|r| r.local_core_continuity_preserved)
            .count();
        let policy_epoch_present_count = rows.iter().filter(|r| r.policy_epoch_present).count();
        let usable_proof_count = snapshot
            .records
            .iter()
            .filter(|r| r.proof_freshness.is_usable())
            .count();
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            surfaces_covered,
            local_core_continuity_preserved_count,
            policy_epoch_present_count,
            usable_proof_count,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the matrix page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceMatrixDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: MatrixNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (surface token or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl NetworkedSurfaceMatrixDefect {
    fn new(
        narrow_reason: MatrixNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: NETWORKED_SURFACE_MATRIX_DEFECT_RECORD_KIND.to_owned(),
            schema_version: NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "remote:defect:networked-surface-transport-matrix:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Matrix page (proof packet)
// ---------------------------------------------------------------------------

/// Stable matrix proof packet for the network-capable surfaces.
///
/// The packet is the single inspectable record that freezes and proves the
/// shared transport-governance vocabulary across every network-capable
/// surface. Dashboards, docs, Help/About surfaces, support exports, release
/// tooling, and diagnostics should ingest this packet rather than cloning
/// subsystem-specific status strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceTransportMatrixPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Repo-relative ref to the canonical evidence index this matrix binds into.
    pub evidence_index_ref: String,
    /// Aggregate summary derived from all rows.
    pub summary: NetworkedSurfaceMatrixSummary,
    /// Per-surface stability rows.
    pub rows: Vec<NetworkedSurfaceMatrixRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<NetworkedSurfaceMatrixDefect>,
    /// The surface matrix snapshot embedded as evidence.
    pub matrix_snapshot: NetworkedSurfaceMatrixSnapshot,
}

impl NetworkedSurfaceTransportMatrixPage {
    /// Build the matrix page from a surface matrix snapshot.
    ///
    /// Rows are derived per surface, and the qualification for each is
    /// computed from the combined audit of the whole snapshot.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        matrix_snapshot: NetworkedSurfaceMatrixSnapshot,
    ) -> Self {
        let defects = audit_snapshot(&matrix_snapshot);
        let rows = derive_matrix_rows(&matrix_snapshot, &defects);
        let summary = NetworkedSurfaceMatrixSummary::from_rows(&rows, &matrix_snapshot, &defects);
        Self {
            record_kind: NETWORKED_SURFACE_MATRIX_PAGE_RECORD_KIND.to_owned(),
            schema_version: NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            evidence_index_ref: NETWORKED_SURFACE_MATRIX_EVIDENCE_INDEX_REF.to_owned(),
            summary,
            rows,
            defects,
            matrix_snapshot,
        }
    }

    /// Returns `true` when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token == MatrixQualificationClass::Stable.as_str()
    }

    /// Returns `true` when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// Returns `true` when all required surfaces are covered.
    pub fn covers_all_required_surfaces(&self) -> bool {
        let covered = self.matrix_snapshot.covered_surface_tokens();
        REQUIRED_SURFACES
            .iter()
            .all(|surface| covered.contains(surface.as_str()))
    }

    /// Returns `true` when every surface record preserves local-core
    /// continuity.
    pub fn all_surfaces_preserve_local_core_continuity(&self) -> bool {
        self.matrix_snapshot
            .records
            .iter()
            .all(|r| r.local_core_continuity_preserved)
    }

    /// Returns `true` when no surface permits a silent fall-through to the
    /// public internet.
    pub fn no_surface_allows_silent_public_fallback(&self) -> bool {
        self.matrix_snapshot
            .records
            .iter()
            .all(|r| r.no_silent_public_fallback)
    }

    /// Returns `true` when every surface that defers actions for replay
    /// restricts those queues to idempotent actions.
    pub fn replay_queues_are_idempotent_only(&self) -> bool {
        self.matrix_snapshot
            .records
            .iter()
            .all(|r| !r.offline_deferral_allowed || r.replay_idempotent_only)
    }

    /// Returns `true` when every egress class that requires a policy epoch ref
    /// carries one.
    pub fn egress_classes_have_policy_epoch_refs(&self) -> bool {
        self.matrix_snapshot.records.iter().all(|r| {
            if r.egress_class.requires_policy_epoch_ref() {
                r.policy_epoch_ref.is_some()
            } else {
                true
            }
        })
    }

    /// Returns `true` when every surface declares trust material with a
    /// non-empty trust-proof ref and a non-empty denial vocabulary.
    pub fn all_surfaces_declare_trust_and_denial(&self) -> bool {
        self.matrix_snapshot
            .records
            .iter()
            .all(|r| !r.trust_proof_ref.is_empty() && !r.denial_vocabulary.is_empty())
    }

    /// Returns `true` when every surface's qualification proof is usable
    /// (fresh or stale only within an accepted grace window).
    pub fn all_surface_proofs_usable(&self) -> bool {
        self.matrix_snapshot
            .records
            .iter()
            .all(|r| r.proof_freshness.is_usable())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that carries the matrix page plus a metadata-safe
/// defect roll-up.
///
/// No raw endpoint URLs, raw hostnames, raw credentials, raw cookies, or raw
/// private key material may appear in this export. Only closed-vocabulary
/// tokens, opaque refs, counts, and plain-language summary sentences cross the
/// boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetworkedSurfaceMatrixSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The matrix page embedded as evidence.
    pub page: NetworkedSurfaceTransportMatrixPage,
    /// Narrow-reason class values present in the page's defect list.
    pub narrow_reasons_present: Vec<MatrixNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// `true` when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl NetworkedSurfaceMatrixSupportExport {
    /// Wrap a matrix page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: NetworkedSurfaceTransportMatrixPage,
    ) -> Self {
        let mut reasons: Vec<MatrixNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: NETWORKED_SURFACE_MATRIX_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION,
            shared_contract_ref: NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions (public API)
// ---------------------------------------------------------------------------

/// Re-run the matrix audit over the snapshot embedded in a page.
pub fn audit_networked_surface_matrix_page(
    page: &NetworkedSurfaceTransportMatrixPage,
) -> Vec<NetworkedSurfaceMatrixDefect> {
    audit_snapshot(&page.matrix_snapshot)
}

/// Validate a matrix page; returns `Ok` when the audit is clean.
pub fn validate_networked_surface_matrix_page(
    page: &NetworkedSurfaceTransportMatrixPage,
) -> Result<(), Vec<NetworkedSurfaceMatrixDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal audit logic
// ---------------------------------------------------------------------------

fn audit_snapshot(snapshot: &NetworkedSurfaceMatrixSnapshot) -> Vec<NetworkedSurfaceMatrixDefect> {
    let mut defects: Vec<NetworkedSurfaceMatrixDefect> = Vec::new();

    // Hard guardrails first — any one of these withdraws the packet and makes
    // no further check meaningful.
    for record in &snapshot.records {
        if !record.raw_private_material_excluded {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::RawPrivateMaterialExposed,
                record.surface_token.clone(),
                format!(
                    "surface '{}' has raw_private_material_excluded: false; packet is withdrawn",
                    record.surface_token
                ),
            ));
            return defects;
        }
        if !record.no_silent_public_fallback {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::SilentPublicFallbackAllowed,
                record.surface_token.clone(),
                format!(
                    "surface '{}' permits a silent fall-through to the public internet; packet is withdrawn",
                    record.surface_token
                ),
            ));
            return defects;
        }
        if record.offline_deferral_allowed && !record.replay_idempotent_only {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::NonIdempotentReplayQueued,
                record.surface_token.clone(),
                format!(
                    "surface '{}' queues non-idempotent actions for offline replay; packet is withdrawn",
                    record.surface_token
                ),
            ));
            return defects;
        }
    }

    let covered: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.surface_token.as_str())
        .collect();

    // Coverage check: all required surfaces must be present.
    for required_surface in &REQUIRED_SURFACES {
        if !covered.contains(required_surface.as_str()) {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::RequiredSurfaceMissing,
                required_surface.as_str(),
                format!(
                    "required surface '{}' has no matrix record; packet is narrowed to preview",
                    required_surface.as_str()
                ),
            ));
        }
    }

    // Per-surface checks.
    for record in &snapshot.records {
        if !record.local_core_continuity_preserved {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::LocalCoreContinuityNotPreserved,
                record.surface_token.clone(),
                format!(
                    "surface '{}' does not preserve local-core continuity; local work may be blocked",
                    record.surface_token
                ),
            ));
        }

        if record.trust_proof_ref.is_empty() || record.trust_material_token.is_empty() {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::TrustMaterialUndeclared,
                record.surface_token.clone(),
                format!(
                    "surface '{}' does not declare trust material with a trust-proof ref; host proof is unverifiable",
                    record.surface_token
                ),
            ));
        }

        if record.denial_vocabulary.is_empty() || record.denial_vocabulary_tokens.is_empty() {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::DenialVocabularyMissing,
                record.surface_token.clone(),
                format!(
                    "surface '{}' declares no denial vocabulary; denial reasons cannot be quoted consistently",
                    record.surface_token
                ),
            ));
        }

        if record.endpoint_class_token.is_empty()
            || record.egress_class_token.is_empty()
            || record.route_choice_token.is_empty()
            || record.auth_posture_token.is_empty()
        {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::TransportClassificationIncomplete,
                record.surface_token.clone(),
                format!(
                    "surface '{}' is missing one of endpoint/egress/route/auth classification tokens",
                    record.surface_token
                ),
            ));
        }

        if record.egress_class.requires_policy_epoch_ref() && record.policy_epoch_ref.is_none() {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::PolicyEpochRefMissing,
                record.surface_token.clone(),
                format!(
                    "surface '{}' ({}) has no policy_epoch_ref; policy epoch must be traceable",
                    record.surface_token, record.egress_class_token
                ),
            ));
        }

        if !record.proof_freshness.is_usable() {
            defects.push(NetworkedSurfaceMatrixDefect::new(
                MatrixNarrowReasonClass::ProofStaleBeyondWindow,
                record.surface_token.clone(),
                format!(
                    "surface '{}' qualification proof is {} (as of {}); stable claim is narrowed to beta",
                    record.surface_token, record.proof_freshness_token, record.proof_as_of
                ),
            ));
        }
    }

    defects
}

fn derive_matrix_rows(
    snapshot: &NetworkedSurfaceMatrixSnapshot,
    page_defects: &[NetworkedSurfaceMatrixDefect],
) -> Vec<NetworkedSurfaceMatrixRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_preview_reason());

    let overall_narrow_reason = if has_withdrawal {
        page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(MatrixNarrowReasonClass::RawPrivateMaterialExposed)
    } else if has_preview {
        MatrixNarrowReasonClass::RequiredSurfaceMissing
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        MatrixNarrowReasonClass::NotNarrowed
    };

    snapshot
        .records
        .iter()
        .map(|record| {
            let row_narrow =
                find_surface_narrow_reason(record, page_defects, overall_narrow_reason);
            let row_qual = qualification_for_reason(row_narrow);
            let summary = build_row_summary(&record.surface_token, &row_qual, row_narrow);
            NetworkedSurfaceMatrixRow {
                record_kind: NETWORKED_SURFACE_MATRIX_ROW_RECORD_KIND.to_owned(),
                schema_version: NETWORKED_SURFACE_MATRIX_SCHEMA_VERSION,
                shared_contract_ref: NETWORKED_SURFACE_MATRIX_SHARED_CONTRACT_REF.to_owned(),
                surface_token: record.surface_token.clone(),
                origin_scope_token: record.origin_scope_token.clone(),
                endpoint_class_token: record.endpoint_class_token.clone(),
                egress_class_token: record.egress_class_token.clone(),
                route_choice_token: record.route_choice_token.clone(),
                auth_posture_token: record.auth_posture_token.clone(),
                trust_material_token: record.trust_material_token.clone(),
                mirror_offline_behavior_token: record.mirror_offline_behavior_token.clone(),
                denial_vocabulary_tokens: record.denial_vocabulary_tokens.clone(),
                local_core_continuity_preserved: record.local_core_continuity_preserved,
                no_silent_public_fallback: record.no_silent_public_fallback,
                replay_idempotent_only: !record.offline_deferral_allowed
                    || record.replay_idempotent_only,
                policy_epoch_present: record.policy_epoch_ref.is_some(),
                proof_freshness_token: record.proof_freshness_token.clone(),
                raw_private_material_excluded: record.raw_private_material_excluded,
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn qualification_for_reason(reason: MatrixNarrowReasonClass) -> MatrixQualificationClass {
    if reason.is_withdrawal_reason() {
        MatrixQualificationClass::Withdrawn
    } else if reason.is_preview_reason() {
        MatrixQualificationClass::Preview
    } else if reason != MatrixNarrowReasonClass::NotNarrowed {
        MatrixQualificationClass::Beta
    } else {
        MatrixQualificationClass::Stable
    }
}

fn find_surface_narrow_reason(
    record: &NetworkedSurfaceRecord,
    page_defects: &[NetworkedSurfaceMatrixDefect],
    overall_narrow_reason: MatrixNarrowReasonClass,
) -> MatrixNarrowReasonClass {
    // A withdrawal reason taints the whole packet; every row is withdrawn.
    if overall_narrow_reason.is_withdrawal_reason() {
        return overall_narrow_reason;
    }
    // Otherwise a surface-specific defect governs the row.
    if let Some(defect) = page_defects
        .iter()
        .find(|d| d.source == record.surface_token)
    {
        return defect.narrow_reason;
    }
    MatrixNarrowReasonClass::NotNarrowed
}

fn build_row_summary(
    surface_token: &str,
    qual: &MatrixQualificationClass,
    narrow_reason: MatrixNarrowReasonClass,
) -> String {
    match qual {
        MatrixQualificationClass::Stable => format!(
            "Surface '{}' qualifies stable: origin scope, endpoint class, egress class, \
             route choice, auth posture, trust material, denial vocabulary, and \
             mirror/offline behavior are all frozen; guardrails hold and the proof is fresh.",
            surface_token
        ),
        _ => format!(
            "Surface '{}' narrowed to {} ({}): see defect list for details.",
            surface_token,
            qual.as_str(),
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page produces zero defects: all required surfaces are covered,
/// no raw private material is present, no surface allows a silent public
/// fall-through, every replay queue is idempotent-only, every surface
/// preserves local-core continuity, declares trust material and a denial
/// vocabulary, carries fully-typed classifications, carries a policy epoch ref
/// where required, and has a fresh proof.
pub fn seeded_networked_surface_matrix_page() -> NetworkedSurfaceTransportMatrixPage {
    NetworkedSurfaceTransportMatrixPage::new(
        "remote:networked_surface_transport_matrix:default",
        "Networked-surface transport, endpoint, route, and trust matrix — stable packet",
        "2026-06-01T00:00:00Z",
        seeded_networked_surface_matrix_snapshot(),
    )
}

/// Build the seeded surface matrix snapshot used by the seeded page.
///
/// Each required surface is represented with a fully-typed, clean record that
/// passes all stability conditions.
pub fn seeded_networked_surface_matrix_snapshot() -> NetworkedSurfaceMatrixSnapshot {
    let proof_as_of = "2026-06-01T00:00:00Z";
    NetworkedSurfaceMatrixSnapshot {
        records: vec![
            // AI inference gateway — managed endpoint, direct, bearer handle.
            NetworkedSurfaceRecord::new(
                SurfaceClass::AiGateway,
                OriginScopeClass::ManagedTenant,
                EndpointClass::InferenceGateway,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::Direct,
                AuthPostureClass::BearerTokenHandle,
                TrustMaterialClass::ManagedTrustBundle,
                "trust:ai_gateway:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::TrustProofExpired,
                    DenialReasonClass::EgressClassForbidden,
                ],
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                Some("epoch:ai_gateway:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "AI inference gateway: managed-endpoint egress, direct route, bearer-token \
                 handle, managed trust bundle; offline behavior is local-core only; \
                 local editing continues without the gateway.",
            ),
            // Docs / browser fetcher — public internet, system proxy, anonymous.
            NetworkedSurfaceRecord::new(
                SurfaceClass::DocsBrowserFetcher,
                OriginScopeClass::ThirdParty,
                EndpointClass::ContentOrigin,
                EgressClass::PublicInternet,
                RouteChoiceClass::SystemProxy,
                AuthPostureClass::Anonymous,
                TrustMaterialClass::SystemTrustStore,
                "trust:docs_browser_fetcher:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::ProxyUnreachable,
                    DenialReasonClass::OfflineNoFallback,
                ],
                MirrorOfflineBehaviorClass::CachedOffline,
                Some("epoch:docs_browser_fetcher:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Docs / browser fetcher: public-internet egress via system proxy, anonymous, \
                 system trust store; serves cached content offline; browsing of fetched \
                 content continues without connectivity.",
            ),
            // Request / API client — user-configured, manual proxy, API key handle.
            NetworkedSurfaceRecord::new(
                SurfaceClass::RequestApiClient,
                OriginScopeClass::UserConfigured,
                EndpointClass::RestApi,
                EgressClass::PublicInternet,
                RouteChoiceClass::ManualProxy,
                AuthPostureClass::ApiKeyHandle,
                TrustMaterialClass::SystemTrustStore,
                "trust:request_api_client:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::ProxyUnreachable,
                    DenialReasonClass::TrustProofMissing,
                ],
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:request_api_client:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Request / API client: public-internet egress via manual proxy, API-key handle, \
                 system trust store; denies all when offline; local work continues unaffected.",
            ),
            // Database / cloud connector — user-configured, direct, client cert handle.
            NetworkedSurfaceRecord::new(
                SurfaceClass::DatabaseCloudConnector,
                OriginScopeClass::UserConfigured,
                EndpointClass::DataStore,
                EgressClass::PublicInternet,
                RouteChoiceClass::Direct,
                AuthPostureClass::ClientCertificateHandle,
                TrustMaterialClass::PinnedCaHandle,
                "trust:database_cloud_connector:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::TrustProofMissing,
                    DenialReasonClass::TrustProofExpired,
                ],
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:database_cloud_connector:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Database / cloud connector: public-internet egress, direct route, client- \
                 certificate handle, pinned CA; denies all when offline; local work continues.",
            ),
            // Registry read — first party, PAC-resolved, anonymous, mirror-first.
            NetworkedSurfaceRecord::new(
                SurfaceClass::RegistryRead,
                OriginScopeClass::FirstParty,
                EndpointClass::ArtifactRegistry,
                EgressClass::MirrorOnly,
                RouteChoiceClass::MirrorFirst,
                AuthPostureClass::Anonymous,
                TrustMaterialClass::MirrorRootHandle,
                "trust:registry_read:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::MirrorRootMismatch,
                    DenialReasonClass::OfflineNoFallback,
                    DenialReasonClass::EgressClassForbidden,
                ],
                MirrorOfflineBehaviorClass::MirrorFirstThenDeny,
                Some("epoch:registry_read:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Registry read: mirror-only egress, mirror-first route, anonymous, mirror-root \
                 trust; routes to the signed mirror then denies rather than reaching the public \
                 internet; installed items continue without registry access.",
            ),
            // Companion handoff — loopback, direct, session cookie handle.
            NetworkedSurfaceRecord::new(
                SurfaceClass::CompanionHandoff,
                OriginScopeClass::LoopbackLocal,
                EndpointClass::PeerDevice,
                EgressClass::LoopbackOnly,
                RouteChoiceClass::Direct,
                AuthPostureClass::SessionCookieHandle,
                TrustMaterialClass::NoTlsLoopback,
                "trust:companion_handoff:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::EgressClassForbidden,
                ],
                MirrorOfflineBehaviorClass::LocalCoreOnly,
                None::<String>,
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Companion handoff: loopback-only egress, direct, session-cookie handle, \
                 on-device trust boundary; offline behavior is local-core only; the desktop \
                 continues without the companion.",
            ),
            // Provider mutation — managed, direct, OAuth delegated handle, no replay.
            NetworkedSurfaceRecord::new(
                SurfaceClass::ProviderMutation,
                OriginScopeClass::ManagedTenant,
                EndpointClass::VcsHost,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::Direct,
                AuthPostureClass::OauthDelegatedHandle,
                TrustMaterialClass::ManagedTrustBundle,
                "trust:provider_mutation:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::NonIdempotentReplayRejected,
                    DenialReasonClass::OfflineNoFallback,
                ],
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:provider_mutation:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Provider mutation: managed-endpoint egress, direct route, delegated-OAuth \
                 handle, managed trust bundle; mutations are not queued for offline replay; \
                 denies all when offline.",
            ),
            // Sync / offboarding — managed, direct, bearer handle, idempotent replay.
            NetworkedSurfaceRecord::new(
                SurfaceClass::SyncOffboarding,
                OriginScopeClass::ManagedTenant,
                EndpointClass::SyncService,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::Direct,
                AuthPostureClass::BearerTokenHandle,
                TrustMaterialClass::ManagedTrustBundle,
                "trust:sync_offboarding:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::NonIdempotentReplayRejected,
                    DenialReasonClass::OfflineNoFallback,
                ],
                MirrorOfflineBehaviorClass::OfflineGrace,
                Some("epoch:sync_offboarding:2026-06-01"),
                true,
                true,
                true,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Sync / offboarding: managed-endpoint egress, direct route, bearer-token handle, \
                 managed trust bundle; defers only idempotent sync actions for replay within an \
                 offline-grace window; local data is retained.",
            ),
            // Remote preview route — first party, direct, bearer handle, deny-all offline.
            NetworkedSurfaceRecord::new(
                SurfaceClass::RemotePreviewRoute,
                OriginScopeClass::FirstParty,
                EndpointClass::PreviewOrigin,
                EgressClass::ManagedEndpoint,
                RouteChoiceClass::Direct,
                AuthPostureClass::BearerTokenHandle,
                TrustMaterialClass::ManagedTrustBundle,
                "trust:remote_preview_route:proof:2026-06-01",
                vec![
                    DenialReasonClass::PolicyBlocked,
                    DenialReasonClass::AuthPostureRejected,
                    DenialReasonClass::EgressClassForbidden,
                    DenialReasonClass::OfflineNoFallback,
                ],
                MirrorOfflineBehaviorClass::DenyAll,
                Some("epoch:remote_preview_route:2026-06-01"),
                true,
                true,
                false,
                true,
                proof_as_of,
                ProofFreshnessClass::Fresh,
                "Remote preview route: managed-endpoint egress, direct route, bearer-token \
                 handle, managed trust bundle; denies all when offline; the local workspace \
                 continues without the preview.",
            ),
        ],
    }
}
