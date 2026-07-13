use super::*;

fn clean_metric_input() -> M5ShellMetricEntryResolutionInput {
    let bounds = M5ShellZone::Sidebar.canonical_bounds();
    M5ShellMetricEntryResolutionInput {
        entry_id: "metric:test".to_owned(),
        token_name: "shell.metric.sidebar.default".to_owned(),
        semantic_role: M5ShellGeometryRole::Zone,
        metric_role: M5ShellMetricRole::BoundToRegistry,
        zone: M5ShellZone::Sidebar,
        surface_context: M5ShellSurfaceContext::Shell,
        density_coverage: M5ShellDensityMode::ALL.to_vec(),
        minimum_px: bounds.minimum_px,
        default_px: bounds.default_px,
        recommended_px: bounds.recommended_px,
        maximum_px: bounds.maximum_px,
        bound_to_registry: true,
        starves_main_workspace: false,
        preserves_task_identity_under_snapped_width: true,
        proof_fresh: true,
    }
}

fn clean_minimum_input() -> M5MinimumSizeEntryResolutionInput {
    M5MinimumSizeEntryResolutionInput {
        entry_id: "minimum:test".to_owned(),
        token_name: "shell.minimum.tab.width".to_owned(),
        minimum_size_role: M5MinimumSizeRole::TabMinimumWidth,
        semantic_role: M5ShellGeometryRole::HitTarget,
        control: M5ShellControlClass::Tab,
        surface_context: M5ShellSurfaceContext::Shell,
        density_coverage: M5ShellDensityMode::ALL.to_vec(),
        declared_minimum_px: 96,
        pointer_and_keyboard_reachable: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_shell_metric_minimum_size_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHELL_METRIC_REGISTRIES_PACKET_ID);
}

#[test]
fn metric_clean_names_meaning_and_is_bound() {
    let resolved = resolve_shell_metric_entry(clean_metric_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.metric_holds_across_density_and_snapped_widths);
    assert!(resolved.covers_all_density_modes);
    assert!(resolved.within_canonical_envelope);
    assert!(resolved.bound_to_registry);
    assert!(resolved.zone_is_classified);
    assert!(!resolved.zone_is_workspace_dominant);
    assert!(!resolved.metric_role_hand_copied);
    assert_eq!(resolved.semantic_role, "zone");
    assert_eq!(resolved.zone, "sidebar");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5ShellRegistryNextAction::ExpandMetricMeaning
    );
}

#[test]
fn metric_token_unstated_degrades() {
    let mut input = clean_metric_input();
    input.token_name = "   ".to_owned();
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::MetricTokenUnstated)
    );
}

#[test]
fn metric_hand_copied_and_unclassified_degrade() {
    let mut input = clean_metric_input();
    input.metric_role = M5ShellMetricRole::HandCopiedConstantDisallowed;
    let resolved = resolve_shell_metric_entry(input).unwrap();
    assert!(resolved.metric_role_hand_copied);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::MetricNotBoundToRegistry)
    );

    let mut input = clean_metric_input();
    input.bound_to_registry = false;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::MetricNotBoundToRegistry)
    );

    let mut input = clean_metric_input();
    input.zone = M5ShellZone::ZoneUnclassified;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::ZoneUnclassified)
    );
}

#[test]
fn metric_outside_envelope_and_starve_and_density_degrade() {
    let mut input = clean_metric_input();
    input.minimum_px = 180;
    let resolved = resolve_shell_metric_entry(input).unwrap();
    assert!(!resolved.within_canonical_envelope);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::MetricOutsideCanonicalEnvelope)
    );

    let mut input = clean_metric_input();
    input.starves_main_workspace = true;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::ZoneStarvesMainWorkspace)
    );

    let mut input = clean_metric_input();
    input.density_coverage = vec![M5ShellDensityMode::Comfortable];
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::DensityCoverageIncomplete)
    );
}

#[test]
fn metric_snapped_and_surface_and_proof_degrade() {
    let mut input = clean_metric_input();
    input.preserves_task_identity_under_snapped_width = false;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::SnappedWidthUnsafe)
    );

    let mut input = clean_metric_input();
    input.surface_context = M5ShellSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_metric_input();
    input.proof_fresh = false;
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap().degrade_reason,
        Some(M5ShellMetricEntryDegradeReason::ProofStale)
    );
}

#[test]
fn metric_empty_id_and_forbidden_material_error() {
    let mut input = clean_metric_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap_err(),
        M5ShellMetricResolutionError::EmptyShellMetricEntryId
    );

    let mut input = clean_metric_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_shell_metric_entry(input).unwrap_err(),
        M5ShellMetricResolutionError::ForbiddenMaterial
    );
}

#[test]
fn metric_editor_group_is_dominant() {
    let bounds = M5ShellZone::MainEditorGroup.canonical_bounds();
    let mut input = clean_metric_input();
    input.zone = M5ShellZone::MainEditorGroup;
    input.semantic_role = M5ShellGeometryRole::WorkspaceDominance;
    input.token_name = "shell.metric.main_editor_group.minimum".to_owned();
    input.minimum_px = bounds.minimum_px;
    input.default_px = bounds.default_px;
    input.recommended_px = bounds.recommended_px;
    input.maximum_px = bounds.maximum_px;
    let resolved = resolve_shell_metric_entry(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.zone_is_workspace_dominant);
    assert_eq!(resolved.minimum_px, 420);
}

#[test]
fn minimum_clean_stays_above_minimum() {
    let resolved = resolve_minimum_size_entry(clean_minimum_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.hit_target_holds_across_density);
    assert!(resolved.covers_all_density_modes);
    assert!(resolved.meets_supported_minimum);
    assert!(!resolved.minimum_size_role_shrinks_below_minimum);
    assert_eq!(resolved.control, "tab");
    assert_eq!(resolved.canonical_minimum_px, 96);
    assert_eq!(resolved.surface_context, "shell");
}

#[test]
fn minimum_below_minimum_and_control_unclassified_degrade() {
    let mut input = clean_minimum_input();
    input.declared_minimum_px = 72;
    let resolved = resolve_minimum_size_entry(input).unwrap();
    assert!(!resolved.meets_supported_minimum);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
    );

    let mut input = clean_minimum_input();
    input.minimum_size_role = M5MinimumSizeRole::ShrinksBelowMinimumDisallowed;
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap().degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
    );

    let mut input = clean_minimum_input();
    input.pointer_and_keyboard_reachable = false;
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap().degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
    );

    let mut input = clean_minimum_input();
    input.control = M5ShellControlClass::ControlUnclassified;
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap().degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::ControlUnclassified)
    );
}

#[test]
fn minimum_density_and_surface_and_id_and_material() {
    let mut input = clean_minimum_input();
    input.density_coverage = vec![M5ShellDensityMode::Standard];
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap().degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::DensityCoverageIncomplete)
    );

    let mut input = clean_minimum_input();
    input.surface_context = M5ShellSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap().degrade_reason,
        Some(M5MinimumSizeEntryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_minimum_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap_err(),
        M5ShellMetricResolutionError::EmptyMinimumSizeEntryId
    );

    let mut input = clean_minimum_input();
    input.token_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_minimum_size_entry(input).unwrap_err(),
        M5ShellMetricResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_shell_metric_minimum_size_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.vocabulary_set.shell_zones.pop();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_SHELL_METRICS_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ShellRegistryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5ShellRegistryExportField::ShellZones);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.registry_rows[0].minimum_size_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    // Force a clean shell-metric entry to also read as workspace-starving — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.shell_metric_entries[0].degrade_reason = None;
    row.shell_metric_entries[0].starves_main_workspace = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_shell_metric_minimum_size_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.lets_zone_starve_main_workspace_below_minimum = true,
            1 => row.shrinks_hit_target_below_supported_minimum = true,
            2 => row.extension_or_embedded_sets_private_fracturing_width = true,
            _ => row.metric_hand_copied_instead_of_registry = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ShellMetricRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn first_consumers_not_proven_when_hand_copied_example_removed() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    for row in &mut packet.registry_rows {
        row.shell_metric_entries.retain(|ex| {
            ex.degrade_reason != Some(M5ShellMetricEntryDegradeReason::MetricNotBoundToRegistry)
        });
    }
    assert!(packet.validate().contains(
        &M5ShellMetricRegistriesViolation::FirstConsumersResolveFromSharedRegistryNotProven
    ));
}

#[test]
fn first_consumers_not_proven_when_surface_collapses() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    // Drop every clean notebook metric so the first-consumer surfaces no longer include it.
    for row in &mut packet.registry_rows {
        row.shell_metric_entries
            .retain(|ex| !(ex.is_clean() && ex.surface_context == "notebook"));
    }
    assert!(packet.validate().contains(
        &M5ShellMetricRegistriesViolation::FirstConsumersResolveFromSharedRegistryNotProven
    ));
}

#[test]
fn minimum_guarantees_not_proven_when_below_minimum_example_removed() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    for row in &mut packet.registry_rows {
        row.minimum_size_entries.retain(|ex| {
            ex.degrade_reason != Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
        });
    }
    assert!(packet.validate().contains(
        &M5ShellMetricRegistriesViolation::MinimumGuaranteesAcrossDensityAndSnappedWidthsNotProven
    ));
}

#[test]
fn minimum_guarantees_not_proven_when_control_class_dropped() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    // Drop every clean icon-only-control minimum so the coverage no longer includes it.
    for row in &mut packet.registry_rows {
        row.minimum_size_entries
            .retain(|ex| !(ex.is_clean() && ex.control == "icon_only_control"));
    }
    assert!(packet.validate().contains(
        &M5ShellMetricRegistriesViolation::MinimumGuaranteesAcrossDensityAndSnappedWidthsNotProven
    ));
}

#[test]
fn drift_not_proven_when_envelope_example_removed() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    for row in &mut packet.registry_rows {
        row.shell_metric_entries.retain(|ex| {
            ex.degrade_reason
                != Some(M5ShellMetricEntryDegradeReason::MetricOutsideCanonicalEnvelope)
        });
    }
    assert!(packet.validate().contains(
        &M5ShellMetricRegistriesViolation::DriftOutsideCanonicalEnvelopeDetectableNotProven
    ));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.governance_review.main_editor_group_stays_dominant = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet
        .consumer_projection
        .support_export_reads_single_registry_source = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_shell_metric_minimum_size_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_shell_metric_minimum_size_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_shell_metric_minimum_size_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_shell_metric_minimum_size_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_shell_metric_minimum_size_registries_export()
        .expect("checked M5 shell-metric / minimum-size registries export validates");
    assert_eq!(from_disk.packet_id, M5_SHELL_METRIC_REGISTRIES_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_shell_metric_minimum_size_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(row.qualification, M5ShellGeometryQualificationClass::Beta);

    let preview = seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5ShellGeometryConsumerSurface::DataUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5ShellGeometryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ShellMetricRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-and-minimum-size-registries/editor_ui_beta_narrowed.json"
    )))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed()
    );

    let preview: M5ShellMetricRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-and-minimum-size-registries/data_ui_preview_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_shell_metric_and_minimum_size() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5ShellGeometryFamily::ShellMetric,
            M5ShellGeometryFamily::MinimumSize
        ]
    );
}
