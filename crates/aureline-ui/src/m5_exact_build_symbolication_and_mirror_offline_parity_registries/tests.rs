use super::*;

fn clean_manifest_input() -> M5ExactBuildSymbolicationEntryResolutionInput {
    M5ExactBuildSymbolicationEntryResolutionInput {
        entry_id: "manifest:test".to_owned(),
        lane_binding_id: "release.lane.release".to_owned(),
        token_name: "verified.input.manifest.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        rebuild_source: M5SymbolicationFidelityKind::HermeticCleanRoomRebuild,
        surface_context: M5ExactBuildSupportSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5ExactBuildSupportResolutionForm::ALL.to_vec(),
        rebuild_config_digest: "build-config.sha256.release-0007".to_owned(),
        replay_receipt: "receipt.materialized.release-0007".to_owned(),
        protected_input_ledger: "provenance.ledger.release-0007".to_owned(),
        rebuild_authority: "verification.release-signing-scoped".to_owned(),
        expected_artifact_families: "artifacts.binaries-packages-sboms".to_owned(),
        hermetic_rebuild_posture: "hermetic.fully-hermetic".to_owned(),
        shared_cache_isolation_rule: "rematerialize.full-rebuild-required".to_owned(),
        bound_to_registry: true,
        rebuild_authority_bounded: true,
        is_replay_trust_risk_source: false,
        cache_trust_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_sidecar_input() -> M5MirrorOfflineParityEntryResolutionInput {
    M5MirrorOfflineParityEntryResolutionInput {
        entry_id: "sidecar:test".to_owned(),
        diff_packet_ref: "release.lane.release".to_owned(),
        token_name: "sidecar.completeness.manifest.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        diff_scope: M5MirrorOfflineParityScope::ByteIdenticalDiff,
        surface_context: M5ExactBuildSupportSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5ExactBuildSupportResolutionForm::ALL.to_vec(),
        resolved_build_identity: "build-id.sha256.release-0007".to_owned(),
        compared_artifact_families: "families.binaries-packages-docs-schemas-sboms-symbols"
            .to_owned(),
        deterministic_diff_ledger: "ledger.all-families-present-release-0007".to_owned(),
        candidate_vs_rebuild_check: "binding.pinned-to-build-id-release-0007".to_owned(),
        divergence_or_missing_reference: "missing-or-mismatched.none".to_owned(),
        attestation_state: "attestation.signed-and-verified".to_owned(),
        last_diff_revision: "convergence.revision.0007".to_owned(),
        keeps_diff_ledger_visible: true,
        packet_is_truthful: true,
        omitted_family_present: false,
        omitted_family_flagged: false,
        material_divergence_present: false,
        material_divergence_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SYMBOLICATION_MIRROR_PARITY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn manifest_clean_names_meaning_and_is_bound() {
    let resolved = resolve_exact_build_symbolication_entry(clean_manifest_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.lane_resolves_across_lanes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.exact_build_symbolication_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.rebuild_source_is_classified);
    assert!(resolved.rebuild_authority_bounded);
    assert_eq!(resolved.semantic_role, "reproducibility_proof");
    assert_eq!(resolved.rebuild_source, "hermetic_clean_room_rebuild");
    assert_eq!(
        resolved.canonical_rebuild_source_mode,
        "hermetic_clean_room_rebuild_mode"
    );
    assert_eq!(resolved.surface_context, "release_center_surface");
    assert_eq!(
        resolved.next_action,
        M5ExactBuildSupportNextAction::ExpandRebuildMeaning
    );
}

#[test]
fn registry_token_unstated_degrades() {
    let mut input = clean_manifest_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::RegistryTokenUnstated)
    );
}

#[test]
fn manifest_unbound_and_unclassified_degrade() {
    let mut input = clean_manifest_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::LaneNotBoundToRegistry)
    );

    let mut input = clean_manifest_input();
    input.rebuild_source = M5SymbolicationFidelityKind::SourceUnclassified;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::RebuildSourceUnclassified)
    );
}

#[test]
fn manifest_object_incomplete_and_admit_fold_and_form_degrade() {
    // An unstated materialized-input receipt leaves the resolved object incomplete.
    let mut input = clean_manifest_input();
    input.replay_receipt = "  ".to_owned();
    let resolved = resolve_exact_build_symbolication_entry(input).unwrap();
    assert!(!resolved.exact_build_symbolication_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::ExactBuildSymbolicationObjectIncomplete)
    );

    // A lane claiming protected-lane admission it must not have degrades with the structured blocker reason.
    let mut input = clean_manifest_input();
    input.rebuild_authority_bounded = false;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt)
    );

    let mut input = clean_manifest_input();
    input.resolution_form_coverage = vec![M5ExactBuildSupportResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn manifest_trust_risk_and_surface_and_proof_degrade() {
    // A trust-risk input source hiding its input-trust marker first fails the admission fold.
    let mut input = clean_manifest_input();
    input.rebuild_source = M5SymbolicationFidelityKind::SharedCacheShortcut;
    input.is_replay_trust_risk_source = true;
    input.cache_trust_disclosed = false;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt)
    );

    let mut input = clean_manifest_input();
    input.surface_context = M5ExactBuildSupportSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_manifest_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_exact_build_symbolication_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ExactBuildSymbolicationEntryDegradeReason::ProofStale)
    );
}

#[test]
fn manifest_empty_id_and_forbidden_material_error() {
    let mut input = clean_manifest_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_exact_build_symbolication_entry(input).unwrap_err(),
        M5ExactBuildSupportResolutionError::EmptyExactBuildSymbolicationEntryId
    );

    let mut input = clean_manifest_input();
    input.rebuild_authority = "see https://cache.internal/leak".to_owned();
    assert_eq!(
        resolve_exact_build_symbolication_entry(input).unwrap_err(),
        M5ExactBuildSupportResolutionError::ForbiddenMaterial
    );
}

#[test]
fn shared_cache_cannot_authorize_rebuild_rejects_unbounded() {
    assert!(shared_cache_cannot_authorize_rebuild(
        M5SymbolicationFidelityKind::HermeticCleanRoomRebuild,
        true,
        false,
        true
    ));
    assert!(!shared_cache_cannot_authorize_rebuild(
        M5SymbolicationFidelityKind::HermeticCleanRoomRebuild,
        false,
        false,
        true
    ));
    assert!(shared_cache_cannot_authorize_rebuild(
        M5SymbolicationFidelityKind::SharedCacheShortcut,
        true,
        true,
        true
    ));
    assert!(!shared_cache_cannot_authorize_rebuild(
        M5SymbolicationFidelityKind::SharedCacheShortcut,
        true,
        true,
        false
    ));
    assert!(!shared_cache_cannot_authorize_rebuild(
        M5SymbolicationFidelityKind::SourceUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn exact_build_symbolication_object_is_complete_requires_all_fields() {
    assert!(exact_build_symbolication_object_is_complete(
        M5SymbolicationFidelityKind::HermeticCleanRoomRebuild,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ));
    assert!(!exact_build_symbolication_object_is_complete(
        M5SymbolicationFidelityKind::HermeticCleanRoomRebuild,
        "build-config.sha256.release-0007",
        "  ",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ));
    assert!(!exact_build_symbolication_object_is_complete(
        M5SymbolicationFidelityKind::SourceUnclassified,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ));
}

#[test]
fn sidecar_clean_stays_converged() {
    let resolved = resolve_mirror_offline_parity_entry(clean_sidecar_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.packet_safe_on_every_lane);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_artifact_diff);
    assert!(resolved.artifact_diff_stays_deterministic);
    assert_eq!(resolved.diff_scope, "byte_identical_diff");
    assert_eq!(resolved.surface_context, "release_center_surface");
}

#[test]
fn sidecar_missing_family_and_unclassified_degrade() {
    // A missing family that is not flagged lets a green build omit a claimed sidecar.
    let mut input = clean_sidecar_input();
    input.omitted_family_present = true;
    input.omitted_family_flagged = false;
    let resolved = resolve_mirror_offline_parity_entry(input).unwrap();
    assert!(!resolved.provides_complete_artifact_diff);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
    );

    // A manifest that hides the family ledger is also caught.
    let mut input = clean_sidecar_input();
    input.keeps_diff_ledger_visible = false;
    assert_eq!(
        resolve_mirror_offline_parity_entry(input).unwrap().degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
    );

    // A mismatched-identity sidecar treated as warning-only is also caught.
    let mut input = clean_sidecar_input();
    input.material_divergence_present = true;
    input.material_divergence_flagged = false;
    assert_eq!(
        resolve_mirror_offline_parity_entry(input).unwrap().degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
    );

    let mut input = clean_sidecar_input();
    input.diff_scope = M5MirrorOfflineParityScope::ScopeUnclassified;
    assert_eq!(
        resolve_mirror_offline_parity_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::DiffScopeUnclassified)
    );
}

#[test]
fn sidecar_form_and_surface_and_id_and_material() {
    let mut input = clean_sidecar_input();
    input.resolution_form_coverage = vec![M5ExactBuildSupportResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_mirror_offline_parity_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::PacketFormCoverageIncomplete)
    );

    let mut input = clean_sidecar_input();
    input.surface_context = M5ExactBuildSupportSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_mirror_offline_parity_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5MirrorOfflineParityEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_sidecar_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_mirror_offline_parity_entry(input).unwrap_err(),
        M5ExactBuildSupportResolutionError::EmptyMirrorOfflineParityEntryId
    );

    let mut input = clean_sidecar_input();
    input.deterministic_diff_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_mirror_offline_parity_entry(input).unwrap_err(),
        M5ExactBuildSupportResolutionError::ForbiddenMaterial
    );
}

#[test]
fn sidecar_flagged_missing_and_mismatched_stay_clean() {
    // A missing family flagged as a blocker stays converged.
    let mut input = clean_sidecar_input();
    input.omitted_family_present = true;
    input.omitted_family_flagged = true;
    assert!(resolve_mirror_offline_parity_entry(input)
        .unwrap()
        .is_clean());

    // A mismatched-identity sidecar flagged as a blocker stays converged.
    let mut input = clean_sidecar_input();
    input.material_divergence_present = true;
    input.material_divergence_flagged = true;
    assert!(resolve_mirror_offline_parity_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.vocabulary_set.rebuild_source_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EXACT_BUILD_SYMBOLICATION_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_MIRROR_OFFLINE_PARITY_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ExactBuildSupportAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ExactBuildSupportExportField::RebuildSourceKinds);
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0]
        .mirror_offline_parity_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    // Force a clean manifest entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.exact_build_symbolication_entries[0].degrade_reason = None;
    row.exact_build_symbolication_entries[0].exact_build_symbolication_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.overclaims_clean_room_parity_on_a_partial_artifact_family_rebuild = true,
            1 => row.lets_a_clean_room_rebuild_rely_on_a_shared_remote_cache_as_authority = true,
            2 => row.treats_a_material_artifact_diff_divergence_as_warning_only = true,
            _ => row.publishes_rc_or_stable_when_clean_room_parity_is_stale_or_incomplete = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SymbolicationMirrorParityRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn exact_build_symbolication_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    for row in &mut packet.registry_rows {
        row.exact_build_symbolication_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ExactBuildSymbolicationEntryDegradeReason::ExactBuildSymbolicationObjectIncomplete,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::ExactBuildSymbolicationResolutionNotProven
    ));
}

#[test]
fn exact_build_symbolication_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    // Drop every clean provenance-surface manifest so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.exact_build_symbolication_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "provenance_surface"));
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::ExactBuildSymbolicationResolutionNotProven
    ));
}

#[test]
fn input_verification_boundary_not_proven_when_admit_fold_example_removed() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    for row in &mut packet.registry_rows {
        row.exact_build_symbolication_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ExactBuildSymbolicationEntryDegradeReason::LaneReliesOnSharedCacheOrHidesReplayReceipt)
        });
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::SharedCacheAuthorityBoundaryNotProven
    ));
}

#[test]
fn input_verification_boundary_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    for row in &mut packet.registry_rows {
        row.exact_build_symbolication_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ExactBuildSymbolicationEntryDegradeReason::LaneNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::SharedCacheAuthorityBoundaryNotProven
    ));
}

#[test]
fn artifact_diff_determinism_not_proven_when_missing_example_removed() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    for row in &mut packet.registry_rows {
        row.mirror_offline_parity_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5MirrorOfflineParityEntryDegradeReason::ArtifactDiffDivergesOrOmitsFamilyOrDriftsBuildIdentity)
        });
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::ArtifactDiffDeterminismNotProven
    ));
}

#[test]
fn artifact_diff_determinism_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    // Drop every clean hermetic-rebuild sidecar so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.mirror_offline_parity_entries
            .retain(|ex| !(ex.is_clean() && ex.diff_scope == "hermetic_rebuild_diff"));
    }
    assert!(packet.validate().contains(
        &M5SymbolicationMirrorParityRegistriesViolation::ArtifactDiffDeterminismNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet
        .governance_review
        .shared_cache_cannot_authorize_rebuild_lanes = false;
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cache.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SymbolicationMirrorParityRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries()
        .export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn exact_build_symbolication_table_lists_only_clean_lanes() {
    let packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
    let table = packet.render_exact_build_symbolication_table();
    // The clean rematerialized and verified-cache manifests are rendered from the registry.
    assert!(table.contains("hermetic_clean_room_rebuild_mode"));
    assert!(table.contains("rematerialized_input_replay_mode"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_exact_build_symbolication_and_mirror_offline_parity_registries_export()
            .expect(
            "checked M5 clean-room-rebuild-lane / artifact-diff-packet registries export validates",
        );
    assert_eq!(
        from_disk.packet_id,
        M5_SYMBOLICATION_MIRROR_PARITY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_hermetic_rebuild_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildLaneConsumerSurface::BuildFarm)
        .unwrap();
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Beta);

    let preview =
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_artifact_diff_preview_narrowed();
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
    let beta: M5SymbolicationMirrorParityRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-exact-build-symbolication-and-mirror-offline-parity-registries/hermetic_rebuild_beta_narrowed.json"
    )))
    .expect("clean-room-rebuild fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_hermetic_rebuild_beta_narrowed()
    );

    let preview: M5SymbolicationMirrorParityRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-exact-build-symbolication-and-mirror-offline-parity-registries/artifact_diff_preview_narrowed.json"
    )))
    .expect("artifact-diff fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_artifact_diff_preview_narrowed()
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
