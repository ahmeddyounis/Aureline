use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_repository_bootstrap_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_REPOSITORY_BOOTSTRAP_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_repository_bootstrap_family() {
    let packet = seeded_m5_repository_bootstrap_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .repository_bootstrap_rows
        .iter()
        .map(|r| r.repository_bootstrap_family)
        .collect();
    for family in M5RepositoryBootstrapFamily::ALL {
        assert!(
            present.contains(&family),
            "missing repository-bootstrap family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.repository_bootstrap_rows.len(),
        M5RepositoryBootstrapFamily::ALL.len()
    );
}

#[test]
fn frozen_repository_bootstrap_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: source_locator / checkout_plan / credential_posture /
    // evidence_packet / staged_trust / resumable_acquisition / post_open_queue stays in one controlled
    // token set that no shell, entry, diagnostics, admin, docs, or support surface reinvents.
    let tokens: Vec<&str> = M5RepositoryBootstrapRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "source_locator",
            "checkout_plan",
            "credential_posture",
            "evidence_packet",
            "staged_trust",
            "resumable_acquisition",
            "post_open_queue",
        ]
    );
    assert!(M5RepositoryBootstrapRole::CredentialPosture
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(M5RepositoryBootstrapRole::EvidencePacket
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(M5RepositoryBootstrapRole::StagedTrust
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(M5RepositoryBootstrapRole::PostOpenQueue
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(!M5RepositoryBootstrapRole::SourceLocator
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(!M5RepositoryBootstrapRole::CheckoutPlan
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
    assert!(!M5RepositoryBootstrapRole::ResumableAcquisition
        .must_stage_trust_and_disclose_provenance_before_bootstrap());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_repository_bootstrap_matrix();
    for row in &packet.repository_bootstrap_rows {
        for label in M5RepositoryBootstrapRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.repository_bootstrap_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.repository_bootstrap_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.repository_bootstrap_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5RepositoryBootstrapAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_repository_bootstrap_matrix();
    for row in &packet.repository_bootstrap_rows {
        let family = row.repository_bootstrap_family;
        assert_eq!(
            !row.open_local_roles.is_empty(),
            family.declares_open_local_roles(),
            "open_local_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.clone_remote_roles.is_empty(),
            family.declares_clone_remote_roles(),
            "clone_remote_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.open_archive_roles.is_empty(),
            family.declares_open_archive_roles(),
            "open_archive_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.import_bundle_roles.is_empty(),
            family.declares_import_bundle_roles(),
            "import_bundle_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.resume_snapshot_roles.is_empty(),
            family.declares_resume_snapshot_roles(),
            "resume_snapshot_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_repository_bootstrap_matrix();
    for role in M5RepositoryBootstrapRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares repository-bootstrap role {}",
            role.as_str()
        );
    }
    for role in M5OpenLocalRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.open_local_roles.contains(&role)),
            "no family declares open-local role {}",
            role.as_str()
        );
    }
    for role in M5CloneRemoteRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.clone_remote_roles.contains(&role)),
            "no family declares clone-remote role {}",
            role.as_str()
        );
    }
    for role in M5OpenArchiveRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.open_archive_roles.contains(&role)),
            "no family declares open-archive role {}",
            role.as_str()
        );
    }
    for role in M5ImportBundleRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.import_bundle_roles.contains(&role)),
            "no family declares import-bundle role {}",
            role.as_str()
        );
    }
    for role in M5ResumeSnapshotRole::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.resume_snapshot_roles.contains(&role)),
            "no family declares resume-snapshot role {}",
            role.as_str()
        );
    }
    for reason in M5RepositoryBootstrapDegradedReason::ALL {
        assert!(
            packet
                .repository_bootstrap_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_repository_bootstrap_family_fails_validation() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows.retain(|row| {
        row.repository_bootstrap_family != M5RepositoryBootstrapFamily::ResumeSnapshot
    });
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[0]
        .required_labels
        .retain(|label| *label != M5RepositoryBootstrapRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let own = M5RepositoryBootstrapFamily::CloneRemote.canonical_domain_schema_ref();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::CloneRemote)
        .expect("clone-remote row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::SemanticRoleMissing));
}

#[test]
fn open_local_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::OpenLocal)
        .expect("open-local present");
    row.open_local_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::OpenLocalRoleMissing));
}

#[test]
fn clone_remote_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::CloneRemote)
        .expect("clone-remote present");
    row.clone_remote_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::CloneRemoteRoleMissing));
}

#[test]
fn open_archive_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::OpenArchive)
        .expect("open-archive present");
    row.open_archive_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::OpenArchiveRoleMissing));
}

#[test]
fn import_bundle_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::ImportBundle)
        .expect("import-bundle present");
    row.import_bundle_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ImportBundleRoleMissing));
}

#[test]
fn resume_snapshot_role_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::ResumeSnapshot)
        .expect("resume-snapshot present");
    row.resume_snapshot_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ResumeSnapshotRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::DegradedReasonMissing));
}

#[test]
fn repository_bootstrap_invariant_violation_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[0]
        .rewrites_clone_into_open_when_local_checkout_already_exists = true;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated));

    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[1].runs_repo_owned_actions_implicitly_during_acquisition =
        true;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated));

    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[3]
        .loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches = true;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated));

    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[4]
        .strands_partial_acquisition_without_resume_discard_or_readonly_choices = true;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated));

    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[1]
        .hides_bootstrap_credential_posture_behind_generic_connected_state_copy = true;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RepositoryBootstrapInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    let row = packet
        .repository_bootstrap_rows
        .iter_mut()
        .find(|row| row.repository_bootstrap_family == M5RepositoryBootstrapFamily::CloneRemote)
        .expect("clone-remote row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[1]
        .consumer_surfaces
        .clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet
        .governance_review
        .source_locator_and_checkout_plan_stay_separately_inspectable = false;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_repository_bootstrap_source = false;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_repository_bootstrap_family() {
    let summary = seeded_m5_repository_bootstrap_matrix().render_markdown_summary();
    for family in M5RepositoryBootstrapFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_repository_bootstrap_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RepositoryBootstrapFamily::ALL.len());
    assert!(
        lines[0].starts_with("repository_bootstrap_family,qualification,owner,canonical_schema,")
    );
    for family in M5RepositoryBootstrapFamily::ALL {
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
    let packet = current_stable_m5_repository_bootstrap_matrix_export()
        .expect("checked M5 repository-bootstrap matrix export validates");
    assert_eq!(packet.packet_id, M5_REPOSITORY_BOOTSTRAP_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_repository_bootstrap_matrix_export()
        .expect("checked M5 repository-bootstrap matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_repository_bootstrap_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_repository_bootstrap_matrix_import_bundle_beta_narrowed(),
        seeded_m5_repository_bootstrap_matrix_resume_snapshot_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.repository_bootstrap_rows.len(),
            M5RepositoryBootstrapFamily::ALL.len()
        );
    }

    let import_bundle = seeded_m5_repository_bootstrap_matrix_import_bundle_beta_narrowed();
    let row = import_bundle
        .repository_bootstrap_rows
        .iter()
        .find(|r| r.repository_bootstrap_family == M5RepositoryBootstrapFamily::ImportBundle)
        .expect("import-bundle row present");
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Beta
    );

    let resume = seeded_m5_repository_bootstrap_matrix_resume_snapshot_preview_narrowed();
    let row = resume
        .repository_bootstrap_rows
        .iter()
        .find(|r| r.repository_bootstrap_family == M5RepositoryBootstrapFamily::ResumeSnapshot)
        .expect("resume-snapshot row present");
    assert_eq!(
        row.qualification,
        M5RepositoryBootstrapQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let import_bundle: M5RepositoryBootstrapMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/workspaces/m5-repository-bootstrap/import_bundle_beta_narrowed.json"
        )))
        .expect("import-bundle fixture parses");
    assert!(import_bundle.validate().is_empty());
    assert_eq!(
        import_bundle,
        seeded_m5_repository_bootstrap_matrix_import_bundle_beta_narrowed()
    );

    let resume: M5RepositoryBootstrapMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/workspaces/m5-repository-bootstrap/resume_snapshot_preview_narrowed.json"
    )))
    .expect("resume-snapshot fixture parses");
    assert!(resume.validate().is_empty());
    assert_eq!(
        resume,
        seeded_m5_repository_bootstrap_matrix_resume_snapshot_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_repository_bootstrap_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_repository_bootstrap_matrix();
    packet.repository_bootstrap_rows[0].scope_summary =
        "raw endpoint https://clone.example/repo leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5RepositoryBootstrapMatrixViolation::RawMaterialInExport));
}
