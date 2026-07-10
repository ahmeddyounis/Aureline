use super::*;

fn full_input(
    consumer: M5ExperimentComponentConsumer,
    family: M5ExperimentComponentFamily,
) -> M5ExperimentComponentBindingInput {
    M5ExperimentComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5ExperimentComponentDescriptor::ALL.to_vec(),
        parity_health: M5ExperimentConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_experiment_component_binding(&full_input(
        M5ExperimentComponentConsumer::NotebookRunHistory,
        M5ExperimentComponentFamily::ExperimentRunRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_unproven_comparability);
    assert!(resolved.asserts_apples_to_apples_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5ExperimentClaimParityState::ClaimsAligned
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5ExperimentComponentFamily::ExperimentRunRow)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5ExperimentComponentBindingInput {
        parity_health: M5ExperimentConsumerParityHealth::NotComparableNarrowed,
        export_caveats: vec![M5ExperimentConsumerExportCaveat::ComparisonNotApplesToApples],
        ..full_input(
            M5ExperimentComponentConsumer::CompareView,
            M5ExperimentComponentFamily::RunComparisonTable,
        )
    };
    let resolved = resolve_experiment_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_apples_to_apples_parity);
    assert_eq!(
        resolved.claim_parity_state,
        M5ExperimentClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5ExperimentConsumerNarrowingReason::ComparabilityUnproven
    );
    assert_eq!(
        banner.recovery_action,
        M5ExperimentConsumerRecoveryAction::ReviewComparabilityBeforeTrustingDelta
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5ExperimentComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("apples-to-apples"));
}

#[test]
fn resolver_unproven_comparability_never_asserts_apples_to_apples() {
    let input = M5ExperimentComponentBindingInput {
        parity_health: M5ExperimentConsumerParityHealth::NotComparableNarrowed,
        ..full_input(
            M5ExperimentComponentConsumer::CompareView,
            M5ExperimentComponentFamily::RunComparisonTable,
        )
    };
    let resolved = resolve_experiment_component_binding(&input).expect("resolves");
    assert!(resolved.reflects_unproven_comparability);
    assert!(!resolved.asserts_apples_to_apples_parity);
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.auto_narrow_banner.expect("banner").reason,
        M5ExperimentConsumerNarrowingReason::ComparabilityUnproven
    );
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5ExperimentConsumerParityHealth::ProvenanceIncompleteNarrowed,
            M5ExperimentConsumerNarrowingReason::LineageProvenanceIncomplete,
        ),
        (
            M5ExperimentConsumerParityHealth::NotComparableNarrowed,
            M5ExperimentConsumerNarrowingReason::ComparabilityUnproven,
        ),
        (
            M5ExperimentConsumerParityHealth::SensitivityRestrictedNarrowed,
            M5ExperimentConsumerNarrowingReason::SensitiveDataRestricted,
        ),
        (
            M5ExperimentConsumerParityHealth::MetadataOnlyExportNarrowed,
            M5ExperimentConsumerNarrowingReason::ExportMetadataOnly,
        ),
    ] {
        let input = M5ExperimentComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5ExperimentComponentConsumer::SupportExport,
                M5ExperimentComponentFamily::ResultSummaryCard,
            )
        };
        let resolved = resolve_experiment_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5ExperimentComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5ExperimentComponentConsumer::NotebookRunHistory,
            M5ExperimentComponentFamily::ExperimentRunRow,
        )
    };
    assert_eq!(
        resolve_experiment_component_binding(&empty),
        Err(M5ExperimentComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5ExperimentComponentBindingInput {
        descriptor_families: vec![M5ExperimentComponentDescriptor::LineageProvenance],
        ..full_input(
            M5ExperimentComponentConsumer::NotebookRunHistory,
            M5ExperimentComponentFamily::ExperimentRunRow,
        )
    };
    assert_eq!(
        resolve_experiment_component_binding(&missing),
        Err(M5ExperimentComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5ExperimentComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5ExperimentComponentConsumer::NotebookRunHistory,
            M5ExperimentComponentFamily::ExperimentRunRow,
        )
    };
    assert_eq!(
        resolve_experiment_component_binding(&forbidden),
        Err(M5ExperimentComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_artifact_lineage_panels_and_result_summary_cards_with_producing_run_identity_stale_diverged_notes_include_raw_toggles_and_export_boundary_truth_across_claimed_m5_experiment_surfaces::ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF;
    use crate::implement_dataset_provenance_cards_and_sensitivity_sharing_banners_with_snapshot_sample_redaction_and_local_remote_location_truth_across_claimed_m5_data_lanes::DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF;
    use crate::implement_experiment_run_rows_and_environment_fingerprint_cards_with_run_origin_code_revision_execution_target_and_outcome_truth_across_claimed_m5_notebook_and_data_surfaces::EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF;
    use crate::implement_run_comparison_tables_and_compare_guard_banners_with_baseline_candidate_identity_confounder_disclosure_and_no_fair_delta_claims_when_parity_evidence_is_incomplete_across_claimed_m5_compare_flows::RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF;
    use M5ExperimentComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::ExperimentRunRow),
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::EnvironmentFingerprintCard),
        EXPERIMENT_RUN_ROW_ENVIRONMENT_FINGERPRINT_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::DatasetProvenanceCard),
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SensitivitySharingBanner),
        DATASET_PROVENANCE_CARD_SENSITIVITY_SHARING_BANNER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ArtifactLineagePanel),
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ResultSummaryCard),
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RunComparisonTable),
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::CompareGuardBanner),
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_experiment_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EXPERIMENT_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_experiment_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5ExperimentComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5ExperimentComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_experiment_component_consumer_packet();
    for family in M5ExperimentComponentFamily::ALL {
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
    let packet = seeded_m5_experiment_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5ExperimentConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ExperimentConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5ExperimentComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_experiment_component_consumer_packet();
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
    let packet = seeded_m5_experiment_component_consumer_packet();
    let cases: Vec<&M5ExperimentComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5ExperimentConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5ExperimentConsumerNarrowingReason::ALL {
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
    for state in M5ExperimentClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn unproven_comparability_bindings_never_assert_apples_to_apples() {
    let packet = seeded_m5_experiment_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_unproven_comparability {
                    seen = true;
                    assert!(!case.resolved.asserts_apples_to_apples_parity);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(
        seen,
        "no unproven-comparability binding present to prove the comparability honesty criterion"
    );
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_experiment_component_consumer_packet();
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
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5ExperimentComponentConsumer::ReviewEvidence);
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5ExperimentComponentDescriptor::ExportScope);
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5ExperimentConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    // Strip every ExperimentRunRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5ExperimentComponentFamily::ExperimentRunRow {
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
        .contains(&M5ExperimentComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ExperimentComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ExperimentComponentConsumerViolation::NarrowingDisclosureUnproven)
    );
}

#[test]
fn comparability_honesty_unproven_fails_when_no_unproven_example_present() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    // Replace every binding with a full-parity case: no unproven-comparability state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5ExperimentComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ComparabilityHonestyUnproven));
}

#[test]
fn comparability_honesty_unproven_fails_when_unproven_state_claims_apples_to_apples() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    // Find an unproven-comparability binding and force it to assert apples-to-apples parity.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_unproven_comparability {
                    case.resolved.asserts_apples_to_apples_parity = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ComparabilityHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].implies_apples_to_apples_without_parity = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn exposes_raw_payload_invariant_violation_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].exposes_raw_payload_by_default = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5ExperimentComponentConsumer::SupportExport)
        .expect("support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5ExperimentComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet
        .governance_review
        .comparison_never_implies_apples_to_apples_without_parity = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.consumer_projection.export_scope_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_experiment_component_consumer_packet().render_markdown_summary();
    for consumer in M5ExperimentComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_experiment_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ExperimentComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5ExperimentComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_experiment_component_consumer_export()
        .expect("checked M5 experiment component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_EXPERIMENT_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_experiment_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_experiment_component_consumer_compare_view_beta_narrowed(),
        seeded_m5_experiment_component_consumer_review_evidence_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5ExperimentComponentConsumer::ALL.len()
        );
    }

    let compare = seeded_m5_experiment_component_consumer_compare_view_beta_narrowed();
    let row = compare
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ExperimentComponentConsumer::CompareView)
        .expect("compare-view row present");
    assert_eq!(row.qualification, M5ExperimentQualificationClass::Beta);

    let review = seeded_m5_experiment_component_consumer_review_evidence_preview_narrowed();
    let row = review
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5ExperimentComponentConsumer::ReviewEvidence)
        .expect("review-evidence row present");
    assert_eq!(row.qualification, M5ExperimentQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let compare: M5ExperimentComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-experiment-component-consumers/compare_view_beta_narrowed.json"
    )))
    .expect("compare-view fixture parses");
    assert!(compare.validate().is_empty());
    assert_eq!(
        compare,
        seeded_m5_experiment_component_consumer_compare_view_beta_narrowed()
    );

    let review: M5ExperimentComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-experiment-component-consumers/review_evidence_preview_narrowed.json"
    )))
    .expect("review-evidence fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_experiment_component_consumer_review_evidence_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_experiment_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
