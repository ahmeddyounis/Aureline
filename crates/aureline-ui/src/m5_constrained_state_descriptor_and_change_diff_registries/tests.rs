use super::*;

fn clean_constrained_state_descriptor_input() -> M5ConstrainedStateDescriptorEntryResolutionInput {
    M5ConstrainedStateDescriptorEntryResolutionInput {
        entry_id: "constrained_state_descriptor:test".to_owned(),
        line_binding_id: "launch.line.core-team-canary".to_owned(),
        token_name: "line.constrained_state_descriptor.core_team_canary".to_owned(),
        semantic_role: M5ConstrainedFileStateRole::StateBadgeClassification,
        report_section: M5ConstrainedStateDescriptorKind::ReadOnlyPathObject,
        surface_context: M5ConstrainedStateDescriptorSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5ConstrainedStateDescriptorResolutionForm::ALL.to_vec(),
        exact_repo_journey_rows: "repo.rows.core-team-canary-journeys".to_owned(),
        bundle_ids: "bundle.ids.canary-0007".to_owned(),
        install_topology: "install.topology.internal-dogfood-ring".to_owned(),
        toolchain_envelope: "toolchain.envelope.pinned-canary".to_owned(),
        known_limits: "known-limits.published.canary".to_owned(),
        rollback_target: "rollback.target.canary-previous-stable".to_owned(),
        diagnostics_posture: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        rollback_and_diagnostics_bounded: true,
        is_public_facing_line: false,
        support_language_matches_line_proof: true,
        proof_fresh: true,
    }
}

fn clean_downgrade_input() -> M5DescriptorChangeDiffEntryResolutionInput {
    M5DescriptorChangeDiffEntryResolutionInput {
        entry_id: "downgrade:test".to_owned(),
        comparison_ref: "launch.line.core-team-canary".to_owned(),
        token_name: "line.downgrade.core_team_canary".to_owned(),
        semantic_role: M5ConstrainedFileStateRole::StateBadgeClassification,
        comparison_scope: M5DescriptorChangeDiffScope::StateClassChange,
        surface_context: M5ConstrainedStateDescriptorSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5ConstrainedStateDescriptorResolutionForm::ALL.to_vec(),
        resolved_line_identity: "line-id.core-team-canary-0007".to_owned(),
        known_limits_ledger: "known-limits.ledger.canary".to_owned(),
        rollback_target_reference: "rollback.target.ref.canary".to_owned(),
        rehearsal_currency_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        readiness_signoff_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        support_language_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_widening_revision: "widening.revision.0007".to_owned(),
        keeps_descriptor_change_diff_visible: true,
        comparison_is_truthful: true,
        support_language_present: false,
        support_language_bound_to_proof: false,
        known_limits_gap_present: false,
        known_limits_gap_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_CONSTRAINED_STATE_DESCRIPTOR_CHANGE_DIFF_REGISTRIES_PACKET_ID
    );
}

#[test]
fn constrained_state_descriptor_clean_names_meaning_and_is_bound() {
    let resolved =
        resolve_constrained_state_descriptor_entry(clean_constrained_state_descriptor_input())
            .unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.constrained_state_descriptor_resolves_across_lines);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.constrained_state_descriptor_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.report_section_is_classified);
    assert!(resolved.rollback_and_diagnostics_bounded);
    assert_eq!(resolved.semantic_role, "state_badge_classification");
    assert_eq!(resolved.report_section, "read_only_path_object");
    assert_eq!(
        resolved.canonical_report_section_mode,
        "read_only_path_object_mode"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5ConstrainedStateDescriptorNextAction::ExpandCohortMeaning
    );
}

#[test]
fn constrained_state_descriptor_token_unstated_degrades() {
    let mut input = clean_constrained_state_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorTokenUnstated)
    );
}

#[test]
fn constrained_state_descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_constrained_state_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry)
    );

    let mut input = clean_constrained_state_descriptor_input();
    input.report_section = M5ConstrainedStateDescriptorKind::ObjectClassUnclassified;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::CohortObjectClassUnclassified)
    );
}

#[test]
fn constrained_state_descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_constrained_state_descriptor_input();
    input.bundle_ids = "  ".to_owned();
    let resolved = resolve_constrained_state_descriptor_entry(input).unwrap();
    assert!(!resolved.constrained_state_descriptor_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
    );

    // A line widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_constrained_state_descriptor_input();
    input.rollback_and_diagnostics_bounded = false;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input).unwrap().degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_constrained_state_descriptor_input();
    input.resolution_form_coverage =
        vec![M5ConstrainedStateDescriptorResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn constrained_state_descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing line running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_constrained_state_descriptor_input();
    input.report_section = M5ConstrainedStateDescriptorKind::ProjectionObject;
    input.is_public_facing_line = true;
    input.support_language_matches_line_proof = false;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input).unwrap().degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_constrained_state_descriptor_input();
    input.surface_context = M5ConstrainedStateDescriptorSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_constrained_state_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ConstrainedStateDescriptorEntryDegradeReason::ProofStale)
    );
}

#[test]
fn constrained_state_descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_constrained_state_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input).unwrap_err(),
        M5ConstrainedStateDescriptorResolutionError::EmptyCohortDescriptorEntryId
    );

    let mut input = clean_constrained_state_descriptor_input();
    input.rollback_target = "see https://line.internal/leak".to_owned();
    assert_eq!(
        resolve_constrained_state_descriptor_entry(input).unwrap_err(),
        M5ConstrainedStateDescriptorResolutionError::ForbiddenMaterial
    );
}

#[test]
fn line_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(line_preserves_rollback_and_diagnostics_before_widening(
        M5ConstrainedStateDescriptorKind::ReadOnlyPathObject,
        true,
        false,
        true
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ConstrainedStateDescriptorKind::ReadOnlyPathObject,
        false,
        false,
        true
    ));
    assert!(line_preserves_rollback_and_diagnostics_before_widening(
        M5ConstrainedStateDescriptorKind::ProjectionObject,
        true,
        true,
        true
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ConstrainedStateDescriptorKind::ProjectionObject,
        true,
        true,
        false
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ConstrainedStateDescriptorKind::ObjectClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn constrained_state_descriptor_object_is_complete_requires_all_fields() {
    assert!(constrained_state_descriptor_object_is_complete(
        M5ConstrainedStateDescriptorKind::ReadOnlyPathObject,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!constrained_state_descriptor_object_is_complete(
        M5ConstrainedStateDescriptorKind::ReadOnlyPathObject,
        "repo.rows.core-team-canary-journeys",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!constrained_state_descriptor_object_is_complete(
        M5ConstrainedStateDescriptorKind::ObjectClassUnclassified,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
}

#[test]
fn downgrade_clean_stays_honest() {
    let resolved = resolve_descriptor_change_diff_entry(clean_downgrade_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.comparison_safe_on_every_line);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_descriptor_change_diff);
    assert!(resolved.descriptor_change_diff_stays_honest);
    assert_eq!(resolved.comparison_scope, "state_class_change");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn downgrade_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to line proof runs support ahead of proof.
    let mut input = clean_downgrade_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = false;
    let resolved = resolve_descriptor_change_diff_entry(input).unwrap();
    assert!(!resolved.provides_complete_descriptor_change_diff);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A packet that hides the line downgrade is also caught.
    let mut input = clean_downgrade_input();
    input.keeps_descriptor_change_diff_visible = false;
    assert_eq!(
        resolve_descriptor_change_diff_entry(input).unwrap().degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_downgrade_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = false;
    assert_eq!(
        resolve_descriptor_change_diff_entry(input).unwrap().degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    let mut input = clean_downgrade_input();
    input.comparison_scope = M5DescriptorChangeDiffScope::DiffScopeUnclassified;
    assert_eq!(
        resolve_descriptor_change_diff_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::EvidenceScopeUnclassified)
    );
}

#[test]
fn downgrade_form_and_surface_and_id_and_material() {
    let mut input = clean_downgrade_input();
    input.resolution_form_coverage =
        vec![M5ConstrainedStateDescriptorResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_descriptor_change_diff_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::EvidenceFormCoverageIncomplete)
    );

    let mut input = clean_downgrade_input();
    input.surface_context = M5ConstrainedStateDescriptorSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_descriptor_change_diff_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5DescriptorChangeDiffEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_downgrade_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_descriptor_change_diff_entry(input).unwrap_err(),
        M5ConstrainedStateDescriptorResolutionError::EmptyCohortEvidencePacketEntryId
    );

    let mut input = clean_downgrade_input();
    input.known_limits_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_descriptor_change_diff_entry(input).unwrap_err(),
        M5ConstrainedStateDescriptorResolutionError::ForbiddenMaterial
    );
}

#[test]
fn downgrade_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to line proof stays honest.
    let mut input = clean_downgrade_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = true;
    assert!(resolve_descriptor_change_diff_entry(input)
        .unwrap()
        .is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_downgrade_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = true;
    assert!(resolve_descriptor_change_diff_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_constrained_state_descriptor_and_change_diff_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.vocabulary_set.report_section_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CONSTRAINED_FILE_STATE_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DESCRIPTOR_CHANGE_DIFF_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ConstrainedStateDescriptorAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ConstrainedStateDescriptorExportField::CohortArchetypes);
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0]
        .descriptor_change_diff_entries
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    // Force a clean constrained_state_descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.constrained_state_descriptor_entries[0].degrade_reason = None;
    row.constrained_state_descriptor_entries[0].constrained_state_descriptor_object_complete =
        false;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.widens_a_line_without_current_rollback_and_diagnostics_downgrade = true,
            1 => row.runs_partner_or_public_support_language_ahead_of_line_proof = true,
            2 => row.hides_the_rollback_target_or_diagnostics_posture_before_widening = true,
            _ => row.collapses_distinct_descriptor_change_diff_classes_into_one_lane = true,
        }
        assert!(packet.validate().contains(
            &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn constrained_state_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    for row in &mut packet.registry_rows {
        row.constrained_state_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ConstrainedStateDescriptorEntryDegradeReason::CohortDescriptorObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn constrained_state_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    // Drop every clean executive-steering-surface constrained_state_descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.constrained_state_descriptor_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    for row in &mut packet.registry_rows {
        row.constrained_state_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    for row in &mut packet.registry_rows {
        row.constrained_state_descriptor_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ConstrainedStateDescriptorEntryDegradeReason::DescriptorNotBoundToRegistry,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn descriptor_change_diff_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    for row in &mut packet.registry_rows {
        row.descriptor_change_diff_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5DescriptorChangeDiffEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::CohortEvidenceIntegrityNotProven
    ));
}

#[test]
fn descriptor_change_diff_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    // Drop every clean go-no-go-signoff downgrade so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.descriptor_change_diff_entries
            .retain(|ex| !(ex.is_clean() && ex.comparison_scope == "write_target_change"));
    }
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::CohortEvidenceIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet
        .governance_review
        .lines_cannot_widen_without_rollback_and_diagnostics = false;
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://line.example/downgrade leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedStateDescriptorChangeDiffRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_constrained_state_descriptor_and_change_diff_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn constrained_state_descriptor_table_lists_only_clean_constrained_state_descriptors() {
    let packet = seeded_m5_constrained_state_descriptor_and_change_diff_registries();
    let table = packet.render_constrained_state_descriptor_table();
    // The clean canary and migration constrained_state_descriptors are rendered from the registry.
    assert!(table.contains("read_only_path_object_mode"));
    assert!(table.contains("generated_artifact_object_mode"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_constrained_state_descriptor_and_change_diff_registries_export()
        .expect(
        "checked M5 line-constrained_state_descriptor / line-downgrade-packet registries export validates",
    );
    assert_eq!(
        from_disk.packet_id,
        M5_CONSTRAINED_STATE_DESCRIPTOR_CHANGE_DIFF_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_constrained_state_descriptor_and_change_diff_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_constrained_state_descriptor_and_change_diff_registries_constrained_state_descriptor_beta_narrowed(
        );
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ConstrainedFileStateConsumerSurface::TabChrome)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5ConstrainedFileStateQualificationClass::Beta
    );

    let preview =
        seeded_m5_constrained_state_descriptor_and_change_diff_registries_descriptor_change_diff_preview_narrowed(
        );
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ConstrainedFileStateConsumerSurface::StatusBar)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5ConstrainedFileStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ConstrainedStateDescriptorChangeDiffRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-state-descriptor-and-change-diff-registries/constrained_state_descriptor_beta_narrowed.json"
    )))
    .expect("line-constrained_state_descriptor fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_constrained_state_descriptor_and_change_diff_registries_constrained_state_descriptor_beta_narrowed(
        )
    );

    let preview: M5ConstrainedStateDescriptorChangeDiffRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-state-descriptor-and-change-diff-registries/descriptor_change_diff_preview_narrowed.json"
    )))
    .expect("line-downgrade fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_constrained_state_descriptor_and_change_diff_registries_descriptor_change_diff_preview_narrowed(
        )
    );
}

#[test]
fn implemented_lines_is_all_six_constrained_file_state_object_classes() {
    assert_eq!(IMPLEMENTED_LINES, M5ConstrainedFileStateObject::ALL);
    assert_eq!(IMPLEMENTED_LINES.len(), 6);
}
