use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_install_topology_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_INSTALL_TOPOLOGY_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_install_topology_family() {
    let packet = seeded_m5_install_topology_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .install_topology_rows
        .iter()
        .map(|r| r.install_topology_family)
        .collect();
    for family in M5InstallTopologyFamily::ALL {
        assert!(
            present.contains(&family),
            "missing install-topology family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.install_topology_rows.len(),
        M5InstallTopologyFamily::ALL.len()
    );
}

#[test]
fn frozen_install_topology_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: install_mode / updater_owner / binary_root /
    // writable_state_roots / policy_roots / rollback_target / rollout_ring stays in one controlled token set
    // that no About, update, diagnostics, admin, docs, or support surface reinvents.
    let tokens: Vec<&str> = M5InstallTopologyRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "install_mode",
            "updater_owner",
            "binary_root",
            "writable_state_roots",
            "policy_roots",
            "rollback_target",
            "rollout_ring",
        ]
    );
    assert!(M5InstallTopologyRole::UpdaterOwner
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(M5InstallTopologyRole::WritableStateRoots
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(M5InstallTopologyRole::PolicyRoots
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(M5InstallTopologyRole::RollbackTarget
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(!M5InstallTopologyRole::InstallMode
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(!M5InstallTopologyRole::BinaryRoot
        .must_preserve_state_isolation_and_ownership_under_coexistence());
    assert!(!M5InstallTopologyRole::RolloutRing
        .must_preserve_state_isolation_and_ownership_under_coexistence());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_install_topology_matrix();
    for row in &packet.install_topology_rows {
        for label in M5InstallTopologyRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.install_topology_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.install_topology_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.install_topology_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5InstallTopologyAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_install_topology_matrix();
    for row in &packet.install_topology_rows {
        let family = row.install_topology_family;
        assert_eq!(
            !row.per_user_managed_install_roles.is_empty(),
            family.declares_per_user_managed_install_roles(),
            "per_user_managed_install_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.per_machine_managed_install_roles.is_empty(),
            family.declares_per_machine_managed_install_roles(),
            "per_machine_managed_install_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.side_by_side_channel_roles.is_empty(),
            family.declares_side_by_side_channel_roles(),
            "side_by_side_channel_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.portable_mode_roles.is_empty(),
            family.declares_portable_mode_roles(),
            "portable_mode_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.offline_airgap_bundle_roles.is_empty(),
            family.declares_offline_airgap_bundle_roles(),
            "offline_airgap_bundle_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_install_topology_matrix();
    for role in M5InstallTopologyRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares install-topology role {}",
            role.as_str()
        );
    }
    for role in M5PerUserManagedInstallRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.per_user_managed_install_roles.contains(&role)),
            "no family declares per-user-managed-install role {}",
            role.as_str()
        );
    }
    for role in M5PerMachineManagedInstallRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.per_machine_managed_install_roles.contains(&role)),
            "no family declares per-machine-managed-install role {}",
            role.as_str()
        );
    }
    for role in M5SideBySideChannelRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.side_by_side_channel_roles.contains(&role)),
            "no family declares side-by-side-channel role {}",
            role.as_str()
        );
    }
    for role in M5PortableModeRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.portable_mode_roles.contains(&role)),
            "no family declares portable-mode role {}",
            role.as_str()
        );
    }
    for role in M5OfflineAirgapBundleRole::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.offline_airgap_bundle_roles.contains(&role)),
            "no family declares offline / air-gap-bundle role {}",
            role.as_str()
        );
    }
    for reason in M5InstallTopologyDegradedReason::ALL {
        assert!(
            packet
                .install_topology_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_install_topology_family_fails_validation() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet
        .install_topology_rows
        .retain(|row| row.install_topology_family != M5InstallTopologyFamily::OfflineAirgapBundle);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[0]
        .required_labels
        .retain(|label| *label != M5InstallTopologyRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let own = M5InstallTopologyFamily::SideBySideStablePreview.canonical_domain_schema_ref();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::SideBySideStablePreview)
        .expect("side-by-side row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::SemanticRoleMissing));
}

#[test]
fn per_user_managed_install_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::PerUserManaged)
        .expect("per-user-managed present");
    row.per_user_managed_install_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::PerUserManagedInstallRoleMissing));
}

#[test]
fn per_machine_managed_install_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::PerMachineManaged)
        .expect("per-machine-managed present");
    row.per_machine_managed_install_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::PerMachineManagedInstallRoleMissing));
}

#[test]
fn side_by_side_channel_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::SideBySideStablePreview)
        .expect("side-by-side present");
    row.side_by_side_channel_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::SideBySideChannelRoleMissing));
}

#[test]
fn portable_mode_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::PortableMode)
        .expect("portable-mode present");
    row.portable_mode_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::PortableModeRoleMissing));
}

#[test]
fn offline_airgap_bundle_role_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::OfflineAirgapBundle)
        .expect("offline / air-gap present");
    row.offline_airgap_bundle_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::OfflineAirgapBundleRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::DegradedReasonMissing));
}

#[test]
fn install_topology_invariant_violation_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[3].portable_mode_writes_hidden_machine_global_durable_state = true;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated));

    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[2].preview_channel_reuses_stable_state_namespace_without_handoff =
        true;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated));

    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[4].rollback_targets_primary_executable_while_sidecars_drift = true;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated));

    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[1].hides_updater_ownership_or_admin_control_in_managed_flow = true;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated));

    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[4]
        .publishes_deployment_claim_outpacing_ring_or_repair_verify_evidence = true;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::InstallTopologyInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    let row = packet
        .install_topology_rows
        .iter_mut()
        .find(|row| row.install_topology_family == M5InstallTopologyFamily::SideBySideStablePreview)
        .expect("side-by-side row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet
        .governance_review
        .binary_placement_and_updater_ownership_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_install_topology_source = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_install_topology_family() {
    let summary = seeded_m5_install_topology_matrix().render_markdown_summary();
    for family in M5InstallTopologyFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_install_topology_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5InstallTopologyFamily::ALL.len());
    assert!(lines[0].starts_with("install_topology_family,qualification,owner,canonical_schema,"));
    for family in M5InstallTopologyFamily::ALL {
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
    let packet = current_stable_m5_install_topology_matrix_export()
        .expect("checked M5 install-topology matrix export validates");
    assert_eq!(packet.packet_id, M5_INSTALL_TOPOLOGY_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_install_topology_matrix_export()
        .expect("checked M5 install-topology matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_install_topology_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed(),
        seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.install_topology_rows.len(),
            M5InstallTopologyFamily::ALL.len()
        );
    }

    let side = seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed();
    let row = side
        .install_topology_rows
        .iter()
        .find(|r| r.install_topology_family == M5InstallTopologyFamily::SideBySideStablePreview)
        .expect("side-by-side row present");
    assert_eq!(row.qualification, M5InstallTopologyQualificationClass::Beta);

    let offline = seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed();
    let row = offline
        .install_topology_rows
        .iter()
        .find(|r| r.install_topology_family == M5InstallTopologyFamily::OfflineAirgapBundle)
        .expect("offline / air-gap row present");
    assert_eq!(
        row.qualification,
        M5InstallTopologyQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let side: M5InstallTopologyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-delivery-topologies/side_by_side_channel_beta_narrowed.json"
    )))
    .expect("side-by-side fixture parses");
    assert!(side.validate().is_empty());
    assert_eq!(
        side,
        seeded_m5_install_topology_matrix_side_by_side_channel_beta_narrowed()
    );

    let offline: M5InstallTopologyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/install/m5-delivery-topologies/offline_airgap_bundle_preview_narrowed.json"
    )))
    .expect("offline / air-gap fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_install_topology_matrix_offline_airgap_bundle_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_install_topology_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_install_topology_matrix();
    packet.install_topology_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5InstallTopologyMatrixViolation::RawMaterialInExport));
}
