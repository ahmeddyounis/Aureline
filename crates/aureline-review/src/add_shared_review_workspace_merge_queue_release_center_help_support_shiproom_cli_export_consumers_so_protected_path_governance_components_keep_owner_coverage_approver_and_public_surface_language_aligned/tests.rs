use super::*;

const PACKET_ID: &str = "governance-component-consumer:stable:0001";

fn trust_review() -> GovernanceComponentConsumerTrustReview {
    GovernanceComponentConsumerTrustReview {
        component_reuse_proven_by_fixtures: true,
        same_change_same_language_across_surfaces: true,
        advisory_never_reads_as_provider_authoritative: true,
        guarded_merge_never_hides_missing_backup_coverage: true,
        guarded_merge_never_hides_expired_approver_state: true,
        public_surface_change_never_hides_diff_or_migration_evidence: true,
        owner_coverage_labels_identical_across_surfaces: true,
        approver_state_language_identical_across_surfaces: true,
        public_surface_impact_language_identical_across_surfaces: true,
        help_support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> GovernanceComponentConsumerProjection {
    GovernanceComponentConsumerProjection {
        review_workspace_reuses_shared_components: true,
        merge_queue_reuses_shared_components: true,
        release_center_reuses_shared_components: true,
        help_surface_reuses_shared_components: true,
        support_packet_reuses_shared_components: true,
        shiproom_reuses_shared_components: true,
        cli_export_reuses_shared_components: true,
        every_component_adopted_by_two_or_more_consumers: true,
        parity_facets_identical_for_same_change: true,
        narrowing_disclosed_not_hidden: true,
    }
}

fn proof_freshness() -> GovernanceComponentConsumerProofFreshness {
    GovernanceComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5GovernanceComponentDowngradeTrigger> {
    vec![
        M5GovernanceComponentDowngradeTrigger::ProofStale,
        M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
        M5GovernanceComponentDowngradeTrigger::ApproverStateExpired,
        M5GovernanceComponentDowngradeTrigger::PublicSurfaceDiffUnavailable,
        M5GovernanceComponentDowngradeTrigger::MigrationEvidenceMissing,
        M5GovernanceComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<GovernanceComponentConsumer> {
    GovernanceComponentConsumer::ALL.to_vec()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        GOVERNANCE_COMPONENT_CONSUMER_SCHEMA_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_DOC_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_PROTECTED_PATH_OWNERSHIP_CONTROLS_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_APPROVER_REVIEW_PACK_CONTROLS_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_PUBLIC_SURFACE_MERGE_CONTROL_CONTROLS_CONTRACT_REF.to_owned(),
        GOVERNANCE_COMPONENT_CONSUMER_DRI_REGISTRY_MERGE_READINESS_CONTROLS_CONTRACT_REF.to_owned(),
    ]
}

fn binding_refs(component: M5GovernanceComponent) -> Vec<String> {
    vec![
        GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        component_canonical_control_schema_ref(component).to_owned(),
    ]
}

/// The frozen governance-state vocabulary tokens each evidence state reuses.
fn vocab_for_evidence(evidence: GovernanceComponentEvidenceState) -> Vec<M5GovernanceStateVocab> {
    match evidence {
        GovernanceComponentEvidenceState::ProviderAuthoritativeFresh => vec![
            M5GovernanceStateVocab::Authoritative,
            M5GovernanceStateVocab::ProviderAuthoritative,
            M5GovernanceStateVocab::Covered,
        ],
        GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate => vec![
            M5GovernanceStateVocab::Advisory,
            M5GovernanceStateVocab::LocalEstimate,
        ],
        GovernanceComponentEvidenceState::OwnerBackupCoverageMissing => vec![
            M5GovernanceStateVocab::Covered,
            M5GovernanceStateVocab::BackupMissing,
        ],
        GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived => vec![
            M5GovernanceStateVocab::Waived,
            M5GovernanceStateVocab::Expired,
        ],
        GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing => vec![
            M5GovernanceStateVocab::Authoritative,
            M5GovernanceStateVocab::LocalEstimate,
        ],
        GovernanceComponentEvidenceState::ProofStaleRelativeToChange => vec![
            M5GovernanceStateVocab::Stale,
            M5GovernanceStateVocab::LocalEstimate,
        ],
    }
}

/// Builds one binding, deriving projection mode, parity state, narrow banner, and
/// disclosure notes from the change's evidence state so the fixture stays
/// self-consistent by construction.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    change_id: &str,
    change_label: &str,
    component: M5GovernanceComponent,
    consumer: GovernanceComponentConsumer,
    evidence: GovernanceComponentEvidenceState,
    facets: &GovernanceComponentParityFacetValues,
) -> GovernanceComponentConsumerBinding {
    let disclosure = resolve_governance_component_disclosure(evidence);

    let narrow_banner = disclosure.narrow_reason.map(|reason| {
        let (next_action, next_action_label) = match reason {
            GovernanceComponentNarrowReason::EnforcementAdvisoryOrLocalEstimate => (
                GovernanceComponentNarrowNextAction::ReviewEnforcementAuthority,
                "Enforcement is advisory / a local estimate here; review the provider authority"
                    .to_owned(),
            ),
            GovernanceComponentNarrowReason::OwnerBackupCoverageMissing => (
                GovernanceComponentNarrowNextAction::ReviewOwnerCoverage,
                "Owner backup coverage is missing; review the owner-coverage state".to_owned(),
            ),
            GovernanceComponentNarrowReason::ApproverStateExpiredOrWaived => (
                GovernanceComponentNarrowNextAction::ReviewApproverState,
                "A required approval is expired or waived; review the approver state".to_owned(),
            ),
            GovernanceComponentNarrowReason::PublicSurfaceDiffOrMigrationMissing => (
                GovernanceComponentNarrowNextAction::ReviewPublicSurfaceEvidence,
                "The machine-generated diff / migration evidence is missing; review it".to_owned(),
            ),
            GovernanceComponentNarrowReason::ProofStaleRelativeToChange => (
                GovernanceComponentNarrowNextAction::RefreshProof,
                "Provider-backed proof is stale relative to the change; refresh it".to_owned(),
            ),
        };
        GovernanceComponentNarrowBanner {
            reason,
            preserved_facets_note:
                "Owner coverage, approver state, public-surface impact, and merge blocker are preserved; only evidence narrowed"
                    .to_owned(),
            next_action,
            next_action_label,
        }
    });

    let enforcement_authority_note = if disclosure.needs_enforcement_authority_note {
        "Enforcement authority is shown explicitly as advisory / local estimate rather than provider-authoritative"
            .to_owned()
    } else {
        String::new()
    };
    let evidence_continuity_note = if disclosure.needs_evidence_continuity_note {
        "Owner-coverage, approver, and public-surface evidence stays explicit under this narrowing"
            .to_owned()
    } else {
        String::new()
    };

    GovernanceComponentConsumerBinding {
        binding_id: binding_id.to_owned(),
        governed_change_id: change_id.to_owned(),
        governed_change_label: change_label.to_owned(),
        component,
        consumer,
        evidence_state: evidence,
        projection_mode: disclosure.expected_mode,
        parity_facets: facets.clone(),
        parity_state: parity_state_for_mode(disclosure.expected_mode),
        narrow_banner,
        enforcement_authority_note,
        evidence_continuity_note,
        governance_state_vocab: vocab_for_evidence(evidence),
        advisory_owner_reads_as_provider_authoritative: false,
        guarded_merge_hides_missing_backup_coverage: false,
        guarded_merge_hides_expired_approver_state: false,
        public_surface_change_hides_diff_or_migration_evidence: false,
        rewords_governance_labels_per_surface: false,
        source_contract_refs: binding_refs(component),
    }
}

fn facets(
    owner: &str,
    approver: &str,
    public_surface: &str,
    merge_blocker: &str,
) -> GovernanceComponentParityFacetValues {
    GovernanceComponentParityFacetValues {
        owner_coverage_label: owner.to_owned(),
        approver_state_label: approver.to_owned(),
        public_surface_impact_label: public_surface.to_owned(),
        merge_blocker_label: merge_blocker.to_owned(),
    }
}

/// The canonical binding set: eight components, each adopted by >= 2 consumers,
/// covering all seven consumer surfaces and every evidence state. Changes sharing an
/// id share parity facets.
fn consumer_bindings() -> Vec<GovernanceComponentConsumerBinding> {
    // Change 1: protected-path row, provider-authoritative fresh.
    let pp = facets(
        "owned by @payments-core · CODEOWNERS authoritative",
        "2 of 2 required approvers satisfied",
        "no public-surface impact",
        "no merge blockers",
    );
    // Change 2: ownership card, owner backup coverage missing.
    let oc = facets(
        "owned by @data-platform · backup owner missing",
        "1 of 1 required approver satisfied",
        "no public-surface impact",
        "blocked: owner backup coverage missing",
    );
    // Change 3: approver matrix, approver state expired / waived.
    let am = facets(
        "owned by @release-eng · CODEOWNERS authoritative",
        "1 required approver expired · 1 waived",
        "no public-surface impact",
        "blocked: required approval expired",
    );
    // Change 4: review-pack summary, proof stale relative to change.
    let rp = facets(
        "owned by @qa-guild · advisory owner hint",
        "review pack parity pending re-evaluation",
        "no public-surface impact",
        "blocked: review pack stale relative to head",
    );
    // Change 5: public-surface diff card, public-surface evidence missing.
    let ps = facets(
        "owned by @sdk-team · CODEOWNERS authoritative",
        "2 of 2 required approvers satisfied",
        "breaking public command surface change · diff missing",
        "blocked: machine-generated diff / migration evidence missing",
    );
    // Change 6: merge-control banner, enforcement advisory / local estimate.
    let mc = facets(
        "owned by @infra · advisory owner hint",
        "approval state estimated locally",
        "no public-surface impact",
        "blocked (local estimate): branch protection not confirmed by provider",
    );
    // Change 7: DRI-registry row, provider-authoritative fresh.
    let dr = facets(
        "DRI @security-lead · backup @security-oncall",
        "2 of 2 required approvers satisfied",
        "no public-surface impact",
        "no merge blockers",
    );
    // Change 8: merge-readiness strip, enforcement advisory / local estimate.
    let mr = facets(
        "owned by @web-platform · advisory owner hint",
        "approval state estimated locally",
        "no public-surface impact",
        "blocked (local estimate): merge queue authority not confirmed",
    );

    vec![
        binding(
            "bind:pp-1:workspace",
            "chg:pp-1",
            "protected path src/payments/*",
            M5GovernanceComponent::ProtectedPathRow,
            GovernanceComponentConsumer::ReviewWorkspace,
            GovernanceComponentEvidenceState::ProviderAuthoritativeFresh,
            &pp,
        ),
        binding(
            "bind:pp-1:shiproom",
            "chg:pp-1",
            "protected path src/payments/*",
            M5GovernanceComponent::ProtectedPathRow,
            GovernanceComponentConsumer::Shiproom,
            GovernanceComponentEvidenceState::ProviderAuthoritativeFresh,
            &pp,
        ),
        binding(
            "bind:oc-2:workspace",
            "chg:oc-2",
            "ownership of data/pipelines/*",
            M5GovernanceComponent::OwnershipCard,
            GovernanceComponentConsumer::ReviewWorkspace,
            GovernanceComponentEvidenceState::OwnerBackupCoverageMissing,
            &oc,
        ),
        binding(
            "bind:oc-2:support",
            "chg:oc-2",
            "ownership of data/pipelines/*",
            M5GovernanceComponent::OwnershipCard,
            GovernanceComponentConsumer::SupportPacket,
            GovernanceComponentEvidenceState::OwnerBackupCoverageMissing,
            &oc,
        ),
        binding(
            "bind:am-3:queue",
            "chg:am-3",
            "approver matrix for release/*",
            M5GovernanceComponent::ApproverMatrix,
            GovernanceComponentConsumer::MergeQueue,
            GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived,
            &am,
        ),
        binding(
            "bind:am-3:help",
            "chg:am-3",
            "approver matrix for release/*",
            M5GovernanceComponent::ApproverMatrix,
            GovernanceComponentConsumer::HelpSurface,
            GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived,
            &am,
        ),
        binding(
            "bind:rp-4:workspace",
            "chg:rp-4",
            "review pack for feature/checkout",
            M5GovernanceComponent::ReviewPackSummary,
            GovernanceComponentConsumer::ReviewWorkspace,
            GovernanceComponentEvidenceState::ProofStaleRelativeToChange,
            &rp,
        ),
        binding(
            "bind:rp-4:cli",
            "chg:rp-4",
            "review pack for feature/checkout",
            M5GovernanceComponent::ReviewPackSummary,
            GovernanceComponentConsumer::CliExport,
            GovernanceComponentEvidenceState::ProofStaleRelativeToChange,
            &rp,
        ),
        binding(
            "bind:ps-5:release",
            "chg:ps-5",
            "public surface diff for sdk v3",
            M5GovernanceComponent::PublicSurfaceDiffCard,
            GovernanceComponentConsumer::ReleaseCenter,
            GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing,
            &ps,
        ),
        binding(
            "bind:ps-5:support",
            "chg:ps-5",
            "public surface diff for sdk v3",
            M5GovernanceComponent::PublicSurfaceDiffCard,
            GovernanceComponentConsumer::SupportPacket,
            GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing,
            &ps,
        ),
        binding(
            "bind:mc-6:queue",
            "chg:mc-6",
            "merge control for hotfix/logging",
            M5GovernanceComponent::MergeControlBanner,
            GovernanceComponentConsumer::MergeQueue,
            GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
            &mc,
        ),
        binding(
            "bind:mc-6:shiproom",
            "chg:mc-6",
            "merge control for hotfix/logging",
            M5GovernanceComponent::MergeControlBanner,
            GovernanceComponentConsumer::Shiproom,
            GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
            &mc,
        ),
        binding(
            "bind:dr-7:release",
            "chg:dr-7",
            "DRI registry for auth service",
            M5GovernanceComponent::DriRegistryRow,
            GovernanceComponentConsumer::ReleaseCenter,
            GovernanceComponentEvidenceState::ProviderAuthoritativeFresh,
            &dr,
        ),
        binding(
            "bind:dr-7:cli",
            "chg:dr-7",
            "DRI registry for auth service",
            M5GovernanceComponent::DriRegistryRow,
            GovernanceComponentConsumer::CliExport,
            GovernanceComponentEvidenceState::ProviderAuthoritativeFresh,
            &dr,
        ),
        binding(
            "bind:mr-8:queue",
            "chg:mr-8",
            "merge readiness for web/*",
            M5GovernanceComponent::MergeReadinessStrip,
            GovernanceComponentConsumer::MergeQueue,
            GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
            &mr,
        ),
        binding(
            "bind:mr-8:help",
            "chg:mr-8",
            "merge readiness for web/*",
            M5GovernanceComponent::MergeReadinessStrip,
            GovernanceComponentConsumer::HelpSurface,
            GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
            &mr,
        ),
    ]
}

fn packet_with(
    bindings: Vec<GovernanceComponentConsumerBinding>,
) -> GovernanceComponentConsumerPacket {
    GovernanceComponentConsumerPacket::new(GovernanceComponentConsumerPacketInput {
        packet_id: PACKET_ID.to_owned(),
        surface_label: "Shared protected-path governance-component consumers".to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn packet() -> GovernanceComponentConsumerPacket {
    packet_with(consumer_bindings())
}

#[test]
fn consumer_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn disclosure_maps_evidence_to_mode() {
    let fresh = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::ProviderAuthoritativeFresh,
    );
    assert_eq!(
        fresh.expected_mode,
        GovernanceComponentProjectionMode::FullParity
    );
    assert!(!fresh.needs_narrow_banner);
    assert!(!fresh.needs_enforcement_authority_note);
    assert!(!fresh.needs_evidence_continuity_note);

    let advisory = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
    );
    assert_eq!(
        advisory.expected_mode,
        GovernanceComponentProjectionMode::EnforcementNarrowed
    );
    assert!(advisory.needs_narrow_banner);
    assert!(advisory.needs_enforcement_authority_note);
    assert!(!advisory.needs_evidence_continuity_note);

    let coverage = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::OwnerBackupCoverageMissing,
    );
    assert_eq!(
        coverage.expected_mode,
        GovernanceComponentProjectionMode::CoverageNarrowed
    );
    assert!(coverage.needs_evidence_continuity_note);
    assert!(!coverage.needs_enforcement_authority_note);

    let approval = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::ApproverStateExpiredOrWaived,
    );
    assert_eq!(
        approval.expected_mode,
        GovernanceComponentProjectionMode::ApprovalNarrowed
    );
    assert!(approval.needs_evidence_continuity_note);

    let public_surface = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing,
    );
    assert_eq!(
        public_surface.expected_mode,
        GovernanceComponentProjectionMode::PublicSurfaceNarrowed
    );
    assert!(public_surface.needs_evidence_continuity_note);

    let stale = resolve_governance_component_disclosure(
        GovernanceComponentEvidenceState::ProofStaleRelativeToChange,
    );
    assert_eq!(
        stale.expected_mode,
        GovernanceComponentProjectionMode::StaleNarrowed
    );
    assert!(stale.needs_enforcement_authority_note);
    assert!(!stale.needs_evidence_continuity_note);
}

#[test]
fn parity_drift_across_surfaces_fails() {
    let mut packet = packet();
    // Reword the owner-coverage label on one surface for a shared change.
    packet.consumer_bindings[1]
        .parity_facets
        .owner_coverage_label = "Reworded owner label for shiproom".to_owned();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn merge_blocker_drift_across_surfaces_fails() {
    let mut packet = packet();
    packet.consumer_bindings[3]
        .parity_facets
        .merge_blocker_label = "Different blocker language".to_owned();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ParityDriftAcrossSurfaces));
}

#[test]
fn component_reuse_by_single_consumer_fails() {
    let mut bindings = consumer_bindings();
    // Drop the second DRI-registry-row binding so it is adopted by one consumer.
    bindings.retain(|b| b.binding_id != "bind:dr-7:cli");
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::GovernanceComponentReuseUnproven));
}

#[test]
fn missing_component_coverage_fails() {
    let mut bindings = consumer_bindings();
    bindings.retain(|b| b.component != M5GovernanceComponent::MergeReadinessStrip);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ComponentCoverageMissing));
}

#[test]
fn missing_consumer_coverage_fails() {
    let mut bindings = consumer_bindings();
    // Remove the only release-center bindings.
    bindings.retain(|b| b.consumer != GovernanceComponentConsumer::ReleaseCenter);
    let packet = packet_with(bindings);
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ConsumerCoverageMissing));
}

#[test]
fn help_support_export_without_canonical_refs_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.consumer == GovernanceComponentConsumer::SupportPacket)
        .expect("support-packet binding present");
    packet.consumer_bindings[index].source_contract_refs =
        vec![GOVERNANCE_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn projection_mode_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.evidence_state == GovernanceComponentEvidenceState::OwnerBackupCoverageMissing
        })
        .expect("coverage-narrowed binding present");
    packet.consumer_bindings[index].projection_mode = GovernanceComponentProjectionMode::FullParity;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ProjectionModeMismatch));
}

#[test]
fn parity_state_mismatch_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].parity_state =
        GovernanceComponentParityState::FacetsDisclosedNarrowed;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ParityStateMismatch));
}

#[test]
fn narrowed_binding_without_banner_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    packet.consumer_bindings[index].narrow_banner = None;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn full_parity_binding_with_banner_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].narrow_banner = Some(GovernanceComponentNarrowBanner {
        reason: GovernanceComponentNarrowReason::EnforcementAdvisoryOrLocalEstimate,
        preserved_facets_note: "note".to_owned(),
        next_action: GovernanceComponentNarrowNextAction::ReviewEnforcementAuthority,
        next_action_label: "Review".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::NarrowBannerMissing));
}

#[test]
fn narrow_reason_mismatch_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.evidence_state == GovernanceComponentEvidenceState::OwnerBackupCoverageMissing
        })
        .expect("coverage-narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.reason = GovernanceComponentNarrowReason::ProofStaleRelativeToChange;
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::NarrowReasonMismatch));
}

#[test]
fn narrow_banner_missing_preserved_facets_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .expect("narrowed binding present");
    if let Some(banner) = packet.consumer_bindings[index].narrow_banner.as_mut() {
        banner.preserved_facets_note = String::new();
    }
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::NarrowBannerPreservedFacetsMissing));
}

#[test]
fn enforcement_authority_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.evidence_state == GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate
        })
        .expect("enforcement-narrowed binding present");
    packet.consumer_bindings[index].enforcement_authority_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::EnforcementAuthorityNoteMissing));
}

#[test]
fn evidence_continuity_note_missing_fails() {
    let mut packet = packet();
    let index = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.evidence_state == GovernanceComponentEvidenceState::OwnerBackupCoverageMissing
        })
        .expect("coverage-narrowed binding present");
    packet.consumer_bindings[index].evidence_continuity_note = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::EvidenceContinuityNoteMissing));
}

#[test]
fn governance_state_vocab_missing_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].governance_state_vocab.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::GovernanceStateVocabMissing));
}

#[test]
fn advisory_owner_reads_as_provider_authoritative_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].advisory_owner_reads_as_provider_authoritative = true;
    assert!(packet.validate().contains(
        &GovernanceComponentConsumerViolation::AdvisoryOwnerReadsAsProviderAuthoritative
    ));
}

#[test]
fn guarded_merge_hides_missing_backup_coverage_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].guarded_merge_hides_missing_backup_coverage = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::GuardedMergeHidesMissingBackupCoverage));
}

#[test]
fn guarded_merge_hides_expired_approver_state_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].guarded_merge_hides_expired_approver_state = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::GuardedMergeHidesExpiredApproverState));
}

#[test]
fn public_surface_change_hides_diff_or_migration_evidence_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].public_surface_change_hides_diff_or_migration_evidence = true;
    assert!(packet.validate().contains(
        &GovernanceComponentConsumerViolation::PublicSurfaceChangeHidesDiffOrMigrationEvidence
    ));
}

#[test]
fn governance_labels_reworded_per_surface_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].rewords_governance_labels_per_surface = true;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::GovernanceLabelsRewordedPerSurface));
}

#[test]
fn parity_facet_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0]
        .parity_facets
        .public_surface_impact_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ParityFacetIncomplete));
}

#[test]
fn incomplete_binding_fails() {
    let mut packet = packet();
    packet.consumer_bindings[0].governed_change_label = String::new();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::BindingIncomplete));
}

#[test]
fn missing_bindings_fails() {
    let mut packet = packet();
    packet.consumer_bindings.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ConsumerBindingsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .advisory_never_reads_as_provider_authoritative = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .every_component_adopted_by_two_or_more_consumers = false;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn downgrade_triggers_missing_fails() {
    let mut packet = packet();
    packet.downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GovernanceComponentConsumerViolation::DowngradeTriggersMissing));
}

#[test]
fn markdown_summary_lists_bindings() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Consumer bindings"));
    assert!(summary.contains("protected_path_row"));
    assert!(summary.contains("merge_readiness_strip"));
    assert!(summary.contains("enforcement_narrowed"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_governance_component_consumer_export()
        .expect("checked governance consumer export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-consumers/enforcement_and_coverage_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance-component-consumers/public_surface_and_stale_narrowed.json"
        )),
    ] {
        let packet: GovernanceComponentConsumerPacket =
            serde_json::from_str(raw).expect("fixture parses as governance consumer packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

// --- Fixture builders ---------------------------------------------------------

/// Re-derives the canonical bindings after overriding some changes' evidence state,
/// keeping the parity facets identical per change so the packet still validates.
fn bindings_with_evidence_overrides(
    overrides: &[(&str, GovernanceComponentEvidenceState)],
) -> Vec<GovernanceComponentConsumerBinding> {
    consumer_bindings()
        .into_iter()
        .map(|existing| {
            if let Some((_, evidence)) = overrides
                .iter()
                .find(|(change_id, _)| *change_id == existing.governed_change_id)
            {
                binding(
                    &existing.binding_id,
                    &existing.governed_change_id,
                    &existing.governed_change_label,
                    existing.component,
                    existing.consumer,
                    *evidence,
                    &existing.parity_facets,
                )
            } else {
                existing
            }
        })
        .collect()
}

fn fixture_enforcement_and_coverage_narrowed() -> GovernanceComponentConsumerPacket {
    let bindings = bindings_with_evidence_overrides(&[
        (
            "chg:pp-1",
            GovernanceComponentEvidenceState::EnforcementAdvisoryOrLocalEstimate,
        ),
        (
            "chg:dr-7",
            GovernanceComponentEvidenceState::OwnerBackupCoverageMissing,
        ),
    ]);
    GovernanceComponentConsumerPacket::new(GovernanceComponentConsumerPacketInput {
        packet_id: "governance-component-consumer:fixture:enforcement-and-coverage-narrowed"
            .to_owned(),
        surface_label:
            "Shared protected-path governance-component consumers: enforcement and coverage narrowed"
                .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
            M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

fn fixture_public_surface_and_stale_narrowed() -> GovernanceComponentConsumerPacket {
    let bindings = bindings_with_evidence_overrides(&[
        (
            "chg:pp-1",
            GovernanceComponentEvidenceState::PublicSurfaceEvidenceMissing,
        ),
        (
            "chg:dr-7",
            GovernanceComponentEvidenceState::ProofStaleRelativeToChange,
        ),
    ]);
    GovernanceComponentConsumerPacket::new(GovernanceComponentConsumerPacketInput {
        packet_id: "governance-component-consumer:fixture:public-surface-and-stale-narrowed"
            .to_owned(),
        surface_label:
            "Shared protected-path governance-component consumers: public surface and stale narrowed"
                .to_owned(),
        consumer_bindings: bindings,
        downgrade_triggers: vec![
            M5GovernanceComponentDowngradeTrigger::PublicSurfaceDiffUnavailable,
            M5GovernanceComponentDowngradeTrigger::ProofStale,
        ],
        consumer_surfaces: consumer_surfaces(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-08T00:00:00Z".to_owned(),
    })
}

/// Regenerates the checked-in support export, summary, and fixtures.
///
/// Gated behind `GEN_GOVERNANCE_COMPONENT_CONSUMER_ARTIFACTS` so it never writes
/// during a normal test run. Run with the env var set to refresh the artifacts after
/// a contract change, then review the diff.
#[test]
fn regenerate_governance_component_consumer_artifacts() {
    if std::env::var("GEN_GOVERNANCE_COMPONENT_CONSUMER_ARTIFACTS").is_err() {
        return;
    }

    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = format!("{manifest}/../..");

    let canonical = packet();
    assert!(
        canonical.validate().is_empty(),
        "{:?}",
        canonical.validate()
    );

    let artifact_dir =
        format!("{root}/artifacts/release/m5-protected-path-governance-consumers-proof");
    std::fs::create_dir_all(&artifact_dir).expect("create artifact dir");
    std::fs::write(
        format!("{artifact_dir}/support_export.json"),
        format!("{}\n", canonical.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        format!("{artifact_dir}/summary.md"),
        canonical.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir =
        format!("{root}/fixtures/ui/m5-protected-path-governance-component-consumers");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for (name, fixture) in [
        (
            "enforcement_and_coverage_narrowed.json",
            fixture_enforcement_and_coverage_narrowed(),
        ),
        (
            "public_surface_and_stale_narrowed.json",
            fixture_public_surface_and_stale_narrowed(),
        ),
    ] {
        assert!(
            fixture.validate().is_empty(),
            "{name}: {:?}",
            fixture.validate()
        );
        std::fs::write(
            format!("{fixture_dir}/{name}"),
            format!("{}\n", fixture.export_safe_json()),
        )
        .expect("write fixture");
    }
}
