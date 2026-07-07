use super::*;

fn restorable_row() -> M5LocalHistoryRowResolutionInput {
    M5LocalHistoryRowResolutionInput {
        snapshot_origin: M5SnapshotOrigin::ManualSave,
        actor_class: M5HistoryActorClass::LocalUser,
        capture_fidelity: M5CaptureFidelity::FullBodySnapshot,
        mutation_class: M5MutationClass::TextEdit,
        retention_posture: M5RetentionPosture::WorkspaceRetained,
        timestamp_label: "2026-07-07T09:15:00Z".to_owned(),
        object_identity: "src/main.rs".to_owned(),
        branch_worktree_label: "feature/x @ main-worktree".to_owned(),
        command_or_trigger: "manual save".to_owned(),
        source_removed: false,
    }
}

fn atomic_card() -> M5CheckpointGroupCardResolutionInput {
    M5CheckpointGroupCardResolutionInput {
        lineage_class: M5CheckpointLineageClass::SingleAction,
        mutation_class: M5MutationClass::TextEdit,
        originating_command: "format on save".to_owned(),
        group_label: "checkpoint: format main.rs".to_owned(),
        file_count: 1,
        risk: M5CheckpointGroupRisk::Reversible,
        export_posture: M5ExportRedactionPosture::FullMetadata,
        touches_managed_files: false,
        restore_path_ready: true,
    }
}

// ---- local-history-row resolver -----------------------------------------

#[test]
fn row_restorable_manual_save_is_openable_with_full_actions() {
    let resolved = resolve_local_history_row(&restorable_row()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5LocalHistoryRowPosture::RestorableSnapshot
    );
    assert!(resolved.can_restore);
    assert!(resolved.is_openable);
    assert!(!resolved.is_automated);
    assert!(!resolved.needs_attribution);
    assert!(!resolved.needs_attention);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5LocalHistoryRowAction::RevealLineage,
            M5LocalHistoryRowAction::Open,
            M5LocalHistoryRowAction::Compare,
            M5LocalHistoryRowAction::Restore,
            M5LocalHistoryRowAction::ExportEvidence,
        ]
    );
    assert_eq!(resolved.object_identity, "src/main.rs");
}

#[test]
fn row_posture_ladder_is_blocking_first() {
    // Expired wins even over a metadata-only capture.
    let expired = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        retention_posture: M5RetentionPosture::ExpiredPurged,
        capture_fidelity: M5CaptureFidelity::MetadataOnly,
        ..restorable_row()
    })
    .expect("resolves");
    assert_eq!(
        expired.row_posture,
        M5LocalHistoryRowPosture::ExpiredUnrestorable
    );
    assert!(!expired.can_restore);
    assert!(!expired.is_openable);
    assert_eq!(
        expired.available_actions,
        vec![M5LocalHistoryRowAction::RevealLineage]
    );

    // Metadata-only next.
    let metadata = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        capture_fidelity: M5CaptureFidelity::MetadataOnly,
        ..restorable_row()
    })
    .expect("resolves");
    assert_eq!(
        metadata.row_posture,
        M5LocalHistoryRowPosture::MetadataOnlyReference
    );
    assert!(!metadata.can_restore);
    assert!(!metadata.is_openable);

    // Unknown actor next.
    let unattributed = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        actor_class: M5HistoryActorClass::UnknownActor,
        ..restorable_row()
    })
    .expect("resolves");
    assert_eq!(
        unattributed.row_posture,
        M5LocalHistoryRowPosture::UnattributedSnapshot
    );
    assert!(unattributed.can_restore);
    assert!(unattributed.needs_attribution);

    // Purge-pending next.
    let purge = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        retention_posture: M5RetentionPosture::PurgePending,
        ..restorable_row()
    })
    .expect("resolves");
    assert_eq!(
        purge.row_posture,
        M5LocalHistoryRowPosture::PurgePendingSnapshot
    );
    assert!(purge.needs_attention);

    // Automated capture next.
    let automated = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        actor_class: M5HistoryActorClass::AiAgent,
        ..restorable_row()
    })
    .expect("resolves");
    assert_eq!(
        automated.row_posture,
        M5LocalHistoryRowPosture::AutomatedCapture
    );
    assert!(automated.is_automated);
    assert!(automated.needs_attribution);
}

#[test]
fn row_removed_source_is_not_openable_but_still_reveals_lineage() {
    let removed = resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
        source_removed: true,
        ..restorable_row()
    })
    .expect("resolves");
    assert!(!removed.is_openable);
    assert!(!removed
        .available_actions
        .contains(&M5LocalHistoryRowAction::Open));
    assert!(removed
        .available_actions
        .contains(&M5LocalHistoryRowAction::RevealLineage));
    // A removed source still restores from the captured body.
    assert!(removed.can_restore);
    assert!(removed
        .available_actions
        .contains(&M5LocalHistoryRowAction::Restore));
}

#[test]
fn row_rejects_malformed_input() {
    assert_eq!(
        resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
            object_identity: " ".to_owned(),
            ..restorable_row()
        }),
        Err(M5LocalHistoryRowResolutionError::EmptyObjectIdentity)
    );
    assert_eq!(
        resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
            timestamp_label: "".to_owned(),
            ..restorable_row()
        }),
        Err(M5LocalHistoryRowResolutionError::EmptyTimestampLabel)
    );
    assert_eq!(
        resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
            branch_worktree_label: "  ".to_owned(),
            ..restorable_row()
        }),
        Err(M5LocalHistoryRowResolutionError::EmptyBranchWorktreeLabel)
    );
    assert_eq!(
        resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
            command_or_trigger: "".to_owned(),
            ..restorable_row()
        }),
        Err(M5LocalHistoryRowResolutionError::EmptyCommandOrTrigger)
    );
    assert_eq!(
        resolve_local_history_row(&M5LocalHistoryRowResolutionInput {
            object_identity: "s3://bucket/file".to_owned(),
            ..restorable_row()
        }),
        Err(M5LocalHistoryRowResolutionError::ForbiddenRowMaterial)
    );
}

// ---- checkpoint-group-card resolver -------------------------------------

#[test]
fn card_atomic_single_action_restores_with_no_preview_scope() {
    let resolved = resolve_checkpoint_group_card(&atomic_card()).expect("resolves");
    assert_eq!(
        resolved.card_posture,
        M5CheckpointGroupCardPosture::AtomicCheckpoint
    );
    assert!(resolved.can_restore);
    assert!(!resolved.is_multi_file);
    assert!(!resolved.touches_generated_or_managed);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5CheckpointGroupCardAction::RevealLineage,
            M5CheckpointGroupCardAction::CompareGroup,
            M5CheckpointGroupCardAction::Restore,
            M5CheckpointGroupCardAction::Export,
        ]
    );
    assert_eq!(resolved.group_label, "checkpoint: format main.rs");
}

#[test]
fn card_posture_ladder_is_blocking_first() {
    // Restore-blocked wins even over high risk.
    let blocked = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        restore_path_ready: false,
        risk: M5CheckpointGroupRisk::DestructiveOverwrite,
        ..atomic_card()
    })
    .expect("resolves");
    assert_eq!(
        blocked.card_posture,
        M5CheckpointGroupCardPosture::RestoreBlockedGroup
    );
    assert!(!blocked.can_restore);
    assert!(!blocked
        .available_actions
        .contains(&M5CheckpointGroupCardAction::Restore));

    // High risk next.
    let high_risk = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        risk: M5CheckpointGroupRisk::IrreversibleWrites,
        ..atomic_card()
    })
    .expect("resolves");
    assert_eq!(
        high_risk.card_posture,
        M5CheckpointGroupCardPosture::HighRiskGroup
    );
    assert!(high_risk.needs_review);

    // Generated / managed next.
    let generated = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        mutation_class: M5MutationClass::GeneratedArtifact,
        ..atomic_card()
    })
    .expect("resolves");
    assert_eq!(
        generated.card_posture,
        M5CheckpointGroupCardPosture::GeneratedArtifactGroup
    );
    assert!(generated.touches_generated_or_managed);
    assert!(generated
        .available_actions
        .contains(&M5CheckpointGroupCardAction::PreviewScope));

    // Imported next.
    let imported = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        lineage_class: M5CheckpointLineageClass::ImportedCheckpoint,
        ..atomic_card()
    })
    .expect("resolves");
    assert_eq!(
        imported.card_posture,
        M5CheckpointGroupCardPosture::ImportedGroup
    );

    // Multi-file next.
    let multi = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        file_count: 4,
        ..atomic_card()
    })
    .expect("resolves");
    assert_eq!(
        multi.card_posture,
        M5CheckpointGroupCardPosture::MultiFileGroup
    );
    assert!(multi.is_multi_file);
    assert!(multi
        .available_actions
        .contains(&M5CheckpointGroupCardAction::PreviewScope));
}

#[test]
fn card_export_blocked_omits_export_action() {
    let resolved = resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
        export_posture: M5ExportRedactionPosture::ExportBlocked,
        ..atomic_card()
    })
    .expect("resolves");
    assert!(!resolved.is_exportable);
    assert!(!resolved
        .available_actions
        .contains(&M5CheckpointGroupCardAction::Export));
}

#[test]
fn card_rejects_malformed_input() {
    assert_eq!(
        resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
            originating_command: " ".to_owned(),
            ..atomic_card()
        }),
        Err(M5CheckpointGroupCardResolutionError::EmptyOriginatingCommand)
    );
    assert_eq!(
        resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
            group_label: "".to_owned(),
            ..atomic_card()
        }),
        Err(M5CheckpointGroupCardResolutionError::EmptyGroupLabel)
    );
    assert_eq!(
        resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
            file_count: 0,
            ..atomic_card()
        }),
        Err(M5CheckpointGroupCardResolutionError::ZeroFileCount)
    );
    assert_eq!(
        resolve_checkpoint_group_card(&M5CheckpointGroupCardResolutionInput {
            group_label: "checkpoint https://leak.test".to_owned(),
            ..atomic_card()
        }),
        Err(M5CheckpointGroupCardResolutionError::ForbiddenCardMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_local_history_row_group_card_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_LOCAL_HISTORY_ROW_GROUP_CARD_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_local_history_row_group_card_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5LocalHistoryCheckpointConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5LocalHistoryCheckpointConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_local_history_row_group_card_packet();
    for row in &packet.rows {
        for part in M5LocalHistoryRowAnatomyPart::MANDATORY {
            assert!(row.row_anatomy_parts.contains(&part));
        }
        for part in M5CheckpointGroupCardAnatomyPart::MANDATORY {
            assert!(row.card_anatomy_parts.contains(&part));
        }
        for field in M5LocalHistoryRowExportField::MANDATORY {
            assert!(row.row_export_fields.contains(&field));
        }
        for field in M5CheckpointGroupCardExportField::MANDATORY {
            assert!(row.card_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5HistoryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.row_examples.is_empty());
        assert!(!row.card_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_local_history_row_group_card_packet();
    let rows: Vec<&M5LocalHistoryRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.row_examples.iter())
        .collect();
    let cards: Vec<&M5CheckpointGroupCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.card_examples.iter())
        .collect();

    for posture in M5LocalHistoryRowPosture::ALL {
        assert!(
            rows.iter().any(|c| c.resolved.row_posture == posture),
            "no row example exercises posture {}",
            posture.as_str()
        );
    }
    for posture in M5CheckpointGroupCardPosture::ALL {
        assert!(
            cards.iter().any(|c| c.resolved.card_posture == posture),
            "no card example exercises posture {}",
            posture.as_str()
        );
    }
    for action in M5LocalHistoryRowAction::ALL {
        assert!(
            rows.iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no row example exercises action {}",
            action.as_str()
        );
    }
    for action in M5CheckpointGroupCardAction::ALL {
        assert!(
            cards
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no card example exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_local_history_row_group_card_packet();
    for row in &packet.rows {
        for case in &row.row_examples {
            assert!(
                case.is_self_consistent(),
                "row case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "row case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.card_examples {
            assert!(
                case.is_self_consistent(),
                "card case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "card case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows.retain(|row| {
        row.consumer_surface != M5LocalHistoryCheckpointConsumerSurface::ImporterActions
    });
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.vocabulary_set.row_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::VocabularySetDrift));
}

#[test]
fn mandatory_row_anatomy_missing_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[0]
        .row_anatomy_parts
        .retain(|p| *p != M5LocalHistoryRowAnatomyPart::ActorCue);
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::MandatoryRowAnatomyMissing));
}

#[test]
fn mandatory_card_export_missing_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[0]
        .card_export_fields
        .retain(|f| *f != M5CheckpointGroupCardExportField::FileCount);
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::MandatoryCardExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[0].row_examples[0].resolved.can_restore = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::ExampleResolutionDrift));
}

#[test]
fn row_example_missing_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[1].row_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowExampleMissing));
}

#[test]
fn row_restore_coverage_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    // Replace every row example with a plainly restorable one so the non-restorable half
    // of the coverage lint fires.
    for row in &mut packet.rows {
        row.row_examples = vec![M5LocalHistoryRowResolutionCase::resolved(restorable_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowRestoreCoverageUnproven));
}

#[test]
fn row_actor_coverage_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    // Replace every row example with an attributed restorable one so the needs-attention
    // half of the coverage lint fires.
    for row in &mut packet.rows {
        row.row_examples = vec![M5LocalHistoryRowResolutionCase::resolved(restorable_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowActorCoverageUnproven));
}

#[test]
fn row_open_reveal_coverage_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    // Replace every row example with an openable one so the removed-source half of the
    // coverage lint fires.
    for row in &mut packet.rows {
        row.row_examples = vec![M5LocalHistoryRowResolutionCase::resolved(restorable_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowOpenRevealCoverageUnproven));
}

#[test]
fn row_automated_disclosure_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    // Replace every row example with a human-actor one so the automated-capture lint
    // fires.
    for row in &mut packet.rows {
        row.row_examples = vec![M5LocalHistoryRowResolutionCase::resolved(restorable_row())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowAutomatedDisclosureUnproven));
}

#[test]
fn card_restore_coverage_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    for row in &mut packet.rows {
        row.card_examples = vec![M5CheckpointGroupCardResolutionCase::resolved(atomic_card())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::CardRestoreCoverageUnproven));
}

#[test]
fn card_managed_caveat_coverage_unproven_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    for row in &mut packet.rows {
        row.card_examples = vec![M5CheckpointGroupCardResolutionCase::resolved(atomic_card())];
    }
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::CardManagedCaveatCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[0].hides_capture_or_managed_caveat = true;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.governance_review.row_posture_never_masks_unrestorable = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.consumer_projection.card_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_local_history_row_group_card_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5LocalHistoryRowGroupCardViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_local_history_row_group_card_packet().render_markdown_summary();
    for surface in M5LocalHistoryCheckpointConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_local_history_row_group_card_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5LocalHistoryCheckpointConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5LocalHistoryCheckpointConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_local_history_row_group_card_export()
        .expect("checked M5 row/card primitive export validates");
    assert_eq!(from_disk.packet_id, M5_LOCAL_HISTORY_ROW_GROUP_CARD_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_local_history_row_group_card_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_local_history_row_group_card_importer_actions_preview_narrowed(),
        seeded_m5_local_history_row_group_card_ai_apply_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5LocalHistoryCheckpointConsumerSurface::ALL.len()
        );
    }

    let importer = seeded_m5_local_history_row_group_card_importer_actions_preview_narrowed();
    let row = importer
        .rows
        .iter()
        .find(|r| {
            r.consumer_surface == M5LocalHistoryCheckpointConsumerSurface::ImporterActions
        })
        .expect("importer-actions row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Preview);

    let ai_apply = seeded_m5_local_history_row_group_card_ai_apply_beta_narrowed();
    let row = ai_apply
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5LocalHistoryCheckpointConsumerSurface::AiApplyReview)
        .expect("ai-apply-review row present");
    assert_eq!(row.qualification, M5HistoryQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let importer: M5LocalHistoryRowGroupCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-local-history-row-and-checkpoint-group-card-primitive/importer_actions_preview_narrowed.json"
    )))
    .expect("importer-actions fixture parses");
    assert!(importer.validate().is_empty());
    assert_eq!(
        importer,
        seeded_m5_local_history_row_group_card_importer_actions_preview_narrowed()
    );

    let ai_apply: M5LocalHistoryRowGroupCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-local-history-row-and-checkpoint-group-card-primitive/ai_apply_beta_narrowed.json"
    )))
    .expect("ai-apply fixture parses");
    assert!(ai_apply.validate().is_empty());
    assert_eq!(
        ai_apply,
        seeded_m5_local_history_row_group_card_ai_apply_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_local_history_row_group_card_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
