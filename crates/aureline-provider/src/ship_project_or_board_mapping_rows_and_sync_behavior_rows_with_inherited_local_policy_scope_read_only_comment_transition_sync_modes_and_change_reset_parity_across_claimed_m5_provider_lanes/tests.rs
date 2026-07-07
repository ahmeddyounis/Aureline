use super::*;

fn explicit_issue_mapping() -> M5MappingRowResolutionInput {
    M5MappingRowResolutionInput {
        target_kind: M5MappingTargetKind::IssueTrackerProject,
        mapping_origin: M5MappingOriginClass::ExplicitUserChoice,
        provider_project_label: "acme-eng issues (chosen)".to_owned(),
        repo_workspace_relation: "repo acme/eng ↔ issues project".to_owned(),
        lock_note: None,
        mapping_ref: "mapping:acme-eng:issues:explicit".to_owned(),
    }
}

fn live_full_sync() -> M5SyncRowResolutionInput {
    M5SyncRowResolutionInput {
        sync_mode: M5ProviderSyncMode::LiveBidirectional,
        write_scope: M5ProviderWriteScope::FullWrite,
        queued_draft_state: M5QueuedDraftState::QueuedPublish,
        sync_label: "acme-eng live sync (queued)".to_owned(),
        sync_ref: "sync:acme-eng:live-full".to_owned(),
    }
}

// ---- project/board mapping-row resolver ---------------------------------

#[test]
fn explicit_choice_is_local_scope_with_change_and_reset() {
    let resolved = resolve_project_board_mapping_row(&explicit_issue_mapping()).expect("resolves");
    assert_eq!(resolved.mapping_scope, M5MappingScopeClass::LocalScope);
    assert_eq!(
        resolved.row_posture,
        M5MappingRowPosture::ExplicitUserChoiceRow
    );
    assert!(resolved.shows_explicit_destination);
    assert!(!resolved.is_policy_locked);
    assert!(!resolved.assumes_default_destination_silently);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MappingRowAction::RevealMapping,
            M5MappingRowAction::ChangeMapping,
            M5MappingRowAction::ResetMapping,
            M5MappingRowAction::ExportRow,
        ]
    );
}

#[test]
fn mapping_posture_and_scope_map_one_to_one_from_origin() {
    let cases = [
        (
            M5MappingOriginClass::ExplicitUserChoice,
            M5MappingRowPosture::ExplicitUserChoiceRow,
            M5MappingScopeClass::LocalScope,
        ),
        (
            M5MappingOriginClass::InheritedDefault,
            M5MappingRowPosture::InheritedDefaultRow,
            M5MappingScopeClass::InheritedScope,
        ),
        (
            M5MappingOriginClass::AutoMatched,
            M5MappingRowPosture::AutoMatchedRow,
            M5MappingScopeClass::InheritedScope,
        ),
        (
            M5MappingOriginClass::ImportedConfig,
            M5MappingRowPosture::ImportedConfigRow,
            M5MappingScopeClass::InheritedScope,
        ),
        (
            M5MappingOriginClass::PolicyPinned,
            M5MappingRowPosture::PolicyPinnedRow,
            M5MappingScopeClass::PolicyScope,
        ),
        (
            M5MappingOriginClass::UnmappedOrigin,
            M5MappingRowPosture::UnmappedRow,
            M5MappingScopeClass::UnmappedScope,
        ),
    ];
    for (origin, expected_posture, expected_scope) in cases {
        let lock_note = matches!(origin, M5MappingOriginClass::PolicyPinned)
            .then(|| "pinned by admin policy".to_owned());
        let resolved = resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            mapping_origin: origin,
            lock_note,
            ..explicit_issue_mapping()
        })
        .expect("resolves");
        assert_eq!(resolved.row_posture, expected_posture);
        assert_eq!(resolved.mapping_scope, expected_scope);
    }
}

#[test]
fn policy_pinned_mapping_is_locked_and_blocks_change() {
    let resolved = resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
        mapping_origin: M5MappingOriginClass::PolicyPinned,
        lock_note: Some("pinned by org admin policy".to_owned()),
        ..explicit_issue_mapping()
    })
    .expect("resolves");
    assert!(resolved.is_policy_locked);
    assert!(!resolved
        .available_actions
        .contains(&M5MappingRowAction::ChangeMapping));
    assert!(!resolved
        .available_actions
        .contains(&M5MappingRowAction::ResetMapping));
    assert_eq!(
        resolved.available_actions,
        vec![
            M5MappingRowAction::RevealMapping,
            M5MappingRowAction::ExportRow,
        ]
    );
}

#[test]
fn inherited_default_offers_change_but_no_reset() {
    let resolved = resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
        mapping_origin: M5MappingOriginClass::InheritedDefault,
        ..explicit_issue_mapping()
    })
    .expect("resolves");
    assert!(resolved
        .available_actions
        .contains(&M5MappingRowAction::ChangeMapping));
    assert!(!resolved
        .available_actions
        .contains(&M5MappingRowAction::ResetMapping));
}

#[test]
fn unmapped_row_flags_unmapped_and_never_defaults() {
    let resolved = resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
        target_kind: M5MappingTargetKind::UnmappedTarget,
        mapping_origin: M5MappingOriginClass::UnmappedOrigin,
        ..explicit_issue_mapping()
    })
    .expect("resolves");
    assert_eq!(resolved.row_posture, M5MappingRowPosture::UnmappedRow);
    assert!(!resolved.shows_explicit_destination);
    assert!(!resolved.assumes_default_destination_silently);
}

#[test]
fn policy_pinned_without_lock_note_is_rejected() {
    assert_eq!(
        resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            mapping_origin: M5MappingOriginClass::PolicyPinned,
            lock_note: None,
            ..explicit_issue_mapping()
        }),
        Err(M5MappingRowResolutionError::MissingLockNoteForPolicyLock)
    );
}

#[test]
fn mapping_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            provider_project_label: " ".to_owned(),
            ..explicit_issue_mapping()
        }),
        Err(M5MappingRowResolutionError::EmptyProjectLabel)
    );
    assert_eq!(
        resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            repo_workspace_relation: "".to_owned(),
            ..explicit_issue_mapping()
        }),
        Err(M5MappingRowResolutionError::EmptyRelation)
    );
    assert_eq!(
        resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            mapping_ref: "".to_owned(),
            ..explicit_issue_mapping()
        }),
        Err(M5MappingRowResolutionError::EmptyMappingRef)
    );
    assert_eq!(
        resolve_project_board_mapping_row(&M5MappingRowResolutionInput {
            provider_project_label: "board at https://provider.example/x".to_owned(),
            ..explicit_issue_mapping()
        }),
        Err(M5MappingRowResolutionError::ForbiddenMappingMaterial)
    );
}

// ---- sync-behavior-row resolver -----------------------------------------

#[test]
fn live_full_sync_is_full_bidirectional_with_queue() {
    let resolved = resolve_sync_behavior_row(&live_full_sync()).expect("resolves");
    assert_eq!(
        resolved.behavior_class,
        M5SyncBehaviorClass::FullBidirectionalSync
    );
    assert!(resolved.can_write_live);
    assert!(!resolved.is_read_only);
    assert!(!resolved.is_offline_capture_only);
    assert!(resolved.has_pending_local_work);
    assert!(!resolved.collapses_into_generic_synced);
    assert!(!resolved.hides_local_draft_queue_state);
    assert!(resolved
        .available_actions
        .contains(&M5SyncRowAction::ViewLocalQueue));
}

#[test]
fn sync_behavior_class_separates_read_comment_status_offline_paused() {
    let cases = [
        (
            M5ProviderSyncMode::ReadOnlyMirror,
            M5ProviderWriteScope::ReadOnly,
            M5SyncBehaviorClass::ReadOnlyMetadata,
        ),
        (
            M5ProviderSyncMode::ManualPush,
            M5ProviderWriteScope::CommentOnly,
            M5SyncBehaviorClass::CommentLinkSync,
        ),
        (
            M5ProviderSyncMode::ScheduledSync,
            M5ProviderWriteScope::StatusOnly,
            M5SyncBehaviorClass::StatusTransitionSync,
        ),
        (
            M5ProviderSyncMode::LiveBidirectional,
            M5ProviderWriteScope::FullWrite,
            M5SyncBehaviorClass::FullBidirectionalSync,
        ),
        (
            M5ProviderSyncMode::OfflineOnly,
            M5ProviderWriteScope::NoWrite,
            M5SyncBehaviorClass::OfflineCaptureOnly,
        ),
        (
            M5ProviderSyncMode::PausedSync,
            M5ProviderWriteScope::ScopeUnknown,
            M5SyncBehaviorClass::SyncPaused,
        ),
    ];
    for (mode, scope, expected) in cases {
        let resolved = resolve_sync_behavior_row(&M5SyncRowResolutionInput {
            sync_mode: mode,
            write_scope: scope,
            ..live_full_sync()
        })
        .expect("resolves");
        assert_eq!(
            resolved.behavior_class,
            expected,
            "sync mode {} / scope {} collapsed",
            mode.as_str(),
            scope.as_str()
        );
    }
}

#[test]
fn read_only_and_offline_modes_never_claim_write() {
    for (mode, scope) in [
        (
            M5ProviderSyncMode::ReadOnlyMirror,
            M5ProviderWriteScope::ReadOnly,
        ),
        (
            M5ProviderSyncMode::OfflineOnly,
            M5ProviderWriteScope::NoWrite,
        ),
        (
            M5ProviderSyncMode::PausedSync,
            M5ProviderWriteScope::ScopeUnknown,
        ),
    ] {
        let resolved = resolve_sync_behavior_row(&M5SyncRowResolutionInput {
            sync_mode: mode,
            write_scope: scope,
            ..live_full_sync()
        })
        .expect("resolves");
        assert!(!resolved.can_write_live, "{} claimed write", mode.as_str());
    }
}

#[test]
fn failed_publish_offers_retry_and_view_queue() {
    let resolved = resolve_sync_behavior_row(&M5SyncRowResolutionInput {
        queued_draft_state: M5QueuedDraftState::PublishFailed,
        ..live_full_sync()
    })
    .expect("resolves");
    assert!(resolved.has_pending_local_work);
    assert!(resolved
        .available_actions
        .contains(&M5SyncRowAction::RetryQueuedPublish));
    assert!(resolved
        .available_actions
        .contains(&M5SyncRowAction::ViewLocalQueue));
}

#[test]
fn no_local_draft_hides_no_queue_action_but_still_reveals() {
    let resolved = resolve_sync_behavior_row(&M5SyncRowResolutionInput {
        queued_draft_state: M5QueuedDraftState::NoLocalDraft,
        ..live_full_sync()
    })
    .expect("resolves");
    assert!(!resolved.has_pending_local_work);
    assert!(!resolved
        .available_actions
        .contains(&M5SyncRowAction::ViewLocalQueue));
    assert_eq!(
        resolved.available_actions,
        vec![
            M5SyncRowAction::RevealSyncBehavior,
            M5SyncRowAction::ChangeSyncMode,
            M5SyncRowAction::ExportRow,
        ]
    );
}

#[test]
fn sync_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_sync_behavior_row(&M5SyncRowResolutionInput {
            sync_label: " ".to_owned(),
            ..live_full_sync()
        }),
        Err(M5SyncRowResolutionError::EmptySyncLabel)
    );
    assert_eq!(
        resolve_sync_behavior_row(&M5SyncRowResolutionInput {
            sync_ref: "".to_owned(),
            ..live_full_sync()
        }),
        Err(M5SyncRowResolutionError::EmptySyncRef)
    );
    assert_eq!(
        resolve_sync_behavior_row(&M5SyncRowResolutionInput {
            sync_ref: "sync:secret-token".to_owned(),
            ..live_full_sync()
        }),
        Err(M5SyncRowResolutionError::ForbiddenSyncMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_provider_mapping_sync_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROVIDER_MAPPING_SYNC_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_provider_mapping_sync_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5MappingSyncConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5MappingSyncConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_provider_mapping_sync_row_packet();
    for row in &packet.rows {
        for part in M5MappingRowAnatomyPart::MANDATORY {
            assert!(row.mapping_anatomy_parts.contains(&part));
        }
        for part in M5SyncRowAnatomyPart::MANDATORY {
            assert!(row.sync_anatomy_parts.contains(&part));
        }
        for field in M5MappingRowExportField::MANDATORY {
            assert!(row.mapping_export_fields.contains(&field));
        }
        for field in M5SyncRowExportField::MANDATORY {
            assert!(row.sync_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable));
        assert!(!row.mapping_examples.is_empty());
        assert!(!row.sync_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_provider_mapping_sync_row_packet();
    let mapping_cases: Vec<&M5MappingRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.mapping_examples.iter())
        .collect();
    let sync_cases: Vec<&M5SyncRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.sync_examples.iter())
        .collect();

    for origin in M5MappingOriginClass::ALL {
        assert!(
            mapping_cases
                .iter()
                .any(|c| c.resolved.mapping_origin == origin),
            "no example exercises mapping origin {}",
            origin.as_str()
        );
    }
    for mode in M5ProviderSyncMode::ALL {
        assert!(
            sync_cases.iter().any(|c| c.resolved.sync_mode == mode),
            "no example exercises sync mode {}",
            mode.as_str()
        );
    }
    for class in [
        M5SyncBehaviorClass::ReadOnlyMetadata,
        M5SyncBehaviorClass::CommentLinkSync,
        M5SyncBehaviorClass::StatusTransitionSync,
        M5SyncBehaviorClass::OfflineCaptureOnly,
    ] {
        assert!(
            sync_cases
                .iter()
                .any(|c| c.resolved.behavior_class == class),
            "no example exercises behavior class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_provider_mapping_sync_row_packet();
    for row in &packet.rows {
        for case in &row.mapping_examples {
            assert!(case.is_self_consistent(), "mapping case drifted");
            assert!(case.preserves_mapping_identity(), "mapping lost identity");
            assert!(case.never_assumes_default(), "mapping assumed a default");
        }
        for case in &row.sync_examples {
            assert!(case.is_self_consistent(), "sync case drifted");
            assert!(case.preserves_sync_identity(), "sync lost identity");
            assert!(
                case.distinguishes_behavior_and_shows_queue(),
                "sync collapsed or hid its queue"
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5MappingSyncConsumerSurface::ProviderStatusBar);
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.vocabulary_set.sync_behavior_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_mapping_anatomy_missing_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[0]
        .mapping_anatomy_parts
        .retain(|p| *p != M5MappingRowAnatomyPart::DestinationTargetCue);
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MandatoryMappingAnatomyMissing));
}

#[test]
fn mandatory_sync_export_missing_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[0]
        .sync_export_fields
        .retain(|f| *f != M5SyncRowExportField::QueuedDraftState);
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MandatorySyncExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[0].sync_examples[0].resolved.can_write_live = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::ExampleResolutionDrift));
}

#[test]
fn mapping_example_missing_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[1].mapping_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MappingExampleMissing));
}

#[test]
fn mapping_origin_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    for row in &mut packet.rows {
        row.mapping_examples = vec![M5MappingRowResolutionCase::resolved(
            explicit_issue_mapping(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MappingOriginCoverageUnproven));
}

#[test]
fn sync_mode_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    for row in &mut packet.rows {
        row.sync_examples = vec![M5SyncRowResolutionCase::resolved(live_full_sync())];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::SyncModeCoverageUnproven));
}

#[test]
fn sync_behavior_separation_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    // Every sync example is full-bidirectional → the read/comment/status/offline separations
    // go unproven.
    for row in &mut packet.rows {
        row.sync_examples = vec![M5SyncRowResolutionCase::resolved(live_full_sync())];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::SyncBehaviorSeparationUnproven));
}

#[test]
fn mapping_action_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    // Every mapping explicit-choice → change and reset present but no policy-locked row.
    for row in &mut packet.rows {
        row.mapping_examples = vec![M5MappingRowResolutionCase::resolved(
            explicit_issue_mapping(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MappingActionCoverageUnproven));
}

#[test]
fn destination_explicitness_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    // Every mapping explicit-choice → an explicit destination but never an unmapped row.
    for row in &mut packet.rows {
        row.mapping_examples = vec![M5MappingRowResolutionCase::resolved(
            explicit_issue_mapping(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::DestinationExplicitnessUnproven));
}

#[test]
fn queued_draft_visibility_unproven_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    // Every sync example has a pending queue → no cleared state ever proven.
    for row in &mut packet.rows {
        row.sync_examples = vec![M5SyncRowResolutionCase::resolved(live_full_sync())];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::QueuedDraftVisibilityUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[0].collapses_sync_into_generic_synced = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet
        .governance_review
        .sync_never_uses_one_generic_synced_label = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.consumer_projection.sync_behavior_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provider_mapping_sync_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderMappingSyncRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_provider_mapping_sync_row_packet().render_markdown_summary();
    for surface in M5MappingSyncConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_provider_mapping_sync_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5MappingSyncConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5MappingSyncConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provider_mapping_sync_row_export()
        .expect("checked M5 mapping/sync row primitive export validates");
    assert_eq!(from_disk.packet_id, M5_PROVIDER_MAPPING_SYNC_ROW_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_provider_mapping_sync_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed(),
        seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5MappingSyncConsumerSurface::ALL.len());
    }

    let sync_behavior = seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed();
    let row = sync_behavior
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MappingSyncConsumerSurface::SyncBehaviorPanel)
        .expect("sync-behavior row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);

    let headless = seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed();
    let row = headless
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5MappingSyncConsumerSurface::HeadlessCliMappings)
        .expect("headless-cli-mappings row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let sync_behavior: M5ProviderMappingSyncRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-mapping-sync-behavior-row-primitive/sync_behavior_preview_narrowed.json"
    )))
    .expect("sync-behavior fixture parses");
    assert!(sync_behavior.validate().is_empty());
    assert_eq!(
        sync_behavior,
        seeded_m5_provider_mapping_sync_row_sync_behavior_preview_narrowed()
    );

    let headless: M5ProviderMappingSyncRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-mapping-sync-behavior-row-primitive/headless_cli_mappings_beta_narrowed.json"
    )))
    .expect("headless-cli fixture parses");
    assert!(headless.validate().is_empty());
    assert_eq!(
        headless,
        seeded_m5_provider_mapping_sync_row_headless_cli_mappings_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_mapping_sync_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
