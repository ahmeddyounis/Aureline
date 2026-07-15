use super::*;

fn clean_descriptor_input() -> M5FreezeExceptionEntryResolutionInput {
    M5FreezeExceptionEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        exception_binding_id: "incident.lane.core-team-canary".to_owned(),
        token_name: "freeze.exception.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        freeze_exception_change_class: M5FreezeExceptionChangeClass::PhaseAllowedChange,
        surface_context: M5FreezeExceptionSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5FreezeExceptionResolutionForm::ALL.to_vec(),
        exception_scope_reference: "repo.rows.core-team-canary-archetypes".to_owned(),
        rollback_or_narrowing_reference: "bundle.ids.canary-0007".to_owned(),
        docs_support_migration_reference: "install.topology.internal-dogfood-ring".to_owned(),
        owner_capture_reference: "toolchain.envelope.pinned-canary".to_owned(),
        risk_capture_reference: "known-limits.published.canary".to_owned(),
        change_budget_reference: "rollback.target.canary-previous-stable".to_owned(),
        expiry_reference: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        freeze_exception_documented_before_widening: true,
        requires_documented_exception: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5GoNoGoEntryResolutionInput {
    M5GoNoGoEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        go_no_go_ref: "incident.lane.core-team-canary".to_owned(),
        token_name: "go.no.go.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        go_no_go_decision: M5GoNoGoDecisionKind::GoDecision,
        surface_context: M5FreezeExceptionSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5FreezeExceptionResolutionForm::ALL.to_vec(),
        resolved_decision_identity: "transition-id.core-team-canary-0007".to_owned(),
        evidence_snapshot_ledger: "known-limits.ledger.canary".to_owned(),
        orr_signoff_reference: "rollback.target.ref.canary".to_owned(),
        on_call_roster_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        go_no_go_freshness_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        widening_stage_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_go_no_go_revision: "widening.revision.0007".to_owned(),
        keeps_evidence_snapshot_visible: true,
        go_no_go_lineage_is_truthful: true,
        override_without_evidence_requested: false,
        blocked_until_evidence_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_freeze_exception_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.freeze_exception_resolves_across_classes);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.freeze_exception_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.freeze_exception_change_class_is_classified);
    assert!(resolved.freeze_exception_documented_before_widening);
    assert_eq!(resolved.semantic_role, "cohort_membership");
    assert_eq!(
        resolved.freeze_exception_change_class,
        "phase_allowed_change"
    );
    assert_eq!(
        resolved.canonical_freeze_exception_change_class_mode,
        "phase_allowed_change_class"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5FreezeExceptionNextAction::ExpandFreezeExceptionMeaning
    );
}

#[test]
fn freeze_exception_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.freeze_exception_change_class = M5FreezeExceptionChangeClass::ChangeClassUnclassified;
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionChangeClassUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.rollback_or_narrowing_reference = "  ".to_owned();
    let resolved = resolve_freeze_exception_entry(input).unwrap();
    assert!(!resolved.freeze_exception_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionObjectIncomplete)
    );

    // A cohort widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_descriptor_input();
    input.freeze_exception_documented_before_widening = false;
    assert_eq!(
        resolve_freeze_exception_entry(input).unwrap().degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5FreezeExceptionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing cohort running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_descriptor_input();
    input.freeze_exception_change_class = M5FreezeExceptionChangeClass::MigrationOrDataChange;
    input.requires_documented_exception = true;
    input.attributable_asset_or_approved_exception = false;
    assert_eq!(
        resolve_freeze_exception_entry(input).unwrap().degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5FreezeExceptionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_freeze_exception_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5FreezeExceptionEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_freeze_exception_entry(input).unwrap_err(),
        M5FreezeExceptionResolutionError::EmptyFreezeExceptionEntryId
    );

    let mut input = clean_descriptor_input();
    input.change_budget_reference = "see https://cohort.internal/leak".to_owned();
    assert_eq!(
        resolve_freeze_exception_entry(input).unwrap_err(),
        M5FreezeExceptionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cohort_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(freeze_exception_stays_documented_before_widening(
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        true,
        false,
        true
    ));
    assert!(!freeze_exception_stays_documented_before_widening(
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        false,
        false,
        true
    ));
    assert!(freeze_exception_stays_documented_before_widening(
        M5FreezeExceptionChangeClass::MigrationOrDataChange,
        true,
        true,
        true
    ));
    assert!(!freeze_exception_stays_documented_before_widening(
        M5FreezeExceptionChangeClass::MigrationOrDataChange,
        true,
        true,
        false
    ));
    assert!(!freeze_exception_stays_documented_before_widening(
        M5FreezeExceptionChangeClass::ChangeClassUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn freeze_exception_object_is_complete_requires_all_fields() {
    assert!(freeze_exception_object_is_complete(
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!freeze_exception_object_is_complete(
        M5FreezeExceptionChangeClass::PhaseAllowedChange,
        "repo.rows.core-team-canary-archetypes",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!freeze_exception_object_is_complete(
        M5FreezeExceptionChangeClass::ChangeClassUnclassified,
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
    let resolved = resolve_go_no_go_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.go_no_go_safe_on_every_decision);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_go_no_go_record);
    assert!(resolved.go_no_go_stays_honest);
    assert_eq!(resolved.go_no_go_decision, "go_decision");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn evidence_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to cohort proof runs support ahead of proof.
    let mut input = clean_evidence_input();
    input.override_without_evidence_requested = true;
    input.blocked_until_evidence_linked = false;
    let resolved = resolve_go_no_go_entry(input).unwrap();
    assert!(!resolved.provides_complete_go_no_go_record);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
    );

    // A packet that hides the cohort evidence is also caught.
    let mut input = clean_evidence_input();
    input.keeps_evidence_snapshot_visible = false;
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap().degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = false;
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap().degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
    );

    let mut input = clean_evidence_input();
    input.go_no_go_decision = M5GoNoGoDecisionKind::DecisionUnclassified;
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap().degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::GoNoGoDecisionUnclassified)
    );
}

#[test]
fn evidence_form_and_surface_and_id_and_material() {
    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5FreezeExceptionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap().degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::GoNoGoFormCoverageIncomplete)
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5FreezeExceptionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap().degrade_reason,
        Some(M5GoNoGoEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap_err(),
        M5FreezeExceptionResolutionError::EmptyGoNoGoEntryId
    );

    let mut input = clean_evidence_input();
    input.evidence_snapshot_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_go_no_go_entry(input).unwrap_err(),
        M5FreezeExceptionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to cohort proof stays honest.
    let mut input = clean_evidence_input();
    input.override_without_evidence_requested = true;
    input.blocked_until_evidence_linked = true;
    assert!(resolve_go_no_go_entry(input).unwrap().is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = true;
    assert!(resolve_go_no_go_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_freeze_exception_and_go_no_go_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet
        .vocabulary_set
        .freeze_exception_change_class_kinds
        .pop();
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_FREEZE_EXCEPTION_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_GO_NO_GO_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5FreezeExceptionAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5FreezeExceptionExportField::FreezeExceptionChangeClasses);
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0].go_no_go_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.freeze_exception_entries[0].degrade_reason = None;
    row.freeze_exception_entries[0].freeze_exception_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.widens_scope_without_a_documented_freeze_exception = true,
            1 => row.lets_a_freeze_exception_become_undocumented_scope_widening = true,
            2 => row.hides_the_change_budget_or_owner_risk_on_the_freeze_exception = true,
            _ => row.collapses_distinct_go_no_go_decision_classes_into_one_lane = true,
        }
        assert!(packet
            .validate()
            .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn cohort_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    for row in &mut packet.registry_rows {
        row.freeze_exception_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionObjectIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::FreezeExceptionResolutionNotProven));
}

#[test]
fn cohort_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    // Drop every clean executive-steering-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.freeze_exception_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::FreezeExceptionResolutionNotProven));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    for row in &mut packet.registry_rows {
        row.freeze_exception_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5FreezeExceptionEntryDegradeReason::FreezeExceptionWidensScopeUndocumentedOrRunsClaimAheadOfProof,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoAttributionNotProven));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    for row in &mut packet.registry_rows {
        row.freeze_exception_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5FreezeExceptionEntryDegradeReason::FreezeExceptionNotBoundToRegistry)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoAttributionNotProven));
}

#[test]
fn go_no_go_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    for row in &mut packet.registry_rows {
        row.go_no_go_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5GoNoGoEntryDegradeReason::GoNoGoDropsEvidenceOrImpliesGreenWhileStale)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoIntegrityNotProven));
}

#[test]
fn go_no_go_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    // Drop every clean go-no-go-signoff evidence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.go_no_go_entries
            .retain(|ex| !(ex.is_clean() && ex.go_no_go_decision == "conditional_go_decision"));
    }
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::GoNoGoIntegrityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet
        .governance_review
        .scope_cannot_widen_without_documented_freeze_exception = false;
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5FreezeExceptionGoNoGoRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_freeze_exception_and_go_no_go_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn cohort_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_freeze_exception_and_go_no_go_registries();
    let table = packet.render_freeze_exception_table();
    // The clean canary and migration descriptors are rendered from the registry.
    assert!(table.contains("phase_allowed_change_class"));
    assert!(table.contains("exception_required_change_class"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_freeze_exception_and_go_no_go_registries_export()
        .expect("checked M5 freeze-exception / go-no-go registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_FREEZE_EXCEPTION_GO_NO_GO_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_freeze_exception_and_go_no_go_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .unwrap();
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let preview = seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed();
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
    let beta: M5FreezeExceptionGoNoGoRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-freeze-exception-and-go-no-go-registries/freeze_exception_beta_narrowed.json"
    )))
    .expect("freeze-exception fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed()
    );

    let preview: M5FreezeExceptionGoNoGoRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-freeze-exception-and-go-no-go-registries/go_no_go_preview_narrowed.json"
    )))
    .expect("go-no-go fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed()
    );
}

#[test]
fn implemented_ring_stages_is_all_five_widening_stages() {
    assert_eq!(
        IMPLEMENTED_FREEZE_EXCEPTION_STAGES,
        [
            M5LaunchControlWideningStage::Alpha,
            M5LaunchControlWideningStage::Beta,
            M5LaunchControlWideningStage::ReleaseCandidate,
            M5LaunchControlWideningStage::Stable,
            M5LaunchControlWideningStage::LongTermSupport,
        ]
    );
}
