use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_collaboration_state_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COLLABORATION_STATE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_collaboration_state_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .collaboration_state_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5CollaborationStateObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.collaboration_state_rows.len(),
        M5CollaborationStateObject::ALL.len()
    );
}

#[test]
fn frozen_collaboration_state_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5CollaborationStateRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "authority_model_disclosure",
            "local_truth_preservation_disclosure",
            "merge_and_drift_semantics_disclosure",
            "downgrade_behavior_disclosure",
            "anchor_drift_history_disclosure",
            "export_posture_disclosure",
            "provenance_and_freshness_disclosure",
        ]
    );
    assert!(M5CollaborationStateRole::AuthorityModelDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(M5CollaborationStateRole::LocalTruthPreservationDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(M5CollaborationStateRole::MergeAndDriftSemanticsDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(M5CollaborationStateRole::DowngradeBehaviorDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(!M5CollaborationStateRole::AnchorDriftHistoryDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(!M5CollaborationStateRole::ExportPostureDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
    assert!(!M5CollaborationStateRole::ProvenanceAndFreshnessDisclosure
        .must_be_present_before_surfacing_as_a_collaboration_state_result());
}

#[test]
fn converged_state_is_mechanically_distinct_from_degraded() {
    let tokens: Vec<&str> = M5ConvergenceState::ALL.iter().map(|s| s.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "converged",
            "converging_pending_ops",
            "server_ordered",
            "host_authoritative",
            "locally_pending_unsent",
            "convergence_degraded",
            "awareness_degraded",
            "anchor_unresolved",
            "anchor_rebound_append_only",
            "relay_partitioned",
            "reconciliation_required",
            "compaction_pending",
            "sealed_archived",
            "local_canonical_preserved",
            "sampled_presence_only",
            "provenance_stale",
        ]
    );
    assert!(M5ConvergenceState::Converged.is_converged());
    for state in M5ConvergenceState::ALL {
        if state != M5ConvergenceState::Converged {
            assert!(
                !state.is_converged(),
                "state {} must not be converged",
                state.as_str()
            );
        }
    }
}

#[test]
fn authority_model_keeps_the_four_kinds_distinct() {
    let tokens: Vec<&str> = M5CollaborationAuthorityModel::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "crdt_convergent_replica",
            "server_ordered_sequence",
            "host_authoritative_state",
            "local_canonical_never_replaced",
        ]
    );
    assert!(M5CollaborationAuthorityModel::CrdtConvergentReplica.is_convergent_replica());
    assert!(!M5CollaborationAuthorityModel::ServerOrderedSequence.is_convergent_replica());
    assert!(!M5CollaborationAuthorityModel::HostAuthoritativeState.is_convergent_replica());
    assert!(!M5CollaborationAuthorityModel::LocalCanonicalNeverReplaced.is_convergent_replica());
}

#[test]
fn downgrade_gate_names_blocked_states() {
    let tokens: Vec<&str> = M5CollaborationDowngradeGate::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "converged_local_work_preserved",
            "blocked_by_unsent_local_work_at_risk",
            "blocked_by_permission_downgrade",
            "blocked_by_relay_partition",
            "blocked_by_unreviewed_anchor_drift",
        ]
    );
    assert!(!M5CollaborationDowngradeGate::ConvergedLocalWorkPreserved
        .is_blocked_pending_local_preservation());
    assert!(M5CollaborationDowngradeGate::BlockedByUnsentLocalWorkAtRisk
        .is_blocked_pending_local_preservation());
    assert!(M5CollaborationDowngradeGate::BlockedByPermissionDowngrade
        .is_blocked_pending_local_preservation());
    assert!(M5CollaborationDowngradeGate::BlockedByRelayPartition
        .is_blocked_pending_local_preservation());
    assert!(M5CollaborationDowngradeGate::BlockedByUnreviewedAnchorDrift
        .is_blocked_pending_local_preservation());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_collaboration_state_matrix();
    for row in &packet.collaboration_state_rows {
        for label in M5CollaborationStateRequiredLabel::MANDATORY {
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
            .contains(&M5CollaborationStateAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_collaboration_state_matrix();
    for row in &packet.collaboration_state_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.surface_label,
            &tr.authority_model,
            &tr.convergence_state,
            &tr.local_truth_disposition,
            &tr.merge_and_drift_summary,
            &tr.export_posture,
            &tr.provenance_and_freshness,
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
    let packet = seeded_m5_collaboration_state_matrix();
    for row in &packet.collaboration_state_rows {
        let class = row.object_class;
        assert_eq!(
            !row.crdt_backed_shared_text_roles.is_empty(),
            class.declares_crdt_backed_shared_text_roles(),
            "crdt_backed_shared_text_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.sampled_presence_cursors_selections_roles.is_empty(),
            class.declares_sampled_presence_cursors_selections_roles(),
            "sampled_presence_cursors_selections_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.server_ordered_comments_annotations_review_pins_roles
                .is_empty(),
            class.declares_server_ordered_comments_annotations_review_pins_roles(),
            "server_ordered_comments_annotations_review_pins_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.presenter_follow_state_roles.is_empty(),
            class.declares_presenter_follow_state_roles(),
            "presenter_follow_state_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.higher_risk_control_plane_roles.is_empty(),
            class.declares_higher_risk_control_plane_roles(),
            "higher_risk_control_plane_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.sealed_session_archive_roles.is_empty(),
            class.declares_sealed_session_archive_roles(),
            "sealed_session_archive_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_collaboration_state_matrix();
    for role in M5CollaborationStateRole::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares collaboration-state role {}",
            role.as_str()
        );
    }
    for role in M5CrdtBackedSharedTextRole::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.crdt_backed_shared_text_roles.contains(&role)),
            "no class declares crdt_backed_shared_text_role {}",
            role.as_str()
        );
    }
    for role in M5SampledPresenceCursorsSelectionsRole::ALL {
        assert!(
            packet.collaboration_state_rows.iter().any(|row| row
                .sampled_presence_cursors_selections_roles
                .contains(&role)),
            "no class declares sampled_presence_cursors_selections_role {}",
            role.as_str()
        );
    }
    for role in M5ServerOrderedCommentsAnnotationsReviewPinsRole::ALL {
        assert!(
            packet.collaboration_state_rows.iter().any(|row| row
                .server_ordered_comments_annotations_review_pins_roles
                .contains(&role)),
            "no class declares server_ordered_comments_annotations_review_pins_role {}",
            role.as_str()
        );
    }
    for role in M5PresenterFollowStateRole::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.presenter_follow_state_roles.contains(&role)),
            "no class declares presenter_follow_state_role {}",
            role.as_str()
        );
    }
    for role in M5HigherRiskControlPlaneRole::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.higher_risk_control_plane_roles.contains(&role)),
            "no class declares higher_risk_control_plane_role {}",
            role.as_str()
        );
    }
    for role in M5SealedSessionArchiveRole::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.sealed_session_archive_roles.contains(&role)),
            "no class declares sealed_session_archive_role {}",
            role.as_str()
        );
    }
    for reason in M5CollaborationStateDegradedReason::ALL {
        assert!(
            packet
                .collaboration_state_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet
        .collaboration_state_rows
        .retain(|row| row.object_class != M5CollaborationStateObject::PresenterFollowState);
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0]
        .required_labels
        .retain(|label| *label != M5CollaborationStateRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let own =
        M5CollaborationStateObject::SampledPresenceCursorsSelections.canonical_domain_schema_ref();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| {
            row.object_class == M5CollaborationStateObject::SampledPresenceCursorsSelections
        })
        .expect("sampled-presence row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::SemanticRoleMissing));
}

#[test]
fn crdt_backed_shared_text_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::CrdtBackedSharedText)
        .expect("CrdtBackedSharedText row present");
    row.crdt_backed_shared_text_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CrdtBackedSharedTextRoleMissing));
}

#[test]
fn sampled_presence_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| {
            row.object_class == M5CollaborationStateObject::SampledPresenceCursorsSelections
        })
        .expect("SampledPresenceCursorsSelections row present");
    row.sampled_presence_cursors_selections_roles.clear();
    assert!(packet.validate().contains(
        &M5CollaborationStateMatrixViolation::SampledPresenceCursorsSelectionsRoleMissing
    ));
}

#[test]
fn server_ordered_comment_pin_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| {
            row.object_class
                == M5CollaborationStateObject::ServerOrderedCommentsAnnotationsReviewPins
        })
        .expect("ServerOrderedCommentsAnnotationsReviewPins row present");
    row.server_ordered_comments_annotations_review_pins_roles
        .clear();
    assert!(packet.validate().contains(
        &M5CollaborationStateMatrixViolation::ServerOrderedCommentsAnnotationsReviewPinsRoleMissing
    ));
}

#[test]
fn presenter_follow_state_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::PresenterFollowState)
        .expect("PresenterFollowState row present");
    row.presenter_follow_state_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::PresenterFollowStateRoleMissing));
}

#[test]
fn higher_risk_control_plane_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::HigherRiskControlPlane)
        .expect("HigherRiskControlPlane row present");
    row.higher_risk_control_plane_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::HigherRiskControlPlaneRoleMissing));
}

#[test]
fn sealed_session_archive_roles_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::SealedSessionArchive)
        .expect("SealedSessionArchive row present");
    row.sealed_session_archive_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::SealedSessionArchiveRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0]
        .required_visible_state
        .surface_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0].backup_owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::DegradedReasonMissing));
}

#[test]
fn collaboration_state_invariant_violation_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0]
        .lets_a_replica_overwrite_local_buffer_vfs_or_git_truth_implicitly = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated));

    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[1]
        .discards_unsent_local_edits_on_permission_downgrade_relay_failure_or_leave = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated));

    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[2]
        .rebinds_comments_annotations_or_review_pins_without_drift_history = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated));

    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[4]
        .collapses_convergence_or_awareness_degraded_state_into_a_generic_stale_badge = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated));

    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[5]
        .exports_op_logs_snapshots_or_archives_without_policy_labeled_redaction_and_lineage = true;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::CollaborationStateInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    let row = packet
        .collaboration_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5CollaborationStateObject::SealedSessionArchive)
        .expect("sealed-session-archive row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[1]
        .classification_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet
        .governance_review
        .converged_state_is_mechanically_distinct_from_degraded = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_collaboration_state_source = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_collaboration_state_matrix().render_markdown_summary();
    for class in M5CollaborationStateObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_collaboration_state_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5CollaborationStateObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,convergence_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5CollaborationStateObject::ALL {
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
        serde_json::from_str(&seeded_m5_collaboration_state_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5CollaborationStateObject::ALL {
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
        "/../../dashboards/m5-collaboration-convergence-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked collaboration-convergence-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_collaboration_state_matrix_export()
        .expect("checked M5 collaboration-state matrix export validates");
    assert_eq!(packet.packet_id, M5_COLLABORATION_STATE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_collaboration_state_matrix_export()
        .expect("checked M5 collaboration-state matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_collaboration_state_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed(),
        seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.collaboration_state_rows.len(),
            M5CollaborationStateObject::ALL.len()
        );
    }

    let beta = seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed();
    let row = beta
        .collaboration_state_rows
        .iter()
        .find(|r| r.object_class == M5CollaborationStateObject::HigherRiskControlPlane)
        .expect("higher-risk-control-plane row present");
    assert_eq!(
        row.qualification,
        M5CollaborationStateQualificationClass::Beta
    );

    let preview = seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed();
    let row = preview
        .collaboration_state_rows
        .iter()
        .find(|r| r.object_class == M5CollaborationStateObject::SealedSessionArchive)
        .expect("sealed-session-archive row present");
    assert_eq!(
        row.qualification,
        M5CollaborationStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5CollaborationStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-convergence/higher_risk_control_plane_beta_narrowed.json"
    )))
    .expect("higher-risk-control-plane fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_collaboration_state_matrix_higher_risk_control_plane_beta_narrowed()
    );

    let preview: M5CollaborationStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/collaboration/m5-convergence/sealed_session_archive_preview_narrowed.json"
    )))
    .expect("sealed-session-archive fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_collaboration_state_matrix_sealed_session_archive_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_collaboration_state_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_collaboration_state_matrix();
    packet.collaboration_state_rows[0].scope_summary =
        "raw endpoint https://relay.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5CollaborationStateMatrixViolation::RawMaterialInExport));
}
