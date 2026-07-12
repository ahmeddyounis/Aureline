use super::*;

fn clean_badge_input() -> M5BadgeResolutionInput {
    M5BadgeResolutionInput {
        badge_id: "badge:test".to_owned(),
        badge_label: "Beta".to_owned(),
        expression: M5BadgeExpression::StatusWord,
        disposition: M5DecisionFeedbackDisposition::Info,
        meaning_taxonomy: M5BadgeMeaningTaxonomy::LifecycleState,
        overflow_behavior: M5BadgeOverflowBehavior::NoOverflow,
        expansion_route: M5BadgeExpansionRoute::DisclosureDrawer,
        surface_context: M5DecisionSurfaceContext::HelpPanel,
        meaning_stated_non_color_only: true,
        plain_language_explanation_present: true,
        explanation_reachable_by_keyboard_sr_export: true,
        taxonomy_stable_across_surfaces: true,
        proof_fresh: true,
    }
}

fn clean_popover_input() -> M5PopoverResolutionInput {
    M5PopoverResolutionInput {
        popover_id: "popover:test".to_owned(),
        accessible_name: "What does Beta mean?".to_owned(),
        dismissal: M5PopoverDismissal::DismissOnEscape,
        disposition: M5DecisionFeedbackDisposition::Info,
        surface_context: M5DecisionSurfaceContext::HelpPanel,
        is_dismissible: true,
        keyboard_operable: true,
        focus_returns_to_trigger: true,
        carries_only_critical_instruction: false,
        critical_steps_available_elsewhere: true,
        is_non_modal_secondary: true,
        content_reachable_without_hover: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_badge_popover_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BADGE_POPOVER_CONTROLS_PACKET_ID);
}

#[test]
fn badge_clean_names_meaning_and_is_legible() {
    let resolved = resolve_badge(clean_badge_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.meaning_legible_without_hover);
    assert!(!resolved.expression_is_color_only);
    assert!(resolved.meaning_is_classified);
    assert_eq!(resolved.expression, "status_word");
    assert_eq!(resolved.meaning_taxonomy, "lifecycle_state");
    assert_eq!(resolved.surface_context, "help_panel");
    assert_eq!(
        resolved.next_action,
        M5BadgePopoverNextAction::ExpandBadgeMeaning
    );
}

#[test]
fn badge_label_unstated_degrades() {
    let mut input = clean_badge_input();
    input.badge_label = "   ".to_owned();
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::BadgeLabelUnstated)
    );
}

#[test]
fn badge_color_only_degrades() {
    let mut input = clean_badge_input();
    input.meaning_stated_non_color_only = false;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::MeaningEncodedByColorAlone)
    );

    let mut input = clean_badge_input();
    input.expression = M5BadgeExpression::ColorOnlyDisallowed;
    let resolved = resolve_badge(input).unwrap();
    assert!(resolved.expression_is_color_only);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BadgeDegradeReason::MeaningEncodedByColorAlone)
    );
}

#[test]
fn badge_hover_only_and_plain_missing_degrade() {
    let mut input = clean_badge_input();
    input.explanation_reachable_by_keyboard_sr_export = false;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::ExpansionOnlyViaHover)
    );

    let mut input = clean_badge_input();
    input.plain_language_explanation_present = false;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::PlainLanguageExplanationMissing)
    );
}

#[test]
fn badge_taxonomy_unclassified_and_drift_degrade() {
    let mut input = clean_badge_input();
    input.meaning_taxonomy = M5BadgeMeaningTaxonomy::TaxonomyUnclassified;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::MeaningTaxonomyUnclassified)
    );

    let mut input = clean_badge_input();
    input.taxonomy_stable_across_surfaces = false;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::TaxonomyDriftedAcrossSurface)
    );
}

#[test]
fn badge_surface_and_overflow_unresolved_degrade() {
    let mut input = clean_badge_input();
    input.surface_context = M5DecisionSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::SurfaceContextUnresolved)
    );

    let mut input = clean_badge_input();
    input.overflow_behavior = M5BadgeOverflowBehavior::BehaviorUnknown;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::OverflowBehaviorUnresolved)
    );

    let mut input = clean_badge_input();
    input.expansion_route = M5BadgeExpansionRoute::RouteUnknown;
    assert_eq!(
        resolve_badge(input).unwrap().degrade_reason,
        Some(M5BadgeDegradeReason::ExpansionRouteUnreachable)
    );
}

#[test]
fn badge_empty_id_and_forbidden_material_error() {
    let mut input = clean_badge_input();
    input.badge_id = "".to_owned();
    assert_eq!(
        resolve_badge(input).unwrap_err(),
        M5BadgePopoverResolutionError::EmptyBadgeId
    );

    let mut input = clean_badge_input();
    input.badge_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_badge(input).unwrap_err(),
        M5BadgePopoverResolutionError::ForbiddenMaterial
    );
}

#[test]
fn popover_clean_stays_lightweight_with_safe_focus() {
    let resolved = resolve_popover(clean_popover_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.stays_lightweight_secondary_with_safe_focus);
    assert!(resolved.focus_returns_to_trigger);
    assert!(!resolved.carries_only_critical_instruction);
    assert_eq!(resolved.dismissal, "dismiss_on_escape");
    assert_eq!(resolved.surface_context, "help_panel");
}

#[test]
fn popover_no_focus_return_degrades() {
    let mut input = clean_popover_input();
    input.focus_returns_to_trigger = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::FocusDoesNotReturnToTrigger)
    );
}

#[test]
fn popover_carries_only_instruction_and_trapped_steps_degrade() {
    let mut input = clean_popover_input();
    input.carries_only_critical_instruction = true;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::CarriesOnlyCriticalInstruction)
    );

    let mut input = clean_popover_input();
    input.critical_steps_available_elsewhere = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::CriticalStepsTrappedInPopover)
    );
}

#[test]
fn popover_dismissal_disallowed_and_not_dismissible_degrade() {
    let mut input = clean_popover_input();
    input.dismissal = M5PopoverDismissal::CarriesOnlyInstructionDisallowed;
    let resolved = resolve_popover(input).unwrap();
    assert!(resolved.dismissal_is_disallowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PopoverDegradeReason::DismissalModelDisallowed)
    );

    let mut input = clean_popover_input();
    input.is_dismissible = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::NotDismissible)
    );
}

#[test]
fn popover_keyboard_and_hover_and_modal_degrade() {
    let mut input = clean_popover_input();
    input.keyboard_operable = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::KeyboardOperationMissing)
    );

    let mut input = clean_popover_input();
    input.is_non_modal_secondary = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::NotLightweightSecondary)
    );

    let mut input = clean_popover_input();
    input.content_reachable_without_hover = false;
    assert_eq!(
        resolve_popover(input).unwrap().degrade_reason,
        Some(M5PopoverDegradeReason::ContentReachableOnlyOnHover)
    );
}

#[test]
fn popover_empty_id_and_forbidden_material_error() {
    let mut input = clean_popover_input();
    input.popover_id = "   ".to_owned();
    assert_eq!(
        resolve_popover(input).unwrap_err(),
        M5BadgePopoverResolutionError::EmptyPopoverId
    );

    let mut input = clean_popover_input();
    input.accessible_name = "see internal://notes".to_owned();
    assert_eq!(
        resolve_popover(input).unwrap_err(),
        M5BadgePopoverResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_badge_popover_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.vocabulary_set.meaning_taxonomies.pop();
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BADGE_CHIP_PILL_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5BadgePopoverAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5BadgePopoverExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.controls_rows[0].popover_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    // Force a clean badge to also read as color-only meaning — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.badge_examples[0].degrade_reason = None;
    row.badge_examples[0].meaning_stated_non_color_only = false;
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_badge_popover_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.badge_meaning_relies_on_color_alone = true,
            1 => row.badge_meaning_hidden_behind_hover_only = true,
            2 => row.popover_carries_only_critical_instruction = true,
            _ => row.popover_fails_to_return_focus_to_trigger = true,
        }
        assert!(packet
            .validate()
            .contains(&M5BadgePopoverControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn badge_taxonomy_not_proven_when_color_only_example_removed() {
    let mut packet = seeded_m5_badge_popover_controls();
    for row in &mut packet.controls_rows {
        row.badge_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BadgeDegradeReason::MeaningEncodedByColorAlone)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::BadgeTaxonomyAndPopoverFocusNotProven));
}

#[test]
fn badge_taxonomy_not_proven_when_expression_grammar_collapses() {
    let mut packet = seeded_m5_badge_popover_controls();
    // Drop every clean status-word badge so the expression grammar no longer covers "status_word".
    for row in &mut packet.controls_rows {
        row.badge_examples
            .retain(|ex| !(ex.is_clean() && ex.expression == "status_word"));
    }
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::BadgeTaxonomyAndPopoverFocusNotProven));
}

#[test]
fn popover_focus_not_proven_when_focus_example_removed() {
    let mut packet = seeded_m5_badge_popover_controls();
    for row in &mut packet.controls_rows {
        row.popover_examples.retain(|ex| {
            ex.degrade_reason != Some(M5PopoverDegradeReason::FocusDoesNotReturnToTrigger)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::BadgeTaxonomyAndPopoverFocusNotProven));
}

#[test]
fn plain_language_not_proven_when_plain_missing_example_removed() {
    let mut packet = seeded_m5_badge_popover_controls();
    for row in &mut packet.controls_rows {
        row.badge_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BadgeDegradeReason::PlainLanguageExplanationMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::PlainLanguageReachabilityNotProven));
}

#[test]
fn drift_not_proven_when_taxonomy_drift_example_removed() {
    let mut packet = seeded_m5_badge_popover_controls();
    for row in &mut packet.controls_rows {
        row.badge_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BadgeDegradeReason::TaxonomyDriftedAcrossSurface)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::BadgePopoverDriftNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet
        .governance_review
        .popover_never_carries_only_critical_instruction = false;
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet
        .consumer_projection
        .support_export_reads_single_badge_source = false;
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_badge_popover_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BadgePopoverControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_badge_popover_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_badge_popover_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_badge_popover_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_badge_popover_controls_export()
        .expect("checked M5 badge / popover controls export validates");
    assert_eq!(from_disk.packet_id, M5_BADGE_POPOVER_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_badge_popover_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_badge_popover_controls_help_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::HelpUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Beta
    );

    let preview = seeded_m5_badge_popover_controls_review_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5BadgePopoverControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-chip-pill-and-popover-controls/help_ui_beta_narrowed.json"
    )))
    .expect("help-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_badge_popover_controls_help_ui_beta_narrowed()
    );

    let preview: M5BadgePopoverControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-badge-chip-pill-and-popover-controls/review_ui_preview_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_badge_popover_controls_review_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_badge_and_popover() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5DecisionFeedbackFamily::BadgeChipPill,
            M5DecisionFeedbackFamily::Popover
        ]
    );
}
