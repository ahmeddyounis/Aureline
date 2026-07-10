use super::*;

fn full_suite_summary() -> M5CoverageSummaryResolutionInput {
    M5CoverageSummaryResolutionInput {
        scope_class: M5CoverageScopeClass::FullSuite,
        metric_kind: M5CoverageMetricKind::LineCoverage,
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        freshness_state: M5CoverageFreshnessState::FreshCurrentRun,
        source_note: M5CoverageSourceNote::LiveLocalRun,
        included_run_count: 1,
        covered_units: 880,
        total_units: 1000,
        has_shard_omission: false,
        scope_label: "full suite: line coverage".to_owned(),
        summary_identity_ref: "coverage:report::full-suite-line".to_owned(),
    }
}

fn covered_overlay() -> M5OverlayMarkerResolutionInput {
    M5OverlayMarkerResolutionInput {
        overlay_state: M5CoverageOverlayState::CoveredLine,
        emphasis_class: M5OverlayEmphasisClass::StableCovered,
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        is_changed_line: false,
        source_run_set_ref: "run-set:report::current-full".to_owned(),
        evidence_object_ref: "coverage-object:report::covered-line-42".to_owned(),
        line_reference: "src/pricing.rs:42".to_owned(),
    }
}

// ---- coverage-summary-bar resolver --------------------------------------

#[test]
fn full_suite_summary_is_single_run_and_fresh() {
    let resolved = resolve_coverage_summary_bar(&full_suite_summary()).expect("resolves");
    assert_eq!(
        resolved.coverage_posture,
        M5CoverageSummaryPosture::FullSuiteSummary
    );
    assert!(!resolved.is_multi_run);
    assert!(!resolved.is_imported);
    assert!(!resolved.is_stale);
    assert!(!resolved.requires_included_run_label);
    assert!(resolved.has_uncovered);
    assert!(!resolved.can_rerun);
    assert!(resolved
        .available_actions
        .contains(&M5CoverageSummaryAction::OpenUncoveredLines));
    assert!(!resolved
        .available_actions
        .contains(&M5CoverageSummaryAction::RerunCoverage));
    assert_eq!(
        resolved.summary_identity_ref,
        "coverage:report::full-suite-line"
    );
}

#[test]
fn every_scope_class_has_a_distinct_posture() {
    // The acceptance-criterion axis: no two coverage scopes collapse into one percentage.
    let cases = [
        (
            M5CoverageScopeClass::FullSuite,
            M5CoverageSummaryPosture::FullSuiteSummary,
        ),
        (
            M5CoverageScopeClass::ChangedFilesOnly,
            M5CoverageSummaryPosture::ChangedFilesSummary,
        ),
        (
            M5CoverageScopeClass::SingleShard,
            M5CoverageSummaryPosture::SingleShardSummary,
        ),
        (
            M5CoverageScopeClass::MergedMultiShard,
            M5CoverageSummaryPosture::MergedMultiShardSummary,
        ),
        (
            M5CoverageScopeClass::ImportedReport,
            M5CoverageSummaryPosture::ImportedReportSummary,
        ),
        (
            M5CoverageScopeClass::PartialIncomplete,
            M5CoverageSummaryPosture::PartialIncompleteSummary,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (scope, expected) in cases {
        let resolved = resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
            scope_class: scope,
            ..full_suite_summary()
        })
        .expect("resolves");
        assert_eq!(resolved.coverage_posture, expected);
        assert_eq!(resolved.coverage_posture.scope(), scope);
        postures.insert(resolved.coverage_posture);
    }
    assert_eq!(postures.len(), M5CoverageSummaryPosture::ALL.len());
}

#[test]
fn merged_multi_shard_requires_an_included_run_label() {
    let resolved = resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
        scope_class: M5CoverageScopeClass::MergedMultiShard,
        source_note: M5CoverageSourceNote::MergedMultiRun,
        freshness_state: M5CoverageFreshnessState::RecentlyMeasured,
        included_run_count: 4,
        ..full_suite_summary()
    })
    .expect("resolves");
    assert!(resolved.is_multi_run);
    assert!(resolved.requires_included_run_label);
    // A non-current merged number always offers a rerun.
    assert!(resolved.can_rerun);
    assert!(resolved
        .available_actions
        .contains(&M5CoverageSummaryAction::RerunCoverage));
}

#[test]
fn imported_report_is_labeled_and_never_a_local_number() {
    let resolved = resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
        scope_class: M5CoverageScopeClass::ImportedReport,
        provenance_class: M5TestIntelligenceProvenanceClass::ImportedCiArtifact,
        freshness_state: M5CoverageFreshnessState::ImportedSnapshot,
        source_note: M5CoverageSourceNote::ImportedReport,
        ..full_suite_summary()
    })
    .expect("resolves");
    assert!(resolved.is_imported);
    assert!(resolved.requires_included_run_label);
    assert!(resolved.needs_attention);
}

#[test]
fn stale_and_shard_omission_stay_disclosed() {
    let resolved = resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
        scope_class: M5CoverageScopeClass::PartialIncomplete,
        provenance_class: M5TestIntelligenceProvenanceClass::StalePriorResult,
        freshness_state: M5CoverageFreshnessState::StaleNeedsRerun,
        source_note: M5CoverageSourceNote::StaleReplay,
        has_shard_omission: true,
        ..full_suite_summary()
    })
    .expect("resolves");
    assert!(resolved.is_stale);
    assert!(resolved.discloses_shard_omission);
    assert!(resolved.needs_attention);
    assert!(resolved.can_rerun);
}

#[test]
fn summary_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
            scope_label: "  ".to_owned(),
            ..full_suite_summary()
        }),
        Err(M5CoverageSummaryResolutionError::EmptyScopeLabel)
    );
    assert_eq!(
        resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
            summary_identity_ref: "".to_owned(),
            ..full_suite_summary()
        }),
        Err(M5CoverageSummaryResolutionError::EmptySummaryIdentity)
    );
    assert_eq!(
        resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
            covered_units: 1200,
            total_units: 1000,
            ..full_suite_summary()
        }),
        Err(M5CoverageSummaryResolutionError::InvalidUnitCount)
    );
    assert_eq!(
        resolve_coverage_summary_bar(&M5CoverageSummaryResolutionInput {
            summary_identity_ref: "coverage:https://ci.example/report".to_owned(),
            ..full_suite_summary()
        }),
        Err(M5CoverageSummaryResolutionError::ForbiddenCoverageMaterial)
    );
}

// ---- coverage-overlay-marker resolver -----------------------------------

#[test]
fn covered_marker_needs_no_attention_and_keeps_continuity() {
    let resolved = resolve_coverage_overlay_marker(&covered_overlay()).expect("resolves");
    assert_eq!(
        resolved.overlay_posture,
        M5OverlayMarkerPosture::CoveredMarker
    );
    assert!(!resolved.needs_attention);
    assert!(resolved.preserves_state_meaning);
    assert!(resolved.has_report_continuity);
    assert!(!resolved.is_emphasized_change);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5OverlayMarkerAction::RevealMarkerDetails,
            M5OverlayMarkerAction::OpenCoverageReport,
            M5OverlayMarkerAction::ExportMarker,
        ]
    );
}

#[test]
fn overlay_posture_is_one_to_one_with_state() {
    for state in M5CoverageOverlayState::ALL {
        let resolved = resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
            overlay_state: state,
            ..covered_overlay()
        })
        .expect("resolves");
        assert_eq!(resolved.overlay_posture.overlay_state(), state);
        assert!(resolved.preserves_state_meaning);
    }
}

#[test]
fn uncovered_changed_line_stays_emphasized_and_offers_context() {
    let resolved = resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
        overlay_state: M5CoverageOverlayState::UncoveredLine,
        emphasis_class: M5OverlayEmphasisClass::NewlyUncovered,
        is_changed_line: true,
        ..covered_overlay()
    })
    .expect("resolves");
    assert_eq!(
        resolved.overlay_posture,
        M5OverlayMarkerPosture::UncoveredMarker
    );
    assert!(resolved.is_emphasized_change);
    assert!(resolved.needs_attention);
    assert!(resolved
        .available_actions
        .contains(&M5OverlayMarkerAction::OpenUncoveredContext));
}

#[test]
fn context_line_is_not_emphasized_even_when_changed() {
    // A changed line whose emphasis is a plain context line is not an emphasized change.
    let resolved = resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
        overlay_state: M5CoverageOverlayState::CoveredLine,
        emphasis_class: M5OverlayEmphasisClass::ContextLine,
        is_changed_line: true,
        ..covered_overlay()
    })
    .expect("resolves");
    assert!(!resolved.is_emphasized_change);
}

#[test]
fn overlay_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
            source_run_set_ref: "  ".to_owned(),
            ..covered_overlay()
        }),
        Err(M5OverlayMarkerResolutionError::EmptySourceRunSet)
    );
    assert_eq!(
        resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
            evidence_object_ref: "".to_owned(),
            ..covered_overlay()
        }),
        Err(M5OverlayMarkerResolutionError::EmptyEvidenceObject)
    );
    assert_eq!(
        resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
            line_reference: "   ".to_owned(),
            ..covered_overlay()
        }),
        Err(M5OverlayMarkerResolutionError::EmptyLineReference)
    );
    assert_eq!(
        resolve_coverage_overlay_marker(&M5OverlayMarkerResolutionInput {
            evidence_object_ref: "coverage-object bearer token".to_owned(),
            ..covered_overlay()
        }),
        Err(M5OverlayMarkerResolutionError::ForbiddenOverlayMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_coverage_components_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_COVERAGE_COMPONENTS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_coverage_components_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5CoverageComponentConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5CoverageComponentConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_coverage_components_packet();
    for row in &packet.rows {
        for part in M5CoverageSummaryAnatomyPart::MANDATORY {
            assert!(row.summary_anatomy_parts.contains(&part));
        }
        for part in M5OverlayMarkerAnatomyPart::MANDATORY {
            assert!(row.overlay_anatomy_parts.contains(&part));
        }
        for field in M5CoverageSummaryExportField::MANDATORY {
            assert!(row.summary_export_fields.contains(&field));
        }
        for field in M5OverlayMarkerExportField::MANDATORY {
            assert!(row.overlay_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.summary_examples.is_empty());
        assert!(!row.overlay_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_coverage_components_packet();
    let summaries: Vec<&M5CoverageSummaryResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.summary_examples.iter())
        .collect();
    let overlays: Vec<&M5OverlayMarkerResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.overlay_examples.iter())
        .collect();

    for posture in M5CoverageSummaryPosture::ALL {
        assert!(
            summaries
                .iter()
                .any(|c| c.resolved.coverage_posture == posture),
            "no example exercises coverage posture {}",
            posture.as_str()
        );
    }
    for action in M5CoverageSummaryAction::ALL {
        assert!(
            summaries
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises summary action {}",
            action.as_str()
        );
    }
    for posture in M5OverlayMarkerPosture::ALL {
        assert!(
            overlays
                .iter()
                .any(|c| c.resolved.overlay_posture == posture),
            "no example exercises overlay posture {}",
            posture.as_str()
        );
    }
    for action in M5OverlayMarkerAction::ALL {
        assert!(
            overlays
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises overlay action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_coverage_components_packet();
    for row in &packet.rows {
        for case in &row.summary_examples {
            assert!(
                case.is_self_consistent(),
                "summary case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "summary case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.overlay_examples {
            assert!(
                case.is_self_consistent(),
                "overlay case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "overlay case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5CoverageComponentConsumerSurface::CiCoverageSummary
    });
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.vocabulary_set.overlay_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::VocabularySetDrift));
}

#[test]
fn mandatory_summary_anatomy_missing_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0]
        .summary_anatomy_parts
        .retain(|p| *p != M5CoverageSummaryAnatomyPart::IncludedRunSetCue);
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MandatorySummaryAnatomyMissing));
}

#[test]
fn mandatory_overlay_anatomy_missing_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0]
        .overlay_anatomy_parts
        .retain(|p| *p != M5OverlayMarkerAnatomyPart::EvidenceLinkCue);
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MandatoryOverlayAnatomyMissing));
}

#[test]
fn mandatory_summary_export_missing_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0]
        .summary_export_fields
        .retain(|f| *f != M5CoverageSummaryExportField::SourceNote);
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MandatorySummaryExportMissing));
}

#[test]
fn mandatory_overlay_export_missing_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0]
        .overlay_export_fields
        .retain(|f| *f != M5OverlayMarkerExportField::EvidenceObjectRef);
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MandatoryOverlayExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0].summary_examples[0].resolved.is_multi_run = true;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[1].overlay_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ExampleMissing));
}

#[test]
fn coverage_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    // Replace every summary example with a full-suite one so most postures go uncovered.
    let full = M5CoverageSummaryResolutionCase::resolved(full_suite_summary());
    for row in &mut packet.rows {
        row.summary_examples = vec![full.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::CoveragePostureCoverageUnproven));
}

#[test]
fn overlay_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    for row in &mut packet.rows {
        row.overlay_examples = vec![M5OverlayMarkerResolutionCase::resolved(covered_overlay())];
    }
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::OverlayPostureCoverageUnproven));
}

#[test]
fn multi_run_disclosure_unproven_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    // Replace every summary example with a single-run full-suite one so the multi half fires.
    let full = M5CoverageSummaryResolutionCase::resolved(full_suite_summary());
    for row in &mut packet.rows {
        row.summary_examples = vec![full.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MultiRunDisclosureUnproven));
}

#[test]
fn stale_disclosure_unproven_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    // Replace every summary example with a fresh one so the stale half fires.
    let full = M5CoverageSummaryResolutionCase::resolved(full_suite_summary());
    for row in &mut packet.rows {
        row.summary_examples = vec![full.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::StaleDisclosureUnproven));
}

#[test]
fn changed_line_emphasis_unproven_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    // Replace every overlay example with a non-emphasized covered one so the emphasis proof
    // fires.
    let covered = M5OverlayMarkerResolutionCase::resolved(covered_overlay());
    for row in &mut packet.rows {
        row.overlay_examples = vec![covered.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ChangedLineEmphasisUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0].collapses_multi_run_into_single_percentage = true;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet
        .governance_review
        .bar_never_collapses_multi_run_into_one_percentage = false;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet
        .consumer_projection
        .ci_and_support_read_same_coverage_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_coverage_components_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5CoverageComponentViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_coverage_components_packet().render_markdown_summary();
    for surface in M5CoverageComponentConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_coverage_components_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5CoverageComponentConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5CoverageComponentConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_coverage_components_export()
        .expect("checked M5 coverage components export validates");
    assert_eq!(from_disk.packet_id, M5_COVERAGE_COMPONENTS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_coverage_components_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_coverage_components_report_panel_preview_narrowed(),
        seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5CoverageComponentConsumerSurface::ALL.len()
        );
    }

    let report_panel = seeded_m5_coverage_components_report_panel_preview_narrowed();
    let row = report_panel
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5CoverageComponentConsumerSurface::CoverageReportPanel)
        .expect("coverage-report-panel row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Preview
    );

    let editor = seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed();
    let row = editor
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5CoverageComponentConsumerSurface::EditorGutterOverlay)
        .expect("editor-gutter-overlay row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Beta
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let report_panel: M5CoverageComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-coverage-summary-overlay-primitive/coverage_report_panel_preview_narrowed.json"
    )))
    .expect("coverage-report-panel fixture parses");
    assert!(report_panel.validate().is_empty());
    assert_eq!(
        report_panel,
        seeded_m5_coverage_components_report_panel_preview_narrowed()
    );

    let editor: M5CoverageComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-coverage-summary-overlay-primitive/editor_gutter_overlay_beta_narrowed.json"
    )))
    .expect("editor-gutter-overlay fixture parses");
    assert!(editor.validate().is_empty());
    assert_eq!(
        editor,
        seeded_m5_coverage_components_editor_gutter_overlay_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_coverage_components_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
