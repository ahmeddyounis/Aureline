//! Canonical seed builders for the M5 verified-input-manifest and sidecar-completeness-manifest registries
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean verified-input-manifest and
//! sidecar-completeness-manifest entries are built so the one typed verified-input-manifest object resolving per
//! lane, unverified inputs never entering protected lanes, the input-trust marker disclosed before any
//! trust-risk input is admitted, the canonical / accessible / audit resolution forms, and the complete
//! build-identity / claimed-families / sidecar-ledger / binding-identity / missing-or-mismatched / attestation /
//! last-convergence-revision sidecar-completeness object are proven across the build-farm, cache-service,
//! release-center, provenance, diagnostics, and support surfaces without any hand-copied per-lane assumption,
//! admit-unverified-input, incomplete object, missing sidecar, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_PACKET_ID: &str =
    "m5-verified-input-manifest-and-sidecar-completeness-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-15T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn manifest(
    input: M5VerifiedInputManifestEntryResolutionInput,
) -> M5ResolvedVerifiedInputManifestEntry {
    resolve_verified_input_manifest_entry(input)
        .expect("seed verified-input-manifest entry resolves")
}

fn sidecar(
    input: M5SidecarCompletenessManifestEntryResolutionInput,
) -> M5ResolvedSidecarCompletenessManifestEntry {
    resolve_sidecar_completeness_manifest_entry(input)
        .expect("seed sidecar-completeness-manifest entry resolves")
}

fn all_forms() -> Vec<M5ExactBuildResolutionForm> {
    M5ExactBuildResolutionForm::ALL.to_vec()
}

// -- Clean verified-input-manifest entries (typed object, bounded admission, bound to the registry) --

#[allow(clippy::too_many_arguments)]
fn clean_manifest_base(
    entry_id: &str,
    lane_binding_id: &str,
    token_name: &str,
    semantic_role: M5BuildLaneTrustRole,
    input_source: M5VerifiedInputSourceKind,
    surface_context: M5ExactBuildSurfaceContext,
    build_config_digest: &str,
    materialized_input_receipt: &str,
    input_provenance_ledger: &str,
    verification_authority: &str,
    expected_artifact_families: &str,
    hermetic_input_posture: &str,
    re_materialization_rule: &str,
) -> M5VerifiedInputManifestEntryResolutionInput {
    M5VerifiedInputManifestEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        lane_binding_id: lane_binding_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        input_source,
        surface_context,
        resolution_form_coverage: all_forms(),
        build_config_digest: build_config_digest.to_owned(),
        materialized_input_receipt: materialized_input_receipt.to_owned(),
        input_provenance_ledger: input_provenance_ledger.to_owned(),
        verification_authority: verification_authority.to_owned(),
        expected_artifact_families: expected_artifact_families.to_owned(),
        hermetic_input_posture: hermetic_input_posture.to_owned(),
        re_materialization_rule: re_materialization_rule.to_owned(),
        bound_to_registry: true,
        input_admission_bounded: true,
        is_trust_risk_source: false,
        input_trust_disclosed: true,
        proof_fresh: true,
    }
}

fn manifest_rematerialized_release_center_clean() -> M5ResolvedVerifiedInputManifestEntry {
    manifest(clean_manifest_base(
        "manifest:release-center:rematerialized",
        "release.lane.release",
        "verified.input.manifest.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5VerifiedInputSourceKind::RematerializedFromSource,
        M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms-symbols-docs",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ))
}

fn manifest_verified_cache_shiproom_clean() -> M5ResolvedVerifiedInputManifestEntry {
    manifest(clean_manifest_base(
        "manifest:shiproom:verified-cache",
        "release.lane.protected-merge",
        "verified.input.manifest.protected_merge",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5VerifiedInputSourceKind::VerifiedCacheInput,
        M5ExactBuildSurfaceContext::ShiproomSurface,
        "build-config.sha256.protected-merge-0007",
        "receipt.materialized.protected-merge-0007",
        "provenance.ledger.protected-merge-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages-sboms",
        "hermetic.verified-inputs",
        "rematerialize.digest-required",
    ))
}

fn manifest_unverified_external_diagnostics_clean() -> M5ResolvedVerifiedInputManifestEntry {
    // An unverified-external input source is trust-risk-bearing and discloses its input-trust marker.
    let mut base = clean_manifest_base(
        "manifest:diagnostics:unverified-external",
        "release.lane.contributor-pr",
        "verified.input.manifest.contributor_pr",
        M5BuildLaneTrustRole::CachePosture,
        M5VerifiedInputSourceKind::UnverifiedExternalInput,
        M5ExactBuildSurfaceContext::DiagnosticsSurface,
        "build-config.sha256.contributor-pr-0007",
        "receipt.pending-materialization",
        "provenance.ledger.contributor-pr-0007",
        "verification.pr-scoped-only",
        "artifacts.none-release-bearing",
        "hermetic.best-effort",
        "rematerialize.not-applicable",
    );
    base.is_trust_risk_source = true;
    base.input_trust_disclosed = true;
    manifest(base)
}

fn manifest_non_materialized_provenance_clean() -> M5ResolvedVerifiedInputManifestEntry {
    // A non-materialized reference input is trust-risk-bearing and discloses its input-trust marker.
    let mut base = clean_manifest_base(
        "manifest:provenance:non-materialized",
        "release.lane.emergency-hotfix",
        "verified.input.manifest.emergency_hotfix",
        M5BuildLaneTrustRole::PublicationAuthority,
        M5VerifiedInputSourceKind::NonMaterializedReference,
        M5ExactBuildSurfaceContext::ProvenanceSurface,
        "build-config.sha256.emergency-hotfix-0007",
        "receipt.reference-only",
        "provenance.ledger.emergency-hotfix-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages-rollback-metadata",
        "hermetic.verified-inputs",
        "rematerialize.rematerialize-required",
    );
    base.is_trust_risk_source = true;
    base.input_trust_disclosed = true;
    manifest(base)
}

fn manifest_pinned_digest_support_clean() -> M5ResolvedVerifiedInputManifestEntry {
    manifest(clean_manifest_base(
        "manifest:support:pinned-digest",
        "release.lane.protected-merge",
        "verified.input.manifest.protected_merge",
        M5BuildLaneTrustRole::HermeticInput,
        M5VerifiedInputSourceKind::PinnedDigestInput,
        M5ExactBuildSurfaceContext::SupportOrExportForm,
        "build-config.sha256.pinned-0007",
        "receipt.materialized.pinned-0007",
        "provenance.ledger.pinned-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages-sboms-schemas",
        "hermetic.pinned-digest-verified",
        "rematerialize.pinned-digest-rule",
    ))
}

fn manifest_verified_cache_support_clean() -> M5ResolvedVerifiedInputManifestEntry {
    manifest(clean_manifest_base(
        "manifest:support:verified-cache",
        "release.lane.release",
        "verified.input.manifest.release",
        M5BuildLaneTrustRole::SupportIdentity,
        M5VerifiedInputSourceKind::VerifiedCacheInput,
        M5ExactBuildSurfaceContext::SupportOrExportForm,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.support-packets-symbols",
        "hermetic.verified-inputs",
        "rematerialize.digest-required",
    ))
}

// -- Degraded verified-input-manifest entries ---------------------------------------------------

/// Degraded manifest entry: the resolved manifest object is incomplete — the materialized-input receipt is
/// unstated.
fn manifest_object_incomplete() -> M5ResolvedVerifiedInputManifestEntry {
    let mut base = clean_manifest_base(
        "manifest:build-farm:incomplete",
        "release.lane.release",
        "verified.input.manifest.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5VerifiedInputSourceKind::RematerializedFromSource,
        M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms-symbols-docs",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    );
    base.materialized_input_receipt = "   ".to_owned();
    manifest(base)
}

/// Degraded manifest entry: the lane's verification authority is unbounded — an unverified input claiming
/// protected-lane admission it must not have. The structured blocker reason for an admit-unverified-input
/// attempt.
fn manifest_admit_fold() -> M5ResolvedVerifiedInputManifestEntry {
    let mut base = clean_manifest_base(
        "manifest:release-center:admit-fold",
        "release.lane.contributor-pr",
        "verified.input.manifest.contributor_pr",
        M5BuildLaneTrustRole::PublicationAuthority,
        M5VerifiedInputSourceKind::VerifiedCacheInput,
        M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        "build-config.sha256.contributor-pr-0007",
        "receipt.materialized.contributor-pr-0007",
        "provenance.ledger.contributor-pr-0007",
        "verification.pr-scoped-only",
        "artifacts.binaries-packages",
        "hermetic.best-effort",
        "rematerialize.not-applicable",
    );
    base.input_admission_bounded = false;
    manifest(base)
}

/// Degraded manifest entry: the behavior is a hand-copied per-entry assumption instead of tracing to the
/// registry.
fn manifest_unbound() -> M5ResolvedVerifiedInputManifestEntry {
    let mut base = clean_manifest_base(
        "manifest:provenance:unbound",
        "release.lane.emergency-hotfix",
        "verified.input.manifest.emergency_hotfix",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5VerifiedInputSourceKind::VerifiedCacheInput,
        M5ExactBuildSurfaceContext::ProvenanceSurface,
        "build-config.sha256.emergency-hotfix-0007",
        "receipt.materialized.emergency-hotfix-0007",
        "provenance.ledger.emergency-hotfix-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages-rollback-metadata",
        "hermetic.verified-inputs",
        "rematerialize.rematerialize-required",
    );
    base.bound_to_registry = false;
    manifest(base)
}

/// Degraded manifest entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn manifest_form_incomplete() -> M5ResolvedVerifiedInputManifestEntry {
    let mut base = clean_manifest_base(
        "manifest:cache-service:form-incomplete",
        "release.lane.protected-merge",
        "verified.input.manifest.protected_merge",
        M5BuildLaneTrustRole::CredentialBoundary,
        M5VerifiedInputSourceKind::VerifiedCacheInput,
        M5ExactBuildSurfaceContext::ShiproomSurface,
        "build-config.sha256.protected-merge-0007",
        "receipt.materialized.protected-merge-0007",
        "provenance.ledger.protected-merge-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages-sboms",
        "hermetic.verified-inputs",
        "rematerialize.digest-required",
    );
    base.resolution_form_coverage = vec![M5ExactBuildResolutionForm::CanonicalObject];
    manifest(base)
}

/// Degraded manifest entry: the canonical registry token name is unstated.
fn manifest_token_unstated() -> M5ResolvedVerifiedInputManifestEntry {
    let mut base = clean_manifest_base(
        "manifest:diagnostics:token-unstated",
        "release.lane.contributor-pr",
        "  ",
        M5BuildLaneTrustRole::CachePosture,
        M5VerifiedInputSourceKind::PinnedDigestInput,
        M5ExactBuildSurfaceContext::DiagnosticsSurface,
        "build-config.sha256.pinned-0007",
        "receipt.materialized.pinned-0007",
        "provenance.ledger.pinned-0007",
        "verification.controlled-scoped-to-lane",
        "artifacts.binaries-packages",
        "hermetic.pinned-digest-verified",
        "rematerialize.pinned-digest-rule",
    );
    base.token_name = "  ".to_owned();
    manifest(base)
}

// -- Clean sidecar-completeness-manifest entries ------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_sidecar_base(
    entry_id: &str,
    manifest_ref: &str,
    token_name: &str,
    semantic_role: M5BuildLaneTrustRole,
    convergence_scope: M5SidecarConvergenceScope,
    surface_context: M5ExactBuildSurfaceContext,
    resolved_build_identity: &str,
    claimed_artifact_families: &str,
    sidecar_family_ledger: &str,
    binding_identity_check: &str,
    missing_or_mismatched_reference: &str,
    attestation_state: &str,
    last_convergence_revision: &str,
) -> M5SidecarCompletenessManifestEntryResolutionInput {
    M5SidecarCompletenessManifestEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        manifest_ref: manifest_ref.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        convergence_scope,
        surface_context,
        resolution_form_coverage: all_forms(),
        resolved_build_identity: resolved_build_identity.to_owned(),
        claimed_artifact_families: claimed_artifact_families.to_owned(),
        sidecar_family_ledger: sidecar_family_ledger.to_owned(),
        binding_identity_check: binding_identity_check.to_owned(),
        missing_or_mismatched_reference: missing_or_mismatched_reference.to_owned(),
        attestation_state: attestation_state.to_owned(),
        last_convergence_revision: last_convergence_revision.to_owned(),
        keeps_family_ledger_visible: true,
        manifest_is_truthful: true,
        missing_family_present: false,
        missing_family_flagged: false,
        mismatched_identity_present: false,
        mismatched_identity_flagged: false,
        proof_fresh: true,
    }
}

fn sidecar_binary_identity_clean() -> M5ResolvedSidecarCompletenessManifestEntry {
    sidecar(clean_sidecar_base(
        "sidecar:release-center:binary-identity",
        "release.lane.release",
        "sidecar.completeness.manifest.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5SidecarConvergenceScope::ConvergedOnBinaryIdentity,
        M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        "build-id.sha256.release-0007",
        "families.binaries-packages-docs-schemas-sboms-symbols-source-maps-rollback-metadata",
        "ledger.all-families-present-release-0007",
        "binding.pinned-to-build-id-release-0007",
        "missing-or-mismatched.none",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    ))
}

fn sidecar_receipt_reconciled_clean() -> M5ResolvedSidecarCompletenessManifestEntry {
    sidecar(clean_sidecar_base(
        "sidecar:shiproom:receipt-reconciled",
        "release.lane.protected-merge",
        "sidecar.completeness.manifest.protected_merge",
        M5BuildLaneTrustRole::ArtifactConvergence,
        M5SidecarConvergenceScope::ReconciledAgainstReceipt,
        M5ExactBuildSurfaceContext::ShiproomSurface,
        "build-id.sha256.protected-merge-0007",
        "families.binaries-packages-docs-schemas-sboms",
        "ledger.all-families-present-protected-merge-0007",
        "binding.reconciled-against-receipt-0007",
        "missing-or-mismatched.none",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    ))
}

fn sidecar_hermetic_rebuild_clean() -> M5ResolvedSidecarCompletenessManifestEntry {
    sidecar(clean_sidecar_base(
        "sidecar:diagnostics:hermetic-rebuild",
        "release.lane.emergency-hotfix",
        "sidecar.completeness.manifest.emergency_hotfix",
        M5BuildLaneTrustRole::SupportIdentity,
        M5SidecarConvergenceScope::HermeticRebuildConverged,
        M5ExactBuildSurfaceContext::DiagnosticsSurface,
        "build-id.sha256.emergency-hotfix-0007",
        "families.binaries-packages-rollback-metadata-symbols",
        "ledger.all-families-present-emergency-hotfix-0007",
        "binding.hermetic-rebuild-converged-0007",
        "missing-or-mismatched.none",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    ))
}

// -- Degraded sidecar-completeness-manifest entries ---------------------------------------------

/// Degraded sidecar entry: the manifest lets a green build omit a claimed sidecar family — a missing family
/// that is not flagged as a blocker reads as a clean pass when the exact-build story is actually incomplete.
fn sidecar_missing_family() -> M5ResolvedSidecarCompletenessManifestEntry {
    let mut base = clean_sidecar_base(
        "sidecar:build-farm:missing-family",
        "release.lane.release",
        "sidecar.completeness.manifest.release",
        M5BuildLaneTrustRole::ReproducibilityProof,
        M5SidecarConvergenceScope::ConvergedOnBinaryIdentity,
        M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        "build-id.sha256.release-0007",
        "families.binaries-packages-docs-schemas-sboms-symbols-source-maps-rollback-metadata",
        "ledger.symbols-family-absent-release-0007",
        "binding.pinned-to-build-id-release-0007",
        "missing-or-mismatched.symbols-family-absent",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    );
    base.missing_family_present = true;
    base.missing_family_flagged = false;
    sidecar(base)
}

/// Degraded sidecar entry: the canonical / accessible / audit resolution-form coverage of the manifest is
/// incomplete.
fn sidecar_form_incomplete() -> M5ResolvedSidecarCompletenessManifestEntry {
    let mut base = clean_sidecar_base(
        "sidecar:cache-service:form-incomplete",
        "release.lane.protected-merge",
        "sidecar.completeness.manifest.protected_merge",
        M5BuildLaneTrustRole::ArtifactConvergence,
        M5SidecarConvergenceScope::ReconciledAgainstReceipt,
        M5ExactBuildSurfaceContext::ShiproomSurface,
        "build-id.sha256.protected-merge-0007",
        "families.binaries-packages-docs-schemas-sboms",
        "ledger.all-families-present-protected-merge-0007",
        "binding.reconciled-against-receipt-0007",
        "missing-or-mismatched.none",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    );
    base.resolution_form_coverage = vec![M5ExactBuildResolutionForm::CanonicalObject];
    sidecar(base)
}

/// Degraded sidecar entry: the convergence scope is unclassified.
fn sidecar_scope_unclassified() -> M5ResolvedSidecarCompletenessManifestEntry {
    sidecar(clean_sidecar_base(
        "sidecar:provenance:scope-unclassified",
        "release.lane.emergency-hotfix",
        "sidecar.completeness.manifest.emergency_hotfix",
        M5BuildLaneTrustRole::SupportIdentity,
        M5SidecarConvergenceScope::ScopeUnclassified,
        M5ExactBuildSurfaceContext::ProvenanceSurface,
        "build-id.sha256.emergency-hotfix-0007",
        "families.binaries-packages-rollback-metadata-symbols",
        "ledger.all-families-present-emergency-hotfix-0007",
        "binding.hermetic-rebuild-converged-0007",
        "missing-or-mismatched.none",
        "attestation.signed-and-verified",
        "convergence.revision.0007",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5VerifiedInputSidecarCompletenessRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildLaneDowngradeTrigger>,
    verified_input_manifest_entries: Vec<M5ResolvedVerifiedInputManifestEntry>,
    sidecar_completeness_manifest_entries: Vec<M5ResolvedSidecarCompletenessManifestEntry>,
) -> M5VerifiedInputSidecarCompletenessRegistriesRow {
    M5VerifiedInputSidecarCompletenessRegistriesRow {
        consumer_surface,
        qualification: M5BuildLaneQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5BuildLaneDeploymentLine::ALL.to_vec(),
        required_labels: M5BuildLaneRequiredLabel::ALL.to_vec(),
        accessibility_routes: M5BuildLaneAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ExactBuildAnatomyPart::ALL.to_vec(),
        export_fields: M5ExactBuildExportField::ALL.to_vec(),
        downgrade_triggers,
        verified_input_manifest_entries,
        sidecar_completeness_manifest_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_SCHEMA_REF,
            M5_VERIFIED_INPUT_MANIFEST_DOMAIN_SCHEMA_REF,
            M5_SIDECAR_COMPLETENESS_MANIFEST_DOMAIN_SCHEMA_REF,
        ]),
        lets_a_green_build_omit_a_claimed_artifact_family_or_sidecar: false,
        binds_a_claimed_sidecar_to_a_different_build_identity: false,
        treats_a_missing_or_mismatched_sidecar_as_warning_only: false,
        admits_an_unverified_or_non_materialized_input_into_a_protected_lane: false,
    }
}

fn registry_rows() -> Vec<M5VerifiedInputSidecarCompletenessRegistriesRow> {
    use M5BuildLaneConsumerSurface as C;
    use M5BuildLaneDowngradeTrigger as D;

    vec![
        base_row(
            C::BuildFarm,
            "Build-farm owner",
            "The build farm resolves the release lane's verified-input manifest to one typed object — input source, build-config digest, materialized-input receipt, input provenance ledger, verification authority, expected artifact families, hermetic-input posture, and re-materialization rule — from the shared registry and proves the binary-identity sidecar-completeness manifest for the winning build identity; a manifest object missing its materialized-input receipt and a sidecar manifest that lets a green build omit a claimed family degrade honestly instead of reading as a clean pass",
            "evidence:m5-exact-build-supportability-build-farm:001",
            vec![
                D::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts,
                D::DriftedASidecarFromTheBinaryBuildIdentity,
                D::ProofStale,
            ],
            vec![
                manifest_rematerialized_release_center_clean(),
                manifest_object_incomplete(),
            ],
            vec![sidecar_binary_identity_clean(), sidecar_missing_family()],
        ),
        base_row(
            C::CacheService,
            "Cache-service owner",
            "The cache service resolves the protected-merge manifest and the receipt-reconciled sidecar-completeness manifest while keeping the sidecar-family ledger visible; a resolution-form gap on a manifest entry and on a sidecar manifest is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-exact-build-supportability-cache-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::CleanRoomProofRuleUnstated,
                D::ProofStale,
            ],
            vec![
                manifest_verified_cache_shiproom_clean(),
                manifest_form_incomplete(),
            ],
            vec![sidecar_receipt_reconciled_clean(), sidecar_form_incomplete()],
        ),
        base_row(
            C::ReleaseCenter,
            "Release-center owner",
            "The release center resolves the contributor / PR manifest while disclosing its unverified-external input-trust marker and reports the hermetic-rebuild sidecar-completeness manifest; an unverified input claiming protected-lane admission it must not have is caught as an admit-unverified-input blocker before it can enter a protected lane",
            "evidence:m5-exact-build-supportability-release-center:001",
            vec![
                D::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts,
                D::CachePostureUnstated,
                D::ProofStale,
            ],
            vec![
                manifest_unverified_external_diagnostics_clean(),
                manifest_admit_fold(),
            ],
            vec![sidecar_hermetic_rebuild_clean()],
        ),
        base_row(
            C::ProvenanceService,
            "Provenance-service owner",
            "The provenance service resolves the emergency-hotfix manifest while disclosing its non-materialized input-trust marker and bound to the registry; a manifest that is a hand-copied per-entry assumption and a sidecar manifest on an unclassified convergence scope degrade honestly",
            "evidence:m5-exact-build-supportability-provenance-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::BuildIdentityUnstated,
                D::ProofStale,
            ],
            vec![
                manifest_non_materialized_provenance_clean(),
                manifest_unbound(),
            ],
            vec![sidecar_scope_unclassified()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics renders the same resolved verified-input-manifest and sidecar-completeness-manifest truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied lane table; an unstated registry token is caught before it can drift",
            "evidence:m5-exact-build-supportability-diagnostics:001",
            vec![
                D::CachePostureUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                manifest_pinned_digest_support_clean(),
                manifest_token_unstated(),
            ],
            vec![sidecar_binary_identity_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved verified-input-manifest and sidecar-completeness-manifest truth, so a hand-copied constant, an unstated registry token, an admit-unverified attempt, or a missing sidecar family is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-exact-build-supportability-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::BuildIdentityUnstated,
                D::ProofStale,
            ],
            vec![manifest_verified_cache_support_clean()],
            vec![sidecar_receipt_reconciled_clean()],
        ),
    ]
}

fn governance_review() -> M5VerifiedInputSidecarCompletenessRegistriesGovernanceReview {
    M5VerifiedInputSidecarCompletenessRegistriesGovernanceReview {
        verified_input_registry_names_token_role_and_source: true,
        lane_resolves_to_typed_manifest_from_shared_registry: true,
        build_config_digest_receipt_and_artifact_families_published: true,
        unverified_inputs_cannot_enter_protected_lanes: true,
        sidecar_manifest_keeps_family_ledger_visible_and_flags_missing_or_mismatched: true,
        input_trust_disclosed_for_trust_risk_sources: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        release_center_shiproom_diagnostics_and_provenance_read_single_source: true,
        manifest_or_sidecar_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5VerifiedInputSidecarCompletenessRegistriesConsumerProjection {
    M5VerifiedInputSidecarCompletenessRegistriesConsumerProjection {
        release_center_and_shiproom_consume_shared_registries: true,
        diagnostics_and_provenance_consume_shared_registries: true,
        build_farm_and_cache_service_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5VerifiedInputSidecarCompletenessRegistriesProofFreshness {
    M5VerifiedInputSidecarCompletenessRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5VerifiedInputSidecarCompletenessRegistriesReleasePosture {
    M5VerifiedInputSidecarCompletenessRegistriesReleasePosture {
        proof_packet_ref: M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_ARTIFACT_REF.to_owned(),
        build_lane_audit_ref: M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_SCHEMA_REF,
        M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_DOC_REF,
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_VERIFIED_INPUT_MANIFEST_DOMAIN_SCHEMA_REF,
        M5_SIDECAR_COMPLETENESS_MANIFEST_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 verified-input-manifest and sidecar-completeness-manifest registries packet.
pub fn seeded_m5_verified_input_manifest_and_sidecar_completeness_registries(
) -> M5VerifiedInputSidecarCompletenessRegistriesPacket {
    M5VerifiedInputSidecarCompletenessRegistriesPacket::new(
        M5VerifiedInputSidecarCompletenessRegistriesPacketInput {
            packet_id: M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 verified-input-manifest and sidecar-completeness-manifest registries with one typed verified-input-manifest object resolving per lane, unverified inputs never entering protected lanes, the input-trust marker disclosed before any trust-risk input is admitted, canonical / accessible / audit resolution-form coverage, and the complete build-identity / claimed-families / sidecar-ledger / binding-identity / missing-or-mismatched / attestation / last-convergence-revision sidecar-completeness object across build-farm, cache-service, release-center, provenance, diagnostics, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set: M5VerifiedInputSidecarCompletenessRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the build-farm row is held at Beta pending verified-input-manifest parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed(
) -> M5VerifiedInputSidecarCompletenessRegistriesPacket {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.packet_id =
        "m5-verified-input-manifest-and-sidecar-completeness-registries:verified-input-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildLaneConsumerSurface::BuildFarm)
        .expect("build-farm row present");
    row.qualification = M5BuildLaneQualificationClass::Beta;
    packet
}

/// Narrowed variant: the release-center row is narrowed to Preview pending sidecar-completeness parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed(
) -> M5VerifiedInputSidecarCompletenessRegistriesPacket {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.packet_id =
        "m5-verified-input-manifest-and-sidecar-completeness-registries:sidecar-completeness-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildLaneConsumerSurface::ReleaseCenter)
        .expect("release-center row present");
    row.qualification = M5BuildLaneQualificationClass::Preview;
    packet
}
