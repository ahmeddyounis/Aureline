use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_build_lane_trust_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BUILD_LANE_TRUST_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_build_lane_family() {
    let packet = seeded_m5_build_lane_trust_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .build_lane_rows
        .iter()
        .map(|r| r.build_lane_family)
        .collect();
    for family in M5BuildLaneFamily::ALL {
        assert!(
            present.contains(&family),
            "missing build lane {}",
            family.as_str()
        );
    }
    assert_eq!(packet.build_lane_rows.len(), M5BuildLaneFamily::ALL.len());
}

#[test]
fn frozen_build_lane_trust_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: cache_posture / publication_authority / credential_boundary /
    // hermetic_input / reproducibility_proof / artifact_convergence / support_identity stays in one
    // controlled token set that no release-center, shiproom, diagnostics, admin, docs, or support surface
    // reinvents.
    let tokens: Vec<&str> = M5BuildLaneTrustRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "cache_posture",
            "publication_authority",
            "credential_boundary",
            "hermetic_input",
            "reproducibility_proof",
            "artifact_convergence",
            "support_identity",
        ]
    );
    assert!(
        M5BuildLaneTrustRole::CachePosture.must_verify_inputs_and_prove_replay_before_promotion()
    );
    assert!(M5BuildLaneTrustRole::PublicationAuthority
        .must_verify_inputs_and_prove_replay_before_promotion());
    assert!(M5BuildLaneTrustRole::ReproducibilityProof
        .must_verify_inputs_and_prove_replay_before_promotion());
    assert!(M5BuildLaneTrustRole::ArtifactConvergence
        .must_verify_inputs_and_prove_replay_before_promotion());
    assert!(!M5BuildLaneTrustRole::CredentialBoundary
        .must_verify_inputs_and_prove_replay_before_promotion());
    assert!(
        !M5BuildLaneTrustRole::HermeticInput.must_verify_inputs_and_prove_replay_before_promotion()
    );
    assert!(!M5BuildLaneTrustRole::SupportIdentity
        .must_verify_inputs_and_prove_replay_before_promotion());
}

#[test]
fn every_lane_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_build_lane_trust_matrix();
    for row in &packet.build_lane_rows {
        for label in M5BuildLaneRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "lane {} missing mandatory label {}",
                row.build_lane_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.build_lane_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "lane {} does not point at its canonical schema",
            row.build_lane_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5BuildLaneAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn lane_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_build_lane_trust_matrix();
    for row in &packet.build_lane_rows {
        let family = row.build_lane_family;
        assert_eq!(
            !row.contributor_pr_roles.is_empty(),
            family.declares_contributor_pr_roles(),
            "contributor_pr_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.protected_merge_roles.is_empty(),
            family.declares_protected_merge_roles(),
            "protected_merge_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.release_roles.is_empty(),
            family.declares_release_roles(),
            "release_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.emergency_hotfix_roles.is_empty(),
            family.declares_emergency_hotfix_roles(),
            "emergency_hotfix_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_lane() {
    let packet = seeded_m5_build_lane_trust_matrix();
    for role in M5BuildLaneTrustRole::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no lane declares build-lane-trust role {}",
            role.as_str()
        );
    }
    for role in M5ContributorPrRole::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.contributor_pr_roles.contains(&role)),
            "no lane declares contributor / PR role {}",
            role.as_str()
        );
    }
    for role in M5ProtectedMergeRole::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.protected_merge_roles.contains(&role)),
            "no lane declares protected-merge role {}",
            role.as_str()
        );
    }
    for role in M5ReleaseRole::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.release_roles.contains(&role)),
            "no lane declares release role {}",
            role.as_str()
        );
    }
    for role in M5EmergencyHotfixRole::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.emergency_hotfix_roles.contains(&role)),
            "no lane declares emergency-hotfix role {}",
            role.as_str()
        );
    }
    for reason in M5BuildLaneDegradedReason::ALL {
        assert!(
            packet
                .build_lane_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no lane declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_build_lane_family_fails_validation() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet
        .build_lane_rows
        .retain(|row| row.build_lane_family != M5BuildLaneFamily::EmergencyHotfix);
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[0]
        .required_labels
        .retain(|label| *label != M5BuildLaneRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let own = M5BuildLaneFamily::ProtectedMerge.canonical_domain_schema_ref();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::ProtectedMerge)
        .expect("protected-merge row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::SemanticRoleMissing));
}

#[test]
fn contributor_pr_role_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::ContributorPr)
        .expect("contributor-pr present");
    row.contributor_pr_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ContributorPrRoleMissing));
}

#[test]
fn protected_merge_role_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::ProtectedMerge)
        .expect("protected-merge present");
    row.protected_merge_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ProtectedMergeRoleMissing));
}

#[test]
fn release_role_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::Release)
        .expect("release present");
    row.release_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ReleaseRoleMissing));
}

#[test]
fn emergency_hotfix_role_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::EmergencyHotfix)
        .expect("emergency-hotfix present");
    row.emergency_hotfix_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::EmergencyHotfixRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::DegradedReasonMissing));
}

#[test]
fn build_lane_invariant_violation_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[0].pr_caches_publish_release_artifacts = true;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated));

    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[1].treats_remote_cache_hits_as_reproducibility_proof = true;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated));

    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[2]
        .lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity = true;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated));

    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[2]
        .overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt = true;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated));

    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[3]
        .hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows =
        true;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::BuildLaneInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    let row = packet
        .build_lane_rows
        .iter_mut()
        .find(|row| row.build_lane_family == M5BuildLaneFamily::ProtectedMerge)
        .expect("protected-merge row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet
        .governance_review
        .contributor_lanes_read_caches_but_never_publish_release_artifacts = false;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_build_lane_source = false;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_build_lane_family() {
    let summary = seeded_m5_build_lane_trust_matrix().render_markdown_summary();
    for family in M5BuildLaneFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing lane {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_build_lane_trust_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5BuildLaneFamily::ALL.len());
    assert!(lines[0].starts_with("build_lane_family,qualification,owner,canonical_schema,"));
    for family in M5BuildLaneFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing lane {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_build_lane_trust_matrix_export()
        .expect("checked M5 build-lane-trust matrix export validates");
    assert_eq!(packet.packet_id, M5_BUILD_LANE_TRUST_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_build_lane_trust_matrix_export()
        .expect("checked M5 build-lane-trust matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_build_lane_trust_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_lanes_visible() {
    for packet in [
        seeded_m5_build_lane_trust_matrix_release_beta_narrowed(),
        seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.build_lane_rows.len(), M5BuildLaneFamily::ALL.len());
    }

    let release = seeded_m5_build_lane_trust_matrix_release_beta_narrowed();
    let row = release
        .build_lane_rows
        .iter()
        .find(|r| r.build_lane_family == M5BuildLaneFamily::Release)
        .expect("release row present");
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Beta);

    let hotfix = seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed();
    let row = hotfix
        .build_lane_rows
        .iter()
        .find(|r| r.build_lane_family == M5BuildLaneFamily::EmergencyHotfix)
        .expect("emergency-hotfix row present");
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let release: M5BuildLaneTrustMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-clean-room-rebuild/clean_room_release_beta_narrowed.json"
    )))
    .expect("release fixture parses");
    assert!(release.validate().is_empty());
    assert_eq!(
        release,
        seeded_m5_build_lane_trust_matrix_release_beta_narrowed()
    );

    let hotfix: M5BuildLaneTrustMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-clean-room-rebuild/clean_room_emergency_hotfix_preview_narrowed.json"
    )))
    .expect("emergency-hotfix fixture parses");
    assert!(hotfix.validate().is_empty());
    assert_eq!(
        hotfix,
        seeded_m5_build_lane_trust_matrix_emergency_hotfix_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_build_lane_trust_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_build_lane_trust_matrix();
    packet.build_lane_rows[0].scope_summary =
        "raw endpoint https://cache.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneTrustMatrixViolation::RawMaterialInExport));
}
