use super::*;

fn clean_preview() -> M5RestorePreviewCardResolutionInput {
    M5RestorePreviewCardResolutionInput {
        mutation_class: M5MutationClass::TextEdit,
        capture_fidelity: M5CaptureFidelity::FullBodySnapshot,
        drift_state: M5RestoreDriftState::CleanApply,
        managed_caveat: M5ManagedFileCaveat::Unmanaged,
        offered_granularity: M5RestoreGranularity::PerHunk,
        retention_posture: M5RetentionPosture::WorkspaceRetained,
        export_posture: M5ExportRedactionPosture::FullMetadata,
        past_state_label: "main.rs at 2026-07-07T09:15:00Z".to_owned(),
        current_state_label: "main.rs working tree".to_owned(),
        object_identity: "src/main.rs".to_owned(),
        selection_valid: true,
        restore_path_ready: true,
    }
}

fn whole_scope_selector() -> M5RestoreGranularitySelectorResolutionInput {
    M5RestoreGranularitySelectorResolutionInput {
        drift_state: M5RestoreDriftState::CleanApply,
        is_multi_file: false,
        selection_valid: false,
        touches_generated_or_managed: false,
        restore_path_ready: true,
        scope_label: "restore scope: main.rs".to_owned(),
    }
}

// ---- restore-preview-card resolver --------------------------------------

#[test]
fn preview_clean_apply_offers_whole_file_and_selected_range() {
    let resolved = resolve_restore_preview_card(&clean_preview()).expect("resolves");
    assert_eq!(
        resolved.preview_posture,
        M5RestorePreviewPosture::CleanRestorePreview
    );
    assert!(resolved.can_restore);
    assert!(!resolved.has_external_drift);
    assert!(!resolved.touches_generated_or_managed);
    assert!(resolved.creates_new_checkpoint);
    assert!(resolved.preserves_history_trail);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5RestorePreviewAction::InspectDiff,
            M5RestorePreviewAction::RestoreWholeFile,
            M5RestorePreviewAction::RestoreSelectedRange,
            M5RestorePreviewAction::ExportAsPatch,
            M5RestorePreviewAction::ExportAsEvidence,
        ]
    );
    assert_eq!(resolved.object_identity, "src/main.rs");
}

#[test]
fn preview_posture_ladder_is_blocking_first() {
    // Restore-blocked wins even over an external-drift baseline.
    let blocked = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        restore_path_ready: false,
        drift_state: M5RestoreDriftState::ExternalDrift,
        ..clean_preview()
    })
    .expect("resolves");
    assert_eq!(
        blocked.preview_posture,
        M5RestorePreviewPosture::RestoreBlockedPreview
    );
    assert!(!blocked.can_restore);
    // A blocked restore still inspects and may still export as evidence, but never offers
    // a restore action.
    assert!(blocked
        .available_actions
        .contains(&M5RestorePreviewAction::InspectDiff));
    assert!(!blocked
        .available_actions
        .contains(&M5RestorePreviewAction::RestoreWholeFile));
    assert!(!blocked
        .available_actions
        .contains(&M5RestorePreviewAction::RestoreSelectedRange));

    // Conflict next.
    let conflict = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        drift_state: M5RestoreDriftState::ConflictPending,
        ..clean_preview()
    })
    .expect("resolves");
    assert_eq!(
        conflict.preview_posture,
        M5RestorePreviewPosture::ConflictPreview
    );
    assert!(!conflict.can_restore);
    assert!(conflict
        .available_actions
        .contains(&M5RestorePreviewAction::ResolveConflict));
    assert!(!conflict
        .available_actions
        .contains(&M5RestorePreviewAction::RestoreWholeFile));

    // External drift next (wins over a managed caveat).
    let drift = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        drift_state: M5RestoreDriftState::ExternalDrift,
        managed_caveat: M5ManagedFileCaveat::GeneratedFile,
        ..clean_preview()
    })
    .expect("resolves");
    assert_eq!(
        drift.preview_posture,
        M5RestorePreviewPosture::ExternalDriftPreview
    );
    assert!(drift.has_external_drift);
    assert!(drift.can_restore);

    // Managed / generated next.
    let managed = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        managed_caveat: M5ManagedFileCaveat::ManagedLockfile,
        ..clean_preview()
    })
    .expect("resolves");
    assert_eq!(
        managed.preview_posture,
        M5RestorePreviewPosture::ManagedFilePreview
    );
    assert!(managed.touches_generated_or_managed);

    // Local edits next.
    let local = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        drift_state: M5RestoreDriftState::LocalEditsPresent,
        ..clean_preview()
    })
    .expect("resolves");
    assert_eq!(
        local.preview_posture,
        M5RestorePreviewPosture::LocalDriftPreview
    );
    assert!(local.can_restore);
}

#[test]
fn preview_generated_mutation_touches_managed() {
    let generated = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        mutation_class: M5MutationClass::GeneratedArtifact,
        managed_caveat: M5ManagedFileCaveat::Unmanaged,
        ..clean_preview()
    })
    .expect("resolves");
    assert!(generated.touches_generated_or_managed);
    assert_eq!(
        generated.preview_posture,
        M5RestorePreviewPosture::ManagedFilePreview
    );
}

#[test]
fn preview_whole_snapshot_granularity_offers_no_selected_range() {
    let whole = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        offered_granularity: M5RestoreGranularity::WholeSnapshot,
        selection_valid: true,
        ..clean_preview()
    })
    .expect("resolves");
    assert!(!whole.offers_partial_granularity);
    assert!(!whole
        .available_actions
        .contains(&M5RestorePreviewAction::RestoreSelectedRange));
    assert!(whole
        .available_actions
        .contains(&M5RestorePreviewAction::RestoreWholeFile));
}

#[test]
fn preview_export_blocked_omits_export_actions() {
    let resolved = resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
        export_posture: M5ExportRedactionPosture::ExportBlocked,
        ..clean_preview()
    })
    .expect("resolves");
    assert!(!resolved.is_exportable);
    assert!(!resolved
        .available_actions
        .contains(&M5RestorePreviewAction::ExportAsPatch));
    assert!(!resolved
        .available_actions
        .contains(&M5RestorePreviewAction::ExportAsEvidence));
}

#[test]
fn preview_rejects_malformed_input() {
    assert_eq!(
        resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
            object_identity: " ".to_owned(),
            ..clean_preview()
        }),
        Err(M5RestorePreviewCardResolutionError::EmptyObjectIdentity)
    );
    assert_eq!(
        resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
            past_state_label: "".to_owned(),
            ..clean_preview()
        }),
        Err(M5RestorePreviewCardResolutionError::EmptyPastStateLabel)
    );
    assert_eq!(
        resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
            current_state_label: "  ".to_owned(),
            ..clean_preview()
        }),
        Err(M5RestorePreviewCardResolutionError::EmptyCurrentStateLabel)
    );
    assert_eq!(
        resolve_restore_preview_card(&M5RestorePreviewCardResolutionInput {
            object_identity: "s3://bucket/file".to_owned(),
            ..clean_preview()
        }),
        Err(M5RestorePreviewCardResolutionError::ForbiddenPreviewMaterial)
    );
}

// ---- restore-granularity-selector resolver ------------------------------

#[test]
fn selector_whole_scope_applies_all_with_no_narrowing() {
    let resolved = resolve_restore_granularity_selector(&whole_scope_selector()).expect("resolves");
    assert_eq!(
        resolved.selector_posture,
        M5RestoreGranularitySelectorPosture::WholeScopeSelector
    );
    assert!(resolved.can_apply);
    assert!(!resolved.can_narrow);
    assert_eq!(resolved.default_mode, M5RestoreSelectionMode::AllChanges);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5RestoreGranularitySelectorAction::InspectScope,
            M5RestoreGranularitySelectorAction::ApplyScope,
        ]
    );
    assert!(resolved.creates_new_checkpoint);
    assert_eq!(resolved.scope_label, "restore scope: main.rs");
}

#[test]
fn selector_posture_ladder_is_blocking_first() {
    // Selector-blocked wins even over a pending conflict.
    let blocked =
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            restore_path_ready: false,
            drift_state: M5RestoreDriftState::ConflictPending,
            ..whole_scope_selector()
        })
        .expect("resolves");
    assert_eq!(
        blocked.selector_posture,
        M5RestoreGranularitySelectorPosture::SelectorBlocked
    );
    assert!(!blocked.can_apply);
    assert_eq!(
        blocked.available_actions,
        vec![M5RestoreGranularitySelectorAction::InspectScope]
    );

    // Dry-run-only next.
    let dry_run =
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            drift_state: M5RestoreDriftState::ConflictPending,
            ..whole_scope_selector()
        })
        .expect("resolves");
    assert_eq!(
        dry_run.selector_posture,
        M5RestoreGranularitySelectorPosture::DryRunOnlySelector
    );
    assert!(!dry_run.can_apply);
    assert_eq!(dry_run.default_mode, M5RestoreSelectionMode::DryRunOnly);

    // Exclude-generated next.
    let exclude =
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            touches_generated_or_managed: true,
            is_multi_file: true,
            ..whole_scope_selector()
        })
        .expect("resolves");
    assert_eq!(
        exclude.selector_posture,
        M5RestoreGranularitySelectorPosture::ExcludeGeneratedSelector
    );
    assert!(exclude.excludes_generated);
    assert!(exclude
        .available_actions
        .contains(&M5RestoreGranularitySelectorAction::ExcludeGenerated));

    // Range-scoped next.
    let range =
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            selection_valid: true,
            ..whole_scope_selector()
        })
        .expect("resolves");
    assert_eq!(
        range.selector_posture,
        M5RestoreGranularitySelectorPosture::RangeScopedSelector
    );
    assert!(range
        .available_actions
        .contains(&M5RestoreGranularitySelectorAction::NarrowToRange));

    // File-scoped next.
    let file = resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
        is_multi_file: true,
        ..whole_scope_selector()
    })
    .expect("resolves");
    assert_eq!(
        file.selector_posture,
        M5RestoreGranularitySelectorPosture::FileScopedSelector
    );
    assert!(file
        .available_actions
        .contains(&M5RestoreGranularitySelectorAction::NarrowToFiles));
}

#[test]
fn selector_rejects_malformed_input() {
    assert_eq!(
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            scope_label: " ".to_owned(),
            ..whole_scope_selector()
        }),
        Err(M5RestoreGranularitySelectorResolutionError::EmptyScopeLabel)
    );
    assert_eq!(
        resolve_restore_granularity_selector(&M5RestoreGranularitySelectorResolutionInput {
            scope_label: "scope https://leak.test".to_owned(),
            ..whole_scope_selector()
        }),
        Err(M5RestoreGranularitySelectorResolutionError::ForbiddenSelectorMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_restore_preview_granularity_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RESTORE_PREVIEW_GRANULARITY_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_restore_preview_granularity_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5RestorePreviewConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5RestorePreviewConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_restore_preview_granularity_packet();
    for row in &packet.rows {
        for part in M5RestorePreviewAnatomyPart::MANDATORY {
            assert!(row.preview_anatomy_parts.contains(&part));
        }
        for part in M5RestoreGranularitySelectorAnatomyPart::MANDATORY {
            assert!(row.selector_anatomy_parts.contains(&part));
        }
        for field in M5RestorePreviewExportField::MANDATORY {
            assert!(row.preview_export_fields.contains(&field));
        }
        for field in M5RestoreGranularitySelectorExportField::MANDATORY {
            assert!(row.selector_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.preview_examples.is_empty());
        assert!(!row.selector_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_restore_preview_granularity_packet();
    let previews: Vec<&M5RestorePreviewCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.preview_examples.iter())
        .collect();
    let selectors: Vec<&M5RestoreGranularitySelectorResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.selector_examples.iter())
        .collect();

    for posture in M5RestorePreviewPosture::ALL {
        assert!(
            previews
                .iter()
                .any(|c| c.resolved.preview_posture == posture),
            "no preview example exercises posture {}",
            posture.as_str()
        );
    }
    for posture in M5RestoreGranularitySelectorPosture::ALL {
        assert!(
            selectors
                .iter()
                .any(|c| c.resolved.selector_posture == posture),
            "no selector example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5RestorePreviewAction::ALL {
        assert!(
            previews
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no preview example exercises action {}",
            action.as_str()
        );
    }
    for action in M5RestoreGranularitySelectorAction::ALL {
        assert!(
            selectors
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no selector example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_restore_preview_granularity_packet();
    for row in &packet.rows {
        for case in &row.preview_examples {
            assert!(
                case.is_self_consistent(),
                "preview case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "preview case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.selector_examples {
            assert!(
                case.is_self_consistent(),
                "selector case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "selector case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5RestorePreviewConsumerSurface::ImportRestore);
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.vocabulary_set.preview_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::VocabularySetDrift));
}

#[test]
fn mandatory_preview_anatomy_missing_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[0]
        .preview_anatomy_parts
        .retain(|p| *p != M5RestorePreviewAnatomyPart::DriftBaselineCue);
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::MandatoryPreviewAnatomyMissing));
}

#[test]
fn mandatory_selector_export_missing_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[0]
        .selector_export_fields
        .retain(|f| *f != M5RestoreGranularitySelectorExportField::CreatesNewCheckpoint);
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::MandatorySelectorExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[0].preview_examples[0].resolved.can_restore = false;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::ExampleResolutionDrift));
}

#[test]
fn preview_example_missing_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[1].preview_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::PreviewExampleMissing));
}

#[test]
fn preview_drift_coverage_unproven_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    // Replace every preview example with a clean one so the external-drift half of the
    // coverage lint fires.
    for row in &mut packet.rows {
        row.preview_examples = vec![M5RestorePreviewCardResolutionCase::resolved(clean_preview())];
    }
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::PreviewDriftCoverageUnproven));
}

#[test]
fn preview_managed_caveat_coverage_unproven_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    for row in &mut packet.rows {
        row.preview_examples = vec![M5RestorePreviewCardResolutionCase::resolved(clean_preview())];
    }
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::PreviewManagedCaveatCoverageUnproven));
}

#[test]
fn preview_restore_coverage_unproven_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    for row in &mut packet.rows {
        row.preview_examples = vec![M5RestorePreviewCardResolutionCase::resolved(clean_preview())];
    }
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::PreviewRestoreCoverageUnproven));
}

#[test]
fn selector_scope_coverage_unproven_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    // Replace every selector example with a whole-scope one so both the can-narrow and the
    // dry-run-only halves of the coverage lint fire.
    for row in &mut packet.rows {
        row.selector_examples = vec![M5RestoreGranularitySelectorResolutionCase::resolved(
            whole_scope_selector(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::SelectorScopeCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[0].erases_history_trail = true;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.governance_review.history_trail_never_erased = false;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet
        .consumer_projection
        .selector_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5RestorePreviewGranularityViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_restore_preview_granularity_packet().render_markdown_summary();
    for surface in M5RestorePreviewConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_restore_preview_granularity_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5RestorePreviewConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5RestorePreviewConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_restore_preview_granularity_export()
        .expect("checked M5 preview/selector primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_RESTORE_PREVIEW_GRANULARITY_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_restore_preview_granularity_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_restore_preview_granularity_import_restore_preview_narrowed(),
        seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5RestorePreviewConsumerSurface::ALL.len()
        );
    }

    let import = seeded_m5_restore_preview_granularity_import_restore_preview_narrowed();
    let row = import
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5RestorePreviewConsumerSurface::ImportRestore)
        .expect("import-restore row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);

    let ai_apply = seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed();
    let row = ai_apply
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5RestorePreviewConsumerSurface::AiApplyRestore)
        .expect("ai-apply-restore row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let import: M5RestorePreviewGranularityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-restore-preview-card-and-restore-granularity-selector-primitive/import_restore_preview_narrowed.json"
    )))
    .expect("import-restore fixture parses");
    assert!(import.validate().is_empty());
    assert_eq!(
        import,
        seeded_m5_restore_preview_granularity_import_restore_preview_narrowed()
    );

    let ai_apply: M5RestorePreviewGranularityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-restore-preview-card-and-restore-granularity-selector-primitive/ai_apply_restore_beta_narrowed.json"
    )))
    .expect("ai-apply-restore fixture parses");
    assert!(ai_apply.validate().is_empty());
    assert_eq!(
        ai_apply,
        seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_restore_preview_granularity_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
