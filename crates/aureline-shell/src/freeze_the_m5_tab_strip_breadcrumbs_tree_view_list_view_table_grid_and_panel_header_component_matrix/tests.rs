use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_navigation_content_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_navigation_content_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5NavigationContentComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5NavigationContentComponentFamily::ALL.len()
    );
}

#[test]
fn frozen_disposition_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: preview / pinned / modified / read-only / blocked /
    // exact-count / loaded-count / all-matching-count / hidden-by-filter / hidden-by-policy /
    // overflowed-local-action / stale-or-partial-hierarchy stays in one controlled token set that no
    // navigation or content surface reinvents.
    let tokens: Vec<&str> = M5NavigationContentDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "preview",
            "pinned",
            "modified",
            "read_only",
            "blocked",
            "exact_count",
            "loaded_count",
            "all_matching_count",
            "hidden_by_filter",
            "hidden_by_policy",
            "overflowed_local_action",
            "stale_or_partial_hierarchy",
        ]
    );
    assert!(M5NavigationContentDisposition::ExactCount.is_count_scope());
    assert!(M5NavigationContentDisposition::LoadedCount.is_count_scope());
    assert!(M5NavigationContentDisposition::AllMatchingCount.is_count_scope());
    assert!(!M5NavigationContentDisposition::Pinned.is_count_scope());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_navigation_content_component_matrix();
    for row in &packet.component_rows {
        for label in M5NavigationContentRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.component_family
                    .canonical_component_schema_ref()
                    .to_owned()
            ),
            "component {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5NavigationContentAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_navigation_content_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.active_context_states.is_empty(),
            family.declares_active_context(),
            "active_context_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.hierarchy_path_states.is_empty(),
            family.declares_hierarchy_path(),
            "hierarchy_path_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.disclosure_states.is_empty(),
            family.declares_disclosure(),
            "disclosure_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.selection_states.is_empty(),
            family.declares_selection(),
            "selection_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.count_scopes.is_empty(),
            family.declares_count_scope(),
            "count_scopes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.item_state_flags.is_empty(),
            family.declares_item_state(),
            "item_state_flags presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.density_variants.is_empty(),
            family.declares_density(),
            "density_variants presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.local_action_budgets.is_empty(),
            family.declares_local_action_budget(),
            "local_action_budgets presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_navigation_content_component_matrix();
    for disposition in M5NavigationContentDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for state in M5ActiveContextState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.active_context_states.contains(&state)),
            "no component declares active-context state {}",
            state.as_str()
        );
    }
    for state in M5HierarchyPathState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.hierarchy_path_states.contains(&state)),
            "no component declares hierarchy / path state {}",
            state.as_str()
        );
    }
    for state in M5DisclosureState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.disclosure_states.contains(&state)),
            "no component declares disclosure state {}",
            state.as_str()
        );
    }
    for state in M5SelectionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.selection_states.contains(&state)),
            "no component declares selection state {}",
            state.as_str()
        );
    }
    for scope in M5CountScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.count_scopes.contains(&scope)),
            "no component declares count scope {}",
            scope.as_str()
        );
    }
    for flag in M5ItemStateFlag::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.item_state_flags.contains(&flag)),
            "no component declares item-state flag {}",
            flag.as_str()
        );
    }
    for density in M5DensityVariant::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.density_variants.contains(&density)),
            "no component declares density variant {}",
            density.as_str()
        );
    }
    for budget in M5LocalActionBudget::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.local_action_budgets.contains(&budget)),
            "no component declares local-action budget {}",
            budget.as_str()
        );
    }
    for reason in M5NavigationContentDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no component declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5NavigationContentComponentFamily::TreeView);
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5NavigationContentRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    let own = M5NavigationContentComponentFamily::TabStrip.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::TabStrip)
        .expect("tab-strip row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::DispositionMissing));
}

#[test]
fn tab_strip_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_navigation_content_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5NavigationContentComponentFamily::TabStrip)
            .expect("tab-strip present");
        let expected = if clear == 0 {
            row.active_context_states.clear();
            M5NavigationContentComponentMatrixViolation::ActiveContextMissing
        } else {
            row.item_state_flags.clear();
            M5NavigationContentComponentMatrixViolation::ItemStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn breadcrumbs_hierarchy_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::Breadcrumbs)
        .expect("breadcrumbs present");
    row.hierarchy_path_states.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::HierarchyPathMissing));
}

#[test]
fn tree_view_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_navigation_content_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5NavigationContentComponentFamily::TreeView)
            .expect("tree-view present");
        let expected = match clear {
            0 => {
                row.disclosure_states.clear();
                M5NavigationContentComponentMatrixViolation::DisclosureMissing
            }
            1 => {
                row.selection_states.clear();
                M5NavigationContentComponentMatrixViolation::SelectionMissing
            }
            _ => {
                row.count_scopes.clear();
                M5NavigationContentComponentMatrixViolation::CountScopeMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn list_view_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_navigation_content_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5NavigationContentComponentFamily::ListView)
            .expect("list-view present");
        let expected = if clear == 0 {
            row.count_scopes.clear();
            M5NavigationContentComponentMatrixViolation::CountScopeMissing
        } else {
            row.density_variants.clear();
            M5NavigationContentComponentMatrixViolation::DensityMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn table_grid_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_navigation_content_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5NavigationContentComponentFamily::TableGrid)
            .expect("table-grid present");
        let expected = if clear == 0 {
            row.density_variants.clear();
            M5NavigationContentComponentMatrixViolation::DensityMissing
        } else {
            row.local_action_budgets.clear();
            M5NavigationContentComponentMatrixViolation::LocalActionBudgetMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn panel_header_vocab_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::PanelHeader)
        .expect("panel-header present");
    row.local_action_budgets.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::LocalActionBudgetMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[2].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[0].tabs_masquerade_as_top_level_workflow_navigation = true;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[3].hides_counts_or_blocked_rows_behind_ambiguous_ellipsis = true;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[2].makes_tree_list_or_table_actions_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[5].panel_header_becomes_cluttered_secondary_toolbar = true;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[4].collapses_exact_loaded_and_all_matching_scopes_into_one_total = true;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5NavigationContentComponentFamily::TabStrip)
        .expect("tab-strip row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet
        .governance_review
        .counts_never_collapsed_into_one_total = false;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_navigation_source = false;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_navigation_content_component_matrix().render_markdown_summary();
    for family in M5NavigationContentComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_navigation_content_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5NavigationContentComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5NavigationContentComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_component_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_navigation_content_component_matrix_export()
        .expect("checked M5 navigation-content component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_NAVIGATION_CONTENT_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_navigation_content_component_matrix_export()
        .expect("checked M5 navigation-content component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_navigation_content_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed(),
        seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5NavigationContentComponentFamily::ALL.len()
        );
    }

    let table = seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed();
    let row = table
        .component_rows
        .iter()
        .find(|r| r.component_family == M5NavigationContentComponentFamily::TableGrid)
        .expect("table-grid row present");
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Beta
    );

    let tree = seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed();
    let row = tree
        .component_rows
        .iter()
        .find(|r| r.component_family == M5NavigationContentComponentFamily::TreeView)
        .expect("tree-view row present");
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let table: M5NavigationContentComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-navigation-content-components/table_grid_beta_narrowed.json"
        )))
        .expect("table-grid fixture parses");
    assert!(table.validate().is_empty());
    assert_eq!(
        table,
        seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed()
    );

    let tree: M5NavigationContentComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-navigation-content-components/tree_view_preview_narrowed.json"
        )))
        .expect("tree-view fixture parses");
    assert!(tree.validate().is_empty());
    assert_eq!(
        tree,
        seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_navigation_content_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_navigation_content_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5NavigationContentComponentMatrixViolation::RawMaterialInExport));
}
