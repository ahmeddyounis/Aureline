use super::*;

fn full_input(
    consumer: M5HistoryComponentConsumer,
    family: M5LocalHistoryWriteScopeComponentFamily,
) -> M5HistoryBindingInput {
    M5HistoryBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5HistoryComponentDescriptor::ALL.to_vec(),
        parity_health: M5HistoryConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_history_binding(&full_input(
        M5HistoryComponentConsumer::EditorRenameRefactor,
        M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert_eq!(
        resolved.claim_parity_state,
        M5HistoryClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5HistoryBindingInput {
        parity_health: M5HistoryConsumerParityHealth::PreviewOnlyNarrowed,
        export_caveats: vec![M5HistoryConsumerExportCaveat::RestoreCommitDisabledPreviewOnly],
        ..full_input(
            M5HistoryComponentConsumer::RepairTransaction,
            M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard,
        )
    };
    let resolved = resolve_history_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert_eq!(
        resolved.claim_parity_state,
        M5HistoryClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5HistoryConsumerNarrowingReason::PreviewOnlyWorkflow
    );
    assert_eq!(
        banner.recovery_action,
        M5HistoryConsumerRecoveryAction::ReturnToRecoveryCenterToCommit
    );
    // Descriptors stay preserved even under the narrowing.
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5HistoryComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    // Not a generic "degraded" note.
    assert!(banner.headline.to_lowercase().contains("preview-only"));
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5HistoryConsumerParityHealth::PreviewOnlyNarrowed,
            M5HistoryConsumerNarrowingReason::PreviewOnlyWorkflow,
        ),
        (
            M5HistoryConsumerParityHealth::ExternalDriftNarrowed,
            M5HistoryConsumerNarrowingReason::ExternalDriftUnreconciled,
        ),
        (
            M5HistoryConsumerParityHealth::GeneratedManagedNarrowed,
            M5HistoryConsumerNarrowingReason::GeneratedOrManagedScope,
        ),
        (
            M5HistoryConsumerParityHealth::ExportRedactedNarrowed,
            M5HistoryConsumerNarrowingReason::ExportRedactionApplied,
        ),
    ] {
        let input = M5HistoryBindingInput {
            parity_health: health,
            ..full_input(
                M5HistoryComponentConsumer::AiReview,
                M5LocalHistoryWriteScopeComponentFamily::RestorePreviewCard,
            )
        };
        let resolved = resolve_history_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5HistoryBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5HistoryComponentConsumer::EditorRenameRefactor,
            M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow,
        )
    };
    assert_eq!(
        resolve_history_binding(&empty),
        Err(M5HistoryBindingError::EmptyDescriptorSet)
    );

    let missing = M5HistoryBindingInput {
        descriptor_families: vec![M5HistoryComponentDescriptor::Restore],
        ..full_input(
            M5HistoryComponentConsumer::EditorRenameRefactor,
            M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow,
        )
    };
    assert_eq!(
        resolve_history_binding(&missing),
        Err(M5HistoryBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5HistoryBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5HistoryComponentConsumer::EditorRenameRefactor,
            M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow,
        )
    };
    assert_eq!(
        resolve_history_binding(&forbidden),
        Err(M5HistoryBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_local_history_rows_and_checkpoint_group_cards_with_actor_lineage_scope_trigger_retention_and_grouped_restore_truth_across_claimed_m5_recovery_surfaces::M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF;
    use crate::implement_restore_preview_cards_with_external_drift_generated_managed_file_caveats_restore_granularity_and_no_history_erasure_truth_across_claimed_m5_mutation_recovery_lanes::M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF;
    use crate::implement_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows::M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF;
    use crate::ship_cross_baseline_compare_and_export_flows_so_current_versus_snapshot_snapshot_versus_disk_snapshot_versus_git_and_patch_or_evidence_export_stay_explicit_across_claimed_m5_history_refactor_import_ai_paths::{
        M5_COMPARE_EXPORT_CARD_SCHEMA_REF, M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
    };
    use M5LocalHistoryWriteScopeComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::LocalHistoryRow),
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::CheckpointGroupCard),
        M5_LOCAL_HISTORY_ROW_GROUP_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RestorePreviewCard),
        M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RestoreGranularitySelector),
        M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::WriteScopePreviewTree),
        M5_WRITE_SCOPE_PREVIEW_TREE_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RetentionExportCard),
        M5_COMPARE_EXPORT_CARD_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::HistoryExportManifest),
        M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5HistoryComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5HistoryComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    for family in M5LocalHistoryWriteScopeComponentFamily::ALL {
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
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5HistoryConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5HistoryConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5HistoryComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
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
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    let cases: Vec<&M5HistoryBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5HistoryConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5HistoryConsumerNarrowingReason::ALL {
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
    for state in M5HistoryClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_local_history_write_scope_component_consumer_packet();
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
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5HistoryComponentConsumer::AiReview);
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5HistoryComponentDescriptor::Restore);
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5HistoryConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    // Strip every LocalHistoryRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5LocalHistoryWriteScopeComponentFamily::LocalHistoryRow {
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
        .contains(&M5HistoryComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5HistoryBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0].inherits_stronger_label_from_healthier_lane = true;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn support_export_reference_missing_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5HistoryComponentConsumer::SupportExport)
        .expect("support / export row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5HistoryComponentConsumerViolation::SupportExportReferenceMissing)
    );
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet
        .governance_review
        .degraded_workflow_auto_narrows_claim = false;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.consumer_projection.restore_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_local_history_write_scope_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5HistoryComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary =
        seeded_m5_local_history_write_scope_component_consumer_packet().render_markdown_summary();
    for consumer in M5HistoryComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_local_history_write_scope_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5HistoryComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5HistoryComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_local_history_write_scope_component_consumer_export()
        .expect("checked M5 local-history write-scope component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_local_history_write_scope_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed(),
        seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5HistoryComponentConsumer::ALL.len()
        );
    }

    let import = seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed();
    let row = import
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5HistoryComponentConsumer::ImportMigration)
        .expect("import / migration row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);

    let ai = seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed();
    let row = ai
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5HistoryComponentConsumer::AiReview)
        .expect("ai-review row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let import: M5HistoryComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-local-history-write-scope-component-consumers/import_migration_preview_narrowed.json"
    )))
    .expect("import / migration fixture parses");
    assert!(import.validate().is_empty());
    assert_eq!(
        import,
        seeded_m5_local_history_write_scope_component_consumer_import_migration_preview_narrowed()
    );

    let ai: M5HistoryComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-local-history-write-scope-component-consumers/ai_review_beta_narrowed.json"
    )))
    .expect("ai-review fixture parses");
    assert!(ai.validate().is_empty());
    assert_eq!(
        ai,
        seeded_m5_local_history_write_scope_component_consumer_ai_review_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json =
        seeded_m5_local_history_write_scope_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
