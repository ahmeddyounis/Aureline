use super::*;

fn full_input(
    consumer: M5ScaffoldComponentConsumer,
    family: M5ScaffoldComponentFamily,
) -> M5ScaffoldComponentBindingInput {
    M5ScaffoldComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5ScaffoldComponentDescriptor::ALL.to_vec(),
        parity_health: M5ScaffoldConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_scaffold_component_binding(&full_input(
        M5ScaffoldComponentConsumer::StartCenter,
        M5ScaffoldComponentFamily::ScaffoldTemplateCard,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_undisclosed_side_effect_risk);
    assert!(resolved.presents_ready_starter_without_caveat);
    assert_eq!(
        resolved.claim_parity_state,
        M5ScaffoldClaimParityState::ClaimsAligned
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5ScaffoldComponentFamily::ScaffoldTemplateCard)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5ScaffoldComponentBindingInput {
        parity_health: M5ScaffoldConsumerParityHealth::SideEffectPendingNarrowed,
        export_caveats: vec![M5ScaffoldConsumerExportCaveat::SideEffectDisclosedNotSilent],
        ..full_input(
            M5ScaffoldComponentConsumer::FrameworkPack,
            M5ScaffoldComponentFamily::ScaffoldPreflightCard,
        )
    };
    let resolved = resolve_scaffold_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.presents_ready_starter_without_caveat);
    assert_eq!(
        resolved.claim_parity_state,
        M5ScaffoldClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5ScaffoldConsumerNarrowingReason::SideEffectDisclosurePending
    );
    assert_eq!(
        banner.recovery_action,
        M5ScaffoldConsumerRecoveryAction::ReviewSideEffectsBeforeCreate
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5ScaffoldComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("side effect"));
}

#[test]
fn resolver_side_effect_starter_never_presents_ready_create() {
    let input = M5ScaffoldComponentBindingInput {
        parity_health: M5ScaffoldConsumerParityHealth::SideEffectPendingNarrowed,
        ..full_input(
            M5ScaffoldComponentConsumer::FrameworkPack,
            M5ScaffoldComponentFamily::ScaffoldPreflightCard,
        )
    };
    let resolved = resolve_scaffold_component_binding(&input).expect("resolves");
    assert!(resolved.reflects_undisclosed_side_effect_risk);
    assert!(!resolved.presents_ready_starter_without_caveat);
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5ScaffoldConsumerNarrowingReason::SideEffectDisclosurePending
    );
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5ScaffoldConsumerParityHealth::SourceOrSupportUnverifiedNarrowed,
            M5ScaffoldConsumerNarrowingReason::SourceOrSupportUnverified,
        ),
        (
            M5ScaffoldConsumerParityHealth::SideEffectPendingNarrowed,
            M5ScaffoldConsumerNarrowingReason::SideEffectDisclosurePending,
        ),
        (
            M5ScaffoldConsumerParityHealth::HealthStaleNarrowed,
            M5ScaffoldConsumerNarrowingReason::HealthFreshnessStale,
        ),
        (
            M5ScaffoldConsumerParityHealth::RecoveryRequiredNarrowed,
            M5ScaffoldConsumerNarrowingReason::RecoveryRequiredAfterPartialGeneration,
        ),
    ] {
        let input = M5ScaffoldComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5ScaffoldComponentConsumer::SafeHandoffExport,
                M5ScaffoldComponentFamily::ScaffoldHandoffBanner,
            )
        };
        let resolved = resolve_scaffold_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5ScaffoldComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5ScaffoldComponentConsumer::StartCenter,
            M5ScaffoldComponentFamily::ScaffoldTemplateCard,
        )
    };
    assert_eq!(
        resolve_scaffold_component_binding(&empty),
        Err(M5ScaffoldComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5ScaffoldComponentBindingInput {
        descriptor_families: vec![M5ScaffoldComponentDescriptor::SourceAndSupport],
        ..full_input(
            M5ScaffoldComponentConsumer::StartCenter,
            M5ScaffoldComponentFamily::ScaffoldTemplateCard,
        )
    };
    assert_eq!(
        resolve_scaffold_component_binding(&missing),
        Err(M5ScaffoldComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5ScaffoldComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5ScaffoldComponentConsumer::StartCenter,
            M5ScaffoldComponentFamily::ScaffoldTemplateCard,
        )
    };
    assert_eq!(
        resolve_scaffold_component_binding(&forbidden),
        Err(M5ScaffoldComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows::SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF;
    use crate::implement_scaffold_template_cards_and_starter_parameter_rows_with_source_support_host_boundary_and_portability_truth_across_claimed_m5_project_entry_surfaces::SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF;
    use crate::ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes::SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF;
    use M5ScaffoldComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::ScaffoldTemplateCard),
        SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::StarterParameterRow),
        SCAFFOLD_ENTRY_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ScaffoldPreflightCard),
        SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::TemplateHealthRow),
        SCAFFOLD_READINESS_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::GeneratedProjectDiffCard),
        SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ScaffoldHandoffBanner),
        SCAFFOLD_GENERATION_CONTROLS_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SCAFFOLD_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5ScaffoldComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5ScaffoldComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
    for family in M5ScaffoldComponentFamily::ALL {
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
    let packet = seeded_m5_scaffold_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5ScaffoldConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ScaffoldConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5ScaffoldComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ScaffoldAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
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
    let packet = seeded_m5_scaffold_component_consumer_packet();
    let cases: Vec<&M5ScaffoldComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5ScaffoldConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5ScaffoldConsumerNarrowingReason::ALL {
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
    for state in M5ScaffoldClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn side_effect_bindings_never_present_ready_create() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_undisclosed_side_effect_risk {
                    seen = true;
                    assert!(!case.resolved.presents_ready_starter_without_caveat);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no side-effect binding present to prove the side-effect honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_scaffold_component_consumer_packet();
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
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5ScaffoldComponentConsumer::TemplateRegistry);
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5ScaffoldComponentDescriptor::RecoveryAndOwnershipBoundary);
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5ScaffoldConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    // Binding [1] (the starter parameter row) renders at full parity; flipping it to narrowed drifts
    // from a fresh resolve.
    packet.consumer_rows[0].component_bindings[1].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    // Strip every ScaffoldTemplateCard binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5ScaffoldComponentFamily::ScaffoldTemplateCard {
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
        .contains(&M5ScaffoldComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ScaffoldComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5ScaffoldComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn side_effect_honesty_unproven_fails_when_no_side_effect_example_present() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    // Replace every binding with a full-parity case: no side-effect state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ScaffoldComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::SideEffectHonestyUnproven));
}

#[test]
fn side_effect_honesty_unproven_fails_when_side_effect_state_presents_ready_create() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    // Find a side-effect binding and force it to present a ready-to-create starter.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_undisclosed_side_effect_risk {
                    case.resolved.presents_ready_starter_without_caveat = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::SideEffectHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0].routes_side_effect_through_generic_create = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn blurs_generated_boundary_invariant_violation_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0].blurs_generated_versus_user_owned_boundary = true;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5ScaffoldComponentConsumer::SafeHandoffExport)
        .expect("safe handoff / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ScaffoldComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet
        .governance_review
        .side_effect_never_routes_through_generic_create = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet
        .consumer_projection
        .recovery_and_ownership_boundary_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_scaffold_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ScaffoldComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_scaffold_component_consumer_packet().render_markdown_summary();
    for consumer in M5ScaffoldComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_scaffold_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ScaffoldComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5ScaffoldComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_scaffold_component_consumer_export()
        .expect("checked M5 scaffold component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_SCAFFOLD_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_scaffold_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_scaffold_component_consumer_framework_pack_beta_narrowed(),
        seeded_m5_scaffold_component_consumer_workspace_admission_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5ScaffoldComponentConsumer::ALL.len()
        );
    }

    let framework = seeded_m5_scaffold_component_consumer_framework_pack_beta_narrowed();
    let row = framework
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ScaffoldComponentConsumer::FrameworkPack)
        .expect("framework-pack row present");
    assert_eq!(row.qualification, M5ScaffoldQualificationClass::Beta);

    let workspace = seeded_m5_scaffold_component_consumer_workspace_admission_preview_narrowed();
    let row = workspace
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ScaffoldComponentConsumer::WorkspaceAdmission)
        .expect("workspace-admission row present");
    assert_eq!(row.qualification, M5ScaffoldQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let framework: M5ScaffoldComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-scaffold-component-consumers/framework_pack_beta_narrowed.json"
    )))
    .expect("framework-pack fixture parses");
    assert!(framework.validate().is_empty());
    assert_eq!(
        framework,
        seeded_m5_scaffold_component_consumer_framework_pack_beta_narrowed()
    );

    let workspace: M5ScaffoldComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-scaffold-component-consumers/workspace_admission_preview_narrowed.json"
    )))
    .expect("workspace-admission fixture parses");
    assert!(workspace.validate().is_empty());
    assert_eq!(
        workspace,
        seeded_m5_scaffold_component_consumer_workspace_admission_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_scaffold_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
