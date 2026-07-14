use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_window_restore_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_WINDOW_RESTORE_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_window_restore_family() {
    let packet = seeded_m5_window_restore_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .window_restore_rows
        .iter()
        .map(|r| r.window_restore_family)
        .collect();
    for family in M5WindowRestoreFamily::ALL {
        assert!(
            present.contains(&family),
            "missing window-restore family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.window_restore_rows.len(),
        M5WindowRestoreFamily::ALL.len()
    );
}

#[test]
fn frozen_window_restore_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: workspace_authority / window_topology / pane_role /
    // layout_skeleton / session_hydration / restore_fidelity / display_affinity stays in one controlled
    // token set that no shell, recovery, diagnostics, admin, docs, or support surface reinvents.
    let tokens: Vec<&str> = M5WindowRestoreRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "workspace_authority",
            "window_topology",
            "pane_role",
            "layout_skeleton",
            "session_hydration",
            "restore_fidelity",
            "display_affinity",
        ]
    );
    assert!(M5WindowRestoreRole::WorkspaceAuthority
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(M5WindowRestoreRole::SessionHydration
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(M5WindowRestoreRole::RestoreFidelity
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(M5WindowRestoreRole::DisplayAffinity
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(!M5WindowRestoreRole::WindowTopology
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(!M5WindowRestoreRole::PaneRole
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
    assert!(!M5WindowRestoreRole::LayoutSkeleton
        .must_preserve_window_local_selection_and_no_rerun_under_shared_authority());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_window_restore_matrix();
    for row in &packet.window_restore_rows {
        for label in M5WindowRestoreRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.window_restore_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.window_restore_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.window_restore_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5WindowRestoreAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_window_restore_matrix();
    for row in &packet.window_restore_rows {
        let family = row.window_restore_family;
        assert_eq!(
            !row.shared_workspace_authority_roles.is_empty(),
            family.declares_shared_workspace_authority_roles(),
            "shared_workspace_authority_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.window_local_topology_roles.is_empty(),
            family.declares_window_local_topology_roles(),
            "window_local_topology_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.skeleton_first_restore_roles.is_empty(),
            family.declares_skeleton_first_restore_roles(),
            "skeleton_first_restore_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.no_rerun_session_hydration_roles.is_empty(),
            family.declares_no_rerun_session_hydration_roles(),
            "no_rerun_session_hydration_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.display_topology_recovery_roles.is_empty(),
            family.declares_display_topology_recovery_roles(),
            "display_topology_recovery_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_window_restore_matrix();
    for role in M5WindowRestoreRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares window-restore role {}",
            role.as_str()
        );
    }
    for role in M5SharedWorkspaceAuthorityRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.shared_workspace_authority_roles.contains(&role)),
            "no family declares shared-workspace-authority role {}",
            role.as_str()
        );
    }
    for role in M5WindowLocalTopologyRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.window_local_topology_roles.contains(&role)),
            "no family declares window-local-topology role {}",
            role.as_str()
        );
    }
    for role in M5SkeletonFirstRestoreRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.skeleton_first_restore_roles.contains(&role)),
            "no family declares skeleton-first-restore role {}",
            role.as_str()
        );
    }
    for role in M5NoRerunSessionHydrationRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.no_rerun_session_hydration_roles.contains(&role)),
            "no family declares no-rerun-session-hydration role {}",
            role.as_str()
        );
    }
    for role in M5DisplayTopologyRecoveryRole::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.display_topology_recovery_roles.contains(&role)),
            "no family declares display-topology-recovery role {}",
            role.as_str()
        );
    }
    for reason in M5WindowRestoreDegradedReason::ALL {
        assert!(
            packet
                .window_restore_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_window_restore_family_fails_validation() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet
        .window_restore_rows
        .retain(|row| row.window_restore_family != M5WindowRestoreFamily::DisplayTopologyRecovery);
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[0]
        .required_labels
        .retain(|label| *label != M5WindowRestoreRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let own = M5WindowRestoreFamily::SkeletonFirstRestore.canonical_domain_schema_ref();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::SkeletonFirstRestore)
        .expect("skeleton-restore row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::SemanticRoleMissing));
}

#[test]
fn shared_workspace_authority_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::SharedWorkspaceAuthority)
        .expect("shared-authority present");
    row.shared_workspace_authority_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::SharedWorkspaceAuthorityRoleMissing));
}

#[test]
fn window_local_topology_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::WindowLocalTopology)
        .expect("window-local present");
    row.window_local_topology_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowLocalTopologyRoleMissing));
}

#[test]
fn skeleton_first_restore_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::SkeletonFirstRestore)
        .expect("skeleton-restore present");
    row.skeleton_first_restore_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::SkeletonFirstRestoreRoleMissing));
}

#[test]
fn no_rerun_session_hydration_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::NoRerunSessionHydration)
        .expect("no-rerun present");
    row.no_rerun_session_hydration_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::NoRerunSessionHydrationRoleMissing));
}

#[test]
fn display_topology_recovery_role_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::DisplayTopologyRecovery)
        .expect("display-recovery present");
    row.display_topology_recovery_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::DisplayTopologyRecoveryRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::DegradedReasonMissing));
}

#[test]
fn window_restore_invariant_violation_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[3]
        .reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore = true;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated));

    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[2]
        .deletes_layout_structure_silently_on_missing_extension_or_remote_target = true;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated));

    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[4]
        .leaves_windows_or_dialogs_unreachable_after_display_topology_remap = true;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated));

    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[0]
        .merges_workspace_authority_and_window_topology_into_one_opaque_blob = true;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated));

    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[2]
        .overclaims_restore_fidelity_when_only_context_or_evidence_reopened = true;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::WindowRestoreInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    let row = packet
        .window_restore_rows
        .iter_mut()
        .find(|row| row.window_restore_family == M5WindowRestoreFamily::SkeletonFirstRestore)
        .expect("skeleton-restore row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet
        .governance_review
        .workspace_authority_and_window_topology_stay_separately_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_window_restore_source = false;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_window_restore_family() {
    let summary = seeded_m5_window_restore_matrix().render_markdown_summary();
    for family in M5WindowRestoreFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_window_restore_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5WindowRestoreFamily::ALL.len());
    assert!(lines[0].starts_with("window_restore_family,qualification,owner,canonical_schema,"));
    for family in M5WindowRestoreFamily::ALL {
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
    let packet = current_stable_m5_window_restore_matrix_export()
        .expect("checked M5 window-restore matrix export validates");
    assert_eq!(packet.packet_id, M5_WINDOW_RESTORE_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_window_restore_matrix_export()
        .expect("checked M5 window-restore matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_window_restore_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed(),
        seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.window_restore_rows.len(),
            M5WindowRestoreFamily::ALL.len()
        );
    }

    let no_rerun = seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed();
    let row = no_rerun
        .window_restore_rows
        .iter()
        .find(|r| r.window_restore_family == M5WindowRestoreFamily::NoRerunSessionHydration)
        .expect("no-rerun row present");
    assert_eq!(row.qualification, M5WindowRestoreQualificationClass::Beta);

    let display = seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed();
    let row = display
        .window_restore_rows
        .iter()
        .find(|r| r.window_restore_family == M5WindowRestoreFamily::DisplayTopologyRecovery)
        .expect("display-recovery row present");
    assert_eq!(
        row.qualification,
        M5WindowRestoreQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let no_rerun: M5WindowRestoreMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-window-restore/no_rerun_session_hydration_beta_narrowed.json"
    )))
    .expect("no-rerun fixture parses");
    assert!(no_rerun.validate().is_empty());
    assert_eq!(
        no_rerun,
        seeded_m5_window_restore_matrix_no_rerun_session_hydration_beta_narrowed()
    );

    let display: M5WindowRestoreMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-window-restore/display_topology_recovery_preview_narrowed.json"
    )))
    .expect("display-recovery fixture parses");
    assert!(display.validate().is_empty());
    assert_eq!(
        display,
        seeded_m5_window_restore_matrix_display_topology_recovery_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_window_restore_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_window_restore_matrix();
    packet.window_restore_rows[0].scope_summary =
        "raw endpoint https://restore.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5WindowRestoreMatrixViolation::RawMaterialInExport));
}
