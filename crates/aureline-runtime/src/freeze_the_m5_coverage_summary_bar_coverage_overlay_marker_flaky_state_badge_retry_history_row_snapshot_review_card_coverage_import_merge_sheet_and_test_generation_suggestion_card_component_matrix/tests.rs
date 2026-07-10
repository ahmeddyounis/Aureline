use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_test_intelligence_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_test_intelligence_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5TestIntelligenceComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5TestIntelligenceComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_provenance_and_deployment_lines() {
    let packet = seeded_m5_test_intelligence_component_matrix();
    for row in &packet.component_rows {
        for label in M5TestIntelligenceRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            !row.provenance_classes.is_empty(),
            "component {} declares no provenance classes",
            row.component_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_test_intelligence_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.coverage_scope_classes.is_empty(),
            family.is_coverage_summary_bar(),
            "coverage_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.coverage_metric_kinds.is_empty(),
            family.is_coverage_summary_bar(),
            "coverage_metric_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.overlay_states.is_empty(),
            family.is_coverage_overlay_marker(),
            "overlay_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.overlay_emphasis_classes.is_empty(),
            family.is_coverage_overlay_marker(),
            "overlay_emphasis_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.flaky_classifications.is_empty(),
            family.is_flaky_state_badge(),
            "flaky_classifications presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.flaky_confidence_classes.is_empty(),
            family.is_flaky_state_badge(),
            "flaky_confidence_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.retry_attempt_outcomes.is_empty(),
            family.is_retry_history_row(),
            "retry_attempt_outcomes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.retry_scope_classes.is_empty(),
            family.is_retry_history_row(),
            "retry_scope_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.snapshot_baseline_identities.is_empty(),
            family.is_snapshot_review_card(),
            "snapshot_baseline_identities presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.snapshot_diff_states.is_empty(),
            family.is_snapshot_review_card(),
            "snapshot_diff_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.coverage_import_sources.is_empty(),
            family.is_coverage_import_merge_sheet(),
            "coverage_import_sources presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.merge_resolution_states.is_empty(),
            family.is_coverage_import_merge_sheet(),
            "merge_resolution_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.generated_assumption_classes.is_empty(),
            family.is_test_generation_suggestion_card(),
            "generated_assumption_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.generated_apply_scopes.is_empty(),
            family.is_test_generation_suggestion_card(),
            "generated_apply_scopes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_test_intelligence_component_matrix();
    for provenance in M5TestIntelligenceProvenanceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.provenance_classes.contains(&provenance)),
            "no component declares provenance class {}",
            provenance.as_str()
        );
    }
    for scope in M5CoverageScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.coverage_scope_classes.contains(&scope)),
            "no component declares coverage scope {}",
            scope.as_str()
        );
    }
    for metric in M5CoverageMetricKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.coverage_metric_kinds.contains(&metric)),
            "no component declares coverage metric {}",
            metric.as_str()
        );
    }
    for state in M5CoverageOverlayState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.overlay_states.contains(&state)),
            "no component declares overlay state {}",
            state.as_str()
        );
    }
    for emphasis in M5OverlayEmphasisClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.overlay_emphasis_classes.contains(&emphasis)),
            "no component declares overlay emphasis {}",
            emphasis.as_str()
        );
    }
    for classification in M5FlakyClassification::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.flaky_classifications.contains(&classification)),
            "no component declares flaky classification {}",
            classification.as_str()
        );
    }
    for confidence in M5FlakyConfidenceClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.flaky_confidence_classes.contains(&confidence)),
            "no component declares flaky confidence {}",
            confidence.as_str()
        );
    }
    for outcome in M5RetryAttemptOutcome::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.retry_attempt_outcomes.contains(&outcome)),
            "no component declares retry outcome {}",
            outcome.as_str()
        );
    }
    for scope in M5RetryScopeClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.retry_scope_classes.contains(&scope)),
            "no component declares retry scope {}",
            scope.as_str()
        );
    }
    for baseline in M5SnapshotBaselineIdentity::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.snapshot_baseline_identities.contains(&baseline)),
            "no component declares snapshot baseline {}",
            baseline.as_str()
        );
    }
    for diff in M5SnapshotDiffState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.snapshot_diff_states.contains(&diff)),
            "no component declares snapshot diff state {}",
            diff.as_str()
        );
    }
    for source in M5CoverageImportSource::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.coverage_import_sources.contains(&source)),
            "no component declares coverage import source {}",
            source.as_str()
        );
    }
    for merge in M5MergeResolutionState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.merge_resolution_states.contains(&merge)),
            "no component declares merge resolution {}",
            merge.as_str()
        );
    }
    for assumption in M5GeneratedAssumptionClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.generated_assumption_classes.contains(&assumption)),
            "no component declares generated assumption {}",
            assumption.as_str()
        );
    }
    for apply in M5GeneratedApplyScope::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.generated_apply_scopes.contains(&apply)),
            "no component declares generated apply scope {}",
            apply.as_str()
        );
    }
}

#[test]
fn provenance_vocabulary_covers_acceptance_criteria_tokens() {
    let vocab = M5TestIntelligenceComponentVocabularySet::canonical();
    for token in [
        "verified_current_run",
        "imported_ci_artifact",
        "cached_local_result",
        "stale_prior_result",
        "suspected_flaky",
        "reproduced_flaky",
        "stable_again",
        "manually_muted",
        "unknown",
    ] {
        assert!(
            vocab.provenance_classes.iter().any(|v| v == token),
            "provenance vocabulary missing acceptance-criteria token {token}"
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet
        .component_rows
        .retain(|row| row.component_family != M5TestIntelligenceComponentFamily::FlakyStateBadge);
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.vocabulary_set.provenance_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5TestIntelligenceRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn provenance_class_missing_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[0].provenance_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ProvenanceClassMissing));
}

#[test]
fn coverage_summary_bar_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestIntelligenceComponentFamily::CoverageSummaryBar
            })
            .expect("coverage-summary bar present");
        let expected = if clear == 0 {
            row.coverage_scope_classes.clear();
            M5TestIntelligenceComponentMatrixViolation::CoverageScopeMissing
        } else {
            row.coverage_metric_kinds.clear();
            M5TestIntelligenceComponentMatrixViolation::CoverageMetricMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn coverage_overlay_marker_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestIntelligenceComponentFamily::CoverageOverlayMarker
            })
            .expect("coverage-overlay marker present");
        let expected = if clear == 0 {
            row.overlay_states.clear();
            M5TestIntelligenceComponentMatrixViolation::OverlayStateMissing
        } else {
            row.overlay_emphasis_classes.clear();
            M5TestIntelligenceComponentMatrixViolation::OverlayEmphasisMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn flaky_state_badge_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5TestIntelligenceComponentFamily::FlakyStateBadge)
            .expect("flaky-state badge present");
        let expected = if clear == 0 {
            row.flaky_classifications.clear();
            M5TestIntelligenceComponentMatrixViolation::FlakyClassificationMissing
        } else {
            row.flaky_confidence_classes.clear();
            M5TestIntelligenceComponentMatrixViolation::FlakyConfidenceMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn retry_history_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| row.component_family == M5TestIntelligenceComponentFamily::RetryHistoryRow)
            .expect("retry-history row present");
        let expected = if clear == 0 {
            row.retry_attempt_outcomes.clear();
            M5TestIntelligenceComponentMatrixViolation::RetryOutcomeMissing
        } else {
            row.retry_scope_classes.clear();
            M5TestIntelligenceComponentMatrixViolation::RetryScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn snapshot_review_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestIntelligenceComponentFamily::SnapshotReviewCard
            })
            .expect("snapshot-review card present");
        let expected = if clear == 0 {
            row.snapshot_baseline_identities.clear();
            M5TestIntelligenceComponentMatrixViolation::SnapshotBaselineMissing
        } else {
            row.snapshot_diff_states.clear();
            M5TestIntelligenceComponentMatrixViolation::SnapshotDiffStateMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn coverage_import_merge_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestIntelligenceComponentFamily::CoverageImportMergeSheet
            })
            .expect("coverage-import merge sheet present");
        let expected = if clear == 0 {
            row.coverage_import_sources.clear();
            M5TestIntelligenceComponentMatrixViolation::CoverageImportSourceMissing
        } else {
            row.merge_resolution_states.clear();
            M5TestIntelligenceComponentMatrixViolation::MergeResolutionMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn test_generation_suggestion_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_intelligence_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5TestIntelligenceComponentFamily::TestGenerationSuggestionCard
            })
            .expect("test-generation suggestion card present");
        let expected = if clear == 0 {
            row.generated_assumption_classes.clear();
            M5TestIntelligenceComponentMatrixViolation::GeneratedAssumptionMissing
        } else {
            row.generated_apply_scopes.clear();
            M5TestIntelligenceComponentMatrixViolation::GeneratedApplyScopeMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[0].masks_provenance_or_freshness_class = true;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[5].hides_shard_omission_behind_single_percentage = true;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[2].labels_intermittent_failure_as_confirmed_flaky = true;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[6].bundles_generated_changes_into_opaque_apply = true;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[1].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TestIntelligenceComponentFamily::CoverageSummaryBar)
        .expect("coverage-summary bar present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_test_intelligence_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TestIntelligenceComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_test_intelligence_component_matrix().render_markdown_summary();
    for family in M5TestIntelligenceComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_test_intelligence_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5TestIntelligenceComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5TestIntelligenceComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_test_intelligence_component_matrix_export()
        .expect("checked M5 test-intelligence component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_TEST_INTELLIGENCE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_test_intelligence_component_matrix_export()
        .expect("checked M5 test-intelligence component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_test_intelligence_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed(),
        seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5TestIntelligenceComponentFamily::ALL.len()
        );
    }

    let flaky = seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed();
    let row = flaky
        .component_rows
        .iter()
        .find(|r| r.component_family == M5TestIntelligenceComponentFamily::FlakyStateBadge)
        .expect("flaky-state-badge row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Beta
    );

    let import =
        seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed();
    let row = import
        .component_rows
        .iter()
        .find(|r| r.component_family == M5TestIntelligenceComponentFamily::CoverageImportMergeSheet)
        .expect("coverage-import-merge-sheet row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let flaky: M5TestIntelligenceComponentMatrixPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-test-intelligence-components/flaky_state_badge_beta_narrowed.json"
        )
    ))
    .expect("flaky-state-badge fixture parses");
    assert!(flaky.validate().is_empty());
    assert_eq!(
        flaky,
        seeded_m5_test_intelligence_component_matrix_flaky_state_badge_beta_narrowed()
    );

    let import: M5TestIntelligenceComponentMatrixPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-test-intelligence-components/coverage_import_merge_sheet_preview_narrowed.json"
        )
    ))
    .expect("coverage-import-merge-sheet fixture parses");
    assert!(import.validate().is_empty());
    assert_eq!(
        import,
        seeded_m5_test_intelligence_component_matrix_coverage_import_merge_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_test_intelligence_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}
