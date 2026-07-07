//! Canonical seed builders for the frozen M5 release-candidate-card,
//! version-bump-row, publish-target-row, artifact-provenance-bundle-card, and
//! promotion-timeline component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call them
//! so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical release-center-component matrix.
pub const M5_RELEASE_CENTER_MATRIX_PACKET_ID: &str = "m5-release-center-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5ReleaseCenterRequiredLabel> {
    M5ReleaseCenterRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5ReleaseCenterRequiredLabel]) -> Vec<M5ReleaseCenterRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5ReleaseCenterComponentFamily,
    qualification: M5ReleaseCenterQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
) -> M5ReleaseCenterComponentRow {
    M5ReleaseCenterComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5PublicationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5DeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        candidate_scope_classes: vec![],
        candidate_blocker_states: vec![],
        version_bump_classes: vec![],
        compatibility_impacts: vec![],
        target_visibilities: vec![],
        target_mutabilities: vec![],
        target_auth_sources: vec![],
        dry_run_availabilities: vec![],
        signature_statuses: vec![],
        attestation_statuses: vec![],
        sbom_statuses: vec![],
        digest_lineage_states: vec![],
        rollout_rings: vec![],
        promotion_stage_states: vec![],
        rollback_blast_radii: vec![],
        revocation_scopes: vec![],
        accessibility_routes: M5ReleaseCenterAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ReleaseCenterConsumerSurface::ReleaseCenterUi,
            M5ReleaseCenterConsumerSurface::SupportExport,
            M5ReleaseCenterConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ReleaseCenterDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RELEASE_CENTER_SCHEMA_REF,
            M5_RELEASE_CENTER_OBJECT_MODEL_REF,
        ]),
        masks_target_auth_source_or_mutability: false,
        conflates_signed_and_unsigned_provenance: false,
        invents_private_release_status_grammar: false,
        overstates_rollback_reversibility_or_drops_evidence_freshness: false,
    }
}

fn component_rows() -> Vec<M5ReleaseCenterComponentRow> {
    use M5AttestationStatus as AT;
    use M5CandidateBlockerState as BL;
    use M5CandidateScopeClass as SC;
    use M5CompatibilityImpact as CI;
    use M5DigestLineageState as DL;
    use M5DryRunAvailability as DR;
    use M5PromotionStageState as PS;
    use M5PublishTargetVisibility as TV;
    use M5ReleaseCenterComponentFamily as F;
    use M5ReleaseCenterConsumerSurface as C;
    use M5ReleaseCenterDowngradeTrigger as D;
    use M5ReleaseCenterQualificationClass as Q;
    use M5ReleaseCenterRequiredLabel as L;
    use M5RevocationScope as RS;
    use M5RollbackBlastRadius as RB;
    use M5RolloutRing as RR;
    use M5SbomStatus as SB;
    use M5SignatureStatus as SG;
    use M5TargetAuthSource as TA;
    use M5TargetMutability as TM;
    use M5VersionBumpClass as VB;

    let mut rows = Vec::new();

    // 1. Release candidate card.
    let mut row = base_row(
        F::ReleaseCandidateCard,
        Q::Stable,
        "Release-candidate component owner",
        "One release-candidate-card model carrying candidate scope — single family, multi family, full train, hotfix, backport line, or preview channel — and the current blocker state with its freshness, so a candidate is never shown as clear while a hard blocker or a stale evaluation is open",
        "evidence:m5-release-candidate-card-parity:001",
    );
    row.candidate_scope_classes = SC::ALL.to_vec();
    row.candidate_blocker_states = BL::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::ServiceHealth,
        C::AdminConsole,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CandidateScopeUnstated,
        D::BlockerFreshnessHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Version-bump row.
    let mut row = base_row(
        F::VersionBumpRow,
        Q::Stable,
        "Version-bump component owner",
        "One version-bump-row model naming the proposed bump class — major, minor, patch, prerelease, build-metadata-only, or republish — and its compatibility impact, so a breaking change is never hidden behind a version number",
        "evidence:m5-version-bump-row-parity:001",
    );
    row.version_bump_classes = VB::ALL.to_vec();
    row.compatibility_impacts = CI::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::DocsPortal,
        C::EvaluationPack,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::VersionBumpImpactUnstated, D::ProofStale];
    rows.push(row);

    // 3. Publish target row / review sheet.
    let mut row = base_row(
        F::PublishTargetRow,
        Q::Stable,
        "Publish-target component owner",
        "One publish-target-row model naming the target's visibility, its mutability, the identity authorized to publish to it, and whether a dry-run preview is available, so a mutable target, an unauthenticated mirror, or a no-dry-run target is never presented as a clean, safe publish",
        "evidence:m5-publish-target-row-parity:001",
    );
    row.target_visibilities = TV::ALL.to_vec();
    row.target_mutabilities = TM::ALL.to_vec();
    row.target_auth_sources = TA::ALL.to_vec();
    row.dry_run_availabilities = DR::ALL.to_vec();
    row.required_labels = labels_with(&[L::AuthSource, L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::AdminConsole,
        C::MirrorConsole,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TargetAuthSourceMasked,
        D::TargetMutabilityHidden,
        D::DryRunAvailabilityUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Artifact provenance bundle card.
    let mut row = base_row(
        F::ArtifactProvenanceBundleCard,
        Q::Stable,
        "Provenance/attestation component owner",
        "One artifact-provenance-bundle-card model carrying signature, attestation, and SBOM status over an immutable digest lineage, so an unsigned, unattested, partial-SBOM, or broken-lineage bundle is never shown as verified",
        "evidence:m5-artifact-provenance-bundle-card-parity:001",
    );
    row.signature_statuses = SG::ALL.to_vec();
    row.attestation_statuses = AT::ALL.to_vec();
    row.sbom_statuses = SB::ALL.to_vec();
    row.digest_lineage_states = DL::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::HelpAbout,
        C::EvaluationPack,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SignatureOrAttestationOverclaimed,
        D::SbomCompletenessOverstated,
        D::DigestLineageBrokenHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Promotion timeline step.
    let mut row = base_row(
        F::PromotionTimelineStep,
        Q::Stable,
        "Promotion-timeline component owner",
        "One promotion-timeline-step model naming its rollout ring — canary, pilot, early access, broad, general availability, or held — and its stage state, so a blocked or rolled-back stage is never shown as promoted and the current ring is always explicit",
        "evidence:m5-promotion-timeline-step-parity:001",
    );
    row.rollout_rings = RR::ALL.to_vec();
    row.promotion_stage_states = PS::ALL.to_vec();
    row.required_labels = labels_with(&[L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::ServiceHealth,
        C::AdminConsole,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::RolloutRingUnstated, D::ProofStale];
    rows.push(row);

    // 6. Rollback / revocation row.
    let mut row = base_row(
        F::RollbackRevocationRow,
        Q::Stable,
        "Rollback/revocation component owner",
        "One rollback-revocation-row model naming a rollback's blast radius — single artifact through fleet-wide — and its revocation scope, so a fleet-wide rollback or a key/trust-root rotation is never understated as a soft, single-artifact undo",
        "evidence:m5-rollback-revocation-row-parity:001",
    );
    row.rollback_blast_radii = RB::ALL.to_vec();
    row.revocation_scopes = RS::ALL.to_vec();
    row.required_labels = labels_with(&[L::RollbackVocabulary, L::EvidenceFreshness]);
    row.consumer_surfaces = vec![
        C::ReleaseCenterUi,
        C::AdminConsole,
        C::MirrorConsole,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::RollbackBlastRadiusUnderstated, D::ProofStale];
    rows.push(row);

    rows
}

fn governance_review() -> M5ReleaseCenterGovernanceReview {
    M5ReleaseCenterGovernanceReview {
        candidate_card_shows_scope_and_blocker_freshness: true,
        version_bump_row_shows_compatibility_impact: true,
        publish_target_row_shows_auth_source_and_mutability: true,
        provenance_card_shows_signature_attestation_sbom: true,
        promotion_timeline_shows_ring_and_stage: true,
        rollback_row_shows_blast_radius_and_revocation_scope: true,
        signed_versus_unsigned_never_conflated: true,
        no_component_invents_second_status_grammar: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ReleaseCenterConsumerProjection {
    M5ReleaseCenterConsumerProjection {
        candidate_and_version_surfaces_consume_matrix: true,
        publish_target_surfaces_consume_auth_vocabulary: true,
        provenance_surfaces_consume_signature_vocabulary: true,
        promotion_and_rollback_surfaces_consume_ring_and_blast_vocabulary: true,
        support_export_reads_single_source: true,
        evaluation_and_mirror_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5ReleaseCenterProofFreshness {
    M5ReleaseCenterProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ReleaseCenterReleasePosture {
    M5ReleaseCenterReleasePosture {
        release_packet_ref:
            "artifacts/release/m5-release-center-component-proof/support_export.json".to_owned(),
        release_center_audit_ref: "artifacts/components/m5-release-center-components.md".to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RELEASE_CENTER_SCHEMA_REF,
        M5_RELEASE_CENTER_DOC_REF,
        M5_RELEASE_CENTER_OBJECT_MODEL_REF,
        M5_RELEASE_CENTER_ROLLBACK_CONTRACT_REF,
        M5_RELEASE_CENTER_PROVENANCE_CONTRACT_REF,
    ])
}

/// Builds the canonical frozen M5 release-center-component matrix packet.
pub fn seeded_m5_release_center_component_matrix() -> M5ReleaseCenterMatrixPacket {
    M5ReleaseCenterMatrixPacket::new(M5ReleaseCenterMatrixPacketInput {
        packet_id: M5_RELEASE_CENTER_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 release-candidate-card, version-bump-row, publish-target-row, artifact-provenance-bundle-card, and promotion-timeline component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5ReleaseCenterVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the promotion timeline step is held at Beta because a slice of
/// rollout-ring transitions do not yet round-trip across every export path; every
/// component stays visible.
pub fn seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed(
) -> M5ReleaseCenterMatrixPacket {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.packet_id = "m5-release-center-components:promotion-timeline-step-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::PromotionTimelineStep)
        .expect("promotion-timeline-step row present");
    row.qualification = M5ReleaseCenterQualificationClass::Beta;
    packet
}

/// Narrowed variant: the rollback/revocation row is narrowed to Preview pending
/// revocation-scope parity proof across every artifact graph; every component stays
/// visible.
pub fn seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed(
) -> M5ReleaseCenterMatrixPacket {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.packet_id =
        "m5-release-center-components:rollback-revocation-row-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::RollbackRevocationRow)
        .expect("rollback-revocation-row row present");
    row.qualification = M5ReleaseCenterQualificationClass::Preview;
    packet
}
