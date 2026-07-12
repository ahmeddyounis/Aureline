use super::*;

fn clean_tab_input() -> M5EditorTabResolutionInput {
    M5EditorTabResolutionInput {
        tab_id: "tab:test".to_owned(),
        file_session_label: "main.rs".to_owned(),
        tab_context: M5EditorTabState::ActiveCurrent,
        item_state: M5EditorTabItemState::Pinned,
        item_state_stated: true,
        pane_kind: M5EditorPaneKind::SingleEditor,
        reopen_reveal_continuity_preserved: true,
        has_blocked_tab: false,
        blocked_tab_stated: true,
        invents_feature_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

fn clean_gutter_input() -> M5GutterResolutionInput {
    M5GutterResolutionInput {
        gutter_id: "gutter:test".to_owned(),
        anchor_label: "main.rs:42".to_owned(),
        marker_kind: M5GutterMarkerKind::Breakpoint,
        marker_layer: M5GutterMarkerLayer::Breakpoint,
        marker_kind_stated: true,
        diagnostic_severity: M5DiagnosticSeverity::Info,
        severity_stated: true,
        layer_precedence_preserved: true,
        readable_in_compact_and_export: true,
        invents_feature_local_badge: false,
        detail_command_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_editor_tab_gutter_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EDITOR_TAB_GUTTER_CONTROLS_PACKET_ID);
}

#[test]
fn tab_clean_names_context_and_is_legible() {
    let resolved = resolve_editor_tab(clean_tab_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.state_legible_at_a_glance);
    assert!(!resolved.invents_feature_local_badge);
    assert_eq!(resolved.tab_context, "active_current");
    assert_eq!(resolved.item_state, "pinned");
    assert_eq!(resolved.pane_kind, "single_editor");
    assert_eq!(
        resolved.next_action,
        M5EditorTabGutterNextAction::OpenStateDetail
    );
}

#[test]
fn tab_shared_generated_and_remote_states_are_named() {
    let mut input = clean_tab_input();
    input.item_state = M5EditorTabItemState::Shared;
    let shared = resolve_editor_tab(input).unwrap();
    assert!(shared.is_clean());
    assert!(shared.item_state_shared);
    assert_eq!(shared.item_state, "shared");

    let mut input = clean_tab_input();
    input.item_state = M5EditorTabItemState::Generated;
    let generated = resolve_editor_tab(input).unwrap();
    assert!(generated.is_clean());
    assert!(generated.item_state_generated);
    assert_eq!(generated.item_state, "generated");

    let mut input = clean_tab_input();
    input.item_state = M5EditorTabItemState::Remote;
    let remote = resolve_editor_tab(input).unwrap();
    assert!(remote.is_clean());
    assert!(remote.item_state_remote);
    assert_eq!(remote.item_state, "remote");
}

#[test]
fn tab_identity_unstated_degrades() {
    let mut input = clean_tab_input();
    input.file_session_label = "   ".to_owned();
    let resolved = resolve_editor_tab(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EditorTabDegradeReason::FileSessionIdentityUnstated)
    );
}

#[test]
fn tab_context_unresolved_degrades() {
    let mut input = clean_tab_input();
    input.tab_context = M5EditorTabState::ContextUnresolved;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::TabContextUnresolved)
    );
}

#[test]
fn tab_badge_invented_degrades() {
    let mut input = clean_tab_input();
    input.invents_feature_local_badge = true;
    let resolved = resolve_editor_tab(input).unwrap();
    assert!(resolved.invents_feature_local_badge);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EditorTabDegradeReason::FeatureLocalBadgeInvented)
    );
}

#[test]
fn tab_item_state_unknown_and_color_only_degrade() {
    let mut input = clean_tab_input();
    input.item_state = M5EditorTabItemState::StateUnknown;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::ItemStateUnresolved)
    );

    let mut input = clean_tab_input();
    input.item_state_stated = false;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::StateEncodedByColorAlone)
    );
}

#[test]
fn tab_blocked_hidden_degrades() {
    let mut input = clean_tab_input();
    input.item_state = M5EditorTabItemState::Blocked;
    input.blocked_tab_stated = false;
    let resolved = resolve_editor_tab(input).unwrap();
    assert!(resolved.has_blocked_tab);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EditorTabDegradeReason::BlockedTabHiddenBehindColorOrEllipsis)
    );
}

#[test]
fn tab_continuity_lost_and_trace_missing_degrade() {
    let mut input = clean_tab_input();
    input.reopen_reveal_continuity_preserved = false;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::ReopenRevealContinuityLost)
    );

    let mut input = clean_tab_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::StateTracePathMissing)
    );
}

#[test]
fn tab_pane_unresolved_degrades() {
    let mut input = clean_tab_input();
    input.pane_kind = M5EditorPaneKind::PaneUnknown;
    assert_eq!(
        resolve_editor_tab(input).unwrap().degrade_reason,
        Some(M5EditorTabDegradeReason::PaneContextUnresolved)
    );
}

#[test]
fn tab_empty_id_and_forbidden_material_error() {
    let mut input = clean_tab_input();
    input.tab_id = "".to_owned();
    assert_eq!(
        resolve_editor_tab(input).unwrap_err(),
        M5EditorTabGutterResolutionError::EmptyTabId
    );

    let mut input = clean_tab_input();
    input.file_session_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_editor_tab(input).unwrap_err(),
        M5EditorTabGutterResolutionError::ForbiddenMaterial
    );
}

#[test]
fn gutter_clean_names_marker_and_is_legible() {
    let resolved = resolve_gutter(clean_gutter_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.layering_legible_at_a_glance);
    assert!(resolved.layer_precedence_preserved);
    assert_eq!(resolved.marker_kind, "breakpoint");
    assert_eq!(resolved.marker_layer, "breakpoint");
    assert_eq!(
        resolved.layer_precedence,
        M5GutterMarkerLayer::Breakpoint.precedence()
    );
    assert!(!resolved.is_diagnostic_layer);
}

#[test]
fn gutter_diagnostic_layer_names_severity() {
    let mut input = clean_gutter_input();
    input.marker_layer = M5GutterMarkerLayer::Diagnostic;
    input.diagnostic_severity = M5DiagnosticSeverity::Error;
    let resolved = resolve_gutter(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_diagnostic_layer);
    assert_eq!(resolved.diagnostic_severity, "error");
    assert_eq!(resolved.layer_precedence, 0);
}

#[test]
fn gutter_anchor_unstated_and_kind_unresolved_degrade() {
    let mut input = clean_gutter_input();
    input.anchor_label = "  ".to_owned();
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::GutterAnchorUnstated)
    );

    let mut input = clean_gutter_input();
    input.marker_kind = M5GutterMarkerKind::MarkerUnresolved;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::MarkerKindUnresolved)
    );
}

#[test]
fn gutter_layer_unresolved_and_badge_degrade() {
    let mut input = clean_gutter_input();
    input.marker_layer = M5GutterMarkerLayer::LayerUnresolved;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::MarkerLayerUnresolved)
    );

    let mut input = clean_gutter_input();
    input.invents_feature_local_badge = true;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::FeatureLocalBadgeInvented)
    );
}

#[test]
fn gutter_marker_color_only_degrades() {
    let mut input = clean_gutter_input();
    input.marker_kind_stated = false;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::MarkerEncodedByColorAlone)
    );
}

#[test]
fn gutter_severity_unresolved_and_color_only_degrade() {
    let mut input = clean_gutter_input();
    input.marker_layer = M5GutterMarkerLayer::Diagnostic;
    input.diagnostic_severity = M5DiagnosticSeverity::SeverityUnknown;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::DiagnosticSeverityUnresolved)
    );

    let mut input = clean_gutter_input();
    input.marker_layer = M5GutterMarkerLayer::Diagnostic;
    input.diagnostic_severity = M5DiagnosticSeverity::Warning;
    input.severity_stated = false;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::SeverityEncodedByColorAlone)
    );
}

#[test]
fn gutter_severity_ignored_off_diagnostic_layer() {
    // An unresolved severity off the diagnostic layer must not degrade a breakpoint marker.
    let mut input = clean_gutter_input();
    input.diagnostic_severity = M5DiagnosticSeverity::SeverityUnknown;
    input.severity_stated = false;
    let resolved = resolve_gutter(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.is_diagnostic_layer);
}

#[test]
fn gutter_precedence_lost_and_unreadable_degrade() {
    let mut input = clean_gutter_input();
    input.layer_precedence_preserved = false;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::LayerPrecedenceLost)
    );

    let mut input = clean_gutter_input();
    input.readable_in_compact_and_export = false;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::MarkerLayeringNotReadableInCompactOrExport)
    );
}

#[test]
fn gutter_reveal_missing_degrades() {
    let mut input = clean_gutter_input();
    input.detail_command_available = false;
    assert_eq!(
        resolve_gutter(input).unwrap().degrade_reason,
        Some(M5GutterDegradeReason::RevealTracePathMissing)
    );
}

#[test]
fn gutter_empty_id_and_forbidden_material_error() {
    let mut input = clean_gutter_input();
    input.gutter_id = "   ".to_owned();
    assert_eq!(
        resolve_gutter(input).unwrap_err(),
        M5EditorTabGutterResolutionError::EmptyGutterId
    );

    let mut input = clean_gutter_input();
    input.anchor_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_gutter(input).unwrap_err(),
        M5EditorTabGutterResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_editor_tab_gutter_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.vocabulary_set.tab_item_states.pop();
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_EDITOR_TAB_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5EditorTabGutterAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5EditorTabGutterExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.controls_rows[0].gutter_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    // Force a clean tab to also read as inventing a badge — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.editor_tab_examples[0].degrade_reason = None;
    row.editor_tab_examples[0].invents_feature_local_badge = true;
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_editor_tab_gutter_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.tabs_invent_feature_local_badges_for_file_session_state = true,
            1 => row.gutter_markers_encode_state_by_color_alone = true,
            2 => row.gutter_marker_layering_loses_identity_or_precedence = true,
            _ => row.reopen_reveal_continuity_breaks_across_panes = true,
        }
        assert!(packet
            .validate()
            .contains(&M5EditorTabGutterControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn state_grammar_not_proven_when_badge_example_removed() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    for row in &mut packet.controls_rows {
        row.editor_tab_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EditorTabDegradeReason::FeatureLocalBadgeInvented)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::TabAndGutterStateGrammarNotProven));
}

#[test]
fn state_grammar_not_proven_when_states_collapse_to_one() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    // Drop every clean tab whose item state is not "pinned" so the grammar collapses to one state.
    for row in &mut packet.controls_rows {
        row.editor_tab_examples
            .retain(|ex| !(ex.is_clean() && ex.item_state != "pinned"));
    }
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::TabAndGutterStateGrammarNotProven));
}

#[test]
fn marker_layering_not_proven_when_precedence_example_removed() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    for row in &mut packet.controls_rows {
        row.gutter_examples
            .retain(|ex| ex.degrade_reason != Some(M5GutterDegradeReason::LayerPrecedenceLost));
    }
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::MarkerLayeringReadabilityNotProven));
}

#[test]
fn marker_layering_not_proven_when_unreadable_example_removed() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    for row in &mut packet.controls_rows {
        row.gutter_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5GutterDegradeReason::MarkerLayeringNotReadableInCompactOrExport)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::MarkerLayeringReadabilityNotProven));
}

#[test]
fn traceability_not_proven_when_clean_gutters_lose_reveal_path() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    for row in &mut packet.controls_rows {
        for g in &mut row.gutter_examples {
            if g.is_clean() {
                g.detail_command_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5EditorTabGutterControlsViolation::StateTraceabilityNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet
        .governance_review
        .gutter_layering_readable_across_representations = false;
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet
        .consumer_projection
        .support_export_reads_single_editor_source = false;
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_editor_tab_gutter_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5EditorTabGutterControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_editor_tab_gutter_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_editor_tab_gutter_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_editor_tab_gutter_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_editor_tab_gutter_controls_export()
        .expect("checked M5 editor-tab / gutter controls export validates");
    assert_eq!(from_disk.packet_id, M5_EDITOR_TAB_GUTTER_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_editor_tab_gutter_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::EditorUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Beta);

    let preview = seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EditorInlineConsumerSurface::DiagnosticsUi)
        .unwrap();
    assert_eq!(row.qualification, M5EditorInlineQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5EditorTabGutterControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-editor-tab-gutter-controls/editor_ui_beta_narrowed.json"
    )))
    .expect("editor-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_editor_tab_gutter_controls_editor_ui_beta_narrowed()
    );

    let preview: M5EditorTabGutterControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-editor-tab-gutter-controls/diagnostics_ui_preview_narrowed.json"
    )))
    .expect("diagnostics-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_editor_tab_gutter_controls_diagnostics_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_editor_tab_and_gutter() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5EditorInlineComponentFamily::EditorTab,
            M5EditorInlineComponentFamily::Gutter,
        ]
    );
}
