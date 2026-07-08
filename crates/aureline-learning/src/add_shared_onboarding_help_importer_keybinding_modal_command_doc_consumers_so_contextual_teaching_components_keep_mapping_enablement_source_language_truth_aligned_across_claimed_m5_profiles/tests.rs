use super::*;

fn full_input(
    consumer: M5TeachingComponentConsumer,
    family: M5ContextualTeachingComponentFamily,
) -> M5TeachingComponentBindingInput {
    M5TeachingComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5TeachingComponentDescriptor::ALL.to_vec(),
        parity_health: M5TeachingConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_teaching_component_binding(&full_input(
        M5TeachingComponentConsumer::OnboardingFlow,
        M5ContextualTeachingComponentFamily::ContextualTipCard,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_partial_or_unsupported_state);
    assert!(resolved.asserts_exact_teaching_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5TeachingClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5ContextualTeachingComponentFamily::ContextualTipCard)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5TeachingComponentBindingInput {
        parity_health: M5TeachingConsumerParityHealth::ImportedBehaviorPartialNarrowed,
        export_caveats: vec![M5TeachingConsumerExportCaveat::ImportedBehaviorPartialNotExact],
        ..full_input(
            M5TeachingComponentConsumer::MigrationImporter,
            M5ContextualTeachingComponentFamily::MigrationBridgeCard,
        )
    };
    let resolved = resolve_teaching_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_exact_teaching_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5TeachingClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5TeachingConsumerNarrowingReason::ImportedBehaviorPartial
    );
    assert_eq!(
        banner.recovery_action,
        M5TeachingConsumerRecoveryAction::ReviewMigrationMappingBeforeTrusting
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5TeachingComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("partially mapped"));
}

#[test]
fn resolver_partial_or_unsupported_state_never_asserts_exact() {
    for (health, reason) in [
        (
            M5TeachingConsumerParityHealth::ImportedBehaviorPartialNarrowed,
            M5TeachingConsumerNarrowingReason::ImportedBehaviorPartial,
        ),
        (
            M5TeachingConsumerParityHealth::SequenceUnsupportedNarrowed,
            M5TeachingConsumerNarrowingReason::SequenceUnsupported,
        ),
    ] {
        let input = M5TeachingComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5TeachingComponentConsumer::KeybindingLeaderHelp,
                M5ContextualTeachingComponentFamily::SequenceHelpStrip,
            )
        };
        let resolved = resolve_teaching_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_partial_or_unsupported_state);
        assert!(!resolved.asserts_exact_teaching_parity);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5TeachingConsumerParityHealth::ImportedBehaviorPartialNarrowed,
            M5TeachingConsumerNarrowingReason::ImportedBehaviorPartial,
        ),
        (
            M5TeachingConsumerParityHealth::SequenceUnsupportedNarrowed,
            M5TeachingConsumerNarrowingReason::SequenceUnsupported,
        ),
        (
            M5TeachingConsumerParityHealth::BlockedOwnerChangedNarrowed,
            M5TeachingConsumerNarrowingReason::BlockedActionOwnerChanged,
        ),
        (
            M5TeachingConsumerParityHealth::LocalizedFallbackStaleNarrowed,
            M5TeachingConsumerNarrowingReason::LocalizedFallbackStaleOrPolicyLimited,
        ),
    ] {
        let input = M5TeachingComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5TeachingComponentConsumer::LocalizedSupportPacket,
                M5ContextualTeachingComponentFamily::ContextualTipCard,
            )
        };
        let resolved = resolve_teaching_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5TeachingComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5TeachingComponentConsumer::OnboardingFlow,
            M5ContextualTeachingComponentFamily::ContextualTipCard,
        )
    };
    assert_eq!(
        resolve_teaching_component_binding(&empty),
        Err(M5TeachingComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5TeachingComponentBindingInput {
        descriptor_families: vec![M5TeachingComponentDescriptor::CommandBinding],
        ..full_input(
            M5TeachingComponentConsumer::OnboardingFlow,
            M5ContextualTeachingComponentFamily::ContextualTipCard,
        )
    };
    assert_eq!(
        resolve_teaching_component_binding(&missing),
        Err(M5TeachingComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5TeachingComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5TeachingComponentConsumer::OnboardingFlow,
            M5ContextualTeachingComponentFamily::ContextualTipCard,
        )
    };
    assert_eq!(
        resolve_teaching_component_binding(&forbidden),
        Err(M5TeachingComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_contextual_tip_cards_with_why_now_relevance_concrete_next_action_stable_command_reference_and_try_open_docs_snooze_dismiss_actions_that_respect_quiet_hours_presentation_mode_and_recent_dismissals_across_claimed_m5_learnability_surfaces::M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF;
    use crate::implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces::M5_SEQUENCE_HELP_STRIP_SCHEMA_REF;
    use crate::implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces::M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF;
    use crate::ship_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces::M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF;
    use M5ContextualTeachingComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::ContextualTipCard),
        M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::MigrationBridgeCard),
        M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SequenceHelpStrip),
        M5_SEQUENCE_HELP_STRIP_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::WhyUnavailableExplanationRow),
        M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SourceLanguageFallback),
        M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TEACHING_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5TeachingComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5TeachingComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    for family in M5ContextualTeachingComponentFamily::ALL {
        let count = packet
            .consumer_rows
            .iter()
            .filter(|row| {
                row.component_bindings
                    .iter()
                    .any(|b| b.component_family == family)
            })
            .count();
        assert!(
            count >= 2,
            "family {} adopted by only {} consumer(s)",
            family.as_str(),
            count
        );
    }
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_descriptors() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5TeachingConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5TeachingConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5TeachingComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            assert_eq!(
                b.canonical_schema_ref,
                family_canonical_schema_ref(b.component_family)
            );
            assert_eq!(
                b.canonical_artifact_ref,
                family_canonical_artifact_ref(b.component_family)
            );
            assert!(b.references_canonical_not_local_prose);
        }
    }
}

#[test]
fn every_parity_health_mode_reason_and_parity_state_is_exercised() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    let cases: Vec<&M5TeachingComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5TeachingConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5TeachingConsumerNarrowingReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked binding exercises narrowing reason {}",
            reason.as_str()
        );
    }
    for state in M5TeachingClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn partial_or_unsupported_bindings_never_assert_exact() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    let mut seen_partial = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_partial_or_unsupported_state {
                    seen_partial = true;
                    assert!(!case.resolved.asserts_exact_teaching_parity);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen_partial,
        "no partial / unsupported binding present to prove the exact-parity honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_teaching_component_consumer_packet();
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                assert!(
                    case.is_self_consistent(),
                    "worked binding for {} drifted from resolver output",
                    row.consumer.as_str()
                );
            }
        }
    }
}

#[test]
fn missing_consumer_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5TeachingComponentConsumer::CommandDocs);
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5TeachingComponentDescriptor::SourceLanguageCitation);
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5TeachingConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    // Strip every ContextualTipCard binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5ContextualTeachingComponentFamily::ContextualTipCard {
                if seen_first {
                    return false;
                }
                seen_first = true;
            }
            true
        });
    }
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5TeachingComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5TeachingComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn exact_parity_honesty_unproven_fails_when_no_partial_example_present() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    // Replace every binding with a full-parity case: no partial / unsupported state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5TeachingComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ExactParityHonestyUnproven));
}

#[test]
fn exact_parity_honesty_unproven_fails_when_partial_state_claims_exact() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    // Find a partial / unsupported binding and force it to assert exact teaching parity.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_partial_or_unsupported_state {
                    case.resolved.asserts_exact_teaching_parity = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ExactParityHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0].shows_partial_or_unsupported_state_as_exact = true;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5TeachingComponentConsumer::LocalizedSupportPacket)
        .expect("localized support-packet row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5TeachingComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet
        .governance_review
        .partial_or_unsupported_state_never_shown_as_exact = false;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet
        .consumer_projection
        .source_language_citation_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TeachingComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_teaching_component_consumer_packet().render_markdown_summary();
    for consumer in M5TeachingComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_teaching_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5TeachingComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5TeachingComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_teaching_component_consumer_export()
        .expect("checked M5 teaching component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_TEACHING_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_teaching_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed(),
        seeded_m5_teaching_component_consumer_help_pane_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5TeachingComponentConsumer::ALL.len()
        );
    }

    let importer = seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed();
    let row = importer
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5TeachingComponentConsumer::MigrationImporter)
        .expect("migration-importer row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let help = seeded_m5_teaching_component_consumer_help_pane_preview_narrowed();
    let row = help
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5TeachingComponentConsumer::HelpPane)
        .expect("help-pane row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let importer: M5TeachingComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-contextual-teaching-component-consumers/migration_importer_beta_narrowed.json"
    )))
    .expect("migration-importer fixture parses");
    assert!(importer.validate().is_empty());
    assert_eq!(
        importer,
        seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed()
    );

    let help: M5TeachingComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-contextual-teaching-component-consumers/help_pane_preview_narrowed.json"
    )))
    .expect("help-pane fixture parses");
    assert!(help.validate().is_empty());
    assert_eq!(
        help,
        seeded_m5_teaching_component_consumer_help_pane_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_teaching_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
