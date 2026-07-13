use super::*;

fn clean_restore_input() -> M5RestoreBoundsEntryResolutionInput {
    M5RestoreBoundsEntryResolutionInput {
        entry_id: "restore:test".to_owned(),
        token_name: "shell.restore.window.bounds".to_owned(),
        semantic_role: M5ShellGeometryRole::Responsive,
        responsive_geometry_role: M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        restore_surface_kind: M5RestoreSurfaceKind::RestorableWindow,
        topology_change: M5TopologyChange::MonitorDetach,
        surface_context: M5RemapSurfaceContext::Shell,
        fidelity_outcome: M5RemapFidelityOutcome::ExactBoundsRestored,
        reopens_fully_off_screen: false,
        traps_focus_after_remap: false,
        clamped_into_visible_bounds: true,
        preserves_usable_geometry: true,
        uses_absolute_coordinates_instead_of_intent: false,
        offers_recenter_reset_affordance: false,
        proof_fresh: true,
    }
}

fn clean_provenance_input() -> M5GeometryRemapProvenanceEntryResolutionInput {
    M5GeometryRemapProvenanceEntryResolutionInput {
        entry_id: "provenance:test".to_owned(),
        token_name: "shell.remap.provenance.exact".to_owned(),
        semantic_role: M5ShellGeometryRole::WorkspaceDominance,
        responsive_geometry_role: M5ResponsiveGeometryRole::PreservesRecoveryCriticalState,
        topology_change: M5TopologyChange::MonitorDetach,
        fidelity_outcome: M5RemapFidelityOutcome::ExactBoundsRestored,
        surface_context: M5RemapSurfaceContext::Shell,
        recorded_provenance_fields: M5RemapProvenanceField::ALL.to_vec(),
        preserves_workspace_focus_and_critical_state: true,
        records_remap_reason: true,
        silently_drops_workspace_or_state: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_PACKET_ID
    );
}

#[test]
fn restore_clean_preserves_on_screen_continuity() {
    let resolved = resolve_restore_bounds_entry(clean_restore_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.restore_preserves_on_screen_continuity);
    assert!(resolved.clamped_into_visible_bounds);
    assert!(resolved.preserves_usable_geometry);
    assert!(resolved.restore_surface_kind_is_classified);
    assert!(resolved.topology_change_is_classified);
    assert!(!resolved.reopens_fully_off_screen);
    assert!(!resolved.traps_focus_after_remap);
    assert!(!resolved.uses_absolute_coordinates_instead_of_intent);
    assert!(!resolved.responsive_role_drops_recovery_state);
    assert!(resolved.semantic_role_preserves_task_identity_under_collapse);
    assert_eq!(resolved.semantic_role, "responsive");
    assert_eq!(resolved.restore_surface_kind, "restorable_window");
    assert_eq!(resolved.topology_change, "monitor_detach");
    assert_eq!(resolved.fidelity_outcome, "exact_bounds_restored");
    assert!(!resolved.fidelity_is_reduced);
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5RestoreRegistryNextAction::ExpandRemapMeaning
    );
}

#[test]
fn dpi_change_and_canonical_sets_are_stable() {
    assert!(M5TopologyChange::DpiChange.is_dpi_change());
    assert!(!M5TopologyChange::MonitorAttach.is_dpi_change());
    assert_eq!(M5TopologyChange::CANONICAL_CHANGES.len(), 6);
    assert_eq!(M5RestoreSurfaceKind::CANONICAL_KINDS.len(), 5);
    assert_eq!(M5RemapFidelityOutcome::CANONICAL_OUTCOMES.len(), 4);
    assert!(M5RemapFidelityOutcome::ProportionalIntentRemap.is_reduced_fidelity());
    assert!(M5RemapFidelityOutcome::MonitorAffinityFallback.is_reduced_fidelity());
    assert!(M5RemapFidelityOutcome::RecenterReset.is_reduced_fidelity());
    assert!(!M5RemapFidelityOutcome::ExactBoundsRestored.is_reduced_fidelity());
}

#[test]
fn restore_token_and_context_and_kind_and_change_degrade() {
    let mut input = clean_restore_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::TokenUnstated)
    );

    let mut input = clean_restore_input();
    input.surface_context = M5RemapSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_restore_input();
    input.restore_surface_kind = M5RestoreSurfaceKind::KindUnclassified;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::SurfaceKindUnclassified)
    );

    let mut input = clean_restore_input();
    input.topology_change = M5TopologyChange::ChangeUnclassified;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::TopologyChangeUnclassified)
    );
}

#[test]
fn restore_off_screen_focus_clamp_and_geometry_degrade() {
    let mut input = clean_restore_input();
    input.reopens_fully_off_screen = true;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::ReopensFullyOffScreen)
    );

    let mut input = clean_restore_input();
    input.traps_focus_after_remap = true;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::TrapsFocusAfterRemap)
    );

    let mut input = clean_restore_input();
    input.clamped_into_visible_bounds = false;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::NotClampedIntoVisibleBounds)
    );

    let mut input = clean_restore_input();
    input.preserves_usable_geometry = false;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::LosesUsableGeometry)
    );

    let mut input = clean_restore_input();
    input.responsive_geometry_role =
        M5ResponsiveGeometryRole::ResponsiveChangeDropsRecoveryStateDisallowed;
    let resolved = resolve_restore_bounds_entry(input).unwrap();
    assert!(resolved.responsive_role_drops_recovery_state);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::LosesUsableGeometry)
    );
}

#[test]
fn restore_stale_coordinates_and_affordance_and_proof_degrade() {
    let mut input = clean_restore_input();
    input.uses_absolute_coordinates_instead_of_intent = true;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::ReplaysStaleAbsoluteCoordinates)
    );

    // Reduced fidelity without a recenter / reset affordance degrades.
    let mut input = clean_restore_input();
    input.fidelity_outcome = M5RemapFidelityOutcome::ProportionalIntentRemap;
    input.offers_recenter_reset_affordance = false;
    let resolved = resolve_restore_bounds_entry(input).unwrap();
    assert!(resolved.fidelity_is_reduced);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::NoRecenterResetAffordance)
    );

    // Reduced fidelity with the affordance is clean.
    let mut input = clean_restore_input();
    input.fidelity_outcome = M5RemapFidelityOutcome::RecenterReset;
    input.offers_recenter_reset_affordance = true;
    assert!(resolve_restore_bounds_entry(input).unwrap().is_clean());

    let mut input = clean_restore_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap().degrade_reason,
        Some(M5RestoreBoundsEntryDegradeReason::ProofStale)
    );
}

#[test]
fn restore_empty_id_and_forbidden_material_error() {
    let mut input = clean_restore_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap_err(),
        M5MonitorGeometryResolutionError::EmptyRestoreBoundsEntryId
    );

    let mut input = clean_restore_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_restore_bounds_entry(input).unwrap_err(),
        M5MonitorGeometryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn provenance_clean_is_diagnosable() {
    let resolved = resolve_geometry_remap_provenance_entry(clean_provenance_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.provenance_is_diagnosable);
    assert!(resolved.topology_change_is_classified);
    assert!(resolved.fidelity_outcome_is_classified);
    assert!(resolved.records_mandatory_provenance);
    assert!(resolved.preserves_workspace_focus_and_critical_state);
    assert!(resolved.records_remap_reason);
    assert!(!resolved.silently_drops_workspace_or_state);
    assert_eq!(resolved.topology_change, "monitor_detach");
    assert_eq!(resolved.fidelity_outcome, "exact_bounds_restored");
    assert_eq!(
        resolved.next_action,
        M5RestoreRegistryNextAction::TraceCanonicalRegistry
    );
}

#[test]
fn provenance_trigger_fidelity_and_silent_drop_degrade() {
    let mut input = clean_provenance_input();
    input.topology_change = M5TopologyChange::ChangeUnclassified;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::TriggerUnclassified)
    );

    let mut input = clean_provenance_input();
    input.fidelity_outcome = M5RemapFidelityOutcome::OutcomeUnclassified;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::FidelityOutcomeUnclassified)
    );

    let mut input = clean_provenance_input();
    input.silently_drops_workspace_or_state = true;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::SilentlyDropsWorkspaceOrState)
    );

    let mut input = clean_provenance_input();
    input.preserves_workspace_focus_and_critical_state = false;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::DropsWorkspaceFocusOrCriticalState)
    );
}

#[test]
fn provenance_reason_and_detail_and_proof_degrade() {
    let mut input = clean_provenance_input();
    input.records_remap_reason = false;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::RemapReasonUnrecorded)
    );

    let mut input = clean_provenance_input();
    input.recorded_provenance_fields = vec![M5RemapProvenanceField::RemapTrigger];
    let resolved = resolve_geometry_remap_provenance_entry(input).unwrap();
    assert!(!resolved.records_mandatory_provenance);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::ProvenanceDetailIncomplete)
    );

    let mut input = clean_provenance_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::ProofStale)
    );
}

#[test]
fn provenance_surface_and_id_and_material() {
    let mut input = clean_provenance_input();
    input.surface_context = M5RemapSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input)
            .unwrap()
            .degrade_reason,
        Some(M5GeometryRemapProvenanceEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_provenance_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input).unwrap_err(),
        M5MonitorGeometryResolutionError::EmptyRemapProvenanceEntryId
    );

    let mut input = clean_provenance_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_geometry_remap_provenance_entry(input).unwrap_err(),
        M5MonitorGeometryResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_monitor_geometry_remap_and_restore_bounds()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.vocabulary_set.topology_changes.pop();
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_DENSITY_MODE_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5RestoreRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5RestoreRegistryExportField::TopologyChanges);
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.registry_rows[0].remap_provenance_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    // Force a clean restore entry to also read as off-screen — reject it.
    let row = &mut packet.registry_rows[0];
    row.restore_bounds_entries[0].degrade_reason = None;
    row.restore_bounds_entries[0].reopens_fully_off_screen = true;
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.restore_reopens_off_screen_or_traps_focus = true,
            1 => row.remap_replays_stale_absolute_coordinates_without_clamp = true,
            2 => row.remap_silently_drops_workspace_focus_or_critical_state = true,
            _ => row.reduced_fidelity_without_recenter_or_provenance = true,
        }
        assert!(packet
            .validate()
            .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::RowInvariantViolated));
    }
}

#[test]
fn off_screen_continuity_not_proven_when_off_screen_example_removed() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.restore_bounds_entries.retain(|ex| {
            ex.degrade_reason != Some(M5RestoreBoundsEntryDegradeReason::ReopensFullyOffScreen)
        });
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::NoOffScreenOrFocusTrapNotProven
    ));
}

#[test]
fn off_screen_continuity_not_proven_when_change_dropped() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.restore_bounds_entries
            .retain(|ex| !(ex.is_clean() && ex.topology_change == "monitor_attach"));
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::NoOffScreenOrFocusTrapNotProven
    ));
}

#[test]
fn mixed_dpi_not_proven_when_loses_geometry_removed() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.restore_bounds_entries.retain(|ex| {
            ex.degrade_reason != Some(M5RestoreBoundsEntryDegradeReason::LosesUsableGeometry)
        });
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::MixedDpiUsableGeometryNotProven
    ));
}

#[test]
fn mixed_dpi_not_proven_when_dpi_clean_restore_removed() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.restore_bounds_entries
            .retain(|ex| !(ex.is_clean() && ex.topology_is_dpi_change));
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::MixedDpiUsableGeometryNotProven
    ));
}

#[test]
fn remap_provenance_not_proven_when_incomplete_removed() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.remap_provenance_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5GeometryRemapProvenanceEntryDegradeReason::ProvenanceDetailIncomplete)
        });
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::RemapProvenanceRecordedNotProven
    ));
}

#[test]
fn remap_provenance_not_proven_when_outcome_dropped() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    for row in &mut packet.registry_rows {
        row.remap_provenance_entries
            .retain(|ex| !(ex.is_clean() && ex.fidelity_outcome == "recenter_reset"));
    }
    assert!(packet.validate().contains(
        &M5MonitorGeometryRemapAndRestoreBoundsViolation::RemapProvenanceRecordedNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet
        .governance_review
        .mixed_dpi_and_topology_drills_preserve_usable_geometry = false;
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MonitorGeometryRemapAndRestoreBoundsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_monitor_geometry_remap_and_restore_bounds().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_monitor_geometry_remap_and_restore_bounds();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_monitor_geometry_remap_and_restore_bounds_export()
        .expect("checked M5 monitor-geometry restore export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_monitor_geometry_remap_and_restore_bounds(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_monitor_geometry_remap_and_restore_bounds_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(row.qualification, M5ShellGeometryQualificationClass::Beta);

    let preview =
        seeded_m5_monitor_geometry_remap_and_restore_bounds_settings_ui_preview_narrowed();
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
    let beta: M5MonitorGeometryRemapAndRestoreBoundsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-monitor-geometry-remap-and-restore-bounds/editor_ui_beta_narrowed.json"
        )
    ))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_monitor_geometry_remap_and_restore_bounds_editor_ui_beta_narrowed()
    );

    let preview: M5MonitorGeometryRemapAndRestoreBoundsPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-monitor-geometry-remap-and-restore-bounds/settings_ui_preview_narrowed.json"
        )
    ))
    .expect("settings-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_monitor_geometry_remap_and_restore_bounds_settings_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_responsive_geometry() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5ShellGeometryFamily::ResponsiveGeometry]
    );
}
