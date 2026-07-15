use super::*;

fn clean_descriptor_input() -> M5RegressionAssetEntryResolutionInput {
    M5RegressionAssetEntryResolutionInput {
        entry_id: "descriptor:test".to_owned(),
        asset_binding_id: "incident.lane.core-team-canary".to_owned(),
        token_name: "regression.asset.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        regression_asset_type: M5RegressionAssetTypeKind::AutomatedTest,
        surface_context: M5RegressionAssetSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5RegressionAssetResolutionForm::ALL.to_vec(),
        exact_build_reference: "repo.rows.core-team-canary-archetypes".to_owned(),
        affected_row_reference: "bundle.ids.canary-0007".to_owned(),
        cohort_ring_reference: "install.topology.internal-dogfood-ring".to_owned(),
        workaround_lineage: "toolchain.envelope.pinned-canary".to_owned(),
        regression_asset_reference: "known-limits.published.canary".to_owned(),
        approved_exception_reference: "rollback.target.canary-previous-stable".to_owned(),
        close_blocker_reference: "diagnostics.posture.full-telemetry".to_owned(),
        bound_to_registry: true,
        regression_asset_linked_before_closure: true,
        is_severe_incident: false,
        attributable_asset_or_approved_exception: true,
        proof_fresh: true,
    }
}

fn clean_evidence_input() -> M5IncidentCloseEntryResolutionInput {
    M5IncidentCloseEntryResolutionInput {
        entry_id: "evidence:test".to_owned(),
        incident_close_ref: "incident.lane.core-team-canary".to_owned(),
        token_name: "incident.close.core_team_canary".to_owned(),
        semantic_role: M5LaunchControlRole::CohortMembership,
        incident_severity: M5IncidentSeverityKind::SevOneIncident,
        surface_context: M5RegressionAssetSurfaceContext::ShiproomSurface,
        resolution_form_coverage: M5RegressionAssetResolutionForm::ALL.to_vec(),
        resolved_incident_identity: "transition-id.core-team-canary-0007".to_owned(),
        linked_regression_asset_ledger: "known-limits.ledger.canary".to_owned(),
        exact_build_and_row_reference: "rollback.target.ref.canary".to_owned(),
        cohort_ring_lineage_state: "rehearsal.currency.dogfood-ring-current".to_owned(),
        close_lineage_freshness_state: "readiness.signoff.dogfood-reviewed".to_owned(),
        workaround_lineage_reference: "support.language.canary-bound-to-proof".to_owned(),
        last_incident_close_revision: "widening.revision.0007".to_owned(),
        keeps_incident_lineage_visible: true,
        close_lineage_is_truthful: true,
        close_without_asset_requested: false,
        close_blocked_until_asset_linked: false,
        lineage_gap_present: false,
        lineage_gap_flagged: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_regression_asset_and_incident_close_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_PACKET_ID
    );
}

#[test]
fn descriptor_clean_names_meaning_and_is_bound() {
    let resolved = resolve_regression_asset_entry(clean_descriptor_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.regression_asset_resolves_across_types);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.regression_asset_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.regression_asset_type_is_classified);
    assert!(resolved.regression_asset_linked_before_closure);
    assert_eq!(resolved.semantic_role, "cohort_membership");
    assert_eq!(resolved.regression_asset_type, "automated_test");
    assert_eq!(
        resolved.canonical_regression_asset_type_mode,
        "automated_test_type"
    );
    assert_eq!(resolved.surface_context, "shiproom_surface");
    assert_eq!(
        resolved.next_action,
        M5RegressionAssetNextAction::ExpandRegressionAssetMeaning
    );
}

#[test]
fn regression_asset_token_unstated_degrades() {
    let mut input = clean_descriptor_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetTokenUnstated)
    );
}

#[test]
fn descriptor_unbound_and_unclassified_degrade() {
    let mut input = clean_descriptor_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetNotBoundToRegistry)
    );

    let mut input = clean_descriptor_input();
    input.regression_asset_type = M5RegressionAssetTypeKind::AssetTypeUnclassified;
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetTypeUnclassified)
    );
}

#[test]
fn descriptor_object_incomplete_and_widen_fold_and_form_degrade() {
    // An unstated bundle IDs field leaves the resolved object incomplete.
    let mut input = clean_descriptor_input();
    input.affected_row_reference = "  ".to_owned();
    let resolved = resolve_regression_asset_entry(input).unwrap();
    assert!(!resolved.regression_asset_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::RegressionAssetObjectIncomplete)
    );

    // A cohort widening without a preserved rollback / diagnostics posture degrades with the structured blocker.
    let mut input = clean_descriptor_input();
    input.regression_asset_linked_before_closure = false;
    assert_eq!(
        resolve_regression_asset_entry(input).unwrap().degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.resolution_form_coverage = vec![M5RegressionAssetResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn descriptor_public_facing_and_surface_and_proof_degrade() {
    // A public-facing cohort running support language ahead of proof first fails the widen-boundary fold.
    let mut input = clean_descriptor_input();
    input.regression_asset_type = M5RegressionAssetTypeKind::SchemaPolicyGuard;
    input.is_severe_incident = true;
    input.attributable_asset_or_approved_exception = false;
    assert_eq!(
        resolve_regression_asset_entry(input).unwrap().degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof)
    );

    let mut input = clean_descriptor_input();
    input.surface_context = M5RegressionAssetSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_descriptor_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_regression_asset_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5RegressionAssetEntryDegradeReason::ProofStale)
    );
}

#[test]
fn descriptor_empty_id_and_forbidden_material_error() {
    let mut input = clean_descriptor_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_regression_asset_entry(input).unwrap_err(),
        M5RegressionAssetResolutionError::EmptyRegressionAssetEntryId
    );

    let mut input = clean_descriptor_input();
    input.approved_exception_reference = "see https://cohort.internal/leak".to_owned();
    assert_eq!(
        resolve_regression_asset_entry(input).unwrap_err(),
        M5RegressionAssetResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cohort_preserves_rollback_and_diagnostics_rejects_unpreserved() {
    assert!(regression_asset_attributable_before_closure(
        M5RegressionAssetTypeKind::AutomatedTest,
        true,
        false,
        true
    ));
    assert!(!regression_asset_attributable_before_closure(
        M5RegressionAssetTypeKind::AutomatedTest,
        false,
        false,
        true
    ));
    assert!(regression_asset_attributable_before_closure(
        M5RegressionAssetTypeKind::SchemaPolicyGuard,
        true,
        true,
        true
    ));
    assert!(!regression_asset_attributable_before_closure(
        M5RegressionAssetTypeKind::SchemaPolicyGuard,
        true,
        true,
        false
    ));
    assert!(!regression_asset_attributable_before_closure(
        M5RegressionAssetTypeKind::AssetTypeUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn regression_asset_object_is_complete_requires_all_fields() {
    assert!(regression_asset_object_is_complete(
        M5RegressionAssetTypeKind::AutomatedTest,
        "repo.rows.core-team-canary-archetypes",
        "bundle.ids.canary-0007",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!regression_asset_object_is_complete(
        M5RegressionAssetTypeKind::AutomatedTest,
        "repo.rows.core-team-canary-archetypes",
        "  ",
        "install.topology.internal-dogfood-ring",
        "toolchain.envelope.pinned-canary",
        "known-limits.published.canary",
        "rollback.target.canary-previous-stable",
        "diagnostics.posture.full-telemetry",
    ));
    assert!(!regression_asset_object_is_complete(
        M5RegressionAssetTypeKind::AssetTypeUnclassified,
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
    let resolved = resolve_incident_close_entry(clean_evidence_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.incident_close_safe_on_every_severity);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_incident_close_record);
    assert!(resolved.incident_close_stays_honest);
    assert_eq!(resolved.incident_severity, "sev_one_incident");
    assert_eq!(resolved.surface_context, "shiproom_surface");
}

#[test]
fn evidence_support_ahead_and_unclassified_degrade() {
    // Support language present but not bound to cohort proof runs support ahead of proof.
    let mut input = clean_evidence_input();
    input.close_without_asset_requested = true;
    input.close_blocked_until_asset_linked = false;
    let resolved = resolve_incident_close_entry(input).unwrap();
    assert!(!resolved.provides_complete_incident_close_record);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset)
    );

    // A packet that hides the cohort evidence is also caught.
    let mut input = clean_evidence_input();
    input.keeps_incident_lineage_visible = false;
    assert_eq!(
        resolve_incident_close_entry(input).unwrap().degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset)
    );

    // A known-limits gap masquerading as covered is also caught.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = false;
    assert_eq!(
        resolve_incident_close_entry(input).unwrap().degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset)
    );

    let mut input = clean_evidence_input();
    input.incident_severity = M5IncidentSeverityKind::SeverityUnclassified;
    assert_eq!(
        resolve_incident_close_entry(input).unwrap().degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::IncidentSeverityUnclassified)
    );
}

#[test]
fn evidence_form_and_surface_and_id_and_material() {
    let mut input = clean_evidence_input();
    input.resolution_form_coverage = vec![M5RegressionAssetResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_incident_close_entry(input).unwrap().degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::IncidentCloseFormCoverageIncomplete)
    );

    let mut input = clean_evidence_input();
    input.surface_context = M5RegressionAssetSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_incident_close_entry(input).unwrap().degrade_reason,
        Some(M5IncidentCloseEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_evidence_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_incident_close_entry(input).unwrap_err(),
        M5RegressionAssetResolutionError::EmptyIncidentCloseEntryId
    );

    let mut input = clean_evidence_input();
    input.linked_regression_asset_ledger = "see internal://notes".to_owned();
    assert_eq!(
        resolve_incident_close_entry(input).unwrap_err(),
        M5RegressionAssetResolutionError::ForbiddenMaterial
    );
}

#[test]
fn evidence_bound_support_and_flagged_gap_stay_clean() {
    // Support language bound to cohort proof stays honest.
    let mut input = clean_evidence_input();
    input.close_without_asset_requested = true;
    input.close_blocked_until_asset_linked = true;
    assert!(resolve_incident_close_entry(input).unwrap().is_clean());

    // A known-limits gap flagged rather than masquerading stays honest.
    let mut input = clean_evidence_input();
    input.lineage_gap_present = true;
    input.lineage_gap_flagged = true;
    assert!(resolve_incident_close_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_regression_asset_and_incident_close_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.vocabulary_set.regression_asset_type_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_REGRESSION_ASSET_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_INCIDENT_CLOSE_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RegressionAssetAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5RegressionAssetExportField::RegressionAssetTypes);
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0].incident_close_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    // Force a clean descriptor entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.regression_asset_entries[0].degrade_reason = None;
    row.regression_asset_entries[0].regression_asset_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.closes_a_severe_incident_without_a_linked_regression_asset = true,
            1 => row.lets_an_approved_exception_become_an_untracked_close = true,
            2 => row.hides_the_build_row_or_cohort_lineage_on_the_regression_asset = true,
            _ => row.collapses_distinct_incident_severity_classes_into_one_lane = true,
        }
        assert!(packet
            .validate()
            .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn cohort_descriptor_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    for row in &mut packet.registry_rows {
        row.regression_asset_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RegressionAssetEntryDegradeReason::RegressionAssetObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::RegressionAssetResolutionNotProven
    ));
}

#[test]
fn cohort_descriptor_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    // Drop every clean executive-steering-surface descriptor so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.regression_asset_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "executive_steering_surface"));
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::RegressionAssetResolutionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_widen_fold_example_removed() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    for row in &mut packet.registry_rows {
        row.regression_asset_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5RegressionAssetEntryDegradeReason::IncidentClosesWithoutRegressionAssetOrRunsClaimAheadOfProof,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseAttributionNotProven
    ));
}

#[test]
fn rollback_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    for row in &mut packet.registry_rows {
        row.regression_asset_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5RegressionAssetEntryDegradeReason::RegressionAssetNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseAttributionNotProven
    ));
}

#[test]
fn incident_close_integrity_not_proven_when_support_ahead_example_removed() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    for row in &mut packet.registry_rows {
        row.incident_close_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5IncidentCloseEntryDegradeReason::IncidentCloseDropsLineageOrClosesWithoutRegressionAsset,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseIntegrityNotProven
    ));
}

#[test]
fn incident_close_integrity_not_proven_when_scope_dropped() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    // Drop every clean go-no-go-signoff evidence so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.incident_close_entries
            .retain(|ex| !(ex.is_clean() && ex.incident_severity == "launch_bearing_failure"));
    }
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::IncidentCloseIntegrityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet
        .governance_review
        .severe_incidents_cannot_close_without_regression_asset_and_lineage = false;
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5RegressionAssetIncidentCloseRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_regression_asset_and_incident_close_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RegressionAssetIncidentCloseRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_regression_asset_and_incident_close_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_regression_asset_and_incident_close_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_regression_asset_and_incident_close_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn cohort_descriptor_table_lists_only_clean_descriptors() {
    let packet = seeded_m5_regression_asset_and_incident_close_registries();
    let table = packet.render_regression_asset_table();
    // The clean canary and migration descriptors are rendered from the registry.
    assert!(table.contains("automated_test_type"));
    assert!(table.contains("fixture_repository_type"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains(":incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_regression_asset_and_incident_close_registries_export()
        .expect("checked M5 regression-asset / incident-close registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_REGRESSION_ASSET_INCIDENT_CLOSE_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_regression_asset_and_incident_close_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5LaunchControlConsumerSurface::Shiproom)
        .unwrap();
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let preview =
        seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed();
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
    let beta: M5RegressionAssetIncidentCloseRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-regression-asset-and-incident-close-registries/regression_asset_beta_narrowed.json"
    )))
    .expect("regression-asset fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_regression_asset_and_incident_close_registries_regression_asset_beta_narrowed()
    );

    let preview: M5RegressionAssetIncidentCloseRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-regression-asset-and-incident-close-registries/incident_close_preview_narrowed.json"
    )))
    .expect("incident-close fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_regression_asset_and_incident_close_registries_incident_close_preview_narrowed()
    );
}

#[test]
fn implemented_ring_stages_is_all_five_widening_stages() {
    assert_eq!(
        IMPLEMENTED_REGRESSION_STAGES,
        [
            M5LaunchControlWideningStage::Alpha,
            M5LaunchControlWideningStage::Beta,
            M5LaunchControlWideningStage::ReleaseCandidate,
            M5LaunchControlWideningStage::Stable,
            M5LaunchControlWideningStage::LongTermSupport,
        ]
    );
}
