use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_release_center_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RELEASE_CENTER_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_release_center_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5ReleaseCenterComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5ReleaseCenterComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_release_center_component_matrix();
    for row in &packet.component_rows {
        for label in M5ReleaseCenterRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_release_center_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.candidate_scope_classes.is_empty(),
            family.is_candidate(),
            "candidate_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.candidate_blocker_states.is_empty(),
            family.is_candidate(),
            "candidate_blocker_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.version_bump_classes.is_empty(),
            family.is_version_bump(),
            "version_bump_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.compatibility_impacts.is_empty(),
            family.is_version_bump(),
            "compatibility_impacts presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.target_visibilities.is_empty(),
            family.is_publish_target(),
            "target_visibilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.target_mutabilities.is_empty(),
            family.is_publish_target(),
            "target_mutabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.target_auth_sources.is_empty(),
            family.is_publish_target(),
            "target_auth_sources presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.dry_run_availabilities.is_empty(),
            family.is_publish_target(),
            "dry_run_availabilities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.signature_statuses.is_empty(),
            family.is_provenance(),
            "signature_statuses presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.attestation_statuses.is_empty(),
            family.is_provenance(),
            "attestation_statuses presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sbom_statuses.is_empty(),
            family.is_provenance(),
            "sbom_statuses presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.digest_lineage_states.is_empty(),
            family.is_provenance(),
            "digest_lineage_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.rollout_rings.is_empty(),
            family.is_promotion(),
            "rollout_rings presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.promotion_stage_states.is_empty(),
            family.is_promotion(),
            "promotion_stage_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.rollback_blast_radii.is_empty(),
            family.is_rollback(),
            "rollback_blast_radii presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.revocation_scopes.is_empty(),
            family.is_rollback(),
            "revocation_scopes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_release_center_component_matrix();
    for scope in M5CandidateScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.candidate_scope_classes.contains(&scope)),
            "no component declares candidate scope class {}",
            scope.as_str()
        );
    }
    for state in M5CandidateBlockerState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.candidate_blocker_states.contains(&state)),
            "no component declares candidate blocker state {}",
            state.as_str()
        );
    }
    for class in M5VersionBumpClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.version_bump_classes.contains(&class)),
            "no component declares version bump class {}",
            class.as_str()
        );
    }
    for impact in M5CompatibilityImpact::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.compatibility_impacts.contains(&impact)),
            "no component declares compatibility impact {}",
            impact.as_str()
        );
    }
    for vis in M5PublishTargetVisibility::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.target_visibilities.contains(&vis)),
            "no component declares target visibility {}",
            vis.as_str()
        );
    }
    for mut_ in M5TargetMutability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.target_mutabilities.contains(&mut_)),
            "no component declares target mutability {}",
            mut_.as_str()
        );
    }
    for auth in M5TargetAuthSource::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.target_auth_sources.contains(&auth)),
            "no component declares target auth source {}",
            auth.as_str()
        );
    }
    for dry in M5DryRunAvailability::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dry_run_availabilities.contains(&dry)),
            "no component declares dry-run availability {}",
            dry.as_str()
        );
    }
    for sig in M5SignatureStatus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.signature_statuses.contains(&sig)),
            "no component declares signature status {}",
            sig.as_str()
        );
    }
    for att in M5AttestationStatus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.attestation_statuses.contains(&att)),
            "no component declares attestation status {}",
            att.as_str()
        );
    }
    for sbom in M5SbomStatus::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.sbom_statuses.contains(&sbom)),
            "no component declares SBOM status {}",
            sbom.as_str()
        );
    }
    for lineage in M5DigestLineageState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.digest_lineage_states.contains(&lineage)),
            "no component declares digest lineage state {}",
            lineage.as_str()
        );
    }
    for ring in M5RolloutRing::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.rollout_rings.contains(&ring)),
            "no component declares rollout ring {}",
            ring.as_str()
        );
    }
    for stage in M5PromotionStageState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.promotion_stage_states.contains(&stage)),
            "no component declares promotion stage state {}",
            stage.as_str()
        );
    }
    for radius in M5RollbackBlastRadius::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.rollback_blast_radii.contains(&radius)),
            "no component declares rollback blast radius {}",
            radius.as_str()
        );
    }
    for scope in M5RevocationScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.revocation_scopes.contains(&scope)),
            "no component declares revocation scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5ReleaseCenterComponentFamily::PublishTargetRow);
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.vocabulary_set.target_auth_sources.pop();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ReleaseCenterRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn candidate_vocab_missing_fails_for_candidate_card() {
    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::ReleaseCandidateCard)
        .expect("candidate card present");
    row.candidate_scope_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::CandidateScopeClassMissing));

    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::ReleaseCandidateCard)
        .expect("candidate card present");
    row.candidate_blocker_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::CandidateBlockerStateMissing));
}

#[test]
fn version_bump_vocab_missing_fails_for_version_bump_row() {
    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::VersionBumpRow)
        .expect("version bump row present");
    row.version_bump_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::VersionBumpClassMissing));

    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::VersionBumpRow)
        .expect("version bump row present");
    row.compatibility_impacts.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::CompatibilityImpactMissing));
}

#[test]
fn publish_target_vocab_missing_fails_for_publish_target_row() {
    for clear in [0u8, 1, 2, 3] {
        let mut packet = seeded_m5_release_center_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ReleaseCenterComponentFamily::PublishTargetRow)
            .expect("publish target row present");
        let expected = match clear {
            0 => {
                row.target_visibilities.clear();
                M5ReleaseCenterMatrixViolation::TargetVisibilityMissing
            }
            1 => {
                row.target_mutabilities.clear();
                M5ReleaseCenterMatrixViolation::TargetMutabilityMissing
            }
            2 => {
                row.target_auth_sources.clear();
                M5ReleaseCenterMatrixViolation::TargetAuthSourceMissing
            }
            _ => {
                row.dry_run_availabilities.clear();
                M5ReleaseCenterMatrixViolation::DryRunAvailabilityMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn provenance_vocab_missing_fails_for_provenance_card() {
    for clear in [0u8, 1, 2, 3] {
        let mut packet = seeded_m5_release_center_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ReleaseCenterComponentFamily::ArtifactProvenanceBundleCard
            })
            .expect("provenance card present");
        let expected = match clear {
            0 => {
                row.signature_statuses.clear();
                M5ReleaseCenterMatrixViolation::SignatureStatusMissing
            }
            1 => {
                row.attestation_statuses.clear();
                M5ReleaseCenterMatrixViolation::AttestationStatusMissing
            }
            2 => {
                row.sbom_statuses.clear();
                M5ReleaseCenterMatrixViolation::SbomStatusMissing
            }
            _ => {
                row.digest_lineage_states.clear();
                M5ReleaseCenterMatrixViolation::DigestLineageStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn promotion_vocab_missing_fails_for_promotion_step() {
    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::PromotionTimelineStep)
        .expect("promotion timeline step present");
    row.rollout_rings.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::RolloutRingMissing));

    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::PromotionTimelineStep)
        .expect("promotion timeline step present");
    row.promotion_stage_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::PromotionStageStateMissing));
}

#[test]
fn rollback_vocab_missing_fails_for_rollback_row() {
    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::RollbackRevocationRow)
        .expect("rollback revocation row present");
    row.rollback_blast_radii.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::RollbackBlastRadiusMissing));

    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::RollbackRevocationRow)
        .expect("rollback revocation row present");
    row.revocation_scopes.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::RevocationScopeMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[2].masks_target_auth_source_or_mutability = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[3].conflates_signed_and_unsigned_provenance = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[0].invents_private_release_status_grammar = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[5].overstates_rollback_reversibility_or_drops_evidence_freshness = true;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ReleaseCenterComponentFamily::ReleaseCandidateCard)
        .expect("candidate card present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet
        .governance_review
        .no_component_invents_second_status_grammar = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet
        .consumer_projection
        .provenance_surfaces_consume_signature_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_release_center_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ReleaseCenterMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_release_center_component_matrix().render_markdown_summary();
    for family in M5ReleaseCenterComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_release_center_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ReleaseCenterComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5ReleaseCenterComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_release_center_component_matrix_export()
        .expect("checked M5 release center matrix export validates");
    assert_eq!(packet.packet_id, M5_RELEASE_CENTER_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_release_center_component_matrix_export()
        .expect("checked M5 release center matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_release_center_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed(),
        seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5ReleaseCenterComponentFamily::ALL.len()
        );
    }

    let promotion =
        seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed();
    let row = promotion
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ReleaseCenterComponentFamily::PromotionTimelineStep)
        .expect("promotion-timeline-step row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let rollback =
        seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed();
    let row = rollback
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ReleaseCenterComponentFamily::RollbackRevocationRow)
        .expect("rollback-revocation-row row present");
    assert_eq!(
        row.qualification,
        M5ReleaseCenterQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let promotion: M5ReleaseCenterMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-release-center-components/promotion_timeline_step_beta_narrowed.json"
    )))
    .expect("promotion fixture parses");
    assert!(promotion.validate().is_empty());
    assert_eq!(
        promotion,
        seeded_m5_release_center_component_matrix_promotion_timeline_step_beta_narrowed()
    );

    let rollback: M5ReleaseCenterMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-release-center-components/rollback_revocation_row_preview_narrowed.json"
    )))
    .expect("rollback fixture parses");
    assert!(rollback.validate().is_empty());
    assert_eq!(
        rollback,
        seeded_m5_release_center_component_matrix_rollback_revocation_row_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_release_center_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
