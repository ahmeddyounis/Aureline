use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_visual_foundation_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_VISUAL_FOUNDATION_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_foundation_family() {
    let packet = seeded_m5_visual_foundation_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .foundation_rows
        .iter()
        .map(|r| r.foundation_family)
        .collect();
    for family in M5VisualFoundationFamily::ALL {
        assert!(
            present.contains(&family),
            "missing foundation family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.foundation_rows.len(),
        M5VisualFoundationFamily::ALL.len()
    );
}

#[test]
fn frozen_semantic_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: brand / interactive / neutral / status / syntax / diff /
    // chart stays in one controlled token set that no shell, editor, review, data, or docs surface
    // reinvents.
    let tokens: Vec<&str> = M5VisualSemanticRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "brand",
            "interactive",
            "neutral",
            "status",
            "syntax",
            "diff",
            "chart",
        ]
    );
    assert!(M5VisualSemanticRole::Status.demands_non_color_cue());
    assert!(M5VisualSemanticRole::Syntax.demands_non_color_cue());
    assert!(M5VisualSemanticRole::Diff.demands_non_color_cue());
    assert!(M5VisualSemanticRole::Chart.demands_non_color_cue());
    assert!(!M5VisualSemanticRole::Brand.demands_non_color_cue());
    assert!(!M5VisualSemanticRole::Neutral.demands_non_color_cue());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_visual_foundation_matrix();
    for row in &packet.foundation_rows {
        for label in M5VisualFoundationRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.foundation_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.foundation_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.foundation_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5VisualFoundationAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_visual_foundation_matrix();
    for row in &packet.foundation_rows {
        let family = row.foundation_family;
        assert_eq!(
            !row.color_roles.is_empty(),
            family.declares_color_roles(),
            "color_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.theme_token_roles.is_empty(),
            family.declares_theme_token_roles(),
            "theme_token_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.syntax_roles.is_empty(),
            family.declares_syntax_roles(),
            "syntax_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.diff_roles.is_empty(),
            family.declares_diff_roles(),
            "diff_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.chart_roles.is_empty(),
            family.declares_chart_roles(),
            "chart_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.typography_roles.is_empty(),
            family.declares_typography_roles(),
            "typography_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.geometry_roles.is_empty(),
            family.declares_geometry_roles(),
            "geometry_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.hit_target_rules.is_empty(),
            family.declares_hit_target_rules(),
            "hit_target_rules presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_visual_foundation_matrix();
    for role in M5VisualSemanticRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares semantic role {}",
            role.as_str()
        );
    }
    for role in M5ColorRoleFamily::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.color_roles.contains(&role)),
            "no family declares color role {}",
            role.as_str()
        );
    }
    for role in M5ThemeTokenRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.theme_token_roles.contains(&role)),
            "no family declares theme-token role {}",
            role.as_str()
        );
    }
    for role in M5SyntaxTokenRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.syntax_roles.contains(&role)),
            "no family declares syntax role {}",
            role.as_str()
        );
    }
    for role in M5DiffTokenRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.diff_roles.contains(&role)),
            "no family declares diff role {}",
            role.as_str()
        );
    }
    for role in M5ChartTokenRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.chart_roles.contains(&role)),
            "no family declares chart role {}",
            role.as_str()
        );
    }
    for role in M5TypographyRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.typography_roles.contains(&role)),
            "no family declares typography role {}",
            role.as_str()
        );
    }
    for role in M5GeometryRole::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.geometry_roles.contains(&role)),
            "no family declares geometry role {}",
            role.as_str()
        );
    }
    for rule in M5HitTargetRule::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.hit_target_rules.contains(&rule)),
            "no family declares hit-target rule {}",
            rule.as_str()
        );
    }
    for reason in M5VisualFoundationDegradedReason::ALL {
        assert!(
            packet
                .foundation_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_foundation_family_fails_validation() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet
        .foundation_rows
        .retain(|row| row.foundation_family != M5VisualFoundationFamily::ChartToken);
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[0]
        .required_labels
        .retain(|label| *label != M5VisualFoundationRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let own = M5VisualFoundationFamily::ColorSystem.canonical_domain_schema_ref();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::ColorSystem)
        .expect("color-system row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::SemanticRoleMissing));
}

#[test]
fn color_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::ColorSystem)
        .expect("color-system present");
    row.color_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ColorRoleMissing));
}

#[test]
fn theme_token_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::SemanticThemeToken)
        .expect("theme-token present");
    row.theme_token_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ThemeTokenRoleMissing));
}

#[test]
fn syntax_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::SyntaxToken)
        .expect("syntax-token present");
    row.syntax_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::SyntaxRoleMissing));
}

#[test]
fn diff_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::DiffToken)
        .expect("diff-token present");
    row.diff_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::DiffRoleMissing));
}

#[test]
fn chart_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::ChartToken)
        .expect("chart-token present");
    row.chart_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ChartRoleMissing));
}

#[test]
fn typography_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::Typography)
        .expect("typography present");
    row.typography_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::TypographyRoleMissing));
}

#[test]
fn geometry_role_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::SpacingSizingRadiiElevation)
        .expect("geometry present");
    row.geometry_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::GeometryRoleMissing));
}

#[test]
fn hit_target_rule_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::HitTarget)
        .expect("hit-target present");
    row.hit_target_rules.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::HitTargetRuleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::DegradedReasonMissing));
}

#[test]
fn foundation_invariant_violation_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[0].collapses_status_or_trust_into_color_only = true;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::FoundationInvariantViolated));

    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[2].lets_syntax_or_diff_palette_collide_with_diagnostics = true;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::FoundationInvariantViolated));

    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[7].shrinks_hit_target_below_supported_minimum = true;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::FoundationInvariantViolated));

    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[4].lets_chart_meaning_depend_on_color_alone = true;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::FoundationInvariantViolated));

    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[6].forks_local_spacing_or_elevation_from_shared_geometry = true;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::FoundationInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::ColorSystem)
        .expect("color-system row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.governance_review.status_meaning_never_color_alone = false;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_visual_foundation_source = false;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_foundation_family() {
    let summary = seeded_m5_visual_foundation_matrix().render_markdown_summary();
    for family in M5VisualFoundationFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_visual_foundation_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5VisualFoundationFamily::ALL.len());
    assert!(lines[0].starts_with("foundation_family,qualification,owner,canonical_schema,"));
    for family in M5VisualFoundationFamily::ALL {
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
    let packet = current_stable_m5_visual_foundation_matrix_export()
        .expect("checked M5 visual-foundation matrix export validates");
    assert_eq!(packet.packet_id, M5_VISUAL_FOUNDATION_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_visual_foundation_matrix_export()
        .expect("checked M5 visual-foundation matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_visual_foundation_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_visual_foundation_matrix_typography_beta_narrowed(),
        seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.foundation_rows.len(),
            M5VisualFoundationFamily::ALL.len()
        );
    }

    let typography = seeded_m5_visual_foundation_matrix_typography_beta_narrowed();
    let row = typography
        .foundation_rows
        .iter()
        .find(|r| r.foundation_family == M5VisualFoundationFamily::Typography)
        .expect("typography row present");
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Beta
    );

    let chart = seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed();
    let row = chart
        .foundation_rows
        .iter()
        .find(|r| r.foundation_family == M5VisualFoundationFamily::ChartToken)
        .expect("chart-token row present");
    assert_eq!(
        row.qualification,
        M5VisualFoundationQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let typography: M5VisualFoundationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-visual-foundations/typography_beta_narrowed.json"
    )))
    .expect("typography fixture parses");
    assert!(typography.validate().is_empty());
    assert_eq!(
        typography,
        seeded_m5_visual_foundation_matrix_typography_beta_narrowed()
    );

    let chart: M5VisualFoundationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-visual-foundations/chart_token_preview_narrowed.json"
    )))
    .expect("chart fixture parses");
    assert!(chart.validate().is_empty());
    assert_eq!(
        chart,
        seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_visual_foundation_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.foundation_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5VisualFoundationMatrixViolation::RawMaterialInExport));
}
