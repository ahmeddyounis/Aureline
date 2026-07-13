use super::*;

fn clean_geometry_input() -> M5GeometryEntryResolutionInput {
    M5GeometryEntryResolutionInput {
        entry_id: "geometry:test".to_owned(),
        token_name: "space.2".to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        geometry_role: M5GeometryRole::SpacingStep,
        primitive_kind: M5GeometryPrimitiveKind::Spacing,
        density_mode: M5GeometryDensityMode::Standard,
        elevation_tier: M5ElevationTier::Base,
        surface_context: M5GeometrySurfaceContext::Shell,
        density_aware: true,
        elevation_hierarchy_preserved: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn clean_hit_target_input() -> M5HitTargetEntryResolutionInput {
    M5HitTargetEntryResolutionInput {
        entry_id: "hit-target:test".to_owned(),
        token_name: "target.compact".to_owned(),
        semantic_role: M5VisualSemanticRole::Interactive,
        hit_target_rule: M5HitTargetRule::CompactMinimum,
        control_kind: M5HitTargetControlKind::Button,
        density_mode: M5GeometryDensityMode::Compact,
        surface_context: M5GeometrySurfaceContext::ListTable,
        meets_supported_minimum: true,
        adequate_target_spacing: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_registries_validates() {
    let packet = seeded_m5_geometry_hit_target_registries();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_PACKET_ID
    );
}

#[test]
fn geometry_clean_reads_canonical() {
    let resolved = resolve_geometry_entry(clean_geometry_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.geometry_is_canonical);
    assert!(resolved.geometry_role_matches_kind);
    assert!(resolved.density_aware);
    assert!(resolved.references_canonical_token);
    assert_eq!(resolved.primitive_kind, "spacing");
    assert_eq!(resolved.surface_context, "shell");
    assert_eq!(
        resolved.next_action,
        M5GeometryNextAction::InspectGeometryScale
    );
}

#[test]
fn geometry_role_fork_and_mismatch_degrade() {
    // The disallowed local-fork role never matches any kind.
    let mut input = clean_geometry_input();
    input.geometry_role = M5GeometryRole::LocalGeometryForkDisallowed;
    let resolved = resolve_geometry_entry(input).unwrap();
    assert!(!resolved.geometry_role_matches_kind);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GeometryDegradeReason::GeometryRoleForked)
    );

    // A spacing primitive that names a radius step does not match its kind.
    let mut input = clean_geometry_input();
    input.geometry_role = M5GeometryRole::RadiusStep;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::GeometryRoleForked)
    );
}

#[test]
fn geometry_density_and_elevation_degrade() {
    let mut input = clean_geometry_input();
    input.density_aware = false;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::NotDensityAware)
    );

    let mut input = clean_geometry_input();
    input.primitive_kind = M5GeometryPrimitiveKind::Elevation;
    input.geometry_role = M5GeometryRole::ElevationLevel;
    input.elevation_tier = M5ElevationTier::Dialog;
    input.elevation_hierarchy_preserved = false;
    let resolved = resolve_geometry_entry(input).unwrap();
    assert!(resolved.is_elevation);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5GeometryDegradeReason::ElevationHierarchyBroken)
    );

    let mut input = clean_geometry_input();
    input.density_mode = M5GeometryDensityMode::DensityUnknown;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::DensityModeUnresolved)
    );
}

#[test]
fn geometry_raw_kind_and_forbidden_degrade() {
    let mut input = clean_geometry_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::RawGeometryValueInlined)
    );

    let mut input = clean_geometry_input();
    input.primitive_kind = M5GeometryPrimitiveKind::KindUnknown;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::PrimitiveKindUnstated)
    );

    let mut input = clean_geometry_input();
    input.token_name = "  ".to_owned();
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::TokenNameUnstated)
    );

    let mut input = clean_geometry_input();
    input.surface_context = M5GeometrySurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_geometry_entry(input).unwrap().degrade_reason,
        Some(M5GeometryDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_geometry_input();
    input.entry_id = "".to_owned();
    assert_eq!(
        resolve_geometry_entry(input).unwrap_err(),
        M5GeometryHitTargetResolutionError::EmptyGeometryEntryId
    );

    let mut input = clean_geometry_input();
    input.token_name = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_geometry_entry(input).unwrap_err(),
        M5GeometryHitTargetResolutionError::ForbiddenMaterial
    );
}

#[test]
fn hit_target_clean_meets_minimum() {
    let resolved = resolve_hit_target_entry(clean_hit_target_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.target_meets_minimum);
    assert!(resolved.meets_supported_minimum);
    assert!(resolved.adequate_target_spacing);
    assert_eq!(resolved.control_kind, "button");
    assert_eq!(resolved.surface_context, "list_table");
    assert_eq!(
        resolved.next_action,
        M5GeometryNextAction::AdjustHitTargetSizing
    );
}

#[test]
fn hit_target_shrink_and_spacing_degrade() {
    let mut input = clean_hit_target_input();
    input.meets_supported_minimum = false;
    let resolved = resolve_hit_target_entry(input).unwrap();
    assert!(!resolved.meets_supported_minimum);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5HitTargetDegradeReason::ShrinksBelowMinimum)
    );

    // The disallowed shrink-below-minimum rule can never read as meeting the minimum.
    let mut input = clean_hit_target_input();
    input.hit_target_rule = M5HitTargetRule::ShrinkBelowMinimumDisallowed;
    let resolved = resolve_hit_target_entry(input).unwrap();
    assert!(!resolved.meets_supported_minimum);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5HitTargetDegradeReason::ShrinksBelowMinimum)
    );

    let mut input = clean_hit_target_input();
    input.adequate_target_spacing = false;
    assert_eq!(
        resolve_hit_target_entry(input).unwrap().degrade_reason,
        Some(M5HitTargetDegradeReason::InadequateTargetSpacing)
    );
}

#[test]
fn hit_target_control_raw_and_forbidden_degrade() {
    let mut input = clean_hit_target_input();
    input.control_kind = M5HitTargetControlKind::ControlUnknown;
    assert_eq!(
        resolve_hit_target_entry(input).unwrap().degrade_reason,
        Some(M5HitTargetDegradeReason::ControlKindUnresolved)
    );

    let mut input = clean_hit_target_input();
    input.references_canonical_token = false;
    assert_eq!(
        resolve_hit_target_entry(input).unwrap().degrade_reason,
        Some(M5HitTargetDegradeReason::RawGeometryValueInlined)
    );

    let mut input = clean_hit_target_input();
    input.entry_id = "  ".to_owned();
    assert_eq!(
        resolve_hit_target_entry(input).unwrap_err(),
        M5GeometryHitTargetResolutionError::EmptyHitTargetEntryId
    );

    let mut input = clean_hit_target_input();
    input.token_name = "bearer abc".to_owned();
    assert_eq!(
        resolve_hit_target_entry(input).unwrap_err(),
        M5GeometryHitTargetResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_geometry_hit_target_registries()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.vocabulary_set.primitive_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::MissingSourceContracts));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.registry_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::DomainSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.registry_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5GeometryAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.registry_rows[0]
        .export_fields
        .retain(|f| *f != M5GeometryExportField::SemanticRoles);
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    let row = &mut packet.registry_rows[0];
    row.geometry_entries.clear();
    row.hit_target_entries.clear();
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    // Force a clean geometry entry to also read as a forked role — the packet must reject it.
    let row = &mut packet.registry_rows[0];
    row.geometry_entries[0].degrade_reason = None;
    row.geometry_entries[0].geometry_role_matches_kind = false;
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_geometry_hit_target_registries();
        let row = &mut packet.registry_rows[0];
        match mutate {
            0 => row.local_geometry_forked_from_foundation = true,
            1 => row.hit_target_shrunk_below_minimum = true,
            2 => row.elevation_hierarchy_broken = true,
            _ => row.raw_geometry_value_inlined_instead_of_token = true,
        }
        assert!(packet
            .validate()
            .contains(&M5GeometryHitTargetRegistriesViolation::RowInvariantViolated));
    }
}

#[test]
fn canonical_primitives_not_proven_when_forked_example_removed() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    for row in &mut packet.registry_rows {
        row.geometry_entries
            .retain(|ex| ex.degrade_reason != Some(M5GeometryDegradeReason::GeometryRoleForked));
    }
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::CanonicalGeometryPrimitivesNotProven));
}

#[test]
fn canonical_primitives_not_proven_when_elevation_kind_dropped() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    // Drop every clean elevation entry so the primitives no longer cover "elevation".
    for row in &mut packet.registry_rows {
        row.geometry_entries
            .retain(|ex| !(ex.is_clean() && ex.primitive_kind == "elevation"));
    }
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::CanonicalGeometryPrimitivesNotProven));
}

#[test]
fn compact_minima_not_proven_when_shrink_example_removed() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    for row in &mut packet.registry_rows {
        row.hit_target_entries
            .retain(|ex| ex.degrade_reason != Some(M5HitTargetDegradeReason::ShrinksBelowMinimum));
    }
    assert!(packet.validate().contains(
        &M5GeometryHitTargetRegistriesViolation::CompactMinimaOrElevationHierarchyNotProven
    ));
}

#[test]
fn compact_minima_not_proven_when_elevation_broken_removed() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    for row in &mut packet.registry_rows {
        row.geometry_entries.retain(|ex| {
            ex.degrade_reason != Some(M5GeometryDegradeReason::ElevationHierarchyBroken)
        });
    }
    assert!(packet.validate().contains(
        &M5GeometryHitTargetRegistriesViolation::CompactMinimaOrElevationHierarchyNotProven
    ));
}

#[test]
fn geometry_drift_not_caught_when_not_density_aware_removed() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    for row in &mut packet.registry_rows {
        row.geometry_entries
            .retain(|ex| ex.degrade_reason != Some(M5GeometryDegradeReason::NotDensityAware));
    }
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::GeometryDriftNotCaught));
}

#[test]
fn geometry_drift_not_caught_when_raw_geometry_removed() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    for row in &mut packet.registry_rows {
        row.geometry_entries.retain(|ex| {
            ex.degrade_reason != Some(M5GeometryDegradeReason::RawGeometryValueInlined)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::GeometryDriftNotCaught));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet
        .governance_review
        .compact_density_preserves_hit_target_minima = false;
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet
        .consumer_projection
        .support_export_reads_single_geometry_source = false;
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_geometry_hit_target_registries();
    packet.registry_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5GeometryHitTargetRegistriesViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_geometry_hit_target_registries().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_geometry_hit_target_registries();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.registry_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_geometry_hit_target_registries();
    let summary = packet.render_markdown_summary();
    for row in &packet.registry_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_geometry_hit_target_registries_export()
        .expect("checked M5 geometry / hit-target registries export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_geometry_hit_target_registries(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.registry_rows.len(), 6);
    let row = beta
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualFoundationConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Beta
    );

    let preview = seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.registry_rows.len(), 6);
    let row = preview
        .registry_rows
        .iter()
        .find(|r| r.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5GeometryHitTargetRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-spacing-sizing-radii-elevation-and-hit-target-registries/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed()
    );

    let preview: M5GeometryHitTargetRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-spacing-sizing-radii-elevation-and-hit-target-registries/data_ui_preview_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_geometry_and_hit_target() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5VisualFoundationFamily::SpacingSizingRadiiElevation,
            M5VisualFoundationFamily::HitTarget
        ]
    );
}
