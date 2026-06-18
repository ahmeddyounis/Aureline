use super::*;

fn packet() -> PackageMutationCertification {
    current_package_mutation_certification().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PACKAGE_MUTATION_CERTIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PACKAGE_MUTATION_CERTIFICATION_RECORD_KIND
    );
    assert_eq!(packet.matrix_ref, M5_PACKAGE_STATE_MATRIX_PATH);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn matrix_is_complete_and_unique() {
    let packet = packet();
    assert_eq!(
        packet.rows.len(),
        packet.certified_ecosystems.len() * packet.deployment_profiles.len()
    );
    for &ecosystem in &packet.certified_ecosystems {
        for &profile in &packet.deployment_profiles {
            assert!(
                packet.row(ecosystem, profile).is_some(),
                "missing cell {}/{}",
                ecosystem.as_str(),
                profile.as_str()
            );
        }
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_claim,
            row.effective_claim(),
            "row {} publishes beyond the gate",
            row.row_id
        );
        assert_eq!(
            row.narrowing_action,
            row.required_narrowing(),
            "row {} narrowing diverges from the gate",
            row.row_id
        );
    }
}

#[test]
fn every_row_carries_complete_dimensions_and_surfaces() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.has_complete_dimensions(),
            "row {} is missing a dimension proof",
            row.row_id
        );
        assert!(
            row.has_complete_surfaces(),
            "row {} is missing a surface parity cell",
            row.row_id
        );
        for proof in &row.dimension_proofs {
            assert!(
                !proof.evidence_ref.trim().is_empty(),
                "row {} dimension {} has no evidence",
                row.row_id,
                proof.dimension.as_str()
            );
        }
    }
}

#[test]
fn every_row_carries_its_own_evidence() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            !row.qualification_packet_ref.trim().is_empty(),
            "row {} has no qualification packet",
            row.row_id
        );
        assert!(
            !row.corpus_ref.trim().is_empty(),
            "row {} has no proof corpus",
            row.row_id
        );
    }
}

#[test]
fn parity_dimension_matches_surfaces() {
    let packet = packet();
    assert!(packet.all_rows_parity_consistent());
    for row in &packet.rows {
        let recorded = row
            .dimension_proof(ProofDimension::CrossSurfaceParity)
            .expect("parity dimension present")
            .state;
        assert_eq!(
            recorded,
            row.recomputed_parity_state(),
            "row {} parity dimension diverges from its surfaces",
            row.row_id
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.matrix_ref, packet.matrix_ref);
    assert_eq!(
        projection.all_rows_gate_consistent,
        packet.all_rows_gate_consistent()
    );
    assert_eq!(
        projection.promotable_count,
        packet.promotable_rows().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_rows().count());
    assert_eq!(projection.withheld_count, packet.withheld_rows().count());
    assert_eq!(
        projection.parity_break_count,
        packet.parity_break_rows().count()
    );
    // Each projected row's limiting dimensions match the typed row.
    for (row, exported) in packet.rows.iter().zip(projection.rows.iter()) {
        let limiting: Vec<String> = row
            .limiting_dimensions()
            .iter()
            .map(|d| d.as_str().to_owned())
            .collect();
        assert_eq!(exported.limiting_dimensions, limiting, "row {}", row.row_id);
    }
}

#[test]
fn published_claims_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<MutationClaimClass> =
        packet.rows.iter().map(|r| r.published_claim).collect();
    for claim in MutationClaimClass::ALL {
        assert!(
            present.contains(&claim),
            "no row publishes claim {}",
            claim.as_str()
        );
    }
}

#[test]
fn narrowing_actions_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ClaimNarrowing> =
        packet.rows.iter().map(|r| r.narrowing_action).collect();
    for action in ClaimNarrowing::ALL {
        assert!(
            present.contains(&action),
            "no row exercises narrowing {}",
            action.as_str()
        );
    }
}

#[test]
fn freshness_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<EvidenceFreshness> =
        packet.rows.iter().map(|r| r.evidence_freshness).collect();
    for freshness in EvidenceFreshness::ALL {
        assert!(
            present.contains(&freshness),
            "no row exercises freshness {}",
            freshness.as_str()
        );
    }
}

#[test]
fn deployment_profiles_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DeploymentProfile> =
        packet.rows.iter().map(|r| r.deployment_profile).collect();
    for profile in DeploymentProfile::ALL {
        assert!(
            present.contains(&profile),
            "no row exercises profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn dimension_proof_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<DimensionProofState> = packet
        .rows
        .iter()
        .flat_map(|r| r.dimension_proofs.iter().map(|p| p.state))
        .collect();
    for state in DimensionProofState::ALL {
        assert!(
            present.contains(&state),
            "no dimension proof exercises state {}",
            state.as_str()
        );
    }
}

#[test]
fn parity_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ParityState> = packet
        .rows
        .iter()
        .flat_map(|r| r.surface_parity.iter().map(|c| c.state))
        .collect();
    for state in ParityState::ALL {
        assert!(
            present.contains(&state),
            "no surface parity cell exercises state {}",
            state.as_str()
        );
    }
}

#[test]
fn promotable_rows_are_clean() {
    let packet = packet();
    assert!(
        packet.promotable_rows().count() > 0,
        "fixture needs a certified row"
    );
    for row in packet.promotable_rows() {
        assert!(row.evidence_freshness.is_current());
        assert!(row.is_fully_proven());
        assert!(row
            .surface_parity
            .iter()
            .all(|c| c.state == ParityState::Consistent));
        assert_eq!(row.published_claim, MutationClaimClass::Certified);
        assert_eq!(row.narrowing_action, ClaimNarrowing::None);
    }
}

#[test]
fn freshness_ceilings_hold() {
    assert_eq!(
        EvidenceFreshness::Current.claim_ceiling(),
        MutationClaimClass::Certified
    );
    assert_eq!(
        EvidenceFreshness::Stale.claim_ceiling(),
        MutationClaimClass::RetestPending
    );
    assert_eq!(
        EvidenceFreshness::Unknown.claim_ceiling(),
        MutationClaimClass::RetestPending
    );
    assert_eq!(
        EvidenceFreshness::Expired.claim_ceiling(),
        MutationClaimClass::Unsupported
    );
}

#[test]
fn dimension_ceilings_hold() {
    assert_eq!(
        DimensionProofState::Proven.claim_ceiling(),
        MutationClaimClass::Certified
    );
    assert_eq!(
        DimensionProofState::Degraded.claim_ceiling(),
        MutationClaimClass::Limited
    );
    assert_eq!(
        DimensionProofState::Stale.claim_ceiling(),
        MutationClaimClass::RetestPending
    );
    assert_eq!(
        DimensionProofState::Unproven.claim_ceiling(),
        MutationClaimClass::Unsupported
    );
}

#[test]
fn claim_rank_orders_low_to_high() {
    assert!(MutationClaimClass::Unsupported.rank() < MutationClaimClass::RetestPending.rank());
    assert!(MutationClaimClass::RetestPending.rank() < MutationClaimClass::Limited.rank());
    assert!(MutationClaimClass::Limited.rank() < MutationClaimClass::Certified.rank());
    assert_eq!(
        MutationClaimClass::Certified.min(MutationClaimClass::Limited),
        MutationClaimClass::Limited
    );
    assert_eq!(
        MutationClaimClass::RetestPending.min(MutationClaimClass::Certified),
        MutationClaimClass::RetestPending
    );
}

#[test]
fn validate_flags_overstated_published_claim() {
    let mut packet = packet();
    // Force a non-certified row to claim it still publishes certified.
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.published_claim != MutationClaimClass::Certified)
    {
        row.published_claim = MutationClaimClass::Certified;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageMutationCertificationViolation::OverstatedPublishedClaim { .. }
        )));
    }
}

#[test]
fn validate_flags_narrowing_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .rows
        .iter_mut()
        .find(|r| r.narrowing_action != ClaimNarrowing::WithholdAsUnsupported)
    {
        row.narrowing_action = ClaimNarrowing::WithholdAsUnsupported;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageMutationCertificationViolation::NarrowingActionMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_parity_state_mismatch() {
    let mut packet = packet();
    // Flip a consistent surface to divergent without updating the parity
    // dimension: the recorded dimension now overclaims relative to the surfaces.
    if let Some(row) = packet.rows.iter_mut().find(|r| {
        r.dimension_proof(ProofDimension::CrossSurfaceParity)
            .map(|p| p.state == DimensionProofState::Proven)
            .unwrap_or(false)
    }) {
        if let Some(cell) = row
            .surface_parity
            .iter_mut()
            .find(|c| c.state == ParityState::Consistent)
        {
            cell.state = ParityState::Divergent;
        }
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageMutationCertificationViolation::ParityStateMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_incomplete_dimensions() {
    let mut packet = packet();
    if let Some(row) = packet.rows.first_mut() {
        row.dimension_proofs.pop();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            PackageMutationCertificationViolation::IncompleteDimensionProofs { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_matrix_cell() {
    let mut packet = packet();
    let removed = packet.rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageMutationCertificationViolation::MissingMatrixCell { .. }
    )));
}

#[test]
fn validate_flags_unclaimed_ecosystem_row() {
    let mut packet = packet();
    packet
        .certified_ecosystems
        .retain(|e| *e != CertifiedEcosystem::PythonPip);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageMutationCertificationViolation::UnclaimedEcosystemRow { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageMutationCertificationViolation::ClosedVocabularyMismatch {
            field: "certified_ecosystems"
        }
    )));
}

#[test]
fn validate_flags_matrix_ref_mismatch() {
    let mut packet = packet();
    packet.matrix_ref = "artifacts/deps/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageMutationCertificationViolation::MatrixRefMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_rows = packet.summary.total_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&PackageMutationCertificationViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(CertifiedEcosystem::Cargo.as_str(), "cargo");
    assert_eq!(CertifiedEcosystem::NodePnpm.as_str(), "node_pnpm");
    assert_eq!(CertifiedEcosystem::PythonPip.as_str(), "python_pip");
    assert_eq!(
        DeploymentProfile::DirectRegistry.as_str(),
        "direct_registry"
    );
    assert_eq!(
        DeploymentProfile::RegistryMirror.as_str(),
        "registry_mirror"
    );
    assert_eq!(
        DeploymentProfile::OfflineSnapshot.as_str(),
        "offline_snapshot"
    );
    assert_eq!(
        ProofDimension::PackageStateTruth.as_str(),
        "package_state_truth"
    );
    assert_eq!(
        ProofDimension::RegistryAuthContinuity.as_str(),
        "registry_auth_continuity"
    );
    assert_eq!(
        ProofDimension::LockfileSafeReview.as_str(),
        "lockfile_safe_review"
    );
    assert_eq!(
        ProofDimension::CrossSurfaceParity.as_str(),
        "cross_surface_parity"
    );
    assert_eq!(MutationClaimClass::RetestPending.as_str(), "retest_pending");
    assert_eq!(
        ClaimNarrowing::WithholdAsUnsupported.as_str(),
        "withhold_as_unsupported"
    );
    assert_eq!(ParityState::Divergent.as_str(), "divergent");
}
