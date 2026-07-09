use super::*;

fn full_input(
    consumer: M5WorkItemComponentConsumer,
    family: M5WorkItemComponentFamily,
) -> M5WorkItemComponentBindingInput {
    M5WorkItemComponentBindingInput {
        consumer,
        component_family: family,
        descriptor_families: M5WorkItemComponentDescriptor::ALL.to_vec(),
        parity_health: M5WorkItemConsumerParityHealth::FullParity,
        export_caveats: vec![],
        note_repr: Some("worked binding".to_owned()),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_full_parity_preserves_descriptors_with_no_banner() {
    let resolved = resolve_work_item_component_binding(&full_input(
        M5WorkItemComponentConsumer::Inbox,
        M5WorkItemComponentFamily::WorkItemRow,
    ))
    .expect("resolves");
    assert!(!resolved.is_narrowed);
    assert!(resolved.auto_narrow_banner.is_none());
    assert!(!resolved.reflects_queued_or_offline_state);
    assert!(resolved.asserts_provider_committed);
    assert_eq!(
        resolved.claim_parity_state,
        M5WorkItemClaimParityState::ClaimsPreserved
    );
    assert_eq!(
        resolved.canonical_schema_ref,
        family_canonical_schema_ref(M5WorkItemComponentFamily::WorkItemRow)
    );
}

#[test]
fn resolver_narrowed_parity_discloses_self_contained_banner() {
    let input = M5WorkItemComponentBindingInput {
        parity_health: M5WorkItemConsumerParityHealth::ProviderScopeLimitedNarrowed,
        export_caveats: vec![M5WorkItemConsumerExportCaveat::ScopeLimitedReadOnly],
        ..full_input(
            M5WorkItemComponentConsumer::Review,
            M5WorkItemComponentFamily::StatusTransitionSheet,
        )
    };
    let resolved = resolve_work_item_component_binding(&input).expect("resolves");
    assert!(resolved.is_narrowed);
    assert!(!resolved.asserts_provider_committed);
    assert_eq!(
        resolved.claim_parity_state,
        M5WorkItemClaimParityState::ClaimsAutoNarrowed
    );
    let banner = resolved.auto_narrow_banner.expect("banner present");
    assert_eq!(
        banner.reason,
        M5WorkItemConsumerNarrowingReason::ProviderScopeLimited
    );
    assert_eq!(
        banner.recovery_action,
        M5WorkItemConsumerRecoveryAction::ReauthorizeForFullScope
    );
    assert_eq!(
        banner.preserved_descriptors.len(),
        M5WorkItemComponentDescriptor::ALL.len()
    );
    assert!(!banner.headline.trim().is_empty());
    assert!(banner.headline.to_lowercase().contains("provider scope"));
}

#[test]
fn resolver_queued_or_offline_state_never_asserts_committed() {
    for (health, reason) in [
        (
            M5WorkItemConsumerParityHealth::SyncPendingNarrowed,
            M5WorkItemConsumerNarrowingReason::SyncPending,
        ),
        (
            M5WorkItemConsumerParityHealth::OfflineHandoffNarrowed,
            M5WorkItemConsumerNarrowingReason::OfflineHandoffLocalOnly,
        ),
    ] {
        let input = M5WorkItemComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5WorkItemComponentConsumer::Incident,
                M5WorkItemComponentFamily::OfflineHandoffPacketCard,
            )
        };
        let resolved = resolve_work_item_component_binding(&input).expect("resolves");
        assert!(resolved.reflects_queued_or_offline_state);
        assert!(!resolved.asserts_provider_committed);
        assert!(resolved.is_narrowed);
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_each_narrowed_mode_maps_to_its_reason() {
    for (health, reason) in [
        (
            M5WorkItemConsumerParityHealth::ProviderScopeLimitedNarrowed,
            M5WorkItemConsumerNarrowingReason::ProviderScopeLimited,
        ),
        (
            M5WorkItemConsumerParityHealth::SyncPendingNarrowed,
            M5WorkItemConsumerNarrowingReason::SyncPending,
        ),
        (
            M5WorkItemConsumerParityHealth::OfflineHandoffNarrowed,
            M5WorkItemConsumerNarrowingReason::OfflineHandoffLocalOnly,
        ),
        (
            M5WorkItemConsumerParityHealth::LinkedContextStaleNarrowed,
            M5WorkItemConsumerNarrowingReason::LinkedContextStale,
        ),
    ] {
        let input = M5WorkItemComponentBindingInput {
            parity_health: health,
            ..full_input(
                M5WorkItemComponentConsumer::Support,
                M5WorkItemComponentFamily::WorkItemRow,
            )
        };
        let resolved = resolve_work_item_component_binding(&input).expect("resolves");
        assert_eq!(resolved.auto_narrow_banner.expect("banner").reason, reason);
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty = M5WorkItemComponentBindingInput {
        descriptor_families: vec![],
        ..full_input(
            M5WorkItemComponentConsumer::Inbox,
            M5WorkItemComponentFamily::WorkItemRow,
        )
    };
    assert_eq!(
        resolve_work_item_component_binding(&empty),
        Err(M5WorkItemComponentBindingError::EmptyDescriptorSet)
    );

    let missing = M5WorkItemComponentBindingInput {
        descriptor_families: vec![M5WorkItemComponentDescriptor::CanonicalIdentity],
        ..full_input(
            M5WorkItemComponentConsumer::Inbox,
            M5WorkItemComponentFamily::WorkItemRow,
        )
    };
    assert_eq!(
        resolve_work_item_component_binding(&missing),
        Err(M5WorkItemComponentBindingError::MissingRequiredDescriptor)
    );

    let forbidden = M5WorkItemComponentBindingInput {
        note_repr: Some("https://example.test/leak".to_owned()),
        ..full_input(
            M5WorkItemComponentConsumer::Inbox,
            M5WorkItemComponentFamily::WorkItemRow,
        )
    };
    assert_eq!(
        resolve_work_item_component_binding(&forbidden),
        Err(M5WorkItemComponentBindingError::ForbiddenBindingMaterial)
    );
}

#[test]
fn family_canonical_refs_match_the_narrowed_primitives() {
    use crate::implement_related_evidence_cards_and_offline_handoff_packet_cards_with_summary_first_evidence_redaction_state_publish_later_target_and_copy_export_retry_truth::EVIDENCE_HANDOFF_SCHEMA_REF;
    use crate::implement_relation_strips_and_sync_pending_pills_with_linked_context_stale_labeling_and_retry_or_export_continuity::RELATION_STRIP_SYNC_PENDING_SCHEMA_REF;
    use crate::implement_work_item_detail_headers_and_status_transition_sheets_with_provider_boundary_side_effect_permission_scope_and_confirm_export_cancel_truth::DETAIL_HEADER_TRANSITION_SCHEMA_REF;
    use crate::implement_work_item_rows_and_provider_chip_groups_with_canonical_id_owner_state_freshness_and_write_scope_truth::WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF;
    use M5WorkItemComponentFamily as Family;

    assert_eq!(
        family_canonical_schema_ref(Family::WorkItemRow),
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::ProviderChipGroup),
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RelationStrip),
        RELATION_STRIP_SYNC_PENDING_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::SyncPendingPill),
        RELATION_STRIP_SYNC_PENDING_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::WorkItemDetailHeader),
        DETAIL_HEADER_TRANSITION_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::StatusTransitionSheet),
        DETAIL_HEADER_TRANSITION_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::RelatedEvidenceCard),
        EVIDENCE_HANDOFF_SCHEMA_REF
    );
    assert_eq!(
        family_canonical_schema_ref(Family::OfflineHandoffPacketCard),
        EVIDENCE_HANDOFF_SCHEMA_REF
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_work_item_component_consumer_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WORK_ITEM_COMPONENT_CONSUMER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer() {
    let packet = seeded_m5_work_item_component_consumer_packet();
    let present: std::collections::BTreeSet<_> =
        packet.consumer_rows.iter().map(|r| r.consumer).collect();
    for consumer in M5WorkItemComponentConsumer::ALL {
        assert!(
            present.contains(&consumer),
            "missing consumer {}",
            consumer.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5WorkItemComponentConsumer::ALL.len()
    );
}

#[test]
fn every_family_is_reused_across_at_least_two_consumers() {
    let packet = seeded_m5_work_item_component_consumer_packet();
    for family in M5WorkItemComponentFamily::ALL {
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
    let packet = seeded_m5_work_item_component_consumer_packet();
    for row in &packet.consumer_rows {
        for part in M5WorkItemConsumerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5WorkItemConsumerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for descriptor in M5WorkItemComponentDescriptor::REQUIRED {
            assert!(row.descriptor_families.contains(&descriptor));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable));
        assert!(!row.component_bindings.is_empty());
    }
}

#[test]
fn every_binding_points_to_canonical_family() {
    let packet = seeded_m5_work_item_component_consumer_packet();
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
    let packet = seeded_m5_work_item_component_consumer_packet();
    let cases: Vec<&M5WorkItemComponentBindingCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|b| b.example_bindings.iter())
        .collect();

    for health in M5WorkItemConsumerParityHealth::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.parity_health == health),
            "no worked binding exercises parity-health mode {}",
            health.as_str()
        );
    }
    for reason in M5WorkItemConsumerNarrowingReason::ALL {
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
    for state in M5WorkItemClaimParityState::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.claim_parity_state == state),
            "no worked binding exercises claim-parity state {}",
            state.as_str()
        );
    }
}

#[test]
fn queued_or_offline_bindings_never_assert_committed() {
    let packet = seeded_m5_work_item_component_consumer_packet();
    let mut seen = false;
    for row in &packet.consumer_rows {
        for b in &row.component_bindings {
            for case in &b.example_bindings {
                if case.resolved.reflects_queued_or_offline_state {
                    seen = true;
                    assert!(!case.resolved.asserts_provider_committed);
                    assert!(case.resolved.is_narrowed);
                }
            }
        }
    }
    assert!(seen, "no queued / offline binding present to prove AC2");
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_work_item_component_consumer_packet();
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
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer != M5WorkItemComponentConsumer::Incident);
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.vocabulary_set.parity_health_modes.pop();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::VocabularySetDrift));
}

#[test]
fn canonical_ref_mismatch_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].canonical_schema_ref =
        "schemas/ui/not-canonical.json".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn local_prose_reference_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].references_canonical_not_local_prose = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::CanonicalRefMismatch));
}

#[test]
fn required_descriptor_missing_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0]
        .descriptor_families
        .retain(|d| *d != M5WorkItemComponentDescriptor::PublishLaterContinuity);
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::RequiredDescriptorMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5WorkItemConsumerExportField::CanonicalSchemaRef);
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0].component_bindings[0].example_bindings[0]
        .resolved
        .is_narrowed = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[1].component_bindings[0]
        .example_bindings
        .clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ExampleBindingMissing));
}

#[test]
fn family_reuse_unproven_fails_when_a_family_drops_below_two_consumers() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    // Strip every WorkItemRow binding except the first consumer's.
    let mut seen_first = false;
    for row in &mut packet.consumer_rows {
        row.component_bindings.retain(|b| {
            if b.component_family == M5WorkItemComponentFamily::WorkItemRow {
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
        .contains(&M5WorkItemComponentConsumerViolation::ComponentFamilyReuseUnproven));
}

#[test]
fn narrowing_disclosure_unproven_fails_when_no_narrowed_example_present() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5WorkItemComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5WorkItemComponentConsumerViolation::NarrowingDisclosureUnproven));
}

#[test]
fn commit_honesty_unproven_fails_when_no_queued_or_offline_example_present() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    // Replace every binding with a full-parity case: no queued / offline state remains.
    for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            b.example_bindings = vec![M5WorkItemComponentBindingCase::resolved(full_input(
                row.consumer,
                b.component_family,
            ))];
        }
    }
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::CommitHonestyUnproven));
}

#[test]
fn commit_honesty_unproven_fails_when_queued_state_claims_committed() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    // Find a queued / offline binding and force it to assert committed state.
    'outer: for row in &mut packet.consumer_rows {
        for b in &mut row.component_bindings {
            for case in &mut b.example_bindings {
                if case.resolved.reflects_queued_or_offline_state {
                    case.resolved.asserts_provider_committed = true;
                    break 'outer;
                }
            }
        }
    }
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::CommitHonestyUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0].shows_queued_or_offline_state_as_committed = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::MissingSourceContracts));
}

#[test]
fn help_support_export_reference_missing_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|r| r.consumer == M5WorkItemComponentConsumer::Support)
        .expect("support row present");
    row.component_bindings[0].references_canonical_not_local_prose = false;
    let violations = packet.validate();
    assert!(violations
        .contains(&M5WorkItemComponentConsumerViolation::HelpSupportExportReferenceMissing));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet
        .governance_review
        .queued_or_offline_state_never_shown_as_committed = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet
        .consumer_projection
        .publish_later_continuity_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentConsumerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer() {
    let summary = seeded_m5_work_item_component_consumer_packet().render_markdown_summary();
    for consumer in M5WorkItemComponentConsumer::ALL {
        assert!(
            summary.contains(consumer.label()),
            "summary missing consumer {}",
            consumer.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_work_item_component_consumer_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5WorkItemComponentConsumer::ALL.len());
    assert!(lines[0].starts_with("consumer,qualification,owner,"));
    for consumer in M5WorkItemComponentConsumer::ALL {
        assert!(
            csv.contains(consumer.as_str()),
            "csv missing consumer {}",
            consumer.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_work_item_component_consumer_export()
        .expect("checked M5 work-item component consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_WORK_ITEM_COMPONENT_CONSUMER_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_work_item_component_consumer_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_work_item_component_consumer_incident_beta_narrowed(),
        seeded_m5_work_item_component_consumer_review_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5WorkItemComponentConsumer::ALL.len()
        );
    }

    let incident = seeded_m5_work_item_component_consumer_incident_beta_narrowed();
    let row = incident
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5WorkItemComponentConsumer::Incident)
        .expect("incident row present");
    assert_eq!(row.qualification, M5WorkItemQualificationClass::Beta);

    let review = seeded_m5_work_item_component_consumer_review_preview_narrowed();
    let row = review
        .consumer_rows
        .iter()
        .find(|r| r.consumer == M5WorkItemComponentConsumer::Review)
        .expect("review row present");
    assert_eq!(row.qualification, M5WorkItemQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let incident: M5WorkItemComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-work-item-component-consumers/incident_beta_narrowed.json"
    )))
    .expect("incident fixture parses");
    assert!(incident.validate().is_empty());
    assert_eq!(
        incident,
        seeded_m5_work_item_component_consumer_incident_beta_narrowed()
    );

    let review: M5WorkItemComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-work-item-component-consumers/review_preview_narrowed.json"
    )))
    .expect("review fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_work_item_component_consumer_review_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_work_item_component_consumer_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
