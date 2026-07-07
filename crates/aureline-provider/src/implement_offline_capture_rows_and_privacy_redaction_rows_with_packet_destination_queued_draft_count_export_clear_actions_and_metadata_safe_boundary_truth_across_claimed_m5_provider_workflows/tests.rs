use super::*;

fn queued_bug_capture() -> M5OfflineCaptureRowResolutionInput {
    M5OfflineCaptureRowResolutionInput {
        capture_state: M5OfflineCaptureState::QueuedForPublish,
        capture_kind: M5OfflineCaptureKind::BugReport,
        destination_class: M5OfflinePacketDestinationClass::RoutedToProvider,
        queued_draft_state: M5QueuedDraftState::QueuedPublish,
        redaction_default: M5ProviderRedactionClass::MetadataOnly,
        queued_draft_count: 3,
        packet_destination_label: "acme-eng issues (queued)".to_owned(),
        capture_label: "crash on export (bug)".to_owned(),
        capture_ref: "capture:acme-eng:bug:queued-1".to_owned(),
    }
}

fn metadata_only_redaction() -> M5PrivacyRedactionRowResolutionInput {
    M5PrivacyRedactionRowResolutionInput {
        redaction_class: M5ProviderRedactionClass::MetadataOnly,
        export_boundary: M5ExportBoundaryClass::MetadataSafe,
        policy_source: M5RedactionPolicySource::UserDefault,
        telemetry_limit: M5TelemetryEventLimit::MetadataCountersOnly,
        policy_label: "acme-eng metadata-only default".to_owned(),
        redaction_ref: "redaction:acme-eng:metadata:1".to_owned(),
    }
}

// ---- offline-capture-row resolver ---------------------------------------

#[test]
fn queued_bug_is_publishes_when_reachable_with_queue_and_defer() {
    let resolved = resolve_offline_capture_row(&queued_bug_capture()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5OfflineCaptureRowPosture::QueuedForPublishRow
    );
    assert_eq!(
        resolved.publish_later_behavior,
        M5PublishLaterBehavior::PublishesWhenReachable
    );
    assert!(resolved.shows_packet_destination);
    assert!(resolved.has_queued_drafts);
    assert!(resolved.retains_prepared_handoff);
    assert!(!resolved.hides_queued_local_work);
    assert!(!resolved.assumes_default_destination_silently);
    assert!(resolved
        .available_actions
        .contains(&M5OfflineCaptureRowAction::DeferPublish));
    assert!(resolved
        .available_actions
        .contains(&M5OfflineCaptureRowAction::ClearCapture));
    assert!(resolved
        .available_actions
        .contains(&M5OfflineCaptureRowAction::ExportPacket));
}

#[test]
fn capture_posture_and_behavior_map_one_to_one_from_state() {
    let cases = [
        (
            M5OfflineCaptureState::CapturedLocal,
            M5OfflineCaptureRowPosture::CapturedLocallyRow,
            M5PublishLaterBehavior::HeldLocallyUntilPublish,
        ),
        (
            M5OfflineCaptureState::QueuedForPublish,
            M5OfflineCaptureRowPosture::QueuedForPublishRow,
            M5PublishLaterBehavior::PublishesWhenReachable,
        ),
        (
            M5OfflineCaptureState::PublishDeferred,
            M5OfflineCaptureRowPosture::PublishDeferredRow,
            M5PublishLaterBehavior::HeldByUserChoice,
        ),
        (
            M5OfflineCaptureState::ConflictHeld,
            M5OfflineCaptureRowPosture::ConflictHeldRow,
            M5PublishLaterBehavior::HeldPendingConflict,
        ),
        (
            M5OfflineCaptureState::DiscardPending,
            M5OfflineCaptureRowPosture::DiscardPendingRow,
            M5PublishLaterBehavior::WillDiscardOnConfirm,
        ),
        (
            M5OfflineCaptureState::SyncedCleared,
            M5OfflineCaptureRowPosture::SyncedClearedRow,
            M5PublishLaterBehavior::AlreadyPublished,
        ),
    ];
    for (state, expected_posture, expected_behavior) in cases {
        let count = if matches!(state, M5OfflineCaptureState::SyncedCleared) {
            0
        } else {
            2
        };
        let resolved = resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            capture_state: state,
            queued_draft_count: count,
            ..queued_bug_capture()
        })
        .expect("resolves");
        assert_eq!(resolved.row_posture, expected_posture);
        assert_eq!(resolved.publish_later_behavior, expected_behavior);
    }
}

#[test]
fn synced_cleared_row_offers_no_clear_and_has_no_queue() {
    let resolved = resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
        capture_state: M5OfflineCaptureState::SyncedCleared,
        queued_draft_state: M5QueuedDraftState::PublishedReconciled,
        queued_draft_count: 0,
        ..queued_bug_capture()
    })
    .expect("resolves");
    assert!(!resolved.has_queued_drafts);
    assert!(!resolved
        .available_actions
        .contains(&M5OfflineCaptureRowAction::ClearCapture));
    assert_eq!(
        resolved.available_actions,
        vec![
            M5OfflineCaptureRowAction::RevealCapture,
            M5OfflineCaptureRowAction::ExportPacket,
        ]
    );
    // Even a synced/cleared row still retains its prepared-handoff invariant flags.
    assert!(resolved.retains_prepared_handoff);
}

#[test]
fn unrouted_packet_flags_unrouted_and_never_defaults() {
    let resolved = resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
        destination_class: M5OfflinePacketDestinationClass::UnroutedPending,
        ..queued_bug_capture()
    })
    .expect("resolves");
    assert!(!resolved.shows_packet_destination);
    assert!(!resolved.assumes_default_destination_silently);
}

#[test]
fn blocked_or_failed_publish_offers_retry() {
    for state in [
        M5QueuedDraftState::PublishBlocked,
        M5QueuedDraftState::PublishFailed,
    ] {
        let resolved = resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            capture_state: M5OfflineCaptureState::ConflictHeld,
            queued_draft_state: state,
            queued_draft_count: 1,
            ..queued_bug_capture()
        })
        .expect("resolves");
        assert!(resolved
            .available_actions
            .contains(&M5OfflineCaptureRowAction::RetryPublish));
    }
}

#[test]
fn cleared_capture_with_queued_drafts_is_rejected() {
    assert_eq!(
        resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            capture_state: M5OfflineCaptureState::SyncedCleared,
            queued_draft_count: 2,
            ..queued_bug_capture()
        }),
        Err(M5OfflineCaptureRowResolutionError::ClearedCaptureHasQueuedDrafts)
    );
}

#[test]
fn offline_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            packet_destination_label: " ".to_owned(),
            ..queued_bug_capture()
        }),
        Err(M5OfflineCaptureRowResolutionError::EmptyDestinationLabel)
    );
    assert_eq!(
        resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            capture_label: "".to_owned(),
            ..queued_bug_capture()
        }),
        Err(M5OfflineCaptureRowResolutionError::EmptyCaptureLabel)
    );
    assert_eq!(
        resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            capture_ref: "".to_owned(),
            ..queued_bug_capture()
        }),
        Err(M5OfflineCaptureRowResolutionError::EmptyCaptureRef)
    );
    assert_eq!(
        resolve_offline_capture_row(&M5OfflineCaptureRowResolutionInput {
            packet_destination_label: "board at https://provider.example/x".to_owned(),
            ..queued_bug_capture()
        }),
        Err(M5OfflineCaptureRowResolutionError::ForbiddenCaptureMaterial)
    );
}

// ---- privacy/redaction-row resolver -------------------------------------

#[test]
fn metadata_only_row_exports_metadata_and_withholds_credentials() {
    let resolved = resolve_privacy_redaction_row(&metadata_only_redaction()).expect("resolves");
    assert_eq!(
        resolved.row_posture,
        M5PrivacyRedactionRowPosture::MetadataOnlyRow
    );
    assert_eq!(
        resolved.support_bundle_treatment,
        M5SupportBundleTreatment::MetadataOnlyInBundle
    );
    assert!(resolved.can_export);
    assert!(resolved.withholds_credentials_and_endpoints);
    assert!(resolved.metadata_safe_default_explicit);
    assert!(resolved.escalation_requires_review);
    assert!(!resolved.hides_export_or_redaction_boundary);
    assert!(!resolved
        .exported_field_classes
        .contains(&M5PrivacyFieldClass::Credentials));
    assert!(!resolved
        .exported_field_classes
        .contains(&M5PrivacyFieldClass::Endpoints));
    assert!(resolved
        .withheld_field_classes
        .contains(&M5PrivacyFieldClass::Credentials));
    assert!(resolved
        .withheld_field_classes
        .contains(&M5PrivacyFieldClass::Endpoints));
    assert!(resolved
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::RequestEscalationReview));
}

#[test]
fn redaction_posture_maps_one_to_one_from_class() {
    let cases = [
        (
            M5ProviderRedactionClass::FullBodyVisible,
            M5PrivacyRedactionRowPosture::FullBodyVisibleRow,
        ),
        (
            M5ProviderRedactionClass::MetadataOnly,
            M5PrivacyRedactionRowPosture::MetadataOnlyRow,
        ),
        (
            M5ProviderRedactionClass::RedactedShare,
            M5PrivacyRedactionRowPosture::RedactedShareRow,
        ),
        (
            M5ProviderRedactionClass::PolicyRestricted,
            M5PrivacyRedactionRowPosture::PolicyRestrictedRow,
        ),
        (
            M5ProviderRedactionClass::RawWithheld,
            M5PrivacyRedactionRowPosture::RawWithheldRow,
        ),
        (
            M5ProviderRedactionClass::NoExport,
            M5PrivacyRedactionRowPosture::NoExportRow,
        ),
    ];
    for (class, expected) in cases {
        let resolved = resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
            redaction_class: class,
            ..metadata_only_redaction()
        })
        .expect("resolves");
        assert_eq!(resolved.row_posture, expected);
        // Credentials and endpoints are never exported, whatever the class.
        assert!(resolved.withholds_credentials_and_endpoints);
    }
}

#[test]
fn no_export_row_offers_no_export_but_still_escalates() {
    let resolved = resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
        redaction_class: M5ProviderRedactionClass::NoExport,
        export_boundary: M5ExportBoundaryClass::LocalOnly,
        policy_source: M5RedactionPolicySource::ProviderPolicy,
        telemetry_limit: M5TelemetryEventLimit::NoEventExport,
        ..metadata_only_redaction()
    })
    .expect("resolves");
    assert!(!resolved.can_export);
    assert!(resolved.exported_field_classes.is_empty());
    assert!(!resolved
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::ExportRedactedBundle));
    assert!(resolved
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::RequestEscalationReview));
}

#[test]
fn org_and_policy_restricted_rows_block_local_adjust() {
    // Org policy is locked.
    let org = resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
        policy_source: M5RedactionPolicySource::OrgPolicy,
        ..metadata_only_redaction()
    })
    .expect("resolves");
    assert!(!org
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::AdjustRedaction));

    // A policy-restricted class blocks adjust even under a workspace policy.
    let restricted = resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
        redaction_class: M5ProviderRedactionClass::PolicyRestricted,
        policy_source: M5RedactionPolicySource::WorkspacePolicy,
        ..metadata_only_redaction()
    })
    .expect("resolves");
    assert!(!restricted
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::AdjustRedaction));

    // A user default may adjust.
    let user = resolve_privacy_redaction_row(&metadata_only_redaction()).expect("resolves");
    assert!(user
        .available_actions
        .contains(&M5PrivacyRedactionRowAction::AdjustRedaction));
}

#[test]
fn full_body_visible_exports_body_but_never_credentials() {
    let resolved = resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
        redaction_class: M5ProviderRedactionClass::FullBodyVisible,
        ..metadata_only_redaction()
    })
    .expect("resolves");
    assert!(resolved
        .exported_field_classes
        .contains(&M5PrivacyFieldClass::BodyText));
    assert!(!resolved
        .exported_field_classes
        .contains(&M5PrivacyFieldClass::Credentials));
    assert!(!resolved
        .exported_field_classes
        .contains(&M5PrivacyFieldClass::Endpoints));
    assert!(resolved.withholds_credentials_and_endpoints);
}

#[test]
fn privacy_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
            policy_label: " ".to_owned(),
            ..metadata_only_redaction()
        }),
        Err(M5PrivacyRedactionRowResolutionError::EmptyPolicyLabel)
    );
    assert_eq!(
        resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
            redaction_ref: "".to_owned(),
            ..metadata_only_redaction()
        }),
        Err(M5PrivacyRedactionRowResolutionError::EmptyRedactionRef)
    );
    assert_eq!(
        resolve_privacy_redaction_row(&M5PrivacyRedactionRowResolutionInput {
            redaction_ref: "redaction:secret-token".to_owned(),
            ..metadata_only_redaction()
        }),
        Err(M5PrivacyRedactionRowResolutionError::ForbiddenRedactionMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_provider_offline_privacy_row_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROVIDER_OFFLINE_PRIVACY_ROW_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_provider_offline_privacy_row_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5OfflinePrivacyConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5OfflinePrivacyConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_provider_offline_privacy_row_packet();
    for row in &packet.rows {
        for part in M5OfflineCaptureRowAnatomyPart::MANDATORY {
            assert!(row.offline_anatomy_parts.contains(&part));
        }
        for part in M5PrivacyRedactionRowAnatomyPart::MANDATORY {
            assert!(row.privacy_anatomy_parts.contains(&part));
        }
        for field in M5OfflineCaptureRowExportField::MANDATORY {
            assert!(row.offline_export_fields.contains(&field));
        }
        for field in M5PrivacyRedactionRowExportField::MANDATORY {
            assert!(row.privacy_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable));
        assert!(!row.offline_examples.is_empty());
        assert!(!row.privacy_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_provider_offline_privacy_row_packet();
    let offline_cases: Vec<&M5OfflineCaptureRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.offline_examples.iter())
        .collect();
    let privacy_cases: Vec<&M5PrivacyRedactionRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.privacy_examples.iter())
        .collect();

    for state in M5OfflineCaptureState::ALL {
        assert!(
            offline_cases
                .iter()
                .any(|c| c.resolved.capture_state == state),
            "no example exercises capture state {}",
            state.as_str()
        );
    }
    for class in M5ProviderRedactionClass::ALL {
        assert!(
            privacy_cases
                .iter()
                .any(|c| c.resolved.redaction_class == class),
            "no example exercises redaction class {}",
            class.as_str()
        );
    }
    for behavior in [
        M5PublishLaterBehavior::PublishesWhenReachable,
        M5PublishLaterBehavior::HeldByUserChoice,
        M5PublishLaterBehavior::HeldPendingConflict,
        M5PublishLaterBehavior::AlreadyPublished,
    ] {
        assert!(
            offline_cases
                .iter()
                .any(|c| c.resolved.publish_later_behavior == behavior),
            "no example exercises publish behavior {}",
            behavior.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_provider_offline_privacy_row_packet();
    for row in &packet.rows {
        for case in &row.offline_examples {
            assert!(case.is_self_consistent(), "offline case drifted");
            assert!(case.preserves_capture_identity(), "offline lost identity");
            assert!(case.retains_handoff(), "offline dropped handoff");
        }
        for case in &row.privacy_examples {
            assert!(case.is_self_consistent(), "privacy case drifted");
            assert!(case.preserves_redaction_identity(), "privacy lost identity");
            assert!(
                case.keeps_boundary_and_withholds(),
                "privacy leaked or hid its boundary"
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5OfflinePrivacyConsumerSurface::ProviderStatusBar);
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.vocabulary_set.privacy_field_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::VocabularySetDrift));
}

#[test]
fn mandatory_offline_anatomy_missing_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[0]
        .offline_anatomy_parts
        .retain(|p| *p != M5OfflineCaptureRowAnatomyPart::PacketDestinationCue);
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::MandatoryOfflineAnatomyMissing));
}

#[test]
fn mandatory_privacy_export_missing_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[0]
        .privacy_export_fields
        .retain(|f| *f != M5PrivacyRedactionRowExportField::ExportedFieldClasses);
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::MandatoryPrivacyExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[0].offline_examples[0]
        .resolved
        .has_queued_drafts = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::ExampleResolutionDrift));
}

#[test]
fn offline_example_missing_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[1].offline_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::OfflineExampleMissing));
}

#[test]
fn offline_capture_state_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    for row in &mut packet.rows {
        row.offline_examples = vec![M5OfflineCaptureRowResolutionCase::resolved(
            queued_bug_capture(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::OfflineCaptureStateCoverageUnproven));
}

#[test]
fn redaction_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    for row in &mut packet.rows {
        row.privacy_examples = vec![M5PrivacyRedactionRowResolutionCase::resolved(
            metadata_only_redaction(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::RedactionClassCoverageUnproven));
}

#[test]
fn publish_later_separation_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    // Every offline example is queued-for-publish → the held/conflict/synced separations go
    // unproven.
    for row in &mut packet.rows {
        row.offline_examples = vec![M5OfflineCaptureRowResolutionCase::resolved(
            queued_bug_capture(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::PublishLaterSeparationUnproven));
}

#[test]
fn export_clear_action_coverage_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    // Every offline example queued-for-publish → clear present but never a synced-and-cleared
    // row, and no retry.
    for row in &mut packet.rows {
        row.offline_examples = vec![M5OfflineCaptureRowResolutionCase::resolved(
            queued_bug_capture(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::ExportClearActionCoverageUnproven));
}

#[test]
fn packet_destination_explicitness_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    // Every offline example routed → an explicit destination but never an unrouted packet.
    for row in &mut packet.rows {
        row.offline_examples = vec![M5OfflineCaptureRowResolutionCase::resolved(
            queued_bug_capture(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::PacketDestinationExplicitnessUnproven));
}

#[test]
fn queued_draft_visibility_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    // Every offline example has a queued count → no cleared state ever proven.
    for row in &mut packet.rows {
        row.offline_examples = vec![M5OfflineCaptureRowResolutionCase::resolved(
            queued_bug_capture(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::QueuedDraftVisibilityUnproven));
}

#[test]
fn metadata_safe_boundary_unproven_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    // Every privacy example is a metadata-safe export → no no-export block ever proven.
    for row in &mut packet.rows {
        row.privacy_examples = vec![M5PrivacyRedactionRowResolutionCase::resolved(
            metadata_only_redaction(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::MetadataSafeBoundaryUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[0].leaks_credentials_or_endpoints = true;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet
        .governance_review
        .metadata_safe_default_explicit_before_leaving_device = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet
        .consumer_projection
        .privacy_redaction_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provider_offline_privacy_row_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProviderOfflinePrivacyRowViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_provider_offline_privacy_row_packet().render_markdown_summary();
    for surface in M5OfflinePrivacyConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_provider_offline_privacy_row_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5OfflinePrivacyConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5OfflinePrivacyConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provider_offline_privacy_row_export()
        .expect("checked M5 offline/privacy row primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_provider_offline_privacy_row_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed(),
        seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5OfflinePrivacyConsumerSurface::ALL.len()
        );
    }

    let offline = seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed();
    let row = offline
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5OfflinePrivacyConsumerSurface::OfflineCapturePanel)
        .expect("offline-capture panel row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Beta);

    let privacy = seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed();
    let row = privacy
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5OfflinePrivacyConsumerSurface::PrivacyRedactionPanel)
        .expect("privacy-redaction panel row present");
    assert_eq!(row.qualification, M5ProviderQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let offline: M5ProviderOfflinePrivacyRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-offline-capture-privacy-redaction-row-primitive/offline_capture_beta_narrowed.json"
    )))
    .expect("offline-capture fixture parses");
    assert!(offline.validate().is_empty());
    assert_eq!(
        offline,
        seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed()
    );

    let privacy: M5ProviderOfflinePrivacyRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-provider-offline-capture-privacy-redaction-row-primitive/privacy_redaction_preview_narrowed.json"
    )))
    .expect("privacy-redaction fixture parses");
    assert!(privacy.validate().is_empty());
    assert_eq!(
        privacy,
        seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provider_offline_privacy_row_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
