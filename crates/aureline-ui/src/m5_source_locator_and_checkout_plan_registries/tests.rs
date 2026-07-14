use super::*;

fn clean_locator_input() -> M5SourceLocatorEntryResolutionInput {
    M5SourceLocatorEntryResolutionInput {
        entry_id: "locator:test".to_owned(),
        entry_flow_id: "entry.acme.open-local".to_owned(),
        token_name: "source.locator.local_path".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::SourceLocator,
        source_locator_kind: M5SourceLocatorKind::LocalPathSource,
        surface_context: M5AcquisitionSurfaceContext::ShellSurface,
        resolution_form_coverage: M5AcquisitionResolutionForm::ALL.to_vec(),
        literal_target: "local-path.acme/repo".to_owned(),
        resolved_root_or_container: "checkout-root.acme/repo".to_owned(),
        trust_stage_metadata: "trust-stage.staged.v3".to_owned(),
        credential_posture: "credential-posture.not-required".to_owned(),
        signer_or_mirror_provenance: "signer-provenance.acme.v3".to_owned(),
        mirror_or_air_gap_hint: "mirror-hint.online".to_owned(),
        bound_to_registry: true,
        literal_target_preserved: true,
        touches_network_or_mirror: false,
        credential_posture_disclosed: true,
        proof_fresh: true,
    }
}

fn clean_plan_input() -> M5CheckoutPlanEntryResolutionInput {
    M5CheckoutPlanEntryResolutionInput {
        entry_id: "plan:test".to_owned(),
        source_ref: "entry.acme.open-local".to_owned(),
        token_name: "checkout.plan.full".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::CheckoutPlan,
        checkout_mode: M5CheckoutMode::FullCheckoutPlan,
        surface_context: M5AcquisitionSurfaceContext::ShellSurface,
        resolution_form_coverage: M5AcquisitionResolutionForm::ALL.to_vec(),
        ref_selection: "ref.main".to_owned(),
        depth_filter: "depth.full".to_owned(),
        submodule_mode: "submodule.recursive-staged".to_owned(),
        lfs_posture: "lfs.deferred".to_owned(),
        destination_path: "destination.acme/repo".to_owned(),
        cost_band: "cost.small".to_owned(),
        keeps_cost_visible_before_mutation: true,
        plan_is_truthful: true,
        repo_owned_action_scheduled: false,
        repo_owned_action_staged_not_auto_run: false,
        implicit_mutation_asserted: false,
        implicit_mutation_explained: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_source_locator_and_checkout_plan_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_PACKET_ID
    );
}

#[test]
fn locator_clean_names_meaning_and_is_bound() {
    let resolved = resolve_source_locator_entry(clean_locator_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.locator_resolves_across_entry_flows);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.source_locator_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.source_locator_kind_is_classified);
    assert!(resolved.literal_target_preserved);
    assert_eq!(resolved.semantic_role, "source_locator");
    assert_eq!(resolved.source_locator_kind, "local_path_source");
    assert_eq!(resolved.canonical_locator_mode, "local_path_locator");
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5AcquisitionNextAction::ExpandAcquisitionMeaning
    );
}

#[test]
fn locator_token_unstated_degrades() {
    let mut input = clean_locator_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::LocatorTokenUnstated)
    );
}

#[test]
fn locator_unbound_and_unclassified_degrade() {
    let mut input = clean_locator_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::LocatorNotBoundToRegistry)
    );

    let mut input = clean_locator_input();
    input.source_locator_kind = M5SourceLocatorKind::KindUnclassified;
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorKindUnclassified)
    );
}

#[test]
fn locator_object_incomplete_and_rewrite_and_form_degrade() {
    // An unstated resolved root leaves the resolved object incomplete.
    let mut input = clean_locator_input();
    input.resolved_root_or_container = "  ".to_owned();
    let resolved = resolve_source_locator_entry(input).unwrap();
    assert!(!resolved.source_locator_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorObjectIncomplete)
    );

    // A rewritten literal target degrades.
    let mut input = clean_locator_input();
    input.literal_target_preserved = false;
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture)
    );

    let mut input = clean_locator_input();
    input.resolution_form_coverage = vec![M5AcquisitionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn locator_credential_and_surface_and_proof_degrade() {
    let mut input = clean_locator_input();
    input.source_locator_kind = M5SourceLocatorKind::RemoteForgeUrlSource;
    input.touches_network_or_mirror = true;
    input.credential_posture_disclosed = false;
    // A network locator hiding its credential posture first fails verb-preserving disclosure.
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture)
    );

    let mut input = clean_locator_input();
    input.surface_context = M5AcquisitionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_locator_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_source_locator_entry(input).unwrap().degrade_reason,
        Some(M5SourceLocatorEntryDegradeReason::ProofStale)
    );
}

#[test]
fn locator_empty_id_and_forbidden_material_error() {
    let mut input = clean_locator_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_source_locator_entry(input).unwrap_err(),
        M5AcquisitionResolutionError::EmptySourceLocatorEntryId
    );

    let mut input = clean_locator_input();
    input.signer_or_mirror_provenance = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_source_locator_entry(input).unwrap_err(),
        M5AcquisitionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn literal_target_stays_verb_preserving_rejects_rewrite() {
    assert!(literal_target_stays_verb_preserving(
        M5SourceLocatorKind::LocalPathSource,
        true,
        false,
        true
    ));
    assert!(!literal_target_stays_verb_preserving(
        M5SourceLocatorKind::LocalPathSource,
        false,
        false,
        true
    ));
    assert!(literal_target_stays_verb_preserving(
        M5SourceLocatorKind::RemoteForgeUrlSource,
        true,
        true,
        true
    ));
    assert!(!literal_target_stays_verb_preserving(
        M5SourceLocatorKind::RemoteForgeUrlSource,
        true,
        true,
        false
    ));
    assert!(!literal_target_stays_verb_preserving(
        M5SourceLocatorKind::KindUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn source_locator_object_is_complete_requires_all_fields() {
    assert!(source_locator_object_is_complete(
        M5SourceLocatorKind::LocalPathSource,
        "local-path.acme/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    ));
    assert!(!source_locator_object_is_complete(
        M5SourceLocatorKind::LocalPathSource,
        "local-path.acme/repo",
        "  ",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    ));
    assert!(!source_locator_object_is_complete(
        M5SourceLocatorKind::KindUnclassified,
        "local-path.acme/repo",
        "checkout-root.acme/repo",
        "trust-stage.staged.v3",
        "credential-posture.not-required",
        "signer-provenance.acme.v3",
        "mirror-hint.online",
    ));
}

#[test]
fn plan_clean_stays_honest() {
    let resolved = resolve_checkout_plan_entry(clean_plan_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.plan_safe_on_every_source);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_checkout_plan);
    assert!(resolved.checkout_plan_stays_honest);
    assert_eq!(resolved.checkout_mode, "full_checkout_plan");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn plan_implicit_and_unclassified_degrade() {
    // A scheduled but un-staged repo-owned action is an implicit bootstrap.
    let mut input = clean_plan_input();
    input.repo_owned_action_scheduled = true;
    input.repo_owned_action_staged_not_auto_run = false;
    let resolved = resolve_checkout_plan_entry(input).unwrap();
    assert!(!resolved.provides_complete_checkout_plan);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost)
    );

    // A plan that hides checkout cost before mutation is also an implicit bootstrap.
    let mut input = clean_plan_input();
    input.keeps_cost_visible_before_mutation = false;
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap().degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost)
    );

    // An unexplained implicit mutation is also an implicit bootstrap.
    let mut input = clean_plan_input();
    input.implicit_mutation_asserted = true;
    input.implicit_mutation_explained = false;
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap().degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost)
    );

    let mut input = clean_plan_input();
    input.checkout_mode = M5CheckoutMode::ModeUnclassified;
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap().degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::CheckoutModeUnclassified)
    );
}

#[test]
fn plan_form_and_surface_and_id_and_material() {
    let mut input = clean_plan_input();
    input.resolution_form_coverage = vec![M5AcquisitionResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap().degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::PlanFormCoverageIncomplete)
    );

    let mut input = clean_plan_input();
    input.surface_context = M5AcquisitionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap().degrade_reason,
        Some(M5CheckoutPlanEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_plan_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap_err(),
        M5AcquisitionResolutionError::EmptyCheckoutPlanEntryId
    );

    let mut input = clean_plan_input();
    input.destination_path = "see internal://notes".to_owned();
    assert_eq!(
        resolve_checkout_plan_entry(input).unwrap_err(),
        M5AcquisitionResolutionError::ForbiddenMaterial
    );
}

#[test]
fn plan_staged_action_and_explained_mutation_stay_clean() {
    // A staged repo-owned action stays honest.
    let mut input = clean_plan_input();
    input.repo_owned_action_scheduled = true;
    input.repo_owned_action_staged_not_auto_run = true;
    assert!(resolve_checkout_plan_entry(input).unwrap().is_clean());

    // An explained implicit mutation stays honest.
    let mut input = clean_plan_input();
    input.implicit_mutation_asserted = true;
    input.implicit_mutation_explained = true;
    assert!(resolve_checkout_plan_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_source_locator_and_checkout_plan_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.vocabulary_set.source_locator_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SOURCE_LOCATOR_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5AcquisitionAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5AcquisitionExportField::SourceLocatorKinds);
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0].checkout_plan_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    // Force a clean locator entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.source_locator_entries[0].degrade_reason = None;
    row.source_locator_entries[0].source_locator_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.rewrites_clone_into_open_when_local_checkout_already_exists = true,
            1 => row.runs_repo_owned_actions_implicitly_during_acquisition = true,
            2 => row.hides_checkout_cost_topology_or_credential_posture_before_mutation = true,
            _ => row.collapses_distinct_acquisition_verbs_into_one_runtime_path = true,
        }
        assert!(packet
            .validate()
            .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn source_locator_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    for row in &mut packet.registry_rows {
        row.source_locator_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SourceLocatorEntryDegradeReason::SourceLocatorObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SourceLocatorCheckoutPlanRegistriesViolation::SourceLocatorResolutionNotProven
    ));
}

#[test]
fn source_locator_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    // Drop every clean admin-surface locator so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.source_locator_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5SourceLocatorCheckoutPlanRegistriesViolation::SourceLocatorResolutionNotProven
    ));
}

#[test]
fn literal_target_preservation_not_proven_when_rewrite_example_removed() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    for row in &mut packet.registry_rows {
        row.source_locator_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SourceLocatorEntryDegradeReason::SourceLocatorRewritesVerbOrHidesCredentialPosture,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SourceLocatorCheckoutPlanRegistriesViolation::LiteralTargetPreservationNotProven
    ));
}

#[test]
fn literal_target_preservation_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    for row in &mut packet.registry_rows {
        row.source_locator_entries.retain(|ex| {
            ex.degrade_reason != Some(M5SourceLocatorEntryDegradeReason::LocatorNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SourceLocatorCheckoutPlanRegistriesViolation::LiteralTargetPreservationNotProven
    ));
}

#[test]
fn checkout_plan_integrity_not_proven_when_implicit_example_removed() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    for row in &mut packet.registry_rows {
        row.checkout_plan_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5CheckoutPlanEntryDegradeReason::CheckoutPlanRunsRepoOwnedActionOrHidesCost,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::CheckoutPlanIntegrityNotProven));
}

#[test]
fn checkout_plan_integrity_not_proven_when_mode_dropped() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    // Drop every clean sparse checkout plan so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.checkout_plan_entries
            .retain(|ex| !(ex.is_clean() && ex.checkout_mode == "sparse_checkout_plan"));
    }
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::CheckoutPlanIntegrityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.governance_review.open_and_clone_stay_distinct_verbs = false;
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_source_locator_and_checkout_plan_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://clone.example/repo leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SourceLocatorCheckoutPlanRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_source_locator_and_checkout_plan_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_source_locator_and_checkout_plan_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_source_locator_and_checkout_plan_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn source_acquisition_table_lists_only_clean_locators() {
    let packet = seeded_m5_source_locator_and_checkout_plan_registries();
    let table = packet.render_source_acquisition_table();
    // The clean local-path and remote-forge locators are rendered from the registry.
    assert!(table.contains("local_path_locator"));
    assert!(table.contains("remote_forge_url_locator"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_source_locator_and_checkout_plan_registries_export()
        .expect("checked M5 source-locator / checkout-plan registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SOURCE_LOCATOR_CHECKOUT_PLAN_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_source_locator_and_checkout_plan_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::AcquisitionEngine)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Beta
    );

    let preview =
        seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::WorkspaceService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5SourceLocatorCheckoutPlanRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-source-locator-and-checkout-plan-registries/local_path_source_beta_narrowed.json"
    )))
    .expect("local-path fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed()
    );

    let preview: M5SourceLocatorCheckoutPlanRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-source-locator-and-checkout-plan-registries/sparse_checkout_preview_narrowed.json"
    )))
    .expect("sparse fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_open_local_clone_remote_and_open_archive() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5RepositoryBootstrapFamily::OpenLocal,
            M5RepositoryBootstrapFamily::CloneRemote,
            M5RepositoryBootstrapFamily::OpenArchive,
        ]
    );
}
