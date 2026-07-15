use super::*;

fn clean_descriptor_input() -> M5CohortDescriptorEntryResolutionInput {
    M5CohortDescriptorEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        cohort_binding_id: "launch.cohort.core-team-canary".to_owned(),
        token_name: "cohort.descriptor.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        cohort_archetype: M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        surface_context: M5CohortSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5CohortResolutionForm::ALL.to_vec(),
        exact_repo_archetype_rows: "repo.rows.core-team-canary-archetypes".to_owned(),
        bundle_ids: "bundle.ids.canary-0007".to_owned(),
        install_topology: "install.topology.internal-dogfood-ring".to_owned(),
        toolchain_envelope: "toolchain.envelope.pinned-canary".to_owned(),
        known_limits: "known-limits.published.canary".to_owned(),
        rollback_target: "rollback.target.canary-previous-stable".to_owned(),
        diagnostics_posture: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_cohort: false,
        support_language_matches_cohort_proof: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5CohortEvidencePacketEntryResolutionInput {
    M5CohortEvidencePacketEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        evidence_ref: "launch.cohort.core-team-canary".to_owned(),
        token_name: "cohort.evidence.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        evidence_scope: M5CohortEvidenceScope::DogfoodRingEvidence,
        surface_context: M5CohortSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5CohortResolutionForm::ALL.to_vec(),
        resolved_cohort_identity: "cohort-id.core-team-canary-0007".to_owned(),
        known_limits_ledger: "known-limits.ledger.canary".to_owned(),
        rollback_target_reference: "rollback.target.ref.canary".to_owned(),
        rehearsal_currency_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        readiness_signoff_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        support_language_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_widening_revision: "widening.revision.0007".to_owned(),
        keeps_cohort_evidence_visible: true,
        evidence_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_cohort_descriptor_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.descriptor_resolves_across_cohorts);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.cohort_descriptor_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.cohort_archetype_is_classified);
    assert!(resolved.rollback_and_diagnostics_bounded);
    assert_eq!(resolved.semantic_role, "cohort_membership");
    assert_eq!(resolved.cohort_archetype, "dogfood_core_team_canary");
    assert_eq!(
        resolved.canonical_cohort_archetype_mode,
        "dogfood_core_team_canary_archetype"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5CohortNextAction::ExpandCohortMeaning
    );
}

#[test]
fn descriptor_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.cohort_archetype = M5CohortArchetypeKind::ArchetypeUnclassified;
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::CohortArchetypeUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.bundle_ids = "  ".to_owned();
    let resolved = resolve_cohort_descriptor_entry(input).unwrap();
    assert!(!resolved.cohort_descriptor_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
    );

    // A cohort widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_descriptor_input();
    input.rollback_and_diagnostics_bounded = false;
    assert_eq!(
        resolve_cohort_descriptor_entry(input).unwrap().degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5CohortResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing cohort running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_descriptor_input();
    input.cohort_archetype = M5CohortArchetypeKind::PublicPreview;
    input.is_public_facing_cohort = true;
    input.support_language_matches_cohort_proof = false;
    assert_eq!(
        resolve_cohort_descriptor_entry(input).unwrap().degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5CohortSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_cohort_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortDescriptorEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_cohort_descriptor_entry(input).unwrap_err(),
        M5CohortResolutionError::EmptyCohortDescriptorEntryId
    );

    let mut input = clean_descriptor_input();
    input.rollback_target = "see https://cohort.internal/leak".to_owned();
    assert_eq!(
        resolve_cohort_descriptor_entry(input).unwrap_err(),
        M5CohortResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cohort_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(cohort_preserves_rollback_and_diagnostics_before_widening(
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        true,
        false,
        true
    ));
    assert!(!cohort_preserves_rollback_and_diagnostics_before_widening(
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        false,
        false,
        true
    ));
    assert!(cohort_preserves_rollback_and_diagnostics_before_widening(
        M5CohortArchetypeKind::PublicPreview,
        true,
        true,
        true
    ));
    assert!(!cohort_preserves_rollback_and_diagnostics_before_widening(
        M5CohortArchetypeKind::PublicPreview,
        true,
        true,
        false
    ));
    assert!(!cohort_preserves_rollback_and_diagnostics_before_widening(
        M5CohortArchetypeKind::ArchetypeUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn cohort_descriptor_object_is_complete_requires_all_fields() {
    assert!(cohort_descriptor_object_is_complete(
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!cohort_descriptor_object_is_complete(
        M5CohortArchetypeKind::DogfoodCoreTeamCanary,
        "repo.rows.core-team-canary-archetypes",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!cohort_descriptor_object_is_complete(
        M5CohortArchetypeKind::ArchetypeUnclassified,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
}

#[test]
fn evidence_clean_stays_honest() {
    let resolved = resolve_cohort_evidence_packet_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.evidence_safe_on_every_cohort);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_cohort_evidence);
    assert!(resolved.cohort_evidence_stays_honest);
    assert_eq!(resolved.evidence_scope, "dogfood_ring_evidence");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn evidence_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to cohort proof runs support ahead of proof.
    let mut input = clean_evidence_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = false;
    let resolved = resolve_cohort_evidence_packet_entry(input).unwrap();
    assert!(!resolved.provides_complete_cohort_evidence);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A packet that hides the cohort evidence is also caught.
    let mut input = clean_evidence_input();
    input.keeps_cohort_evidence_visible = false;
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input).unwrap().degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_evidence_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = false;
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input).unwrap().degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    let mut input = clean_evidence_input();
    input.evidence_scope = M5CohortEvidenceScope::ScopeUnclassified;
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceScopeUnclassified)
    );
}

#[test]
fn evidence_form_and_surface_and_id_and_material() {
    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5CohortResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::EvidenceFormCoverageIncomplete)
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5CohortSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5CohortEvidencePacketEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input).unwrap_err(),
        M5CohortResolutionError::EmptyCohortEvidencePacketEntryId
    );

    let mut input = clean_evidence_input();
    input.known_limits_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_cohort_evidence_packet_entry(input).unwrap_err(),
        M5CohortResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to cohort proof stays honest.
    let mut input = clean_evidence_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = true;
    assert!(resolve_cohort_evidence_packet_entry(input)
        .unwrap()
        .is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_evidence_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = true;
    assert!(resolve_cohort_evidence_packet_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_cohort_descriptor_and_evidence_packet_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.vocabulary_set.cohort_archetype_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_COHORT_DESCRIPTOR_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_COHORT_EVIDENCE_PACKET_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5CohortAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5CohortExportField::CohortArchetypes);
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0]
        .cohort_evidence_packet_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.cohort_descriptor_entries[0].degrade_reason = None;
    row.cohort_descriptor_entries[0].cohort_descriptor_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.widens_a_cohort_without_current_rollback_and_diagnostics_evidence = true,
            1 => row.runs_partner_or_public_support_language_ahead_of_cohort_proof = true,
            2 => row.hides_the_rollback_target_or_diagnostics_posture_before_widening = true,
            _ => row.collapses_distinct_cohort_evidence_classes_into_one_lane = true,
        }
        assert!(packet
            .validate()
            .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn cohort_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    for row in &mut packet.registry_rows {
        row.cohort_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CohortDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn cohort_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    // Drop every clean executive-steering-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.cohort_descriptor_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    for row in &mut packet.registry_rows {
        row.cohort_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CohortDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    for row in &mut packet.registry_rows {
        row.cohort_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5CohortDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn cohort_evidence_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    for row in &mut packet.registry_rows {
        row.cohort_evidence_packet_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CohortEvidencePacketEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::CohortEvidenceIntegrityNotProven
    ));
}

#[test]
fn cohort_evidence_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    // Drop every clean go-no-go-signoff evidence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.cohort_evidence_packet_entries
            .retain(|ex| !(ex.is_clean() && ex.evidence_scope == "go_no_go_signoff_evidence"));
    }
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::CohortEvidenceIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet
        .governance_review
        .cohorts_cannot_widen_without_rollback_and_diagnostics = false;
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5CohortDescriptorEvidencePacketRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CohortDescriptorEvidencePacketRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_cohort_descriptor_and_evidence_packet_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn cohort_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_cohort_descriptor_and_evidence_packet_registries();
    let table = packet.render_cohort_descriptor_table();
    // The clean canary and migration descriptors are rendered from the registry.
    assert!(table.contains("dogfood_core_team_canary_archetype"));
    assert!(table.contains("migration_alpha_archetype"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_cohort_descriptor_and_evidence_packet_registries_export()
        .expect(
            "checked M5 cohort-descriptor / cohort-evidence-packet registries export validates",
        );
    assert_eq!(
        from_disk.packet_id,
        M5_COHORT_DESCRIPTOR_EVIDENCE_PACKET_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_cohort_descriptor_and_evidence_packet_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_descriptor_beta_narrowed(
        );
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .unwrap();
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let preview =
        seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_evidence_preview_narrowed(
        );
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::ReleaseCenter)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5LaunchControlQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5CohortDescriptorEvidencePacketRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-cohort-descriptor-and-evidence-packet-registries/cohort_descriptor_beta_narrowed.json"
    )))
    .expect("cohort-descriptor fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_descriptor_beta_narrowed(
        )
    );

    let preview: M5CohortDescriptorEvidencePacketRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-cohort-descriptor-and-evidence-packet-registries/cohort_evidence_preview_narrowed.json"
    )))
    .expect("cohort-evidence fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_cohort_descriptor_and_evidence_packet_registries_cohort_evidence_preview_narrowed(
        )
    );
}

#[test]
fn implemented_cohorts_is_all_five_launch_bearing_cohorts() {
    assert_eq!(
        IMPLEMENTED_COHORTS,
        [
            M5LaunchControlCohort::CoreTeamCanary,
            M5LaunchControlCohort::DesignPartnerPreview,
            M5LaunchControlCohort::ExtensionAuthor,
            M5LaunchControlCohort::PublicPreview,
            M5LaunchControlCohort::CertifiedArchetype,
        ]
    );
}
