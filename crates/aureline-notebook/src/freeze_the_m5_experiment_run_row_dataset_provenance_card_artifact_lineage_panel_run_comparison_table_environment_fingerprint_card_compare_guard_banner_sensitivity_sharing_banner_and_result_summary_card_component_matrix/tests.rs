use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_experiment_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_EXPERIMENT_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_experiment_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5ExperimentComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5ExperimentComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_dispositions_and_deployment_lines() {
    let packet = seeded_m5_experiment_component_matrix();
    for row in &packet.component_rows {
        for label in M5ExperimentRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.dispositions.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_experiment_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.run_origin_kinds.is_empty(),
            family.is_experiment_run_row(),
            "run_origin_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.run_status_states.is_empty(),
            family.is_experiment_run_row(),
            "run_status_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.dataset_source_classes.is_empty(),
            family.is_dataset_provenance_card(),
            "dataset_source_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.dataset_provenance_states.is_empty(),
            family.is_dataset_provenance_card(),
            "dataset_provenance_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.artifact_kind_classes.is_empty(),
            family.is_artifact_lineage_panel(),
            "artifact_kind_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.lineage_states.is_empty(),
            family.is_artifact_lineage_panel(),
            "lineage_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.comparison_axis_classes.is_empty(),
            family.is_run_comparison_table(),
            "comparison_axis_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.comparability_states.is_empty(),
            family.is_run_comparison_table(),
            "comparability_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.fingerprint_scope_classes.is_empty(),
            family.is_environment_fingerprint_card(),
            "fingerprint_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.fingerprint_states.is_empty(),
            family.is_environment_fingerprint_card(),
            "fingerprint_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.compare_guard_reasons.is_empty(),
            family.is_compare_guard_banner(),
            "compare_guard_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.compare_guard_states.is_empty(),
            family.is_compare_guard_banner(),
            "compare_guard_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.sensitivity_classes.is_empty(),
            family.is_sensitivity_sharing_banner(),
            "sensitivity_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.share_scope_states.is_empty(),
            family.is_sensitivity_sharing_banner(),
            "share_scope_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.summary_content_classes.is_empty(),
            family.is_result_summary_card(),
            "summary_content_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.summary_export_scopes.is_empty(),
            family.is_result_summary_card(),
            "summary_export_scopes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_experiment_component_matrix();
    for disposition in M5ExperimentDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dispositions.contains(&disposition)),
            "no component declares disposition {}",
            disposition.as_str()
        );
    }
    for kind in M5RunOriginKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.run_origin_kinds.contains(&kind)),
            "no component declares run origin kind {}",
            kind.as_str()
        );
    }
    for state in M5RunStatusState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.run_status_states.contains(&state)),
            "no component declares run status state {}",
            state.as_str()
        );
    }
    for class in M5DatasetSourceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dataset_source_classes.contains(&class)),
            "no component declares dataset source class {}",
            class.as_str()
        );
    }
    for state in M5DatasetProvenanceState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.dataset_provenance_states.contains(&state)),
            "no component declares dataset provenance state {}",
            state.as_str()
        );
    }
    for class in M5ArtifactKindClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.artifact_kind_classes.contains(&class)),
            "no component declares artifact kind class {}",
            class.as_str()
        );
    }
    for state in M5LineageState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.lineage_states.contains(&state)),
            "no component declares lineage state {}",
            state.as_str()
        );
    }
    for class in M5ComparisonAxisClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.comparison_axis_classes.contains(&class)),
            "no component declares comparison axis class {}",
            class.as_str()
        );
    }
    for state in M5ComparabilityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.comparability_states.contains(&state)),
            "no component declares comparability state {}",
            state.as_str()
        );
    }
    for class in M5FingerprintScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.fingerprint_scope_classes.contains(&class)),
            "no component declares fingerprint scope class {}",
            class.as_str()
        );
    }
    for state in M5FingerprintState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.fingerprint_states.contains(&state)),
            "no component declares fingerprint state {}",
            state.as_str()
        );
    }
    for reason in M5CompareGuardReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.compare_guard_reasons.contains(&reason)),
            "no component declares compare guard reason {}",
            reason.as_str()
        );
    }
    for state in M5CompareGuardState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.compare_guard_states.contains(&state)),
            "no component declares compare guard state {}",
            state.as_str()
        );
    }
    for class in M5SensitivityClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.sensitivity_classes.contains(&class)),
            "no component declares sensitivity class {}",
            class.as_str()
        );
    }
    for state in M5ShareScopeState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.share_scope_states.contains(&state)),
            "no component declares share scope state {}",
            state.as_str()
        );
    }
    for class in M5SummaryContentClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.summary_content_classes.contains(&class)),
            "no component declares summary content class {}",
            class.as_str()
        );
    }
    for scope in M5SummaryExportScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.summary_export_scopes.contains(&scope)),
            "no component declares summary export scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn ac_disposition_vocabulary_is_frozen_exactly() {
    // The acceptance criteria pin one controlled vocabulary; assert the exact tokens.
    let tokens: Vec<&str> = M5ExperimentDisposition::ALL
        .iter()
        .map(|d| d.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "local_run",
            "managed_run",
            "imported_run",
            "manual_attach",
            "reproducible",
            "likely_reproducible",
            "needs_rerun",
            "context_incomplete",
        ]
    );
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5ExperimentComponentFamily::ArtifactLineagePanel);
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.vocabulary_set.dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5ExperimentRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn dispositions_missing_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[0].dispositions.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::DispositionsMissing));
}

#[test]
fn experiment_run_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::ExperimentRunRow)
            .expect("experiment-run-row row present");
        let expected = if clear == 0 {
            row.run_origin_kinds.clear();
            M5ExperimentComponentMatrixViolation::RunOriginKindMissing
        } else {
            row.run_status_states.clear();
            M5ExperimentComponentMatrixViolation::RunStatusStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn dataset_provenance_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::DatasetProvenanceCard)
            .expect("dataset-provenance-card row present");
        let expected = if clear == 0 {
            row.dataset_source_classes.clear();
            M5ExperimentComponentMatrixViolation::DatasetSourceClassMissing
        } else {
            row.dataset_provenance_states.clear();
            M5ExperimentComponentMatrixViolation::DatasetProvenanceStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn artifact_lineage_panel_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::ArtifactLineagePanel)
            .expect("artifact-lineage-panel row present");
        let expected = if clear == 0 {
            row.artifact_kind_classes.clear();
            M5ExperimentComponentMatrixViolation::ArtifactKindClassMissing
        } else {
            row.lineage_states.clear();
            M5ExperimentComponentMatrixViolation::LineageStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn run_comparison_table_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::RunComparisonTable)
            .expect("run-comparison-table row present");
        let expected = if clear == 0 {
            row.comparison_axis_classes.clear();
            M5ExperimentComponentMatrixViolation::ComparisonAxisClassMissing
        } else {
            row.comparability_states.clear();
            M5ExperimentComponentMatrixViolation::ComparabilityStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn environment_fingerprint_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ExperimentComponentFamily::EnvironmentFingerprintCard
            })
            .expect("environment-fingerprint-card row present");
        let expected = if clear == 0 {
            row.fingerprint_scope_classes.clear();
            M5ExperimentComponentMatrixViolation::FingerprintScopeClassMissing
        } else {
            row.fingerprint_states.clear();
            M5ExperimentComponentMatrixViolation::FingerprintStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn compare_guard_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::CompareGuardBanner)
            .expect("compare-guard-banner row present");
        let expected = if clear == 0 {
            row.compare_guard_reasons.clear();
            M5ExperimentComponentMatrixViolation::CompareGuardReasonMissing
        } else {
            row.compare_guard_states.clear();
            M5ExperimentComponentMatrixViolation::CompareGuardStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn sensitivity_sharing_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5ExperimentComponentFamily::SensitivitySharingBanner
            })
            .expect("sensitivity-sharing-banner row present");
        let expected = if clear == 0 {
            row.sensitivity_classes.clear();
            M5ExperimentComponentMatrixViolation::SensitivityClassMissing
        } else {
            row.share_scope_states.clear();
            M5ExperimentComponentMatrixViolation::ShareScopeStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn result_summary_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_experiment_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5ExperimentComponentFamily::ResultSummaryCard)
            .expect("result-summary-card row present");
        let expected = if clear == 0 {
            row.summary_content_classes.clear();
            M5ExperimentComponentMatrixViolation::SummaryContentClassMissing
        } else {
            row.summary_export_scopes.clear();
            M5ExperimentComponentMatrixViolation::SummaryExportScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[0].masks_provenance_or_sensitivity_state = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[0].hides_run_origin_or_revision = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[3].implies_apples_to_apples_without_parity = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5ExperimentComponentFamily::DatasetProvenanceCard)
        .expect("dataset-provenance-card row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_experiment_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ExperimentComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_experiment_component_matrix().render_markdown_summary();
    for family in M5ExperimentComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_experiment_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ExperimentComponentFamily::ALL.len());
    assert!(lines[0].starts_with("component_family,qualification,owner,dispositions,"));
    for family in M5ExperimentComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_experiment_component_matrix_export()
        .expect("checked M5 experiment component matrix export validates");
    assert_eq!(packet.packet_id, M5_EXPERIMENT_COMPONENT_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_experiment_component_matrix_export()
        .expect("checked M5 experiment component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_experiment_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed(),
        seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5ExperimentComponentFamily::ALL.len()
        );
    }

    let comparison = seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed();
    let row = comparison
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ExperimentComponentFamily::RunComparisonTable)
        .expect("run-comparison-table row present");
    assert_eq!(row.qualification, M5ExperimentQualificationClass::Beta);

    let sensitivity =
        seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed();
    let row = sensitivity
        .component_rows
        .iter()
        .find(|r| r.component_family == M5ExperimentComponentFamily::SensitivitySharingBanner)
        .expect("sensitivity-sharing-banner row present");
    assert_eq!(row.qualification, M5ExperimentQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let comparison: M5ExperimentComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-experiment-components/run_comparison_table_beta_narrowed.json"
        )))
        .expect("run-comparison-table fixture parses");
    assert!(comparison.validate().is_empty());
    assert_eq!(
        comparison,
        seeded_m5_experiment_component_matrix_run_comparison_table_beta_narrowed()
    );

    let sensitivity: M5ExperimentComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-experiment-components/sensitivity_sharing_banner_preview_narrowed.json"
    )))
    .expect("sensitivity-sharing-banner fixture parses");
    assert!(sensitivity.validate().is_empty());
    assert_eq!(
        sensitivity,
        seeded_m5_experiment_component_matrix_sensitivity_sharing_banner_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_experiment_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
