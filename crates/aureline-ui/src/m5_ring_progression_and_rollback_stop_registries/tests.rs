use super::*;

fn clean_descriptor_input() -> M5RingProgressionEntryResolutionInput {
    M5RingProgressionEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        transition_binding_id: "launch.ring.core-team-canary".to_owned(),
        token_name: "ring.progression.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        ring_widening_transition: M5RingWideningTransitionKind::CanaryWidening,
        surface_context: M5RingSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5RingResolutionForm::ALL.to_vec(),
        entry_evidence_minimum: "repo.rows.core-team-canary-archetypes".to_owned(),
        soak_window_expectation: "bundle.ids.canary-0007".to_owned(),
        widening_allow_rationale: "install.topology.internal-dogfood-ring".to_owned(),
        issue_template_ref: "toolchain.envelope.pinned-canary".to_owned(),
        known_limits: "known-limits.published.canary".to_owned(),
        claim_narrowing_action: "rollback.target.canary-previous-stable".to_owned(),
        rollback_stop_reference: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        stop_and_rollback_visible_before_widening: true,
        is_public_facing_ring: false,
        support_language_matches_ring_proof: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5RollbackStopEntryResolutionInput {
    M5RollbackStopEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        stop_condition_ref: "launch.ring.core-team-canary".to_owned(),
        token_name: "rollback.stop.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        rollback_stop_condition: M5RollbackStopConditionKind::CrashDataLossOrTrustDefect,
        surface_context: M5RingSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5RingResolutionForm::ALL.to_vec(),
        resolved_transition_identity: "transition-id.core-team-canary-0007".to_owned(),
        active_stop_condition_ledger: "known-limits.ledger.canary".to_owned(),
        rollback_stop_target_reference: "rollback.target.ref.canary".to_owned(),
        protected_metric_regression_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        packet_freshness_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        crash_data_loss_or_trust_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_ring_transition_revision: "widening.revision.0007".to_owned(),
        keeps_rollback_stop_visible: true,
        stop_state_is_truthful: true,
        stop_condition_active: false,
        ring_progression_halted_when_stop_active: false,
        protected_metric_regression_present: false,
        protected_metric_regression_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_ring_progression_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.ring_progression_resolves_across_transitions);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.ring_progression_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.ring_widening_transition_is_classified);
    assert!(resolved.stop_and_rollback_visible_before_widening);
    assert_eq!(resolved.semantic_role, "cohort_membership");
    assert_eq!(resolved.ring_widening_transition, "canary_widening");
    assert_eq!(
        resolved.canonical_ring_widening_transition_mode,
        "canary_widening_transition"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(resolved.next_action, M5RingNextAction::ExpandRingMeaning);
}

#[test]
fn ring_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingProgressionNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.ring_widening_transition = M5RingWideningTransitionKind::TransitionUnclassified;
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingWideningTransitionUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.soak_window_expectation = "  ".to_owned();
    let resolved = resolve_ring_progression_entry(input).unwrap();
    assert!(!resolved.ring_progression_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingProgressionObjectIncomplete)
    );

    // A cohort widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_descriptor_input();
    input.stop_and_rollback_visible_before_widening = false;
    assert_eq!(
        resolve_ring_progression_entry(input).unwrap().degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5RingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing cohort running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_descriptor_input();
    input.ring_widening_transition = M5RingWideningTransitionKind::PublicPreviewWidening;
    input.is_public_facing_ring = true;
    input.support_language_matches_ring_proof = false;
    assert_eq!(
        resolve_ring_progression_entry(input).unwrap().degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5RingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_ring_progression_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RingProgressionEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_ring_progression_entry(input).unwrap_err(),
        M5RingResolutionError::EmptyRingProgressionEntryId
    );

    let mut input = clean_descriptor_input();
    input.claim_narrowing_action = "see https://cohort.internal/leak".to_owned();
    assert_eq!(
        resolve_ring_progression_entry(input).unwrap_err(),
        M5RingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cohort_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(ring_states_stop_and_rollback_before_widening(
        M5RingWideningTransitionKind::CanaryWidening,
        true,
        false,
        true
    ));
    assert!(!ring_states_stop_and_rollback_before_widening(
        M5RingWideningTransitionKind::CanaryWidening,
        false,
        false,
        true
    ));
    assert!(ring_states_stop_and_rollback_before_widening(
        M5RingWideningTransitionKind::PublicPreviewWidening,
        true,
        true,
        true
    ));
    assert!(!ring_states_stop_and_rollback_before_widening(
        M5RingWideningTransitionKind::PublicPreviewWidening,
        true,
        true,
        false
    ));
    assert!(!ring_states_stop_and_rollback_before_widening(
        M5RingWideningTransitionKind::TransitionUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn ring_progression_object_is_complete_requires_all_fields() {
    assert!(ring_progression_object_is_complete(
        M5RingWideningTransitionKind::CanaryWidening,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!ring_progression_object_is_complete(
        M5RingWideningTransitionKind::CanaryWidening,
        "repo.rows.core-team-canary-archetypes",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!ring_progression_object_is_complete(
        M5RingWideningTransitionKind::TransitionUnclassified,
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
    let resolved = resolve_rollback_stop_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.rollback_stop_safe_on_every_transition);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_rollback_stop_record);
    assert!(resolved.rollback_stop_stays_honest);
    assert_eq!(
        resolved.rollback_stop_condition,
        "crash_data_loss_or_trust_defect"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn evidence_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to cohort proof runs support ahead of proof.
    let mut input = clean_evidence_input();
    input.stop_condition_active = true;
    input.ring_progression_halted_when_stop_active = false;
    let resolved = resolve_rollback_stop_entry(input).unwrap();
    assert!(!resolved.provides_complete_rollback_stop_record);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence)
    );

    // A packet that hides the cohort evidence is also caught.
    let mut input = clean_evidence_input();
    input.keeps_rollback_stop_visible = false;
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap().degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_evidence_input();
    input.protected_metric_regression_present = true;
    input.protected_metric_regression_flagged = false;
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap().degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence)
    );

    let mut input = clean_evidence_input();
    input.rollback_stop_condition = M5RollbackStopConditionKind::ConditionUnclassified;
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap().degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::RollbackStopConditionUnclassified)
    );
}

#[test]
fn evidence_form_and_surface_and_id_and_material() {
    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5RingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap().degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::RollbackStopFormCoverageIncomplete)
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5RingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap().degrade_reason,
        Some(M5RollbackStopEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap_err(),
        M5RingResolutionError::EmptyRollbackStopEntryId
    );

    let mut input = clean_evidence_input();
    input.active_stop_condition_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_rollback_stop_entry(input).unwrap_err(),
        M5RingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to cohort proof stays honest.
    let mut input = clean_evidence_input();
    input.stop_condition_active = true;
    input.ring_progression_halted_when_stop_active = true;
    assert!(resolve_rollback_stop_entry(input).unwrap().is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_evidence_input();
    input.protected_metric_regression_present = true;
    input.protected_metric_regression_flagged = true;
    assert!(resolve_rollback_stop_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_ring_progression_and_rollback_stop_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.vocabulary_set.ring_widening_transition_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RING_PROGRESSION_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_ROLLBACK_STOP_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RingAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5RingExportField::RingWideningTransitions);
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0].rollback_stop_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.ring_progression_entries[0].degrade_reason = None;
    row.ring_progression_entries[0].ring_progression_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.advances_a_ring_without_current_known_limits_and_rollback_stop_evidence = true,
            1 => row.runs_partner_or_public_support_language_ahead_of_ring_proof = true,
            2 => row.hides_the_known_limits_or_rollback_stop_posture_before_widening = true,
            _ => row.collapses_distinct_rollback_stop_condition_classes_into_one_lane = true,
        }
        assert!(packet
            .validate()
            .contains(&M5RingProgressionRollbackStopRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn cohort_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    for row in &mut packet.registry_rows {
        row.ring_progression_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RingProgressionEntryDegradeReason::RingProgressionObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RingProgressionResolutionNotProven
    ));
}

#[test]
fn cohort_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    // Drop every clean executive-steering-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.ring_progression_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RingProgressionResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    for row in &mut packet.registry_rows {
        row.ring_progression_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5RingProgressionEntryDegradeReason::RingAdvancesWithoutRollbackStopOrRunsSupportAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RollbackStopVisibilityNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    for row in &mut packet.registry_rows {
        row.ring_progression_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RingProgressionEntryDegradeReason::RingProgressionNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RollbackStopVisibilityNotProven
    ));
}

#[test]
fn rollback_stop_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    for row in &mut packet.registry_rows {
        row.rollback_stop_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5RollbackStopEntryDegradeReason::RollbackStopAdvancesRingWhileActiveOrDropsStopEvidence,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RollbackStopIntegrityNotProven
    ));
}

#[test]
fn rollback_stop_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    // Drop every clean go-no-go-signoff evidence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.rollback_stop_entries.retain(|ex| {
            !(ex.is_clean() && ex.rollback_stop_condition == "stale_readiness_packet")
        });
    }
    assert!(packet.validate().contains(
        &M5RingProgressionRollbackStopRegistriesViolation::RollbackStopIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet
        .governance_review
        .rings_cannot_advance_without_rollback_stop_and_known_limits = false;
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RingProgressionRollbackStopRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_ring_progression_and_rollback_stop_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn cohort_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_ring_progression_and_rollback_stop_registries();
    let table = packet.render_ring_progression_table();
    // The clean canary and migration descriptors are rendered from the registry.
    assert!(table.contains("canary_widening_transition"));
    assert!(table.contains("broad_internal_dogfood_widening_transition"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ring_progression_and_rollback_stop_registries_export()
        .expect(
            "checked M5 cohort-descriptor / cohort-evidence-packet registries export validates",
        );
    assert_eq!(
        from_disk.packet_id,
        M5_RING_PROGRESSION_ROLLBACK_STOP_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_ring_progression_and_rollback_stop_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .unwrap();
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let preview =
        seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed();
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
    let beta: M5RingProgressionRollbackStopRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-ring-progression-and-rollback-stop-registries/ring_progression_beta_narrowed.json"
    )))
    .expect("cohort-descriptor fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_ring_progression_and_rollback_stop_registries_ring_progression_beta_narrowed()
    );

    let preview: M5RingProgressionRollbackStopRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-ring-progression-and-rollback-stop-registries/rollback_stop_preview_narrowed.json"
    )))
    .expect("cohort-evidence fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_ring_progression_and_rollback_stop_registries_rollback_stop_preview_narrowed()
    );
}

#[test]
fn implemented_ring_stages_is_all_five_widening_stages() {
    assert_eq!(
        IMPLEMENTED_RING_STAGES,
        [
            M5LaunchControlWideningStage::Alpha,
            M5LaunchControlWideningStage::Beta,
            M5LaunchControlWideningStage::ReleaseCandidate,
            M5LaunchControlWideningStage::Stable,
            M5LaunchControlWideningStage::LongTermSupport,
        ]
    );
}
