//! Canonical seed builders for the M5 build-lane-descriptor and reproducibility-proof registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean build-lane-descriptor and reproducibility-proof
//! entries are built so the one typed build-lane-descriptor object resolving per lane, untrusted lanes never
//! publishing release artifacts, the cache-trust marker disclosed before any trust-risk cache is read, the
//! canonical / accessible / audit resolution forms, and the complete build-identity / input-source-ledger /
//! clean-room-diff / sidecar-convergence / attestation / rollback-metadata / last-rebuild-revision
//! reproducibility-proof object are proven across the build-farm, cache-service, release-center, provenance,
//! diagnostics, and support surfaces without any hand-copied per-lane assumption, publish-from-untrusted,
//! incomplete object, hidden input source, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_PACKET_ID: &str =
    "m5-build-lane-descriptor-and-reproducibility-proof-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn descriptor(
    input: M5BuildLaneDescriptorEntryResolutionInput,
) -> M5ResolvedBuildLaneDescriptorEntry {
    resolve_build_lane_descriptor_entry(input).expect("seed build-lane-descriptor entry resolves")
}

fn proof(input: M5ReproducibilityProofEntryResolutionInput) -> M5ResolvedReproducibilityProofEntry {
    resolve_reproducibility_proof_entry(input).expect("seed reproducibility-proof entry resolves")
}

fn all_forms() -> Vec<M5BuildLaneResolutionForm> {
    M5BuildLaneResolutionForm::ALL.to_vec()
}

// -- Clean build-lane-descriptor entries (typed object, bounded publication, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_descriptor_base(
    entry_id: &str,
    lane_binding_id: &str,
    token_name: &str,
    semantic_role: M5BuildLaneTrustRole,
    cache_posture: M5BuildLaneCachePostureKind,
    surface_context: M5BuildLaneSurfaceContext,
    cache_read_scope: &str,
    cache_write_scope: &str,
    credential_class: &str,
    publication_rights: &str,
    expected_artifact_families: &str,
    hermetic_input_posture: &str,
    clean_room_rebuild_rule: &str,
) -> M5BuildLaneDescriptorEntryResolutionInput {
    M5BuildLaneDescriptorEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        lane_binding_id: lane_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        cache_posture,
        surface_context,
        resolution_form_coverage: all_forms(),
        cache_read_scope: cache_read_scope.to_owned(),
        cache_write_scope: cache_write_scope.to_owned(),
        credential_class: credential_class.to_owned(),
        publication_rights: publication_rights.to_owned(),
        expected_artifact_families: expected_artifact_families.to_owned(),
        hermetic_input_posture: hermetic_input_posture.to_owned(),
        clean_room_rebuild_rule: clean_room_rebuild_rule.to_owned(),
        bound_to_registry: true,
        publication_authority_bounded: true,
        is_trust_risk_posture: false,
        cache_trust_disclosed: true,
        proof_fresh: true,
    }
}

fn descriptor_hermetic_release_center_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:release-center:hermetic",
        "release.lane.release",
        "build.lane.descriptor.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5BuildLaneCachePostureKind::HermeticNoCache,
        M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        "cache.read.none",
        "cache.write.none",
        "credential.release-signing-scoped",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms-symbols-docs",
        "hermetic.fully-hermetic",
        "clean-room.full-rebuild-required",
    ))
}

fn descriptor_verified_shiproom_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:shiproom:verified",
        "release.lane.protected-merge",
        "build.lane.descriptor.protected_merge",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5BuildLaneCachePostureKind::VerifiedInputsOnly,
        M5BuildLaneSurfaceContext::ShiproomSurface,
        "cache.read.verified-inputs-only",
        "cache.write.protected-merge-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms",
        "hermetic.verified-inputs",
        "clean-room.digest-required",
    ))
}

fn descriptor_shared_untrusted_diagnostics_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    // A shared-untrusted cache posture is trust-risk-bearing and discloses its cache-trust marker.
    let mut base = clean_descriptor_base(
        "descriptor:diagnostics:shared-untrusted",
        "release.lane.contributor-pr",
        "build.lane.descriptor.contributor_pr",
        M5BuildLaneTrustRole::CachePosture,
        M5BuildLaneCachePostureKind::SharedReadableUntrusted,
        M5BuildLaneSurfaceContext::DiagnosticsSurface,
        "cache.read.shared-readable",
        "cache.write.none",
        "credential.pr-scoped-only",
        "publication.withheld",
        "artifacts.none-release-bearing",
        "hermetic.best-effort",
        "clean-room.not-applicable",
    );
    base.is_trust_risk_posture = true;
    base.cache_trust_disclosed = true;
    descriptor(base)
}

fn descriptor_remote_publishing_provenance_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    // A remote-publishing cache posture is trust-risk-bearing and discloses its cache-trust marker.
    let mut base = clean_descriptor_base(
        "descriptor:provenance:remote-publishing",
        "release.lane.emergency-hotfix",
        "build.lane.descriptor.emergency_hotfix",
        M5BuildLaneTrustRole::PublicationAuthority,
        M5BuildLaneCachePostureKind::RemotePublishingCache,
        M5BuildLaneSurfaceContext::ProvenanceSurface,
        "cache.read.verified-inputs-only",
        "cache.write.release-channel-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-rollback-metadata-support-packets",
        "hermetic.verified-inputs",
        "clean-room.rematerialize-required",
    );
    base.is_trust_risk_posture = true;
    base.cache_trust_disclosed = true;
    descriptor(base)
}

fn descriptor_mirror_replay_support_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:support:mirror-replay",
        "release.lane.protected-merge",
        "build.lane.descriptor.protected_merge",
        M5BuildLaneTrustRole::HermeticInput,
        M5BuildLaneCachePostureKind::MirrorReplayCache,
        M5BuildLaneSurfaceContext::SupportOrExportForm,
        "cache.read.mirror-replay",
        "cache.write.mirror-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms-schemas",
        "hermetic.mirror-verified",
        "clean-room.mirror-replay-rule",
    ))
}

fn descriptor_verified_support_clean() -> M5ResolvedBuildLaneDescriptorEntry {
    descriptor(clean_descriptor_base(
        "descriptor:support:verified",
        "release.lane.release",
        "build.lane.descriptor.release",
        M5BuildLaneTrustRole::SupportIdentity,
        M5BuildLaneCachePostureKind::VerifiedInputsOnly,
        M5BuildLaneSurfaceContext::SupportOrExportForm,
        "cache.read.verified-inputs-only",
        "cache.write.release-channel-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.support-packets-symbols",
        "hermetic.verified-inputs",
        "clean-room.digest-required",
    ))
}

// -- Degraded build-lane-descriptor entries -----------------------------------------------------

/// Degraded descriptor entry: the resolved descriptor object is incomplete — the cache write scope is unstated.
fn descriptor_object_incomplete() -> M5ResolvedBuildLaneDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:build-farm:incomplete",
        "release.lane.release",
        "build.lane.descriptor.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5BuildLaneCachePostureKind::HermeticNoCache,
        M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        "cache.read.none",
        "cache.write.none",
        "credential.release-signing-scoped",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms-symbols-docs",
        "hermetic.fully-hermetic",
        "clean-room.full-rebuild-required",
    );
    base.cache_write_scope = "   ".to_owned();
    descriptor(base)
}

/// Degraded descriptor entry: the lane's publication authority is unbounded — a lane claiming release publish
/// rights it must not have. The structured blocker reason for a publish-from-untrusted-lane attempt.
fn descriptor_publish_fold() -> M5ResolvedBuildLaneDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:release-center:publish-fold",
        "release.lane.contributor-pr",
        "build.lane.descriptor.contributor_pr",
        M5BuildLaneTrustRole::PublicationAuthority,
        M5BuildLaneCachePostureKind::VerifiedInputsOnly,
        M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        "cache.read.verified-inputs-only",
        "cache.write.none",
        "credential.pr-scoped-only",
        "publication.claims-release-publication",
        "artifacts.binaries-packages",
        "hermetic.best-effort",
        "clean-room.not-applicable",
    );
    base.publication_authority_bounded = false;
    descriptor(base)
}

/// Degraded descriptor entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn descriptor_unbound() -> M5ResolvedBuildLaneDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:provenance:unbound",
        "release.lane.emergency-hotfix",
        "build.lane.descriptor.emergency_hotfix",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5BuildLaneCachePostureKind::VerifiedInputsOnly,
        M5BuildLaneSurfaceContext::ProvenanceSurface,
        "cache.read.verified-inputs-only",
        "cache.write.release-channel-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-rollback-metadata",
        "hermetic.verified-inputs",
        "clean-room.rematerialize-required",
    );
    base.bound_to_registry = false;
    descriptor(base)
}

/// Degraded descriptor entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn descriptor_form_incomplete() -> M5ResolvedBuildLaneDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:cache-service:form-incomplete",
        "release.lane.protected-merge",
        "build.lane.descriptor.protected_merge",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5BuildLaneCachePostureKind::VerifiedInputsOnly,
        M5BuildLaneSurfaceContext::ShiproomSurface,
        "cache.read.verified-inputs-only",
        "cache.write.protected-merge-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms",
        "hermetic.verified-inputs",
        "clean-room.digest-required",
    );
    base.resolution_form_coverage = vec![M5BuildLaneResolutionForm::CanonicalObject];
    descriptor(base)
}

/// Degraded descriptor entry: the canonical registry token name is unstated.
fn descriptor_token_unstated() -> M5ResolvedBuildLaneDescriptorEntry {
    let mut base = clean_descriptor_base(
        "descriptor:diagnostics:token-unstated",
        "release.lane.contributor-pr",
        "  ",
        M5BuildLaneTrustRole::CachePosture,
        M5BuildLaneCachePostureKind::MirrorReplayCache,
        M5BuildLaneSurfaceContext::DiagnosticsSurface,
        "cache.read.mirror-replay",
        "cache.write.mirror-scoped",
        "credential.controlled-scoped-to-lane",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages",
        "hermetic.mirror-verified",
        "clean-room.mirror-replay-rule",
    );
    base.token_name = "  ".to_owned();
    descriptor(base)
}

// -- Clean reproducibility-proof entries --------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_proof_base(
    entry_id: &str,
    proof_ref: &str,
    token_name: &str,
    semantic_role: M5BuildLaneTrustRole,
    convergence_scope: M5ReproducibilityConvergenceScope,
    surface_context: M5BuildLaneSurfaceContext,
    resolved_build_identity: &str,
    input_source_ledger: &str,
    clean_room_diff_reference: &str,
    sidecar_convergence_state: &str,
    attestation_state: &str,
    rollback_metadata_reference: &str,
    last_rebuild_revision: &str,
) -> M5ReproducibilityProofEntryResolutionInput {
    M5ReproducibilityProofEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        proof_ref: proof_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        convergence_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_build_identity: resolved_build_identity.to_owned(),
        input_source_ledger: input_source_ledger.to_owned(),
        clean_room_diff_reference: clean_room_diff_reference.to_owned(),
        sidecar_convergence_state: sidecar_convergence_state.to_owned(),
        attestation_state: attestation_state.to_owned(),
        rollback_metadata_reference: rollback_metadata_reference.to_owned(),
        last_rebuild_revision: last_rebuild_revision.to_owned(),
        keeps_input_source_visible: true,
        proof_is_truthful: true,
        remote_cache_hit_present: false,
        remote_cache_hit_marked_not_proof: false,
        non_hermetic_input_present: false,
        non_hermetic_input_flagged: false,
        proof_fresh: true,
    }
}

fn proof_verified_cache_clean() -> M5ResolvedReproducibilityProofEntry {
    // A verified-cache build reads a remote cache but marks the hit as never being reproducibility proof.
    let mut base = clean_proof_base(
        "proof:release-center:verified-cache",
        "release.lane.release",
        "reproducibility.proof.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5ReproducibilityConvergenceScope::VerifiedCacheInputs,
        M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        "build-id.sha256.release-0007",
        "inputs.verified-cache",
        "clean-room.diff.release-0007",
        "sidecars.converged-docs-schemas-sboms-symbols",
        "attestation.signed-and-verified",
        "rollback.metadata.release-0007",
        "rebuild.revision.0007",
    );
    base.remote_cache_hit_present = true;
    base.remote_cache_hit_marked_not_proof = true;
    proof(base)
}

fn proof_rematerialized_clean() -> M5ResolvedReproducibilityProofEntry {
    proof(clean_proof_base(
        "proof:shiproom:rematerialized",
        "release.lane.protected-merge",
        "reproducibility.proof.protected_merge",
        M5BuildLaneTrustRole::ArtifactConvergence,
        M5ReproducibilityConvergenceScope::RematerializedInputs,
        M5BuildLaneSurfaceContext::ShiproomSurface,
        "build-id.sha256.protected-merge-0007",
        "inputs.rematerialized-from-source",
        "clean-room.diff.protected-merge-0007",
        "sidecars.converged-docs-schemas-sboms",
        "attestation.signed-and-verified",
        "rollback.metadata.protected-merge-0007",
        "rebuild.revision.0007",
    ))
}

fn proof_hermetic_rebuild_clean() -> M5ResolvedReproducibilityProofEntry {
    proof(clean_proof_base(
        "proof:diagnostics:hermetic-rebuild",
        "release.lane.emergency-hotfix",
        "reproducibility.proof.emergency_hotfix",
        M5BuildLaneTrustRole::SupportIdentity,
        M5ReproducibilityConvergenceScope::HermeticRebuildInputs,
        M5BuildLaneSurfaceContext::DiagnosticsSurface,
        "build-id.sha256.emergency-hotfix-0007",
        "inputs.hermetic-rebuild",
        "clean-room.diff.emergency-hotfix-0007",
        "sidecars.converged-rollback-metadata-support-packets",
        "attestation.signed-and-verified",
        "rollback.metadata.emergency-hotfix-0007",
        "rebuild.revision.0007",
    ))
}

// -- Degraded reproducibility-proof entries -----------------------------------------------------

/// Degraded proof entry: the proof would treat a remote-cache hit as reproducibility proof — a cache hit that
/// is not marked as never being proof reads as trustworthy when the binaries are not actually replayable.
fn proof_treats_cache_hit_as_proof() -> M5ResolvedReproducibilityProofEntry {
    let mut base = clean_proof_base(
        "proof:build-farm:cache-hit-as-proof",
        "release.lane.release",
        "reproducibility.proof.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5ReproducibilityConvergenceScope::VerifiedCacheInputs,
        M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        "build-id.sha256.release-0007",
        "inputs.verified-cache",
        "clean-room.diff.release-0007",
        "sidecars.converged-docs-schemas-sboms-symbols",
        "attestation.signed-and-verified",
        "rollback.metadata.release-0007",
        "rebuild.revision.0007",
    );
    base.remote_cache_hit_present = true;
    base.remote_cache_hit_marked_not_proof = false;
    proof(base)
}

/// Degraded proof entry: the canonical / accessible / audit resolution-form coverage of the proof is
/// incomplete.
fn proof_form_incomplete() -> M5ResolvedReproducibilityProofEntry {
    let mut base = clean_proof_base(
        "proof:cache-service:form-incomplete",
        "release.lane.protected-merge",
        "reproducibility.proof.protected_merge",
        M5BuildLaneTrustRole::ArtifactConvergence,
        M5ReproducibilityConvergenceScope::RematerializedInputs,
        M5BuildLaneSurfaceContext::ShiproomSurface,
        "build-id.sha256.protected-merge-0007",
        "inputs.rematerialized-from-source",
        "clean-room.diff.protected-merge-0007",
        "sidecars.converged-docs-schemas-sboms",
        "attestation.signed-and-verified",
        "rollback.metadata.protected-merge-0007",
        "rebuild.revision.0007",
    );
    base.resolution_form_coverage = vec![M5BuildLaneResolutionForm::CanonicalObject];
    proof(base)
}

/// Degraded proof entry: the convergence scope is unclassified.
fn proof_scope_unclassified() -> M5ResolvedReproducibilityProofEntry {
    proof(clean_proof_base(
        "proof:provenance:scope-unclassified",
        "release.lane.emergency-hotfix",
        "reproducibility.proof.emergency_hotfix",
        M5BuildLaneTrustRole::SupportIdentity,
        M5ReproducibilityConvergenceScope::ScopeUnclassified,
        M5BuildLaneSurfaceContext::ProvenanceSurface,
        "build-id.sha256.emergency-hotfix-0007",
        "inputs.hermetic-rebuild",
        "clean-room.diff.emergency-hotfix-0007",
        "sidecars.converged-rollback-metadata-support-packets",
        "attestation.signed-and-verified",
        "rollback.metadata.emergency-hotfix-0007",
        "rebuild.revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildLaneDowngradeTrigger>,
    build_lane_descriptor_entries: Vec<M5ResolvedBuildLaneDescriptorEntry>,
    reproducibility_proof_entries: Vec<M5ResolvedReproducibilityProofEntry>,
) -> M5BuildLaneDescriptorReproducibilityProofRegistriesRow {
    M5BuildLaneDescriptorReproducibilityProofRegistriesRow {
        consumer_surface,
        qualification: M5BuildLaneQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5BuildLaneDeploymentLine::ALL.to_vec(),
        required_labels: M5BuildLaneRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5BuildLaneAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5BuildLaneAnatomyPart::ALL.to_vec(),
        export_fields: M5BuildLaneExportField::ALL.to_vec(),
        downgrade_triggers,
        build_lane_descriptor_entries,
        reproducibility_proof_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_REF,
            M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
        ]),
        lets_a_pr_or_contributor_lane_publish_release_artifacts: false,
        treats_a_remote_cache_hit_as_reproducibility_proof: false,
        hides_the_cache_credential_or_publication_boundary_before_promotion: false,
        collapses_distinct_build_lane_input_sources_into_one_path: false,
    }
}

fn registry_rows() -> Vec<M5BuildLaneDescriptorReproducibilityProofRegistriesRow> {
    use M5BuildLaneConsumerSurface as C;
    use M5BuildLaneDowngradeTrigger as D;

    vec![
        base_row(
            C::BuildFarm,
            "Build-farm owner",
            "The build farm resolves the release lane's build-lane descriptor to one typed object — cache posture, cache read / write scopes, controlled credential class, publication rights, expected artifact families, hermetic-input posture, and clean-room rebuild rule — from the shared registry and proves the verified-cache reproducibility proof for the winning build identity; a descriptor object missing its cache write scope and a proof that treats a remote-cache hit as reproducibility proof degrade honestly instead of reading as a clean pass",
            "evidence:m5-build-lane-trust-build-farm:001",
            vec![
                D::PublishedReleaseArtifactsFromAPrCache,
                D::TreatedARemoteCacheHitAsReproducibilityProof,
                D::ProofStale,
            ],
            vec![
                descriptor_hermetic_release_center_clean(),
                descriptor_object_incomplete(),
            ],
            vec![proof_verified_cache_clean(), proof_treats_cache_hit_as_proof()],
        ),
        base_row(
            C::CacheService,
            "Cache-service owner",
            "The cache service resolves the protected-merge descriptor and the re-materialized reproducibility proof while keeping the verified-versus-re-materialized input source visible; a resolution-form gap on a descriptor entry and on a proof is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-build-lane-trust-cache-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CleanRoomProofRuleUnstated,
                D::ProofStale,
            ],
            vec![descriptor_verified_shiproom_clean(), descriptor_form_incomplete()],
            vec![proof_rematerialized_clean(), proof_form_incomplete()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the contributor / PR descriptor while disclosing its shared-untrusted cache-trust marker and reports the hermetic-rebuild reproducibility proof; a lane claiming release publish rights it must not have is caught as a publish-from-untrusted-lane blocker before it can publish a release artifact",
            "evidence:m5-build-lane-trust-release-center:001",
            vec![
                D::PublishedReleaseArtifactsFromAPrCache,
                D::CachePostureUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_shared_untrusted_diagnostics_clean(),
                descriptor_publish_fold(),
            ],
            vec![proof_hermetic_rebuild_clean()],
        ),
        base_row(
            C::ProvenanceService,
            "Provenance-service owner",
            "The provenance service resolves the emergency-hotfix descriptor while disclosing its remote-publishing cache-trust marker and bound to the registry; a descriptor that is a hand-copied per-entry assumption and a proof on an unclassified convergence scope degrade honestly",
            "evidence:m5-build-lane-trust-provenance-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::BuildIdentityUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_remote_publishing_provenance_clean(),
                descriptor_unbound(),
            ],
            vec![proof_scope_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved build-lane-descriptor and reproducibility-proof truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied lane table; an unstated registry token is caught before it can drift",
            "evidence:m5-build-lane-trust-diagnostics:001",
            vec![
                D::CachePostureUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                descriptor_mirror_replay_support_clean(),
                descriptor_token_unstated(),
            ],
            vec![proof_verified_cache_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved build-lane-descriptor and reproducibility-proof truth, so a hand-copied constant, an unstated registry token, a publish-from-untrusted attempt, or a cache hit treated as proof is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-build-lane-trust-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::BuildIdentityUnstated,
                D::ProofStale,
            ],
            vec![descriptor_verified_support_clean()],
            vec![proof_rematerialized_clean()],
        ),
    ]
}

fn governance_review() -> M5BuildLaneDescriptorReproducibilityProofRegistriesGovernanceReview {
    M5BuildLaneDescriptorReproducibilityProofRegistriesGovernanceReview {
        descriptor_registry_names_token_role_and_posture: true,
        lane_resolves_to_typed_descriptor_from_shared_registry: true,
        cache_credential_publication_and_artifact_families_published: true,
        untrusted_lanes_cannot_publish_release_artifacts: true,
        reproducibility_proof_keeps_input_source_visible_and_marks_cache_hits_not_proof: true,
        cache_trust_disclosed_for_trust_risk_postures: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        release_center_shiproom_diagnostics_and_provenance_read_single_source: true,
        descriptor_or_proof_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerProjection {
    M5BuildLaneDescriptorReproducibilityProofRegistriesConsumerProjection {
        release_center_and_shiproom_consume_shared_registries: true,
        diagnostics_and_provenance_consume_shared_registries: true,
        build_farm_and_cache_service_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5BuildLaneDescriptorReproducibilityProofRegistriesProofFreshness {
    M5BuildLaneDescriptorReproducibilityProofRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BuildLaneDescriptorReproducibilityProofRegistriesReleasePosture {
    M5BuildLaneDescriptorReproducibilityProofRegistriesReleasePosture {
        proof_packet_ref: M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        build_lane_audit_ref: M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_SCHEMA_REF,
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_DOC_REF,
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 build-lane-descriptor and reproducibility-proof registries packet.
pub fn seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries(
) -> M5BuildLaneDescriptorReproducibilityProofRegistriesPacket {
    M5BuildLaneDescriptorReproducibilityProofRegistriesPacket::new(
        M5BuildLaneDescriptorReproducibilityProofRegistriesPacketInput {
            packet_id: M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_PACKET_ID
                .to_owned(),
            registries_label:
                "M5 build-lane-descriptor and reproducibility-proof registries with one typed build-lane-descriptor object resolving per lane, untrusted lanes never publishing release artifacts, the cache-trust marker disclosed before any trust-risk cache is read, canonical / accessible / audit resolution-form coverage, and the complete build-identity / input-source-ledger / clean-room-diff / sidecar-convergence / attestation / rollback-metadata / last-rebuild-revision reproducibility-proof object across build-farm, cache-service, release-center, provenance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5BuildLaneDescriptorReproducibilityProofRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the build-farm row is held at Beta pending build-lane-descriptor parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_build_lane_descriptor_beta_narrowed(
) -> M5BuildLaneDescriptorReproducibilityProofRegistriesPacket {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.packet_id =
        "m5-build-lane-descriptor-and-reproducibility-proof-registries:build-lane-descriptor-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildLaneConsumerSurface::BuildFarm)
        .expect("build-farm row present");
    row.qualification = M5BuildLaneQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending reproducibility-proof parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_reproducibility_proof_preview_narrowed(
) -> M5BuildLaneDescriptorReproducibilityProofRegistriesPacket {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.packet_id =
        "m5-build-lane-descriptor-and-reproducibility-proof-registries:reproducibility-proof-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildLaneConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5BuildLaneQualificationClass::Preview;
    packet
}
