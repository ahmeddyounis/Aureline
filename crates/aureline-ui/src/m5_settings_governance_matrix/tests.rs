use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_settings_governance_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SETTINGS_GOVERNANCE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_settings_governance_family() {
    let packet = seeded_m5_settings_governance_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .settings_governance_rows
        .iter()
        .map(|r| r.settings_governance_family)
        .collect();
    for family in M5SettingsGovernanceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing settings-governance family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.settings_governance_rows.len(),
        M5SettingsGovernanceFamily::ALL.len()
    );
}

#[test]
fn frozen_settings_governance_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: setting_definition / effective_resolution / write_intent /
    // policy_constraint / sync_conflict / schema_migration / capability_lifecycle stays in one controlled
    // token set that no settings, shell, diagnostics, admin, docs, or support surface reinvents.
    let tokens: Vec<&str> = M5SettingsGovernanceRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "setting_definition",
            "effective_resolution",
            "write_intent",
            "policy_constraint",
            "sync_conflict",
            "schema_migration",
            "capability_lifecycle",
        ]
    );
    assert!(M5SettingsGovernanceRole::WriteIntent
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(M5SettingsGovernanceRole::PolicyConstraint
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(M5SettingsGovernanceRole::SyncConflict
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(M5SettingsGovernanceRole::CapabilityLifecycle
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(!M5SettingsGovernanceRole::SettingDefinition
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(!M5SettingsGovernanceRole::EffectiveResolution
        .must_preserve_evidence_and_disclose_cause_before_applying());
    assert!(!M5SettingsGovernanceRole::SchemaMigration
        .must_preserve_evidence_and_disclose_cause_before_applying());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_settings_governance_matrix();
    for row in &packet.settings_governance_rows {
        for label in M5SettingsGovernanceRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.settings_governance_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.settings_governance_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.settings_governance_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5SettingsGovernanceAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_settings_governance_matrix();
    for row in &packet.settings_governance_rows {
        let family = row.settings_governance_family;
        assert_eq!(
            !row.resolve_setting_roles.is_empty(),
            family.declares_resolve_setting_roles(),
            "resolve_setting_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.write_setting_roles.is_empty(),
            family.declares_write_setting_roles(),
            "write_setting_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sync_scope_roles.is_empty(),
            family.declares_sync_scope_roles(),
            "sync_scope_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.migrate_schema_roles.is_empty(),
            family.declares_migrate_schema_roles(),
            "migrate_schema_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.rollout_capability_roles.is_empty(),
            family.declares_rollout_capability_roles(),
            "rollout_capability_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_settings_governance_matrix();
    for role in M5SettingsGovernanceRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares settings-governance role {}",
            role.as_str()
        );
    }
    for role in M5ResolveSettingRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.resolve_setting_roles.contains(&role)),
            "no family declares resolve-setting role {}",
            role.as_str()
        );
    }
    for role in M5WriteSettingRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.write_setting_roles.contains(&role)),
            "no family declares write-setting role {}",
            role.as_str()
        );
    }
    for role in M5SyncScopeRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.sync_scope_roles.contains(&role)),
            "no family declares sync-scope role {}",
            role.as_str()
        );
    }
    for role in M5MigrateSchemaRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.migrate_schema_roles.contains(&role)),
            "no family declares migrate-schema role {}",
            role.as_str()
        );
    }
    for role in M5RolloutCapabilityRole::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.rollout_capability_roles.contains(&role)),
            "no family declares rollout-capability role {}",
            role.as_str()
        );
    }
    for reason in M5SettingsGovernanceDegradedReason::ALL {
        assert!(
            packet
                .settings_governance_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_settings_governance_family_fails_validation() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows.retain(|row| {
        row.settings_governance_family != M5SettingsGovernanceFamily::RolloutCapability
    });
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[0]
        .required_labels
        .retain(|label| *label != M5SettingsGovernanceRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let own = M5SettingsGovernanceFamily::WriteSetting.canonical_domain_schema_ref();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::WriteSetting)
        .expect("write-setting row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SemanticRoleMissing));
}

#[test]
fn resolve_setting_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::ResolveSetting)
        .expect("resolve-setting present");
    row.resolve_setting_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::ResolveSettingRoleMissing));
}

#[test]
fn write_setting_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::WriteSetting)
        .expect("write-setting present");
    row.write_setting_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::WriteSettingRoleMissing));
}

#[test]
fn sync_scope_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::SyncScope)
        .expect("sync-scope present");
    row.sync_scope_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SyncScopeRoleMissing));
}

#[test]
fn migrate_schema_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::MigrateSchema)
        .expect("migrate-schema present");
    row.migrate_schema_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::MigrateSchemaRoleMissing));
}

#[test]
fn rollout_capability_role_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::RolloutCapability)
        .expect("rollout-capability present");
    row.rollout_capability_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::RolloutCapabilityRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::DegradedReasonMissing));
}

#[test]
fn settings_governance_invariant_violation_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[0].recycles_a_retired_setting_id = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated));

    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[1].rewrites_a_scoped_write_into_a_broader_scope = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated));

    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[2]
        .silently_overwrites_locked_or_machine_only_state_during_sync = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated));

    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[4]
        .hides_lifecycle_or_experiment_dependency_behind_unpublished_markers = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated));

    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[4]
        .hides_kill_switch_or_policy_disable_cause_behind_generic_unavailable_copy = true;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::SettingsGovernanceInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    let row = packet
        .settings_governance_rows
        .iter_mut()
        .find(|row| row.settings_governance_family == M5SettingsGovernanceFamily::WriteSetting)
        .expect("write-setting row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet
        .governance_review
        .setting_definition_and_effective_resolution_stay_separately_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_settings_governance_source = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_settings_governance_family() {
    let summary = seeded_m5_settings_governance_matrix().render_markdown_summary();
    for family in M5SettingsGovernanceFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_settings_governance_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SettingsGovernanceFamily::ALL.len());
    assert!(
        lines[0].starts_with("settings_governance_family,qualification,owner,canonical_schema,")
    );
    for family in M5SettingsGovernanceFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing family {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_settings_governance_matrix_export()
        .expect("checked M5 settings-governance matrix export validates");
    assert_eq!(packet.packet_id, M5_SETTINGS_GOVERNANCE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_settings_governance_matrix_export()
        .expect("checked M5 settings-governance matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_settings_governance_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed(),
        seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.settings_governance_rows.len(),
            M5SettingsGovernanceFamily::ALL.len()
        );
    }

    let sync_scope = seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed();
    let row = sync_scope
        .settings_governance_rows
        .iter()
        .find(|r| r.settings_governance_family == M5SettingsGovernanceFamily::SyncScope)
        .expect("sync-scope row present");
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Beta
    );

    let rollout = seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed();
    let row = rollout
        .settings_governance_rows
        .iter()
        .find(|r| r.settings_governance_family == M5SettingsGovernanceFamily::RolloutCapability)
        .expect("rollout-capability row present");
    assert_eq!(
        row.qualification,
        M5SettingsGovernanceQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let sync_scope: M5SettingsGovernanceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-settings-runtime/sync_scope_beta_narrowed.json"
    )))
    .expect("sync-scope fixture parses");
    assert!(sync_scope.validate().is_empty());
    assert_eq!(
        sync_scope,
        seeded_m5_settings_governance_matrix_sync_scope_beta_narrowed()
    );

    let rollout: M5SettingsGovernanceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/config/m5-settings-runtime/rollout_capability_preview_narrowed.json"
    )))
    .expect("rollout-capability fixture parses");
    assert!(rollout.validate().is_empty());
    assert_eq!(
        rollout,
        seeded_m5_settings_governance_matrix_rollout_capability_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_settings_governance_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_settings_governance_matrix();
    packet.settings_governance_rows[0].scope_summary =
        "raw endpoint https://sync.example/scope leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SettingsGovernanceMatrixViolation::RawMaterialInExport));
}
