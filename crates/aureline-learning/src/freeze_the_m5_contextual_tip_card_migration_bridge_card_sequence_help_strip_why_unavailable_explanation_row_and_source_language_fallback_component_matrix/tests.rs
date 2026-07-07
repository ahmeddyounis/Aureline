use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_contextual_teaching_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_contextual_teaching_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5ContextualTeachingComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5ContextualTeachingComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_contextual_teaching_component_matrix();
    for row in &packet.component_rows {
        for label in M5TeachingRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_contextual_teaching_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.tip_trigger_classes.is_empty(),
            family.is_contextual_tip_card(),
            "tip_trigger_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.tip_dismissal_states.is_empty(),
            family.is_contextual_tip_card(),
            "tip_dismissal_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.migration_mapping_classes.is_empty(),
            family.is_migration_bridge_card(),
            "migration_mapping_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.source_tool_classes.is_empty(),
            family.is_migration_bridge_card(),
            "source_tool_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sequence_help_states.is_empty(),
            family.is_sequence_help_strip(),
            "sequence_help_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sequence_step_kinds.is_empty(),
            family.is_sequence_help_strip(),
            "sequence_step_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.command_backing_states.is_empty(),
            family.is_contextual_tip_card() || family.is_sequence_help_strip(),
            "command_backing_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.blocked_action_owners.is_empty(),
            family.is_why_unavailable_explanation_row(),
            "blocked_action_owners presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.unavailable_reason_classes.is_empty(),
            family.is_why_unavailable_explanation_row(),
            "unavailable_reason_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.next_safe_action_classes.is_empty(),
            family.is_why_unavailable_explanation_row(),
            "next_safe_action_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.source_language_classes.is_empty(),
            family.is_source_language_fallback(),
            "source_language_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.fallback_state_classes.is_empty(),
            family.is_source_language_fallback(),
            "fallback_state_classes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_contextual_teaching_component_matrix();
    for trigger in M5TipTriggerClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tip_trigger_classes.contains(&trigger)),
            "no component declares tip trigger class {}",
            trigger.as_str()
        );
    }
    for state in M5TipDismissalState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.tip_dismissal_states.contains(&state)),
            "no component declares tip dismissal state {}",
            state.as_str()
        );
    }
    for class in M5MigrationMappingClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.migration_mapping_classes.contains(&class)),
            "no component declares migration mapping class {}",
            class.as_str()
        );
    }
    for tool in M5SourceToolClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.source_tool_classes.contains(&tool)),
            "no component declares source tool class {}",
            tool.as_str()
        );
    }
    for state in M5SequenceHelpState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.sequence_help_states.contains(&state)),
            "no component declares sequence help state {}",
            state.as_str()
        );
    }
    for kind in M5SequenceStepKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.sequence_step_kinds.contains(&kind)),
            "no component declares sequence step kind {}",
            kind.as_str()
        );
    }
    for state in M5CommandBackingState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.command_backing_states.contains(&state)),
            "no component declares command backing state {}",
            state.as_str()
        );
    }
    for owner in M5BlockedActionOwner::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.blocked_action_owners.contains(&owner)),
            "no component declares blocked action owner {}",
            owner.as_str()
        );
    }
    for reason in M5UnavailableReasonClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.unavailable_reason_classes.contains(&reason)),
            "no component declares unavailable reason {}",
            reason.as_str()
        );
    }
    for action in M5NextSafeActionClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.next_safe_action_classes.contains(&action)),
            "no component declares next safe action {}",
            action.as_str()
        );
    }
    for class in M5SourceLanguageClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.source_language_classes.contains(&class)),
            "no component declares source language class {}",
            class.as_str()
        );
    }
    for state in M5FallbackStateClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.fallback_state_classes.contains(&state)),
            "no component declares fallback state {}",
            state.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5ContextualTeachingComponentFamily::SequenceHelpStrip
    });
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.vocabulary_set.migration_mapping_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5TeachingRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn tip_card_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_contextual_teaching_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ContextualTeachingComponentFamily::ContextualTipCard
            })
            .expect("contextual-tip-card row present");
        let expected = match clear {
            0 => {
                row.tip_trigger_classes.clear();
                M5ContextualTeachingComponentMatrixViolation::TipTriggerClassMissing
            }
            1 => {
                row.tip_dismissal_states.clear();
                M5ContextualTeachingComponentMatrixViolation::TipDismissalStateMissing
            }
            _ => {
                row.command_backing_states.clear();
                M5ContextualTeachingComponentMatrixViolation::CommandBackingStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn migration_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_contextual_teaching_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ContextualTeachingComponentFamily::MigrationBridgeCard
            })
            .expect("migration-bridge-card row present");
        let expected = if clear == 0 {
            row.migration_mapping_classes.clear();
            M5ContextualTeachingComponentMatrixViolation::MigrationMappingClassMissing
        } else {
            row.source_tool_classes.clear();
            M5ContextualTeachingComponentMatrixViolation::SourceToolClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn sequence_help_strip_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_contextual_teaching_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ContextualTeachingComponentFamily::SequenceHelpStrip
            })
            .expect("sequence-help-strip row present");
        let expected = match clear {
            0 => {
                row.sequence_help_states.clear();
                M5ContextualTeachingComponentMatrixViolation::SequenceHelpStateMissing
            }
            1 => {
                row.sequence_step_kinds.clear();
                M5ContextualTeachingComponentMatrixViolation::SequenceStepKindMissing
            }
            _ => {
                row.command_backing_states.clear();
                M5ContextualTeachingComponentMatrixViolation::CommandBackingStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn why_unavailable_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_contextual_teaching_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5ContextualTeachingComponentFamily::WhyUnavailableExplanationRow
            })
            .expect("why-unavailable row present");
        let expected = match clear {
            0 => {
                row.blocked_action_owners.clear();
                M5ContextualTeachingComponentMatrixViolation::BlockedActionOwnerMissing
            }
            1 => {
                row.unavailable_reason_classes.clear();
                M5ContextualTeachingComponentMatrixViolation::UnavailableReasonClassMissing
            }
            _ => {
                row.next_safe_action_classes.clear();
                M5ContextualTeachingComponentMatrixViolation::NextSafeActionClassMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn source_language_fallback_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_contextual_teaching_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ContextualTeachingComponentFamily::SourceLanguageFallback
            })
            .expect("source-language-fallback row present");
        let expected = if clear == 0 {
            row.source_language_classes.clear();
            M5ContextualTeachingComponentMatrixViolation::SourceLanguageClassMissing
        } else {
            row.fallback_state_classes.clear();
            M5ContextualTeachingComponentMatrixViolation::FallbackStateClassMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[0].masks_command_binding_or_migration_mapping = true;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[3].hides_blocked_action_owner_or_reason = true;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[4].severs_source_language_citation = true;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ContextualTeachingComponentFamily::ContextualTipCard)
        .expect("contextual-tip-card row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_contextual_teaching_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTeachingComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_contextual_teaching_component_matrix().render_markdown_summary();
    for family in M5ContextualTeachingComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_contextual_teaching_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5ContextualTeachingComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5ContextualTeachingComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_contextual_teaching_component_matrix_export()
        .expect("checked M5 contextual-teaching component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_contextual_teaching_component_matrix_export()
        .expect("checked M5 contextual-teaching component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_contextual_teaching_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed(),
        seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5ContextualTeachingComponentFamily::ALL.len()
        );
    }

    let migration =
        seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed();
    let row = migration
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ContextualTeachingComponentFamily::MigrationBridgeCard)
        .expect("migration-bridge-card row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let fallback =
        seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed();
    let row = fallback
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ContextualTeachingComponentFamily::SourceLanguageFallback)
        .expect("source-language-fallback row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let migration: M5ContextualTeachingComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-contextual-teaching-components/migration_bridge_card_beta_narrowed.json"
        )))
        .expect("migration-bridge-card fixture parses");
    assert!(migration.validate().is_empty());
    assert_eq!(
        migration,
        seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed()
    );

    let fallback: M5ContextualTeachingComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-contextual-teaching-components/source_language_fallback_preview_narrowed.json"
        )))
        .expect("source-language-fallback fixture parses");
    assert!(fallback.validate().is_empty());
    assert_eq!(
        fallback,
        seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_contextual_teaching_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
