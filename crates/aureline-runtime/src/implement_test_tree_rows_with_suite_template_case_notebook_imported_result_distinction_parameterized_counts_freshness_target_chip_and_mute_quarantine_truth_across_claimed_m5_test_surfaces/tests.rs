use super::*;

fn live_concrete_row() -> M5TestTreeRowResolutionInput {
    M5TestTreeRowResolutionInput {
        item_class: M5TestTreeItemClass::ConcreteCase,
        identity_class: M5TestIdentityClass::DurableKeyed,
        result_origin: M5TestResultOrigin::LiveLocal,
        result_freshness: M5TestResultFreshness::Fresh,
        current_verdict: M5InlineMarkerVerdict::Passed,
        target_class: M5TestTargetClass::UnitTest,
        environment_lane: M5TestEnvironmentLane::LocalHost,
        quarantine_ownership: M5QuarantineOwnership::Unowned,
        release_impact: M5TestReleaseImpact::NoImpact,
        parameterized_case_count: 1,
        item_muted: false,
        item_label: "token refresh returns a fresh token".to_owned(),
        item_identity_ref: "tree:auth-unit-suite::token-refresh".to_owned(),
    }
}

// ---- test-tree-row resolver ---------------------------------------------

#[test]
fn live_concrete_case_reruns_debugs_and_shows_live_certainty() {
    let resolved = resolve_test_tree_row(&live_concrete_row()).expect("resolves");
    assert_eq!(resolved.row_posture, M5TestTreeRowPosture::LiveConcreteRow);
    assert_eq!(resolved.rerun_scope, M5TestTreeRerunScope::SingleCase);
    assert!(resolved.can_rerun);
    assert!(resolved.can_debug);
    assert!(resolved.shows_live_certainty);
    assert!(!resolved.carries_reduced_certainty);
    assert!(!resolved.is_muted);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5TestTreeRowAction::RevealItemIdentity,
            M5TestTreeRowAction::RerunItem,
            M5TestTreeRowAction::DebugItem,
            M5TestTreeRowAction::ExportRow,
        ]
    );
    assert_eq!(
        resolved.item_identity_ref,
        "tree:auth-unit-suite::token-refresh"
    );
}

#[test]
fn posture_ladder_is_honesty_first() {
    // Muted / quarantined wins even over a fresh live concrete case.
    let muted = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
        item_muted: true,
        quarantine_ownership: M5QuarantineOwnership::TeamOwned,
        release_impact: M5TestReleaseImpact::HiddenFromRelease,
        ..live_concrete_row()
    })
    .expect("resolves");
    assert_eq!(muted.row_posture, M5TestTreeRowPosture::QuarantinedRow);
    assert!(muted.is_muted);
    assert!(muted
        .available_actions
        .contains(&M5TestTreeRowAction::ReviewQuarantine));
    // A quarantined concrete case still names its single-case rerun.
    assert_eq!(muted.rerun_scope, M5TestTreeRerunScope::SingleCase);
    assert!(!muted.shows_live_certainty);

    // Partial-discovery next.
    let partial = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
        item_class: M5TestTreeItemClass::PartialDiscoveryPlaceholder,
        identity_class: M5TestIdentityClass::DiscoveredOnly,
        result_origin: M5TestResultOrigin::UnknownOrigin,
        result_freshness: M5TestResultFreshness::NeverRun,
        current_verdict: M5InlineMarkerVerdict::NotRun,
        ..live_concrete_row()
    })
    .expect("resolves");
    assert_eq!(
        partial.row_posture,
        M5TestTreeRowPosture::PartialDiscoveryRow
    );
    assert_eq!(
        partial.rerun_scope,
        M5TestTreeRerunScope::NothingConcreteYet
    );
    assert!(!partial.can_rerun);
    assert!(!partial.can_debug);
    assert!(partial.carries_reduced_certainty);
    assert!(!partial.shows_live_certainty);
    assert!(!partial
        .available_actions
        .contains(&M5TestTreeRowAction::RerunItem));

    // Imported evidence next.
    let imported = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
        item_class: M5TestTreeItemClass::ImportedResult,
        identity_class: M5TestIdentityClass::ImportedRecord,
        result_origin: M5TestResultOrigin::ImportedCi,
        current_verdict: M5InlineMarkerVerdict::Failed,
        ..live_concrete_row()
    })
    .expect("resolves");
    assert_eq!(
        imported.row_posture,
        M5TestTreeRowPosture::ImportedEvidenceRow
    );
    assert_eq!(
        imported.rerun_scope,
        M5TestTreeRerunScope::ImportedReplayOnly
    );
    assert!(!imported.can_rerun);
    assert!(imported.carries_reduced_certainty);
    assert!(!imported.shows_live_certainty);

    // Stale live-local result next.
    let stale = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
        result_freshness: M5TestResultFreshness::Stale,
        current_verdict: M5InlineMarkerVerdict::Failed,
        ..live_concrete_row()
    })
    .expect("resolves");
    assert_eq!(stale.row_posture, M5TestTreeRowPosture::StaleResultRow);
    assert!(stale.can_rerun);
    assert!(!stale.shows_live_certainty);

    // Suite aggregate next.
    let suite = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
        item_class: M5TestTreeItemClass::Suite,
        ..live_concrete_row()
    })
    .expect("resolves");
    assert_eq!(suite.row_posture, M5TestTreeRowPosture::SuiteAggregateRow);
    assert_eq!(suite.rerun_scope, M5TestTreeRerunScope::WholeSuite);
    assert!(suite.can_rerun);
    assert!(!suite.can_debug);
}

#[test]
fn imported_and_partial_never_read_as_live() {
    for origin in [
        M5TestResultOrigin::ImportedCi,
        M5TestResultOrigin::ImportedTeammate,
        M5TestResultOrigin::ReplayedSnapshot,
    ] {
        let resolved = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
            result_origin: origin,
            ..live_concrete_row()
        })
        .expect("resolves");
        assert!(
            !resolved.shows_live_certainty,
            "imported origin {} read as live",
            origin.as_str()
        );
        assert!(resolved.carries_reduced_certainty);
    }
}

#[test]
fn rerun_scope_never_widens_beyond_item_class() {
    let cases = [
        (M5TestTreeItemClass::Suite, M5TestTreeRerunScope::WholeSuite),
        (
            M5TestTreeItemClass::Template,
            M5TestTreeRerunScope::ParameterizedGroup,
        ),
        (
            M5TestTreeItemClass::ConcreteCase,
            M5TestTreeRerunScope::SingleCase,
        ),
        (
            M5TestTreeItemClass::NotebookBackedItem,
            M5TestTreeRerunScope::NotebookCells,
        ),
        (
            M5TestTreeItemClass::ImportedResult,
            M5TestTreeRerunScope::ImportedReplayOnly,
        ),
        (
            M5TestTreeItemClass::PartialDiscoveryPlaceholder,
            M5TestTreeRerunScope::NothingConcreteYet,
        ),
    ];
    for (item_class, expected_scope) in cases {
        let resolved = resolve_test_tree_row(&M5TestTreeRowResolutionInput {
            item_class,
            ..live_concrete_row()
        })
        .expect("resolves");
        assert_eq!(
            resolved.rerun_scope,
            expected_scope,
            "item class {} widened its rerun scope",
            item_class.as_str()
        );
    }
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_test_tree_row(&M5TestTreeRowResolutionInput {
            item_label: " ".to_owned(),
            ..live_concrete_row()
        }),
        Err(M5TestTreeRowResolutionError::EmptyItemLabel)
    );
    assert_eq!(
        resolve_test_tree_row(&M5TestTreeRowResolutionInput {
            item_identity_ref: "".to_owned(),
            ..live_concrete_row()
        }),
        Err(M5TestTreeRowResolutionError::EmptyItemIdentity)
    );
    assert_eq!(
        resolve_test_tree_row(&M5TestTreeRowResolutionInput {
            item_identity_ref: "tree:https://ci.example/run".to_owned(),
            ..live_concrete_row()
        }),
        Err(M5TestTreeRowResolutionError::ForbiddenTreeMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_test_tree_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_TEST_TREE_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_test_tree_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5TestTreeConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5TestTreeConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_test_tree_row_packet();
    for row in &packet.rows {
        for part in M5TestTreeRowAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5TestTreeRowExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestAccessibilityRoute::KeyboardFocusable));
        assert!(!row.tree_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_test_tree_row_packet();
    let cases: Vec<&M5TestTreeRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.tree_examples.iter())
        .collect();

    for posture in M5TestTreeRowPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.row_posture == posture),
            "no example exercises posture {}",
            posture.as_str()
        );
    }
    for scope in M5TestTreeRerunScope::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.rerun_scope == scope),
            "no example exercises rerun scope {}",
            scope.as_str()
        );
    }
    for action in M5TestTreeRowAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for class in M5TestTreeItemClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.item_class == class),
            "no example exercises item class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_test_tree_row_packet();
    for row in &packet.rows {
        for case in &row.tree_examples {
            assert!(
                case.is_self_consistent(),
                "tree case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "tree case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5TestTreeConsumerSurface::RunPanelTree);
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.vocabulary_set.row_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5TestTreeRowAnatomyPart::OriginCue);
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5TestTreeRowExportField::RerunScope);
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[0].tree_examples[0].resolved.can_rerun = false;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::ExampleResolutionDrift));
}

#[test]
fn tree_example_missing_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[1].tree_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::TreeExampleMissing));
}

#[test]
fn item_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    // Replace every example with a live concrete one so most item classes go uncovered.
    for row in &mut packet.rows {
        row.tree_examples = vec![M5TestTreeRowResolutionCase::resolved(live_concrete_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::ItemClassCoverageUnproven));
}

#[test]
fn certainty_coverage_unproven_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    // Replace every example with a live concrete one so the reduced-certainty half fires.
    for row in &mut packet.rows {
        row.tree_examples = vec![M5TestTreeRowResolutionCase::resolved(live_concrete_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::CertaintyCoverageUnproven));
}

#[test]
fn rerun_coverage_unproven_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    // Replace every example with a rerunnable one so the not-rerunnable half fires.
    for row in &mut packet.rows {
        row.tree_examples = vec![M5TestTreeRowResolutionCase::resolved(live_concrete_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::RerunCoverageUnproven));
}

#[test]
fn quarantine_coverage_unproven_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    // Replace every example with a non-muted one so the muted half fires.
    for row in &mut packet.rows {
        row.tree_examples = vec![M5TestTreeRowResolutionCase::resolved(live_concrete_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::QuarantineCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[0].overstates_imported_certainty = true;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet
        .governance_review
        .imported_or_partial_never_reads_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.consumer_projection.rerun_scope_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_test_tree_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5TestTreeRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_test_tree_row_packet().render_markdown_summary();
    for surface in M5TestTreeConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_test_tree_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5TestTreeConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5TestTreeConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_test_tree_row_export()
        .expect("checked M5 tree row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_TEST_TREE_ROW_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_test_tree_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_test_tree_row_run_panel_tree_preview_narrowed(),
        seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5TestTreeConsumerSurface::ALL.len());
    }

    let run_panel = seeded_m5_test_tree_row_run_panel_tree_preview_narrowed();
    let row = run_panel
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5TestTreeConsumerSurface::RunPanelTree)
        .expect("run-panel-tree row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Preview);

    let headless = seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5TestTreeConsumerSurface::HeadlessCliTree)
        .expect("headless-cli-tree row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let run_panel: M5TestTreeRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-test-tree-row-primitive/run_panel_tree_preview_narrowed.json"
    )))
    .expect("run-panel fixture parses");
    assert!(run_panel.validate().is_empty());
    assert_eq!(
        run_panel,
        seeded_m5_test_tree_row_run_panel_tree_preview_narrowed()
    );

    let headless: M5TestTreeRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-test-tree-row-primitive/headless_cli_tree_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_test_tree_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
