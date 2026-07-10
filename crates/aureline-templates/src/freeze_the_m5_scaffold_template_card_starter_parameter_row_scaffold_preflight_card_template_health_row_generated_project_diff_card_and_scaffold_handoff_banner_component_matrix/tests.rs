use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_scaffold_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SCAFFOLD_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_scaffold_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5ScaffoldComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5ScaffoldComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_dispositions_and_deployment_lines() {
    let packet = seeded_m5_scaffold_component_matrix();
    for row in &packet.component_rows {
        for label in M5ScaffoldRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.dispositions.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_scaffold_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.starter_source_classes.is_empty(),
            family.is_scaffold_template_card(),
            "starter_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.template_support_classes.is_empty(),
            family.is_scaffold_template_card(),
            "template_support_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.parameter_source_layers.is_empty(),
            family.is_starter_parameter_row(),
            "parameter_source_layers presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.parameter_action_timings.is_empty(),
            family.is_starter_parameter_row(),
            "parameter_action_timings presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.preflight_check_classes.is_empty(),
            family.is_scaffold_preflight_card(),
            "preflight_check_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.preflight_result_states.is_empty(),
            family.is_scaffold_preflight_card(),
            "preflight_result_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.health_signal_classes.is_empty(),
            family.is_template_health_row(),
            "health_signal_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.health_freshness_states.is_empty(),
            family.is_template_health_row(),
            "health_freshness_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.generated_zone_classes.is_empty(),
            family.is_generated_project_diff_card(),
            "generated_zone_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.diff_review_states.is_empty(),
            family.is_generated_project_diff_card(),
            "diff_review_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_outcome_classes.is_empty(),
            family.is_scaffold_handoff_banner(),
            "handoff_outcome_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_recovery_actions.is_empty(),
            family.is_scaffold_handoff_banner(),
            "handoff_recovery_actions presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_scaffold_component_matrix();
    for disposition in M5ScaffoldDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for class in M5StarterSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.starter_source_classes.contains(&class)),
            "no component declares starter source class {}",
            class.as_str()
        );
    }
    for class in M5TemplateSupportClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.template_support_classes.contains(&class)),
            "no component declares template support class {}",
            class.as_str()
        );
    }
    for layer in M5ParameterSourceLayer::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.parameter_source_layers.contains(&layer)),
            "no component declares parameter source layer {}",
            layer.as_str()
        );
    }
    for timing in M5ParameterActionTiming::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.parameter_action_timings.contains(&timing)),
            "no component declares parameter action timing {}",
            timing.as_str()
        );
    }
    for class in M5PreflightCheckClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.preflight_check_classes.contains(&class)),
            "no component declares preflight check class {}",
            class.as_str()
        );
    }
    for state in M5PreflightResultState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.preflight_result_states.contains(&state)),
            "no component declares preflight result state {}",
            state.as_str()
        );
    }
    for class in M5HealthSignalClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.health_signal_classes.contains(&class)),
            "no component declares health signal class {}",
            class.as_str()
        );
    }
    for state in M5HealthFreshnessState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.health_freshness_states.contains(&state)),
            "no component declares health freshness state {}",
            state.as_str()
        );
    }
    for class in M5GeneratedZoneClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.generated_zone_classes.contains(&class)),
            "no component declares generated zone class {}",
            class.as_str()
        );
    }
    for state in M5DiffReviewState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.diff_review_states.contains(&state)),
            "no component declares diff review state {}",
            state.as_str()
        );
    }
    for class in M5HandoffOutcomeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_outcome_classes.contains(&class)),
            "no component declares handoff outcome class {}",
            class.as_str()
        );
    }
    for action in M5HandoffRecoveryAction::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_recovery_actions.contains(&action)),
            "no component declares handoff recovery action {}",
            action.as_str()
        );
    }
}

#[test]
fn ac_disposition_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin one controlled vocabulary; assert the exact tokens.
    let tokens: Vec<&str> = M5ScaffoldDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "first_party",
            "team_managed",
            "community",
            "local_only",
            "create_empty",
            "continue_without_starter",
            "blocked",
            "warning",
            "optional",
        ]
    );
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5ScaffoldComponentFamily::ScaffoldPreflightCard);
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ScaffoldRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn dispositions_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::DispositionsMissing));
}

#[test]
fn scaffold_template_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldTemplateCard)
            .expect("scaffold-template-card row present");
        let expected = if clear == 0 {
            row.starter_source_classes.clear();
            M5ScaffoldComponentMatrixViolation::StarterSourceClassMissing
        } else {
            row.template_support_classes.clear();
            M5ScaffoldComponentMatrixViolation::TemplateSupportClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn starter_parameter_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::StarterParameterRow)
            .expect("starter-parameter-row row present");
        let expected = if clear == 0 {
            row.parameter_source_layers.clear();
            M5ScaffoldComponentMatrixViolation::ParameterSourceLayerMissing
        } else {
            row.parameter_action_timings.clear();
            M5ScaffoldComponentMatrixViolation::ParameterActionTimingMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn scaffold_preflight_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldPreflightCard)
            .expect("scaffold-preflight-card row present");
        let expected = if clear == 0 {
            row.preflight_check_classes.clear();
            M5ScaffoldComponentMatrixViolation::PreflightCheckClassMissing
        } else {
            row.preflight_result_states.clear();
            M5ScaffoldComponentMatrixViolation::PreflightResultStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn template_health_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::TemplateHealthRow)
            .expect("template-health-row row present");
        let expected = if clear == 0 {
            row.health_signal_classes.clear();
            M5ScaffoldComponentMatrixViolation::HealthSignalClassMissing
        } else {
            row.health_freshness_states.clear();
            M5ScaffoldComponentMatrixViolation::HealthFreshnessStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn generated_project_diff_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::GeneratedProjectDiffCard)
            .expect("generated-project-diff-card row present");
        let expected = if clear == 0 {
            row.generated_zone_classes.clear();
            M5ScaffoldComponentMatrixViolation::GeneratedZoneClassMissing
        } else {
            row.diff_review_states.clear();
            M5ScaffoldComponentMatrixViolation::DiffReviewStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn scaffold_handoff_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_scaffold_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldHandoffBanner)
            .expect("scaffold-handoff-banner row present");
        let expected = if clear == 0 {
            row.handoff_outcome_classes.clear();
            M5ScaffoldComponentMatrixViolation::HandoffOutcomeClassMissing
        } else {
            row.handoff_recovery_actions.clear();
            M5ScaffoldComponentMatrixViolation::HandoffRecoveryActionMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[0].hides_starter_source_or_support_class = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[2].hides_side_effect_behind_generic_create = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[4].hides_generated_versus_user_owned_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[5].omits_recovery_or_continue_without_starter_path = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[1].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ScaffoldComponentFamily::ScaffoldTemplateCard)
        .expect("scaffold-template-card row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet
        .governance_review
        .no_generic_create_hides_side_effects = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_scaffold_component_matrix().render_markdown_summary();
    for family in M5ScaffoldComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_scaffold_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ScaffoldComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,dispositions,"));
    for family in M5ScaffoldComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_scaffold_component_matrix_export()
        .expect("checked M5 scaffold component matrix export validates");
    assert_eq!(packet.packet_id, M5_SCAFFOLD_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_scaffold_component_matrix_export()
        .expect("checked M5 scaffold component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_scaffold_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_scaffold_component_matrix_scaffold_preflight_card_beta_narrowed(),
        seeded_m5_scaffold_component_matrix_scaffold_handoff_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5ScaffoldComponentFamily::ALL.len()
        );
    }

    let preflight = seeded_m5_scaffold_component_matrix_scaffold_preflight_card_beta_narrowed();
    let row = preflight
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ScaffoldComponentFamily::ScaffoldPreflightCard)
        .expect("scaffold-preflight-card row present");
    assert_eq!(row.qualification, M5ScaffoldQualificationClass::Beta);

    let handoff = seeded_m5_scaffold_component_matrix_scaffold_handoff_banner_preview_narrowed();
    let row = handoff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ScaffoldComponentFamily::ScaffoldHandoffBanner)
        .expect("scaffold-handoff-banner row present");
    assert_eq!(row.qualification, M5ScaffoldQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let preflight: M5ScaffoldComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-scaffold-components/scaffold_preflight_card_beta_narrowed.json"
    )))
    .expect("scaffold-preflight-card fixture parses");
    assert!(preflight.validate().is_empty());
    assert_eq!(
        preflight,
        seeded_m5_scaffold_component_matrix_scaffold_preflight_card_beta_narrowed()
    );

    let handoff: M5ScaffoldComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-scaffold-components/scaffold_handoff_banner_preview_narrowed.json"
    )))
    .expect("scaffold-handoff-banner fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_scaffold_component_matrix_scaffold_handoff_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_scaffold_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
