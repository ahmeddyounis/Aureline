use super::*;

fn clean_window_input() -> M5WindowClassEntryResolutionInput {
    let bounds = M5WindowClass::StandardDesktop.canonical_bounds();
    M5WindowClassEntryResolutionInput {
        entry_id: "window:test".to_owned(),
        token_name: "shell.responsive.standard.class".to_owned(),
        semantic_role: M5ShellGeometryRole::Responsive,
        responsive_geometry_role: M5ResponsiveGeometryRole::StandardClass,
        window_class: M5WindowClass::StandardDesktop,
        surface_context: M5ResponsiveSurfaceContext::Shell,
        min_width_px: bounds.min_width_px,
        max_width_px: bounds.max_width_px,
        coexisting_zones: M5ResponsiveShellZone::ALL.to_vec(),
        preserves_task_identity: true,
        preserves_recovery_critical_state: true,
        makes_essential_action_hover_only: false,
        narrows_editor_group_into_unusable_pane: false,
        proof_fresh: true,
    }
}

fn clean_collapse_input() -> M5CollapseStepEntryResolutionInput {
    M5CollapseStepEntryResolutionInput {
        entry_id: "collapse:test".to_owned(),
        token_name: "shell.collapse.inspector_detail.sheet".to_owned(),
        semantic_role: M5ShellGeometryRole::Collapse,
        collapse_priority_role: M5CollapsePriorityRole::CollapseOrderDeclared,
        collapse_target: M5CollapseTarget::OptionalRightInspectorDetail,
        transition_form: M5IdentityTransitionForm::Sheet,
        surface_context: M5ResponsiveSurfaceContext::Shell,
        collapses: true,
        declared_collapse_rank: 0,
        preserves_identity_state_and_keyboard_route: true,
        starves_main_workspace: false,
        uses_private_fracturing_width: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_PACKET_ID
    );
}

#[test]
fn window_clean_names_meaning_and_matches_canonical() {
    let resolved = resolve_window_class_entry(clean_window_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.responsive_change_preserves_task_identity);
    assert!(resolved.matches_canonical_bounds);
    assert!(resolved.covers_all_zones);
    assert!(resolved.preserves_task_identity);
    assert!(resolved.preserves_recovery_critical_state);
    assert!(resolved.window_class_is_classified);
    assert!(!resolved.responsive_role_drops_recovery_state);
    assert!(resolved.semantic_role_preserves_task_identity_under_collapse);
    assert_eq!(resolved.semantic_role, "responsive");
    assert_eq!(resolved.window_class, "standard_desktop");
    assert_eq!(resolved.min_width_px, 1280);
    assert_eq!(resolved.max_width_px, 1599);
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5ResponsiveRegistryNextAction::ExpandResponsiveMeaning
    );
}

#[test]
fn canonical_bounds_match_the_contract() {
    assert_eq!(
        M5WindowClass::CompactDesktop
            .canonical_bounds()
            .min_width_px,
        1024
    );
    assert_eq!(
        M5WindowClass::CompactDesktop
            .canonical_bounds()
            .max_width_px,
        1279
    );
    assert_eq!(
        M5WindowClass::StandardDesktop
            .canonical_bounds()
            .min_width_px,
        1280
    );
    assert_eq!(
        M5WindowClass::StandardDesktop
            .canonical_bounds()
            .max_width_px,
        1599
    );
    assert_eq!(
        M5WindowClass::ExpandedDesktop
            .canonical_bounds()
            .min_width_px,
        1600
    );
    assert_eq!(
        M5WindowClass::ExpandedDesktop
            .canonical_bounds()
            .max_width_px,
        EXPANDED_UPPER_BOUND_SENTINEL
    );
}

#[test]
fn window_token_unstated_degrades() {
    let mut input = clean_window_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::TokenUnstated)
    );
}

#[test]
fn window_class_and_recovery_and_identity_degrade() {
    let mut input = clean_window_input();
    input.window_class = M5WindowClass::ClassUnclassified;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::ClassUnclassified)
    );

    let mut input = clean_window_input();
    input.preserves_recovery_critical_state = false;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::DropsRecoveryCriticalState)
    );

    let mut input = clean_window_input();
    input.responsive_geometry_role =
        M5ResponsiveGeometryRole::ResponsiveChangeDropsRecoveryStateDisallowed;
    let resolved = resolve_window_class_entry(input).unwrap();
    assert!(resolved.responsive_role_drops_recovery_state);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WindowClassEntryDegradeReason::DropsRecoveryCriticalState)
    );

    let mut input = clean_window_input();
    input.preserves_task_identity = false;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::DropsTaskIdentity)
    );
}

#[test]
fn window_hover_only_and_unusable_pane_and_bounds_and_zone_degrade() {
    let mut input = clean_window_input();
    input.makes_essential_action_hover_only = true;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::EssentialActionBecomesHoverOnly)
    );

    let mut input = clean_window_input();
    input.narrows_editor_group_into_unusable_pane = true;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::EditorGroupNarrowsIntoUnusablePane)
    );

    // A width that stays plausible but does not match the canonical class bounds degrades as a private
    // breakpoint.
    let mut input = clean_window_input();
    input.max_width_px = 1650;
    let resolved = resolve_window_class_entry(input).unwrap();
    assert!(!resolved.matches_canonical_bounds);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5WindowClassEntryDegradeReason::BoundsOutsideCanonicalClass)
    );

    let mut input = clean_window_input();
    input.coexisting_zones = vec![M5ResponsiveShellZone::MainWorkspace];
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::ShellZoneCoexistenceIncomplete)
    );
}

#[test]
fn window_surface_and_proof_degrade() {
    let mut input = clean_window_input();
    input.surface_context = M5ResponsiveSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_window_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_window_class_entry(input).unwrap().degrade_reason,
        Some(M5WindowClassEntryDegradeReason::ProofStale)
    );
}

#[test]
fn window_empty_id_and_forbidden_material_error() {
    let mut input = clean_window_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_window_class_entry(input).unwrap_err(),
        M5ResponsiveResolutionError::EmptyWindowClassEntryId
    );

    let mut input = clean_window_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_window_class_entry(input).unwrap_err(),
        M5ResponsiveResolutionError::ForbiddenMaterial
    );
}

#[test]
fn collapse_clean_is_identity_stable() {
    let resolved = resolve_collapse_step_entry(clean_collapse_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.transition_is_identity_stable);
    assert!(resolved.target_is_classified);
    assert!(resolved.form_is_classified);
    assert!(resolved.keeps_main_workspace_dominant);
    assert!(resolved.matches_canonical_order);
    assert!(!resolved.target_is_protected);
    assert!(!resolved.hides_primary_workflow_behind_overlay_only);
    assert_eq!(resolved.collapse_target, "optional_right_inspector_detail");
    assert_eq!(resolved.canonical_collapse_rank, Some(0));
    assert_eq!(
        resolved.next_action,
        M5ResponsiveRegistryNextAction::TraceCanonicalRegistry
    );
}

#[test]
fn collapse_protected_dominant_is_clean() {
    let mut input = clean_collapse_input();
    input.collapse_target = M5CollapseTarget::EditorWorkspace;
    input.collapse_priority_role = M5CollapsePriorityRole::MainWorkspaceStaysDominant;
    input.transition_form = M5IdentityTransitionForm::Docked;
    input.collapses = false;
    input.declared_collapse_rank = 0;
    let resolved = resolve_collapse_step_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.target_is_protected);
    assert_eq!(resolved.canonical_collapse_rank, None);
}

#[test]
fn collapse_target_form_and_protected_and_identity_degrade() {
    let mut input = clean_collapse_input();
    input.collapse_target = M5CollapseTarget::TargetUnclassified;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::TargetUnclassified)
    );

    let mut input = clean_collapse_input();
    input.transition_form = M5IdentityTransitionForm::FormUnclassified;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::FormUnclassified)
    );

    let mut input = clean_collapse_input();
    input.collapse_priority_role =
        M5CollapsePriorityRole::PrivateWidthThatFracturesLayoutDisallowed;
    let resolved = resolve_collapse_step_entry(input).unwrap();
    assert!(resolved.collapse_role_fractures_layout);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::FracturesLayoutWithPrivateWidth)
    );

    let mut input = clean_collapse_input();
    input.collapse_target = M5CollapseTarget::EditorWorkspace;
    input.collapses = true;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::CollapsesProtectedTarget)
    );

    let mut input = clean_collapse_input();
    input.preserves_identity_state_and_keyboard_route = false;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::DropsIdentityStateOrRoute)
    );
}

#[test]
fn collapse_starve_overlay_and_order_degrade() {
    let mut input = clean_collapse_input();
    input.starves_main_workspace = true;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::StarvesMainWorkspace)
    );

    let mut input = clean_collapse_input();
    input.collapse_target = M5CollapseTarget::PrimaryNavigation;
    input.declared_collapse_rank = 3;
    input.transition_form = M5IdentityTransitionForm::Overlay;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::OverlayOnlyPrimaryFallback)
    );

    let mut input = clean_collapse_input();
    input.declared_collapse_rank = 2;
    let resolved = resolve_collapse_step_entry(input).unwrap();
    assert!(!resolved.matches_canonical_order);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::CollapseOrderOutsideCanonical)
    );
}

#[test]
fn collapse_surface_and_id_and_material() {
    let mut input = clean_collapse_input();
    input.surface_context = M5ResponsiveSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap().degrade_reason,
        Some(M5CollapseStepEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_collapse_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap_err(),
        M5ResponsiveResolutionError::EmptyCollapseStepEntryId
    );

    let mut input = clean_collapse_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_collapse_step_entry(input).unwrap_err(),
        M5ResponsiveResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(
        seeded_m5_responsive_geometry_and_collapse_priority_registries()
            .vocabulary_set
            .matches_canonical()
    );
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.vocabulary_set.window_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.source_contract_refs.clear();
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MissingSourceContracts
    ));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DENSITY_MODE_SCHEMA_REF);
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::DomainSchemaRefMissing
    ));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ResponsiveRegistryAnatomyPart::Identity);
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MandatoryAnatomyMissing
    ));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ResponsiveRegistryExportField::WindowClasses);
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MandatoryExportFieldMissing
    ));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.registry_rows[0].collapse_step_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    // Force a clean window-class entry to also read as hover-only — reject it.
    let row = &mut packet.registry_rows[0];
    row.window_class_entries[0].degrade_reason = None;
    row.window_class_entries[0].makes_essential_action_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.responsive_or_collapse_alters_command_focus_or_trust = true,
            1 => row.extension_sets_private_fracturing_width = true,
            2 => row.lets_zone_starve_main_workspace_below_minimum = true,
            _ => row.hides_primary_workflow_behind_overlay_only_fallback = true,
        }
        assert!(packet.validate().contains(
            &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::RowInvariantViolated
        ));
    }
}

#[test]
fn window_classes_not_proven_when_bounds_drift_example_removed() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    for row in &mut packet.registry_rows {
        row.window_class_entries.retain(|ex| {
            ex.degrade_reason != Some(M5WindowClassEntryDegradeReason::BoundsOutsideCanonicalClass)
        });
    }
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ResponsiveWindowClassesAcrossSurfacesNotProven
    ));
}

#[test]
fn window_classes_not_proven_when_class_dropped() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    for row in &mut packet.registry_rows {
        row.window_class_entries
            .retain(|ex| !(ex.is_clean() && ex.window_class == "expanded_desktop"));
    }
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ResponsiveWindowClassesAcrossSurfacesNotProven
    ));
}

#[test]
fn identity_stable_not_proven_when_drops_identity_removed() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    for row in &mut packet.registry_rows {
        row.collapse_step_entries.retain(|ex| {
            ex.degrade_reason != Some(M5CollapseStepEntryDegradeReason::DropsIdentityStateOrRoute)
        });
    }
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::IdentityStableTransitionsNotProven
    ));
}

#[test]
fn identity_stable_not_proven_when_hover_only_removed() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    for row in &mut packet.registry_rows {
        row.window_class_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5WindowClassEntryDegradeReason::EssentialActionBecomesHoverOnly)
        });
    }
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::IdentityStableTransitionsNotProven
    ));
}

#[test]
fn extension_degradation_not_proven_when_starve_removed() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    for row in &mut packet.registry_rows {
        row.collapse_step_entries.retain(|ex| {
            ex.degrade_reason != Some(M5CollapseStepEntryDegradeReason::StarvesMainWorkspace)
        });
    }
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ExtensionCannotFracturePrivateWidthNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet
        .governance_review
        .responsive_preserves_task_identity_and_recovery_state = false;
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::GovernanceReviewIncomplete
    ));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ProofFreshnessIncomplete
    ));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ReleasePostureIncomplete
    ));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet.validate().contains(
        &M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::RawMaterialInExport
    ));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_responsive_geometry_and_collapse_priority_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_responsive_geometry_and_collapse_priority_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_responsive_geometry_and_collapse_priority_registries_export()
        .expect("checked M5 responsive-geometry registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_responsive_geometry_and_collapse_priority_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta =
        seeded_m5_responsive_geometry_and_collapse_priority_registries_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(row.qualification, M5ShellGeometryQualificationClass::Beta);

    let preview =
        seeded_m5_responsive_geometry_and_collapse_priority_registries_settings_ui_preview_narrowed(
        );
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::SettingsUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5ShellGeometryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-responsive-geometry-and-collapse-priority-registries/editor_ui_beta_narrowed.json"
        )))
        .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_responsive_geometry_and_collapse_priority_registries_editor_ui_beta_narrowed()
    );

    let preview: M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-responsive-geometry-and-collapse-priority-registries/settings_ui_preview_narrowed.json"
        )))
        .expect("settings-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_responsive_geometry_and_collapse_priority_registries_settings_ui_preview_narrowed(
        )
    );
}

#[test]
fn implemented_families_are_responsive_and_collapse() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5ShellGeometryFamily::ResponsiveGeometry,
            M5ShellGeometryFamily::CollapsePriority
        ]
    );
}
