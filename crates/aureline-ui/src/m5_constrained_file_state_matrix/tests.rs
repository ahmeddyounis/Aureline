use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_constrained_file_state_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CONSTRAINED_FILE_STATE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_constrained_file_state_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .constrained_file_state_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5ConstrainedFileStateObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.constrained_file_state_rows.len(),
        M5ConstrainedFileStateObject::ALL.len()
    );
}

#[test]
fn frozen_constrained_file_state_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5ConstrainedFileStateRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "state_badge_classification",
            "blocked_write_reason",
            "canonical_source_relation",
            "exact_write_target",
            "allowed_blocked_action_set",
            "safe_next_step_guidance",
            "export_retain_disclosure",
        ]
    );
    assert!(M5ConstrainedFileStateRole::StateBadgeClassification
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(M5ConstrainedFileStateRole::BlockedWriteReason
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(M5ConstrainedFileStateRole::CanonicalSourceRelation
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(M5ConstrainedFileStateRole::ExactWriteTarget
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(!M5ConstrainedFileStateRole::AllowedBlockedActionSet
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(!M5ConstrainedFileStateRole::SafeNextStepGuidance
        .must_be_present_before_surfacing_as_constrained_object());
    assert!(!M5ConstrainedFileStateRole::ExportRetainDisclosure
        .must_be_present_before_surfacing_as_constrained_object());
}

#[test]
fn write_constrained_state_is_mechanically_distinct_from_directly_writable() {
    let tokens: Vec<&str> = M5ConstrainedFileStateWriteDisposition::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "directly_writable",
            "read_only_blocked",
            "regenerate_only",
            "approval_gated",
            "detach_required",
            "restore_only",
        ]
    );
    assert!(!M5ConstrainedFileStateWriteDisposition::DirectlyWritable.is_write_constrained());
    for disposition in [
        M5ConstrainedFileStateWriteDisposition::ReadOnlyBlocked,
        M5ConstrainedFileStateWriteDisposition::RegenerateOnly,
        M5ConstrainedFileStateWriteDisposition::ApprovalGated,
        M5ConstrainedFileStateWriteDisposition::DetachRequired,
        M5ConstrainedFileStateWriteDisposition::RestoreOnly,
    ] {
        assert!(disposition.is_write_constrained());
    }
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_horizon_stages() {
    let packet = seeded_m5_constrained_file_state_matrix();
    for row in &packet.constrained_file_state_rows {
        for label in M5ConstrainedFileStateRequiredLabel::MANDATORY {
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
            .contains(&M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_visible_state() {
    let packet = seeded_m5_constrained_file_state_matrix();
    for row in &packet.constrained_file_state_rows {
        let tr = &row.required_visible_state;
        for field in [
            &tr.state_badge,
            &tr.reason,
            &tr.canonical_source_or_live_target,
            &tr.exact_write_target,
            &tr.allowed_actions,
            &tr.blocked_actions,
            &tr.export_retain_notes,
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
    let packet = seeded_m5_constrained_file_state_matrix();
    for row in &packet.constrained_file_state_rows {
        let class = row.object_class;
        assert_eq!(
            !row.read_only_roles.is_empty(),
            class.declares_read_only_roles(),
            "read_only_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.generated_roles.is_empty(),
            class.declares_generated_roles(),
            "generated_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.policy_locked_roles.is_empty(),
            class.declares_policy_locked_roles(),
            "policy_locked_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.managed_roles.is_empty(),
            class.declares_managed_roles(),
            "managed_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.projection_roles.is_empty(),
            class.declares_projection_roles(),
            "projection_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.captured_snapshot_roles.is_empty(),
            class.declares_captured_snapshot_roles(),
            "captured_snapshot_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_constrained_file_state_matrix();
    for role in M5ConstrainedFileStateRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares constrained-file-state role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStateReadOnlyRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.read_only_roles.contains(&role)),
            "no class declares read_only_role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStateGeneratedRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.generated_roles.contains(&role)),
            "no class declares generated_role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStatePolicyLockedRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.policy_locked_roles.contains(&role)),
            "no class declares policy_locked_role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStateManagedRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.managed_roles.contains(&role)),
            "no class declares managed_role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStateProjectionRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.projection_roles.contains(&role)),
            "no class declares projection_role {}",
            role.as_str()
        );
    }
    for role in M5ConstrainedFileStateCapturedSnapshotRole::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.captured_snapshot_roles.contains(&role)),
            "no class declares captured_snapshot_role {}",
            role.as_str()
        );
    }
    for reason in M5ConstrainedFileStateDegradedReason::ALL {
        assert!(
            packet
                .constrained_file_state_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet
        .constrained_file_state_rows
        .retain(|row| row.object_class != M5ConstrainedFileStateObject::Projection);
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0]
        .required_labels
        .retain(|label| *label != M5ConstrainedFileStateRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let own = M5ConstrainedFileStateObject::Generated.canonical_domain_schema_ref();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Generated)
        .expect("support-export-evidence row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::SemanticRoleMissing));
}

#[test]
fn read_only_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::ReadOnly)
        .expect("ReadOnly row present");
    row.read_only_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ReadOnlyRoleMissing));
}

#[test]
fn generated_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Generated)
        .expect("Generated row present");
    row.generated_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::GeneratedRoleMissing));
}

#[test]
fn policy_locked_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::PolicyLocked)
        .expect("PolicyLocked row present");
    row.policy_locked_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::PolicyLockedRoleMissing));
}

#[test]
fn managed_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Managed)
        .expect("Managed row present");
    row.managed_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ManagedRoleMissing));
}

#[test]
fn projection_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Projection)
        .expect("Projection row present");
    row.projection_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ProjectionRoleMissing));
}

#[test]
fn captured_snapshot_roles_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::CapturedSnapshot)
        .expect("CapturedSnapshot row present");
    row.captured_snapshot_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::CapturedSnapshotRoleMissing));
}

#[test]
fn visible_state_incomplete_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0]
        .required_visible_state
        .state_badge
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::VisibleStateIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0]
        .backup_owner_role
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[4]
        .degraded_reasons
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::DegradedReasonMissing));
}

#[test]
fn constrained_file_state_invariant_violation_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0]
        .lets_one_constrained_state_class_hide_another_when_both_materially_affect_behavior = true;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated));

    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[1].lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write = true;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated));

    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[2].gives_ai_automation_import_or_repair_flows_a_hidden_bypass_around_constrained_state_rules = true;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated));

    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[3].leaves_canonical_source_exact_write_target_preserved_versus_lost_sync_or_recovery_path_unstated = true;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated));

    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[4].presents_a_constrained_object_as_directly_writable_or_hides_the_recovery_or_regenerate_path = true;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConstrainedFileStateInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    let row = packet
        .constrained_file_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5ConstrainedFileStateObject::Generated)
        .expect("support-export-evidence row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_horizon_stages_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[1]
        .classification_stages
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ClassificationStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet
        .governance_review
        .write_constrained_state_is_mechanically_distinct_from_directly_writable_state = false;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_constrained_file_state_source = false;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_constrained_file_state_matrix().render_markdown_summary();
    for class in M5ConstrainedFileStateObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_constrained_file_state_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ConstrainedFileStateObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,write_disposition,owner,backup_owner,canonical_schema,"
    ));
    for class in M5ConstrainedFileStateObject::ALL {
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
        serde_json::from_str(&seeded_m5_constrained_file_state_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5ConstrainedFileStateObject::ALL {
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
        "/../../dashboards/m5-constrained-object-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked constrained-object-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_constrained_file_state_matrix_export()
        .expect("checked M5 constrained-file-state matrix export validates");
    assert_eq!(packet.packet_id, M5_CONSTRAINED_FILE_STATE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_constrained_file_state_matrix_export()
        .expect("checked M5 constrained-file-state matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_constrained_file_state_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_constrained_file_state_matrix_managed_beta_narrowed(),
        seeded_m5_constrained_file_state_matrix_projection_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.constrained_file_state_rows.len(),
            M5ConstrainedFileStateObject::ALL.len()
        );
    }

    let beta = seeded_m5_constrained_file_state_matrix_managed_beta_narrowed();
    let row = beta
        .constrained_file_state_rows
        .iter()
        .find(|r| r.object_class == M5ConstrainedFileStateObject::Managed)
        .expect("managed row present");
    assert_eq!(
        row.qualification,
        M5ConstrainedFileStateQualificationClass::Beta
    );

    let preview = seeded_m5_constrained_file_state_matrix_projection_preview_narrowed();
    let row = preview
        .constrained_file_state_rows
        .iter()
        .find(|r| r.object_class == M5ConstrainedFileStateObject::Projection)
        .expect("projection row present");
    assert_eq!(
        row.qualification,
        M5ConstrainedFileStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5ConstrainedFileStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-object-states/managed_beta_narrowed.json"
    )))
    .expect("managed fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_constrained_file_state_matrix_managed_beta_narrowed()
    );

    let preview: M5ConstrainedFileStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-object-states/projection_preview_narrowed.json"
    )))
    .expect("projection fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_constrained_file_state_matrix_projection_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_constrained_file_state_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_constrained_file_state_matrix();
    packet.constrained_file_state_rows[0].scope_summary =
        "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5ConstrainedFileStateMatrixViolation::RawMaterialInExport));
}
