use super::*;

fn clean_staging_input() -> M5StagedTrustEntryResolutionInput {
    M5StagedTrustEntryResolutionInput {
        entry_id: "staging:test".to_owned(),
        acquisition_path_id: "entry.acme.open-local".to_owned(),
        token_name: "staged.trust.browse_tree_and_manifests".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::StagedTrust,
        trust_stage_kind: M5TrustStageKind::BrowseTreeAndManifests,
        surface_context: M5StagingSurfaceContext::ShellSurface,
        resolution_form_coverage: M5StagingResolutionForm::ALL.to_vec(),
        browse_scope_ref: "browse-scope.acme/tree-manifests-docs".to_owned(),
        computed_metadata_ref: "metadata.acme/safe-computed".to_owned(),
        deferred_repo_action_set: "deferred-actions.acme/hooks-tasks-extensions".to_owned(),
        trust_prompt_policy: "trust-prompt.deferred.v3".to_owned(),
        explicit_approval_reference: "approval-ref.acme/handle-none".to_owned(),
        staged_trust_provenance: "staged-trust-provenance.acme.v3".to_owned(),
        bound_to_registry: true,
        browse_safe_before_widening: true,
        widens_trust_or_runs_code: false,
        explicit_approval_recorded: true,
        proof_fresh: true,
    }
}

fn clean_queue_input() -> M5PostOpenQueueEntryResolutionInput {
    M5PostOpenQueueEntryResolutionInput {
        entry_id: "queue:test".to_owned(),
        source_ref: "entry.acme.clone-remote".to_owned(),
        token_name: "post.open.queue.runs_repo_owned_code".to_owned(),
        semantic_role: M5RepositoryBootstrapRole::PostOpenQueue,
        queue_class: M5PostOpenQueueClass::RunsRepoOwnedCode,
        surface_context: M5StagingSurfaceContext::ShellSurface,
        resolution_form_coverage: M5StagingResolutionForm::ALL.to_vec(),
        queue_item_kind: "queue-item.repo-hook-or-task".to_owned(),
        execution_site: "site.worktree".to_owned(),
        trust_consequence: "consequence.widens-trust-runs-code".to_owned(),
        network_consequence: "consequence.offline".to_owned(),
        approval_requirement: "approval.explicit-required".to_owned(),
        attribution_ref: "attribution.acquisition-engine".to_owned(),
        identifies_run_site_and_consequence: true,
        item_is_truthfully_typed: true,
        is_protected_item: true,
        explicit_approval_or_policy_gated: true,
        schedules_deferred_followup: false,
        followup_is_disclosed: false,
        auto_executes_during_acquisition: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_PACKET_ID
    );
}

#[test]
fn staging_clean_names_meaning_and_is_bound() {
    let resolved = resolve_staged_trust_entry(clean_staging_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.staging_resolves_across_entry_flows);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.staged_trust_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.trust_stage_kind_is_classified);
    assert!(resolved.browse_safe_before_widening);
    assert_eq!(resolved.semantic_role, "staged_trust");
    assert_eq!(resolved.trust_stage_kind, "browse_tree_and_manifests");
    assert_eq!(
        resolved.canonical_trust_mode,
        "browse_tree_and_manifests_stage"
    );
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5StagingNextAction::ExpandStagingMeaning
    );
}

#[test]
fn staging_token_unstated_degrades() {
    let mut input = clean_staging_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::StageTokenUnstated)
    );
}

#[test]
fn staging_unbound_and_unclassified_degrade() {
    let mut input = clean_staging_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::StagingNotBoundToRegistry)
    );

    let mut input = clean_staging_input();
    input.trust_stage_kind = M5TrustStageKind::StageUnclassified;
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::TrustStageUnclassified)
    );
}

#[test]
fn staging_object_incomplete_and_widen_and_form_degrade() {
    // An unstated deferred repo-owned action set leaves the resolved object incomplete.
    let mut input = clean_staging_input();
    input.deferred_repo_action_set = "  ".to_owned();
    let resolved = resolve_staged_trust_entry(input).unwrap();
    assert!(!resolved.staged_trust_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::StagedTrustObjectIncomplete)
    );

    // A trust-widening stage with no recorded approval widens trust early and degrades.
    let mut input = clean_staging_input();
    input.trust_stage_kind = M5TrustStageKind::RunRepoOwnedActionAfterApproval;
    input.widens_trust_or_runs_code = true;
    input.explicit_approval_recorded = false;
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(
            M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly
        )
    );

    let mut input = clean_staging_input();
    input.resolution_form_coverage = vec![M5StagingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn staging_browse_unsafe_and_surface_and_proof_degrade() {
    let mut input = clean_staging_input();
    input.browse_safe_before_widening = false;
    // A stage that is not browse-safe before widening first fails browse-safety.
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(
            M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly
        )
    );

    let mut input = clean_staging_input();
    input.surface_context = M5StagingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_staging_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap().degrade_reason,
        Some(M5StagedTrustEntryDegradeReason::ProofStale)
    );
}

#[test]
fn staging_empty_id_and_forbidden_material_error() {
    let mut input = clean_staging_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap_err(),
        M5StagingResolutionError::EmptyStagedTrustEntryId
    );

    let mut input = clean_staging_input();
    input.staged_trust_provenance = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_staged_trust_entry(input).unwrap_err(),
        M5StagingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn staged_trust_stays_browse_safe_rejects_early_widening() {
    assert!(staged_trust_stays_browse_safe(
        M5TrustStageKind::BrowseTreeAndManifests,
        true,
        false,
        true
    ));
    assert!(!staged_trust_stays_browse_safe(
        M5TrustStageKind::BrowseTreeAndManifests,
        false,
        false,
        true
    ));
    assert!(staged_trust_stays_browse_safe(
        M5TrustStageKind::RunRepoOwnedActionAfterApproval,
        true,
        true,
        true
    ));
    assert!(!staged_trust_stays_browse_safe(
        M5TrustStageKind::RunRepoOwnedActionAfterApproval,
        true,
        true,
        false
    ));
    assert!(!staged_trust_stays_browse_safe(
        M5TrustStageKind::StageUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn staged_trust_object_is_complete_requires_all_fields() {
    assert!(staged_trust_object_is_complete(
        M5TrustStageKind::BrowseTreeAndManifests,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks-extensions",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ));
    assert!(!staged_trust_object_is_complete(
        M5TrustStageKind::BrowseTreeAndManifests,
        "browse-scope.acme/tree-manifests-docs",
        "  ",
        "deferred-actions.acme/hooks-tasks-extensions",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ));
    assert!(!staged_trust_object_is_complete(
        M5TrustStageKind::StageUnclassified,
        "browse-scope.acme/tree-manifests-docs",
        "metadata.acme/safe-computed",
        "deferred-actions.acme/hooks-tasks-extensions",
        "trust-prompt.deferred.v3",
        "approval-ref.acme/handle-none",
        "staged-trust-provenance.acme.v3",
    ));
}

#[test]
fn queue_clean_holds_for_approval() {
    let resolved = resolve_post_open_queue_entry(clean_queue_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.queue_safe_on_every_source);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_post_open_queue);
    assert!(resolved.post_open_queue_item_holds_for_approval);
    assert!(resolved.queue_class_is_protected);
    assert!(!resolved.auto_executes_during_acquisition);
    assert_eq!(resolved.queue_class, "runs_repo_owned_code");
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn queue_implicit_execution_and_unclassified_degrade() {
    // A protected item that auto-executes during acquisition breaks the queue.
    let mut input = clean_queue_input();
    input.auto_executes_during_acquisition = true;
    let resolved = resolve_post_open_queue_entry(input).unwrap();
    assert!(!resolved.provides_complete_post_open_queue);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence)
    );

    // A protected item that is not gated behind an explicit approval also breaks.
    let mut input = clean_queue_input();
    input.explicit_approval_or_policy_gated = false;
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence)
    );

    // A hidden run site / consequence also breaks the queue item.
    let mut input = clean_queue_input();
    input.identifies_run_site_and_consequence = false;
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence)
    );

    // An undisclosed scheduled follow-up also breaks the queue item.
    let mut input = clean_queue_input();
    input.schedules_deferred_followup = true;
    input.followup_is_disclosed = false;
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence)
    );

    let mut input = clean_queue_input();
    input.queue_class = M5PostOpenQueueClass::ClassUnclassified;
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::PostOpenQueueClassUnclassified)
    );
}

#[test]
fn queue_form_and_surface_and_id_and_material() {
    let mut input = clean_queue_input();
    input.resolution_form_coverage = vec![M5StagingResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::QueueFormCoverageIncomplete)
    );

    let mut input = clean_queue_input();
    input.surface_context = M5StagingSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap().degrade_reason,
        Some(M5PostOpenQueueEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_queue_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap_err(),
        M5StagingResolutionError::EmptyPostOpenQueueEntryId
    );

    let mut input = clean_queue_input();
    input.attribution_ref = "see internal://notes".to_owned();
    assert_eq!(
        resolve_post_open_queue_entry(input).unwrap_err(),
        M5StagingResolutionError::ForbiddenMaterial
    );
}

#[test]
fn queue_inert_and_gated_protected_stay_clean() {
    // An inert recommendation is not protected and stays clean without a gate.
    let mut input = clean_queue_input();
    input.queue_class = M5PostOpenQueueClass::InertRecommendation;
    input.is_protected_item = false;
    input.explicit_approval_or_policy_gated = false;
    let resolved = resolve_post_open_queue_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.queue_class_is_protected);

    // A protected item with a disclosed follow-up stays clean.
    let mut input = clean_queue_input();
    input.schedules_deferred_followup = true;
    input.followup_is_disclosed = true;
    assert!(resolve_post_open_queue_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_staged_trust_and_post_open_queue_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.vocabulary_set.trust_stage_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_CHECKOUT_PLAN_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::DomainSchemaRefMissing));

    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BOOTSTRAP_EVIDENCE_DOMAIN_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5StagingAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5StagingExportField::TrustStageKinds);
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0].post_open_queue_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    // Force a clean staging entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.staged_trust_entries[0].degrade_reason = None;
    row.staged_trust_entries[0].staged_trust_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.runs_repo_owned_actions_implicitly_during_acquisition = true,
            1 => row.auto_executes_post_open_bootstrap_queue_without_explicit_approval = true,
            2 => row.hides_what_a_queue_item_would_run_or_its_trust_or_network_consequence = true,
            _ => row.widens_trust_before_browse_safe_metadata_is_computed = true,
        }
        assert!(packet
            .validate()
            .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn staged_trust_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    for row in &mut packet.registry_rows {
        row.staged_trust_entries.retain(|ex| {
            ex.degrade_reason != Some(M5StagedTrustEntryDegradeReason::StagedTrustObjectIncomplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::StagedTrustResolutionNotProven));
}

#[test]
fn staged_trust_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    // Drop every clean admin-surface staging so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.staged_trust_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::StagedTrustResolutionNotProven));
}

#[test]
fn browse_safe_not_proven_when_widen_example_removed() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    for row in &mut packet.registry_rows {
        row.staged_trust_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5StagedTrustEntryDegradeReason::StagedTrustRunsRepoOwnedActionImplicitlyOrWidensTrustEarly,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::BrowseSafeStagingNotProven));
}

#[test]
fn browse_safe_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    for row in &mut packet.registry_rows {
        row.staged_trust_entries.retain(|ex| {
            ex.degrade_reason != Some(M5StagedTrustEntryDegradeReason::StagingNotBoundToRegistry)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::BrowseSafeStagingNotProven));
}

#[test]
fn post_open_queue_gating_not_proven_when_implicit_example_removed() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    for row in &mut packet.registry_rows {
        row.post_open_queue_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5PostOpenQueueEntryDegradeReason::PostOpenQueueItemExecutesImplicitlyOrHidesConsequence,
                )
        });
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::PostOpenQueueGatingNotProven));
}

#[test]
fn post_open_queue_gating_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    // Drop every clean mutates-reviewed-checkout item so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.post_open_queue_entries
            .retain(|ex| !(ex.is_clean() && ex.queue_class == "mutates_reviewed_checkout"));
    }
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::PostOpenQueueGatingNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet
        .governance_review
        .staged_trust_stays_browse_safe_no_implicit_repo_action = false;
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://clone.example/repo leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5StagedTrustPostOpenQueueRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_staged_trust_and_post_open_queue_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn post_open_queue_table_lists_only_clean_queue_items() {
    let packet = seeded_m5_staged_trust_and_post_open_queue_registries();
    let table = packet.render_post_open_queue_table();
    // The clean runs-code and hydrates-network queue items are rendered from the registry.
    assert!(table.contains("runs_repo_owned_code"));
    assert!(table.contains("hydrates_network_backed_content"));
    // A degraded, implicitly-executing entry never leaks into the generated table.
    assert!(!table.contains("implicit-execution"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_staged_trust_and_post_open_queue_registries_export()
        .expect("checked M5 staged-trust / post-open-queue registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_STAGED_TRUST_POST_OPEN_QUEUE_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_staged_trust_and_post_open_queue_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_staged_trust_and_post_open_queue_registries_deferred_hydrate_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::TrustService)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Beta
    );

    let preview =
        seeded_m5_staged_trust_and_post_open_queue_registries_trust_prompt_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5RepositoryBootstrapConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5StagedTrustPostOpenQueueRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-staged-trust-and-post-open-queue-registries/deferred_hydrate_beta_narrowed.json"
    )))
    .expect("deferred-hydrate fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_staged_trust_and_post_open_queue_registries_deferred_hydrate_beta_narrowed()
    );

    let preview: M5StagedTrustPostOpenQueueRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-staged-trust-and-post-open-queue-registries/trust_prompt_preview_narrowed.json"
    )))
    .expect("trust-prompt fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_staged_trust_and_post_open_queue_registries_trust_prompt_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_all_five_acquisition_verbs() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5RepositoryBootstrapFamily::OpenLocal,
            M5RepositoryBootstrapFamily::CloneRemote,
            M5RepositoryBootstrapFamily::OpenArchive,
            M5RepositoryBootstrapFamily::ImportBundle,
            M5RepositoryBootstrapFamily::ResumeSnapshot,
        ]
    );
}
