use super::*;

fn live_local_marker() -> M5InlineMarkerResolutionInput {
    M5InlineMarkerResolutionInput {
        verdict: M5InlineMarkerVerdict::Passed,
        failure_category: None,
        stability_chip: M5MarkerStabilityChip::StableChip,
        result_origin: M5TestResultOrigin::LiveLocal,
        result_freshness: M5TestResultFreshness::Fresh,
        source_mapping: M5MarkerSourceMapping::ExactMapping,
        target_class: M5TestTargetClass::UnitTest,
        environment_lane: M5TestEnvironmentLane::LocalHost,
        attempt_lineage: M5AttemptLineageKind::RetriedPass,
        quarantine_ownership: M5QuarantineOwnership::Unowned,
        release_impact: M5TestReleaseImpact::NoImpact,
        recent_attempt_count: 3,
        item_muted: false,
        marker_label: "token refresh returns a fresh token".to_owned(),
        marker_identity_ref: "marker:auth-unit::token-refresh".to_owned(),
    }
}

// ---- inline-result-marker resolver --------------------------------------

#[test]
fn live_local_marker_reruns_opens_attempts_and_implies_current_result() {
    let resolved = resolve_inline_result_marker(&live_local_marker()).expect("resolves");
    assert_eq!(
        resolved.marker_posture,
        M5InlineMarkerPosture::LiveLocalMarker
    );
    assert!(resolved.can_rerun_from_marker);
    assert!(resolved.can_open_recent_attempts);
    assert!(resolved.shows_live_certainty);
    assert!(resolved.implies_current_local_result);
    assert!(!resolved.carries_reduced_certainty);
    assert!(!resolved.is_muted);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5InlineMarkerAction::RevealMarkerEvidence,
            M5InlineMarkerAction::OpenRecentAttempts,
            M5InlineMarkerAction::RerunFromMarker,
            M5InlineMarkerAction::ExportMarker,
        ]
    );
    assert_eq!(
        resolved.marker_identity_ref,
        "marker:auth-unit::token-refresh"
    );
}

#[test]
fn posture_ladder_is_honesty_first() {
    // Muted / quarantined wins even over a fresh live-local exactly-mapped pass.
    let muted = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        item_muted: true,
        quarantine_ownership: M5QuarantineOwnership::TeamOwned,
        release_impact: M5TestReleaseImpact::HiddenFromRelease,
        ..live_local_marker()
    })
    .expect("resolves");
    assert_eq!(
        muted.marker_posture,
        M5InlineMarkerPosture::QuarantinedMarker
    );
    assert!(muted.is_muted);
    assert!(muted
        .available_actions
        .contains(&M5InlineMarkerAction::ReviewQuarantine));
    // A quarantined live-local marker still reruns but never implies a current result.
    assert!(muted.can_rerun_from_marker);
    assert!(!muted.shows_live_certainty);
    assert!(!muted.implies_current_local_result);

    // Unmapped-to-buffer next.
    let unmapped = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        source_mapping: M5MarkerSourceMapping::UnmappedToBuffer,
        recent_attempt_count: 0,
        ..live_local_marker()
    })
    .expect("resolves");
    assert_eq!(
        unmapped.marker_posture,
        M5InlineMarkerPosture::UnmappedMarker
    );
    assert!(!unmapped.can_rerun_from_marker);
    assert!(!unmapped.can_open_recent_attempts);
    assert!(unmapped.carries_reduced_certainty);
    assert!(!unmapped.shows_live_certainty);
    assert!(!unmapped
        .available_actions
        .contains(&M5InlineMarkerAction::RerunFromMarker));

    // Approximate-mapping next.
    let approximate = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        source_mapping: M5MarkerSourceMapping::ApproximateMapping,
        ..live_local_marker()
    })
    .expect("resolves");
    assert_eq!(
        approximate.marker_posture,
        M5InlineMarkerPosture::ApproximateMappingMarker
    );
    assert!(approximate.can_rerun_from_marker);
    assert!(approximate.carries_reduced_certainty);
    assert!(!approximate.implies_current_local_result);

    // Imported evidence next.
    let imported = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        result_origin: M5TestResultOrigin::ImportedCi,
        verdict: M5InlineMarkerVerdict::Failed,
        failure_category: Some(M5FailureCategory::Timeout),
        ..live_local_marker()
    })
    .expect("resolves");
    assert_eq!(
        imported.marker_posture,
        M5InlineMarkerPosture::ImportedEvidenceMarker
    );
    assert!(!imported.can_rerun_from_marker);
    assert!(imported.carries_reduced_certainty);
    assert!(!imported.implies_current_local_result);

    // Stale live-local result next.
    let stale = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        result_freshness: M5TestResultFreshness::Stale,
        verdict: M5InlineMarkerVerdict::Failed,
        ..live_local_marker()
    })
    .expect("resolves");
    assert_eq!(
        stale.marker_posture,
        M5InlineMarkerPosture::StaleResultMarker
    );
    assert!(stale.can_rerun_from_marker);
    assert!(stale.carries_reduced_certainty);
    assert!(!stale.shows_live_certainty);
    assert!(!stale.implies_current_local_result);
}

#[test]
fn imported_and_stale_never_imply_a_current_local_result() {
    for origin in [
        M5TestResultOrigin::ImportedCi,
        M5TestResultOrigin::ImportedTeammate,
        M5TestResultOrigin::ReplayedSnapshot,
        M5TestResultOrigin::SyntheticSeed,
        M5TestResultOrigin::UnknownOrigin,
    ] {
        let resolved = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
            result_origin: origin,
            ..live_local_marker()
        })
        .expect("resolves");
        assert!(
            !resolved.implies_current_local_result,
            "non-live origin {} implied a current local result",
            origin.as_str()
        );
        assert!(resolved.carries_reduced_certainty);
    }
    for freshness in [
        M5TestResultFreshness::Stale,
        M5TestResultFreshness::OutdatedSource,
        M5TestResultFreshness::Expired,
    ] {
        let resolved = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
            result_freshness: freshness,
            ..live_local_marker()
        })
        .expect("resolves");
        assert!(
            !resolved.implies_current_local_result,
            "stale freshness {} implied a current local result",
            freshness.as_str()
        );
    }
}

#[test]
fn open_recent_attempts_present_exactly_when_attempts_exist() {
    let with_attempts = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        recent_attempt_count: 5,
        ..live_local_marker()
    })
    .expect("resolves");
    assert!(with_attempts.can_open_recent_attempts);
    assert!(with_attempts
        .available_actions
        .contains(&M5InlineMarkerAction::OpenRecentAttempts));

    let without_attempts = resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
        recent_attempt_count: 0,
        ..live_local_marker()
    })
    .expect("resolves");
    assert!(!without_attempts.can_open_recent_attempts);
    assert!(!without_attempts
        .available_actions
        .contains(&M5InlineMarkerAction::OpenRecentAttempts));
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
            marker_label: " ".to_owned(),
            ..live_local_marker()
        }),
        Err(M5InlineMarkerResolutionError::EmptyMarkerLabel)
    );
    assert_eq!(
        resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
            marker_identity_ref: "".to_owned(),
            ..live_local_marker()
        }),
        Err(M5InlineMarkerResolutionError::EmptyMarkerIdentity)
    );
    assert_eq!(
        resolve_inline_result_marker(&M5InlineMarkerResolutionInput {
            marker_identity_ref: "marker:https://ci.example/run".to_owned(),
            ..live_local_marker()
        }),
        Err(M5InlineMarkerResolutionError::ForbiddenMarkerMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_inline_result_marker_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_INLINE_RESULT_MARKER_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_inline_result_marker_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5InlineMarkerConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5InlineMarkerConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_inline_result_marker_packet();
    for row in &packet.rows {
        for part in M5InlineMarkerAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5InlineMarkerExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestAccessibilityRoute::KeyboardFocusable));
        assert!(!row.marker_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_inline_result_marker_packet();
    let cases: Vec<&M5InlineMarkerResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.marker_examples.iter())
        .collect();

    for posture in M5InlineMarkerPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.marker_posture == posture),
            "no example exercises posture {}",
            posture.as_str()
        );
    }
    for mapping in M5MarkerSourceMapping::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.source_mapping == mapping),
            "no example exercises source mapping {}",
            mapping.as_str()
        );
    }
    for action in M5InlineMarkerAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_inline_result_marker_packet();
    for row in &packet.rows {
        for case in &row.marker_examples {
            assert!(
                case.is_self_consistent(),
                "marker case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "marker case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5InlineMarkerConsumerSurface::NotebookCellMarker);
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.vocabulary_set.marker_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5InlineMarkerAnatomyPart::OriginClassCue);
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5InlineMarkerExportField::AttemptLineage);
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[0].marker_examples[0]
        .resolved
        .can_rerun_from_marker = false;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::ExampleResolutionDrift));
}

#[test]
fn marker_example_missing_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[1].marker_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::MarkerExampleMissing));
}

#[test]
fn mapping_coverage_unproven_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    // Replace every example with a live-local exactly-mapped one so most mappings go
    // uncovered.
    for row in &mut packet.rows {
        row.marker_examples = vec![M5InlineMarkerResolutionCase::resolved(live_local_marker())];
    }
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::MappingCoverageUnproven));
}

#[test]
fn certainty_coverage_unproven_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    // Replace every example with a live-local one so the reduced-certainty half fires.
    for row in &mut packet.rows {
        row.marker_examples = vec![M5InlineMarkerResolutionCase::resolved(live_local_marker())];
    }
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::CertaintyCoverageUnproven));
}

#[test]
fn recent_attempts_coverage_unproven_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    // Replace every example with one that has recent attempts so the no-attempts half fires.
    for row in &mut packet.rows {
        row.marker_examples = vec![M5InlineMarkerResolutionCase::resolved(live_local_marker())];
    }
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::RecentAttemptsCoverageUnproven));
}

#[test]
fn quarantine_coverage_unproven_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    // Replace every example with a non-muted one so the muted half fires.
    for row in &mut packet.rows {
        row.marker_examples = vec![M5InlineMarkerResolutionCase::resolved(live_local_marker())];
    }
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::QuarantineCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[0].overstates_imported_or_stale_as_live = true;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet
        .governance_review
        .imported_or_stale_never_reads_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.consumer_projection.tree_and_triage_read_same_labels = false;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_inline_result_marker_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5InlineMarkerViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_inline_result_marker_packet().render_markdown_summary();
    for surface in M5InlineMarkerConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_inline_result_marker_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5InlineMarkerConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5InlineMarkerConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_inline_result_marker_export()
        .expect("checked M5 inline marker primitive export validates");
    assert_eq!(from_disk.packet_id, M5_INLINE_RESULT_MARKER_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_inline_result_marker_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed(),
        seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5InlineMarkerConsumerSurface::ALL.len());
    }

    let notebook = seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed();
    let row = notebook
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5InlineMarkerConsumerSurface::NotebookCellMarker)
        .expect("notebook-cell-marker row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Preview);

    let headless = seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5InlineMarkerConsumerSurface::HeadlessCliMarker)
        .expect("headless-cli-marker row present");
    assert_eq!(row.qualification, M5TestQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let notebook: M5InlineResultMarkerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-inline-result-marker-primitive/notebook_cell_marker_preview_narrowed.json"
    )))
    .expect("notebook-cell fixture parses");
    assert!(notebook.validate().is_empty());
    assert_eq!(
        notebook,
        seeded_m5_inline_result_marker_notebook_cell_marker_preview_narrowed()
    );

    let headless: M5InlineResultMarkerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-inline-result-marker-primitive/headless_cli_marker_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_inline_result_marker_headless_cli_marker_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_inline_result_marker_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
