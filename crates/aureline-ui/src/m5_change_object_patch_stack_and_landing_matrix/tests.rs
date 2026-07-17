use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_change_orchestration_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CHANGE_ORCHESTRATION_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_change_orchestration_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .change_orchestration_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5ChangeOrchestrationObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.change_orchestration_rows.len(),
        M5ChangeOrchestrationObject::ALL.len()
    );
}

#[test]
fn frozen_change_orchestration_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5ChangeOrchestrationRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "selected_change_object_disclosure",
            "worktree_binding_disclosure",
            "stack_membership_disclosure",
            "landing_state_disclosure",
            "validation_freshness_disclosure",
            "rollback_export_fallback_disclosure",
            "cleanup_safety_disclosure",
        ]
    );
    assert!(M5ChangeOrchestrationRole::SelectedChangeObjectDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(M5ChangeOrchestrationRole::WorktreeBindingDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(M5ChangeOrchestrationRole::StackMembershipDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(M5ChangeOrchestrationRole::LandingStateDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(!M5ChangeOrchestrationRole::ValidationFreshnessDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(!M5ChangeOrchestrationRole::RollbackExportFallbackDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
    assert!(!M5ChangeOrchestrationRole::CleanupSafetyDisclosure
        .must_be_present_before_surfacing_as_a_change_orchestration_result());
}

#[test]
fn queue_eligible_is_mechanically_distinct_from_selected_change() {
    let tokens: Vec<&str> = M5ChangeOrchestrationState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "selected_change",
            "stale_validation",
            "restack_required",
            "queue_eligible",
            "queue_blocked",
            "protected_branch_blocked",
            "orphaned",
            "abandoned",
            "exported",
            "imported_reopened",
        ]
    );
    assert!(M5ChangeOrchestrationState::QueueEligible.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::SelectedChange.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::StaleValidation.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::RestackRequired.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::QueueBlocked.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::ProtectedBranchBlocked.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::Orphaned.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::Abandoned.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::Exported.is_queue_eligible());
    assert!(!M5ChangeOrchestrationState::ImportedReopened.is_queue_eligible());
}

#[test]
fn stack_membership_source_keeps_the_four_kinds_distinct() {
    let tokens: Vec<&str> = M5ChangeOrchestrationStackMembershipSource::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "declared_in_change_object",
            "declared_locally",
            "inferred_from_branch_name",
            "stale_or_broken_membership",
        ]
    );
    assert!(
        M5ChangeOrchestrationStackMembershipSource::DeclaredInChangeObject
            .is_explicitly_declared_membership()
    );
    assert!(!M5ChangeOrchestrationStackMembershipSource::DeclaredLocally
        .is_explicitly_declared_membership());
    assert!(
        !M5ChangeOrchestrationStackMembershipSource::InferredFromBranchName
            .is_explicitly_declared_membership()
    );
    assert!(
        !M5ChangeOrchestrationStackMembershipSource::StaleOrBrokenMembership
            .is_explicitly_declared_membership()
    );
}

#[test]
fn cleanup_safety_names_blocked_and_escalated_states() {
    let tokens: Vec<&str> = M5ChangeOrchestrationCleanupSafety::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "clear_to_land",
            "blocked_by_stale_validation",
            "blocked_by_restack_required",
            "blocked_by_queue_dependency",
            "blocked_by_protected_branch",
        ]
    );
    assert!(!M5ChangeOrchestrationCleanupSafety::ClearToLand.is_blocked_from_landing());
    assert!(M5ChangeOrchestrationCleanupSafety::BlockedByStaleValidation.is_blocked_from_landing());
    assert!(M5ChangeOrchestrationCleanupSafety::BlockedByRestackRequired.is_blocked_from_landing());
    assert!(M5ChangeOrchestrationCleanupSafety::BlockedByQueueDependency.is_blocked_from_landing());
    assert!(M5ChangeOrchestrationCleanupSafety::BlockedByProtectedBranch.is_blocked_from_landing());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_stages() {
    let packet = seeded_m5_change_orchestration_matrix();
    for row in &packet.change_orchestration_rows {
        for label in M5ChangeOrchestrationRequiredLabel::MANDATORY {
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
            .contains(&M5ChangeOrchestrationAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_change_orchestration_matrix();
    for row in &packet.change_orchestration_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.surface_label,
            &tr.selected_change_object,
            &tr.worktree_base_identity,
            &tr.stack_membership_and_order,
            &tr.landing_state_summary,
            &tr.cleanup_safety,
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
    let packet = seeded_m5_change_orchestration_matrix();
    for row in &packet.change_orchestration_rows {
        let class = row.object_class;
        assert_eq!(
            !row.change_object_roles.is_empty(),
            class.declares_change_object_roles(),
            "change_object_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.patch_stack_queue_roles.is_empty(),
            class.declares_patch_stack_queue_roles(),
            "patch_stack_queue_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.stack_edit_review_roles.is_empty(),
            class.declares_stack_edit_review_roles(),
            "stack_edit_review_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.landing_candidate_roles.is_empty(),
            class.declares_landing_candidate_roles(),
            "landing_candidate_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.portable_shelf_roles.is_empty(),
            class.declares_portable_shelf_roles(),
            "portable_shelf_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.worktree_cleanup_roles.is_empty(),
            class.declares_worktree_cleanup_roles(),
            "worktree_cleanup_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_change_orchestration_matrix();
    for role in M5ChangeOrchestrationRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares change-orchestration role {}",
            role.as_str()
        );
    }
    for role in M5ChangeObjectRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.change_object_roles.contains(&role)),
            "no class declares change_object_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeOrchestrationPatchStackQueueRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.patch_stack_queue_roles.contains(&role)),
            "no class declares patch_stack_queue_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeOrchestrationStackEditReviewRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.stack_edit_review_roles.contains(&role)),
            "no class declares stack_edit_review_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeOrchestrationLandingCandidateRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.landing_candidate_roles.contains(&role)),
            "no class declares landing_candidate_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeOrchestrationPortableShelfRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.portable_shelf_roles.contains(&role)),
            "no class declares portable_shelf_role {}",
            role.as_str()
        );
    }
    for role in M5ChangeOrchestrationWorktreeCleanupRole::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.worktree_cleanup_roles.contains(&role)),
            "no class declares worktree_cleanup_role {}",
            role.as_str()
        );
    }
    for reason in M5ChangeOrchestrationDegradedReason::ALL {
        assert!(
            packet
                .change_orchestration_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet
        .change_orchestration_rows
        .retain(|row| row.object_class != M5ChangeOrchestrationObject::StackEditReviewSheet);
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0]
        .required_labels
        .retain(|label| *label != M5ChangeOrchestrationRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let own = M5ChangeOrchestrationObject::PatchStackQueue.canonical_domain_schema_ref();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::PatchStackQueue)
        .expect("start-work-sheet row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::SemanticRoleMissing));
}

#[test]
fn change_object_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::ChangeObject)
        .expect("ChangeObject row present");
    row.change_object_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeObjectRoleMissing));
}

#[test]
fn patch_stack_queue_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::PatchStackQueue)
        .expect("PatchStackQueue row present");
    row.patch_stack_queue_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::PatchStackQueueRoleMissing));
}

#[test]
fn stack_edit_review_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::StackEditReviewSheet)
        .expect("StackEditReviewSheet row present");
    row.stack_edit_review_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::StackEditReviewRoleMissing));
}

#[test]
fn landing_candidate_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::LandingCandidateSheet)
        .expect("LandingCandidateSheet row present");
    row.landing_candidate_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::LandingCandidateRoleMissing));
}

#[test]
fn portable_shelf_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::PortableShelf)
        .expect("PortableShelf row present");
    row.portable_shelf_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::PortableShelfRoleMissing));
}

#[test]
fn worktree_cleanup_roles_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::WorktreeCleanupPreview)
        .expect("WorktreeCleanupPreview row present");
    row.worktree_cleanup_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::WorktreeCleanupRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0]
        .required_visible_state
        .surface_label
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0]
        .backup_owner_role
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::DegradedReasonMissing));
}

#[test]
fn change_orchestration_invariant_violation_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0].infers_stack_membership_from_branch_names_alone = true;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated));

    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[1]
        .mutates_files_in_another_worktree_without_an_explicit_selected_change_object_and_worktree_binding =
        true;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated));

    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[2].silently_reorders_collapses_or_retargets_stack_members =
        true;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated));

    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[3]
        .lands_from_ambient_branch_state_without_a_reviewed_landing_candidate = true;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated));

    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[4]
        .deletes_orphaned_worktrees_or_stale_stack_members_without_previewing_running_work_and_export_safe_evidence = true;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ChangeOrchestrationInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    let row = packet
        .change_orchestration_rows
        .iter_mut()
        .find(|row| row.object_class == M5ChangeOrchestrationObject::PatchStackQueue)
        .expect("start-work-sheet row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_classification_stages_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[1]
        .classification_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet
        .governance_review
        .queue_eligible_state_is_mechanically_distinct_from_selected_change = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_change_orchestration_source = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_change_orchestration_matrix().render_markdown_summary();
    for class in M5ChangeOrchestrationObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_change_orchestration_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ChangeOrchestrationObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,landing_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5ChangeOrchestrationObject::ALL {
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
        serde_json::from_str(&seeded_m5_change_orchestration_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5ChangeOrchestrationObject::ALL {
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
        "/../../dashboards/m5-change-orchestration-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked change-orchestration-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_change_orchestration_matrix_export()
        .expect("checked M5 change-orchestration matrix export validates");
    assert_eq!(packet.packet_id, M5_CHANGE_ORCHESTRATION_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_change_orchestration_matrix_export()
        .expect("checked M5 change-orchestration matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_change_orchestration_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed(),
        seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.change_orchestration_rows.len(),
            M5ChangeOrchestrationObject::ALL.len()
        );
    }

    let beta = seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed();
    let row = beta
        .change_orchestration_rows
        .iter()
        .find(|r| r.object_class == M5ChangeOrchestrationObject::PatchStackQueue)
        .expect("start-work-sheet row present");
    assert_eq!(
        row.qualification,
        M5ChangeOrchestrationQualificationClass::Beta
    );

    let preview = seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed();
    let row = preview
        .change_orchestration_rows
        .iter()
        .find(|r| r.object_class == M5ChangeOrchestrationObject::WorktreeCleanupPreview)
        .expect("blocked-escalate-card row present");
    assert_eq!(
        row.qualification,
        M5ChangeOrchestrationQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ChangeOrchestrationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m5-change-orchestration/patch_stack_queue_beta_narrowed.json"
    )))
    .expect("start-work-sheet fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_change_orchestration_matrix_patch_stack_queue_beta_narrowed()
    );

    let preview: M5ChangeOrchestrationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/git/m5-change-orchestration/worktree_cleanup_preview_preview_narrowed.json"
    )))
    .expect("blocked-escalate-card fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_change_orchestration_matrix_worktree_cleanup_preview_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_change_orchestration_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_change_orchestration_matrix();
    packet.change_orchestration_rows[0].scope_summary =
        "raw endpoint https://tracker.example/item leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ChangeOrchestrationMatrixViolation::RawMaterialInExport));
}
