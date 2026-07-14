//! Canonical seed builders for the M5 credential-posture and fetch-route registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean credential-posture and fetch-route entries are built
//! so the one stable credential-posture object resolving per acquisition path, the credential posture staying
//! handle-only with no raw secret embedded, the host trust state disclosed rather than hidden, the canonical /
//! accessible / audit resolution forms, and the complete route-endpoint / signer-continuity / digest-continuity
//! / mirror-provenance / recovery-language / trust-proof fetch-route object are proven across the
//! acquisition-engine, git, trust, diagnostics, CLI, and support surfaces without any hand-copied per-entry
//! assumption, embedded secret, hidden host trust state, dropped signer continuity, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_PACKET_ID: &str =
    "m5-bootstrap-credential-posture-and-fetch-route-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn posture(input: M5CredentialPostureEntryResolutionInput) -> M5ResolvedCredentialPostureEntry {
    resolve_credential_posture_entry(input).expect("seed credential-posture entry resolves")
}

fn route(input: M5FetchRouteEntryResolutionInput) -> M5ResolvedFetchRouteEntry {
    resolve_fetch_route_entry(input).expect("seed fetch-route entry resolves")
}

fn all_forms() -> Vec<M5TrustResolutionForm> {
    M5TrustResolutionForm::ALL.to_vec()
}

// -- Clean credential-posture entries (stable object, handle-only, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_posture_base(
    entry_id: &str,
    acquisition_path_id: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    auth_source_kind: M5CredentialAuthSourceKind,
    surface_context: M5TrustSurfaceContext,
    auth_source_ref: &str,
    proxy_or_mirror_route: &str,
    host_key_or_tls_pin_state: &str,
    delegated_token_policy: &str,
    handle_only_secret_reference: &str,
    mirror_or_signer_provenance: &str,
) -> M5CredentialPostureEntryResolutionInput {
    M5CredentialPostureEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        acquisition_path_id: acquisition_path_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        auth_source_kind,
        surface_context,
        resolution_form_coverage: all_forms(),
        auth_source_ref: auth_source_ref.to_owned(),
        proxy_or_mirror_route: proxy_or_mirror_route.to_owned(),
        host_key_or_tls_pin_state: host_key_or_tls_pin_state.to_owned(),
        delegated_token_policy: delegated_token_policy.to_owned(),
        handle_only_secret_reference: handle_only_secret_reference.to_owned(),
        mirror_or_signer_provenance: mirror_or_signer_provenance.to_owned(),
        bound_to_registry: true,
        host_trust_disclosed: true,
        references_secret_material: false,
        secret_kept_handle_only: true,
        proof_fresh: true,
    }
}

fn posture_acq_anon_clean() -> M5ResolvedCredentialPostureEntry {
    posture(clean_posture_base(
        "posture:acquisition:anonymous-public",
        "entry.acme.clone-remote-public",
        "credential.posture.anonymous_public",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::AnonymousPublic,
        M5TrustSurfaceContext::ShellSurface,
        "auth-source.acme/anonymous",
        "route.acme/public-upstream",
        "host-key.tofu-recorded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ))
}

fn posture_git_delegated_clean() -> M5ResolvedCredentialPostureEntry {
    // A delegated-token auth references secret material and keeps it handle-only, never embedded raw.
    let mut base = clean_posture_base(
        "posture:git:delegated-token",
        "entry.acme.clone-remote-delegated",
        "credential.posture.delegated_token",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::DelegatedToken,
        M5TrustSurfaceContext::EntrySurface,
        "auth-source.acme/delegated-app",
        "route.acme/direct",
        "tls-pin.pinned.v3",
        "delegated-token.short-lived.v3",
        "secret-handle.acme/ref-0007",
        "signer-provenance.acme.v3",
    );
    base.references_secret_material = true;
    base.secret_kept_handle_only = true;
    posture(base)
}

fn posture_diagnostics_hostkey_clean() -> M5ResolvedCredentialPostureEntry {
    posture(clean_posture_base(
        "posture:diagnostics:host-key",
        "entry.acme.clone-remote-pinned",
        "credential.posture.host_key_or_tls_pinned",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::HostKeyOrTlsPinned,
        M5TrustSurfaceContext::DiagnosticsSurface,
        "auth-source.acme/host-key",
        "route.acme/direct",
        "host-key.pinned.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ))
}

fn posture_admin_handle_clean() -> M5ResolvedCredentialPostureEntry {
    // A stored-handle credential references secret material via a handle-only reference over a mirror route.
    let mut base = clean_posture_base(
        "posture:admin:stored-handle",
        "entry.acme.mirrored-fetch",
        "credential.posture.stored_handle_credential",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::StoredHandleCredential,
        M5TrustSurfaceContext::AdminSurface,
        "auth-source.acme/stored-handle",
        "route.acme/mirror-eu",
        "tls-pin.pinned.v3",
        "delegated-token.not-required",
        "secret-handle.acme/ref-0042",
        "mirror-provenance.acme.v3",
    );
    base.references_secret_material = true;
    base.secret_kept_handle_only = true;
    posture(base)
}

fn posture_support_airgap_clean() -> M5ResolvedCredentialPostureEntry {
    posture(clean_posture_base(
        "posture:support:air-gap",
        "entry.acme.air-gap-import",
        "credential.posture.air_gap_offline",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::AirGapOffline,
        M5TrustSurfaceContext::SupportOrExportForm,
        "auth-source.acme/air-gap",
        "route.acme/air-gap-bundle",
        "host-key.bundle-embedded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    ))
}

// -- Degraded credential-posture entries --------------------------------------------------------

/// Degraded posture entry: the resolved credential-posture object is incomplete — the proxy / mirror route is
/// unstated.
fn posture_object_incomplete() -> M5ResolvedCredentialPostureEntry {
    let mut base = clean_posture_base(
        "posture:acquisition:incomplete",
        "entry.acme.clone-remote-public",
        "credential.posture.anonymous_public",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::AnonymousPublic,
        M5TrustSurfaceContext::ShellSurface,
        "auth-source.acme/anonymous",
        "route.acme/public-upstream",
        "host-key.tofu-recorded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    );
    base.proxy_or_mirror_route = "   ".to_owned();
    posture(base)
}

/// Degraded posture entry: a secret-referencing posture embedded raw secret material in the portable manifest
/// instead of a handle-only reference.
fn posture_embed_raw() -> M5ResolvedCredentialPostureEntry {
    let mut base = clean_posture_base(
        "posture:trust:embed-raw",
        "entry.acme.clone-remote-delegated",
        "credential.posture.delegated_token",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::DelegatedToken,
        M5TrustSurfaceContext::DiagnosticsSurface,
        "auth-source.acme/delegated-app",
        "route.acme/direct",
        "tls-pin.pinned.v3",
        "delegated-token.short-lived.v3",
        "secret-handle.acme/ref-0007",
        "signer-provenance.acme.v3",
    );
    base.references_secret_material = true;
    base.secret_kept_handle_only = false;
    posture(base)
}

/// Degraded posture entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn posture_unbound() -> M5ResolvedCredentialPostureEntry {
    let mut base = clean_posture_base(
        "posture:diagnostics:unbound",
        "entry.acme.mirrored-fetch",
        "credential.posture.stored_handle_credential",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::StoredHandleCredential,
        M5TrustSurfaceContext::AdminSurface,
        "auth-source.acme/stored-handle",
        "route.acme/mirror-eu",
        "tls-pin.pinned.v3",
        "delegated-token.not-required",
        "secret-handle.acme/ref-0042",
        "mirror-provenance.acme.v3",
    );
    base.references_secret_material = true;
    base.bound_to_registry = false;
    posture(base)
}

/// Degraded posture entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn posture_form_incomplete() -> M5ResolvedCredentialPostureEntry {
    let mut base = clean_posture_base(
        "posture:git:form-incomplete",
        "entry.acme.clone-remote-pinned",
        "credential.posture.host_key_or_tls_pinned",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::HostKeyOrTlsPinned,
        M5TrustSurfaceContext::EntrySurface,
        "auth-source.acme/host-key",
        "route.acme/direct",
        "host-key.pinned.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    );
    base.resolution_form_coverage = vec![M5TrustResolutionForm::CanonicalObject];
    posture(base)
}

/// Degraded posture entry: the canonical registry token name is unstated.
fn posture_token_unstated() -> M5ResolvedCredentialPostureEntry {
    let mut base = clean_posture_base(
        "posture:support:token-unstated",
        "entry.acme.air-gap-import",
        "  ",
        M5RepositoryBootstrapRole::CredentialPosture,
        M5CredentialAuthSourceKind::AirGapOffline,
        M5TrustSurfaceContext::SupportOrExportForm,
        "auth-source.acme/air-gap",
        "route.acme/air-gap-bundle",
        "host-key.bundle-embedded.v3",
        "delegated-token.not-required",
        "secret-handle.none",
        "signer-provenance.acme.v3",
    );
    base.token_name = "  ".to_owned();
    posture(base)
}

// -- Clean fetch-route entries ------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_route_base(
    entry_id: &str,
    source_ref: &str,
    token_name: &str,
    semantic_role: M5RepositoryBootstrapRole,
    route_class: M5FetchRouteClass,
    surface_context: M5TrustSurfaceContext,
    route_endpoint_class: &str,
    signer_continuity_ref: &str,
    digest_continuity_ref: &str,
    mirror_provenance_ref: &str,
    recovery_language: &str,
    trust_proof_ref: &str,
) -> M5FetchRouteEntryResolutionInput {
    M5FetchRouteEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        source_ref: source_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        route_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        route_endpoint_class: route_endpoint_class.to_owned(),
        signer_continuity_ref: signer_continuity_ref.to_owned(),
        digest_continuity_ref: digest_continuity_ref.to_owned(),
        mirror_provenance_ref: mirror_provenance_ref.to_owned(),
        recovery_language: recovery_language.to_owned(),
        trust_proof_ref: trust_proof_ref.to_owned(),
        keeps_signer_continuity_visible: true,
        route_is_truthful: true,
        crosses_offline_or_mirror: false,
        signer_continuity_preserved: false,
        asserts_recovery: false,
        recovery_explained: false,
        proof_fresh: true,
    }
}

fn route_public_shell_clean() -> M5ResolvedFetchRouteEntry {
    route(clean_route_base(
        "route:acquisition:public-upstream",
        "entry.acme.clone-remote-public",
        "fetch.route.public_upstream",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::PublicUpstreamFetch,
        M5TrustSurfaceContext::ShellSurface,
        "route-class.public-upstream",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    ))
}

fn route_mirror_entry_clean() -> M5ResolvedFetchRouteEntry {
    // An approved mirror fetch crosses a mirrored fetch but preserves signer / mirror provenance.
    let mut base = clean_route_base(
        "route:git:approved-mirror",
        "entry.acme.mirrored-fetch",
        "fetch.route.approved_mirror",
        M5RepositoryBootstrapRole::StagedTrust,
        M5FetchRouteClass::ApprovedMirrorFetch,
        M5TrustSurfaceContext::EntrySurface,
        "route-class.approved-mirror",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme-eu.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    );
    base.crosses_offline_or_mirror = true;
    base.signer_continuity_preserved = true;
    route(base)
}

fn route_airgap_diagnostics_clean() -> M5ResolvedFetchRouteEntry {
    // An air-gap bundle import crosses an offline fetch, preserves provenance, and explains its recovery path.
    let mut base = clean_route_base(
        "route:trust:air-gap-bundle",
        "entry.acme.air-gap-import",
        "fetch.route.air_gap_bundle",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::AirGapBundleImport,
        M5TrustSurfaceContext::DiagnosticsSurface,
        "route-class.air-gap-bundle",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    );
    base.crosses_offline_or_mirror = true;
    base.signer_continuity_preserved = true;
    base.asserts_recovery = true;
    base.recovery_explained = true;
    route(base)
}

fn route_managed_admin_clean() -> M5ResolvedFetchRouteEntry {
    let mut base = clean_route_base(
        "route:diagnostics:managed-snapshot",
        "entry.acme.managed-snapshot-resume",
        "fetch.route.managed_snapshot",
        M5RepositoryBootstrapRole::StagedTrust,
        M5FetchRouteClass::ManagedSnapshotResume,
        M5TrustSurfaceContext::AdminSurface,
        "route-class.managed-snapshot",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    );
    base.crosses_offline_or_mirror = true;
    base.signer_continuity_preserved = true;
    route(base)
}

fn route_public_support_clean() -> M5ResolvedFetchRouteEntry {
    route(clean_route_base(
        "route:support:public-upstream",
        "entry.acme.clone-remote-public",
        "fetch.route.public_upstream",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::PublicUpstreamFetch,
        M5TrustSurfaceContext::SupportOrExportForm,
        "route-class.public-upstream",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    ))
}

// -- Degraded fetch-route entries ---------------------------------------------------------------

/// Degraded route entry: the route would lose signer / mirror provenance across a mirrored fetch — the route
/// reads as safe when it has quietly dropped signer continuity.
fn route_continuity_break() -> M5ResolvedFetchRouteEntry {
    let mut base = clean_route_base(
        "route:acquisition:continuity-break",
        "entry.acme.mirrored-fetch",
        "fetch.route.approved_mirror",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::ApprovedMirrorFetch,
        M5TrustSurfaceContext::ShellSurface,
        "route-class.approved-mirror",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme-eu.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    );
    base.crosses_offline_or_mirror = true;
    base.signer_continuity_preserved = false;
    route(base)
}

/// Degraded route entry: the canonical / accessible / audit resolution-form coverage of the route is
/// incomplete.
fn route_form_incomplete() -> M5ResolvedFetchRouteEntry {
    let mut base = clean_route_base(
        "route:git:form-incomplete",
        "entry.acme.clone-remote-public",
        "fetch.route.public_upstream",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::PublicUpstreamFetch,
        M5TrustSurfaceContext::EntrySurface,
        "route-class.public-upstream",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    );
    base.resolution_form_coverage = vec![M5TrustResolutionForm::CanonicalObject];
    route(base)
}

/// Degraded route entry: the fetch-route class is unclassified.
fn route_class_unclassified() -> M5ResolvedFetchRouteEntry {
    route(clean_route_base(
        "route:diagnostics:class-unclassified",
        "entry.acme.managed-snapshot-resume",
        "fetch.route.unknown",
        M5RepositoryBootstrapRole::EvidencePacket,
        M5FetchRouteClass::RouteUnclassified,
        M5TrustSurfaceContext::AdminSurface,
        "route-class.public-upstream",
        "signer-continuity.acme.v3",
        "digest-continuity.acme.v3",
        "mirror-provenance.acme.v3",
        "recovery.resume-or-discard",
        "trust-proof.acme.v3",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5CredentialPostureFetchRouteRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5RepositoryBootstrapDowngradeTrigger>,
    credential_posture_entries: Vec<M5ResolvedCredentialPostureEntry>,
    fetch_route_entries: Vec<M5ResolvedFetchRouteEntry>,
) -> M5CredentialPostureFetchRouteRegistriesRow {
    M5CredentialPostureFetchRouteRegistriesRow {
        consumer_surface,
        qualification: M5RepositoryBootstrapQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5RepositoryBootstrapDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5RepositoryBootstrapRequiredLabel::Identity,
            M5RepositoryBootstrapRequiredLabel::SemanticRole,
            M5RepositoryBootstrapRequiredLabel::RegistryReference,
            M5RepositoryBootstrapRequiredLabel::CredentialPosture,
            M5RepositoryBootstrapRequiredLabel::CheckoutPlan,
        ],
        accessibility_routes: M5RepositoryBootstrapAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TrustAnatomyPart::ALL.to_vec(),
        export_fields: M5TrustExportField::ALL.to_vec(),
        downgrade_triggers,
        credential_posture_entries,
        fetch_route_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_REF,
            M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
            M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
        ]),
        embeds_raw_secret_token_or_host_trust_state_in_portable_manifest: false,
        loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: false,
        hides_bootstrap_credential_posture_behind_generic_connected_state_copy: false,
        collapses_distinct_fetch_routes_into_one_runtime_path: false,
    }
}

fn registry_rows() -> Vec<M5CredentialPostureFetchRouteRegistriesRow> {
    use M5RepositoryBootstrapConsumerSurface as C;
    use M5RepositoryBootstrapDowngradeTrigger as D;

    vec![
        base_row(
            C::AcquisitionEngine,
            "Acquisition-engine owner",
            "The acquisition engine resolves the anonymous-public credential posture to one stable object — auth-source reference, proxy / mirror route, host-key / TLS-pin state, delegated-token policy, handle-only secret reference, and mirror / signer provenance — from the shared registry and derives the public-upstream fetch route; a posture object missing its proxy / mirror route and a route that drops signer continuity across a mirrored fetch degrade honestly instead of reading as a clean pass",
            "evidence:m5-repository-bootstrap-acquisition-engine:001",
            vec![
                D::CredentialPostureUnstated,
                D::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
                D::ProofStale,
            ],
            vec![posture_acq_anon_clean(), posture_object_incomplete()],
            vec![route_public_shell_clean(), route_continuity_break()],
        ),
        base_row(
            C::GitService,
            "Git-service owner",
            "The git service resolves the delegated-token credential posture while keeping the secret handle-only, and renders the approved-mirror fetch route with signer continuity preserved; a resolution-form gap on a posture entry and on a fetch route is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-repository-bootstrap-git-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![posture_git_delegated_clean(), posture_form_incomplete()],
            vec![route_mirror_entry_clean(), route_form_incomplete()],
        ),
        base_row(
            C::TrustService,
            "Trust-service owner",
            "The trust service reports the host-key / TLS-pinned credential posture and the air-gap bundle-import route without manual reconstruction; a delegated-token posture that embedded raw secret material in the portable manifest instead of a handle-only reference is caught as a secret embed",
            "evidence:m5-repository-bootstrap-trust-service:001",
            vec![
                D::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![posture_diagnostics_hostkey_clean(), posture_embed_raw()],
            vec![route_airgap_diagnostics_clean()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics resolves the stored-handle credential posture while keeping the secret handle-only and bound to the registry, and renders the managed-snapshot resume route; a posture that is a hand-copied per-entry assumption and a fetch route on an unclassified class degrade honestly",
            "evidence:m5-repository-bootstrap-diagnostics:001",
            vec![
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![posture_admin_handle_clean(), posture_unbound()],
            vec![route_managed_admin_clean(), route_class_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI-export owner",
            "The CLI export renders the same resolved credential-posture and fetch-route truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied credential table",
            "evidence:m5-repository-bootstrap-cli-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CheckoutPlanBoundaryDriftedBySurface,
                D::ProofStale,
            ],
            vec![posture_diagnostics_hostkey_clean(), posture_form_incomplete()],
            vec![route_mirror_entry_clean(), route_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved credential-posture and fetch-route truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, an embedded secret, or a dropped signer continuity is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-repository-bootstrap-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::HidBootstrapCredentialPostureBehindGenericConnectedStateCopy,
                D::ProofStale,
            ],
            vec![posture_support_airgap_clean(), posture_token_unstated()],
            vec![route_public_support_clean()],
        ),
    ]
}

fn governance_review() -> M5CredentialPostureFetchRouteRegistriesGovernanceReview {
    M5CredentialPostureFetchRouteRegistriesGovernanceReview {
        credential_registry_names_token_role_and_source: true,
        entry_flow_resolves_to_stable_posture_from_shared_registry: true,
        auth_source_route_host_trust_and_provenance_published: true,
        credential_posture_stays_handle_only_no_raw_secret: true,
        fetch_route_keeps_signer_continuity_and_trust_proof: true,
        host_trust_state_disclosed_not_generic_copy: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        acquisition_git_trust_diagnostics_read_single_source: true,
        posture_or_route_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5CredentialPostureFetchRouteRegistriesConsumerProjection {
    M5CredentialPostureFetchRouteRegistriesConsumerProjection {
        acquisition_and_git_consume_shared_registries: true,
        trust_and_diagnostics_consume_shared_registries: true,
        cli_and_support_export_consume_shared_registries: true,
        docs_help_and_workspace_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5CredentialPostureFetchRouteRegistriesProofFreshness {
    M5CredentialPostureFetchRouteRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CredentialPostureFetchRouteRegistriesReleasePosture {
    M5CredentialPostureFetchRouteRegistriesReleasePosture {
        proof_packet_ref: M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        repository_bootstrap_audit_ref:
            M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_SCHEMA_REF,
        M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_DOC_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF,
        M5_REPOSITORY_BOOTSTRAP_MATRIX_DOC_REF,
        M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF,
        M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 credential-posture and fetch-route registries packet.
pub fn seeded_m5_bootstrap_credential_posture_and_fetch_route_registries(
) -> M5CredentialPostureFetchRouteRegistriesPacket {
    M5CredentialPostureFetchRouteRegistriesPacket::new(
        M5CredentialPostureFetchRouteRegistriesPacketInput {
            packet_id: M5_BOOTSTRAP_CREDENTIAL_POSTURE_FETCH_ROUTE_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 bootstrap credential-posture and fetch-route registries with one stable credential-posture object resolving per acquisition path, the posture staying handle-only with no raw secret embedded and host trust disclosed, canonical / accessible / audit resolution-form coverage, and the complete route-endpoint / signer-continuity / digest-continuity / mirror-provenance / recovery-language / trust-proof fetch-route object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5CredentialPostureFetchRouteRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the trust-service row is held at Beta pending air-gap bundle-import signer-continuity
/// parity on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_air_gap_bundle_beta_narrowed(
) -> M5CredentialPostureFetchRouteRegistriesPacket {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.packet_id =
        "m5-bootstrap-credential-posture-and-fetch-route-registries:air-gap-bundle-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .expect("trust-service row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending managed-snapshot resume parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_bootstrap_credential_posture_and_fetch_route_registries_managed_snapshot_preview_narrowed(
) -> M5CredentialPostureFetchRouteRegistriesPacket {
    let mut packet = seeded_m5_bootstrap_credential_posture_and_fetch_route_registries();
    packet.packet_id =
        "m5-bootstrap-credential-posture-and-fetch-route-registries:managed-snapshot-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5RepositoryBootstrapQualificationClass::Preview;
    packet
}
