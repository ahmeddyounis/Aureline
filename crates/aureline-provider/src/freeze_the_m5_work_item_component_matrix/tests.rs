use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_work_item_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WORK_ITEM_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_work_item_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5WorkItemComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5WorkItemComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_work_item_component_matrix();
    for row in &packet.component_rows {
        for label in M5WorkItemRequiredLabel::MANDATORY {
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
            .contains(&M5WorkItemAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_work_item_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.work_item_kinds.is_empty(),
            family.carries_work_item_kind(),
            "work_item_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.provider_authorities.is_empty(),
            family.carries_provider_authority(),
            "provider_authorities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.local_states.is_empty(),
            family.carries_local_state(),
            "local_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.relation_kinds.is_empty(),
            family.is_relation_strip(),
            "relation_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.evidence_kinds.is_empty(),
            family.is_related_evidence_card(),
            "evidence_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.transition_effects.is_empty(),
            family.is_status_transition_sheet(),
            "transition_effects presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.handoff_destinations.is_empty(),
            family.is_offline_handoff_packet_card(),
            "handoff_destinations presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.export_boundaries.is_empty(),
            family.is_offline_handoff_packet_card(),
            "export_boundaries presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_work_item_component_matrix();
    for kind in M5WorkItemKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.work_item_kinds.contains(&kind)),
            "no component declares work-item kind {}",
            kind.as_str()
        );
    }
    for authority in M5WorkItemProviderAuthority::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.provider_authorities.contains(&authority)),
            "no component declares provider authority {}",
            authority.as_str()
        );
    }
    for state in M5WorkItemLocalState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.local_states.contains(&state)),
            "no component declares local state {}",
            state.as_str()
        );
    }
    for relation in M5WorkItemRelationKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.relation_kinds.contains(&relation)),
            "no component declares relation kind {}",
            relation.as_str()
        );
    }
    for evidence in M5WorkItemEvidenceKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.evidence_kinds.contains(&evidence)),
            "no component declares evidence kind {}",
            evidence.as_str()
        );
    }
    for effect in M5WorkItemTransitionEffect::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.transition_effects.contains(&effect)),
            "no component declares transition effect {}",
            effect.as_str()
        );
    }
    for destination in M5WorkItemHandoffDestination::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.handoff_destinations.contains(&destination)),
            "no component declares handoff destination {}",
            destination.as_str()
        );
    }
    for boundary in M5WorkItemExportBoundary::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.export_boundaries.contains(&boundary)),
            "no component declares export boundary {}",
            boundary.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5WorkItemComponentFamily::RelationStrip);
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.vocabulary_set.provider_authorities.pop();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5WorkItemRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn work_item_row_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_work_item_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5WorkItemComponentFamily::WorkItemRow)
            .expect("work-item row present");
        let expected = match clear {
            0 => {
                row.work_item_kinds.clear();
                M5WorkItemComponentMatrixViolation::WorkItemKindMissing
            }
            1 => {
                row.provider_authorities.clear();
                M5WorkItemComponentMatrixViolation::ProviderAuthorityMissing
            }
            _ => {
                row.local_states.clear();
                M5WorkItemComponentMatrixViolation::LocalStateMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn relation_strip_vocab_missing_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::RelationStrip)
        .expect("relation-strip row present");
    row.relation_kinds.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::RelationKindMissing));
}

#[test]
fn evidence_card_vocab_missing_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::RelatedEvidenceCard)
        .expect("related-evidence-card row present");
    row.evidence_kinds.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::EvidenceKindMissing));
}

#[test]
fn transition_sheet_vocab_missing_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::StatusTransitionSheet)
        .expect("status-transition-sheet row present");
    row.transition_effects.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::TransitionEffectMissing));
}

#[test]
fn offline_handoff_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_work_item_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5WorkItemComponentFamily::OfflineHandoffPacketCard)
            .expect("offline-handoff-packet-card row present");
        let expected = if clear == 0 {
            row.handoff_destinations.clear();
            M5WorkItemComponentMatrixViolation::HandoffDestinationMissing
        } else {
            row.export_boundaries.clear();
            M5WorkItemComponentMatrixViolation::ExportBoundaryMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[0].masks_identity_or_authority = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[3].hides_local_or_publish_later_state = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[5].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[1].uses_generic_ticket_wording = true;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::WorkItemRow)
        .expect("work-item row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet
        .governance_review
        .no_generic_ticket_wording_conceals_authority = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WorkItemComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_work_item_component_matrix().render_markdown_summary();
    for family in M5WorkItemComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_work_item_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5WorkItemComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5WorkItemComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_work_item_component_matrix_export()
        .expect("checked M5 work-item component matrix export validates");
    assert_eq!(packet.packet_id, M5_WORK_ITEM_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_work_item_component_matrix_export()
        .expect("checked M5 work-item component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_work_item_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed(),
        seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5WorkItemComponentFamily::ALL.len()
        );
    }

    let transition = seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed();
    let row = transition
        .component_rows
        .iter()
        .find(|r| r.component_family == M5WorkItemComponentFamily::StatusTransitionSheet)
        .expect("status-transition-sheet row present");
    assert_eq!(row.qualification, M5WorkItemQualificationClass::Beta);

    let handoff =
        seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed();
    let row = handoff
        .component_rows
        .iter()
        .find(|r| r.component_family == M5WorkItemComponentFamily::OfflineHandoffPacketCard)
        .expect("offline-handoff-packet-card row present");
    assert_eq!(row.qualification, M5WorkItemQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let transition: M5WorkItemComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-work-item-components/status_transition_sheet_beta_narrowed.json"
    )))
    .expect("status-transition-sheet fixture parses");
    assert!(transition.validate().is_empty());
    assert_eq!(
        transition,
        seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed()
    );

    let handoff: M5WorkItemComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-work-item-components/offline_handoff_packet_card_preview_narrowed.json"
    )))
    .expect("offline-handoff-packet-card fixture parses");
    assert!(handoff.validate().is_empty());
    assert_eq!(
        handoff,
        seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_work_item_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
