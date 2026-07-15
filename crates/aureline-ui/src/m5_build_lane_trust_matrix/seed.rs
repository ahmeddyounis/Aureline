//! Canonical seed builders for the frozen M5 build-lane-trust matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical build-lane-trust matrix.
pub const M5_BUILD_LANE_TRUST_MATRIX_PACKET_ID: &str = "m5-build-lane-trust:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every lane must be able to show.
fn mandatory_labels() -> Vec<M5BuildLaneRequiredLabel> {
    M5BuildLaneRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a lane carries.
fn labels_with(extra: &[M5BuildLaneRequiredLabel]) -> Vec<M5BuildLaneRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every lane filled in and every lane-specific vocabulary left empty
/// for the caller to populate.
fn base_row(
    build_lane_family: M5BuildLaneFamily,
    qualification: M5BuildLaneQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5BuildLaneRow {
    M5BuildLaneRow {
        build_lane_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5BuildLaneSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5BuildLaneDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        contributor_pr_roles: vec![],
        protected_merge_roles: vec![],
        release_roles: vec![],
        emergency_hotfix_roles: vec![],
        degraded_reasons: M5BuildLaneDegradedReason::ALL.to_vec(),
        accessibility_routes: M5BuildLaneAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5BuildLaneConsumerSurface::SupportExport,
            M5BuildLaneConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5BuildLaneDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        pr_caches_publish_release_artifacts: false,
        treats_remote_cache_hits_as_reproducibility_proof: false,
        lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity: false,
        overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt: false,
        hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows:
            false,
    }
}

fn build_lane_rows() -> Vec<M5BuildLaneRow> {
    use M5BuildLaneConsumerSurface as C;
    use M5BuildLaneDowngradeTrigger as D;
    use M5BuildLaneFamily as F;
    use M5BuildLaneQualificationClass as Q;
    use M5BuildLaneRequiredLabel as L;
    use M5BuildLaneTrustRole as R;

    let mut rows = Vec::new();

    // 1. Contributor / PR lane.
    let mut row = base_row(
        F::ContributorPr,
        Q::Stable,
        "Contributor-lane owner",
        "One contributor / PR lane naming the shared cache readable without publication authority, the withheld release-artifact publication, the untrusted-cache posture, and the PR-scoped credentials so a PR lane may read shared caches but never publishes a release artifact from a PR cache",
        "evidence:m5-contributor-pr-lane-parity:001",
        &[
            M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
            M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_ARTIFACT_PUBLICATION_LANDED_SCHEMA_REF,
        ],
    );
    row.contributor_pr_roles = M5ContributorPrRole::ALL.to_vec();
    row.semantic_roles = vec![R::CachePosture, R::PublicationAuthority];
    row.required_labels = labels_with(&[L::CachePosture]);
    row.consumer_surfaces = vec![
        C::BuildFarm,
        C::CacheService,
        C::ReleaseCenter,
        C::Diagnostics,
        C::SupportExport,
        C::DocsHelp,
    ];
    row.downgrade_triggers = vec![
        D::PublishedReleaseArtifactsFromAPrCache,
        D::CachePostureUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Protected-merge lane.
    let mut row = base_row(
        F::ProtectedMerge,
        Q::Stable,
        "Protected-merge owner",
        "One protected-merge lane naming the controlled credentials scoped to the lane, the verified cache inputs only, the cache posture verified before promotion, and the missing digest that blocks promotion so a protected-merge lane uses controlled credentials and verified caches and never promotes from an untrusted cache",
        "evidence:m5-protected-merge-lane-parity:001",
        &[
            M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
            M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_ARTIFACT_PUBLICATION_LANDED_SCHEMA_REF,
        ],
    );
    row.protected_merge_roles = M5ProtectedMergeRole::ALL.to_vec();
    row.semantic_roles = vec![R::PublicationAuthority, R::CredentialBoundary];
    row.required_labels = labels_with(&[L::PublicationAuthority]);
    row.consumer_surfaces = vec![
        C::BuildFarm,
        C::CacheService,
        C::ReleaseCenter,
        C::ProvenanceService,
        C::Diagnostics,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::UsedAnUntrustedCache,
        D::PublicationAuthorityUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Release lane.
    let mut row = base_row(
        F::Release,
        Q::Stable,
        "Release-engineering owner",
        "One release lane naming the verified or re-materialized inputs only, the artifacts converging on one exact build identity, the fresh clean-room rebuild proof, and the sidecars pinned to the binary build identity so a release lane converges binaries, packages, SBOMs, symbols, and docs on one exact build identity and never treats a remote-cache hit as reproducibility proof",
        "evidence:m5-release-lane-parity:001",
        &[
            M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
            M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
            M5_REPRODUCIBLE_RC_LANDED_SCHEMA_REF,
        ],
    );
    row.release_roles = M5ReleaseRole::ALL.to_vec();
    row.semantic_roles = vec![R::ReproducibilityProof, R::ArtifactConvergence];
    row.required_labels = labels_with(&[L::BuildIdentity]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::ProvenanceService,
        C::Shiproom,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::TreatedARemoteCacheHitAsReproducibilityProof,
        D::DriftedASidecarFromTheBinaryBuildIdentity,
        D::OverclaimedCleanRoomParityOnPartialRebuild,
        D::BuildIdentityUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Emergency-hotfix lane.
    let mut row = base_row(
        F::EmergencyHotfix,
        Q::Stable,
        "Emergency-hotfix owner",
        "One emergency-hotfix lane naming the re-materialized inputs under controlled credentials, the exact build identity preserved under expedite, the rollback metadata and support packet converged, and the hermetic inputs verified despite urgency so an emergency-hotfix lane still converges on one exact build identity and never waives non-hermetic inputs for speed",
        "evidence:m5-emergency-hotfix-lane-parity:001",
        &[
            M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
            M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
            M5_REPRODUCIBLE_RC_LANDED_SCHEMA_REF,
        ],
    );
    row.emergency_hotfix_roles = M5EmergencyHotfixRole::ALL.to_vec();
    row.semantic_roles = vec![R::SupportIdentity, R::HermeticInput];
    row.required_labels = labels_with(&[L::BuildIdentity]);
    row.consumer_surfaces = vec![
        C::ReleaseCenter,
        C::ProvenanceService,
        C::Shiproom,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::HidNonHermeticInputsCachePoisoningOrUnreplayableArtifacts,
        D::CleanRoomProofRuleUnstated,
        D::BuildIdentityUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5BuildLaneTrustGovernanceReview {
    M5BuildLaneTrustGovernanceReview {
        contributor_lanes_read_caches_but_never_publish_release_artifacts: true,
        protected_merge_lanes_use_controlled_credentials_and_verified_caches: true,
        release_and_hotfix_lanes_use_verified_or_rematerialized_inputs: true,
        release_artifacts_converge_on_one_exact_build_identity: true,
        remote_cache_hits_are_never_treated_as_reproducibility_proof: true,
        docs_schema_sbom_and_symbol_sidecars_stay_pinned_to_binary_build_identity: true,
        clean_room_parity_is_never_overclaimed_on_partial_rebuilds: true,
        non_hermetic_inputs_cache_poisoning_and_unreplayable_artifacts_block_promotion: true,
        missing_digests_block_protected_promotion: true,
        every_lane_declares_deployment_contexts: true,
        every_lane_declares_accessibility_route: true,
        support_export_reads_single_build_lane_source: true,
        release_center_shiproom_and_diagnostics_bind_to_single_build_lane_source: true,
        later_rows_cannot_invent_parallel_build_lane_vocabulary: true,
        build_lane_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_registry_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5BuildLaneTrustConsumerProjection {
    M5BuildLaneTrustConsumerProjection {
        release_center_and_shiproom_consume_shared_build_lane_truth: true,
        diagnostics_and_admin_consume_shared_cache_and_credential_boundaries: true,
        build_farm_and_cache_service_consume_shared_cache_posture_and_publication_authority: true,
        docs_help_and_screenshots_read_single_build_lane_source: true,
        reproducibility_and_clean_room_proofs_bind_to_shared_exact_build_identity: true,
        support_export_reads_single_build_lane_source: true,
    }
}

fn proof_freshness() -> M5BuildLaneTrustProofFreshness {
    M5BuildLaneTrustProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5BuildLaneTrustReleasePosture {
    M5BuildLaneTrustReleasePosture {
        proof_packet_ref: M5_BUILD_LANE_TRUST_ARTIFACT_REF.to_owned(),
        build_lane_audit_ref: M5_BUILD_LANE_TRUST_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_BUILD_LANE_TRUST_MATRIX_SCHEMA_REF,
        M5_BUILD_LANE_TRUST_MATRIX_DOC_REF,
        M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF,
        M5_ARTIFACT_PUBLICATION_LANDED_SCHEMA_REF,
        M5_REPRODUCIBLE_RC_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 build-lane-trust matrix packet.
pub fn seeded_m5_build_lane_trust_matrix() -> M5BuildLaneTrustMatrixPacket {
    M5BuildLaneTrustMatrixPacket::new(M5BuildLaneTrustMatrixPacketInput {
        packet_id: M5_BUILD_LANE_TRUST_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 build-farm, cache-trust, clean-room-rebuild, and exact-build-supportability matrix"
                .to_owned(),
        build_lane_rows: build_lane_rows(),
        vocabulary_set: M5BuildLaneTrustVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the release lane is held at Beta because clean-room parity is not yet proven across
/// every artifact class; every lane stays visible.
pub fn seeded_m5_build_lane_trust_matrix_release_beta_narrowed() -> M5BuildLaneTrustMatrixPacket {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.packet_id = "m5-build-lane-trust:release-beta:0001".to_owned();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::Release)
        .expect("release row present");
    row.qualification = M5BuildLaneQualificationClass::Beta;
    packet
}

/// Narrowed variant: the emergency-hotfix lane is narrowed to Preview pending complete exact-build
/// supportability evidence across every build context; every lane stays visible.
pub fn seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed(
) -> M5BuildLaneTrustMatrixPacket {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.packet_id = "m5-build-lane-trust:emergency-hotfix-preview:0001".to_owned();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::EmergencyHotfix)
        .expect("emergency-hotfix row present");
    row.qualification = M5BuildLaneQualificationClass::Preview;
    packet
}
