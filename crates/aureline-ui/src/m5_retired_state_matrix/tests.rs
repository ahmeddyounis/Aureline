use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_retired_state_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RETIRED_STATE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_object_class() {
    let packet = seeded_m5_retired_state_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .retired_state_rows
        .iter()
        .map(|r| r.object_class)
        .collect();
    for class in M5RetiredStateObject::ALL {
        assert!(
            present.contains(&class),
            "missing object class {}",
            class.as_str()
        );
    }
    assert_eq!(
        packet.retired_state_rows.len(),
        M5RetiredStateObject::ALL.len()
    );
}

#[test]
fn frozen_retirement_role_vocabulary_is_exact() {
    let tokens: Vec<&str> = M5RetiredStateRole::ALL.iter().map(|r| r.as_str()).collect();
    assert_eq!(
        tokens,
        vec![
            "last_supported_pin",
            "successor_routing",
            "disable_path",
            "export_rollback_route",
            "archival_note",
            "migration_outcome",
            "support_note_closure",
        ]
    );
    assert!(M5RetiredStateRole::LastSupportedPin.must_be_closed_before_flipping_to_retired());
    assert!(M5RetiredStateRole::SuccessorRouting.must_be_closed_before_flipping_to_retired());
    assert!(M5RetiredStateRole::DisablePath.must_be_closed_before_flipping_to_retired());
    assert!(!M5RetiredStateRole::ExportRollbackRoute.must_be_closed_before_flipping_to_retired());
    assert!(!M5RetiredStateRole::ArchivalNote.must_be_closed_before_flipping_to_retired());
    assert!(!M5RetiredStateRole::MigrationOutcome.must_be_closed_before_flipping_to_retired());
    assert!(M5RetiredStateRole::SupportNoteClosure.must_be_closed_before_flipping_to_retired());
}

#[test]
fn retired_is_mechanically_distinct_from_other_lifecycle_states() {
    let tokens: Vec<&str> = M5RetiredStateLifecycleState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "deprecated",
            "disabled_by_policy",
            "stable_line_narrowed",
            "retired"
        ]
    );
    assert!(M5RetiredStateLifecycleState::Retired.is_retired());
    assert!(!M5RetiredStateLifecycleState::Deprecated.is_retired());
    assert!(!M5RetiredStateLifecycleState::DisabledByPolicy.is_retired());
    assert!(!M5RetiredStateLifecycleState::StableLineNarrowed.is_retired());
}

#[test]
fn every_class_declares_mandatory_labels_schema_and_horizon_stages() {
    let packet = seeded_m5_retired_state_matrix();
    for row in &packet.retired_state_rows {
        for label in M5RetiredStateRequiredLabel::MANDATORY {
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
        assert!(!row.removal_horizon_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5RetiredStateAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn every_class_declares_complete_transition_metadata() {
    let packet = seeded_m5_retired_state_matrix();
    for row in &packet.retired_state_rows {
        let tr = &row.retirement_transition;
        for field in [
            &tr.last_supported_version_or_channel,
            &tr.cutoff_date,
            &tr.successor_path,
            &tr.disable_path,
            &tr.export_rollback_route,
            &tr.archival_note,
            &tr.migration_outcome,
            &tr.support_note_closure_state,
        ] {
            assert!(
                !field.trim().is_empty(),
                "transition field empty on {}",
                row.object_class.as_str()
            );
        }
    }
}

#[test]
fn class_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_retired_state_matrix();
    for row in &packet.retired_state_rows {
        let class = row.object_class;
        assert_eq!(
            !row.supported_line_roles.is_empty(),
            class.declares_supported_line_roles(),
            "supported_line_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.stable_capability_roles.is_empty(),
            class.declares_stable_capability_roles(),
            "stable_capability_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.bundle_roles.is_empty(),
            class.declares_bundle_roles(),
            "bundle_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.command_deep_link_roles.is_empty(),
            class.declares_command_deep_link_roles(),
            "command_deep_link_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.schema_bearing_surface_roles.is_empty(),
            class.declares_schema_bearing_surface_roles(),
            "schema_bearing_surface_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.registry_visible_package_roles.is_empty(),
            class.declares_registry_visible_package_roles(),
            "registry_visible_package_roles presence wrong for {}",
            class.as_str()
        );
        assert_eq!(
            !row.managed_tenant_feature_roles.is_empty(),
            class.declares_managed_tenant_feature_roles(),
            "managed_tenant_feature_roles presence wrong for {}",
            class.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_class() {
    let packet = seeded_m5_retired_state_matrix();
    for role in M5RetiredStateRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no class declares retirement role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateSupportedLineRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.supported_line_roles.contains(&role)),
            "no class declares supported_line_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateStableCapabilityRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.stable_capability_roles.contains(&role)),
            "no class declares stable_capability_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateBundleRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.bundle_roles.contains(&role)),
            "no class declares bundle_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateCommandDeepLinkRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.command_deep_link_roles.contains(&role)),
            "no class declares command_deep_link_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateSchemaBearingSurfaceRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.schema_bearing_surface_roles.contains(&role)),
            "no class declares schema_bearing_surface_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateRegistryVisiblePackageRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.registry_visible_package_roles.contains(&role)),
            "no class declares registry_visible_package_role {}",
            role.as_str()
        );
    }
    for role in M5RetiredStateManagedTenantFeatureRole::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.managed_tenant_feature_roles.contains(&role)),
            "no class declares managed_tenant_feature_role {}",
            role.as_str()
        );
    }
    for reason in M5RetiredStateDegradedReason::ALL {
        assert!(
            packet
                .retired_state_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no class declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_object_class_fails_validation() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet
        .retired_state_rows
        .retain(|row| row.object_class != M5RetiredStateObject::ManagedTenantFeature);
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RequiredObjectMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0]
        .required_labels
        .retain(|label| *label != M5RetiredStateRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let own = M5RetiredStateObject::Bundle.canonical_domain_schema_ref();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::Bundle)
        .expect("bundle row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::SemanticRoleMissing));
}

#[test]
fn supported_line_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::SupportedLine)
        .expect("SupportedLine row present");
    row.supported_line_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::SupportedLineRoleMissing));
}

#[test]
fn stable_capability_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::StableCapability)
        .expect("StableCapability row present");
    row.stable_capability_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::StableCapabilityRoleMissing));
}

#[test]
fn bundle_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::Bundle)
        .expect("Bundle row present");
    row.bundle_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::BundleRoleMissing));
}

#[test]
fn command_deep_link_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::CommandDeepLink)
        .expect("CommandDeepLink row present");
    row.command_deep_link_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::CommandDeepLinkRoleMissing));
}

#[test]
fn schema_bearing_surface_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::SchemaBearingSurface)
        .expect("SchemaBearingSurface row present");
    row.schema_bearing_surface_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::SchemaBearingSurfaceRoleMissing));
}

#[test]
fn registry_visible_package_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::RegistryVisiblePackage)
        .expect("RegistryVisiblePackage row present");
    row.registry_visible_package_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RegistryVisiblePackageRoleMissing));
}

#[test]
fn managed_tenant_feature_roles_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::ManagedTenantFeature)
        .expect("ManagedTenantFeature row present");
    row.managed_tenant_feature_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::ManagedTenantFeatureRoleMissing));
}

#[test]
fn transition_metadata_incomplete_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0]
        .retirement_transition
        .successor_path
        .clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::TransitionMetadataIncomplete));
}

#[test]
fn backup_owner_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0].backup_owner_role.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateRowIncomplete));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[4].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::DegradedReasonMissing));
}

#[test]
fn retired_state_invariant_violation_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0]
        .lets_a_retired_surface_disappear_without_tombstone_archival_route_or_successor_pointer =
        true;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateInvariantViolated));

    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[1]
        .keeps_a_retired_class_selectable_in_new_install_new_tenant_marketplace_or_upgrade_flow =
        true;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateInvariantViolated));

    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[2]
        .destroys_last_supported_docs_schemas_or_evidence_before_support_note_closure = true;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateInvariantViolated));

    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[3].leaves_retirement_state_unjoined_to_build_line_identity_deployment_profile_and_migration_outcome = true;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateInvariantViolated));

    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[4].retires_a_surface_through_silent_disappearance_stale_selection_ui_or_orphaned_support_truth = true;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RetiredStateInvariantViolated));
}

#[test]
fn stable_class_missing_closure_artifact_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    let row = packet
        .retired_state_rows
        .iter_mut()
        .find(|row| row.object_class == M5RetiredStateObject::Bundle)
        .expect("bundle row present");
    row.required_closure_artifact_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::StableObjectMissingClosureArtifact));
}

#[test]
fn missing_horizon_stages_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[1].removal_horizon_stages.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RemovalHorizonStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet
        .governance_review
        .retired_is_mechanically_distinct_from_deprecated_disabled_and_narrowed = false;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_retirement_source = false;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_class() {
    let summary = seeded_m5_retired_state_matrix().render_markdown_summary();
    for class in M5RetiredStateObject::ALL {
        assert!(
            summary.contains(class.as_str()),
            "summary missing class {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_class() {
    let csv = seeded_m5_retired_state_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RetiredStateObject::ALL.len());
    assert!(lines[0].starts_with(
        "object_class,qualification,lifecycle_state,owner,backup_owner,canonical_schema,"
    ));
    for class in M5RetiredStateObject::ALL {
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
        serde_json::from_str(&seeded_m5_retired_state_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for class in M5RetiredStateObject::ALL {
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
        "/../../dashboards/m5-retired-surface-health.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked retired-surface-health dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_retired_state_matrix_export()
        .expect("checked M5 retired-state matrix export validates");
    assert_eq!(packet.packet_id, M5_RETIRED_STATE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_retired_state_matrix_export()
        .expect("checked M5 retired-state matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_retired_state_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_classes_visible() {
    for packet in [
        seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed(),
        seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.retired_state_rows.len(),
            M5RetiredStateObject::ALL.len()
        );
    }

    let beta = seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed();
    let row = beta
        .retired_state_rows
        .iter()
        .find(|r| r.object_class == M5RetiredStateObject::RegistryVisiblePackage)
        .expect("registry-visible-package row present");
    assert_eq!(row.qualification, M5RetiredStateQualificationClass::Beta);

    let preview = seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed();
    let row = preview
        .retired_state_rows
        .iter()
        .find(|r| r.object_class == M5RetiredStateObject::ManagedTenantFeature)
        .expect("managed-tenant-feature row present");
    assert_eq!(row.qualification, M5RetiredStateQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5RetiredStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-retired-state/registry_visible_package_beta_narrowed.json"
    )))
    .expect("registry-visible-package fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_retired_state_matrix_registry_visible_package_beta_narrowed()
    );

    let preview: M5RetiredStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-retired-state/managed_tenant_feature_preview_narrowed.json"
    )))
    .expect("managed-tenant-feature fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_retired_state_matrix_managed_tenant_feature_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_retired_state_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_retired_state_matrix();
    packet.retired_state_rows[0].scope_summary =
        "raw endpoint https://line.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RetiredStateMatrixViolation::RawMaterialInExport));
}
