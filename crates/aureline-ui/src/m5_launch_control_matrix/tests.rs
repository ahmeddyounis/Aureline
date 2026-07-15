use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_launch_control_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LAUNCH_CONTROL_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_cohort() {
    let packet = seeded_m5_launch_control_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .launch_control_rows
        .iter()
        .map(|r| r.cohort_class)
        .collect();
    for cohort in M5LaunchControlCohort::ALL {
        assert!(
            present.contains(&cohort),
            "missing cohort {}",
            cohort.as_str()
        );
    }
    assert_eq!(
        packet.launch_control_rows.len(),
        M5LaunchControlCohort::ALL.len()
    );
}

#[test]
fn frozen_launch_control_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: cohort_membership / readiness_event / rehearsal_currency /
    // freeze_exception_authority / go_no_go_authority / rollback_stop / regression_asset stays in one
    // controlled token set that no shiproom, release-center, executive-steering, program-governance, docs, or
    // support surface reinvents.
    let tokens: Vec<&str> = M5LaunchControlRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "cohort_membership",
            "readiness_event",
            "rehearsal_currency",
            "freeze_exception_authority",
            "go_no_go_authority",
            "rollback_stop",
            "regression_asset",
        ]
    );
    assert!(M5LaunchControlRole::CohortMembership
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5LaunchControlRole::ReadinessEvent
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5LaunchControlRole::GoNoGoAuthority
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(M5LaunchControlRole::FreezeExceptionAuthority
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5LaunchControlRole::RehearsalCurrency
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5LaunchControlRole::RollbackStop
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
    assert!(!M5LaunchControlRole::RegressionAsset
        .must_preserve_evidence_snapshot_and_signoff_before_widening());
}

#[test]
fn every_cohort_declares_mandatory_labels_schema_and_widening_stages() {
    let packet = seeded_m5_launch_control_matrix();
    for row in &packet.launch_control_rows {
        for label in M5LaunchControlRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "cohort {} missing mandatory label {}",
                row.cohort_class.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs
                .contains(&row.cohort_class.canonical_domain_schema_ref().to_owned()),
            "cohort {} does not point at its canonical schema",
            row.cohort_class.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.widening_stages.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5LaunchControlAccessibilityRoute::HighZoomReflow));
    }
}

#[test]
fn cohort_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_launch_control_matrix();
    for row in &packet.launch_control_rows {
        let cohort = row.cohort_class;
        assert_eq!(
            !row.core_team_canary_roles.is_empty(),
            cohort.declares_core_team_canary_roles(),
            "core_team_canary_roles presence wrong for {}",
            cohort.as_str()
        );
        assert_eq!(
            !row.design_partner_preview_roles.is_empty(),
            cohort.declares_design_partner_preview_roles(),
            "design_partner_preview_roles presence wrong for {}",
            cohort.as_str()
        );
        assert_eq!(
            !row.extension_author_roles.is_empty(),
            cohort.declares_extension_author_roles(),
            "extension_author_roles presence wrong for {}",
            cohort.as_str()
        );
        assert_eq!(
            !row.public_preview_roles.is_empty(),
            cohort.declares_public_preview_roles(),
            "public_preview_roles presence wrong for {}",
            cohort.as_str()
        );
        assert_eq!(
            !row.certified_archetype_roles.is_empty(),
            cohort.declares_certified_archetype_roles(),
            "certified_archetype_roles presence wrong for {}",
            cohort.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_cohort() {
    let packet = seeded_m5_launch_control_matrix();
    for role in M5LaunchControlRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no cohort declares launch-control role {}",
            role.as_str()
        );
    }
    for role in M5CoreTeamCanaryRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.core_team_canary_roles.contains(&role)),
            "no cohort declares core-team-canary role {}",
            role.as_str()
        );
    }
    for role in M5DesignPartnerPreviewRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.design_partner_preview_roles.contains(&role)),
            "no cohort declares design-partner-preview role {}",
            role.as_str()
        );
    }
    for role in M5ExtensionAuthorRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.extension_author_roles.contains(&role)),
            "no cohort declares extension-author role {}",
            role.as_str()
        );
    }
    for role in M5PublicPreviewRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.public_preview_roles.contains(&role)),
            "no cohort declares public-preview role {}",
            role.as_str()
        );
    }
    for role in M5CertifiedArchetypeRole::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.certified_archetype_roles.contains(&role)),
            "no cohort declares certified-archetype role {}",
            role.as_str()
        );
    }
    for reason in M5LaunchControlDegradedReason::ALL {
        assert!(
            packet
                .launch_control_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no cohort declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_cohort_fails_validation() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet
        .launch_control_rows
        .retain(|row| row.cohort_class != M5LaunchControlCohort::CertifiedArchetype);
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::RequiredCohortMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[0]
        .required_labels
        .retain(|label| *label != M5LaunchControlRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let own = M5LaunchControlCohort::DesignPartnerPreview.canonical_domain_schema_ref();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::DesignPartnerPreview)
        .expect("design-partner-preview row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::SemanticRoleMissing));
}

#[test]
fn core_team_canary_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::CoreTeamCanary)
        .expect("core-team-canary present");
    row.core_team_canary_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::CoreTeamCanaryRoleMissing));
}

#[test]
fn design_partner_preview_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::DesignPartnerPreview)
        .expect("design-partner-preview present");
    row.design_partner_preview_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::DesignPartnerPreviewRoleMissing));
}

#[test]
fn extension_author_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::ExtensionAuthor)
        .expect("extension-author present");
    row.extension_author_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::ExtensionAuthorRoleMissing));
}

#[test]
fn public_preview_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::PublicPreview)
        .expect("public-preview present");
    row.public_preview_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::PublicPreviewRoleMissing));
}

#[test]
fn certified_archetype_role_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::CertifiedArchetype)
        .expect("certified-archetype present");
    row.certified_archetype_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::CertifiedArchetypeRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[4].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::DegradedReasonMissing));
}

#[test]
fn launch_control_invariant_violation_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[0]
        .widens_a_stable_claim_without_current_cohort_and_rehearsal_evidence = true;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::LaunchControlInvariantViolated));

    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[2].lets_a_freeze_exception_become_undocumented_scope_widening = true;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::LaunchControlInvariantViolated));

    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[1].closes_a_sev_one_or_sev_two_incident_without_a_regression_asset =
        true;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::LaunchControlInvariantViolated));

    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[4].implies_green_when_go_no_go_records_or_orr_packets_are_stale =
        true;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::LaunchControlInvariantViolated));

    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[3]
        .maintains_partner_or_public_support_language_that_outruns_current_cohort_proof = true;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::LaunchControlInvariantViolated));
}

#[test]
fn stable_cohort_missing_proof_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    let row = packet
        .launch_control_rows
        .iter_mut()
        .find(|row| row.cohort_class == M5LaunchControlCohort::DesignPartnerPreview)
        .expect("design-partner-preview row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::StableCohortMissingProof));
}

#[test]
fn missing_widening_stages_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[1].widening_stages.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::WideningStageMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.governance_review.no_stable_claim_skips_cohorts = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_launch_control_source = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_cohort() {
    let summary = seeded_m5_launch_control_matrix().render_markdown_summary();
    for cohort in M5LaunchControlCohort::ALL {
        assert!(
            summary.contains(cohort.as_str()),
            "summary missing cohort {}",
            cohort.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_cohort() {
    let csv = seeded_m5_launch_control_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5LaunchControlCohort::ALL.len());
    assert!(lines[0].starts_with("cohort_class,qualification,owner,canonical_schema,"));
    for cohort in M5LaunchControlCohort::ALL {
        assert!(
            csv.contains(cohort.as_str()),
            "csv missing cohort {}",
            cohort.as_str()
        );
        assert!(
            csv.contains(cohort.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            cohort.as_str()
        );
    }
}

#[test]
fn dashboard_json_names_every_cohort_and_matches_checked_in_file() {
    let rendered: serde_json::Value =
        serde_json::from_str(&seeded_m5_launch_control_matrix().render_dashboard_json())
            .expect("rendered dashboard parses");
    for cohort in M5LaunchControlCohort::ALL {
        assert!(
            rendered["cohorts"]
                .as_array()
                .expect("cohorts array")
                .iter()
                .any(|c| c["cohort"] == cohort.as_str()),
            "dashboard missing cohort {}",
            cohort.as_str()
        );
    }
    let from_disk: serde_json::Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-launch-control-dashboard.json"
    )))
    .expect("checked dashboard parses");
    assert_eq!(
        from_disk, rendered,
        "checked launch-control dashboard drifted from the seed builder"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_launch_control_matrix_export()
        .expect("checked M5 launch-control matrix export validates");
    assert_eq!(packet.packet_id, M5_LAUNCH_CONTROL_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_launch_control_matrix_export()
        .expect("checked M5 launch-control matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_launch_control_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_cohorts_visible() {
    for packet in [
        seeded_m5_launch_control_matrix_public_preview_beta_narrowed(),
        seeded_m5_launch_control_matrix_certified_archetype_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.launch_control_rows.len(),
            M5LaunchControlCohort::ALL.len()
        );
    }

    let public = seeded_m5_launch_control_matrix_public_preview_beta_narrowed();
    let row = public
        .launch_control_rows
        .iter()
        .find(|r| r.cohort_class == M5LaunchControlCohort::PublicPreview)
        .expect("public-preview row present");
    assert_eq!(row.qualification, M5LaunchControlQualificationClass::Beta);

    let certified = seeded_m5_launch_control_matrix_certified_archetype_preview_narrowed();
    let row = certified
        .launch_control_rows
        .iter()
        .find(|r| r.cohort_class == M5LaunchControlCohort::CertifiedArchetype)
        .expect("certified-archetype row present");
    assert_eq!(
        row.qualification,
        M5LaunchControlQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let public: M5LaunchControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-launch-control/public_preview_beta_narrowed.json"
    )))
    .expect("public-preview fixture parses");
    assert!(public.validate().is_empty());
    assert_eq!(
        public,
        seeded_m5_launch_control_matrix_public_preview_beta_narrowed()
    );

    let certified: M5LaunchControlMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5-launch-control/certified_archetype_preview_narrowed.json"
    )))
    .expect("certified-archetype fixture parses");
    assert!(certified.validate().is_empty());
    assert_eq!(
        certified,
        seeded_m5_launch_control_matrix_certified_archetype_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_launch_control_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_launch_control_matrix();
    packet.launch_control_rows[0].scope_summary =
        "raw endpoint https://cohort.example/evidence leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5LaunchControlMatrixViolation::RawMaterialInExport));
}
