use super::*;

fn clean_skeleton_input() -> M5SkeletonRestoreEntryResolutionInput {
    M5SkeletonRestoreEntryResolutionInput {
        entry_id: "skeleton:test".to_owned(),
        restore_target_id: "restore.acme.warm".to_owned(),
        token_name: "restore.skeleton.live".to_owned(),
        semantic_role: M5WindowRestoreRole::LayoutSkeleton,
        restore_fidelity_class: M5RestoreFidelityClass::LiveHydratedPane,
        surface_context: M5RestoreOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5RestoreOrchestrationResolutionForm::ALL.to_vec(),
        window_shell_id: "window-shell.main".to_owned(),
        pane_tree_structure: "pane-tree.main.v3".to_owned(),
        pane_role_set: "pane-roles.editor|terminal|preview".to_owned(),
        placeholder_set: "placeholders.none.0007".to_owned(),
        layout_skeleton_root: "layout-skeleton.acme/warm".to_owned(),
        hydration_plan_ref: "hydration-plan.inline".to_owned(),
        bound_to_registry: true,
        skeleton_rebuilt_before_hydration: true,
        defers_heavy_hydration: false,
        pane_roles_preserved_when_deferred: true,
        proof_fresh: true,
    }
}

fn clean_hydration_input() -> M5SessionHydrationEntryResolutionInput {
    M5SessionHydrationEntryResolutionInput {
        entry_id: "hydration:test".to_owned(),
        pane_id: "pane.terminal.main".to_owned(),
        token_name: "hydration.terminal.no_rerun".to_owned(),
        semantic_role: M5WindowRestoreRole::SessionHydration,
        hydration_surface: M5SessionHydrationSurface::TerminalOrRemoteShellHydration,
        surface_context: M5RestoreOrchestrationSurfaceContext::ShellSurface,
        resolution_form_coverage: M5RestoreOrchestrationResolutionForm::ALL.to_vec(),
        preserved_pane_role: "pane-role.terminal.main".to_owned(),
        missing_dependency_class: "dependency.present".to_owned(),
        restore_fidelity_hint: "restore-fidelity.live".to_owned(),
        preserves_pane_role_and_topology: true,
        hydration_is_truthful: true,
        dependency_missing_used: false,
        placeholder_substituted_on_missing: false,
        heavy_dependency_deferred: false,
        deferred_fidelity_disclosed: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_PACKET_ID
    );
}

#[test]
fn skeleton_clean_names_meaning_and_is_bound() {
    let resolved = resolve_skeleton_restore_entry(clean_skeleton_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.skeleton_resolves_across_restores);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.restore_skeleton_object_complete);
    assert!(resolved.bound_to_registry);
    assert!(resolved.restore_fidelity_class_is_classified);
    assert!(resolved.skeleton_rebuilt_before_hydration);
    assert_eq!(resolved.semantic_role, "layout_skeleton");
    assert_eq!(resolved.restore_fidelity_class, "live_hydrated_pane");
    assert_eq!(resolved.canonical_restore_fidelity_mode, "live_hydrated");
    assert_eq!(resolved.surface_context, "shell_surface");
    assert_eq!(
        resolved.next_action,
        M5RestoreOrchestrationNextAction::ExpandRestoreMeaning
    );
}

#[test]
fn skeleton_token_unstated_degrades() {
    let mut input = clean_skeleton_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::SkeletonTokenUnstated)
    );
}

#[test]
fn skeleton_unbound_and_unclassified_degrade() {
    let mut input = clean_skeleton_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::SkeletonNotBoundToRegistry)
    );

    let mut input = clean_skeleton_input();
    input.restore_fidelity_class = M5RestoreFidelityClass::FidelityUnclassified;
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::RestoreFidelityClassUnclassified)
    );
}

#[test]
fn skeleton_object_incomplete_and_hydration_first_and_form_degrade() {
    // An unstated pane-tree structure leaves the resolved object incomplete.
    let mut input = clean_skeleton_input();
    input.pane_tree_structure = "  ".to_owned();
    let resolved = resolve_skeleton_restore_entry(input).unwrap();
    assert!(!resolved.restore_skeleton_object_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::RestoreSkeletonObjectIncomplete)
    );

    // Heavy hydration that ran before the skeleton was rebuilt degrades.
    let mut input = clean_skeleton_input();
    input.skeleton_rebuilt_before_hydration = false;
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton)
    );

    let mut input = clean_skeleton_input();
    input.resolution_form_coverage = vec![M5RestoreOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::ResolutionFormCoverageIncomplete)
    );
}

#[test]
fn skeleton_deferred_roles_and_surface_and_proof_degrade() {
    let mut input = clean_skeleton_input();
    input.restore_fidelity_class = M5RestoreFidelityClass::PaneRolePlaceholder;
    input.defers_heavy_hydration = true;
    input.pane_roles_preserved_when_deferred = false;
    // A deferred-hydration skeleton losing its pane roles first fails skeleton-precedes-hydration.
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton)
    );

    let mut input = clean_skeleton_input();
    input.surface_context = M5RestoreOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_skeleton_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_skeleton_restore_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SkeletonRestoreEntryDegradeReason::ProofStale)
    );
}

#[test]
fn skeleton_empty_id_and_forbidden_material_error() {
    let mut input = clean_skeleton_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_skeleton_restore_entry(input).unwrap_err(),
        M5RestoreOrchestrationResolutionError::EmptySkeletonEntryId
    );

    let mut input = clean_skeleton_input();
    input.layout_skeleton_root = "see https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_skeleton_restore_entry(input).unwrap_err(),
        M5RestoreOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn skeleton_precedes_hydration_rejects_hydration_first() {
    assert!(skeleton_precedes_hydration(
        M5RestoreFidelityClass::LiveHydratedPane,
        true,
        false,
        true
    ));
    assert!(!skeleton_precedes_hydration(
        M5RestoreFidelityClass::LiveHydratedPane,
        false,
        false,
        true
    ));
    assert!(skeleton_precedes_hydration(
        M5RestoreFidelityClass::PaneRolePlaceholder,
        true,
        true,
        true
    ));
    assert!(!skeleton_precedes_hydration(
        M5RestoreFidelityClass::PaneRolePlaceholder,
        true,
        true,
        false
    ));
    assert!(!skeleton_precedes_hydration(
        M5RestoreFidelityClass::FidelityUnclassified,
        true,
        false,
        true
    ));
}

#[test]
fn restore_skeleton_object_is_complete_requires_all_fields() {
    assert!(restore_skeleton_object_is_complete(
        M5RestoreFidelityClass::LiveHydratedPane,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    ));
    assert!(!restore_skeleton_object_is_complete(
        M5RestoreFidelityClass::LiveHydratedPane,
        "window-shell.main",
        "  ",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    ));
    assert!(!restore_skeleton_object_is_complete(
        M5RestoreFidelityClass::FidelityUnclassified,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    ));
}

#[test]
fn hydration_clean_stays_no_rerun() {
    let resolved = resolve_session_hydration_entry(clean_hydration_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.hydration_no_rerun_on_every_pane);
    assert!(resolved.covers_all_resolution_forms);
    assert!(resolved.provides_complete_disclosure_triple);
    assert!(resolved.hydration_stays_no_rerun);
    assert_eq!(
        resolved.hydration_surface,
        "terminal_or_remote_shell_hydration"
    );
    assert_eq!(resolved.surface_context, "shell_surface");
}

#[test]
fn hydration_collapses_and_unclassified_degrade() {
    // A missing dependency that is not placeholder-substituted collapses the layout.
    let mut input = clean_hydration_input();
    input.dependency_missing_used = true;
    input.placeholder_substituted_on_missing = false;
    let resolved = resolve_session_hydration_entry(input).unwrap();
    assert!(!resolved.provides_complete_disclosure_triple);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout)
    );

    // A hydration that no longer preserves the pane role and topology is also a rerun / collapse.
    let mut input = clean_hydration_input();
    input.preserves_pane_role_and_topology = false;
    assert_eq!(
        resolve_session_hydration_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout)
    );

    // An overclaimed deferred fidelity is also a rerun / collapse.
    let mut input = clean_hydration_input();
    input.heavy_dependency_deferred = true;
    input.deferred_fidelity_disclosed = false;
    assert_eq!(
        resolve_session_hydration_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout)
    );

    let mut input = clean_hydration_input();
    input.hydration_surface = M5SessionHydrationSurface::HydrationSurfaceUnclassified;
    assert_eq!(
        resolve_session_hydration_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationSurfaceUnclassified)
    );
}

#[test]
fn hydration_form_and_surface_and_id_and_material() {
    let mut input = clean_hydration_input();
    input.resolution_form_coverage = vec![M5RestoreOrchestrationResolutionForm::CanonicalObject];
    assert_eq!(
        resolve_session_hydration_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::HydrationFormCoverageIncomplete)
    );

    let mut input = clean_hydration_input();
    input.surface_context = M5RestoreOrchestrationSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_session_hydration_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5SessionHydrationEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_hydration_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_session_hydration_entry(input).unwrap_err(),
        M5RestoreOrchestrationResolutionError::EmptySessionHydrationEntryId
    );

    let mut input = clean_hydration_input();
    input.preserved_pane_role = "see internal://notes".to_owned();
    assert_eq!(
        resolve_session_hydration_entry(input).unwrap_err(),
        M5RestoreOrchestrationResolutionError::ForbiddenMaterial
    );
}

#[test]
fn hydration_placeholder_and_disclosed_defer_stay_clean() {
    // A placeholder-substituted missing dependency stays no-rerun.
    let mut input = clean_hydration_input();
    input.dependency_missing_used = true;
    input.placeholder_substituted_on_missing = true;
    assert!(resolve_session_hydration_entry(input).unwrap().is_clean());

    // A disclosed deferred heavy dependency stays no-rerun.
    let mut input = clean_hydration_input();
    input.heavy_dependency_deferred = true;
    input.deferred_fidelity_disclosed = true;
    assert!(resolve_session_hydration_entry(input).unwrap().is_clean());
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_skeleton_first_restore_and_session_hydration_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.vocabulary_set.restore_fidelity_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_RESTORE_FIDELITY_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::DomainSchemaRefMissing
    ));

    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RestoreOrchestrationAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5RestoreOrchestrationExportField::RestoreFidelityClasses);
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0].session_hydration_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    // Force a clean skeleton entry to also read as object-incomplete — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.skeleton_restore_entries[0].degrade_reason = None;
    row.skeleton_restore_entries[0].restore_skeleton_object_complete = false;
    assert!(packet
        .validate()
        .contains(&M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => {
                row.reruns_session_scoped_work_or_reattaches_privileged_sessions_during_restore =
                    true
            }
            1 => row.deletes_layout_structure_silently_on_missing_dependency = true,
            2 => row.merges_skeleton_and_hydration_into_one_opaque_blob = true,
            _ => row.overclaims_restore_fidelity_when_only_context_or_evidence_reopened = true,
        }
        assert!(packet.validate().contains(
            &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn skeleton_object_not_proven_when_incomplete_example_removed() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    for row in &mut packet.registry_rows {
        row.skeleton_restore_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SkeletonRestoreEntryDegradeReason::RestoreSkeletonObjectIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonObjectResolutionNotProven
    ));
}

#[test]
fn skeleton_object_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    // Drop every clean admin skeleton so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.skeleton_restore_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "admin_surface"));
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonObjectResolutionNotProven
    ));
}

#[test]
fn skeleton_before_hydration_not_proven_when_hydration_first_example_removed() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    for row in &mut packet.registry_rows {
        row.skeleton_restore_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton)
        });
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonBeforeHydrationNotProven
    ));
}

#[test]
fn skeleton_before_hydration_not_proven_when_unbound_example_removed() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    for row in &mut packet.registry_rows {
        row.skeleton_restore_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5SkeletonRestoreEntryDegradeReason::SkeletonNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonBeforeHydrationNotProven
    ));
}

#[test]
fn placeholder_continuity_not_proven_when_collapse_example_removed() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    for row in &mut packet.registry_rows {
        row.session_hydration_entries.retain(|ex| {
            ex.degrade_reason
                != Some(
                    M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout,
                )
        });
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::PaneRolePlaceholderContinuityNotProven
    ));
}

#[test]
fn placeholder_continuity_not_proven_when_surface_dropped() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    // Drop every clean preview hydration so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.session_hydration_entries.retain(|ex| {
            !(ex.is_clean() && ex.hydration_surface == "preview_or_collaboration_hydration")
        });
    }
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::PaneRolePlaceholderContinuityNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet
        .governance_review
        .skeleton_rebuilt_before_heavy_hydration = false;
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json =
        seeded_m5_skeleton_first_restore_and_session_hydration_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn restore_fidelity_table_lists_only_clean_skeletons() {
    let packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    let table = packet.render_restore_fidelity_table();
    // The clean live and placeholder skeletons are rendered from the registry.
    assert!(table.contains("live_hydrated"));
    assert!(table.contains("pane_role_placeholder"));
    // A degraded, incomplete entry never leaks into the generated table.
    assert!(!table.contains("incomplete"));
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk =
        current_stable_m5_skeleton_first_restore_and_session_hydration_registries_export()
            .expect("checked M5 skeleton-restore / session-hydration registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_skeleton_first_restore_and_session_hydration_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .unwrap();
    assert_eq!(row.qualification, M5WindowRestoreQualificationClass::Beta);

    let preview =
        seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5WindowRestoreQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5SkeletonFirstRestoreSessionHydrationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-skeleton-first-restore-and-session-hydration-registries/placeholder_pane_continuity_beta_narrowed.json"
    )))
    .expect("placeholder-pane fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed()
    );

    let preview: M5SkeletonFirstRestoreSessionHydrationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-skeleton-first-restore-and-session-hydration-registries/context_only_hydration_preview_narrowed.json"
    )))
    .expect("context-only fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed()
    );
}

#[test]
fn implemented_families_is_skeleton_first_restore_and_no_rerun_session_hydration() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5WindowRestoreFamily::SkeletonFirstRestore,
            M5WindowRestoreFamily::NoRerunSessionHydration,
        ]
    );
}
