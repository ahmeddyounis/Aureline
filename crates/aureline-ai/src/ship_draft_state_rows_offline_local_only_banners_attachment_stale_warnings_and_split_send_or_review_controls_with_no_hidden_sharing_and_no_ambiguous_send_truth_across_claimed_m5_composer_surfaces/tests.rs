use super::*;

fn local_draft() -> M5DraftStateRowResolutionInput {
    M5DraftStateRowResolutionInput {
        draft_id: "draft.test.local".to_owned(),
        draft_label: "Test draft".to_owned(),
        locality: M5DraftLocality::LocalOnly,
        saved: true,
        shared_or_retained: false,
        sharing_exception_note: None,
        sync_or_policy_note: None,
        clearable: true,
        deletable: false,
    }
}

fn synced_draft() -> M5DraftStateRowResolutionInput {
    M5DraftStateRowResolutionInput {
        draft_id: "draft.test.synced".to_owned(),
        draft_label: "Synced draft".to_owned(),
        locality: M5DraftLocality::WorkspaceSynced,
        saved: true,
        shared_or_retained: true,
        sharing_exception_note: Some("synced to this workspace".to_owned()),
        sync_or_policy_note: None,
        clearable: true,
        deletable: true,
    }
}

fn offline_banner() -> M5AttachmentStaleBannerResolutionInput {
    M5AttachmentStaleBannerResolutionInput {
        banner_id: "stale.test.offline".to_owned(),
        attachment_label: "attached doc".to_owned(),
        offline_local_only: true,
        staleness_reason: None,
        refresh_available: true,
        local_safe_alternative_available: true,
        recovery_note: None,
    }
}

fn split_send() -> M5SendReviewControlResolutionInput {
    M5SendReviewControlResolutionInput {
        control_id: "send.test.split".to_owned(),
        control_label: "Widened send".to_owned(),
        route_before: Some(M5ComposerRouteClass::LocalModel),
        route_after: M5ComposerRouteClass::ManagedRoute,
        widens_authority: true,
        is_mutating_route: true,
        pending_reviews: vec![M5ReviewRequirement::RouteChangeAck],
        policy_blocked: false,
        over_budget: false,
        taint_blocked: false,
    }
}

// ---- draft-state row ----------------------------------------------------

#[test]
fn draft_local_only_is_not_shared_and_offers_view() {
    let resolved = resolve_draft_state_row(&local_draft()).expect("resolves");
    assert_eq!(
        resolved.retention_posture,
        M5DraftRetentionPosture::LocalOnlyPersisted
    );
    assert!(resolved.is_local_only);
    assert!(!resolved.leaves_device);
    assert!(resolved.discloses_sharing);
    assert!(resolved.no_hidden_sharing);
    assert!(resolved
        .available_actions
        .contains(&M5DraftStateAction::ViewRetentionDetail));
    assert!(resolved
        .available_actions
        .contains(&M5DraftStateAction::ClearDraft));
    assert_eq!(resolved.draft_id, "draft.test.local");
}

#[test]
fn draft_retention_posture_maps_locality() {
    for (locality, posture) in [
        (
            M5DraftLocality::EphemeralUnsaved,
            M5DraftRetentionPosture::LocalOnlyEphemeral,
        ),
        (
            M5DraftLocality::LocalOnly,
            M5DraftRetentionPosture::LocalOnlyPersisted,
        ),
        (
            M5DraftLocality::WorkspaceSynced,
            M5DraftRetentionPosture::WorkspaceRetained,
        ),
        (
            M5DraftLocality::AccountSynced,
            M5DraftRetentionPosture::AccountRetained,
        ),
        (
            M5DraftLocality::SharedThread,
            M5DraftRetentionPosture::SharedToThread,
        ),
        (
            M5DraftLocality::RetentionPendingPurge,
            M5DraftRetentionPosture::PurgePending,
        ),
    ] {
        let note_required = locality.as_str(); // any non-empty
        let resolved = resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            locality,
            shared_or_retained: true,
            sharing_exception_note: Some("disclosed".to_owned()),
            sync_or_policy_note: Some(note_required.to_owned()),
            ..local_draft()
        })
        .expect("resolves");
        assert_eq!(resolved.retention_posture, posture);
    }
}

#[test]
fn draft_ephemeral_unsaved_offers_save_locally() {
    let resolved = resolve_draft_state_row(&M5DraftStateRowResolutionInput {
        locality: M5DraftLocality::EphemeralUnsaved,
        saved: false,
        ..local_draft()
    })
    .expect("resolves");
    assert!(resolved
        .available_actions
        .contains(&M5DraftStateAction::SaveLocally));
}

#[test]
fn draft_shared_thread_offers_stop_sharing_and_discloses() {
    let resolved = resolve_draft_state_row(&synced_draft()).expect("resolves");
    assert!(resolved.leaves_device);
    assert!(resolved.discloses_sharing);

    let shared = resolve_draft_state_row(&M5DraftStateRowResolutionInput {
        locality: M5DraftLocality::SharedThread,
        deletable: true,
        ..synced_draft()
    })
    .expect("resolves");
    assert!(shared.is_shared);
    assert!(shared
        .available_actions
        .contains(&M5DraftStateAction::StopSharing));
    assert!(shared
        .available_actions
        .contains(&M5DraftStateAction::DeleteDraft));
}

#[test]
fn draft_rejects_hidden_sharing_and_malformed_input() {
    assert_eq!(
        resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            draft_id: "  ".to_owned(),
            ..local_draft()
        }),
        Err(M5DraftStateRowResolutionError::EmptyDraftId)
    );
    assert_eq!(
        resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            draft_label: "".to_owned(),
            ..local_draft()
        }),
        Err(M5DraftStateRowResolutionError::EmptyDraftLabel)
    );
    // A synced draft that does not disclose its sharing is rejected.
    assert_eq!(
        resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            locality: M5DraftLocality::WorkspaceSynced,
            shared_or_retained: false,
            sharing_exception_note: None,
            ..local_draft()
        }),
        Err(M5DraftStateRowResolutionError::SharedDraftWithoutDisclosure)
    );
    // A purge-pending draft without its retention note is rejected.
    assert_eq!(
        resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            locality: M5DraftLocality::RetentionPendingPurge,
            shared_or_retained: true,
            sharing_exception_note: Some("retained copy exists".to_owned()),
            sync_or_policy_note: None,
            ..local_draft()
        }),
        Err(M5DraftStateRowResolutionError::PurgePendingWithoutNote)
    );
    assert_eq!(
        resolve_draft_state_row(&M5DraftStateRowResolutionInput {
            draft_label: "draft https://leak.test".to_owned(),
            ..local_draft()
        }),
        Err(M5DraftStateRowResolutionError::ForbiddenDraftMaterial)
    );
}

// ---- attachment-stale / offline-local-only banner -----------------------

#[test]
fn stale_offline_preserves_draft_and_offers_resolution() {
    let resolved = resolve_attachment_stale_banner(&offline_banner()).expect("resolves");
    assert_eq!(
        resolved.banner_posture,
        M5StaleBannerPosture::OfflineLocalOnly
    );
    assert!(resolved.draft_preserved);
    assert!(resolved.is_offline_local_only);
    assert!(resolved.offers_resolution_path);
    assert!(resolved
        .available_actions
        .contains(&M5StaleBannerAction::KeepDraftLocal));
    assert!(resolved
        .available_actions
        .contains(&M5StaleBannerAction::RefreshAttachment));
    assert_eq!(resolved.banner_id, "stale.test.offline");
}

#[test]
fn stale_posture_ladder_is_specific_first() {
    for (reason, posture) in [
        (
            M5StalenessReason::SourceEdited,
            M5StaleBannerPosture::StaleRefreshable,
        ),
        (
            M5StalenessReason::SourceMoved,
            M5StaleBannerPosture::StaleRefreshable,
        ),
        (
            M5StalenessReason::IndexReindexed,
            M5StaleBannerPosture::StaleRefreshable,
        ),
        (
            M5StalenessReason::RevisionSuperseded,
            M5StaleBannerPosture::StaleSupersededReview,
        ),
        (
            M5StalenessReason::SourceDeleted,
            M5StaleBannerPosture::StaleSourceGone,
        ),
        (
            M5StalenessReason::PermissionRevoked,
            M5StaleBannerPosture::StaleAccessRevoked,
        ),
    ] {
        let recovery = if matches!(
            reason,
            M5StalenessReason::SourceDeleted | M5StalenessReason::PermissionRevoked
        ) {
            Some("use the local snapshot".to_owned())
        } else {
            None
        };
        let resolved = resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            offline_local_only: false,
            staleness_reason: Some(reason),
            refresh_available: true,
            local_safe_alternative_available: true,
            recovery_note: recovery,
            ..offline_banner()
        })
        .expect("resolves");
        assert_eq!(
            resolved.banner_posture,
            posture,
            "reason {}",
            reason.as_str()
        );
        assert!(resolved.is_stale);
        assert!(resolved.draft_preserved);
        assert!(resolved
            .available_actions
            .contains(&M5StaleBannerAction::ReviewAttachment));
        assert!(resolved.offers_resolution_path);
    }
}

#[test]
fn stale_fresh_reads_fresh() {
    let resolved = resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
        offline_local_only: false,
        staleness_reason: None,
        refresh_available: false,
        local_safe_alternative_available: false,
        recovery_note: None,
        ..offline_banner()
    })
    .expect("resolves");
    assert_eq!(resolved.banner_posture, M5StaleBannerPosture::Fresh);
    assert!(!resolved.is_stale);
    assert!(resolved.draft_preserved);
}

#[test]
fn stale_gone_source_detaches_and_uses_alternative() {
    let resolved = resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
        offline_local_only: false,
        staleness_reason: Some(M5StalenessReason::SourceDeleted),
        refresh_available: false,
        local_safe_alternative_available: true,
        recovery_note: Some("use the checked-in snapshot".to_owned()),
        ..offline_banner()
    })
    .expect("resolves");
    assert_eq!(
        resolved.banner_posture,
        M5StaleBannerPosture::StaleSourceGone
    );
    assert!(!resolved.refreshable);
    assert!(resolved
        .available_actions
        .contains(&M5StaleBannerAction::DetachAttachment));
    assert!(resolved
        .available_actions
        .contains(&M5StaleBannerAction::UseLocalSafeAlternative));
    assert!(!resolved
        .available_actions
        .contains(&M5StaleBannerAction::RefreshAttachment));
}

#[test]
fn stale_rejects_malformed_input() {
    assert_eq!(
        resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            banner_id: " ".to_owned(),
            ..offline_banner()
        }),
        Err(M5AttachmentStaleBannerResolutionError::EmptyBannerId)
    );
    assert_eq!(
        resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            attachment_label: "".to_owned(),
            ..offline_banner()
        }),
        Err(M5AttachmentStaleBannerResolutionError::EmptyAttachmentLabel)
    );
    assert_eq!(
        resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            offline_local_only: false,
            staleness_reason: Some(M5StalenessReason::PermissionRevoked),
            recovery_note: None,
            ..offline_banner()
        }),
        Err(M5AttachmentStaleBannerResolutionError::GoneAttachmentWithoutRecoveryNote)
    );
    assert_eq!(
        resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            offline_local_only: true,
            staleness_reason: None,
            refresh_available: false,
            local_safe_alternative_available: false,
            recovery_note: None,
            ..offline_banner()
        }),
        Err(M5AttachmentStaleBannerResolutionError::OfflineWithoutRefreshOrAlternative)
    );
    assert_eq!(
        resolve_attachment_stale_banner(&M5AttachmentStaleBannerResolutionInput {
            attachment_label: "see https://leak.test".to_owned(),
            ..offline_banner()
        }),
        Err(M5AttachmentStaleBannerResolutionError::ForbiddenStaleMaterial)
    );
}

// ---- split-send / review control ----------------------------------------

#[test]
fn send_widened_authority_splits_and_stays_unambiguous() {
    let resolved = resolve_send_review_control(&split_send()).expect("resolves");
    assert_eq!(resolved.send_posture, M5SendPosture::SplitSendReview);
    assert!(resolved.widens_authority);
    assert!(resolved.is_split);
    assert!(resolved.no_ambiguous_send);
    assert!(resolved.requires_review_before_send);
    assert!(resolved.is_sendable);
    assert_eq!(
        resolved.send_paths,
        vec![
            M5SendPath::ExplainOnly,
            M5SendPath::ReviewThenSend,
            M5SendPath::DirectSend
        ]
    );
    assert!(resolved
        .available_actions
        .contains(&M5SendControlAction::OpenSendReview));
    assert_eq!(resolved.control_id, "send.test.split");
}

#[test]
fn send_posture_ladder_is_blocking_first() {
    let policy = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        policy_blocked: true,
        over_budget: true,
        taint_blocked: true,
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(policy.send_posture, M5SendPosture::PolicyBlocked);
    assert!(!policy.is_sendable);
    assert!(policy.send_paths.is_empty());
    assert!(policy
        .available_actions
        .contains(&M5SendControlAction::ResolveBlocker));

    let taint = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        policy_blocked: false,
        over_budget: true,
        taint_blocked: true,
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(taint.send_posture, M5SendPosture::TaintBlocked);

    let over = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        policy_blocked: false,
        over_budget: true,
        taint_blocked: false,
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(over.send_posture, M5SendPosture::OverBudgetBlocked);

    let review = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        widens_authority: false,
        is_mutating_route: false,
        pending_reviews: vec![M5ReviewRequirement::AttachmentReview],
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(review.send_posture, M5SendPosture::ReviewBeforeSend);
    assert_eq!(
        review.send_paths,
        vec![M5SendPath::ExplainOnly, M5SendPath::ReviewThenSend]
    );

    let ready = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        widens_authority: false,
        is_mutating_route: false,
        pending_reviews: vec![],
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(ready.send_posture, M5SendPosture::ReadyToSend);
    assert_eq!(ready.send_paths, vec![M5SendPath::DirectSend]);
    assert!(ready.no_ambiguous_send);
}

#[test]
fn send_ready_mutating_offers_explain_and_direct() {
    let resolved = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        widens_authority: false,
        is_mutating_route: true,
        pending_reviews: vec![],
        ..split_send()
    })
    .expect("resolves");
    assert_eq!(resolved.send_posture, M5SendPosture::ReadyToSend);
    assert_eq!(
        resolved.send_paths,
        vec![M5SendPath::ExplainOnly, M5SendPath::DirectSend]
    );
    assert!(resolved
        .available_actions
        .contains(&M5SendControlAction::ChooseExplainOnly));
    assert!(resolved
        .available_actions
        .contains(&M5SendControlAction::ConfirmSend));
}

#[test]
fn send_rejects_malformed_input() {
    assert_eq!(
        resolve_send_review_control(&M5SendReviewControlResolutionInput {
            control_id: " ".to_owned(),
            ..split_send()
        }),
        Err(M5SendReviewControlResolutionError::EmptyControlId)
    );
    assert_eq!(
        resolve_send_review_control(&M5SendReviewControlResolutionInput {
            control_label: "".to_owned(),
            ..split_send()
        }),
        Err(M5SendReviewControlResolutionError::EmptyControlLabel)
    );
    assert_eq!(
        resolve_send_review_control(&M5SendReviewControlResolutionInput {
            pending_reviews: vec![M5ReviewRequirement::None],
            ..split_send()
        }),
        Err(M5SendReviewControlResolutionError::ReviewRequirementNotActionable)
    );
    // A widened-authority, non-mutating, unblocked send with no pending review still folds into
    // review-before-send (two paths), so widening never collapses to a single unqualified send.
    let widened_non_mutating = resolve_send_review_control(&M5SendReviewControlResolutionInput {
        widens_authority: true,
        is_mutating_route: false,
        pending_reviews: vec![],
        ..split_send()
    })
    .expect("resolves");
    assert!(widened_non_mutating.is_split);
    assert!(widened_non_mutating.no_ambiguous_send);
    assert_eq!(
        resolve_send_review_control(&M5SendReviewControlResolutionInput {
            control_label: "send bearer token".to_owned(),
            ..split_send()
        }),
        Err(M5SendReviewControlResolutionError::ForbiddenSendMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_draft_send_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DRAFT_SEND_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_draft_send_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5DraftSendConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5DraftSendConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_draft_send_packet();
    for row in &packet.rows {
        for part in M5DraftStateRowAnatomyPart::MANDATORY {
            assert!(row.draft_anatomy_parts.contains(&part));
        }
        for part in M5StaleBannerAnatomyPart::MANDATORY {
            assert!(row.stale_anatomy_parts.contains(&part));
        }
        for part in M5SendControlAnatomyPart::MANDATORY {
            assert!(row.send_anatomy_parts.contains(&part));
        }
        for field in M5DraftStateRowExportField::MANDATORY {
            assert!(row.draft_export_fields.contains(&field));
        }
        for field in M5StaleBannerExportField::MANDATORY {
            assert!(row.stale_export_fields.contains(&field));
        }
        for field in M5SendControlExportField::MANDATORY {
            assert!(row.send_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable));
        assert!(!row.draft_examples.is_empty());
        assert!(!row.stale_examples.is_empty());
        assert!(!row.send_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_draft_send_packet();
    let drafts: Vec<&M5DraftStateRowResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.draft_examples.iter())
        .collect();
    let stales: Vec<&M5AttachmentStaleBannerResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.stale_examples.iter())
        .collect();
    let sends: Vec<&M5SendReviewControlResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.send_examples.iter())
        .collect();

    for locality in M5DraftLocality::ALL {
        assert!(
            drafts.iter().any(|c| c.resolved.locality == locality),
            "no draft example exercises locality {}",
            locality.as_str()
        );
    }
    for posture in M5DraftRetentionPosture::ALL {
        assert!(
            drafts
                .iter()
                .any(|c| c.resolved.retention_posture == posture),
            "no draft example exercises retention posture {}",
            posture.as_str()
        );
    }
    for action in M5DraftStateAction::ALL {
        assert!(
            drafts
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no draft example exercises action {}",
            action.as_str()
        );
    }

    for posture in M5StaleBannerPosture::ALL {
        assert!(
            stales.iter().any(|c| c.resolved.banner_posture == posture),
            "no stale example exercises posture {}",
            posture.as_str()
        );
    }
    for reason in M5StalenessReason::ALL {
        assert!(
            stales
                .iter()
                .any(|c| c.resolved.staleness_reason == Some(reason)),
            "no stale example exercises reason {}",
            reason.as_str()
        );
    }
    for action in M5StaleBannerAction::ALL {
        assert!(
            stales
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no stale example exercises action {}",
            action.as_str()
        );
    }

    for posture in M5SendPosture::ALL {
        assert!(
            sends.iter().any(|c| c.resolved.send_posture == posture),
            "no send example exercises posture {}",
            posture.as_str()
        );
    }
    for path in M5SendPath::ALL {
        assert!(
            sends.iter().any(|c| c.resolved.send_paths.contains(&path)),
            "no send example exercises path {}",
            path.as_str()
        );
    }
    for action in M5SendControlAction::ALL {
        assert!(
            sends
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no send example exercises action {}",
            action.as_str()
        );
    }
    for requirement in [
        M5ReviewRequirement::AttachmentReview,
        M5ReviewRequirement::TaintAck,
        M5ReviewRequirement::BudgetAck,
        M5ReviewRequirement::RouteChangeAck,
    ] {
        assert!(
            sends
                .iter()
                .any(|c| c.resolved.pending_reviews.contains(&requirement)),
            "no send example exercises review requirement {}",
            requirement.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_draft_send_packet();
    for row in &packet.rows {
        for case in &row.draft_examples {
            assert!(
                case.is_self_consistent(),
                "draft case {} drifted",
                case.resolved.draft_id
            );
            assert!(case.preserves_identity());
        }
        for case in &row.stale_examples {
            assert!(
                case.is_self_consistent(),
                "stale case {} drifted",
                case.resolved.banner_id
            );
            assert!(case.preserves_identity());
        }
        for case in &row.send_examples {
            assert!(
                case.is_self_consistent(),
                "send case {} drifted",
                case.resolved.control_id
            );
            assert!(case.preserves_identity());
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5DraftSendConsumerSurface::SidePanel);
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.vocabulary_set.banner_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::VocabularySetDrift));
}

#[test]
fn mandatory_draft_anatomy_missing_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[0]
        .draft_anatomy_parts
        .retain(|p| *p != M5DraftStateRowAnatomyPart::SharingExceptionCue);
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::MandatoryDraftAnatomyMissing));
}

#[test]
fn mandatory_send_export_missing_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[0]
        .send_export_fields
        .retain(|f| *f != M5SendControlExportField::NoAmbiguousSend);
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::MandatorySendExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[0].send_examples[0].resolved.is_sendable = false;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::ExampleResolutionDrift));
}

#[test]
fn stale_example_missing_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[1].stale_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::StaleExampleMissing));
}

#[test]
fn draft_locality_disclosure_unproven_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    // Replace every draft example with a plainly local-only one so no non-local disclosure
    // survives.
    for row in &mut packet.rows {
        row.draft_examples = vec![M5DraftStateRowResolutionCase::resolved(local_draft())];
    }
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::DraftLocalityDisclosureUnproven));
}

#[test]
fn draft_hidden_sharing_found_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    // Forge a resolved draft that leaves the device but does not disclose its sharing.
    let mut forged = M5DraftStateRowResolutionCase::resolved(synced_draft());
    forged.resolved.discloses_sharing = false;
    forged.resolved.no_hidden_sharing = false;
    packet.rows[0].draft_examples.push(forged);
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::DraftHiddenSharingFound));
}

#[test]
fn stale_preserves_draft_unproven_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    // Replace every stale example with a fresh one so no stale-or-offline proof survives.
    for row in &mut packet.rows {
        row.stale_examples = vec![M5AttachmentStaleBannerResolutionCase::resolved(
            M5AttachmentStaleBannerResolutionInput {
                offline_local_only: false,
                staleness_reason: None,
                refresh_available: false,
                local_safe_alternative_available: false,
                recovery_note: None,
                ..offline_banner()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::StalePreservesDraftUnproven));
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::StaleConditionCoverageUnproven));
}

#[test]
fn send_split_no_ambiguous_unproven_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    // Replace every send example with a non-widening ready send so no widened split proof
    // survives.
    for row in &mut packet.rows {
        row.send_examples = vec![M5SendReviewControlResolutionCase::resolved(
            M5SendReviewControlResolutionInput {
                widens_authority: false,
                is_mutating_route: false,
                pending_reviews: vec![],
                ..split_send()
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::SendSplitNoAmbiguousUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[0].assumes_hidden_sharing = true;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.governance_review.stale_banner_preserves_draft = false;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.consumer_projection.send_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_draft_send_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DraftSendViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_draft_send_packet().render_markdown_summary();
    for surface in M5DraftSendConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_draft_send_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DraftSendConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DraftSendConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_draft_send_export()
        .expect("checked M5 draft/send primitive export validates");
    assert_eq!(from_disk.packet_id, M5_DRAFT_SEND_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_draft_send_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_draft_send_patch_draft_preview_narrowed(),
        seeded_m5_draft_send_cli_headless_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5DraftSendConsumerSurface::ALL.len());
    }

    let patch = seeded_m5_draft_send_patch_draft_preview_narrowed();
    let row = patch
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5DraftSendConsumerSurface::PatchDraft)
        .expect("patch-draft row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Preview);

    let cli = seeded_m5_draft_send_cli_headless_beta_narrowed();
    let row = cli
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5DraftSendConsumerSurface::CliHeadless)
        .expect("cli-headless row present");
    assert_eq!(row.qualification, M5ComposerQualificationClass::Beta);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let patch: M5DraftSendPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/patch_draft_preview_narrowed.json"
    )))
    .expect("patch-draft fixture parses");
    assert!(patch.validate().is_empty());
    assert_eq!(patch, seeded_m5_draft_send_patch_draft_preview_narrowed());

    let cli: M5DraftSendPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/cli_headless_beta_narrowed.json"
    )))
    .expect("cli-headless fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(cli, seeded_m5_draft_send_cli_headless_beta_narrowed());
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_draft_send_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
