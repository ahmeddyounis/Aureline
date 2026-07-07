use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_component_family() {
    let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .component_rows
        .iter()
        .map(|r| r.component_family)
        .collect();
    for family in M5TestExplorerWatchTriageComponentFamily::ALL {
        assert!(
            present.contains(&family),
            "missing component family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.component_rows.len(),
        M5TestExplorerWatchTriageComponentFamily::ALL.len()
    );
}

#[test]
fn every_component_declares_mandatory_labels_and_deployment_lines() {
    let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    for row in &packet.component_rows {
        for label in M5TestRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "component {} missing mandatory label {}",
                row.component_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5TestAccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    for row in &packet.component_rows {
        let family = row.component_family;
        assert_eq!(
            !row.test_identity_classes.is_empty(),
            family.is_test_tree_row(),
            "test_identity_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.result_origins.is_empty(),
            family.is_test_tree_row() || family.is_inline_result_marker(),
            "result_origins presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.marker_verdicts.is_empty(),
            family.is_inline_result_marker(),
            "marker_verdicts presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.result_freshness.is_empty(),
            family.is_inline_result_marker(),
            "result_freshness presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.session_outcomes.is_empty(),
            family.is_session_summary_bar(),
            "session_outcomes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.attempt_lineage_kinds.is_empty(),
            family.is_session_summary_bar(),
            "attempt_lineage_kinds presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.watch_fidelity_states.is_empty(),
            family.is_watch_mode_banner(),
            "watch_fidelity_states presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.watch_degrade_reasons.is_empty(),
            family.is_watch_mode_banner(),
            "watch_degrade_reasons presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.failure_categories.is_empty(),
            family.is_failure_triage_panel(),
            "failure_categories presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.triage_dispositions.is_empty(),
            family.is_failure_triage_panel(),
            "triage_dispositions presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.quarantine_ownership_classes.is_empty(),
            family.is_quarantine_review_sheet(),
            "quarantine_ownership_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.release_impacts.is_empty(),
            family.is_quarantine_review_sheet(),
            "release_impacts presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.target_classes.is_empty(),
            family.is_environment_matrix_card(),
            "target_classes presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.environment_lanes.is_empty(),
            family.is_environment_matrix_card(),
            "environment_lanes presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_component() {
    let packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    for identity in M5TestIdentityClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.test_identity_classes.contains(&identity)),
            "no component declares identity class {}",
            identity.as_str()
        );
    }
    for origin in M5TestResultOrigin::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.result_origins.contains(&origin)),
            "no component declares result origin {}",
            origin.as_str()
        );
    }
    for verdict in M5InlineMarkerVerdict::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.marker_verdicts.contains(&verdict)),
            "no component declares marker verdict {}",
            verdict.as_str()
        );
    }
    for freshness in M5TestResultFreshness::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.result_freshness.contains(&freshness)),
            "no component declares result freshness {}",
            freshness.as_str()
        );
    }
    for outcome in M5TestSessionOutcome::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.session_outcomes.contains(&outcome)),
            "no component declares session outcome {}",
            outcome.as_str()
        );
    }
    for lineage in M5AttemptLineageKind::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.attempt_lineage_kinds.contains(&lineage)),
            "no component declares attempt lineage kind {}",
            lineage.as_str()
        );
    }
    for fidelity in M5WatchFidelityState::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.watch_fidelity_states.contains(&fidelity)),
            "no component declares watch fidelity state {}",
            fidelity.as_str()
        );
    }
    for reason in M5WatchDegradeReason::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.watch_degrade_reasons.contains(&reason)),
            "no component declares watch degrade reason {}",
            reason.as_str()
        );
    }
    for category in M5FailureCategory::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.failure_categories.contains(&category)),
            "no component declares failure category {}",
            category.as_str()
        );
    }
    for disposition in M5TriageDisposition::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.triage_dispositions.contains(&disposition)),
            "no component declares triage disposition {}",
            disposition.as_str()
        );
    }
    for ownership in M5QuarantineOwnership::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.quarantine_ownership_classes.contains(&ownership)),
            "no component declares quarantine ownership {}",
            ownership.as_str()
        );
    }
    for impact in M5TestReleaseImpact::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.release_impacts.contains(&impact)),
            "no component declares release impact {}",
            impact.as_str()
        );
    }
    for target in M5TestTargetClass::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.target_classes.contains(&target)),
            "no component declares target class {}",
            target.as_str()
        );
    }
    for lane in M5TestEnvironmentLane::ALL {
        assert!(
            packet
                .component_rows
                .iter()
                .any(|row| row.environment_lanes.contains(&lane)),
            "no component declares environment lane {}",
            lane.as_str()
        );
    }
}

#[test]
fn missing_component_family_fails_validation() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows.retain(|row| {
        row.component_family != M5TestExplorerWatchTriageComponentFamily::WatchModeBanner
    });
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::RequiredComponentMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.vocabulary_set.watch_fidelity_states.pop();
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[0]
        .required_labels
        .retain(|label| *label != M5TestRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn test_tree_row_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestExplorerWatchTriageComponentFamily::TestTreeRow
            })
            .expect("test-tree row present");
        let expected = if clear == 0 {
            row.test_identity_classes.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::TestIdentityClassMissing
        } else {
            row.result_origins.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::ResultOriginMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn inline_result_marker_vocab_missing_fails() {
    for clear in [0u8, 1, 2] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestExplorerWatchTriageComponentFamily::InlineResultMarker
            })
            .expect("inline result marker present");
        let expected = match clear {
            0 => {
                row.marker_verdicts.clear();
                M5TestExplorerWatchTriageComponentMatrixViolation::MarkerVerdictMissing
            }
            1 => {
                row.result_freshness.clear();
                M5TestExplorerWatchTriageComponentMatrixViolation::ResultFreshnessMissing
            }
            _ => {
                row.result_origins.clear();
                M5TestExplorerWatchTriageComponentMatrixViolation::ResultOriginMissing
            }
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn session_summary_bar_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestExplorerWatchTriageComponentFamily::SessionSummaryBar
            })
            .expect("session-summary bar present");
        let expected = if clear == 0 {
            row.session_outcomes.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::SessionOutcomeMissing
        } else {
            row.attempt_lineage_kinds.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::AttemptLineageMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn watch_mode_banner_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestExplorerWatchTriageComponentFamily::WatchModeBanner
            })
            .expect("watch-mode banner present");
        let expected = if clear == 0 {
            row.watch_fidelity_states.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::WatchFidelityStateMissing
        } else {
            row.watch_degrade_reasons.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::WatchDegradeReasonMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn failure_triage_panel_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family == M5TestExplorerWatchTriageComponentFamily::FailureTriagePanel
            })
            .expect("failure-triage panel present");
        let expected = if clear == 0 {
            row.failure_categories.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::FailureCategoryMissing
        } else {
            row.triage_dispositions.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::TriageDispositionMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn quarantine_review_sheet_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5TestExplorerWatchTriageComponentFamily::QuarantineReviewSheet
            })
            .expect("quarantine-review sheet present");
        let expected = if clear == 0 {
            row.quarantine_ownership_classes.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::QuarantineOwnershipMissing
        } else {
            row.release_impacts.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::ReleaseImpactMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn environment_matrix_card_vocab_missing_fails() {
    for clear in [0u8, 1] {
        let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
        let row = packet
            .component_rows
            .iter_mut()
            .find(|row| {
                row.component_family
                    == M5TestExplorerWatchTriageComponentFamily::EnvironmentMatrixCard
            })
            .expect("environment-matrix card present");
        let expected = if clear == 0 {
            row.target_classes.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::TargetClassMissing
        } else {
            row.environment_lanes.clear();
            M5TestExplorerWatchTriageComponentMatrixViolation::EnvironmentLaneMissing
        };
        assert!(packet.validate().contains(&expected));
    }
}

#[test]
fn component_invariant_violation_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[0].masks_identity_or_origin = true;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[5].hides_quarantine_release_impact = true;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[2].invents_alternate_state_label = true;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ComponentInvariantViolated));

    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[2].widens_rerun_scope_silently = true;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ComponentInvariantViolated));
}

#[test]
fn stable_component_missing_proof_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5TestExplorerWatchTriageComponentFamily::TestTreeRow)
        .expect("test-tree row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::StableComponentMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.component_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet
        .governance_review
        .no_surface_invents_alternate_state_label = false;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet.validate().contains(
        &M5TestExplorerWatchTriageComponentMatrixViolation::ConsumerProjectionIncomplete
    ));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_test_explorer_watch_triage_component_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TestExplorerWatchTriageComponentMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_component_family() {
    let summary = seeded_m5_test_explorer_watch_triage_component_matrix().render_markdown_summary();
    for family in M5TestExplorerWatchTriageComponentFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_component() {
    let csv = seeded_m5_test_explorer_watch_triage_component_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5TestExplorerWatchTriageComponentFamily::ALL.len()
    );
    assert!(lines[0].starts_with("component_family,qualification,owner,"));
    for family in M5TestExplorerWatchTriageComponentFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing component {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_test_explorer_watch_triage_component_matrix_export()
        .expect("checked M5 test-explorer watch triage component matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_test_explorer_watch_triage_component_matrix_export()
        .expect("checked M5 test-explorer watch triage component matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_test_explorer_watch_triage_component_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_components_visible() {
    for packet in [
        seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed(),
        seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.component_rows.len(),
            M5TestExplorerWatchTriageComponentFamily::ALL.len()
        );
    }

    let watch =
        seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed();
    let row = watch
        .component_rows
        .iter()
        .find(|r| r.component_family == M5TestExplorerWatchTriageComponentFamily::WatchModeBanner)
        .expect("watch-mode-banner row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Beta);

    let quarantine =
        seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed();
    let row = quarantine
        .component_rows
        .iter()
        .find(|r| {
            r.component_family == M5TestExplorerWatchTriageComponentFamily::QuarantineReviewSheet
        })
        .expect("quarantine-review-sheet row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let watch: M5TestExplorerWatchTriageComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-test-explorer-watch-triage-components/watch_mode_banner_beta_narrowed.json"
        )))
        .expect("watch-mode-banner fixture parses");
    assert!(watch.validate().is_empty());
    assert_eq!(
        watch,
        seeded_m5_test_explorer_watch_triage_component_matrix_watch_mode_banner_beta_narrowed()
    );

    let quarantine: M5TestExplorerWatchTriageComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-test-explorer-watch-triage-components/quarantine_review_sheet_preview_narrowed.json"
        )))
        .expect("quarantine-review-sheet fixture parses");
    assert!(quarantine.validate().is_empty());
    assert_eq!(
        quarantine,
        seeded_m5_test_explorer_watch_triage_component_matrix_quarantine_review_sheet_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_test_explorer_watch_triage_component_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
