use super::*;

fn clean_tab_input() -> M5TabStripResolutionInput {
    M5TabStripResolutionInput {
        strip_id: "tab:test".to_owned(),
        active_context_label: "main.rs".to_owned(),
        active_context: M5ActiveContextState::ActiveCurrent,
        item_state: M5TabItemState::Pinned,
        item_state_stated: true,
        local_action_budget: M5LocalActionBudget::WithinBudget,
        has_blocked_tab: false,
        blocked_tab_stated: true,
        reads_as_top_level_workflow_navigation: false,
        invents_surface_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_trail_input() -> M5BreadcrumbsResolutionInput {
    M5BreadcrumbsResolutionInput {
        trail_id: "trail:test".to_owned(),
        leaf_label: "main.rs".to_owned(),
        ancestry_kind: M5BreadcrumbAncestryKind::FilePath,
        hierarchy_path: M5HierarchyPathState::FullPathShown,
        path_explicit_in_compact_and_expanded: true,
        collapses_missing_scope_into_ellipsis: false,
        presents_partial_or_stale_as_complete: false,
        reads_as_top_level_workflow_navigation: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_tab_strip_breadcrumbs_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_PACKET_ID
    );
}

#[test]
fn tab_clean_names_context_and_is_legible() {
    let resolved = resolve_tab_strip(clean_tab_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.context_legible_at_a_glance);
    assert!(!resolved.presents_as_top_level_navigation);
    assert!(!resolved.invents_surface_local_badge);
    assert_eq!(resolved.active_context, "active_current");
    assert_eq!(resolved.item_state, "pinned");
    assert_eq!(
        resolved.next_action,
        M5TabBreadcrumbsNextAction::OpenContextDetail
    );
}

#[test]
fn tab_shared_and_reopened_states_are_named() {
    let mut input = clean_tab_input();
    input.item_state = M5TabItemState::Shared;
    let shared = resolve_tab_strip(input).unwrap();
    assert!(shared.is_clean());
    assert!(shared.item_state_shared);
    assert_eq!(shared.item_state, "shared");

    let mut input = clean_tab_input();
    input.item_state = M5TabItemState::Reopened;
    let reopened = resolve_tab_strip(input).unwrap();
    assert!(reopened.is_clean());
    assert!(reopened.item_state_reopened);
    assert_eq!(reopened.item_state, "reopened");
}

#[test]
fn tab_active_context_unstated_degrades() {
    let mut input = clean_tab_input();
    input.active_context_label = "   ".to_owned();
    let resolved = resolve_tab_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TabStripDegradeReason::ActiveContextUnstated)
    );
}

#[test]
fn tab_masquerade_degrades() {
    let mut input = clean_tab_input();
    input.reads_as_top_level_workflow_navigation = true;
    let resolved = resolve_tab_strip(input).unwrap();
    assert!(resolved.presents_as_top_level_navigation);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TabStripDegradeReason::TabsMasqueradeAsTopLevelNavigation)
    );
    assert_eq!(
        resolved.next_action,
        M5TabBreadcrumbsNextAction::OpenContextDetail
    );
}

#[test]
fn tab_badge_invented_degrades() {
    let mut input = clean_tab_input();
    input.invents_surface_local_badge = true;
    let resolved = resolve_tab_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TabStripDegradeReason::SurfaceLocalBadgeInvented)
    );
}

#[test]
fn tab_item_state_unknown_and_color_only_degrade() {
    let mut input = clean_tab_input();
    input.item_state = M5TabItemState::StateUnknown;
    assert_eq!(
        resolve_tab_strip(input).unwrap().degrade_reason,
        Some(M5TabStripDegradeReason::ItemStateUnresolved)
    );

    let mut input = clean_tab_input();
    input.item_state_stated = false;
    assert_eq!(
        resolve_tab_strip(input).unwrap().degrade_reason,
        Some(M5TabStripDegradeReason::ItemStateHiddenBehindColorOnly)
    );
}

#[test]
fn tab_blocked_hidden_degrades() {
    let mut input = clean_tab_input();
    input.item_state = M5TabItemState::Blocked;
    input.blocked_tab_stated = false;
    let resolved = resolve_tab_strip(input).unwrap();
    assert!(resolved.has_blocked_tab);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5TabStripDegradeReason::BlockedTabHiddenBehindEllipsis)
    );
}

#[test]
fn tab_trace_missing_degrades() {
    let mut input = clean_tab_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_tab_strip(input).unwrap().degrade_reason,
        Some(M5TabStripDegradeReason::ContextTracePathMissing)
    );
}

#[test]
fn tab_empty_id_and_forbidden_material_error() {
    let mut input = clean_tab_input();
    input.strip_id = "".to_owned();
    assert_eq!(
        resolve_tab_strip(input).unwrap_err(),
        M5TabBreadcrumbsResolutionError::EmptyStripId
    );

    let mut input = clean_tab_input();
    input.active_context_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_tab_strip(input).unwrap_err(),
        M5TabBreadcrumbsResolutionError::ForbiddenMaterial
    );
}

#[test]
fn trail_clean_names_ancestry_and_is_explicit() {
    let resolved = resolve_breadcrumbs(clean_trail_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.path_explicit);
    assert!(resolved.source_aware);
    assert_eq!(resolved.ancestry_kind, "file_path");
    assert_eq!(resolved.hierarchy_path, "full_path_shown");
}

#[test]
fn trail_partial_hierarchy_is_clean_when_not_shown_complete() {
    let mut input = clean_trail_input();
    input.hierarchy_path = M5HierarchyPathState::PartialHierarchy;
    let resolved = resolve_breadcrumbs(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.path_is_partial_or_stale);
    assert!(!resolved.presents_partial_or_stale_as_complete);
}

#[test]
fn trail_leaf_unstated_and_ancestry_unknown_degrade() {
    let mut input = clean_trail_input();
    input.leaf_label = "  ".to_owned();
    assert_eq!(
        resolve_breadcrumbs(input).unwrap().degrade_reason,
        Some(M5BreadcrumbsDegradeReason::LeafIdentityUnstated)
    );

    let mut input = clean_trail_input();
    input.ancestry_kind = M5BreadcrumbAncestryKind::AncestryUnknown;
    let resolved = resolve_breadcrumbs(input).unwrap();
    assert!(!resolved.source_aware);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BreadcrumbsDegradeReason::AncestryKindUnresolved)
    );
}

#[test]
fn trail_masquerade_degrades() {
    let mut input = clean_trail_input();
    input.reads_as_top_level_workflow_navigation = true;
    assert_eq!(
        resolve_breadcrumbs(input).unwrap().degrade_reason,
        Some(M5BreadcrumbsDegradeReason::BreadcrumbsMasqueradeAsTopLevelNavigation)
    );
}

#[test]
fn trail_ellipsis_collapse_degrades() {
    let mut input = clean_trail_input();
    input.hierarchy_path = M5HierarchyPathState::TruncatedMiddle;
    input.collapses_missing_scope_into_ellipsis = true;
    let resolved = resolve_breadcrumbs(input).unwrap();
    assert!(resolved.collapses_missing_scope_into_ellipsis);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BreadcrumbsDegradeReason::MissingScopeCollapsedIntoEllipsis)
    );
}

#[test]
fn trail_partial_shown_complete_degrades() {
    let mut input = clean_trail_input();
    input.hierarchy_path = M5HierarchyPathState::StaleHierarchy;
    input.presents_partial_or_stale_as_complete = true;
    let resolved = resolve_breadcrumbs(input).unwrap();
    assert!(resolved.presents_partial_or_stale_as_complete);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BreadcrumbsDegradeReason::PartialOrStaleShownAsComplete)
    );
}

#[test]
fn trail_not_explicit_and_trace_missing_degrade() {
    let mut input = clean_trail_input();
    input.path_explicit_in_compact_and_expanded = false;
    assert_eq!(
        resolve_breadcrumbs(input).unwrap().degrade_reason,
        Some(M5BreadcrumbsDegradeReason::PathNotExplicitAcrossViews)
    );

    let mut input = clean_trail_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_breadcrumbs(input).unwrap().degrade_reason,
        Some(M5BreadcrumbsDegradeReason::AncestryTracePathMissing)
    );
}

#[test]
fn trail_empty_id_and_forbidden_material_error() {
    let mut input = clean_trail_input();
    input.trail_id = "   ".to_owned();
    assert_eq!(
        resolve_breadcrumbs(input).unwrap_err(),
        M5TabBreadcrumbsResolutionError::EmptyTrailId
    );

    let mut input = clean_trail_input();
    input.leaf_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_breadcrumbs(input).unwrap_err(),
        M5TabBreadcrumbsResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_tab_strip_breadcrumbs_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.vocabulary_set.tab_item_states.pop();
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_TAB_STRIP_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TabBreadcrumbsAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5TabBreadcrumbsExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.controls_rows[0].breadcrumbs_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    // Force a clean tab to also read as masquerading — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.tab_strip_examples[0].degrade_reason = None;
    row.tab_strip_examples[0].presents_as_top_level_navigation = true;
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.tabs_masquerade_as_top_level_workflow_navigation = true,
            1 => row.breadcrumbs_masquerade_as_top_level_workflow_navigation = true,
            2 => row.invents_surface_local_badges_for_shared_context = true,
            _ => row.collapses_missing_scope_or_hides_blocked_behind_ellipsis = true,
        }
        assert!(packet
            .validate()
            .contains(&M5TabBreadcrumbsControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn tab_state_grammar_not_proven_when_masquerade_example_removed() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    for row in &mut packet.controls_rows {
        row.tab_strip_examples.retain(|ex| {
            ex.degrade_reason != Some(M5TabStripDegradeReason::TabsMasqueradeAsTopLevelNavigation)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::TabStateGrammarNotProven));
}

#[test]
fn tab_state_grammar_not_proven_when_states_collapse_to_one() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    // Drop every clean tab whose item state is not "pinned" so the grammar collapses to one state.
    for row in &mut packet.controls_rows {
        row.tab_strip_examples
            .retain(|ex| !(ex.is_clean() && ex.item_state != "pinned"));
    }
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::TabStateGrammarNotProven));
}

#[test]
fn breadcrumb_explicitness_not_proven_when_ellipsis_example_removed() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    for row in &mut packet.controls_rows {
        row.breadcrumbs_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BreadcrumbsDegradeReason::MissingScopeCollapsedIntoEllipsis)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::BreadcrumbExplicitnessNotProven));
}

#[test]
fn breadcrumb_explicitness_not_proven_when_partial_shown_complete_removed() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    for row in &mut packet.controls_rows {
        row.breadcrumbs_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BreadcrumbsDegradeReason::PartialOrStaleShownAsComplete)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::BreadcrumbExplicitnessNotProven));
}

#[test]
fn traceability_not_proven_when_clean_breadcrumbs_lose_trace_path() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    for row in &mut packet.controls_rows {
        for trail in &mut row.breadcrumbs_examples {
            if trail.is_clean() {
                trail.detail_command_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations
        .contains(&M5TabBreadcrumbsControlsViolation::ContextAndAncestryTraceabilityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.governance_review.breadcrumbs_explicit_across_views = false;
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet
        .consumer_projection
        .support_export_reads_single_navigation_source = false;
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_tab_strip_breadcrumbs_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TabBreadcrumbsControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_tab_strip_breadcrumbs_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_tab_strip_breadcrumbs_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_tab_strip_breadcrumbs_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_tab_strip_breadcrumbs_controls_export()
        .expect("checked M5 tab-strip / breadcrumbs controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TAB_STRIP_BREADCRUMBS_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_tab_strip_breadcrumbs_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Beta
    );

    let preview = seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::SearchUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5TabBreadcrumbsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-tab-strip-breadcrumbs-controls/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed()
    );

    let preview: M5TabBreadcrumbsControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-tab-strip-breadcrumbs-controls/search_ui_preview_narrowed.json"
    )))
    .expect("search-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_tab_strip_and_breadcrumbs() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5NavigationContentComponentFamily::TabStrip,
            M5NavigationContentComponentFamily::Breadcrumbs,
        ]
    );
}
