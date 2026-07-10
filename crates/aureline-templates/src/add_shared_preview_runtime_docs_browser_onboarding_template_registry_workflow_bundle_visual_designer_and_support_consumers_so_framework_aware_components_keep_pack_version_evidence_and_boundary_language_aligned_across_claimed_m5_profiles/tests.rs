use super::*;

fn full_input(
    consumer: M5FrameworkComponentConsumer,
    family: M5FrameworkComponentFamily,
) -> M5FrameworkComponentBindingInput {
    M5FrameworkComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5FrameworkComponentDescriptor::ALL.to_vec(),
        parity_health: M5FrameworkConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_framework_component_binding(&full_input(
        M5FrameworkComponentConsumer::PreviewRuntime,
        M5FrameworkComponentFamily::FrameworkPackHeader,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_write_or_boundary_risk);
    assert!(resolved.presents_safe_action_without_caveat);
    assert_eq!(
        resolved.claim_parity_state,
        M5FrameworkClaimParityState::ClaimsAligned
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5FrameworkComponentFamily::FrameworkPackHeader)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5FrameworkComponentBindingInput {
        parity_health: M5FrameworkConsumerParityHealth::ExecutionBoundaryOrWriteEffectNarrowed,
        export_caveats: vec![
            M5FrameworkConsumerExportCaveat::ExecutionBoundaryOrWriteEffectDisclosedNotSilent,
        ],
        ..full_input(
            M5FrameworkComponentConsumer::Onboarding,
            M5FrameworkComponentFamily::GeneratorPreviewSheet,
        )
    };
    let resolved = resolve_framework_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.presents_safe_action_without_caveat);
    assert_eq!(
        resolved.claim_parity_state,
        M5FrameworkClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5FrameworkConsumerNarrowingReason::ExecutionBoundaryOrWriteEffectPending
    );
    assert_eq!(
        banner.recovery_action,
        M5FrameworkConsumerRecoveryAction::ReviewExecutionBoundaryAndImpactBeforeDispatch
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5FrameworkComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("write effect"));
}

#[test]
fn resolver_write_or_boundary_binding_never_presents_safe_action() {
    let input = M5FrameworkComponentBindingInput {
        parity_health: M5FrameworkConsumerParityHealth::ExecutionBoundaryOrWriteEffectNarrowed,
        ..full_input(
            M5FrameworkComponentConsumer::PreviewRuntime,
            M5FrameworkComponentFamily::RunConfigScaffoldCard,
        )
    };
    let resolved = resolve_framework_component_binding(&input).expect("resolves");
    assert!(resolved.reflects_write_or_boundary_risk);
    assert!(!resolved.presents_safe_action_without_caveat);
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5FrameworkConsumerNarrowingReason::ExecutionBoundaryOrWriteEffectPending
    );
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5FrameworkConsumerParityHealth::PackOrSupportUnverifiedNarrowed,
            M5FrameworkConsumerNarrowingReason::PackOrSupportUnverified,
        ),
        (
            M5FrameworkConsumerParityHealth::HeuristicEvidenceNarrowed,
            M5FrameworkConsumerNarrowingReason::HeuristicEvidenceNotExact,
        ),
        (
            M5FrameworkConsumerParityHealth::ExecutionBoundaryOrWriteEffectNarrowed,
            M5FrameworkConsumerNarrowingReason::ExecutionBoundaryOrWriteEffectPending,
        ),
        (
            M5FrameworkConsumerParityHealth::RecoveryRequiredNarrowed,
            M5FrameworkConsumerNarrowingReason::RecoveryRequiredAfterGeneratorWrite,
        ),
    ] {
        let input = M5FrameworkComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5FrameworkComponentConsumer::SupportExport,
                M5FrameworkComponentFamily::DerivedRelationshipBanner,
            )
        };
        let resolved = resolve_framework_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5FrameworkComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5FrameworkComponentConsumer::PreviewRuntime,
            M5FrameworkComponentFamily::FrameworkPackHeader,
        )
    };
    assert_eq!(
        resolve_framework_component_binding(&empty),
        Err(M5FrameworkComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5FrameworkComponentBindingInput {
        descriptor_families: vec![M5FrameworkComponentDescriptor::PackIdentityAndSupport],
        ..full_input(
            M5FrameworkComponentConsumer::PreviewRuntime,
            M5FrameworkComponentFamily::FrameworkPackHeader,
        )
    };
    assert_eq!(
        resolve_framework_component_binding(&missing),
        Err(M5FrameworkComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5FrameworkComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5FrameworkComponentConsumer::PreviewRuntime,
            M5FrameworkComponentFamily::FrameworkPackHeader,
        )
    };
    assert_eq!(
        resolve_framework_component_binding(&forbidden),
        Err(M5FrameworkComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_convention_diagnostic_rows_and_derived_relationship_banners_with_diagnostic_class_affected_entity_or_file_certainty_detected_source_suggested_fix_or_open_docs_actions_support_class_caveats_and_open_raw_source_or_wider_graph_continuity::CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF;
    use crate::implement_framework_pack_headers_and_framework_status_strips_with_pack_identity_version_support_range_provider_source_freshness_compatibility_and_local_versus_remote_scope_truth::FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF;
    use crate::implement_generator_preview_sheets_and_run_config_scaffold_cards_with_generator_version_file_effect_classes_dependency_config_impact_rollback_or_regenerate_posture_required_toolchains_and_local_container_ssh_managed_target_truth::GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF;
    use crate::implement_route_endpoint_rows_and_component_service_tree_nodes_with_authored_versus_generated_state_proving_source_files_or_symbols_exact_versus_heuristic_labels_and_open_source_or_open_references_continuity::ROUTE_TREE_CONTROLS_SCHEMA_REF;
    use M5FrameworkComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::FrameworkPackHeader),
        FRAMEWORK_PACK_HEADER_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RouteEndpointRow),
        ROUTE_TREE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ComponentServiceTreeNode),
        ROUTE_TREE_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ConventionDiagnosticRow),
        CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DerivedRelationshipBanner),
        CONVENTION_RELATIONSHIP_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::GeneratorPreviewSheet),
        GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RunConfigScaffoldCard),
        GENERATOR_RUN_CONFIG_CONTROLS_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_framework_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_FRAMEWORK_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_framework_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5FrameworkComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5FrameworkComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_framework_component_consumer_packet();
    for family in M5FrameworkComponentFamily::ALL {
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
    let packet = seeded_m5_framework_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5FrameworkConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5FrameworkConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5FrameworkComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5FrameworkAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_framework_component_consumer_packet();
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
    let packet = seeded_m5_framework_component_consumer_packet();
    let cases: Vec<&M5FrameworkComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5FrameworkConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5FrameworkConsumerNarrowingReason::ALL {
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
    for state in M5FrameworkClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn write_or_boundary_bindings_never_present_safe_action() {
    let packet = seeded_m5_framework_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_write_or_boundary_risk {
                    seen = true;
                    assert!(!case.resolved.presents_safe_action_without_caveat);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no write-or-boundary binding present to prove the write / boundary honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_framework_component_consumer_packet();
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
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5FrameworkComponentConsumer::TemplateRegistry);
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5FrameworkComponentDescriptor::RecoveryAndRollbackBoundary);
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5FrameworkConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    // Binding [1] (the run-config scaffold card) renders narrowed; flipping it to full parity drifts
    // from a fresh resolve.
    packet.consumer_rows[0].component_bindings[1].example_bindings[0]
        .resolved
        .is_narrowed = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    // Strip every FrameworkPackHeader binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5FrameworkComponentFamily::FrameworkPackHeader {
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
        .contains(&M5FrameworkComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5FrameworkComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5FrameworkComponentConsumerViolation::NarrowingDisclosureUnproven)
    );
}

#[test]
fn write_boundary_honesty_unproven_fails_when_no_write_or_boundary_example_present() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    // Replace every binding with a full-parity case: no write / boundary state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5FrameworkComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::WriteBoundaryHonestyUnproven));
}

#[test]
fn write_boundary_honesty_unproven_fails_when_write_state_presents_safe_action() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    // Find a write / boundary binding and force it to present a safe action.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_write_or_boundary_risk {
                    case.resolved.presents_safe_action_without_caveat = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::WriteBoundaryHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0].implies_no_op_write_or_hides_execution_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn heuristic_masquerade_invariant_violation_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0].lets_heuristic_masquerade_as_exact = true;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5FrameworkComponentConsumer::SupportExport)
        .expect("safe support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5FrameworkComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet
        .governance_review
        .generator_never_implies_no_op_write_or_hides_boundary = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet
        .consumer_projection
        .recovery_and_rollback_boundary_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_framework_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5FrameworkComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_framework_component_consumer_packet().render_markdown_summary();
    for consumer in M5FrameworkComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_framework_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5FrameworkComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5FrameworkComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_framework_component_consumer_export()
        .expect("checked M5 framework component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_FRAMEWORK_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_framework_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_framework_component_consumer_preview_runtime_beta_narrowed(),
        seeded_m5_framework_component_consumer_onboarding_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5FrameworkComponentConsumer::ALL.len()
        );
    }

    let preview = seeded_m5_framework_component_consumer_preview_runtime_beta_narrowed();
    let row = preview
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5FrameworkComponentConsumer::PreviewRuntime)
        .expect("preview-runtime row present");
    assert_eq!(row.qualification, M5FrameworkQualificationClass::Beta);

    let onboarding = seeded_m5_framework_component_consumer_onboarding_preview_narrowed();
    let row = onboarding
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5FrameworkComponentConsumer::Onboarding)
        .expect("onboarding row present");
    assert_eq!(row.qualification, M5FrameworkQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let preview: M5FrameworkComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-framework-component-consumers/preview_runtime_beta_narrowed.json"
    )))
    .expect("preview-runtime fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_framework_component_consumer_preview_runtime_beta_narrowed()
    );

    let onboarding: M5FrameworkComponentConsumerPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-framework-component-consumers/onboarding_preview_narrowed.json"
        )))
        .expect("onboarding fixture parses");
    assert!(onboarding.validate().is_empty());
    assert_eq!(
        onboarding,
        seeded_m5_framework_component_consumer_onboarding_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_framework_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
