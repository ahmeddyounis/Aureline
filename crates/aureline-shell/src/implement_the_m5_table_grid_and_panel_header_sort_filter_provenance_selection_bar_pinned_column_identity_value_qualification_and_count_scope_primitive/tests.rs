use super::*;

fn clean_grid_input() -> M5TableGridResolutionInput {
    M5TableGridResolutionInput {
        grid_id: "grid:test".to_owned(),
        grid_label: "provider row 1".to_owned(),
        selection: M5SelectionState::SelectedAndCurrent,
        selection_versus_current_distinct: true,
        row_focus_visible: true,
        current_selection_hover_only: false,
        item_state: M5ItemStateFlag::Pinned,
        has_blocked_row: false,
        blocked_state_hover_only: false,
        sort_filter_provenance: M5SortFilterProvenance::UserSorted,
        pinned_column: M5PinnedColumnState::IdentityColumnPinned,
        pinned_column_identity_lost: false,
        value_qualification: M5ValueQualification::ExactCanonical,
        qualified_value_shown_as_canonical: false,
        count_scope: M5TablePanelScopeKind::ExactCount,
        count_scopes_distinct: true,
        loaded_shown_as_exact: false,
        density: M5DensityVariant::Comfortable,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        backend_stale_or_partial: false,
        presents_stale_or_partial_as_complete: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_header_input() -> M5PanelHeaderResolutionInput {
    M5PanelHeaderResolutionInput {
        header_id: "header:test".to_owned(),
        header_label: "Providers".to_owned(),
        active_context: M5ActiveContextState::ActiveCurrent,
        background_context_shown_as_active: false,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        becomes_secondary_toolbar: false,
        overflowed_action_dropped: false,
        references_canonical_model: true,
        re_encodes_canonical_counts_locally: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_table_grid_panel_header_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_PACKET_ID
    );
}

#[test]
fn grid_clean_names_structure_and_is_legible() {
    let resolved = resolve_table_grid(clean_grid_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.structure_legible_at_a_glance);
    assert!(!resolved.current_selection_hover_only);
    assert!(!resolved.loaded_shown_as_exact);
    assert_eq!(resolved.selection, "selected_and_current");
    assert_eq!(resolved.sort_filter_provenance, "user_sorted");
    assert_eq!(resolved.pinned_column, "identity_column_pinned");
    assert!(resolved.pinned_column_pinned);
    assert_eq!(resolved.value_qualification, "exact_canonical");
    assert!(!resolved.value_is_qualified);
    assert_eq!(resolved.count_scope, "exact_count");
    assert_eq!(
        resolved.next_action,
        M5TablePanelNextAction::OpenScopeDetail
    );
}

#[test]
fn grid_qualified_value_is_clean_when_not_shown_canonical() {
    let mut input = clean_grid_input();
    input.value_qualification = M5ValueQualification::Imported;
    input.backend_stale_or_partial = true;
    let resolved = resolve_table_grid(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.value_is_qualified);
    assert!(!resolved.qualified_value_shown_as_canonical);
}

#[test]
fn grid_identity_and_selection_degrade() {
    let mut input = clean_grid_input();
    input.grid_label = "   ".to_owned();
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::GridIdentityUnstated)
    );

    let mut input = clean_grid_input();
    input.selection_versus_current_distinct = false;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::SelectionVersusCurrentCollapsed)
    );

    let mut input = clean_grid_input();
    input.selection = M5SelectionState::SelectionUnknown;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::SelectionVersusCurrentCollapsed)
    );

    let mut input = clean_grid_input();
    input.row_focus_visible = false;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::RowFocusNotVisible)
    );
}

#[test]
fn grid_hover_and_blocked_degrade() {
    let mut input = clean_grid_input();
    input.current_selection_hover_only = true;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::CurrentSelectionHoverOnly)
    );

    let mut input = clean_grid_input();
    input.item_state = M5ItemStateFlag::Blocked;
    input.blocked_state_hover_only = true;
    let resolved = resolve_table_grid(input).unwrap();
    assert!(resolved.has_blocked_row);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TableGridDegradeReason::BlockedStateHoverOnly)
    );

    let mut input = clean_grid_input();
    input.local_actions_hover_only = true;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::LocalActionsHoverOnly)
    );
}

#[test]
fn grid_provenance_pin_and_value_degrade() {
    let mut input = clean_grid_input();
    input.sort_filter_provenance = M5SortFilterProvenance::ProvenanceUnknown;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::SortFilterProvenanceUnstated)
    );

    let mut input = clean_grid_input();
    input.pinned_column_identity_lost = true;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::PinnedColumnIdentityLost)
    );

    let mut input = clean_grid_input();
    input.pinned_column = M5PinnedColumnState::PinUnresolved;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::PinnedColumnUnresolved)
    );

    let mut input = clean_grid_input();
    input.value_qualification = M5ValueQualification::Estimated;
    input.qualified_value_shown_as_canonical = true;
    let resolved = resolve_table_grid(input).unwrap();
    assert!(resolved.qualified_value_shown_as_canonical);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TableGridDegradeReason::QualifiedValueShownAsCanonical)
    );

    let mut input = clean_grid_input();
    input.value_qualification = M5ValueQualification::QualificationUnknown;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::ValueQualificationUnresolved)
    );
}

#[test]
fn grid_exact_canonical_never_flags_qualified_overclaim() {
    // An exact canonical value flagged shown-as-canonical is not a qualified overclaim; the grid
    // stays clean because there is nothing to qualify.
    let mut input = clean_grid_input();
    input.qualified_value_shown_as_canonical = true;
    let resolved = resolve_table_grid(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.qualified_value_shown_as_canonical);
}

#[test]
fn grid_count_scope_and_loaded_degrade() {
    let mut input = clean_grid_input();
    input.count_scopes_distinct = false;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::CountScopeCollapsed)
    );

    let mut input = clean_grid_input();
    input.count_scope = M5TablePanelScopeKind::ScopeUnresolved;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::CountScopeUnresolved)
    );

    let mut input = clean_grid_input();
    input.count_scope = M5TablePanelScopeKind::LoadedCount;
    input.loaded_shown_as_exact = true;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::LoadedShownAsExact)
    );
}

#[test]
fn grid_stale_complete_and_trace_degrade() {
    let mut input = clean_grid_input();
    input.backend_stale_or_partial = true;
    input.presents_stale_or_partial_as_complete = true;
    let resolved = resolve_table_grid(input).unwrap();
    assert!(resolved.presents_stale_or_partial_as_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TableGridDegradeReason::StaleOrPartialShownAsComplete)
    );

    let mut input = clean_grid_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_table_grid(input).unwrap().degrade_reason,
        Some(M5TableGridDegradeReason::ContextTracePathMissing)
    );
}

#[test]
fn grid_empty_id_and_forbidden_material_error() {
    let mut input = clean_grid_input();
    input.grid_id = "".to_owned();
    assert_eq!(
        resolve_table_grid(input).unwrap_err(),
        M5TablePanelResolutionError::EmptyGridId
    );

    let mut input = clean_grid_input();
    input.grid_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_table_grid(input).unwrap_err(),
        M5TablePanelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn header_clean_names_context_and_is_legible() {
    let resolved = resolve_panel_header(clean_header_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.header_legible_at_a_glance);
    assert_eq!(resolved.active_context, "active_current");
    assert!(resolved.references_canonical_model);
    assert_eq!(
        resolved.next_action,
        M5TablePanelNextAction::OpenScopeDetail
    );
}

#[test]
fn header_identity_context_and_background_degrade() {
    let mut input = clean_header_input();
    input.header_label = "  ".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::HeaderIdentityUnstated)
    );

    let mut input = clean_header_input();
    input.active_context = M5ActiveContextState::ContextUnresolved;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::ActiveContextUnresolved)
    );

    let mut input = clean_header_input();
    input.active_context = M5ActiveContextState::BackgroundOpen;
    input.background_context_shown_as_active = true;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::BackgroundContextShownAsActive)
    );
}

#[test]
fn header_background_shown_honestly_is_clean() {
    let mut input = clean_header_input();
    input.active_context = M5ActiveContextState::BackgroundOpen;
    let resolved = resolve_panel_header(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(resolved.active_context, "background_open");
}

#[test]
fn header_overload_overflow_and_reencode_degrade() {
    let mut input = clean_header_input();
    input.local_actions_hover_only = true;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::LocalActionsHoverOnly)
    );

    let mut input = clean_header_input();
    input.becomes_secondary_toolbar = true;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::PanelHeaderOverloadedAsToolbar)
    );

    let mut input = clean_header_input();
    input.overflowed_action_dropped = true;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::OverflowedActionDropped)
    );

    let mut input = clean_header_input();
    input.re_encodes_canonical_counts_locally = true;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    );

    let mut input = clean_header_input();
    input.references_canonical_model = false;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
    );
}

#[test]
fn header_empty_id_and_forbidden_material_error() {
    let mut input = clean_header_input();
    input.header_id = "   ".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap_err(),
        M5TablePanelResolutionError::EmptyHeaderId
    );

    let mut input = clean_header_input();
    input.header_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap_err(),
        M5TablePanelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_table_grid_panel_header_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.vocabulary_set.sort_filter_provenances.pop();
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_PANEL_HEADER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TablePanelAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5TablePanelExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.controls_rows[0].panel_header_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    // Force a clean grid to also read as hover-only — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.table_grid_examples[0].degrade_reason = None;
    row.table_grid_examples[0].current_selection_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_table_grid_panel_header_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_current_selection_blocked_or_actions_behind_hover_only = true,
            1 => row.collapses_selection_versus_current_or_count_scopes = true,
            2 => row.presents_qualified_stale_or_partial_grid_as_canonical = true,
            _ => row.panel_header_overloads_or_re_encodes_counts = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TablePanelControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn shared_semantics_not_proven_when_provenance_unstated_removed() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    for row in &mut packet.controls_rows {
        row.table_grid_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TableGridDegradeReason::SortFilterProvenanceUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::SharedSortFilterAndCountSemanticsNotProven));
}

#[test]
fn shared_semantics_not_proven_when_scopes_collapse_to_one() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    // Drop every clean grid whose count scope is not "exact_count" so the grammar collapses.
    for row in &mut packet.controls_rows {
        row.table_grid_examples
            .retain(|ex| !(ex.is_clean() && ex.count_scope != "exact_count"));
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::SharedSortFilterAndCountSemanticsNotProven));
}

#[test]
fn pinned_identity_not_proven_when_pinned_lost_removed() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    for row in &mut packet.controls_rows {
        row.table_grid_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TableGridDegradeReason::PinnedColumnIdentityLost)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::PinnedIdentityAndProvenanceTruthNotProven));
}

#[test]
fn pinned_identity_not_proven_when_qualified_as_canonical_removed() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    for row in &mut packet.controls_rows {
        row.table_grid_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TableGridDegradeReason::QualifiedValueShownAsCanonical)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::PinnedIdentityAndProvenanceTruthNotProven));
}

#[test]
fn canonical_header_not_proven_when_overload_removed() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    for row in &mut packet.controls_rows {
        row.panel_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PanelHeaderDegradeReason::PanelHeaderOverloadedAsToolbar)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::CanonicalHeaderAndSelectionModelNotProven));
}

#[test]
fn canonical_header_not_proven_when_reencode_removed() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    for row in &mut packet.controls_rows {
        row.panel_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::CanonicalHeaderAndSelectionModelNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet
        .governance_review
        .table_pinned_column_identity_stable_under_virtualization = false;
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet
        .consumer_projection
        .support_export_reads_single_grid_source = false;
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_table_grid_panel_header_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TablePanelControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_table_grid_panel_header_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_table_grid_panel_header_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_table_grid_panel_header_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_table_grid_panel_header_controls_export()
        .expect("checked M5 table-grid / panel-header controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TABLE_GRID_PANEL_HEADER_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_table_grid_panel_header_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_table_grid_panel_header_controls_data_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::DataUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Beta
    );

    let preview = seeded_m5_table_grid_panel_header_controls_review_ui_preview_narrowed();
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
    let beta: M5TablePanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-table-grid-panel-header-controls/data_ui_beta_narrowed.json"
    )))
    .expect("data-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_table_grid_panel_header_controls_data_ui_beta_narrowed()
    );

    let preview: M5TablePanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-table-grid-panel-header-controls/review_ui_preview_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_table_grid_panel_header_controls_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_table_grid_and_panel_header() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5NavigationContentComponentFamily::TableGrid,
            M5NavigationContentComponentFamily::PanelHeader,
        ]
    );
}
