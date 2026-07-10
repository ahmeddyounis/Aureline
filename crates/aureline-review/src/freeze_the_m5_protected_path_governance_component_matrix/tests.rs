use super::*;

const PACKET_ID: &str = "m5-protected-path-governance-component-matrix:stable:0001";

fn component_rows() -> Vec<M5GovernanceComponentMatrixRow> {
    vec![
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::ProtectedPathRow,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Protected-path row naming why a file or surface is guarded (owner rule, protected-path policy, public-surface class) so the protection reason is always explicit rather than an anonymous lock icon".to_owned(),
            enforcement_distinction: "Provider-enforced branch protection is labeled provider_authoritative; a local protected-path match is labeled authoritative when the manifest enforces it and advisory when it is only a hint; the local match count is labeled local_estimate and never presented as the provider's final gate".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Advisory,
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
                M5GovernanceStateVocab::Stale,
            ],
            escalation_boundary: "Requesting a protected-path exception on the provider or shiproom is an explicit handoff with a labeled return path to the review workspace".to_owned(),
            backup_coverage_fallback: "When provider protection state is stale the row keeps the last-known protection reason labeled stale and continues local protected-path matching without asserting the provider's gate".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:protected-path-row-protection-reason:m5".to_owned(),
                "evidence:protected-path-row-enforcement-authority:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
                M5GovernanceComponentDowngradeTrigger::TrustNarrowing,
                M5GovernanceComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::GovernanceDashboard,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::OwnershipCard,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Ownership card naming the owner source (CODEOWNERS entry, DRI registry, manifest) and whether the guarded path is covered or has missing backup coverage".to_owned(),
            enforcement_distinction: "Provider-resolved owners are labeled provider_authoritative; owners derived from a local manifest are labeled authoritative when enforced and advisory when a hint; an advisory owner hint never masquerades as provider_authoritative enforcement".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Advisory,
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::Covered,
                M5GovernanceStateVocab::BackupMissing,
                M5GovernanceStateVocab::ProviderAuthoritative,
            ],
            escalation_boundary: "Requesting an owner or backup assignment on the provider or shiproom is an explicit handoff with a labeled return path to the ownership card".to_owned(),
            backup_coverage_fallback: "When owner backup coverage is missing the card labels the path backup_missing, keeps the primary owner shown, and never presents the path as covered".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:ownership-card-owner-source:m5".to_owned(),
                "evidence:ownership-card-backup-coverage:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
                M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::OwnerCoveragePanel,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::ApproverMatrix,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Approver matrix naming which approvers are required per protected path and each approver's state (satisfied, waived, expired, stale) rather than a single approved/blocked pill".to_owned(),
            enforcement_distinction: "Provider-recomputed approval state is labeled provider_authoritative; a local prediction of remaining approvals is labeled local_estimate and never asserted as the provider's decision".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::Waived,
                M5GovernanceStateVocab::Expired,
                M5GovernanceStateVocab::Stale,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Re-requesting or waiving an approval on the provider or shiproom is an explicit handoff with a labeled return path to the approver matrix".to_owned(),
            backup_coverage_fallback: "When provider approval state is stale the matrix keeps last-known approver rows labeled stale, marks any expired approvals expired, and continues local review without asserting fresh approval".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:approver-matrix-required-approvers:m5".to_owned(),
                "evidence:approver-matrix-approver-state:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::ApproverStateExpired,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
                M5GovernanceComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ProviderMutationAttributable,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::GovernanceDashboard,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::ReviewPackSummary,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Review-pack summary naming review-pack freshness and parity so a stale review pack or a pack that has drifted from the current head is visible rather than presented as fresh".to_owned(),
            enforcement_distinction: "Provider-confirmed review-pack results are labeled provider_authoritative; the locally evaluated review-pack parity is labeled local_estimate and never presented as the provider's confirmed verdict".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::Stale,
                M5GovernanceStateVocab::Expired,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Re-running the review pack on the provider or shiproom is an explicit handoff with a labeled return path to the review-pack summary".to_owned(),
            backup_coverage_fallback: "When the review pack is stale the summary labels it stale, shows the last-known parity result, and offers a local re-run to restore parity without asserting fresh provider truth".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:review-pack-summary-freshness:m5".to_owned(),
                "evidence:review-pack-summary-parity:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::ReviewPackStale,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
                M5GovernanceComponentDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::ReleaseCandidate,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::PublicSurfaceDiffCard,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Public-surface diff card naming the change class (command, schema, SDK, token) and carrying a machine-generated diff so a public-surface change never lands without diff and migration context".to_owned(),
            enforcement_distinction: "Provider-published surface truth is labeled provider_authoritative; the locally generated public-surface diff is labeled local_estimate until the provider confirms it and never presented as the published surface of record".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::Stale,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Publishing or migrating a public surface on the provider or shiproom is an explicit handoff with a labeled return path to the public-surface diff card".to_owned(),
            backup_coverage_fallback: "When the machine-generated diff is unavailable the card blocks the public-surface claim, labels the diff missing, and never presents the change as a safe no-op".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:public-surface-diff-card-change-class:m5".to_owned(),
                "evidence:public-surface-diff-card-machine-diff:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::PublicSurfaceDiffUnavailable,
                M5GovernanceComponentDowngradeTrigger::MigrationEvidenceMissing,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::EvidencePreservedNoRevert,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReleaseCandidate,
                M5GovernanceComponentConsumerSurface::GovernanceDashboard,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::MergeControlBanner,
            maturity: M5GovernanceComponentMaturityClass::Stable,
            scope_summary: "Merge-control banner naming each merge blocker (missing owner backup, expired approval, stale review pack, unreviewed public surface) rather than collapsing them into one generic warning pill".to_owned(),
            enforcement_distinction: "Provider-enforced merge blockers are labeled provider_authoritative; locally predicted blockers are labeled local_estimate and never asserted as the provider's final block".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Advisory,
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::BackupMissing,
                M5GovernanceStateVocab::Expired,
                M5GovernanceStateVocab::Stale,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Overriding or waiving a merge blocker on the provider or shiproom is an explicit handoff with a labeled return path to the merge-control banner".to_owned(),
            backup_coverage_fallback: "When provider merge-control state is stale the banner keeps last-known blockers labeled stale, names each blocker individually, and never flattens them into a single ready/blocked state".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-control-banner-blocker-truth:m5".to_owned(),
                "evidence:merge-control-banner-enforcement-authority:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::OwnerCoverageBackupMissing,
                M5GovernanceComponentDowngradeTrigger::ApproverStateExpired,
                M5GovernanceComponentDowngradeTrigger::PolicyBlocked,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReturnPathPreserved,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::Shiproom,
                M5GovernanceComponentConsumerSurface::CliHeadless,
                M5GovernanceComponentConsumerSurface::SupportExport,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::DriRegistryRow,
            maturity: M5GovernanceComponentMaturityClass::Beta,
            scope_summary: "DRI-registry row naming the directly responsible individual for a guarded surface and whether the DRI has coverage or a gap, so a surface with no DRI is visible rather than silently unowned".to_owned(),
            enforcement_distinction: "A registry-recorded DRI is labeled authoritative; an inferred DRI from recent authorship is labeled advisory and local_estimate and never presented as the registry's authoritative assignment".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Advisory,
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::Covered,
                M5GovernanceStateVocab::BackupMissing,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Assigning or reassigning a DRI on the shiproom or governance surface is an explicit handoff with a labeled return path to the DRI registry".to_owned(),
            backup_coverage_fallback: "When the DRI registry has a coverage gap the row labels the surface backup_missing, shows any inferred DRI as advisory, and never presents the surface as covered".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:dri-registry-row-dri-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::DriCoverageGap,
                M5GovernanceComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::GovernanceDashboard,
                M5GovernanceComponentConsumerSurface::OwnerCoveragePanel,
                M5GovernanceComponentConsumerSurface::SupportExport,
                M5GovernanceComponentConsumerSurface::HelpAbout,
            ],
        },
        M5GovernanceComponentMatrixRow {
            component: M5GovernanceComponent::MergeReadinessStrip,
            maturity: M5GovernanceComponentMaturityClass::Preview,
            scope_summary: "Merge-readiness strip summarizing blocking state and ownership as a compact strip; blocking reasons stay explicit rather than collapsed into a single ready/not-ready pill".to_owned(),
            enforcement_distinction: "Provider-enforced readiness gates are labeled provider_authoritative; the local readiness estimate is labeled local_estimate and never presented as the provider's final gate".to_owned(),
            governance_state_vocab: vec![
                M5GovernanceStateVocab::Authoritative,
                M5GovernanceStateVocab::BackupMissing,
                M5GovernanceStateVocab::Stale,
                M5GovernanceStateVocab::ProviderAuthoritative,
                M5GovernanceStateVocab::LocalEstimate,
            ],
            escalation_boundary: "Resolving a readiness blocker on the provider or shiproom is an explicit handoff with a labeled return path to the merge-readiness strip".to_owned(),
            backup_coverage_fallback: "When provider readiness state is stale the strip keeps last-known gates labeled stale and continues local readiness estimation without asserting provider approval".to_owned(),
            evidence_requirement: M5GovernanceComponentEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:merge-readiness-strip-blocking-truth:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5GovernanceComponentDowngradeTrigger::ProofStale,
                M5GovernanceComponentDowngradeTrigger::EscalationHandoffUnavailable,
                M5GovernanceComponentDowngradeTrigger::ScopeExpansionUnqualified,
            ],
            rollback_posture: M5GovernanceComponentRollbackPosture::ReadOnlyNoMutation,
            source_contract_refs: vec![
                M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5GovernanceComponentConsumerSurface::ReviewWorkspace,
                M5GovernanceComponentConsumerSurface::Shiproom,
                M5GovernanceComponentConsumerSurface::SupportExport,
                M5GovernanceComponentConsumerSurface::HelpAbout,
            ],
        },
    ]
}

fn trust_review() -> M5GovernanceComponentMatrixTrustReview {
    M5GovernanceComponentMatrixTrustReview {
        advisory_never_masquerades_as_authoritative: true,
        provider_authoritative_versus_local_estimate_distinct: true,
        owner_coverage_backup_missing_explicit: true,
        approver_expired_waived_stale_explicit: true,
        review_pack_freshness_and_parity_explicit: true,
        public_surface_diff_machine_generated_required: true,
        migration_evidence_required_for_public_surface_change: true,
        protection_reason_always_explicit: true,
        dri_coverage_gap_explicit: true,
        merge_control_blocker_never_generic: true,
        escalation_handoff_explicit: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> M5GovernanceComponentMatrixConsumerProjection {
    M5GovernanceComponentMatrixConsumerProjection {
        protected_path_row_shows_reason_and_enforcement_authority: true,
        ownership_card_shows_owner_source_and_coverage: true,
        approver_matrix_shows_required_and_state: true,
        review_pack_summary_shows_freshness_and_parity: true,
        public_surface_diff_card_shows_change_class_and_diff: true,
        merge_control_banner_shows_blockers_not_generic: true,
        dri_registry_row_shows_dri_and_coverage: true,
        merge_readiness_strip_shows_blocking_and_ownership: true,
        cli_headless_shows_component_truth: true,
        support_export_shows_component_truth: true,
    }
}

fn proof_freshness() -> M5GovernanceComponentMatrixProofFreshness {
    M5GovernanceComponentMatrixProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-10T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_GOVERNANCE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_PROTECTED_PATH_ROW_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_OWNERSHIP_CARD_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_APPROVER_MATRIX_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_REVIEW_PACK_SUMMARY_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_PUBLIC_SURFACE_DIFF_CARD_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_CONTROL_BANNER_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_DRI_REGISTRY_ROW_CONTRACT_REF.to_owned(),
        M5_GOVERNANCE_COMPONENT_MATRIX_MERGE_READINESS_STRIP_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> M5GovernanceComponentMatrixPacket {
    M5GovernanceComponentMatrixPacket::new(M5GovernanceComponentMatrixPacketInput {
        packet_id: PACKET_ID.to_owned(),
        matrix_label: "M5 Protected-Path Governance Component Matrix".to_owned(),
        component_rows: component_rows(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-10T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_governance_component_matrix_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn frozen_governance_state_vocabulary_is_exact() {
    assert_eq!(
        M5GovernanceStateVocab::ALL.map(M5GovernanceStateVocab::as_str),
        [
            "advisory",
            "authoritative",
            "covered",
            "backup_missing",
            "waived",
            "expired",
            "stale",
            "provider_authoritative",
            "local_estimate",
        ]
    );
}

#[test]
fn every_component_binds_its_own_contract_ref() {
    for row in packet().component_rows {
        assert!(
            row.source_contract_refs
                .contains(&row.component.contract_ref().to_owned()),
            "component {} missing its per-component contract ref",
            row.component.as_str()
        );
    }
}

#[test]
fn missing_component_fails_validation() {
    let mut packet = packet();
    packet
        .component_rows
        .retain(|row| row.component != M5GovernanceComponent::PublicSurfaceDiffCard);
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn stable_component_missing_evidence_fails() {
    let mut packet = packet();
    packet.component_rows[0]
        .required_evidence_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::StableComponentMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.component_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.component_rows[2].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_enforcement_distinction_fails() {
    let mut packet = packet();
    packet.component_rows[0].enforcement_distinction = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::EnforcementDistinctionMissing));
}

#[test]
fn missing_governance_state_vocab_fails() {
    let mut packet = packet();
    packet.component_rows[4].governance_state_vocab.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::GovernanceStateVocabMissing));
}

#[test]
fn missing_escalation_boundary_fails() {
    let mut packet = packet();
    packet.component_rows[3].escalation_boundary = String::new();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::EscalationBoundaryMissing));
}

#[test]
fn missing_backup_coverage_fallback_fails() {
    let mut packet = packet();
    packet.component_rows[5].backup_coverage_fallback = String::new();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::BackupCoverageFallbackMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .advisory_never_masquerades_as_authoritative = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .public_surface_diff_card_shows_change_class_and_diff = false;
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5GovernanceComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_every_component() {
    let summary = packet().render_markdown_summary();
    for component in M5GovernanceComponent::ALL {
        assert!(
            summary.contains(component.as_str()),
            "summary missing component {}",
            component.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_governance_component_matrix_export()
        .expect("checked M5 governance-component matrix export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed_packet() {
    let seed = packet();
    let checked = current_stable_m5_governance_component_matrix_export()
        .expect("checked M5 governance-component matrix export validates");
    assert_eq!(checked, seed);
}

/// Gated generator for the checked artifacts and narrowed fixtures.
///
/// Run in isolation with the env gate set, then run the full suite:
/// `GEN_GOVERNANCE_COMPONENT_MATRIX_ARTIFACTS=1 cargo test -p aureline-review
/// freeze_the_m5_protected_path_governance_component_matrix::tests::gen_governance_component_matrix_artifacts
/// -- --exact --ignored`
#[test]
#[ignore = "artifact generator; run explicitly with the env gate set"]
fn gen_governance_component_matrix_artifacts() {
    if std::env::var("GEN_GOVERNANCE_COMPONENT_MATRIX_ARTIFACTS").is_err() {
        return;
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");

    let seed = packet();
    let proof_dir = root.join("artifacts/release/m5-protected-path-governance-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", seed.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(proof_dir.join("summary.md"), seed.render_markdown_summary())
        .expect("write summary");

    let fixture_dir = root.join("fixtures/ui/m5-protected-path-governance");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    let mut backup_missing = packet();
    backup_missing.packet_id =
        "m5-protected-path-governance-component-matrix:ownership-backup-missing:0001".to_owned();
    if let Some(row) = backup_missing
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5GovernanceComponent::OwnershipCard)
    {
        row.maturity = M5GovernanceComponentMaturityClass::Beta;
    }
    assert!(
        backup_missing.validate().is_empty(),
        "{:?}",
        backup_missing.validate()
    );
    std::fs::write(
        fixture_dir.join("ownership_card_backup_missing_narrowed.json"),
        format!("{}\n", backup_missing.export_safe_json()),
    )
    .expect("write backup-missing fixture");

    let mut banner_held = packet();
    banner_held.packet_id =
        "m5-protected-path-governance-component-matrix:merge-control-held:0001".to_owned();
    if let Some(row) = banner_held
        .component_rows
        .iter_mut()
        .find(|row| row.component == M5GovernanceComponent::MergeControlBanner)
    {
        row.maturity = M5GovernanceComponentMaturityClass::Held;
    }
    assert!(
        banner_held.validate().is_empty(),
        "{:?}",
        banner_held.validate()
    );
    std::fs::write(
        fixture_dir.join("merge_control_banner_held.json"),
        format!("{}\n", banner_held.export_safe_json()),
    )
    .expect("write banner-held fixture");
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance/ownership_card_backup_missing_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-protected-path-governance/merge_control_banner_held.json"
        )),
    ] {
        let packet: M5GovernanceComponentMatrixPacket =
            serde_json::from_str(raw).expect("fixture parses as matrix packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
