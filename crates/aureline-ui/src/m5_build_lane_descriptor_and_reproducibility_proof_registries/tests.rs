use super::*;

fn clean_descriptor_input() -> M5BuildLaneDescriptorEntryResolutionInput {
    M5BuildLaneDescriptorEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        lane_binding_id: "release.lane.release".to_owned(),
        token_name: "build.lane.descriptor.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        cache_posture: M5BuildLaneCachePostureKind::HermeticNoCache,
        surface_context: M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5BuildLaneResolutionForm::ALL.to_vec(),
        cache_read_scope: "cache.read.none".to_owned(),
        cache_write_scope: "cache.write.none".to_owned(),
        credential_class: "credential.release-signing-scoped".to_owned(),
        publication_rights: "publication.controlled-release-publication".to_owned(),
        expected_artifact_families: "artifacts.binaries-packages-sboms".to_owned(),
        hermetic_input_posture: "hermetic.fully-hermetic".to_owned(),
        clean_room_rebuild_rule: "clean-room.full-rebuild-required".to_owned(),
        bound_to_registry: true,
        publication_authority_bounded: true,
        is_trust_risk_posture: false,
        cache_trust_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_proof_input() -> M5ReproducibilityProofEntryResolutionInput {
    M5ReproducibilityProofEntryResolutionInput {
        entry_id: "proof:test".to_owned(),
        proof_ref: "release.lane.release".to_owned(),
        token_name: "reproducibility.proof.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        convergence_scope: M5ReproducibilityConvergenceScope::VerifiedCacheInputs,
        surface_context: M5BuildLaneSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5BuildLaneResolutionForm::ALL.to_vec(),
        resolved_build_identity: "build-id.sha256.release-0007".to_owned(),
        input_source_ledger: "inputs.verified-cache".to_owned(),
        clean_room_diff_reference: "clean-room.diff.release-0007".to_owned(),
        sidecar_convergence_state: "sidecars.converged-docs-schemas-sboms-symbols".to_owned(),
        attestation_state: "attestation.signed-and-verified".to_owned(),
        rollback_metadata_reference: "rollback.metadata.release-0007".to_owned(),
        last_rebuild_revision: "rebuild.revision.0007".to_owned(),
        keeps_input_source_visible: true,
        proof_is_truthful: true,
        remote_cache_hit_present: false,
        remote_cache_hit_marked_not_proof: false,
        non_hermetic_input_present: false,
        non_hermetic_input_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_build_lane_descriptor_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.descriptor_resolves_across_lanes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.build_lane_descriptor_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.cache_posture_is_classified);
    assert!(resolved.publication_authority_bounded);
    assert_eq!(resolved.semantic_role, "reproducibility_proof");
    assert_eq!(resolved.cache_posture, "hermetic_no_cache");
    assert_eq!(
        resolved.canonical_cache_posture_mode,
        "hermetic_no_cache_posture"
    );
    assert_eq!(resolved.surface_context, "release_center_surface");
    assert_eq!(
        resolved.next_action,
        M5BuildLaneNextAction::ExpandLaneMeaning
    );
}

#[test]
fn descriptor_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.cache_posture = M5BuildLaneCachePostureKind::PostureUnclassified;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::CachePostureUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_publish_fold_and_form_degrade() {
    // An unstated cache write scope leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.cache_write_scope = "  ".to_owned();
    let resolved = resolve_build_lane_descriptor_entry(input).unwrap();
    assert!(!resolved.build_lane_descriptor_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::BuildLaneDescriptorObjectIncomplete)
    );

    // A lane claiming publish rights it must not have degrades with the structured blocker reason.
    let mut input = clean_descriptor_input();
    input.publication_authority_bounded = false;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust)
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5BuildLaneResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_trust_risk_and_surface_and_proof_degrade() {
    // A trust-risk cache posture hiding its cache-trust marker first fails the publish-boundary fold.
    let mut input = clean_descriptor_input();
    input.cache_posture = M5BuildLaneCachePostureKind::SharedReadableUntrusted;
    input.is_trust_risk_posture = true;
    input.cache_trust_disclosed = false;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust)
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5BuildLaneSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_build_lane_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5BuildLaneDescriptorEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_build_lane_descriptor_entry(input).unwrap_err(),
        M5BuildLaneResolutionError::EmptyBuildLaneDescriptorEntryId
    );

    let mut input = clean_descriptor_input();
    input.publication_rights = "see https://cache.internal/leak".to_owned();
    assert_eq!(
        resolve_build_lane_descriptor_entry(input).unwrap_err(),
        M5BuildLaneResolutionError::ForbiddenMaterial
    );
}

#[test]
fn untrusted_lane_cannot_publish_rejects_unbounded() {
    assert!(untrusted_lane_cannot_publish(
        M5BuildLaneCachePostureKind::HermeticNoCache,
        true,
        false,
        true
    ));
    assert!(!untrusted_lane_cannot_publish(
        M5BuildLaneCachePostureKind::HermeticNoCache,
        false,
        false,
        true
    ));
    assert!(untrusted_lane_cannot_publish(
        M5BuildLaneCachePostureKind::SharedReadableUntrusted,
        true,
        true,
        true
    ));
    assert!(!untrusted_lane_cannot_publish(
        M5BuildLaneCachePostureKind::SharedReadableUntrusted,
        true,
        true,
        false
    ));
    assert!(!untrusted_lane_cannot_publish(
        M5BuildLaneCachePostureKind::PostureUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn build_lane_descriptor_object_is_complete_requires_all_fields() {
    assert!(build_lane_descriptor_object_is_complete(
        M5BuildLaneCachePostureKind::HermeticNoCache,
        "cache.read.none",
        "cache.write.none",
        "credential.release-signing-scoped",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "clean-room.full-rebuild-required",
    ));
    assert!(!build_lane_descriptor_object_is_complete(
        M5BuildLaneCachePostureKind::HermeticNoCache,
        "cache.read.none",
        "  ",
        "credential.release-signing-scoped",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "clean-room.full-rebuild-required",
    ));
    assert!(!build_lane_descriptor_object_is_complete(
        M5BuildLaneCachePostureKind::PostureUnclassified,
        "cache.read.none",
        "cache.write.none",
        "credential.release-signing-scoped",
        "publication.controlled-release-publication",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "clean-room.full-rebuild-required",
    ));
}

#[test]
fn proof_clean_stays_honest() {
    let resolved = resolve_reproducibility_proof_entry(clean_proof_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.proof_safe_on_every_lane);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_reproducibility_proof);
    assert!(resolved.reproducibility_proof_stays_honest);
    assert_eq!(resolved.convergence_scope, "verified_cache_inputs");
    assert_eq!(resolved.surface_context, "release_center_surface");
}

#[test]
fn proof_cache_hit_and_unclassified_degrade() {
    // A cache hit that is not marked as never being proof treats the hit as reproducibility proof.
    let mut input = clean_proof_input();
    input.remote_cache_hit_present = true;
    input.remote_cache_hit_marked_not_proof = false;
    let resolved = resolve_reproducibility_proof_entry(input).unwrap();
    assert!(!resolved.provides_complete_reproducibility_proof);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity)
    );

    // A proof that hides the input source is also caught.
    let mut input = clean_proof_input();
    input.keeps_input_source_visible = false;
    assert_eq!(
        resolve_reproducibility_proof_entry(input).unwrap().degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity)
    );

    // A non-hermetic input masquerading as hermetic is also caught.
    let mut input = clean_proof_input();
    input.non_hermetic_input_present = true;
    input.non_hermetic_input_flagged = false;
    assert_eq!(
        resolve_reproducibility_proof_entry(input).unwrap().degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity)
    );

    let mut input = clean_proof_input();
    input.convergence_scope = M5ReproducibilityConvergenceScope::ScopeUnclassified;
    assert_eq!(
        resolve_reproducibility_proof_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::ConvergenceScopeUnclassified)
    );
}

#[test]
fn proof_form_and_surface_and_id_and_material() {
    let mut input = clean_proof_input();
    input.resolution_form_coverage = vec![M5BuildLaneResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_reproducibility_proof_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::ProofFormCoverageIncomplete)
    );

    let mut input = clean_proof_input();
    input.surface_context = M5BuildLaneSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_reproducibility_proof_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReproducibilityProofEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_proof_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_reproducibility_proof_entry(input).unwrap_err(),
        M5BuildLaneResolutionError::EmptyReproducibilityProofEntryId
    );

    let mut input = clean_proof_input();
    input.input_source_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_reproducibility_proof_entry(input).unwrap_err(),
        M5BuildLaneResolutionError::ForbiddenMaterial
    );
}

#[test]
fn proof_marked_cache_hit_and_flagged_non_hermetic_stay_clean() {
    // A remote-cache hit marked as never being proof stays honest.
    let mut input = clean_proof_input();
    input.remote_cache_hit_present = true;
    input.remote_cache_hit_marked_not_proof = true;
    assert!(resolve_reproducibility_proof_entry(input)
        .unwrap()
        .is_clean());

    // A non-hermetic input flagged rather than masquerading stays honest.
    let mut input = clean_proof_input();
    input.non_hermetic_input_present = true;
    input.non_hermetic_input_flagged = true;
    assert!(resolve_reproducibility_proof_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.vocabulary_set.cache_posture_kinds.pop();
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BUILD_LANE_DESCRIPTOR_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REPRODUCIBILITY_PROOF_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5BuildLaneAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5BuildLaneExportField::CachePostures);
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0]
        .reproducibility_proof_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.build_lane_descriptor_entries[0].degrade_reason = None;
    row.build_lane_descriptor_entries[0].build_lane_descriptor_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.lets_a_pr_or_contributor_lane_publish_release_artifacts = true,
            1 => row.treats_a_remote_cache_hit_as_reproducibility_proof = true,
            2 => row.hides_the_cache_credential_or_publication_boundary_before_promotion = true,
            _ => row.collapses_distinct_build_lane_input_sources_into_one_path = true,
        }
        assert!(packet.validate().contains(
            &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn build_lane_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    for row in &mut packet.registry_rows {
        row.build_lane_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5BuildLaneDescriptorEntryDegradeReason::BuildLaneDescriptorObjectIncomplete,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::BuildLaneDescriptorResolutionNotProven
    ));
}

#[test]
fn build_lane_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    // Drop every clean provenance-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.build_lane_descriptor_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "provenance_surface"));
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::BuildLaneDescriptorResolutionNotProven
    ));
}

#[test]
fn publication_boundary_not_proven_when_publish_fold_example_removed() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    for row in &mut packet.registry_rows {
        row.build_lane_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5BuildLaneDescriptorEntryDegradeReason::DescriptorLetsUntrustedLanePublishOrHidesCacheTrust,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::PublicationBoundaryPreservationNotProven
    ));
}

#[test]
fn publication_boundary_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    for row in &mut packet.registry_rows {
        row.build_lane_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5BuildLaneDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::PublicationBoundaryPreservationNotProven
    ));
}

#[test]
fn reproducibility_proof_integrity_not_proven_when_cache_hit_example_removed() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    for row in &mut packet.registry_rows {
        row.reproducibility_proof_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ReproducibilityProofEntryDegradeReason::ReproducibilityProofTreatsCacheHitAsProofOrDriftsBuildIdentity,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ReproducibilityProofIntegrityNotProven
    ));
}

#[test]
fn reproducibility_proof_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    // Drop every clean hermetic-rebuild proof so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.reproducibility_proof_entries
            .retain(|ex| !(ex.is_clean() && ex.convergence_scope == "hermetic_rebuild_inputs"));
    }
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ReproducibilityProofIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet
        .governance_review
        .untrusted_lanes_cannot_publish_release_artifacts = false;
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cache.example/artifact leaked".to_owned();
    assert!(packet.validate().contains(
        &M5BuildLaneDescriptorReproducibilityProofRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn build_lane_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries();
    let table = packet.render_build_lane_descriptor_table();
    // The clean hermetic and verified descriptors are rendered from the registry.
    assert!(table.contains("hermetic_no_cache_posture"));
    assert!(table.contains("verified_inputs_only_posture"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_build_lane_descriptor_and_reproducibility_proof_registries_export()
            .expect(
            "checked M5 build-lane-descriptor / reproducibility-proof registries export validates",
        );
    assert_eq!(
        from_disk.packet_id,
        M5_BUILD_LANE_DESCRIPTOR_REPRODUCIBILITY_PROOF_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_build_lane_descriptor_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildLaneConsumerSurface::BuildFarm)
        .unwrap();
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Beta);

    let preview =
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_reproducibility_proof_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildLaneConsumerSurface::ReleaseCenter)
        .unwrap();
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5BuildLaneDescriptorReproducibilityProofRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-build-lane-descriptor-and-reproducibility-proof-registries/build_lane_descriptor_beta_narrowed.json"
    )))
    .expect("build-lane-descriptor fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_build_lane_descriptor_beta_narrowed()
    );

    let preview: M5BuildLaneDescriptorReproducibilityProofRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-build-lane-descriptor-and-reproducibility-proof-registries/reproducibility_proof_preview_narrowed.json"
    )))
    .expect("reproducibility-proof fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_build_lane_descriptor_and_reproducibility_proof_registries_reproducibility_proof_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_all_four_build_lanes() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5BuildLaneFamily::ContributorPr,
            M5BuildLaneFamily::ProtectedMerge,
            M5BuildLaneFamily::Release,
            M5BuildLaneFamily::EmergencyHotfix,
        ]
    );
}
