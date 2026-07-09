use super::*;

fn full_input(
    consumer: M5LearningComponentConsumer,
    family: M5LearningComponentFamily,
) -> M5LearningComponentBindingInput {
    M5LearningComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5LearningComponentDescriptor::ALL.to_vec(),
        parity_health: M5LearningConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_learning_component_binding(&full_input(
        M5LearningComponentConsumer::Onboarding,
        M5LearningComponentFamily::LearningModeToggle,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_uncited_or_unavailable_source);
    assert!(resolved.asserts_live_cited_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5LearningClaimParityState::ClaimsAligned
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5LearningComponentFamily::LearningModeToggle)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5LearningComponentBindingInput {
        parity_health: M5LearningConsumerParityHealth::CitationUnavailableNarrowed,
        export_caveats: vec![M5LearningConsumerExportCaveat::CitedSourceUnavailableOrNotInstalled],
        ..full_input(
            M5LearningComponentConsumer::CompanionHandoff,
            M5LearningComponentFamily::SafeExplanationBanner,
        )
    };
    let resolved = resolve_learning_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_live_cited_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5LearningClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5LearningConsumerNarrowingReason::CitedSourceUnavailableOrNotInstalled
    );
    assert_eq!(
        banner.recovery_action,
        M5LearningConsumerRecoveryAction::OpenCitedSourceOrRequestAccess
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5LearningComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("unavailable"));
}

#[test]
fn resolver_uncited_or_unavailable_state_never_asserts_live_cited() {
    let input = M5LearningComponentBindingInput {
        parity_health: M5LearningConsumerParityHealth::CitationUnavailableNarrowed,
        ..full_input(
            M5LearningComponentConsumer::CompanionHandoff,
            M5LearningComponentFamily::SafeExplanationBanner,
        )
    };
    let resolved = resolve_learning_component_binding(&input).expect("resolves");
    assert!(resolved.reflects_uncited_or_unavailable_source);
    assert!(!resolved.asserts_live_cited_parity);
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5LearningConsumerNarrowingReason::CitedSourceUnavailableOrNotInstalled
    );
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5LearningConsumerParityHealth::CachedPackNarrowed,
            M5LearningConsumerNarrowingReason::CachedPackServed,
        ),
        (
            M5LearningConsumerParityHealth::StaleSourceNarrowed,
            M5LearningConsumerNarrowingReason::SourceContentStale,
        ),
        (
            M5LearningConsumerParityHealth::CitationUnavailableNarrowed,
            M5LearningConsumerNarrowingReason::CitedSourceUnavailableOrNotInstalled,
        ),
        (
            M5LearningConsumerParityHealth::ProgressLocalOnlyNarrowed,
            M5LearningConsumerNarrowingReason::ProgressLocalOnly,
        ),
    ] {
        let input = M5LearningComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5LearningComponentConsumer::SupportExport,
                M5LearningComponentFamily::TipCard,
            )
        };
        let resolved = resolve_learning_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5LearningComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5LearningComponentConsumer::Onboarding,
            M5LearningComponentFamily::LearningModeToggle,
        )
    };
    assert_eq!(
        resolve_learning_component_binding(&empty),
        Err(M5LearningComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5LearningComponentBindingInput {
        descriptor_families: vec![M5LearningComponentDescriptor::CitationSource],
        ..full_input(
            M5LearningComponentConsumer::Onboarding,
            M5LearningComponentFamily::LearningModeToggle,
        )
    };
    assert_eq!(
        resolve_learning_component_binding(&missing),
        Err(M5LearningComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5LearningComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5LearningComponentConsumer::Onboarding,
            M5LearningComponentFamily::LearningModeToggle,
        )
    };
    assert_eq!(
        resolve_learning_component_binding(&forbidden),
        Err(M5LearningComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_glossary_chips_or_cards_and_safe_explanation_banners_with_cited_file_symbol_doc_truth_freshness_source_class_labels_and_explain_versus_do_separation_across_claimed_m5_learning_surfaces::GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF;
    use crate::implement_guided_exercise_steps_and_progress_markers_with_target_object_success_criteria_hint_reveal_reset_skip_sandbox_or_preview_preference_and_privacy_bounded_resume_export_truth_across_claimed_m5_learnability_lanes::GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF;
    use crate::implement_learning_mode_toggles_and_tip_cards_with_user_workspace_scope_pause_snooze_reset_why_now_context_and_stable_command_file_docs_deep_link_truth_across_claimed_m5_onboarding_and_help_surfaces::LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF;
    use M5LearningComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::LearningModeToggle),
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::TipCard),
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::GuidedExerciseStep),
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ProgressMarker),
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::GlossaryChipOrCard),
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SafeExplanationBanner),
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_learning_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LEARNING_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_learning_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5LearningComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5LearningComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_learning_component_consumer_packet();
    for family in M5LearningComponentFamily::ALL {
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
    let packet = seeded_m5_learning_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5LearningConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5LearningConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5LearningComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5LearningAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_learning_component_consumer_packet();
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
    let packet = seeded_m5_learning_component_consumer_packet();
    let cases: Vec<&M5LearningComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5LearningConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5LearningConsumerNarrowingReason::ALL {
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
    for state in M5LearningClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn uncited_or_unavailable_bindings_never_assert_live_cited() {
    let packet = seeded_m5_learning_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_uncited_or_unavailable_source {
                    seen = true;
                    assert!(!case.resolved.asserts_live_cited_parity);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no uncited / unavailable binding present to prove the live-cited honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_learning_component_consumer_packet();
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
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5LearningComponentConsumer::ContextualHelp);
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5LearningComponentDescriptor::ExplainVersusDo);
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5LearningConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    // Strip every TipCard binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5LearningComponentFamily::TipCard {
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
        .contains(&M5LearningComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5LearningComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5LearningComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn live_cited_honesty_unproven_fails_when_no_uncited_example_present() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    // Replace every binding with a full-parity case: no uncited / unavailable state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5LearningComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::LiveCitedHonestyUnproven));
}

#[test]
fn live_cited_honesty_unproven_fails_when_uncited_state_claims_live_cited() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    // Find an uncited / unavailable binding and force it to assert live cited parity.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_uncited_or_unavailable_source {
                    case.resolved.asserts_live_cited_parity = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::LiveCitedHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].shows_uncited_or_unavailable_source_as_live_cited = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn widens_trust_invariant_violation_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].widens_trust_or_mutating_authority = true;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5LearningComponentConsumer::SupportExport)
        .expect("support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5LearningComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet
        .governance_review
        .uncited_or_unavailable_source_never_shown_as_live_cited = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet
        .consumer_projection
        .explain_versus_do_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LearningComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_learning_component_consumer_packet().render_markdown_summary();
    for consumer in M5LearningComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_learning_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5LearningComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5LearningComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_learning_component_consumer_export()
        .expect("checked M5 learning component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_LEARNING_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_learning_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_learning_component_consumer_docs_browser_beta_narrowed(),
        seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5LearningComponentConsumer::ALL.len()
        );
    }

    let docs = seeded_m5_learning_component_consumer_docs_browser_beta_narrowed();
    let row = docs
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5LearningComponentConsumer::DocsBrowser)
        .expect("docs-browser row present");
    assert_eq!(row.qualification, M5LearningQualificationClass::Beta);

    let handoff = seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed();
    let row = handoff
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5LearningComponentConsumer::CompanionHandoff)
        .expect("companion-handoff row present");
    assert_eq!(row.qualification, M5LearningQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let docs: M5LearningComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-learning-component-consumers/docs_browser_beta_narrowed.json"
    )))
    .expect("docs-browser fixture parses");
    assert!(docs.validate().is_empty());
    assert_eq!(
        docs,
        seeded_m5_learning_component_consumer_docs_browser_beta_narrowed()
    );

    let handoff: M5LearningComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-learning-component-consumers/companion_handoff_preview_narrowed.json"
    )))
    .expect("companion-handoff fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_learning_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
