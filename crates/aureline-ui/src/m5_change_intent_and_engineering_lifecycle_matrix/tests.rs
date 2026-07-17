use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_change_intent_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CHANGE_INTENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_change_intent_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .change_intent_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5ChangeIntentObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.change_intent_rows.len(),
        M5ChangeIntentObject::ALL.len()
    );
}

#[test]
fn frozen_change_intent_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5ChangeIntentRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "provider_ownership_disclosure",
            "local_versus_provider_state_disclosure",
            "linked_engineering_identity_disclosure",
            "side_effect_disclosure",
            "validation_evidence_disclosure",
            "publish_later_fallback_disclosure",
            "final_resolution_authority_disclosure",
        ]
    );
    assert!(M5ChangeIntentRole::ProviderOwnershipDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(M5ChangeIntentRole::LocalVersusProviderStateDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(M5ChangeIntentRole::LinkedEngineeringIdentityDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(M5ChangeIntentRole::SideEffectDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(!M5ChangeIntentRole::ValidationEvidenceDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(!M5ChangeIntentRole::PublishLaterFallbackDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
    assert!(!M5ChangeIntentRole::FinalResolutionAuthorityDisclosure
        .must_be_present_before_surfacing_as_a_change_intent_result());
}

#[test]
fn provider_committed_is_mechanically_distinct_from_local_only_draft() {
    let tokens: Vec<&str> = M5ChangeIntentCommitState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "provider_committed",
            "local_only_draft",
            "queued_for_publish",
            "publish_failed_retained",
            "provider_unavailable",
            "offline_handoff_packet",
            "stale_relative_to_provider",
        ]
    );
    assert!(M5ChangeIntentCommitState::ProviderCommitted.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::LocalOnlyDraft.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::QueuedForPublish.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::PublishFailedRetained.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::ProviderUnavailable.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::OfflineHandoffPacket.is_provider_committed());
    assert!(!M5ChangeIntentCommitState::StaleRelativeToProvider.is_provider_committed());
}

#[test]
fn relation_source_keeps_the_four_kinds_distinct() {
    let tokens: Vec<&str> = M5ChangeIntentRelationSource::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "linked_by_provider",
            "linked_locally",
            "suggested_by_aureline",
            "stale_or_broken_relation",
        ]
    );
    assert!(M5ChangeIntentRelationSource::LinkedByProvider.is_provider_linked());
    assert!(!M5ChangeIntentRelationSource::LinkedLocally.is_provider_linked());
    assert!(!M5ChangeIntentRelationSource::SuggestedByAureline.is_provider_linked());
    assert!(!M5ChangeIntentRelationSource::StaleOrBrokenRelation.is_provider_linked());
}

#[test]
fn blocker_state_names_blocked_and_escalated_states() {
    let tokens: Vec<&str> = M5ChangeIntentBlockerState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "ready_to_resolve",
            "blocked_by_engineering",
            "escalation_open",
            "awaiting_provider_write",
            "resolution_authority_missing",
        ]
    );
    assert!(!M5ChangeIntentBlockerState::ReadyToResolve.is_blocked_or_unresolved());
    assert!(M5ChangeIntentBlockerState::BlockedByEngineering.is_blocked_or_unresolved());
    assert!(M5ChangeIntentBlockerState::EscalationOpen.is_blocked_or_unresolved());
    assert!(M5ChangeIntentBlockerState::AwaitingProviderWrite.is_blocked_or_unresolved());
    assert!(M5ChangeIntentBlockerState::ResolutionAuthorityMissing.is_blocked_or_unresolved());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_change_intent_matrix();
    for row in &packet.change_intent_rows {
        for label in M5ChangeIntentRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "class {} missing mandatory label {}",
                row.object_class.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.object_class.canonical_domain_schema_ref().to_owned()),
            "class {} does not point at its canonical schema",
            row.object_class.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.classification_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ChangeIntentAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_change_intent_matrix();
    for row in &packet.change_intent_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.surface_label,
            &tr.provider_ownership,
            &tr.local_versus_provider_state,
            &tr.linked_engineering_identity,
            &tr.relation_source_state,
            &tr.blocker_state,
            &tr.validation_evidence,
        ] {
            assert!(
                !field.trim().is_empty(),
                "visible-state field empty on {}",
                row.object_class.as_str()
            );
        }
    }
}

#[test]
fn class_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_change_intent_matrix();
    for row in &packet.change_intent_rows {
        let class = row.object_class;
        assert_eq!(
            !row.change_intent_record_roles.is_empty(),
            class.declares_change_intent_record_roles(),
            "change_intent_record_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.start_work_roles.is_empty(),
            class.declares_start_work_roles(),
            "start_work_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.linked_change_roles.is_empty(),
            class.declares_linked_change_roles(),
            "linked_change_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.handoff_roles.is_empty(),
            class.declares_handoff_roles(),
            "handoff_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.resolve_roles.is_empty(),
            class.declares_resolve_roles(),
            "resolve_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.blocked_escalate_roles.is_empty(),
            class.declares_blocked_escalate_roles(),
            "blocked_escalate_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_change_intent_matrix();
    for role in M5ChangeIntentRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares change-intent role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentRecordRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.change_intent_record_roles.contains(&role)),
            "no class declares change_intent_record_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentStartWorkRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.start_work_roles.contains(&role)),
            "no class declares start_work_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentLinkedChangeRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.linked_change_roles.contains(&role)),
            "no class declares linked_change_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentHandoffRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.handoff_roles.contains(&role)),
            "no class declares handoff_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentResolveRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.resolve_roles.contains(&role)),
            "no class declares resolve_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeIntentBlockedEscalateRole::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.blocked_escalate_roles.contains(&role)),
            "no class declares blocked_escalate_role {}",
            role.as_str()
        );
    }
    for reason in M5ChangeIntentDegradedReason::ALL {
        assert!(
            packet
                .change_intent_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet
        .change_intent_rows
        .retain(|row| row.object_class != M5ChangeIntentObject::LinkedChangePanel);
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0]
        .required_labels
        .retain(|label| *label != M5ChangeIntentRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let own = M5ChangeIntentObject::StartWorkSheet.canonical_domain_schema_ref();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::StartWorkSheet)
        .expect("start-work-sheet row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::SemanticRoleMissing));
}

#[test]
fn change_intent_record_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::ChangeIntentRecord)
        .expect("ChangeIntentRecord row present");
    row.change_intent_record_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentRecordRoleMissing));
}

#[test]
fn start_work_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::StartWorkSheet)
        .expect("StartWorkSheet row present");
    row.start_work_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::StartWorkRoleMissing));
}

#[test]
fn linked_change_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::LinkedChangePanel)
        .expect("LinkedChangePanel row present");
    row.linked_change_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::LinkedChangeRoleMissing));
}

#[test]
fn handoff_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::ReadyForReviewHandoffSheet)
        .expect("ReadyForReviewHandoffSheet row present");
    row.handoff_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::HandoffRoleMissing));
}

#[test]
fn resolve_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::ResolveCloseSheet)
        .expect("ResolveCloseSheet row present");
    row.resolve_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ResolveRoleMissing));
}

#[test]
fn blocked_escalate_roles_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::BlockedEscalateCard)
        .expect("BlockedEscalateCard row present");
    row.blocked_escalate_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::BlockedEscalateRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0]
        .required_visible_state
        .surface_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0].backup_owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::DegradedReasonMissing));
}

#[test]
fn change_intent_invariant_violation_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0].lets_start_work_silently_create_a_side_effect_without_disclosure =
        true;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated));

    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[1]
        .lets_a_local_handoff_packet_or_queued_publish_masquerade_as_a_provider_committed_update =
        true;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated));

    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[2]
        .flattens_linked_by_provider_linked_locally_suggested_and_stale_into_one_relation_badge =
        true;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated));

    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[3]
        .auto_resolves_tracked_work_while_engineering_blockers_remain_unresolved = true;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated));

    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[4]
        .drops_local_notes_handoff_packets_or_linked_evidence_when_provider_write_fails = true;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ChangeIntentInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    let row = packet
        .change_intent_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeIntentObject::StartWorkSheet)
        .expect("start-work-sheet row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[1].classification_stages.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet
        .governance_review
        .provider_committed_state_is_mechanically_distinct_from_local_only_draft = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_change_intent_source = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_change_intent_matrix().render_markdown_summary();
    for class in M5ChangeIntentObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_change_intent_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ChangeIntentObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,commit_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5ChangeIntentObject::ALL {
        assert!(
            csv.contains(class.as_str()),
            "csv missing class {}",
            class.as_str()
        );
        assert!(
            csv.contains(class.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            class.as_str()
        );
    }
}

#[test]
fn dashboard_json_names_every_class_and_matches_checked_in_file() {
    let rendered: serde_json::Value =
        serde_json::from_str(&seeded_m5_change_intent_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5ChangeIntentObject::ALL {
        assert!(
            rendered["objects"]
                .as_array()
                .expect("objects array")
                .iter()
                .any(|c| c["object_class"] == class.as_str()),
            "dashboard missing class {}",
            class.as_str()
        );
    }
    let from_disk: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-change-intent-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked change-intent-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_change_intent_matrix_export()
        .expect("checked M5 change-intent matrix export validates");
    assert_eq!(packet.packet_id, M5_CHANGE_INTENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_change_intent_matrix_export()
        .expect("checked M5 change-intent matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_change_intent_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed(),
        seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.change_intent_rows.len(),
            M5ChangeIntentObject::ALL.len()
        );
    }

    let beta = seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed();
    let row = beta
        .change_intent_rows
        .iter()
        .find(|r| r.object_class == M5ChangeIntentObject::StartWorkSheet)
        .expect("start-work-sheet row present");
    assert_eq!(row.qualification, M5ChangeIntentQualificationClass::Beta);

    let preview = seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed();
    let row = preview
        .change_intent_rows
        .iter()
        .find(|r| r.object_class == M5ChangeIntentObject::BlockedEscalateCard)
        .expect("blocked-escalate-card row present");
    assert_eq!(row.qualification, M5ChangeIntentQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ChangeIntentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/teamwork/m5-change-intent/start_work_sheet_beta_narrowed.json"
    )))
    .expect("start-work-sheet fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed()
    );

    let preview: M5ChangeIntentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/teamwork/m5-change-intent/blocked_escalate_card_preview_narrowed.json"
    )))
    .expect("blocked-escalate-card fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_change_intent_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_change_intent_matrix();
    packet.change_intent_rows[0].scope_summary =
        "raw endpoint https://tracker.example/item leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ChangeIntentMatrixViolation::RawMaterialInExport));
}
