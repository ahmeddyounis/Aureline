use super::*;

fn diff_detected_card() -> M5SnapshotCardResolutionInput {
    M5SnapshotCardResolutionInput {
        artifact_kind: M5SnapshotArtifactKind::ImageSnapshot,
        baseline_identity: M5SnapshotBaselineIdentity::CommittedBaseline,
        diff_state: M5SnapshotDiffState::DiffDetected,
        fallback_mode: M5SnapshotFallbackMode::SideBySide,
        scope_dimensions: vec![
            M5SnapshotScopeDimension::Environment,
            M5SnapshotScopeDimension::Viewport,
            M5SnapshotScopeDimension::Theme,
        ],
        diff_count: 3,
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        card_identity_ref: "snapshot-card:review-panel::checkout-visual".to_owned(),
        baseline_ref: "baseline:review-panel::checkout-visual-committed".to_owned(),
    }
}

fn shard_omission_sheet() -> M5MergeSheetResolutionInput {
    M5MergeSheetResolutionInput {
        import_source: M5CoverageImportSource::ImportedCiArtifact,
        merge_resolution: M5MergeResolutionState::ShardOmissionDetected,
        metric_kinds: vec![
            M5CoverageMetricKind::LineCoverage,
            M5CoverageMetricKind::BranchCoverage,
        ],
        included_runs: vec!["ci-run-1".to_owned()],
        excluded_runs: vec!["ci-shard-b".to_owned(), "ci-shard-c".to_owned()],
        provenance_class: M5TestIntelligenceProvenanceClass::ImportedCiArtifact,
        is_stale: false,
        is_incompatible: false,
        claims_exact_current_truth: false,
        commit_identity_ref: "commit:import-panel::ghi789".to_owned(),
        build_identity_ref: "build:import-panel::ci-5120".to_owned(),
        sheet_identity_ref: "merge-sheet:import-panel::ci-shard-omission".to_owned(),
    }
}

// ---- snapshot-review-card resolver ---------------------------------------

#[test]
fn diff_detected_card_is_scoped_acceptance() {
    let resolved = resolve_snapshot_review_card(&diff_detected_card()).expect("resolves");
    assert_eq!(
        resolved.review_posture,
        M5SnapshotReviewPosture::DiffDetectedCard
    );
    assert!(resolved.is_acceptance_decision);
    assert!(resolved.has_scope_disclosed);
    assert!(resolved.acceptance_is_scoped);
    assert!(resolved.has_raw_fallback);
    assert!(resolved.needs_attention);
    assert!(resolved
        .available_actions
        .contains(&M5SnapshotCardAction::AcceptBaseline));
    assert!(resolved
        .available_actions
        .contains(&M5SnapshotCardAction::RejectChange));
    assert_eq!(
        resolved.card_identity_ref,
        "snapshot-card:review-panel::checkout-visual"
    );
}

#[test]
fn every_diff_state_has_a_distinct_posture() {
    // The acceptance-criterion axis: a new snapshot never borrows a matched-baseline posture.
    let cases = [
        (
            M5SnapshotDiffState::MatchesBaseline,
            M5SnapshotReviewPosture::MatchesBaselineCard,
        ),
        (
            M5SnapshotDiffState::DiffDetected,
            M5SnapshotReviewPosture::DiffDetectedCard,
        ),
        (
            M5SnapshotDiffState::NewSnapshot,
            M5SnapshotReviewPosture::NewSnapshotCard,
        ),
        (
            M5SnapshotDiffState::ObsoleteSnapshot,
            M5SnapshotReviewPosture::ObsoleteSnapshotCard,
        ),
        (
            M5SnapshotDiffState::RenderUnavailable,
            M5SnapshotReviewPosture::RenderUnavailableCard,
        ),
        (
            M5SnapshotDiffState::RawTextFallback,
            M5SnapshotReviewPosture::RawTextFallbackCard,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (diff_state, expected) in cases {
        let resolved = resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            diff_state,
            // A raw fallback so the render-unavailable case still resolves.
            fallback_mode: M5SnapshotFallbackMode::SideBySide,
            ..diff_detected_card()
        })
        .expect("resolves");
        assert_eq!(resolved.review_posture, expected);
        assert_eq!(resolved.review_posture.diff_state(), diff_state);
        postures.insert(resolved.review_posture);
    }
    assert_eq!(postures.len(), M5SnapshotReviewPosture::ALL.len());
}

#[test]
fn acceptance_without_scope_cannot_collapse_to_blind_accept() {
    // A detected diff with no disclosed scope fails resolution — the core acceptance criterion.
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            diff_state: M5SnapshotDiffState::DiffDetected,
            scope_dimensions: vec![],
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::BlindAcceptanceWithoutScope)
    );
    // A brand-new snapshot with no scope also fails.
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            diff_state: M5SnapshotDiffState::NewSnapshot,
            scope_dimensions: vec![],
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::BlindAcceptanceWithoutScope)
    );
}

#[test]
fn opaque_binary_without_raw_fallback_fails() {
    // A binary artifact with a rendered-only diff has no raw / text fallback — it fails.
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            artifact_kind: M5SnapshotArtifactKind::BinarySnapshot,
            diff_state: M5SnapshotDiffState::MatchesBaseline,
            fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::RawFallbackMissingForOpaqueArtifact)
    );
    // A render-unavailable card with a rendered-only diff also fails.
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            diff_state: M5SnapshotDiffState::RenderUnavailable,
            fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::RawFallbackMissingForOpaqueArtifact)
    );
}

#[test]
fn matched_baseline_needs_no_acceptance_but_keeps_actions() {
    let resolved = resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
        diff_state: M5SnapshotDiffState::MatchesBaseline,
        fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
        scope_dimensions: vec![M5SnapshotScopeDimension::Serializer],
        ..diff_detected_card()
    })
    .expect("resolves");
    assert_eq!(
        resolved.review_posture,
        M5SnapshotReviewPosture::MatchesBaselineCard
    );
    assert!(!resolved.is_acceptance_decision);
    assert!(!resolved
        .available_actions
        .contains(&M5SnapshotCardAction::AcceptBaseline));
    assert!(resolved
        .available_actions
        .contains(&M5SnapshotCardAction::OpenRawFallback));
    assert!(resolved
        .available_actions
        .contains(&M5SnapshotCardAction::ExportSnapshotReview));
}

#[test]
fn snapshot_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            card_identity_ref: "  ".to_owned(),
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::EmptyCardIdentity)
    );
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            baseline_ref: "".to_owned(),
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::EmptyBaselineReference)
    );
    assert_eq!(
        resolve_snapshot_review_card(&M5SnapshotCardResolutionInput {
            baseline_ref: "baseline:https://ci.example/golden".to_owned(),
            ..diff_detected_card()
        }),
        Err(M5SnapshotCardResolutionError::ForbiddenSnapshotMaterial)
    );
}

// ---- coverage-import-merge-sheet resolver --------------------------------

#[test]
fn shard_omission_sheet_exposes_excluded_runs() {
    let resolved = resolve_coverage_import_merge_sheet(&shard_omission_sheet()).expect("resolves");
    assert_eq!(
        resolved.merge_posture,
        M5CoverageMergePosture::ShardOmissionSheet
    );
    assert!(resolved.exposes_omitted_shards);
    assert!(resolved.discloses_metric_dimension);
    assert!(resolved.is_imported);
    assert!(!resolved.is_exact_current_truth);
    assert!(resolved.needs_attention);
    assert!(resolved
        .available_actions
        .contains(&M5MergeSheetAction::OpenIncompatibleReport));
    assert_eq!(resolved.excluded_runs, vec!["ci-shard-b", "ci-shard-c"]);
}

#[test]
fn merge_posture_is_one_to_one_with_resolution_state() {
    for state in M5MergeResolutionState::ALL {
        let (excluded, exact) = match state {
            // Omission postures must name an excluded run to resolve.
            M5MergeResolutionState::ShardOmissionDetected
            | M5MergeResolutionState::PartialMerge => (vec!["shard-x".to_owned()], false),
            // Only a clean merge may claim exact current truth.
            M5MergeResolutionState::MergedClean => (vec![], true),
            _ => (vec![], false),
        };
        let resolved = resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            merge_resolution: state,
            excluded_runs: excluded,
            claims_exact_current_truth: exact,
            is_stale: false,
            is_incompatible: false,
            ..shard_omission_sheet()
        })
        .expect("resolves");
        assert_eq!(resolved.merge_posture.merge_resolution(), state);
        assert!(resolved.discloses_metric_dimension);
    }
}

#[test]
fn omitted_shards_without_disclosure_fails() {
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            merge_resolution: M5MergeResolutionState::ShardOmissionDetected,
            excluded_runs: vec![],
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::OmittedShardsWithoutDisclosure)
    );
}

#[test]
fn exact_truth_with_unresolved_warnings_fails() {
    // A shard omission cannot be claimed as exact current truth.
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            claims_exact_current_truth: true,
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::ExactTruthWithUnresolvedWarnings)
    );
    // A stale but otherwise clean merge cannot be exact current truth either.
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            merge_resolution: M5MergeResolutionState::MergedClean,
            excluded_runs: vec![],
            is_stale: true,
            claims_exact_current_truth: true,
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::ExactTruthWithUnresolvedWarnings)
    );
}

#[test]
fn clean_merge_may_be_exact_current_truth() {
    let resolved = resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
        import_source: M5CoverageImportSource::LocalRun,
        merge_resolution: M5MergeResolutionState::MergedClean,
        excluded_runs: vec![],
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        is_stale: false,
        is_incompatible: false,
        claims_exact_current_truth: true,
        ..shard_omission_sheet()
    })
    .expect("resolves");
    assert_eq!(
        resolved.merge_posture,
        M5CoverageMergePosture::MergedCleanSheet
    );
    assert!(resolved.is_exact_current_truth);
    assert!(resolved.exact_truth_is_qualified);
    assert!(!resolved.exposes_omitted_shards);
    assert!(!resolved.needs_attention);
    assert!(!resolved.is_imported);
}

#[test]
fn merge_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            sheet_identity_ref: "  ".to_owned(),
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::EmptySheetIdentity)
    );
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            build_identity_ref: "".to_owned(),
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::EmptyCommitOrBuildIdentity)
    );
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            included_runs: vec![],
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::EmptyRunScope)
    );
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            metric_kinds: vec![],
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::EmptyMetricSupport)
    );
    assert_eq!(
        resolve_coverage_import_merge_sheet(&M5MergeSheetResolutionInput {
            included_runs: vec!["run bearer token".to_owned()],
            ..shard_omission_sheet()
        }),
        Err(M5MergeSheetResolutionError::ForbiddenMergeMaterial)
    );
}

// ---- packet --------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_snapshot_merge_components_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SNAPSHOT_MERGE_COMPONENTS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_snapshot_merge_components_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5SnapshotMergeComponentConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5SnapshotMergeComponentConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_snapshot_merge_components_packet();
    for row in &packet.rows {
        for part in M5SnapshotCardAnatomyPart::MANDATORY {
            assert!(row.snapshot_anatomy_parts.contains(&part));
        }
        for part in M5MergeSheetAnatomyPart::MANDATORY {
            assert!(row.merge_anatomy_parts.contains(&part));
        }
        for field in M5SnapshotCardExportField::MANDATORY {
            assert!(row.snapshot_export_fields.contains(&field));
        }
        for field in M5MergeSheetExportField::MANDATORY {
            assert!(row.merge_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.snapshot_examples.is_empty());
        assert!(!row.merge_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_snapshot_merge_components_packet();
    let snapshots: Vec<&M5SnapshotCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.snapshot_examples.iter())
        .collect();
    let merges: Vec<&M5MergeSheetResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.merge_examples.iter())
        .collect();

    for posture in M5SnapshotReviewPosture::ALL {
        assert!(
            snapshots
                .iter()
                .any(|c| c.resolved.review_posture == posture),
            "no example exercises snapshot posture {}",
            posture.as_str()
        );
    }
    for action in M5SnapshotCardAction::ALL {
        assert!(
            snapshots
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises snapshot action {}",
            action.as_str()
        );
    }
    for posture in M5CoverageMergePosture::ALL {
        assert!(
            merges.iter().any(|c| c.resolved.merge_posture == posture),
            "no example exercises merge posture {}",
            posture.as_str()
        );
    }
    for action in M5MergeSheetAction::ALL {
        assert!(
            merges
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises merge action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_snapshot_merge_components_packet();
    for row in &packet.rows {
        for case in &row.snapshot_examples {
            assert!(
                case.is_self_consistent(),
                "snapshot case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "snapshot case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.merge_examples {
            assert!(
                case.is_self_consistent(),
                "merge case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "merge case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5SnapshotMergeComponentConsumerSurface::CoverageImportMergePanel
    });
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.vocabulary_set.snapshot_review_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::VocabularySetDrift));
}

#[test]
fn mandatory_snapshot_anatomy_missing_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0]
        .snapshot_anatomy_parts
        .retain(|p| *p != M5SnapshotCardAnatomyPart::ScopeCue);
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MandatorySnapshotAnatomyMissing));
}

#[test]
fn mandatory_merge_anatomy_missing_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0]
        .merge_anatomy_parts
        .retain(|p| *p != M5MergeSheetAnatomyPart::LineVersusBranchCue);
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MandatoryMergeAnatomyMissing));
}

#[test]
fn mandatory_snapshot_export_missing_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0]
        .snapshot_export_fields
        .retain(|f| *f != M5SnapshotCardExportField::FallbackMode);
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MandatorySnapshotExportMissing));
}

#[test]
fn mandatory_merge_export_missing_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0]
        .merge_export_fields
        .retain(|f| *f != M5MergeSheetExportField::IncludedRuns);
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MandatoryMergeExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0].snapshot_examples[0]
        .resolved
        .is_acceptance_decision = false;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[1].merge_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ExampleMissing));
}

#[test]
fn snapshot_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    let matched = M5SnapshotCardResolutionCase::resolved(M5SnapshotCardResolutionInput {
        diff_state: M5SnapshotDiffState::MatchesBaseline,
        fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
        ..diff_detected_card()
    });
    for row in &mut packet.rows {
        row.snapshot_examples = vec![matched.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::SnapshotPostureCoverageUnproven));
}

#[test]
fn merge_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    let clean = M5MergeSheetResolutionCase::resolved(M5MergeSheetResolutionInput {
        import_source: M5CoverageImportSource::LocalRun,
        merge_resolution: M5MergeResolutionState::MergedClean,
        excluded_runs: vec![],
        is_stale: false,
        is_incompatible: false,
        claims_exact_current_truth: false,
        ..shard_omission_sheet()
    });
    for row in &mut packet.rows {
        row.merge_examples = vec![clean.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MergePostureCoverageUnproven));
}

#[test]
fn acceptance_scope_disclosure_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    // Replace every snapshot example with a matched baseline so no scoped acceptance is proven.
    let matched = M5SnapshotCardResolutionCase::resolved(M5SnapshotCardResolutionInput {
        diff_state: M5SnapshotDiffState::MatchesBaseline,
        fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
        ..diff_detected_card()
    });
    for row in &mut packet.rows {
        row.snapshot_examples = vec![matched.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::AcceptanceScopeDisclosureUnproven));
}

#[test]
fn raw_fallback_disclosure_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    // Replace every snapshot example with a matched, non-opaque baseline so the opaque-fallback
    // proof fires.
    let matched = M5SnapshotCardResolutionCase::resolved(M5SnapshotCardResolutionInput {
        artifact_kind: M5SnapshotArtifactKind::TextSerializerSnapshot,
        diff_state: M5SnapshotDiffState::MatchesBaseline,
        fallback_mode: M5SnapshotFallbackMode::RenderedDiff,
        ..diff_detected_card()
    });
    for row in &mut packet.rows {
        row.snapshot_examples = vec![matched.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::RawFallbackDisclosureUnproven));
}

#[test]
fn omission_disclosure_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    // Replace every merge example with a clean merge so no disclosed omission is proven.
    let clean = M5MergeSheetResolutionCase::resolved(M5MergeSheetResolutionInput {
        import_source: M5CoverageImportSource::LocalRun,
        merge_resolution: M5MergeResolutionState::MergedClean,
        excluded_runs: vec![],
        is_stale: false,
        is_incompatible: false,
        claims_exact_current_truth: false,
        ..shard_omission_sheet()
    });
    for row in &mut packet.rows {
        row.merge_examples = vec![clean.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::OmissionDisclosureUnproven));
}

#[test]
fn import_source_coverage_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    // Replace every merge example with an imported-CI one so local / cached / stale go uncovered.
    let imported = M5MergeSheetResolutionCase::resolved(M5MergeSheetResolutionInput {
        import_source: M5CoverageImportSource::ImportedCiArtifact,
        ..shard_omission_sheet()
    });
    for row in &mut packet.rows {
        row.merge_examples = vec![imported.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ImportSourceCoverageUnproven));
}

#[test]
fn baseline_identity_coverage_unproven_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    // Replace every snapshot example with a committed-baseline one so imported / pending / missing
    // go uncovered.
    let committed = M5SnapshotCardResolutionCase::resolved(diff_detected_card());
    for row in &mut packet.rows {
        row.snapshot_examples = vec![committed.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::BaselineIdentityCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0].collapses_snapshot_accept_without_scope_or_fallback = true;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet
        .governance_review
        .acceptance_never_blind_without_scope_and_fallback = false;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet
        .consumer_projection
        .ci_and_support_read_same_snapshot_merge_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_snapshot_merge_components_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SnapshotMergeComponentViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_snapshot_merge_components_packet().render_markdown_summary();
    for surface in M5SnapshotMergeComponentConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_snapshot_merge_components_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5SnapshotMergeComponentConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5SnapshotMergeComponentConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_snapshot_merge_components_export()
        .expect("checked M5 snapshot merge components export validates");
    assert_eq!(from_disk.packet_id, M5_SNAPSHOT_MERGE_COMPONENTS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_snapshot_merge_components_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed(),
        seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5SnapshotMergeComponentConsumerSurface::ALL.len()
        );
    }

    let panel = seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed();
    let row = panel
        .rows
        .iter()
        .find(|r| {
            r.consumer_surface == M5SnapshotMergeComponentConsumerSurface::SnapshotReviewPanel
        })
        .expect("snapshot-review-panel row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Preview
    );

    let merge = seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed();
    let row = merge
        .rows
        .iter()
        .find(|r| {
            r.consumer_surface == M5SnapshotMergeComponentConsumerSurface::CoverageImportMergePanel
        })
        .expect("coverage-import-merge-panel row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Beta
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let panel: M5SnapshotMergeComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-snapshot-coverage-import-primitive/snapshot_review_panel_preview_narrowed.json"
    )))
    .expect("snapshot-review-panel fixture parses");
    assert!(panel.validate().is_empty());
    assert_eq!(
        panel,
        seeded_m5_snapshot_merge_components_snapshot_review_panel_preview_narrowed()
    );

    let merge: M5SnapshotMergeComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-snapshot-coverage-import-primitive/coverage_import_merge_panel_beta_narrowed.json"
    )))
    .expect("coverage-import-merge-panel fixture parses");
    assert!(merge.validate().is_empty());
    assert_eq!(
        merge,
        seeded_m5_snapshot_merge_components_coverage_import_merge_panel_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_snapshot_merge_components_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
