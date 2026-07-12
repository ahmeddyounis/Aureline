use super::*;

fn clean_button_input() -> M5ButtonResolutionInput {
    M5ButtonResolutionInput {
        button_id: "button:test".to_owned(),
        action_label: "Submit request".to_owned(),
        emphasis: M5ButtonEmphasis::Primary,
        emphasis_stated: true,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5ActionSurfaceContext::PaneHeader,
        loading_behavior: M5ButtonLoadingBehavior::NotLoading,
        loading_preserves_label_and_width: true,
        blocked_state_distinct: true,
        command_id: "command:forms.submit".to_owned(),
        forks_feature_local_style: false,
        command_route_available: true,
        proof_fresh: true,
    }
}

fn clean_icon_input() -> M5IconButtonResolutionInput {
    M5IconButtonResolutionInput {
        icon_button_id: "icon:test".to_owned(),
        accessible_name: "Filter results".to_owned(),
        label_mode: M5IconLabelMode::AccessibleNameOnly,
        emphasis: M5ButtonEmphasis::Quiet,
        disposition: M5CoreControlDisposition::Default,
        surface_context: M5ActionSurfaceContext::PaneHeader,
        command_surface: M5ActionCommandSurface::InlineTrigger,
        tooltip_parity: true,
        command_id: "command:forms.filter".to_owned(),
        command_parity_across_surfaces: true,
        invents_brand_only_affordance: false,
        command_route_available: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_button_icon_button_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BUTTON_ICON_BUTTON_CONTROLS_PACKET_ID);
}

#[test]
fn button_clean_names_emphasis_and_is_attributable() {
    let resolved = resolve_button(clean_button_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.action_attributable_at_a_glance);
    assert!(!resolved.forks_feature_local_style);
    assert_eq!(resolved.emphasis, "primary");
    assert_eq!(resolved.disposition, "default");
    assert_eq!(resolved.surface_context, "pane_header");
    assert_eq!(
        resolved.next_action,
        M5ButtonIconButtonNextAction::OpenCommandDetail
    );
}

#[test]
fn button_loading_preserves_label_and_width() {
    let mut input = clean_button_input();
    input.disposition = M5CoreControlDisposition::Loading;
    input.loading_behavior = M5ButtonLoadingBehavior::LabelPreservedSpinnerLeading;
    let resolved = resolve_button(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_loading);
    assert!(resolved.loading_preserves_label_and_width);
}

#[test]
fn button_label_unstated_degrades() {
    let mut input = clean_button_input();
    input.action_label = "   ".to_owned();
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::ActionLabelUnstated)
    );
}

#[test]
fn button_loading_relabel_degrades() {
    let mut input = clean_button_input();
    input.disposition = M5CoreControlDisposition::Loading;
    input.loading_behavior = M5ButtonLoadingBehavior::WidthReservedLabelKept;
    input.loading_preserves_label_and_width = false;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::LoadingRelabeledOrResized)
    );
}

#[test]
fn button_locked_hidden_behind_disabled_degrades() {
    let mut input = clean_button_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = false;
    let resolved = resolve_button(input).unwrap();
    assert!(resolved.disposition_is_blocked);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ButtonDegradeReason::LockedOrDegradedHiddenBehindDisabled)
    );
}

#[test]
fn button_locked_distinct_is_clean() {
    let mut input = clean_button_input();
    input.disposition = M5CoreControlDisposition::Locked;
    input.blocked_state_distinct = true;
    let resolved = resolve_button(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.disposition_is_blocked);
    assert!(resolved.blocked_state_distinct);
}

#[test]
fn button_style_fork_and_color_only_degrade() {
    let mut input = clean_button_input();
    input.forks_feature_local_style = true;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::FeatureLocalStyleForked)
    );

    let mut input = clean_button_input();
    input.emphasis_stated = false;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::EmphasisEncodedByColorAlone)
    );
}

#[test]
fn button_command_and_trace_degrade() {
    let mut input = clean_button_input();
    input.command_id = "   ".to_owned();
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_button_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn button_surface_and_loading_behavior_unresolved_degrade() {
    let mut input = clean_button_input();
    input.surface_context = M5ActionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_button_input();
    input.loading_behavior = M5ButtonLoadingBehavior::BehaviorUnknown;
    assert_eq!(
        resolve_button(input).unwrap().degrade_reason,
        Some(M5ButtonDegradeReason::LoadingBehaviorUnresolved)
    );
}

#[test]
fn button_empty_id_and_forbidden_material_error() {
    let mut input = clean_button_input();
    input.button_id = "".to_owned();
    assert_eq!(
        resolve_button(input).unwrap_err(),
        M5ButtonIconButtonResolutionError::EmptyButtonId
    );

    let mut input = clean_button_input();
    input.action_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_button(input).unwrap_err(),
        M5ButtonIconButtonResolutionError::ForbiddenMaterial
    );
}

#[test]
fn icon_clean_exposes_name_and_is_legible() {
    let resolved = resolve_icon_button(clean_icon_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.name_and_command_legible_at_a_glance);
    assert!(resolved.exposes_accessible_name);
    assert!(resolved.command_parity_across_surfaces);
    assert_eq!(resolved.label_mode, "accessible_name_only");
    assert_eq!(resolved.command_surface, "inline_trigger");
}

#[test]
fn icon_destructive_labeled_is_clean() {
    let mut input = clean_icon_input();
    input.emphasis = M5ButtonEmphasis::Destructive;
    input.label_mode = M5IconLabelMode::TooltipLabeled;
    input.accessible_name = "Delete comment".to_owned();
    let resolved = resolve_icon_button(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.emphasis_is_destructive);
    assert!(resolved.exposes_accessible_name);
}

#[test]
fn icon_destructive_unlabeled_degrades() {
    let mut input = clean_icon_input();
    input.emphasis = M5ButtonEmphasis::Destructive;
    input.label_mode = M5IconLabelMode::DecorativeOnly;
    input.accessible_name = "Delete comment".to_owned();
    let resolved = resolve_icon_button(input).unwrap();
    assert!(!resolved.exposes_accessible_name);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5IconButtonDegradeReason::IconOnlyDestructiveUnlabeled)
    );
}

#[test]
fn icon_name_unstated_and_label_mode_unresolved_degrade() {
    let mut input = clean_icon_input();
    input.accessible_name = "  ".to_owned();
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::AccessibleNameUnstated)
    );

    let mut input = clean_icon_input();
    input.label_mode = M5IconLabelMode::LabelUnresolved;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::LabelModeUnresolved)
    );
}

#[test]
fn icon_brand_only_and_tooltip_parity_degrade() {
    let mut input = clean_icon_input();
    input.invents_brand_only_affordance = true;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::BrandOnlyAffordanceInvented)
    );

    let mut input = clean_icon_input();
    input.tooltip_parity = false;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::TooltipParityMissing)
    );
}

#[test]
fn icon_command_surface_and_parity_degrade() {
    let mut input = clean_icon_input();
    input.command_surface = M5ActionCommandSurface::SurfaceUnknown;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::CommandSurfaceUnresolved)
    );

    let mut input = clean_icon_input();
    input.command_parity_across_surfaces = false;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::CommandParityBrokenAcrossSurfaces)
    );
}

#[test]
fn icon_command_unstated_and_trace_missing_degrade() {
    let mut input = clean_icon_input();
    input.command_id = "  ".to_owned();
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::CommandBindingUnstated)
    );

    let mut input = clean_icon_input();
    input.command_route_available = false;
    assert_eq!(
        resolve_icon_button(input).unwrap().degrade_reason,
        Some(M5IconButtonDegradeReason::CommandTracePathMissing)
    );
}

#[test]
fn icon_empty_id_and_forbidden_material_error() {
    let mut input = clean_icon_input();
    input.icon_button_id = "   ".to_owned();
    assert_eq!(
        resolve_icon_button(input).unwrap_err(),
        M5ButtonIconButtonResolutionError::EmptyIconButtonId
    );

    let mut input = clean_icon_input();
    input.accessible_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_icon_button(input).unwrap_err(),
        M5ButtonIconButtonResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_button_icon_button_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.vocabulary_set.loading_behaviors.pop();
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BUTTON_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ButtonIconButtonAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5ButtonIconButtonExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.controls_rows[0].icon_button_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    // Force a clean button to also read as forking a feature-local style — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.button_examples[0].degrade_reason = None;
    row.button_examples[0].forks_feature_local_style = true;
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_button_icon_button_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.buttons_relabel_or_resize_when_loading = true,
            1 => row.icon_only_destructive_actions_go_unlabeled = true,
            2 => row.locked_or_degraded_semantics_hidden_behind_disabled = true,
            _ => row.controls_fork_feature_local_styles = true,
        }
        assert!(packet
            .validate()
            .contains(&M5ButtonIconButtonControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn button_state_behavior_not_proven_when_relabel_example_removed() {
    let mut packet = seeded_m5_button_icon_button_controls();
    for row in &mut packet.controls_rows {
        row.button_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ButtonDegradeReason::LoadingRelabeledOrResized)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ButtonStateBehaviorNotProven));
}

#[test]
fn button_state_behavior_not_proven_when_emphasis_grammar_collapses() {
    let mut packet = seeded_m5_button_icon_button_controls();
    // Drop every clean destructive button so the emphasis grammar no longer covers "destructive".
    for row in &mut packet.controls_rows {
        row.button_examples
            .retain(|ex| !(ex.is_clean() && ex.emphasis == "destructive"));
    }
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ButtonStateBehaviorNotProven));
}

#[test]
fn icon_parity_not_proven_when_unlabeled_example_removed() {
    let mut packet = seeded_m5_button_icon_button_controls();
    for row in &mut packet.controls_rows {
        row.icon_button_examples.retain(|ex| {
            ex.degrade_reason != Some(M5IconButtonDegradeReason::IconOnlyDestructiveUnlabeled)
        });
    }
    assert!(packet.validate().contains(
        &M5ButtonIconButtonControlsViolation::IconAccessibleNameAndCommandParityNotProven
    ));
}

#[test]
fn icon_parity_not_proven_when_brand_only_example_removed() {
    let mut packet = seeded_m5_button_icon_button_controls();
    for row in &mut packet.controls_rows {
        row.icon_button_examples.retain(|ex| {
            ex.degrade_reason != Some(M5IconButtonDegradeReason::BrandOnlyAffordanceInvented)
        });
    }
    assert!(packet.validate().contains(
        &M5ButtonIconButtonControlsViolation::IconAccessibleNameAndCommandParityNotProven
    ));
}

#[test]
fn traceability_not_proven_when_clean_icons_lose_command_route() {
    let mut packet = seeded_m5_button_icon_button_controls();
    for row in &mut packet.controls_rows {
        for i in &mut row.icon_button_examples {
            if i.is_clean() {
                i.command_route_available = false;
            }
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ButtonIconButtonControlsViolation::ButtonStateTraceabilityNotProven)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet
        .governance_review
        .icon_button_never_unlabeled_when_destructive = false;
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet
        .consumer_projection
        .support_export_reads_single_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_button_icon_button_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ButtonIconButtonControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_button_icon_button_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_button_icon_button_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_button_icon_button_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_button_icon_button_controls_export()
        .expect("checked M5 button / icon-button controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_BUTTON_ICON_BUTTON_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_button_icon_button_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_button_icon_button_controls_forms_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::FormsUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Beta);

    let preview = seeded_m5_button_icon_button_controls_review_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5CoreControlConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ButtonIconButtonControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-button-icon-button-controls/forms_ui_beta_narrowed.json"
    )))
    .expect("forms-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_button_icon_button_controls_forms_ui_beta_narrowed()
    );

    let preview: M5ButtonIconButtonControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-button-icon-button-controls/review_ui_preview_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_button_icon_button_controls_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_button_and_icon_button() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [M5CoreControlFamily::Button, M5CoreControlFamily::IconButton]
    );
}
