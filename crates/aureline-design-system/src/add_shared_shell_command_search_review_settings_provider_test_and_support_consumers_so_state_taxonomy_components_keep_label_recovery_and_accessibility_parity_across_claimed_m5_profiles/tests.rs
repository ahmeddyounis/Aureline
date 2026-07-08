use super::*;

fn full_input(
    consumer: M5StateComponentConsumer,
    family: M5SharedComponentStateFamily,
) -> M5StateComponentBindingInput {
    M5StateComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5StateComponentDescriptor::ALL.to_vec(),
        parity_health: M5StateConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_state_component_binding(&full_input(
        M5StateComponentConsumer::ShellChrome,
        M5SharedComponentStateFamily::InteractiveState,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_incomplete_or_degraded_state);
    assert!(resolved.asserts_exact_state_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5StateClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5SharedComponentStateFamily::InteractiveState)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5StateComponentBindingInput {
        parity_health: M5StateConsumerParityHealth::StateCauseUnresolvedNarrowed,
        export_caveats: vec![M5StateConsumerExportCaveat::StateCauseUnresolvedNotExplained],
        ..full_input(
            M5StateComponentConsumer::ReviewWorkItem,
            M5SharedComponentStateFamily::DegradedStateApplication,
        )
    };
    let resolved = resolve_state_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_exact_state_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5StateClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5StateConsumerNarrowingReason::StateCauseUnresolved
    );
    assert_eq!(
        banner.recovery_action,
        M5StateConsumerRecoveryAction::ResolveStateCauseBeforeTrusting
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5StateComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("not yet resolved"));
}

#[test]
fn resolver_incomplete_or_degraded_state_never_asserts_exact() {
    for (health, reason) in [
        (
            M5StateConsumerParityHealth::StateCauseUnresolvedNarrowed,
            M5StateConsumerNarrowingReason::StateCauseUnresolved,
        ),
        (
            M5StateConsumerParityHealth::RecoveryUnavailableNarrowed,
            M5StateConsumerNarrowingReason::RecoveryUnavailable,
        ),
    ] {
        let input = M5StateComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5StateComponentConsumer::ProviderOfflineCapture,
                M5SharedComponentStateFamily::DegradedStateApplication,
            )
        };
        let resolved = resolve_state_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_incomplete_or_degraded_state);
        assert!(!resolved.asserts_exact_state_parity);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_lock_owner_and_accessibility_narrowings_are_not_incomplete() {
    for health in [
        M5StateConsumerParityHealth::LockOwnerUnresolvedNarrowed,
        M5StateConsumerParityHealth::AccessibilityRouteReducedNarrowed,
    ] {
        let input = M5StateComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5StateComponentConsumer::SettingsCapability,
                M5SharedComponentStateFamily::SelectionOrLockState,
            )
        };
        let resolved = resolve_state_component_binding(&input).expect("resolves");
        assert!(resolved.is_narrowed);
        assert!(!resolved.asserts_exact_state_parity);
        // These narrowings are still explainable — not the incomplete/degraded boundary.
        assert!(!resolved.reflects_incomplete_or_degraded_state);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5StateConsumerParityHealth::StateCauseUnresolvedNarrowed,
            M5StateConsumerNarrowingReason::StateCauseUnresolved,
        ),
        (
            M5StateConsumerParityHealth::RecoveryUnavailableNarrowed,
            M5StateConsumerNarrowingReason::RecoveryUnavailable,
        ),
        (
            M5StateConsumerParityHealth::LockOwnerUnresolvedNarrowed,
            M5StateConsumerNarrowingReason::LockOwnerUnresolved,
        ),
        (
            M5StateConsumerParityHealth::AccessibilityRouteReducedNarrowed,
            M5StateConsumerNarrowingReason::AccessibilityRouteReduced,
        ),
    ] {
        let input = M5StateComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5StateComponentConsumer::SupportRecovery,
                M5SharedComponentStateFamily::SharedComponentStateTaxonomy,
            )
        };
        let resolved = resolve_state_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5StateComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5StateComponentConsumer::ShellChrome,
            M5SharedComponentStateFamily::InteractiveState,
        )
    };
    assert_eq!(
        resolve_state_component_binding(&empty),
        Err(M5StateComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5StateComponentBindingInput {
        descriptor_families: vec![M5StateComponentDescriptor::StateSemantics],
        ..full_input(
            M5StateComponentConsumer::ShellChrome,
            M5SharedComponentStateFamily::InteractiveState,
        )
    };
    assert_eq!(
        resolve_state_component_binding(&missing),
        Err(M5StateComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5StateComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5StateComponentConsumer::ShellChrome,
            M5SharedComponentStateFamily::InteractiveState,
        )
    };
    assert_eq!(
        resolve_state_component_binding(&forbidden),
        Err(M5StateComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::M5_SHARED_COMPONENT_STATE_SCHEMA_REF;
    use crate::implement_default_hover_focus_visible_pressed_state_contracts_with_no_color_only_and_no_layout_shift_rules_across_claimed_m5_controls_and_pane_affordances::M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF;
    use crate::implement_loading_pending_warning_error_and_degraded_state_blocks_with_submission_lineage_health_and_recovery_truth_across_claimed_m5_workflows::M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF;
    use crate::implement_selected_current_read_only_disabled_and_locked_state_parity_with_owner_reason_recovery_truth_across_claimed_m5_tabs_trees_lists_tables_badges_and_inspectors::M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF;
    use M5SharedComponentStateFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::SharedComponentStateTaxonomy),
        M5_SHARED_COMPONENT_STATE_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::InteractiveState),
        M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SelectionOrLockState),
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DegradedStateApplication),
        M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_state_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_STATE_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_state_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5StateComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5StateComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_state_component_consumer_packet();
    for family in M5SharedComponentStateFamily::ALL {
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
    let packet = seeded_m5_state_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5StateConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5StateConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5StateComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_state_component_consumer_packet();
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
    let packet = seeded_m5_state_component_consumer_packet();
    let cases: Vec<&M5StateComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5StateConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5StateConsumerNarrowingReason::ALL {
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
    for state in M5StateClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn incomplete_or_degraded_bindings_never_assert_exact() {
    let packet = seeded_m5_state_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_incomplete_or_degraded_state {
                    seen = true;
                    assert!(!case.resolved.asserts_exact_state_parity);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no incomplete / degraded binding present to prove the exact-parity honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_state_component_consumer_packet();
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
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5StateComponentConsumer::SearchDenseCollection);
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5StateComponentDescriptor::AccessibilityLabel);
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5StateConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    // Strip every InteractiveState binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5SharedComponentStateFamily::InteractiveState {
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
        .contains(&M5StateComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5StateComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5StateComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn exact_parity_honesty_unproven_fails_when_no_incomplete_example_present() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    // Replace every binding with a full-parity case: no incomplete / degraded state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5StateComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ExactParityHonestyUnproven));
}

#[test]
fn exact_parity_honesty_unproven_fails_when_incomplete_state_claims_exact() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    // Find an incomplete / degraded binding and force it to assert exact state parity.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_incomplete_or_degraded_state {
                    case.resolved.asserts_exact_state_parity = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ExactParityHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0].shows_partial_state_as_exact = true;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5StateComponentConsumer::SupportRecovery)
        .expect("support-recovery row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations.contains(&M5StateComponentConsumerViolation::SupportExportReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.governance_review.partial_state_never_shown_as_exact = false;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet
        .consumer_projection
        .accessibility_label_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5StateComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_state_component_consumer_packet().render_markdown_summary();
    for consumer in M5StateComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_state_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5StateComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5StateComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_state_component_consumer_export()
        .expect("checked M5 state component consumer export validates");
    assert_eq!(from_disk.packet_id, M5_STATE_COMPONENT_CONSUMER_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_state_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed(),
        seeded_m5_state_component_consumer_test_watch_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5StateComponentConsumer::ALL.len()
        );
    }

    let provider = seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed();
    let row = provider
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5StateComponentConsumer::ProviderOfflineCapture)
        .expect("provider-offline-capture row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let test = seeded_m5_state_component_consumer_test_watch_preview_narrowed();
    let row = test
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5StateComponentConsumer::TestWatch)
        .expect("test-watch row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let provider: M5StateComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shared-state-taxonomy-component-consumers/provider_offline_capture_beta_narrowed.json"
    )))
    .expect("provider fixture parses");
    assert!(provider.validate().is_empty());
    assert_eq!(
        provider,
        seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed()
    );

    let test: M5StateComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shared-state-taxonomy-component-consumers/test_watch_preview_narrowed.json"
    )))
    .expect("test-watch fixture parses");
    assert!(test.validate().is_empty());
    assert_eq!(
        test,
        seeded_m5_state_component_consumer_test_watch_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_state_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
