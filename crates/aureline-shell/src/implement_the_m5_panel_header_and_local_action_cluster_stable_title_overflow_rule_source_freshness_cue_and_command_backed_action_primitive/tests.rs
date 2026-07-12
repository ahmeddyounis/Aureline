use super::*;

fn clean_header_input() -> M5PanelHeaderResolutionInput {
    M5PanelHeaderResolutionInput {
        header_id: "header:test".to_owned(),
        header_label: "Providers".to_owned(),
        title_slot_stable: true,
        active_context: M5ActiveContextState::ActiveCurrent,
        background_context_shown_as_active: false,
        source_freshness: M5PanelSourceFreshness::Current,
        freshness_cue_shown: false,
        overstates_readiness: false,
        references_canonical_model: true,
        re_encodes_canonical_counts_locally: false,
        refresh_command_available: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_cluster_input() -> M5LocalActionClusterResolutionInput {
    M5LocalActionClusterResolutionInput {
        cluster_id: "cluster:test".to_owned(),
        cluster_label: "Grid actions".to_owned(),
        local_action_budget: M5LocalActionBudget::WithinBudget,
        local_actions_hover_only: false,
        keyboard_reachable: true,
        advanced_actions_persistent_clutter: false,
        action_placement: M5PanelActionPlacement::InlinePrimary,
        overflowed_action_dropped: false,
        compaction_mode: M5PanelCompactionMode::CompactHeader,
        reinstantiates_different_surface: false,
        compaction_preserves_identity: true,
        compaction_preserves_action_semantics: true,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_panel_header_local_action_cluster_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_PACKET_ID
    );
}

#[test]
fn header_clean_names_title_context_and_is_legible() {
    let resolved = resolve_panel_header(clean_header_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.header_legible_at_a_glance);
    assert!(resolved.title_slot_stable);
    assert_eq!(resolved.active_context, "active_current");
    assert_eq!(resolved.source_freshness, "current");
    assert!(!resolved.source_is_qualified);
    assert!(resolved.references_canonical_model);
    assert_eq!(resolved.next_action, M5PanelNextAction::OpenPanelDetail);
}

#[test]
fn header_qualified_pane_is_clean_when_cue_shown() {
    let mut input = clean_header_input();
    input.source_freshness = M5PanelSourceFreshness::Cached;
    input.freshness_cue_shown = true;
    let resolved = resolve_panel_header(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.source_is_qualified);
    assert!(!resolved.freshness_cue_missing);
    assert!(!resolved.readiness_overstated);
}

#[test]
fn header_current_pane_needs_no_cue() {
    // A current pane flagged overstates-readiness stays clean: there is nothing to overstate.
    let mut input = clean_header_input();
    input.overstates_readiness = true;
    let resolved = resolve_panel_header(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.readiness_overstated);
}

#[test]
fn header_title_and_context_degrade() {
    let mut input = clean_header_input();
    input.header_label = "   ".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::HeaderTitleUnstated)
    );

    let mut input = clean_header_input();
    input.title_slot_stable = false;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::TitleSlotUnstable)
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
fn header_freshness_and_readiness_degrade() {
    let mut input = clean_header_input();
    input.source_freshness = M5PanelSourceFreshness::FreshnessUnknown;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::SourceFreshnessUnresolved)
    );

    let mut input = clean_header_input();
    input.source_freshness = M5PanelSourceFreshness::Cached;
    input.freshness_cue_shown = false;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::FreshnessCueMissing)
    );

    let mut input = clean_header_input();
    input.source_freshness = M5PanelSourceFreshness::Stale;
    input.freshness_cue_shown = true;
    input.overstates_readiness = true;
    let resolved = resolve_panel_header(input).unwrap();
    assert!(resolved.readiness_overstated);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PanelHeaderDegradeReason::ReadinessOverstated)
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
fn header_reencode_refresh_and_trace_degrade() {
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

    let mut input = clean_header_input();
    input.refresh_command_available = false;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::RefreshCommandMissing)
    );

    let mut input = clean_header_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_panel_header(input).unwrap().degrade_reason,
        Some(M5PanelHeaderDegradeReason::ContextTracePathMissing)
    );
}

#[test]
fn header_empty_id_and_forbidden_material_error() {
    let mut input = clean_header_input();
    input.header_id = "   ".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap_err(),
        M5PanelResolutionError::EmptyHeaderId
    );

    let mut input = clean_header_input();
    input.header_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_panel_header(input).unwrap_err(),
        M5PanelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn cluster_clean_names_actions_and_is_legible() {
    let resolved = resolve_local_action_cluster(clean_cluster_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.cluster_legible_at_a_glance);
    assert!(resolved.keyboard_reachable);
    assert!(resolved.compaction_is_compacted);
    assert!(!resolved.compaction_loses_identity);
    assert_eq!(resolved.action_placement, "inline_primary");
    assert_eq!(resolved.next_action, M5PanelNextAction::OpenPanelDetail);
}

#[test]
fn cluster_full_header_ignores_compaction_preservation_flags() {
    // A full (non-compacted) header cluster stays clean even if the preservation flags are false:
    // there is nothing compacted to preserve.
    let mut input = clean_cluster_input();
    input.compaction_mode = M5PanelCompactionMode::FullHeader;
    input.compaction_preserves_identity = false;
    input.reinstantiates_different_surface = true;
    let resolved = resolve_local_action_cluster(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.compaction_is_compacted);
    assert!(!resolved.compaction_loses_identity);
    assert!(!resolved.reinstantiates_different_surface);
}

#[test]
fn cluster_identity_budget_and_hover_degrade() {
    let mut input = clean_cluster_input();
    input.cluster_label = "  ".to_owned();
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::ClusterIdentityUnstated)
    );

    let mut input = clean_cluster_input();
    input.local_action_budget = M5LocalActionBudget::BudgetUnknown;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::ActionBudgetUnresolved)
    );

    let mut input = clean_cluster_input();
    input.local_actions_hover_only = true;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::LocalActionsHoverOnly)
    );

    let mut input = clean_cluster_input();
    input.keyboard_reachable = false;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::KeyboardAccessMissing)
    );
}

#[test]
fn cluster_clutter_placement_and_overflow_degrade() {
    let mut input = clean_cluster_input();
    input.advanced_actions_persistent_clutter = true;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::AdvancedActionsPersistentClutter)
    );

    let mut input = clean_cluster_input();
    input.action_placement = M5PanelActionPlacement::PlacementUnknown;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::ActionPlacementUnresolved)
    );

    let mut input = clean_cluster_input();
    input.overflowed_action_dropped = true;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::OverflowedActionDropped)
    );
}

#[test]
fn cluster_compaction_degrades() {
    let mut input = clean_cluster_input();
    input.compaction_mode = M5PanelCompactionMode::CompactionUnknown;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
    );

    let mut input = clean_cluster_input();
    input.reinstantiates_different_surface = true;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
    );

    let mut input = clean_cluster_input();
    input.compaction_preserves_identity = false;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::CompactionLosesPanelIdentity)
    );

    let mut input = clean_cluster_input();
    input.compaction_preserves_action_semantics = false;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::CompactionLosesActionSemantics)
    );

    let mut input = clean_cluster_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_local_action_cluster(input).unwrap().degrade_reason,
        Some(M5LocalActionClusterDegradeReason::ContextTracePathMissing)
    );
}

#[test]
fn cluster_empty_id_and_forbidden_material_error() {
    let mut input = clean_cluster_input();
    input.cluster_id = "".to_owned();
    assert_eq!(
        resolve_local_action_cluster(input).unwrap_err(),
        M5PanelResolutionError::EmptyClusterId
    );

    let mut input = clean_cluster_input();
    input.cluster_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_local_action_cluster(input).unwrap_err(),
        M5PanelResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_panel_header_local_action_cluster_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.vocabulary_set.source_freshness_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_PANEL_HEADER_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5PanelAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5PanelExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.controls_rows[0]
        .local_action_cluster_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    // Force a clean cluster to also read as hover-only — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.local_action_cluster_examples[0].degrade_reason = None;
    row.local_action_cluster_examples[0].local_actions_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_actions_behind_hover_only_or_loses_keyboard_access = true,
            1 => row.overstates_readiness_or_hides_source_freshness_cue = true,
            2 => row.overloads_header_or_keeps_advanced_actions_as_persistent_clutter = true,
            _ => row.compaction_reinstantiates_surface_or_loses_panel_identity = true,
        }
        assert!(packet
            .validate()
            .contains(&M5PanelControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn one_header_grammar_not_proven_when_cue_missing_removed() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    for row in &mut packet.controls_rows {
        row.panel_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PanelHeaderDegradeReason::FreshnessCueMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::OneHeaderGrammarNotProven));
}

#[test]
fn one_header_grammar_not_proven_when_freshness_collapses() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    // Drop every clean header whose source freshness is not "current" so the grammar collapses.
    for row in &mut packet.controls_rows {
        row.panel_header_examples
            .retain(|ex| !(ex.is_clean() && ex.source_freshness != "current"));
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::OneHeaderGrammarNotProven));
}

#[test]
fn compaction_not_proven_when_reinstantiate_removed() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    for row in &mut packet.controls_rows {
        row.local_action_cluster_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5LocalActionClusterDegradeReason::CompactionReinstantiatesSurface)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::CompactionIdentityAndActionSemanticsNotProven));
}

#[test]
fn compaction_not_proven_when_loses_identity_removed() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    for row in &mut packet.controls_rows {
        row.local_action_cluster_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5LocalActionClusterDegradeReason::CompactionLosesPanelIdentity)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::CompactionIdentityAndActionSemanticsNotProven));
}

#[test]
fn low_noise_header_not_proven_when_reencode_removed() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    for row in &mut packet.controls_rows {
        row.panel_header_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PanelHeaderDegradeReason::ReEncodesCanonicalCountsLocally)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::LowNoiseSufficientHeaderNotProven));
}

#[test]
fn low_noise_header_not_proven_when_persistent_clutter_removed() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    for row in &mut packet.controls_rows {
        row.local_action_cluster_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5LocalActionClusterDegradeReason::AdvancedActionsPersistentClutter)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::LowNoiseSufficientHeaderNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet
        .governance_review
        .compaction_preserves_panel_identity_and_action_semantics = false;
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet
        .consumer_projection
        .support_export_reads_single_panel_header_source = false;
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_panel_header_local_action_cluster_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PanelControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_panel_header_local_action_cluster_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_panel_header_local_action_cluster_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_panel_header_local_action_cluster_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_panel_header_local_action_cluster_controls_export()
        .expect("checked M5 panel-header / local-action-cluster controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PANEL_HEADER_LOCAL_ACTION_CLUSTER_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_panel_header_local_action_cluster_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::ShellUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Beta
    );

    let preview =
        seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5NavigationContentConsumerSurface::SupportExport)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5NavigationContentQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5PanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-panel-header-local-action-cluster-controls/shell_ui_beta_narrowed.json"
    )))
    .expect("shell-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_panel_header_local_action_cluster_controls_shell_ui_beta_narrowed()
    );

    let preview: M5PanelControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-panel-header-local-action-cluster-controls/support_export_preview_narrowed.json"
    )))
    .expect("support-export fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_panel_header_local_action_cluster_controls_support_export_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_panel_header() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5NavigationContentComponentFamily::PanelHeader]
    );
}
