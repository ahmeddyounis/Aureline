use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_core_action_input_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CORE_CONTROL_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_core_action_input_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5CoreControlFamily::ALL {
        assert!(
            present.contains(&family),
            "missing control family {}",
            family.as_str()
        );
    }
    assert_eq!(packet.component_rows.len(), M5CoreControlFamily::ALL.len());
}

#[test]
fn frozen_interaction_state_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: default / hover / focus-visible / pressed / loading /
    // disabled / locked / read-only / degraded stays in one controlled token set that no forms,
    // settings, search, entry, review, or repair surface reinvents.
    let tokens: Vec<&str> = M5CoreControlDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "default",
            "hover",
            "focus_visible",
            "pressed",
            "loading",
            "disabled",
            "locked",
            "read_only",
            "degraded",
        ]
    );
    assert!(M5CoreControlDisposition::Disabled.is_interaction_blocked());
    assert!(M5CoreControlDisposition::Locked.is_interaction_blocked());
    assert!(M5CoreControlDisposition::ReadOnly.is_interaction_blocked());
    assert!(!M5CoreControlDisposition::Loading.is_interaction_blocked());
}

#[test]
fn every_component_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_core_action_input_component_matrix();
    for row in &packet.component_rows {
        for label in M5CoreControlRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "control {} missing mandatory label {}",
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
            "control {} does not point at its canonical schema",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.dispositions.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5CoreControlAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_core_action_input_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.button_emphases.is_empty(),
            family.declares_button_emphasis(),
            "button_emphases presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.icon_label_modes.is_empty(),
            family.declares_icon_label_mode(),
            "icon_label_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.split_postures.is_empty(),
            family.declares_split_posture(),
            "split_postures presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.field_label_modes.is_empty(),
            family.declares_field_label_mode(),
            "field_label_modes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.field_validations.is_empty(),
            family.declares_field_validation(),
            "field_validations presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.search_affordances.is_empty(),
            family.declares_search_affordance(),
            "search_affordances presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.combobox_value_sources.is_empty(),
            family.declares_combobox_value_source(),
            "combobox_value_sources presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.toggle_semantics.is_empty(),
            family.declares_toggle_semantics(),
            "toggle_semantics presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.segmented_modes.is_empty(),
            family.declares_segmented_mode(),
            "segmented_modes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_core_action_input_component_matrix();
    for disposition in M5CoreControlDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no control declares interaction state {}",
            disposition.as_str()
        );
    }
    for emphasis in M5ButtonEmphasis::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.button_emphases.contains(&emphasis)),
            "no control declares button emphasis {}",
            emphasis.as_str()
        );
    }
    for mode in M5IconLabelMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.icon_label_modes.contains(&mode)),
            "no control declares icon-label mode {}",
            mode.as_str()
        );
    }
    for posture in M5SplitDefaultPosture::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.split_postures.contains(&posture)),
            "no control declares split posture {}",
            posture.as_str()
        );
    }
    for mode in M5FieldLabelMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.field_label_modes.contains(&mode)),
            "no control declares field label mode {}",
            mode.as_str()
        );
    }
    for state in M5FieldValidationState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.field_validations.contains(&state)),
            "no control declares field validation {}",
            state.as_str()
        );
    }
    for affordance in M5SearchFieldAffordance::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.search_affordances.contains(&affordance)),
            "no control declares search affordance {}",
            affordance.as_str()
        );
    }
    for source in M5ComboboxValueSource::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.combobox_value_sources.contains(&source)),
            "no control declares combobox value source {}",
            source.as_str()
        );
    }
    for semantics in M5ToggleSemantics::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.toggle_semantics.contains(&semantics)),
            "no control declares toggle semantics {}",
            semantics.as_str()
        );
    }
    for mode in M5SegmentedMode::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.segmented_modes.contains(&mode)),
            "no control declares segmented mode {}",
            mode.as_str()
        );
    }
    for reason in M5CoreControlDegradedReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no control declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5CoreControlFamily::Combobox);
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5CoreControlRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let own = M5CoreControlFamily::Button.canonical_component_schema_ref();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::Button)
        .expect("button row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentSchemaRefMissing));
}

#[test]
fn disposition_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::DispositionMissing));
}

#[test]
fn button_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::Button)
        .expect("button present");
    row.button_emphases.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ButtonEmphasisMissing));
}

#[test]
fn icon_button_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::IconButton)
        .expect("icon-button present");
    row.icon_label_modes.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::IconLabelModeMissing));
}

#[test]
fn split_button_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::SplitButton)
        .expect("split-button present");
    row.split_postures.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::SplitPostureMissing));
}

#[test]
fn text_field_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_core_action_input_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5CoreControlFamily::TextField)
            .expect("text-field present");
        let expected = if clear == 0 {
            row.field_label_modes.clear();
            M5CoreControlComponentMatrixViolation::FieldLabelModeMissing
        } else {
            row.field_validations.clear();
            M5CoreControlComponentMatrixViolation::FieldValidationMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn search_field_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::SearchField)
        .expect("search-field present");
    row.search_affordances.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::SearchAffordanceMissing));
}

#[test]
fn combobox_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::Combobox)
        .expect("combobox present");
    row.combobox_value_sources.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComboboxValueSourceMissing));
}

#[test]
fn toggle_control_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::ToggleControl)
        .expect("toggle-control present");
    row.toggle_semantics.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ToggleSemanticsMissing));
}

#[test]
fn segmented_control_vocab_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::SegmentedControl)
        .expect("segmented-control present");
    row.segmented_modes.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::SegmentedModeMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[3].lets_placeholder_text_replace_the_label = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[0].lets_loading_relabel_the_action_or_lose_attribution = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[1].leaves_icon_only_destructive_action_unlabeled = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[6].blurs_switch_with_deferred_checkbox = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[2].lets_split_button_default_to_riskier_alternate = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[3].hides_locked_or_degraded_semantics_behind_generic_disabled = true;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5CoreControlFamily::Button)
        .expect("button row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.governance_review.placeholder_never_replaces_label = false;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_core_action_input_component_matrix().render_markdown_summary();
    for family in M5CoreControlFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing control {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_core_action_input_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CoreControlFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,canonical_schema,"));
    for family in M5CoreControlFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing control {}",
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
    let packet = current_stable_m5_core_action_input_component_matrix_export()
        .expect("checked M5 core-action-input component matrix export validates");
    assert_eq!(packet.packet_id, M5_CORE_CONTROL_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_core_action_input_component_matrix_export()
        .expect("checked M5 core-action-input component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_core_action_input_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed(),
        seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.component_rows.len(), M5CoreControlFamily::ALL.len());
    }

    let combobox = seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed();
    let row = combobox
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CoreControlFamily::Combobox)
        .expect("combobox row present");
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Beta);

    let segmented =
        seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed();
    let row = segmented
        .component_rows
        .iter()
        .find(|r| r.component_family == M5CoreControlFamily::SegmentedControl)
        .expect("segmented-control row present");
    assert_eq!(row.qualification, M5CoreControlQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let combobox: M5CoreControlComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-core-action-input-components/combobox_beta_narrowed.json"
    )))
    .expect("combobox fixture parses");
    assert!(combobox.validate().is_empty());
    assert_eq!(
        combobox,
        seeded_m5_core_action_input_component_matrix_combobox_beta_narrowed()
    );

    let segmented: M5CoreControlComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-core-action-input-components/segmented_control_preview_narrowed.json"
    )))
        .expect("segmented-control fixture parses");
    assert!(segmented.validate().is_empty());
    assert_eq!(
        segmented,
        seeded_m5_core_action_input_component_matrix_segmented_control_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_core_action_input_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_core_action_input_component_matrix();
    packet.component_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CoreControlComponentMatrixViolation::RawMaterialInExport));
}
