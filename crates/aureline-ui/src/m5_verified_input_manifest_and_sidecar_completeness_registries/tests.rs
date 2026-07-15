use super::*;

fn clean_manifest_input() -> M5VerifiedInputManifestEntryResolutionInput {
    M5VerifiedInputManifestEntryResolutionInput {
        entry_id: "manifest:test".to_owned(),
        lane_binding_id: "release.lane.release".to_owned(),
        token_name: "verified.input.manifest.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        input_source: M5VerifiedInputSourceKind::RematerializedFromSource,
        surface_context: M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5ExactBuildResolutionForm::ALL.to_vec(),
        build_config_digest: "build-config.sha256.release-0007".to_owned(),
        materialized_input_receipt: "receipt.materialized.release-0007".to_owned(),
        input_provenance_ledger: "provenance.ledger.release-0007".to_owned(),
        verification_authority: "verification.release-signing-scoped".to_owned(),
        expected_artifact_families: "artifacts.binaries-packages-sboms".to_owned(),
        hermetic_input_posture: "hermetic.fully-hermetic".to_owned(),
        re_materialization_rule: "rematerialize.full-rebuild-required".to_owned(),
        bound_to_registry: true,
        input_admission_bounded: true,
        is_trust_risk_source: false,
        input_trust_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_sidecar_input() -> M5SidecarCompletenessManifestEntryResolutionInput {
    M5SidecarCompletenessManifestEntryResolutionInput {
        entry_id: "sidecar:test".to_owned(),
        manifest_ref: "release.lane.release".to_owned(),
        token_name: "sidecar.completeness.manifest.release".to_owned(),
        semantic_role: M5BuildLaneTrustRole::ReproducibilityProof,
        convergence_scope: M5SidecarConvergenceScope::ConvergedOnBinaryIdentity,
        surface_context: M5ExactBuildSurfaceContext::ReleaseCenterSurface,
        resolution_form_coverage: M5ExactBuildResolutionForm::ALL.to_vec(),
        resolved_build_identity: "build-id.sha256.release-0007".to_owned(),
        claimed_artifact_families: "families.binaries-packages-docs-schemas-sboms-symbols"
            .to_owned(),
        sidecar_family_ledger: "ledger.all-families-present-release-0007".to_owned(),
        binding_identity_check: "binding.pinned-to-build-id-release-0007".to_owned(),
        missing_or_mismatched_reference: "missing-or-mismatched.none".to_owned(),
        attestation_state: "attestation.signed-and-verified".to_owned(),
        last_convergence_revision: "convergence.revision.0007".to_owned(),
        keeps_family_ledger_visible: true,
        manifest_is_truthful: true,
        missing_family_present: false,
        missing_family_flagged: false,
        mismatched_identity_present: false,
        mismatched_identity_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_PACKET_ID
    );
}

#[test]
fn manifest_clean_names_meaning_and_is_bound() {
    let resolved = resolve_verified_input_manifest_entry(clean_manifest_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.manifest_resolves_across_lanes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.verified_input_manifest_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.input_source_is_classified);
    assert!(resolved.input_admission_bounded);
    assert_eq!(resolved.semantic_role, "reproducibility_proof");
    assert_eq!(resolved.input_source, "rematerialized_from_source");
    assert_eq!(
        resolved.canonical_input_source_mode,
        "rematerialized_from_source_input"
    );
    assert_eq!(resolved.surface_context, "release_center_surface");
    assert_eq!(
        resolved.next_action,
        M5ExactBuildNextAction::ExpandManifestMeaning
    );
}

#[test]
fn manifest_token_unstated_degrades() {
    let mut input = clean_manifest_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ManifestTokenUnstated)
    );
}

#[test]
fn manifest_unbound_and_unclassified_degrade() {
    let mut input = clean_manifest_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ManifestNotBoundToRegistry)
    );

    let mut input = clean_manifest_input();
    input.input_source = M5VerifiedInputSourceKind::SourceUnclassified;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::InputSourceUnclassified)
    );
}

#[test]
fn manifest_object_incomplete_and_admit_fold_and_form_degrade() {
    // An unstated materialized-input receipt leaves the resolved object incomplete.
    let mut input = clean_manifest_input();
    input.materialized_input_receipt = "  ".to_owned();
    let resolved = resolve_verified_input_manifest_entry(input).unwrap();
    assert!(!resolved.verified_input_manifest_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::VerifiedInputManifestObjectIncomplete)
    );

    // A lane claiming protected-lane admission it must not have degrades with the structured blocker reason.
    let mut input = clean_manifest_input();
    input.input_admission_bounded = false;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ManifestAdmitsUnverifiedInputOrHidesDigest)
    );

    let mut input = clean_manifest_input();
    input.resolution_form_coverage = vec![M5ExactBuildResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn manifest_trust_risk_and_surface_and_proof_degrade() {
    // A trust-risk input source hiding its input-trust marker first fails the admission fold.
    let mut input = clean_manifest_input();
    input.input_source = M5VerifiedInputSourceKind::UnverifiedExternalInput;
    input.is_trust_risk_source = true;
    input.input_trust_disclosed = false;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ManifestAdmitsUnverifiedInputOrHidesDigest)
    );

    let mut input = clean_manifest_input();
    input.surface_context = M5ExactBuildSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_manifest_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_verified_input_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5VerifiedInputManifestEntryDegradeReason::ProofStale)
    );
}

#[test]
fn manifest_empty_id_and_forbidden_material_error() {
    let mut input = clean_manifest_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_verified_input_manifest_entry(input).unwrap_err(),
        M5ExactBuildResolutionError::EmptyVerifiedInputManifestEntryId
    );

    let mut input = clean_manifest_input();
    input.verification_authority = "see https://cache.internal/leak".to_owned();
    assert_eq!(
        resolve_verified_input_manifest_entry(input).unwrap_err(),
        M5ExactBuildResolutionError::ForbiddenMaterial
    );
}

#[test]
fn unverified_input_cannot_enter_protected_lane_rejects_unbounded() {
    assert!(unverified_input_cannot_enter_protected_lane(
        M5VerifiedInputSourceKind::RematerializedFromSource,
        true,
        false,
        true
    ));
    assert!(!unverified_input_cannot_enter_protected_lane(
        M5VerifiedInputSourceKind::RematerializedFromSource,
        false,
        false,
        true
    ));
    assert!(unverified_input_cannot_enter_protected_lane(
        M5VerifiedInputSourceKind::UnverifiedExternalInput,
        true,
        true,
        true
    ));
    assert!(!unverified_input_cannot_enter_protected_lane(
        M5VerifiedInputSourceKind::UnverifiedExternalInput,
        true,
        true,
        false
    ));
    assert!(!unverified_input_cannot_enter_protected_lane(
        M5VerifiedInputSourceKind::SourceUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn verified_input_manifest_object_is_complete_requires_all_fields() {
    assert!(verified_input_manifest_object_is_complete(
        M5VerifiedInputSourceKind::RematerializedFromSource,
        "build-config.sha256.release-0007",
        "receipt.materialized.release-0007",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ));
    assert!(!verified_input_manifest_object_is_complete(
        M5VerifiedInputSourceKind::RematerializedFromSource,
        "build-config.sha256.release-0007",
        "  ",
        "provenance.ledger.release-0007",
        "verification.release-signing-scoped",
        "artifacts.binaries-packages-sboms",
        "hermetic.fully-hermetic",
        "rematerialize.full-rebuild-required",
    ));
    assert!(!verified_input_manifest_object_is_complete(
        M5VerifiedInputSourceKind::SourceUnclassified,
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
    let resolved = resolve_sidecar_completeness_manifest_entry(clean_sidecar_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.manifest_safe_on_every_lane);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_sidecar_completeness);
    assert!(resolved.sidecar_family_stays_converged);
    assert_eq!(resolved.convergence_scope, "converged_on_binary_identity");
    assert_eq!(resolved.surface_context, "release_center_surface");
}

#[test]
fn sidecar_missing_family_and_unclassified_degrade() {
    // A missing family that is not flagged lets a green build omit a claimed sidecar.
    let mut input = clean_sidecar_input();
    input.missing_family_present = true;
    input.missing_family_flagged = false;
    let resolved = resolve_sidecar_completeness_manifest_entry(input).unwrap();
    assert!(!resolved.provides_complete_sidecar_completeness);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::SidecarFamilyMissingOrMismatchedOrDriftsBuildIdentity)
    );

    // A manifest that hides the family ledger is also caught.
    let mut input = clean_sidecar_input();
    input.keeps_family_ledger_visible = false;
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input).unwrap().degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::SidecarFamilyMissingOrMismatchedOrDriftsBuildIdentity)
    );

    // A mismatched-identity sidecar treated as warning-only is also caught.
    let mut input = clean_sidecar_input();
    input.mismatched_identity_present = true;
    input.mismatched_identity_flagged = false;
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input).unwrap().degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::SidecarFamilyMissingOrMismatchedOrDriftsBuildIdentity)
    );

    let mut input = clean_sidecar_input();
    input.convergence_scope = M5SidecarConvergenceScope::ScopeUnclassified;
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::ConvergenceScopeUnclassified)
    );
}

#[test]
fn sidecar_form_and_surface_and_id_and_material() {
    let mut input = clean_sidecar_input();
    input.resolution_form_coverage = vec![M5ExactBuildResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::ManifestFormCoverageIncomplete)
    );

    let mut input = clean_sidecar_input();
    input.surface_context = M5ExactBuildSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SidecarCompletenessManifestEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_sidecar_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input).unwrap_err(),
        M5ExactBuildResolutionError::EmptySidecarCompletenessManifestEntryId
    );

    let mut input = clean_sidecar_input();
    input.sidecar_family_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_sidecar_completeness_manifest_entry(input).unwrap_err(),
        M5ExactBuildResolutionError::ForbiddenMaterial
    );
}

#[test]
fn sidecar_flagged_missing_and_mismatched_stay_clean() {
    // A missing family flagged as a blocker stays converged.
    let mut input = clean_sidecar_input();
    input.missing_family_present = true;
    input.missing_family_flagged = true;
    assert!(resolve_sidecar_completeness_manifest_entry(input)
        .unwrap()
        .is_clean());

    // A mismatched-identity sidecar flagged as a blocker stays converged.
    let mut input = clean_sidecar_input();
    input.mismatched_identity_present = true;
    input.mismatched_identity_flagged = true;
    assert!(resolve_sidecar_completeness_manifest_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.vocabulary_set.input_source_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_VERIFIED_INPUT_MANIFEST_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SIDECAR_COMPLETENESS_MANIFEST_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ExactBuildAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ExactBuildExportField::InputSourceKinds);
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0]
        .sidecar_completeness_manifest_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    // Force a clean manifest entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.verified_input_manifest_entries[0].degrade_reason = None;
    row.verified_input_manifest_entries[0].verified_input_manifest_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.lets_a_green_build_omit_a_claimed_artifact_family_or_sidecar = true,
            1 => row.binds_a_claimed_sidecar_to_a_different_build_identity = true,
            2 => row.treats_a_missing_or_mismatched_sidecar_as_warning_only = true,
            _ => row.admits_an_unverified_or_non_materialized_input_into_a_protected_lane = true,
        }
        assert!(packet.validate().contains(
            &M5VerifiedInputSidecarCompletenessRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn verified_input_manifest_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    for row in &mut packet.registry_rows {
        row.verified_input_manifest_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5VerifiedInputManifestEntryDegradeReason::VerifiedInputManifestObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::VerifiedInputManifestResolutionNotProven
    ));
}

#[test]
fn verified_input_manifest_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    // Drop every clean provenance-surface manifest so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.verified_input_manifest_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "provenance_surface"));
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::VerifiedInputManifestResolutionNotProven
    ));
}

#[test]
fn input_verification_boundary_not_proven_when_admit_fold_example_removed() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    for row in &mut packet.registry_rows {
        row.verified_input_manifest_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5VerifiedInputManifestEntryDegradeReason::ManifestAdmitsUnverifiedInputOrHidesDigest)
        });
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::InputVerificationBoundaryPreservationNotProven
    ));
}

#[test]
fn input_verification_boundary_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    for row in &mut packet.registry_rows {
        row.verified_input_manifest_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5VerifiedInputManifestEntryDegradeReason::ManifestNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::InputVerificationBoundaryPreservationNotProven
    ));
}

#[test]
fn sidecar_completeness_integrity_not_proven_when_missing_example_removed() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    for row in &mut packet.registry_rows {
        row.sidecar_completeness_manifest_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SidecarCompletenessManifestEntryDegradeReason::SidecarFamilyMissingOrMismatchedOrDriftsBuildIdentity)
        });
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::SidecarCompletenessIntegrityNotProven
    ));
}

#[test]
fn sidecar_completeness_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    // Drop every clean hermetic-rebuild sidecar so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.sidecar_completeness_manifest_entries
            .retain(|ex| !(ex.is_clean() && ex.convergence_scope == "hermetic_rebuild_converged"));
    }
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::SidecarCompletenessIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet
        .governance_review
        .unverified_inputs_cannot_enter_protected_lanes = false;
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5VerifiedInputSidecarCompletenessRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cache.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5VerifiedInputSidecarCompletenessRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn verified_input_manifest_table_lists_only_clean_manifests() {
    let packet = seeded_m5_verified_input_manifest_and_sidecar_completeness_registries();
    let table = packet.render_verified_input_manifest_table();
    // The clean rematerialized and verified-cache manifests are rendered from the registry.
    assert!(table.contains("rematerialized_from_source_input"));
    assert!(table.contains("verified_cache_input_mode"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_verified_input_manifest_and_sidecar_completeness_registries_export()
            .expect(
                "checked M5 verified-input-manifest / sidecar-completeness-manifest registries export validates",
            );
    assert_eq!(
        from_disk.packet_id,
        M5_VERIFIED_INPUT_SIDECAR_COMPLETENESS_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildLaneConsumerSurface::BuildFarm)
        .unwrap();
    assert_eq!(row.qualification, M5BuildLaneQualificationClass::Beta);

    let preview =
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed();
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
    let beta: M5VerifiedInputSidecarCompletenessRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-verified-input-manifest-and-sidecar-completeness-registries/verified_input_beta_narrowed.json"
    )))
    .expect("verified-input fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_verified_input_beta_narrowed()
    );

    let preview: M5VerifiedInputSidecarCompletenessRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-verified-input-manifest-and-sidecar-completeness-registries/sidecar_completeness_preview_narrowed.json"
    )))
    .expect("sidecar-completeness fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_verified_input_manifest_and_sidecar_completeness_registries_sidecar_completeness_preview_narrowed()
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
