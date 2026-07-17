use super::*;

fn clean_review_template_packet_input() -> M5ReviewTemplatePacketEntryResolutionInput {
    M5ReviewTemplatePacketEntryResolutionInput {
        entry_id: "review_template_packet:test".to_owned(),
        line_binding_id: "launch.line.core-team-canary".to_owned(),
        token_name: "line.review_template_packet.core_team_canary".to_owned(),
        semantic_role: M5ReviewPackRole::EvaluatorResultClassDisclosure,
        report_section: M5ReviewTemplatePacketKind::RationaleBlock,
        surface_context: M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5ReviewTemplatePacketResolutionForm::ALL.to_vec(),
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

fn clean_downgrade_input() -> M5TemplatePublishAttributionEntryResolutionInput {
    M5TemplatePublishAttributionEntryResolutionInput {
        entry_id: "downgrade:test".to_owned(),
        comparison_ref: "launch.line.core-team-canary".to_owned(),
        token_name: "line.downgrade.core_team_canary".to_owned(),
        semantic_role: M5ReviewPackRole::EvaluatorResultClassDisclosure,
        comparison_scope: M5TemplatePublishAttributionScope::FieldProvenanceBinding,
        surface_context: M5ReviewTemplatePacketSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5ReviewTemplatePacketResolutionForm::ALL.to_vec(),
        resolved_line_identity: "line-id.core-team-canary-0007".to_owned(),
        known_limits_ledger: "known-limits.ledger.canary".to_owned(),
        rollback_target_reference: "rollback.target.ref.canary".to_owned(),
        rehearsal_currency_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        readiness_signoff_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        support_language_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_widening_revision: "widening.revision.0007".to_owned(),
        keeps_template_publish_attribution_visible: true,
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
    let packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_PACKET_ID
    );
}

#[test]
fn review_template_packet_clean_names_meaning_and_is_bound() {
    let resolved =
        resolve_review_template_packet_entry(clean_review_template_packet_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.review_template_packet_resolves_across_lines);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.review_template_packet_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.report_section_is_classified);
    assert!(resolved.rollback_and_diagnostics_bounded);
    assert_eq!(resolved.semantic_role, "evaluator_result_class_disclosure");
    assert_eq!(resolved.report_section, "rationale_block");
    assert_eq!(
        resolved.canonical_report_section_mode,
        "rationale_block_mode"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5ReviewTemplatePacketNextAction::ExpandCohortMeaning
    );
}

#[test]
fn review_template_packet_token_unstated_degrades() {
    let mut input = clean_review_template_packet_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::DescriptorTokenUnstated)
    );
}

#[test]
fn review_template_packet_unbound_and_unclassified_degrade() {
    let mut input = clean_review_template_packet_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::DescriptorNotBoundToRegistry)
    );

    let mut input = clean_review_template_packet_input();
    input.report_section = M5ReviewTemplatePacketKind::TemplatePacketUnclassified;
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::CohortTemplatePacketUnclassified)
    );
}

#[test]
fn review_template_packet_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_review_template_packet_input();
    input.bundle_ids = "  ".to_owned();
    let resolved = resolve_review_template_packet_entry(input).unwrap();
    assert!(!resolved.review_template_packet_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::CohortDescriptorObjectIncomplete)
    );

    // A line widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_review_template_packet_input();
    input.rollback_and_diagnostics_bounded = false;
    assert_eq!(
        resolve_review_template_packet_entry(input).unwrap().degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_review_template_packet_input();
    input.resolution_form_coverage = vec![M5ReviewTemplatePacketResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn review_template_packet_public_facing_and_surface_and_proof_degrade() {
    // A public-facing line running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_review_template_packet_input();
    input.report_section = M5ReviewTemplatePacketKind::RedactedField;
    input.is_public_facing_line = true;
    input.support_language_matches_line_proof = false;
    assert_eq!(
        resolve_review_template_packet_entry(input).unwrap().degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof)
    );

    let mut input = clean_review_template_packet_input();
    input.surface_context = M5ReviewTemplatePacketSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_review_template_packet_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_review_template_packet_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5ReviewTemplatePacketEntryDegradeReason::ProofStale)
    );
}

#[test]
fn review_template_packet_empty_id_and_forbidden_material_error() {
    let mut input = clean_review_template_packet_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_review_template_packet_entry(input).unwrap_err(),
        M5ReviewTemplatePacketResolutionError::EmptyCohortDescriptorEntryId
    );

    let mut input = clean_review_template_packet_input();
    input.rollback_target = "see https://line.internal/leak".to_owned();
    assert_eq!(
        resolve_review_template_packet_entry(input).unwrap_err(),
        M5ReviewTemplatePacketResolutionError::ForbiddenMaterial
    );
}

#[test]
fn line_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(line_preserves_rollback_and_diagnostics_before_widening(
        M5ReviewTemplatePacketKind::RationaleBlock,
        true,
        false,
        true
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ReviewTemplatePacketKind::RationaleBlock,
        false,
        false,
        true
    ));
    assert!(line_preserves_rollback_and_diagnostics_before_widening(
        M5ReviewTemplatePacketKind::RedactedField,
        true,
        true,
        true
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ReviewTemplatePacketKind::RedactedField,
        true,
        true,
        false
    ));
    assert!(!line_preserves_rollback_and_diagnostics_before_widening(
        M5ReviewTemplatePacketKind::TemplatePacketUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn review_template_packet_object_is_complete_requires_all_fields() {
    assert!(review_template_packet_object_is_complete(
        M5ReviewTemplatePacketKind::RationaleBlock,
        "repo.rows.core-team-canary-journeys",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!review_template_packet_object_is_complete(
        M5ReviewTemplatePacketKind::RationaleBlock,
        "repo.rows.core-team-canary-journeys",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!review_template_packet_object_is_complete(
        M5ReviewTemplatePacketKind::TemplatePacketUnclassified,
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
    let resolved = resolve_template_publish_attribution_entry(clean_downgrade_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.comparison_safe_on_every_line);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_template_publish_attribution);
    assert!(resolved.template_publish_attribution_stays_honest);
    assert_eq!(resolved.comparison_scope, "field_provenance_binding");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn downgrade_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to line proof runs support ahead of proof.
    let mut input = clean_downgrade_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = false;
    let resolved = resolve_template_publish_attribution_entry(input).unwrap();
    assert!(!resolved.provides_complete_template_publish_attribution);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A packet that hides the line downgrade is also caught.
    let mut input = clean_downgrade_input();
    input.keeps_template_publish_attribution_visible = false;
    assert_eq!(
        resolve_template_publish_attribution_entry(input).unwrap().degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_downgrade_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = false;
    assert_eq!(
        resolve_template_publish_attribution_entry(input).unwrap().degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence)
    );

    let mut input = clean_downgrade_input();
    input.comparison_scope =
        M5TemplatePublishAttributionScope::TemplatePublishAttributionUnclassified;
    assert_eq!(
        resolve_template_publish_attribution_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::EvidenceScopeUnclassified)
    );
}

#[test]
fn downgrade_form_and_surface_and_id_and_material() {
    let mut input = clean_downgrade_input();
    input.resolution_form_coverage = vec![M5ReviewTemplatePacketResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_template_publish_attribution_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::EvidenceFormCoverageIncomplete)
    );

    let mut input = clean_downgrade_input();
    input.surface_context = M5ReviewTemplatePacketSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_template_publish_attribution_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5TemplatePublishAttributionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_downgrade_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_template_publish_attribution_entry(input).unwrap_err(),
        M5ReviewTemplatePacketResolutionError::EmptyCohortEvidencePacketEntryId
    );

    let mut input = clean_downgrade_input();
    input.known_limits_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_template_publish_attribution_entry(input).unwrap_err(),
        M5ReviewTemplatePacketResolutionError::ForbiddenMaterial
    );
}

#[test]
fn downgrade_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to line proof stays honest.
    let mut input = clean_downgrade_input();
    input.support_language_present = true;
    input.support_language_bound_to_proof = true;
    assert!(resolve_template_publish_attribution_entry(input)
        .unwrap()
        .is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_downgrade_input();
    input.known_limits_gap_present = true;
    input.known_limits_gap_flagged = true;
    assert!(resolve_template_publish_attribution_entry(input)
        .unwrap()
        .is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_review_template_packet_and_publish_attribution_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.vocabulary_set.report_section_kinds.pop();
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::VocabularySetDrift
    ));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REVIEW_TEMPLATE_PACKET_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TEMPLATE_PUBLISH_ATTRIBUTION_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ReviewTemplatePacketAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ReviewTemplatePacketExportField::CohortArchetypes);
    assert!(packet
        .validate()
        .contains(&M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0]
        .template_publish_attribution_entries
        .clear();
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::ExamplesMissing
    ));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    // Force a clean review_template_packet entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.review_template_packet_entries[0].degrade_reason = None;
    row.review_template_packet_entries[0].review_template_packet_object_complete = false;
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::DishonestExample
    ));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.widens_a_line_without_current_rollback_and_diagnostics_downgrade = true,
            1 => row.runs_partner_or_public_support_language_ahead_of_line_proof = true,
            2 => row.hides_the_rollback_target_or_diagnostics_posture_before_widening = true,
            _ => row.collapses_distinct_template_publish_attribution_classes_into_one_lane = true,
        }
        assert!(packet.validate().contains(
            &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn review_template_packet_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    for row in &mut packet.registry_rows {
        row.review_template_packet_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ReviewTemplatePacketEntryDegradeReason::CohortDescriptorObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn review_template_packet_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    // Drop every clean executive-steering-surface review_template_packet so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.review_template_packet_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::CohortDescriptorResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    for row in &mut packet.registry_rows {
        row.review_template_packet_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5ReviewTemplatePacketEntryDegradeReason::DescriptorLetsCohortWidenWithoutRollbackOrRunsSupportAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    for row in &mut packet.registry_rows {
        row.review_template_packet_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ReviewTemplatePacketEntryDegradeReason::DescriptorNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::RollbackAndDiagnosticsPreservationNotProven
    ));
}

#[test]
fn template_publish_attribution_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    for row in &mut packet.registry_rows {
        row.template_publish_attribution_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5TemplatePublishAttributionEntryDegradeReason::CohortEvidenceRunsSupportAheadOfProofOrDropsCohortEvidence,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::CohortEvidenceIntegrityNotProven));
}

#[test]
fn template_publish_attribution_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    // Drop every clean go-no-go-signoff downgrade so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.template_publish_attribution_entries.retain(|ex| {
            !(ex.is_clean() && ex.comparison_scope == "destination_and_redaction_binding")
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::CohortEvidenceIntegrityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet
        .governance_review
        .lines_cannot_widen_without_rollback_and_diagnostics = false;
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://line.example/downgrade leaked".to_owned();
    assert!(packet.validate().contains(
        &M5ReviewTemplatePacketAndPublishAttributionRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_review_template_packet_and_publish_attribution_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn review_template_packet_table_lists_only_clean_review_template_packets() {
    let packet = seeded_m5_review_template_packet_and_publish_attribution_registries();
    let table = packet.render_review_template_packet_table();
    // The clean canary and migration review_template_packets are rendered from the registry.
    assert!(table.contains("rationale_block_mode"));
    assert!(table.contains("checklist_text_mode"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_review_template_packet_and_publish_attribution_registries_export().expect(
        "checked M5 line-review_template_packet / line-downgrade-packet registries export validates",
    );
    assert_eq!(
        from_disk.packet_id,
        M5_REVIEW_TEMPLATE_PACKET_AND_PUBLISH_ATTRIBUTION_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_review_template_packet_and_publish_attribution_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReviewPackConsumerSurface::ReviewDetail)
        .unwrap();
    assert_eq!(row.qualification, M5ReviewPackQualificationClass::Beta);

    let preview =
        seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ReviewPackConsumerSurface::AiReviewPanel)
        .unwrap();
    assert_eq!(row.qualification, M5ReviewPackQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-template-packet-and-publish-attribution-registries/review_template_packet_beta_narrowed.json"
    )))
    .expect("line-review_template_packet fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_review_template_packet_and_publish_attribution_registries_review_template_packet_beta_narrowed()
    );

    let preview: M5ReviewTemplatePacketAndPublishAttributionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-review-template-packet-and-publish-attribution-registries/template_publish_attribution_preview_narrowed.json"
    )))
    .expect("line-downgrade fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_review_template_packet_and_publish_attribution_registries_template_publish_attribution_preview_narrowed()
    );
}

#[test]
fn implemented_lines_is_all_six_constrained_file_state_object_classes() {
    assert_eq!(IMPLEMENTED_LINES, M5ReviewPackObject::ALL);
    assert_eq!(IMPLEMENTED_LINES.len(), 6);
}
