use super::*;

fn clean_tree_input() -> M5TreeViewResolutionInput {
    M5TreeViewResolutionInput {
        tree_id: "tree:test".to_owned(),
        node_label: "main.rs".to_owned(),
        disclosure: M5DisclosureState::Expanded,
        lazy_subtree_shown_as_leaf: false,
        selection: M5SelectionState::SingleSelected,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state: M5ItemStateFlag::Pinned,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        count_scope: M5TreeListScopeKind::ExactCount,
        count_scopes_distinct: true,
        density: M5DensityVariant::Comfortable,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        drag_reorder: M5DragReorderPosture::ReorderEnabled,
        overclaims_drag_reorder: false,
        cross_surface_continuity: M5CrossSurfaceContinuity::SinglePaneOnly,
        overclaims_cross_surface_continuity: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_list_input() -> M5ListViewResolutionInput {
    M5ListViewResolutionInput {
        list_id: "list:test".to_owned(),
        row_label: "match 1".to_owned(),
        selection: M5SelectionState::SelectedAndCurrent,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state: M5ItemStateFlag::Pinned,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        count_scope: M5TreeListScopeKind::ExactCount,
        count_scopes_distinct: true,
        loaded_shown_as_exact: false,
        density: M5DensityVariant::Comfortable,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        drag_reorder: M5DragReorderPosture::ReorderEnabled,
        overclaims_drag_reorder: false,
        cross_surface_continuity: M5CrossSurfaceContinuity::CrossPaneMirrored,
        overclaims_cross_surface_continuity: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_tree_view_list_view_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TREE_VIEW_LIST_VIEW_CONTROLS_PACKET_ID);
}

#[test]
fn tree_clean_names_structure_and_is_legible() {
    let resolved = resolve_tree_view(clean_tree_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.structure_legible_at_a_glance);
    assert!(!resolved.current_selection_hover_only);
    assert!(!resolved.lazy_subtree_shown_as_leaf);
    assert_eq!(resolved.disclosure, "expanded");
    assert_eq!(resolved.selection, "single_selected");
    assert_eq!(resolved.count_scope, "exact_count");
    assert!(resolved.drag_reorder_permitted);
    assert_eq!(resolved.next_action, M5TreeListNextAction::OpenScopeDetail);
}

#[test]
fn tree_partial_backend_is_clean_when_not_shown_complete() {
    let mut input = clean_tree_input();
    input.disclosure = M5DisclosureState::LazyUnloaded;
    input.backend_stale_or_partial = true;
    let resolved = resolve_tree_view(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.backend_stale_or_partial);
    assert!(!resolved.presents_stale_or_partial_as_complete);
}

#[test]
fn tree_node_unstated_degrades() {
    let mut input = clean_tree_input();
    input.node_label = "   ".to_owned();
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::NodeIdentityUnstated)
    );
}

#[test]
fn tree_disclosure_unknown_and_lazy_leaf_degrade() {
    let mut input = clean_tree_input();
    input.disclosure = M5DisclosureState::DisclosureUnknown;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::DisclosureStateUnresolved)
    );

    let mut input = clean_tree_input();
    input.disclosure = M5DisclosureState::LazyUnloaded;
    input.lazy_subtree_shown_as_leaf = true;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::LazySubtreeShownAsEmptyLeaf)
    );
}

#[test]
fn tree_selection_and_focus_degrade() {
    let mut input = clean_tree_input();
    input.selection_versus_current_distinct = false;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::SelectionVersusCurrentCollapsed)
    );

    let mut input = clean_tree_input();
    input.selection = M5SelectionState::SelectionUnknown;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::SelectionVersusCurrentCollapsed)
    );

    let mut input = clean_tree_input();
    input.row_focus_visible = false;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::RowFocusNotVisible)
    );
}

#[test]
fn tree_hover_only_and_blocked_hover_degrade() {
    let mut input = clean_tree_input();
    input.current_selection_hover_only = true;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::CurrentSelectionHoverOnly)
    );

    let mut input = clean_tree_input();
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    let resolved = resolve_tree_view(input).unwrap();
    assert!(resolved.has_blocked_row);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TreeViewDegradeReason::BlockedStateHoverOnly)
    );

    let mut input = clean_tree_input();
    input.local_actions_hover_only = true;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::LocalActionsHoverOnly)
    );
}

#[test]
fn tree_count_scope_degrades() {
    let mut input = clean_tree_input();
    input.count_scopes_distinct = false;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::CountScopeCollapsed)
    );

    let mut input = clean_tree_input();
    input.count_scope = M5TreeListScopeKind::ScopeUnresolved;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::CountScopeUnresolved)
    );
}

#[test]
fn tree_stale_complete_and_overclaims_degrade() {
    let mut input = clean_tree_input();
    input.backend_stale_or_partial = true;
    input.presents_stale_or_partial_as_complete = true;
    let resolved = resolve_tree_view(input).unwrap();
    assert!(resolved.presents_stale_or_partial_as_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TreeViewDegradeReason::StaleOrPartialShownAsComplete)
    );

    let mut input = clean_tree_input();
    input.overclaims_drag_reorder = true;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::DragReorderOverclaimed)
    );

    let mut input = clean_tree_input();
    input.overclaims_cross_surface_continuity = true;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::CrossSurfaceContinuityOverclaimed)
    );

    let mut input = clean_tree_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_tree_view(input).unwrap().degrade_reason,
        Some(M5TreeViewDegradeReason::ContextTracePathMissing)
    );
}

#[test]
fn tree_empty_id_and_forbidden_material_error() {
    let mut input = clean_tree_input();
    input.tree_id = "".to_owned();
    assert_eq!(
        resolve_tree_view(input).unwrap_err(),
        M5TreeListResolutionError::EmptyTreeId
    );

    let mut input = clean_tree_input();
    input.node_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_tree_view(input).unwrap_err(),
        M5TreeListResolutionError::ForbiddenMaterial
    );
}

#[test]
fn list_clean_names_structure_and_is_legible() {
    let resolved = resolve_list_view(clean_list_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.structure_legible_at_a_glance);
    assert!(!resolved.loaded_shown_as_exact);
    assert_eq!(resolved.selection, "selected_and_current");
    assert_eq!(resolved.count_scope, "exact_count");
    assert!(resolved.continuity_continuous);
    assert_eq!(resolved.next_action, M5TreeListNextAction::OpenScopeDetail);
}

#[test]
fn list_loaded_shown_as_exact_degrades() {
    let mut input = clean_list_input();
    input.count_scope = M5TreeListScopeKind::LoadedCount;
    input.loaded_shown_as_exact = true;
    let resolved = resolve_list_view(input).unwrap();
    assert!(resolved.loaded_shown_as_exact);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ListViewDegradeReason::LoadedShownAsExact)
    );
}

#[test]
fn list_row_and_selection_degrade() {
    let mut input = clean_list_input();
    input.row_label = "  ".to_owned();
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::RowIdentityUnstated)
    );

    let mut input = clean_list_input();
    input.selection_versus_current_distinct = false;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::SelectionVersusCurrentCollapsed)
    );

    let mut input = clean_list_input();
    input.row_focus_visible = false;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::RowFocusNotVisible)
    );
}

#[test]
fn list_hover_only_and_count_degrade() {
    let mut input = clean_list_input();
    input.current_selection_hover_only = true;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::CurrentSelectionHoverOnly)
    );

    let mut input = clean_list_input();
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::BlockedStateHoverOnly)
    );

    let mut input = clean_list_input();
    input.local_actions_hover_only = true;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::LocalActionsHoverOnly)
    );

    let mut input = clean_list_input();
    input.count_scopes_distinct = false;
    assert_eq!(
        resolve_list_view(input).unwrap().degrade_reason,
        Some(M5ListViewDegradeReason::CountScopeCollapsed)
    );
}

#[test]
fn list_empty_id_and_forbidden_material_error() {
    let mut input = clean_list_input();
    input.list_id = "   ".to_owned();
    assert_eq!(
        resolve_list_view(input).unwrap_err(),
        M5TreeListResolutionError::EmptyListId
    );

    let mut input = clean_list_input();
    input.row_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_list_view(input).unwrap_err(),
        M5TreeListResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_tree_view_list_view_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.vocabulary_set.count_scope_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TREE_VIEW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TreeListAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5TreeListExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.controls_rows[0].list_view_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    // Force a clean tree to also read as hover-only — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.tree_view_examples[0].degrade_reason = None;
    row.tree_view_examples[0].current_selection_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_tree_view_list_view_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_current_selection_blocked_or_actions_behind_hover_only = true,
            1 => row.collapses_selection_versus_current_or_count_scopes = true,
            2 => row.presents_stale_partial_or_lazy_collection_as_complete = true,
            _ => row.overclaims_drag_reorder_or_cross_surface_continuity = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TreeListControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn shared_row_semantics_not_proven_when_list_scope_collapse_removed() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    for row in &mut packet.controls_rows {
        row.list_view_examples
            .retain(|ex| ex.degrade_reason != Some(M5ListViewDegradeReason::CountScopeCollapsed));
    }
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::SharedRowSemanticsNotProven));
}

#[test]
fn shared_row_semantics_not_proven_when_scopes_collapse_to_one() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    // Drop every clean example whose count scope is not "exact_count" so the grammar collapses.
    for row in &mut packet.controls_rows {
        row.tree_view_examples
            .retain(|ex| !(ex.is_clean() && ex.count_scope != "exact_count"));
        row.list_view_examples
            .retain(|ex| !(ex.is_clean() && ex.count_scope != "exact_count"));
    }
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::SharedRowSemanticsNotProven));
}

#[test]
fn selection_and_disclosure_truth_not_proven_when_lazy_leaf_removed() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    for row in &mut packet.controls_rows {
        row.tree_view_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TreeViewDegradeReason::LazySubtreeShownAsEmptyLeaf)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::SelectionAndDisclosureTruthNotProven));
}

#[test]
fn selection_and_disclosure_truth_not_proven_when_stale_complete_removed() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    for row in &mut packet.controls_rows {
        row.tree_view_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TreeViewDegradeReason::StaleOrPartialShownAsComplete)
        });
        row.list_view_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ListViewDegradeReason::StaleOrPartialShownAsComplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::SelectionAndDisclosureTruthNotProven));
}

#[test]
fn no_hover_only_discovery_not_proven_when_blocked_hover_removed() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    for row in &mut packet.controls_rows {
        row.tree_view_examples
            .retain(|ex| ex.degrade_reason != Some(M5TreeViewDegradeReason::BlockedStateHoverOnly));
        row.list_view_examples
            .retain(|ex| ex.degrade_reason != Some(M5ListViewDegradeReason::BlockedStateHoverOnly));
    }
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::NoHoverOnlyDiscoveryNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet
        .governance_review
        .tree_virtualization_honest_never_fakes_complete = false;
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet
        .consumer_projection
        .support_export_reads_single_collection_source = false;
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_tree_view_list_view_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TreeListControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_tree_view_list_view_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_tree_view_list_view_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_tree_view_list_view_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_tree_view_list_view_controls_export()
        .expect("checked M5 tree-view / list-view controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TREE_VIEW_LIST_VIEW_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_tree_view_list_view_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::ExplorerUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Beta
    );

    let preview = seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5TreeListControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-tree-view-list-view-controls/explorer_ui_beta_narrowed.json"
    )))
    .expect("explorer-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed()
    );

    let preview: M5TreeListControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-tree-view-list-view-controls/review_ui_preview_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_tree_view_and_list_view() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5NavigationContentComponentFamily::TreeView,
            M5NavigationContentComponentFamily::ListView,
        ]
    );
}
