use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_shell_metric_density_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHELL_METRIC_DENSITY_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_geometry_family() {
    let packet = seeded_m5_shell_metric_density_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .geometry_rows
        .iter()
        .map(|r| r.geometry_family)
        .collect();
    for family in M5ShellGeometryFamily::ALL {
        assert!(
            present.contains(&family),
            "missing geometry family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.geometry_rows.len(), M5ShellGeometryFamily::ALL.len());
}

#[test]
fn frozen_shell_geometry_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: zone / metric / hit_target / density / responsive /
    // collapse / workspace_dominance stays in one controlled token set that no desktop, editor, review,
    // notebook, or data surface reinvents.
    let tokens: Vec<&str> = M5ShellGeometryRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "zone",
            "metric",
            "hit_target",
            "density",
            "responsive",
            "collapse",
            "workspace_dominance",
        ]
    );
    assert!(M5ShellGeometryRole::Density.must_preserve_task_identity_under_collapse());
    assert!(M5ShellGeometryRole::Responsive.must_preserve_task_identity_under_collapse());
    assert!(M5ShellGeometryRole::Collapse.must_preserve_task_identity_under_collapse());
    assert!(M5ShellGeometryRole::WorkspaceDominance.must_preserve_task_identity_under_collapse());
    assert!(!M5ShellGeometryRole::Zone.must_preserve_task_identity_under_collapse());
    assert!(!M5ShellGeometryRole::Metric.must_preserve_task_identity_under_collapse());
    assert!(!M5ShellGeometryRole::HitTarget.must_preserve_task_identity_under_collapse());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_shell_metric_density_matrix();
    for row in &packet.geometry_rows {
        for label in M5ShellGeometryRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.geometry_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.geometry_family.canonical_domain_schema_ref().to_owned()),
            "family {} does not point at its canonical schema",
            row.geometry_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ShellGeometryAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_shell_metric_density_matrix();
    for row in &packet.geometry_rows {
        let family = row.geometry_family;
        assert_eq!(
            !row.shell_metric_roles.is_empty(),
            family.declares_shell_metric_roles(),
            "shell_metric_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.minimum_size_roles.is_empty(),
            family.declares_minimum_size_roles(),
            "minimum_size_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.density_mode_roles.is_empty(),
            family.declares_density_mode_roles(),
            "density_mode_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.responsive_geometry_roles.is_empty(),
            family.declares_responsive_geometry_roles(),
            "responsive_geometry_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.collapse_priority_roles.is_empty(),
            family.declares_collapse_priority_roles(),
            "collapse_priority_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_shell_metric_density_matrix();
    for role in M5ShellGeometryRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares shell-geometry role {}",
            role.as_str()
        );
    }
    for role in M5ShellMetricRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.shell_metric_roles.contains(&role)),
            "no family declares shell-metric role {}",
            role.as_str()
        );
    }
    for role in M5MinimumSizeRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.minimum_size_roles.contains(&role)),
            "no family declares minimum-size role {}",
            role.as_str()
        );
    }
    for role in M5DensityModeRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.density_mode_roles.contains(&role)),
            "no family declares density-mode role {}",
            role.as_str()
        );
    }
    for role in M5ResponsiveGeometryRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.responsive_geometry_roles.contains(&role)),
            "no family declares responsive-geometry role {}",
            role.as_str()
        );
    }
    for role in M5CollapsePriorityRole::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.collapse_priority_roles.contains(&role)),
            "no family declares collapse-priority role {}",
            role.as_str()
        );
    }
    for reason in M5ShellGeometryDegradedReason::ALL {
        assert!(
            packet
                .geometry_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_geometry_family_fails_validation() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet
        .geometry_rows
        .retain(|row| row.geometry_family != M5ShellGeometryFamily::CollapsePriority);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[0]
        .required_labels
        .retain(|label| *label != M5ShellGeometryRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let own = M5ShellGeometryFamily::ShellMetric.canonical_domain_schema_ref();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::ShellMetric)
        .expect("shell-metric row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::SemanticRoleMissing));
}

#[test]
fn shell_metric_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::ShellMetric)
        .expect("shell-metric present");
    row.shell_metric_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ShellMetricRoleMissing));
}

#[test]
fn minimum_size_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::MinimumSize)
        .expect("minimum-size present");
    row.minimum_size_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::MinimumSizeRoleMissing));
}

#[test]
fn density_mode_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::DensityMode)
        .expect("density-mode present");
    row.density_mode_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::DensityModeRoleMissing));
}

#[test]
fn responsive_geometry_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::ResponsiveGeometry)
        .expect("responsive-geometry present");
    row.responsive_geometry_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ResponsiveGeometryRoleMissing));
}

#[test]
fn collapse_priority_role_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::CollapsePriority)
        .expect("collapse-priority present");
    row.collapse_priority_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::CollapsePriorityRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::DegradedReasonMissing));
}

#[test]
fn geometry_invariant_violation_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[2].density_or_collapse_changes_command_focus_or_trust = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated));

    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[4].extension_or_embedded_sets_private_fracturing_width = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated));

    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[1].shrinks_hit_target_below_supported_minimum = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated));

    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[4].hides_primary_workflow_behind_overlay_only_fallback = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated));

    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[0].lets_zone_starve_main_workspace_below_minimum = true;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    let row = packet
        .geometry_rows
        .iter_mut()
        .find(|row| row.geometry_family == M5ShellGeometryFamily::ShellMetric)
        .expect("shell-metric row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.governance_review.main_workspace_remains_dominant = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_shell_geometry_source = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_geometry_family() {
    let summary = seeded_m5_shell_metric_density_matrix().render_markdown_summary();
    for family in M5ShellGeometryFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_shell_metric_density_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ShellGeometryFamily::ALL.len());
    assert!(lines[0].starts_with("geometry_family,qualification,owner,canonical_schema,"));
    for family in M5ShellGeometryFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing family {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_shell_metric_density_matrix_export()
        .expect("checked M5 shell-metric / density matrix export validates");
    assert_eq!(packet.packet_id, M5_SHELL_METRIC_DENSITY_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_shell_metric_density_matrix_export()
        .expect("checked M5 shell-metric / density matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_shell_metric_density_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed(),
        seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.geometry_rows.len(), M5ShellGeometryFamily::ALL.len());
    }

    let responsive = seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed();
    let row = responsive
        .geometry_rows
        .iter()
        .find(|r| r.geometry_family == M5ShellGeometryFamily::ResponsiveGeometry)
        .expect("responsive-geometry row present");
    assert_eq!(row.qualification, M5ShellGeometryQualificationClass::Beta);

    let collapse = seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed();
    let row = collapse
        .geometry_rows
        .iter()
        .find(|r| r.geometry_family == M5ShellGeometryFamily::CollapsePriority)
        .expect("collapse-priority row present");
    assert_eq!(
        row.qualification,
        M5ShellGeometryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let responsive: M5ShellMetricDensityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-density/responsive_geometry_beta_narrowed.json"
    )))
    .expect("responsive-geometry fixture parses");
    assert!(responsive.validate().is_empty());
    assert_eq!(
        responsive,
        seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed()
    );

    let collapse: M5ShellMetricDensityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-metric-density/collapse_priority_preview_narrowed.json"
    )))
    .expect("collapse-priority fixture parses");
    assert!(collapse.validate().is_empty());
    assert_eq!(
        collapse,
        seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_shell_metric_density_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_shell_metric_density_matrix();
    packet.geometry_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ShellMetricDensityMatrixViolation::RawMaterialInExport));
}
