//! Implemented M5 bootstrap credential-posture and fetch-route registries.
//!
//! The frozen [repository-bootstrap matrix][matrix] names Aureline's five project-entry acquisition families
//! and locks their controlled vocabulary. This is the credential-boundary + mirror/trust-route implement lane:
//! it turns the *credential-posture* grammar (how a bootstrap authenticates, which trust roots or mirrors it
//! depends on, and how it references secrets) and the *fetch-route* grammar (public upstream fetch, approved
//! mirror fetch, air-gap bundle import, and managed snapshot resume) into registry resolvers that produce
//! export-safe, honest projections. Every claimed M5 acquisition path then resolves to one stable
//! credential-posture object — the auth-source kind and canonical auth mode, the auth-source reference, the
//! proxy / mirror route, the host-key or TLS-pin state, the delegated-token policy, the handle-only secret
//! reference kept out of the export boundary, and the mirror / signer provenance — and to one fetch-route
//! object — the route endpoint class, the signer-continuity reference, the digest-continuity reference, the
//! mirror-provenance reference, the recovery language, and the trust-proof reference — that the acquisition,
//! git, trust, diagnostics, CLI, and support / export surfaces can inspect without manual reconstruction, so a
//! credential posture never embeds a raw secret or token in a portable manifest, host trust state is disclosed
//! rather than hidden behind generic connected-state copy, public / mirrored / air-gapped / resumed fetch
//! routes stay distinct, signer and mirror provenance stay continuous, and an acquisition path that cannot
//! explain how it authenticated or which trust route it took degrades honestly instead of reading as a clean
//! pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable credential-posture object per acquisition path.** [`resolve_credential_posture_entry`]
//!   refuses to read as a clean, registry-bound posture entry unless it names a canonical registry token, a
//!   classified [auth source][M5CredentialAuthSourceKind], a repository-bootstrap role, covers every
//!   [resolution form][M5TrustResolutionForm] (the canonical object, the accessible summary, and the audit
//!   record), publishes every posture field (auth-source reference, proxy / mirror route, host-key or TLS-pin
//!   state, delegated-token policy, handle-only secret reference, and mirror / signer provenance), keeps any
//!   referenced secret handle-only, and discloses the host trust state; otherwise it degrades.
//! * **Keep the credential posture from embedding raw secrets or hiding host trust.**
//!   [`credential_posture_stays_handle_only`] rejects a posture that embedded raw secret material in a portable
//!   manifest instead of a handle-only reference, so it degrades to
//!   [`M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust`], and a posture
//!   that hides its host-key / TLS-pin state behind generic connected-state copy degrades the same way.
//! * **Keep the fetch route from breaking signer continuity or hiding trust proof.**
//!   [`resolve_fetch_route_entry`] names a classified [fetch-route class][M5FetchRouteClass], requires the full
//!   route-endpoint / signer-continuity / digest-continuity / mirror-provenance / recovery-language /
//!   trust-proof fetch-route object, covers every resolution form, and degrades to
//!   [`M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof`] when the route would
//!   lose signer or mirror provenance across an offline or mirrored fetch, hides its trust proof, or asserts a
//!   recovery it cannot explain, so a fetch route can never read as safe when it has quietly dropped signer
//!   continuity.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5RepositoryBootstrapRole`] role
//! vocabulary and the [`M5RepositoryBootstrapConsumerSurface`] consumer-surface taxonomy — so the acquisition,
//! shell, git, trust, diagnostics, docs, CLI, and support surfaces can never fork their own credential or route
//! meaning. Raw secret values, tokens, and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_repository_bootstrap_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries,
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed,
    seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed,
    M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_repository_bootstrap_matrix::{
    M5RepositoryBootstrapAccessibilityRoute, M5RepositoryBootstrapConsumerSurface,
    M5RepositoryBootstrapDeploymentLine, M5RepositoryBootstrapDowngradeTrigger,
    M5RepositoryBootstrapFamily, M5RepositoryBootstrapQualificationClass,
    M5RepositoryBootstrapRequiredLabel, M5RepositoryBootstrapRole,
    M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF, M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
    M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF, M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5CredentialPostureFetchRouteRegistriesPacket`].
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_bootstrap_credential_posture_and_fetch_route_registries";

/// Schema version for M5 credential-posture / fetch-route registry records.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_DOC_REF: &str =
    "docs/workspaces/m5_bootstrap_credential_posture_and_fetch_route_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-bootstrap-credential-posture-and-fetch-route-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5CredentialPostureFetchRouteRegistriesConsumerSurface =
    M5RepositoryBootstrapConsumerSurface;

/// One of the three resolution forms every credential-posture or fetch-route entry must hold across so its
/// truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// credential-posture and fetch-route *roles* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustResolutionForm {
    /// The canonical resolved credential-posture / fetch-route object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved posture / route discoverable without
    /// visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved posture / route inspectable off-renderer.
    AuditRecord,
}

impl M5TrustResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled auth-source kind a credential-posture entry resolves, so the canonical credential model shares one
/// registry rather than a hand-copied per-entry assumption. Minted by this lane because the frozen matrix
/// carries the acquisition families but not the concrete anonymous-public / delegated-token /
/// stored-handle-credential / host-key-or-TLS-pinned / air-gap-offline auth source a posture resolves against.
/// Every classified kind carries its canonical auth mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialAuthSourceKind {
    /// An anonymous public upstream fetch (no credential material).
    AnonymousPublic,
    /// A delegated short-lived token policy (references secret material, kept handle-only).
    DelegatedToken,
    /// A stored handle credential (references secret material via a handle-only reference).
    StoredHandleCredential,
    /// A host-key or TLS-pinned trust root (pins host trust; no embedded secret).
    HostKeyOrTlsPinned,
    /// An air-gapped, offline bootstrap (no network auth).
    AirGapOffline,
    /// The auth source is unclassified, which is disallowed.
    KindUnclassified,
}

impl M5CredentialAuthSourceKind {
    /// Every auth-source kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AnonymousPublic,
        Self::DelegatedToken,
        Self::StoredHandleCredential,
        Self::HostKeyOrTlsPinned,
        Self::AirGapOffline,
        Self::KindUnclassified,
    ];

    /// The five canonical auth sources every claimed M5 acquisition path resolves against.
    pub const CANONICAL_KINDS: [Self; 5] = [
        Self::AnonymousPublic,
        Self::DelegatedToken,
        Self::StoredHandleCredential,
        Self::HostKeyOrTlsPinned,
        Self::AirGapOffline,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnonymousPublic => "anonymous_public",
            Self::DelegatedToken => "delegated_token",
            Self::StoredHandleCredential => "stored_handle_credential",
            Self::HostKeyOrTlsPinned => "host_key_or_tls_pinned",
            Self::AirGapOffline => "air_gap_offline",
            Self::KindUnclassified => "kind_unclassified",
        }
    }

    /// Whether the kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::KindUnclassified)
    }

    /// The canonical auth mode for this kind.
    pub const fn canonical_auth_mode(self) -> &'static str {
        match self {
            Self::AnonymousPublic => "anonymous_public_auth",
            Self::DelegatedToken => "delegated_token_auth",
            Self::StoredHandleCredential => "stored_handle_credential_auth",
            Self::HostKeyOrTlsPinned => "host_key_or_tls_pin_auth",
            Self::AirGapOffline => "air_gap_offline_auth",
            Self::KindUnclassified => "",
        }
    }

    /// Whether this auth source references secret material and so must keep it handle-only, never embedded raw
    /// in a portable manifest.
    pub const fn references_secret_material(self) -> bool {
        matches!(self, Self::DelegatedToken | Self::StoredHandleCredential)
    }
}

/// Controlled fetch-route class a fetch-route entry must resolve its route from, so a fetch route shares one
/// registry rather than a hand-copied per-entry route. Minted by this lane, tracking the public-upstream /
/// approved-mirror / air-gap-bundle / managed-snapshot routes the implementation requirement differentiates by
/// name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FetchRouteClass {
    /// The public upstream fetch route.
    PublicUpstreamFetch,
    /// The approved mirror fetch route.
    ApprovedMirrorFetch,
    /// The air-gap bundle import route.
    AirGapBundleImport,
    /// The managed snapshot resume route.
    ManagedSnapshotResume,
    /// The fetch-route class is unclassified, which is disallowed.
    RouteUnclassified,
}

impl M5FetchRouteClass {
    /// Every fetch-route class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PublicUpstreamFetch,
        Self::ApprovedMirrorFetch,
        Self::AirGapBundleImport,
        Self::ManagedSnapshotResume,
        Self::RouteUnclassified,
    ];

    /// The four canonical routes every fetch route must stay distinct across.
    pub const CANONICAL_CLASSES: [Self; 4] = [
        Self::PublicUpstreamFetch,
        Self::ApprovedMirrorFetch,
        Self::AirGapBundleImport,
        Self::ManagedSnapshotResume,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicUpstreamFetch => "public_upstream_fetch",
            Self::ApprovedMirrorFetch => "approved_mirror_fetch",
            Self::AirGapBundleImport => "air_gap_bundle_import",
            Self::ManagedSnapshotResume => "managed_snapshot_resume",
            Self::RouteUnclassified => "route_unclassified",
        }
    }

    /// Whether the fetch-route class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::RouteUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a credential-posture or
/// fetch-route token's meaning stays stable whether it appears in the shell, entry, diagnostics, admin, or a
/// support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustSurfaceContext {
    /// The shell surface.
    ShellSurface,
    /// The project-entry surface.
    EntrySurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5TrustSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellSurface,
        Self::EntrySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::EntrySurface => "entry_surface",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a credential-posture or fetch-route entry must be able to show, so no auth
/// source, route, host-trust state, secret handling, provenance, or registry fact is left implicit behind a
/// hand-copied per-entry assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The auth-source kind the entry resolves (credential-posture entry).
    CredentialAuthSource,
    /// The auth-source reference and proxy / mirror route the entry publishes (credential-posture entry).
    AuthSourceAndRouteFields,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The host-key / TLS-pin state, delegated-token policy, and handle-only secret handling the entry publishes
    /// (credential-posture entry).
    HostTrustAndSecretHandling,
    /// The fetch-route fields (route class, signer / digest continuity, mirror provenance, recovery language)
    /// the entry publishes (fetch-route entry).
    FetchRouteFields,
    /// The trust-proof reference the entry publishes (fetch-route entry).
    TrustProofHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved credential posture or fetch route (both entries).
    PlainLanguageMeaning,
}

impl M5TrustAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::CredentialAuthSource,
        Self::AuthSourceAndRouteFields,
        Self::ResolutionFormCoverage,
        Self::HostTrustAndSecretHandling,
        Self::FetchRouteFields,
        Self::TrustProofHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::CredentialAuthSource => "credential_auth_source",
            Self::AuthSourceAndRouteFields => "auth_source_and_route_fields",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::HostTrustAndSecretHandling => "host_trust_and_secret_handling",
            Self::FetchRouteFields => "fetch_route_fields",
            Self::TrustProofHint => "trust_proof_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// credential posture, a fetch route, or a degraded credential-posture / fetch-route entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustNextAction {
    /// Expand the resolved posture's or route's plain-language meaning.
    ExpandTrustMeaning,
    /// Inspect the auth source or fetch-route class the entry resolves.
    InspectAuthSourceOrRoute,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5TrustNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandTrustMeaning,
        Self::InspectAuthSourceOrRoute,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandTrustMeaning => "expand_trust_meaning",
            Self::InspectAuthSourceOrRoute => "inspect_auth_source_or_route",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The repository-bootstrap families covered.
    RepositoryBootstrapFamilies,
    /// The credential auth sources carried.
    CredentialAuthSources,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The fetch-route classes carried.
    FetchRouteClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The auth modes carried.
    AuthModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TrustExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::CredentialAuthSources,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::FetchRouteClasses,
        Self::SurfaceContext,
        Self::AuthModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::RepositoryBootstrapFamilies,
        Self::CredentialAuthSources,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::RepositoryBootstrapFamilies => "repository_bootstrap_families",
            Self::CredentialAuthSources => "credential_auth_sources",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::FetchRouteClasses => "fetch_route_classes",
            Self::SurfaceContext => "surface_context",
            Self::AuthModes => "auth_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a credential-posture entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, secret-embedding, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CredentialPostureEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the posture means.
    PostureTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The auth source is unclassified (not in the resolved taxonomy).
    CredentialAuthSourceUnclassified,
    /// The behavior is a hand-copied per-entry assumption instead of tracing to the canonical registry.
    PostureNotBoundToRegistry,
    /// The resolved credential-posture object is incomplete: the auth-source reference, proxy / mirror route,
    /// host-key / TLS-pin state, delegated-token policy, handle-only secret reference, or mirror / signer
    /// provenance is unstated.
    CredentialPostureObjectIncomplete,
    /// The posture embedded raw secret material in a portable manifest instead of a handle-only reference, or it
    /// hid its host-key / TLS-pin state behind generic connected-state copy.
    CredentialPostureEmbedsRawSecretOrHidesHostTrust,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A secret-referencing posture embedded raw secret material in the portable manifest.
    SecretMaterialEmbeddedRawInManifest,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CredentialPostureEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PostureTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::CredentialAuthSourceUnclassified,
        Self::PostureNotBoundToRegistry,
        Self::CredentialPostureObjectIncomplete,
        Self::CredentialPostureEmbedsRawSecretOrHidesHostTrust,
        Self::ResolutionFormCoverageIncomplete,
        Self::SecretMaterialEmbeddedRawInManifest,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostureTokenUnstated => "posture_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::CredentialAuthSourceUnclassified => "credential_auth_source_unclassified",
            Self::PostureNotBoundToRegistry => "posture_not_bound_to_registry",
            Self::CredentialPostureObjectIncomplete => "credential_posture_object_incomplete",
            Self::CredentialPostureEmbedsRawSecretOrHidesHostTrust => {
                "credential_posture_embeds_raw_secret_or_hides_host_trust"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::SecretMaterialEmbeddedRawInManifest => "secret_material_embedded_raw_in_manifest",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TrustNextAction {
        match self {
            Self::PostureTokenUnstated | Self::PostureNotBoundToRegistry => {
                M5TrustNextAction::TraceCanonicalRegistry
            }
            Self::CredentialAuthSourceUnclassified
            | Self::CredentialPostureObjectIncomplete
            | Self::CredentialPostureEmbedsRawSecretOrHidesHostTrust => {
                M5TrustNextAction::InspectAuthSourceOrRoute
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5TrustNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::SecretMaterialEmbeddedRawInManifest
            | Self::ProofStale => M5TrustNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::PostureTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::CredentialAuthSourceUnclassified | Self::CredentialPostureObjectIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::CredentialPostureUnstated
            }
            Self::PostureNotBoundToRegistry => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::CredentialPostureEmbedsRawSecretOrHidesHostTrust
            | Self::SecretMaterialEmbeddedRawInManifest => {
                M5RepositoryBootstrapDowngradeTrigger::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a fetch-route entry degraded below a clean, safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FetchRouteEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RouteTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The fetch-route class is unclassified (not in the resolved taxonomy).
    FetchRouteClassUnclassified,
    /// The fetch route would lose signer or mirror provenance across an offline or mirrored fetch, hides its
    /// trust proof, or asserts a recovery it cannot explain, or it dropped one of the required fetch-route
    /// fields (route endpoint, signer continuity, digest continuity, mirror provenance, recovery language,
    /// trust proof).
    FetchRouteBreaksSignerContinuityOrHidesTrustProof,
    /// The canonical / accessible / audit resolution-form coverage of the fetch route is incomplete.
    RouteFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5FetchRouteEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RouteTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::FetchRouteClassUnclassified,
        Self::FetchRouteBreaksSignerContinuityOrHidesTrustProof,
        Self::RouteFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteTokenUnstated => "route_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::FetchRouteClassUnclassified => "fetch_route_class_unclassified",
            Self::FetchRouteBreaksSignerContinuityOrHidesTrustProof => {
                "fetch_route_breaks_signer_continuity_or_hides_trust_proof"
            }
            Self::RouteFormCoverageIncomplete => "route_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TrustNextAction {
        match self {
            Self::RouteTokenUnstated => M5TrustNextAction::TraceCanonicalRegistry,
            Self::FetchRouteClassUnclassified
            | Self::FetchRouteBreaksSignerContinuityOrHidesTrustProof => {
                M5TrustNextAction::InspectAuthSourceOrRoute
            }
            Self::RouteFormCoverageIncomplete => M5TrustNextAction::CompleteResolutionFormCoverage,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5TrustNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5RepositoryBootstrapDowngradeTrigger {
        match self {
            Self::RouteTokenUnstated => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::FetchRouteClassUnclassified => {
                M5RepositoryBootstrapDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::FetchRouteBreaksSignerContinuityOrHidesTrustProof => {
                M5RepositoryBootstrapDowngradeTrigger::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches
            }
            Self::RouteFormCoverageIncomplete => {
                M5RepositoryBootstrapDowngradeTrigger::CheckoutPlanBoundaryDriftedBySurface
            }
            Self::ProofStale => M5RepositoryBootstrapDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_credential_posture_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CredentialPostureEntryResolutionInput {
    /// Stable identity of the credential-posture-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this posture binds to (e.g. `entry.acme.clone-remote`); empty means
    /// unstated.
    pub acquisition_path_id: String,
    /// The canonical registry token name (e.g. `credential.posture.delegated_token`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The auth source this entry resolves.
    pub auth_source_kind: M5CredentialAuthSourceKind,
    /// The render / surface context.
    pub surface_context: M5TrustSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5TrustResolutionForm>,
    /// The published auth-source reference; empty means unstated.
    pub auth_source_ref: String,
    /// The published proxy / mirror route; empty means unstated.
    pub proxy_or_mirror_route: String,
    /// The published host-key / TLS-pin state; empty means unstated.
    pub host_key_or_tls_pin_state: String,
    /// The published delegated-token policy; empty means unstated.
    pub delegated_token_policy: String,
    /// The published handle-only secret reference kept out of the export boundary; empty means unstated.
    pub handle_only_secret_reference: String,
    /// The published mirror / signer provenance; empty means unstated.
    pub mirror_or_signer_provenance: String,
    /// True when the behavior traces to the credential-posture registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the host-key / TLS-pin trust state is disclosed rather than hidden behind generic
    /// connected-state copy (a hard invariant when `false`).
    pub host_trust_disclosed: bool,
    /// True when this posture references secret material.
    pub references_secret_material: bool,
    /// True when any referenced secret material is kept as a handle-only reference, never embedded raw in a
    /// portable manifest.
    pub secret_kept_handle_only: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe credential-posture-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCredentialPostureEntry {
    /// Stable identity of the credential-posture-registry entry.
    pub entry_id: String,
    /// The stable acquisition-path ID this posture binds to.
    pub acquisition_path_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The auth-source-kind token named by the entry.
    pub auth_source_kind: String,
    /// Whether the auth-source kind is classified into the resolved taxonomy.
    pub auth_source_kind_is_classified: bool,
    /// The canonical auth mode for the entry's kind.
    pub canonical_auth_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published auth-source reference.
    pub auth_source_ref: String,
    /// The published proxy / mirror route.
    pub proxy_or_mirror_route: String,
    /// The published host-key / TLS-pin state.
    pub host_key_or_tls_pin_state: String,
    /// The published delegated-token policy.
    pub delegated_token_policy: String,
    /// The published handle-only secret reference.
    pub handle_only_secret_reference: String,
    /// The published mirror / signer provenance.
    pub mirror_or_signer_provenance: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved credential-posture object publishes every required field.
    pub credential_posture_object_complete: bool,
    /// Whether the entry traces to the credential-posture registry.
    pub bound_to_registry: bool,
    /// Whether the host-key / TLS-pin trust state is disclosed.
    pub host_trust_disclosed: bool,
    /// Whether this posture references secret material.
    pub references_secret_material: bool,
    /// Whether any referenced secret material is kept handle-only.
    pub secret_kept_handle_only: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5CredentialPostureEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TrustNextAction,
    /// Whether the posture resolves to one stable object across every claimed acquisition path (clean entry
    /// naming every fact).
    pub posture_resolves_across_entry_flows: bool,
}

impl M5ResolvedCredentialPostureEntry {
    /// Whether this credential-posture entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_fetch_route_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FetchRouteEntryResolutionInput {
    /// Stable identity of the fetch-route entry.
    pub entry_id: String,
    /// The stable source-ref this route binds to; empty means unstated.
    pub source_ref: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5RepositoryBootstrapRole,
    /// The fetch-route class this entry must resolve its route from.
    pub route_class: M5FetchRouteClass,
    /// The render / surface context.
    pub surface_context: M5TrustSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5TrustResolutionForm>,
    /// The published route endpoint class; empty means missing.
    pub route_endpoint_class: String,
    /// The published signer-continuity reference; empty means missing.
    pub signer_continuity_ref: String,
    /// The published digest-continuity reference; empty means missing.
    pub digest_continuity_ref: String,
    /// The published mirror-provenance reference; empty means missing.
    pub mirror_provenance_ref: String,
    /// The published recovery language; empty means missing.
    pub recovery_language: String,
    /// The published trust-proof reference; empty means missing.
    pub trust_proof_ref: String,
    /// True when the route keeps signer continuity and its trust proof visible before any fetch.
    pub keeps_signer_continuity_visible: bool,
    /// True when the route is truthful (never claims a safe route over a dropped signer continuity).
    pub route_is_truthful: bool,
    /// True when the route crosses an offline or mirrored fetch (mirror / air-gap / managed-snapshot routes).
    pub crosses_offline_or_mirror: bool,
    /// True when signer / mirror provenance is preserved across the offline or mirrored fetch.
    pub signer_continuity_preserved: bool,
    /// True when the route asserts a recovery (resume / discard / read-only) path.
    pub asserts_recovery: bool,
    /// True when an asserted recovery is explained rather than left implicit.
    pub recovery_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe fetch-route projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFetchRouteEntry {
    /// Stable identity of the fetch-route entry.
    pub entry_id: String,
    /// The stable source-ref this route binds to.
    pub source_ref: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must stage trust and disclose provenance before bootstrap.
    pub semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: bool,
    /// The fetch-route-class token named by the entry.
    pub route_class: String,
    /// Whether the fetch-route class is classified into the resolved taxonomy.
    pub route_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published route endpoint class.
    pub route_endpoint_class: String,
    /// The published signer-continuity reference.
    pub signer_continuity_ref: String,
    /// The published digest-continuity reference.
    pub digest_continuity_ref: String,
    /// The published mirror-provenance reference.
    pub mirror_provenance_ref: String,
    /// The published recovery language.
    pub recovery_language: String,
    /// The published trust-proof reference.
    pub trust_proof_ref: String,
    /// Whether the route keeps signer continuity and its trust proof visible before any fetch.
    pub keeps_signer_continuity_visible: bool,
    /// Whether the route is truthful.
    pub route_is_truthful: bool,
    /// Whether the route crosses an offline or mirrored fetch.
    pub crosses_offline_or_mirror: bool,
    /// Whether signer / mirror provenance is preserved across the offline or mirrored fetch.
    pub signer_continuity_preserved: bool,
    /// Whether the route asserts a recovery path.
    pub asserts_recovery: bool,
    /// Whether an asserted recovery is explained.
    pub recovery_explained: bool,
    /// Whether the route stays signer-continuous (continuity visible, provenance preserved, explained recovery).
    pub fetch_route_stays_signer_continuous: bool,
    /// Whether the entry provides the complete fetch-route object (route endpoint, signer / digest continuity,
    /// mirror provenance, recovery language, trust proof).
    pub provides_complete_fetch_route: bool,
    /// Degrade reason, if the entry could not read as a clean, safe state.
    pub degrade_reason: Option<M5FetchRouteEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TrustNextAction,
    /// Whether the fetch route is safe on every claimed source (clean entry naming every fact).
    pub route_safe_on_every_source: bool,
}

impl M5ResolvedFetchRouteEntry {
    /// Whether this fetch-route entry reads as a clean, safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TrustResolutionError {
    /// The credential-posture-entry id was empty.
    EmptyCredentialPostureEntryId,
    /// The fetch-route-entry id was empty.
    EmptyFetchRouteEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TrustResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCredentialPostureEntryId => "empty_credential_posture_entry_id",
            Self::EmptyFetchRouteEntryId => "empty_fetch_route_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TrustResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 credential-posture / fetch-route registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TrustResolutionError {}

fn form_tokens(forms: &[M5TrustResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5TrustResolutionForm]) -> bool {
    let present: BTreeSet<M5TrustResolutionForm> = forms.iter().copied().collect();
    M5TrustResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved credential-posture object publishes every required field: auth mode (via a classified
/// kind), auth-source reference, proxy / mirror route, host-key / TLS-pin state, delegated-token policy,
/// handle-only secret reference, and mirror / signer provenance. An unclassified kind or any empty field never
/// resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn credential_posture_object_is_complete(
    kind: M5CredentialAuthSourceKind,
    auth_source_ref: &str,
    proxy_or_mirror_route: &str,
    host_key_or_tls_pin_state: &str,
    delegated_token_policy: &str,
    handle_only_secret_reference: &str,
    mirror_or_signer_provenance: &str,
) -> bool {
    kind.is_classified()
        && !auth_source_ref.trim().is_empty()
        && !proxy_or_mirror_route.trim().is_empty()
        && !host_key_or_tls_pin_state.trim().is_empty()
        && !delegated_token_policy.trim().is_empty()
        && !handle_only_secret_reference.trim().is_empty()
        && !mirror_or_signer_provenance.trim().is_empty()
}

/// Whether the credential posture stays a handle-only, host-trust-disclosing posture: the kind must be
/// classified, the host-key / TLS-pin trust state must be disclosed (never hidden behind generic connected-state
/// copy), and a secret-referencing posture must keep the secret handle-only (never embedded raw in a portable
/// manifest). An unclassified kind, a hidden host trust state, or an embedded raw secret never matches.
pub fn credential_posture_stays_handle_only(
    kind: M5CredentialAuthSourceKind,
    host_trust_disclosed: bool,
    references_secret_material: bool,
    secret_kept_handle_only: bool,
) -> bool {
    kind.is_classified()
        && host_trust_disclosed
        && (!references_secret_material || secret_kept_handle_only)
}

/// Whether a fetch route stays signer-continuous: the class must be classified, the route must be truthful, it
/// must keep signer continuity and its trust proof visible before any fetch, any offline or mirrored fetch must
/// preserve signer / mirror provenance, and any asserted recovery must be explained.
pub fn fetch_route_stays_signer_continuous(
    class: M5FetchRouteClass,
    route_is_truthful: bool,
    keeps_signer_continuity_visible: bool,
    crosses_offline_or_mirror: bool,
    signer_continuity_preserved: bool,
    asserts_recovery: bool,
    recovery_explained: bool,
) -> bool {
    class.is_classified()
        && route_is_truthful
        && keeps_signer_continuity_visible
        && (!crosses_offline_or_mirror || signer_continuity_preserved)
        && (!asserts_recovery || recovery_explained)
}

/// Resolves a credential-posture-registry entry so it stays bound to the credential-posture registry: the entry
/// names its canonical token, semantic role, and auth source, covers all three resolution forms, publishes a
/// complete credential-posture object (auth-source reference, proxy / mirror route, host-key / TLS-pin state,
/// delegated-token policy, handle-only secret reference, mirror / signer provenance), keeps any referenced
/// secret handle-only, and discloses the host trust state.
pub fn resolve_credential_posture_entry(
    input: M5CredentialPostureEntryResolutionInput,
) -> Result<M5ResolvedCredentialPostureEntry, M5TrustResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5TrustResolutionError::EmptyCredentialPostureEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.acquisition_path_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.auth_source_ref)
        || string_is_forbidden(&input.proxy_or_mirror_route)
        || string_is_forbidden(&input.host_key_or_tls_pin_state)
        || string_is_forbidden(&input.delegated_token_policy)
        || string_is_forbidden(&input.handle_only_secret_reference)
        || string_is_forbidden(&input.mirror_or_signer_provenance)
    {
        return Err(M5TrustResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = credential_posture_object_is_complete(
        input.auth_source_kind,
        &input.auth_source_ref,
        &input.proxy_or_mirror_route,
        &input.host_key_or_tls_pin_state,
        &input.delegated_token_policy,
        &input.handle_only_secret_reference,
        &input.mirror_or_signer_provenance,
    );
    let handle_only_ok = credential_posture_stays_handle_only(
        input.auth_source_kind,
        input.host_trust_disclosed,
        input.references_secret_material,
        input.secret_kept_handle_only,
    );
    let secret_embedded_raw = input.references_secret_material && !input.secret_kept_handle_only;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CredentialPostureEntryDegradeReason::PostureTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CredentialPostureEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.auth_source_kind.is_classified() {
        Some(M5CredentialPostureEntryDegradeReason::CredentialAuthSourceUnclassified)
    } else if !input.bound_to_registry {
        Some(M5CredentialPostureEntryDegradeReason::PostureNotBoundToRegistry)
    } else if !object_complete {
        Some(M5CredentialPostureEntryDegradeReason::CredentialPostureObjectIncomplete)
    } else if !handle_only_ok {
        Some(
            M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust,
        )
    } else if !all_forms {
        Some(M5CredentialPostureEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if secret_embedded_raw {
        Some(M5CredentialPostureEntryDegradeReason::SecretMaterialEmbeddedRawInManifest)
    } else if !input.proof_fresh {
        Some(M5CredentialPostureEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TrustNextAction::ExpandTrustMeaning,
    };

    Ok(M5ResolvedCredentialPostureEntry {
        entry_id: input.entry_id,
        acquisition_path_id: input.acquisition_path_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        auth_source_kind: input.auth_source_kind.as_str().to_owned(),
        auth_source_kind_is_classified: input.auth_source_kind.is_classified(),
        canonical_auth_mode: input.auth_source_kind.canonical_auth_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        auth_source_ref: input.auth_source_ref,
        proxy_or_mirror_route: input.proxy_or_mirror_route,
        host_key_or_tls_pin_state: input.host_key_or_tls_pin_state,
        delegated_token_policy: input.delegated_token_policy,
        handle_only_secret_reference: input.handle_only_secret_reference,
        mirror_or_signer_provenance: input.mirror_or_signer_provenance,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        credential_posture_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        host_trust_disclosed: input.host_trust_disclosed,
        references_secret_material: input.references_secret_material,
        secret_kept_handle_only: input.secret_kept_handle_only,
        degrade_reason,
        next_action,
        posture_resolves_across_entry_flows: degrade_reason.is_none(),
    })
}

/// Resolves a fetch-route entry so its route stays safe: the entry names its canonical token, semantic role,
/// and fetch-route class, covers all three resolution forms, provides the complete route-endpoint /
/// signer-continuity / digest-continuity / mirror-provenance / recovery-language / trust-proof fetch-route
/// object, and degrades honestly when the route would lose signer or mirror provenance across an offline or
/// mirrored fetch, hides its trust proof, or asserts a recovery it cannot explain.
pub fn resolve_fetch_route_entry(
    input: M5FetchRouteEntryResolutionInput,
) -> Result<M5ResolvedFetchRouteEntry, M5TrustResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5TrustResolutionError::EmptyFetchRouteEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.source_ref)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.route_endpoint_class)
        || string_is_forbidden(&input.signer_continuity_ref)
        || string_is_forbidden(&input.digest_continuity_ref)
        || string_is_forbidden(&input.mirror_provenance_ref)
        || string_is_forbidden(&input.recovery_language)
        || string_is_forbidden(&input.trust_proof_ref)
    {
        return Err(M5TrustResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let route_stays_continuous = fetch_route_stays_signer_continuous(
        input.route_class,
        input.route_is_truthful,
        input.keeps_signer_continuity_visible,
        input.crosses_offline_or_mirror,
        input.signer_continuity_preserved,
        input.asserts_recovery,
        input.recovery_explained,
    );
    let provides_route = input.route_class.is_classified()
        && !input.route_endpoint_class.trim().is_empty()
        && !input.signer_continuity_ref.trim().is_empty()
        && !input.digest_continuity_ref.trim().is_empty()
        && !input.mirror_provenance_ref.trim().is_empty()
        && !input.recovery_language.trim().is_empty()
        && !input.trust_proof_ref.trim().is_empty()
        && route_stays_continuous;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5FetchRouteEntryDegradeReason::RouteTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5FetchRouteEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.route_class.is_classified() {
        Some(M5FetchRouteEntryDegradeReason::FetchRouteClassUnclassified)
    } else if !provides_route {
        Some(M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof)
    } else if !all_forms {
        Some(M5FetchRouteEntryDegradeReason::RouteFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5FetchRouteEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TrustNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedFetchRouteEntry {
        entry_id: input.entry_id,
        source_ref: input.source_ref,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_must_stage_trust_and_disclose_provenance_before_bootstrap: input
            .semantic_role
            .must_stage_trust_and_disclose_provenance_before_bootstrap(),
        route_class: input.route_class.as_str().to_owned(),
        route_class_is_classified: input.route_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        route_endpoint_class: input.route_endpoint_class,
        signer_continuity_ref: input.signer_continuity_ref,
        digest_continuity_ref: input.digest_continuity_ref,
        mirror_provenance_ref: input.mirror_provenance_ref,
        recovery_language: input.recovery_language,
        trust_proof_ref: input.trust_proof_ref,
        keeps_signer_continuity_visible: input.keeps_signer_continuity_visible,
        route_is_truthful: input.route_is_truthful,
        crosses_offline_or_mirror: input.crosses_offline_or_mirror,
        signer_continuity_preserved: input.signer_continuity_preserved,
        asserts_recovery: input.asserts_recovery,
        recovery_explained: input.recovery_explained,
        fetch_route_stays_signer_continuous: route_stays_continuous,
        provides_complete_fetch_route: provides_route,
        degrade_reason,
        next_action,
        route_safe_on_every_source: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved credential-posture and fetch-route entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CredentialPostureFetchRouteRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5RepositoryBootstrapQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Acquisition contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5RepositoryBootstrapDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5RepositoryBootstrapRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5RepositoryBootstrapAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5TrustAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TrustExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    /// Resolved credential-posture-registry examples.
    pub credential_posture_entries: Vec<M5ResolvedCredentialPostureEntry>,
    /// Resolved fetch-route examples.
    pub fetch_route_entries: Vec<M5ResolvedFetchRouteEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the checkout-plan and bootstrap-evidence
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this row never embeds a raw secret, token, or host trust state in a portable manifest.
    /// MUST be `false`.
    pub embeds_raw_secret_token_or_host_trust_state_in_portable_manifest: bool,
    /// Hard invariant: this row never loses signer or mirror provenance across offline or mirrored fetches. MUST
    /// be `false`.
    pub loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: bool,
    /// Hard invariant: this row never hides bootstrap credential posture behind generic connected-state copy.
    /// MUST be `false`.
    pub hides_bootstrap_credential_posture_behind_generic_connected_state_copy: bool,
    /// Hard invariant: this row never collapses distinct fetch routes into one runtime path. MUST be `false`.
    pub collapses_distinct_fetch_routes_into_one_runtime_path: bool,
}

impl M5CredentialPostureFetchRouteRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TrustAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5TrustAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TrustExportField> = self.export_fields.iter().copied().collect();
        M5TrustExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.embeds_raw_secret_token_or_host_trust_state_in_portable_manifest
            && !self.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches
            && !self.hides_bootstrap_credential_posture_behind_generic_connected_state_copy
            && !self.collapses_distinct_fetch_routes_into_one_runtime_path
    }

    /// True when a clean credential-posture entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified auth source, publishes a complete posture object, discloses the host trust state,
    /// covers all three resolution forms, and keeps any referenced secret handle-only.
    fn posture_is_honest(ex: &M5ResolvedCredentialPostureEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.auth_source_kind_is_classified
                && ex.credential_posture_object_complete
                && ex.host_trust_disclosed
                && ex.covers_all_resolution_forms
                && (!ex.references_secret_material || ex.secret_kept_handle_only))
    }

    /// True when a clean fetch-route entry preserves a safe route: it keeps a classified class, provides the
    /// complete fetch-route object, stays signer-continuous, and covers all three resolution forms.
    fn route_is_honest(ex: &M5ResolvedFetchRouteEntry) -> bool {
        !ex.is_clean()
            || (ex.route_class_is_classified
                && ex.provides_complete_fetch_route
                && ex.fetch_route_stays_signer_continuous
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.credential_posture_entries
            .iter()
            .all(Self::posture_is_honest)
            && self.fetch_route_entries.iter().all(Self::route_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Credential auth-source tokens (minted by this lane).
    pub credential_auth_sources: Vec<String>,
    /// Fetch-route-class tokens (minted by this lane).
    pub fetch_route_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Credential-posture-entry degrade-reason tokens.
    pub credential_posture_degrade_reasons: Vec<String>,
    /// Fetch-route-entry degrade-reason tokens.
    pub fetch_route_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5CredentialPostureFetchRouteRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5RepositoryBootstrapRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5TrustResolutionForm::ALL, |v| v.as_str()),
            credential_auth_sources: tokens(&M5CredentialAuthSourceKind::ALL, |v| v.as_str()),
            fetch_route_classes: tokens(&M5FetchRouteClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5TrustSurfaceContext::ALL, |v| v.as_str()),
            credential_posture_degrade_reasons: tokens(
                &M5CredentialPostureEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            fetch_route_degrade_reasons: tokens(&M5FetchRouteEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5TrustAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TrustNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TrustExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5RepositoryBootstrapConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesGovernanceReview {
    /// The credential registry names a canonical token, semantic role, and auth source for every entry.
    pub credential_registry_names_token_role_and_source: bool,
    /// Every claimed acquisition path resolves to one stable credential-posture object from the shared registry,
    /// not per-entry reconstruction.
    pub entry_flow_resolves_to_stable_posture_from_shared_registry: bool,
    /// The auth-source reference, proxy / mirror route, host trust state, delegated-token policy, handle-only
    /// secret reference, and mirror / signer provenance are published for every resolved posture.
    pub auth_source_route_host_trust_and_provenance_published: bool,
    /// The credential posture stays handle-only; no raw secret or token is embedded in a portable manifest.
    pub credential_posture_stays_handle_only_no_raw_secret: bool,
    /// The fetch route keeps signer / mirror continuity and its trust proof visible before any fetch.
    pub fetch_route_keeps_signer_continuity_and_trust_proof: bool,
    /// The host-key / TLS-pin trust state is disclosed rather than hidden behind generic connected-state copy.
    pub host_trust_state_disclosed_not_generic_copy: bool,
    /// Every credential-posture and fetch-route entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Credential-posture and fetch-route behavior stay bound to the shared registries rather than hand-copied
    /// per acquisition path.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Acquisition, git, trust, and diagnostics read a single credential / route source.
    pub acquisition_git_trust_diagnostics_read_single_source: bool,
    /// An embedded raw secret, a dropped signer continuity, or a hidden host trust state is caught by fixtures
    /// before release evidence turns green.
    pub posture_or_route_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesConsumerProjection {
    /// Acquisition engine and git service consume the shared credential-posture registry.
    pub acquisition_and_git_consume_shared_registries: bool,
    /// Trust service and diagnostics consume the shared fetch-route registry.
    pub trust_and_diagnostics_consume_shared_registries: bool,
    /// CLI export and support export consume the shared registries.
    pub cli_and_support_export_consume_shared_registries: bool,
    /// Docs, help, and workspace services consume the shared registries.
    pub docs_help_and_workspace_consume_shared_registries: bool,
    /// Behavior traces back to the canonical checkout-plan and bootstrap-evidence domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical credential-posture / fetch-route registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting repository-bootstrap audit for the lane.
    pub repository_bootstrap_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CredentialPostureFetchRouteRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CredentialPostureFetchRouteRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CredentialPostureFetchRouteRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialPostureFetchRouteRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialPostureFetchRouteRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialPostureFetchRouteRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialPostureFetchRouteRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialPostureFetchRouteRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 credential-posture and fetch-route registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CredentialPostureFetchRouteRegistriesPacket {
    /// Record kind; must equal [`M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5CredentialPostureFetchRouteRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CredentialPostureFetchRouteRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CredentialPostureFetchRouteRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CredentialPostureFetchRouteRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CredentialPostureFetchRouteRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CredentialPostureFetchRouteRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CredentialPostureFetchRouteRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5CredentialPostureFetchRouteRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5CredentialPostureFetchRouteRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_RECORD_KIND {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 credential-posture / fetch-route registries packet serializes"),
        ) {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 credential-posture / fetch-route registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,credential_posture_entries,fetch_route_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .credential_posture_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.fetch_route_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.credential_posture_entries.len(),
                row.fetch_route_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Credential-Posture and Fetch-Route Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Credential auth sources: {}\n",
            self.vocabulary_set.credential_auth_sources.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Credential-posture entries: {} / fetch-route entries: {}\n",
                row.credential_posture_entries.len(),
                row.fetch_route_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-entry credential-posture reference table generated from the registry, so docs and admin
    /// runbooks render the same auth-mode / auth-source / route / host-trust / delegated-token / provenance
    /// truth the resolvers produced rather than a hand-copied credential table. Only clean, registry-bound
    /// credential-posture entries are listed.
    pub fn render_credential_posture_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| acquisition_path_id | auth_mode | auth_source_ref | proxy_or_mirror_route | host_key_or_tls_pin_state | delegated_token_policy | mirror_or_signer_provenance |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.credential_posture_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.acquisition_path_id,
                    ex.canonical_auth_mode,
                    ex.auth_source_ref,
                    ex.proxy_or_mirror_route,
                    ex.host_key_or_tls_pin_state,
                    ex.delegated_token_policy,
                    ex.mirror_or_signer_provenance
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5CredentialPostureFetchRouteRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CredentialPostureFetchRouteRegistriesViolation>),
}

impl fmt::Display for M5CredentialPostureFetchRouteRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 credential-posture / fetch-route registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 credential-posture / fetch-route registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CredentialPostureFetchRouteRegistriesArtifactError {}

/// Validation failures emitted by [`M5CredentialPostureFetchRouteRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CredentialPostureFetchRouteRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the checkout-plan and bootstrap-evidence domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, secret-embedding, field-incomplete,
    /// form-incomplete, or a fetch-route entry missing the complete route object).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Credential-posture-resolution is not proven: clean posture entries do not cover the canonical auth
    /// sources or the first shell / entry / diagnostics / admin / support surfaces, no object-incomplete example
    /// degrades, or a clean posture entry published an incomplete object.
    CredentialPostureResolutionNotProven,
    /// Handle-only-secret-preservation is not proven: no secret-embed example and no unbound example degrade, no
    /// clean handle-only posture entry is present, or a clean posture entry embedded a raw secret or is unbound.
    HandleOnlySecretPreservationNotProven,
    /// Fetch-route-continuity is not proven: clean fetch-route entries do not cover the canonical public /
    /// mirror / air-gap / managed-snapshot routes with full resolution-form coverage while providing the
    /// complete route object, no continuity-break or form-incomplete example degrades, or a clean fetch-route
    /// entry is missing the complete route object.
    FetchRouteContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CredentialPostureFetchRouteRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::CredentialPostureResolutionNotProven => {
                "credential_posture_resolution_not_proven"
            }
            Self::HandleOnlySecretPreservationNotProven => {
                "handle_only_secret_preservation_not_proven"
            }
            Self::FetchRouteContinuityNotProven => "fetch_route_continuity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_bootstrap_credential_posture_and_fetch_route_registries_export() -> Result<
    M5CredentialPostureFetchRouteRegistriesPacket,
    M5CredentialPostureFetchRouteRegistriesArtifactError,
> {
    let packet: M5CredentialPostureFetchRouteRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-bootstrap-credential-posture-and-fetch-route-registries-proof/support_export.json"
        )
    ))
    .map_err(M5CredentialPostureFetchRouteRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CredentialPostureFetchRouteRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_REF,
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5CredentialPostureFetchRouteRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5CredentialPostureFetchRouteRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations
                .push(M5CredentialPostureFetchRouteRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5CredentialPostureFetchRouteRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5CredentialPostureFetchRouteRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF)
        {
            violations
                .push(M5CredentialPostureFetchRouteRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.credential_posture_entries.is_empty() || row.fetch_route_entries.is_empty() {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5CredentialPostureFetchRouteRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.credential_registry_names_token_role_and_source,
        review.entry_flow_resolves_to_stable_posture_from_shared_registry,
        review.auth_source_route_host_trust_and_provenance_published,
        review.credential_posture_stays_handle_only_no_raw_secret,
        review.fetch_route_keeps_signer_continuity_and_trust_proof,
        review.host_trust_state_disclosed_not_generic_copy,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.acquisition_git_trust_diagnostics_read_single_source,
        review.posture_or_route_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5CredentialPostureFetchRouteRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.acquisition_and_git_consume_shared_registries,
        projection.trust_and_diagnostics_consume_shared_registries,
        projection.cli_and_support_export_consume_shared_registries,
        projection.docs_help_and_workspace_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5CredentialPostureFetchRouteRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CredentialPostureFetchRouteRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.repository_bootstrap_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CredentialPostureFetchRouteRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5CredentialPostureFetchRouteRegistriesPacket,
    violations: &mut Vec<M5CredentialPostureFetchRouteRegistriesViolation>,
) {
    let postures = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.credential_posture_entries.iter())
    };
    let routes = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.fetch_route_entries.iter())
    };

    // AC1: every claimed acquisition path resolves to one stable credential-posture object with auth-source /
    // route / host-trust / delegated-token / provenance fields. Clean posture entries cover the canonical auth
    // sources and the first shell / entry / diagnostics / admin / support surfaces, an object-incomplete example
    // degrades, and no clean posture entry published an incomplete object.
    let clean_kinds: BTreeSet<String> = postures()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.auth_source_kind.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = postures()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let kinds_covered = M5CredentialAuthSourceKind::CANONICAL_KINDS
        .iter()
        .all(|k| clean_kinds.contains(k.as_str()));
    let first_surfaces_covered = M5TrustSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = postures().any(|ex| {
        ex.degrade_reason
            == Some(M5CredentialPostureEntryDegradeReason::CredentialPostureObjectIncomplete)
    });
    let no_clean_incomplete =
        !postures().any(|ex| ex.is_clean() && !ex.credential_posture_object_complete);
    if !(kinds_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5CredentialPostureFetchRouteRegistriesViolation::CredentialPostureResolutionNotProven,
        );
    }

    // AC2: the credential posture stays handle-only — no raw secret or token is embedded and host trust stays
    // disclosed. A secret-embed example degrades, an unbound example degrades, at least one clean handle-only
    // posture entry is present, and no clean posture entry embedded a raw secret or is unbound.
    let embed_degrades = postures().any(|ex| {
        ex.degrade_reason
            == Some(
                M5CredentialPostureEntryDegradeReason::CredentialPostureEmbedsRawSecretOrHidesHostTrust,
            )
    });
    let unbound_degrades = postures().any(|ex| {
        ex.degrade_reason == Some(M5CredentialPostureEntryDegradeReason::PostureNotBoundToRegistry)
    });
    let handle_only_clean_posture = postures().any(|ex| {
        ex.is_clean()
            && ex.host_trust_disclosed
            && (!ex.references_secret_material || ex.secret_kept_handle_only)
    });
    let no_clean_unbound = !postures().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_embedded = !postures()
        .any(|ex| ex.is_clean() && ex.references_secret_material && !ex.secret_kept_handle_only);
    if !(embed_degrades
        && unbound_degrades
        && handle_only_clean_posture
        && no_clean_unbound
        && no_clean_embedded)
    {
        violations.push(
            M5CredentialPostureFetchRouteRegistriesViolation::HandleOnlySecretPreservationNotProven,
        );
    }

    // AC3: the suite fails when a fetch route drops signer continuity. Clean fetch-route entries cover every
    // canonical public / mirror / air-gap / managed-snapshot route with full resolution-form coverage while
    // providing the complete route object, a continuity-break example degrades, a form-incomplete example
    // degrades, and no clean fetch-route entry is missing the complete route object.
    let clean_route_classes: BTreeSet<String> = routes()
        .filter(|ex| {
            ex.is_clean()
                && ex.route_class_is_classified
                && ex.provides_complete_fetch_route
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.route_class.clone())
        .collect();
    let route_classes_covered = M5FetchRouteClass::CANONICAL_CLASSES
        .iter()
        .all(|c| clean_route_classes.contains(c.as_str()));
    let continuity_break_degrades = routes().any(|ex| {
        ex.degrade_reason
            == Some(
                M5FetchRouteEntryDegradeReason::FetchRouteBreaksSignerContinuityOrHidesTrustProof,
            )
    });
    let form_incomplete_degrades = routes().any(|ex| {
        ex.degrade_reason == Some(M5FetchRouteEntryDegradeReason::RouteFormCoverageIncomplete)
    });
    let no_clean_missing_route =
        !routes().any(|ex| ex.is_clean() && !ex.provides_complete_fetch_route);
    if !(route_classes_covered
        && continuity_break_degrades
        && form_incomplete_degrades
        && no_clean_missing_route)
    {
        violations
            .push(M5CredentialPostureFetchRouteRegistriesViolation::FetchRouteContinuityNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The repository-bootstrap families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5RepositoryBootstrapFamily; 3] = [
    M5RepositoryBootstrapFamily::CloneRemote,
    M5RepositoryBootstrapFamily::ImportBundle,
    M5RepositoryBootstrapFamily::ResumeSnapshot,
];
