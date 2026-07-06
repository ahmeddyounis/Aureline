use super::*;

fn participant(
    participant_repr: &str,
    role: M5CollaborationRole,
    follow_state: M5FollowState,
    liveness: M5PresenceParticipantLiveness,
    is_self: bool,
) -> M5PresenceParticipant {
    M5PresenceParticipant {
        participant_repr: participant_repr.to_owned(),
        role,
        follow_state,
        liveness,
        is_self,
    }
}

fn live_pair(title: &str) -> M5PresenceStackResolutionInput {
    M5PresenceStackResolutionInput {
        session_title: title.to_owned(),
        participants: vec![
            participant(
                "presenter-x",
                M5CollaborationRole::Presenter,
                M5FollowState::PresentingToOthers,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
            participant(
                "local-you",
                M5CollaborationRole::Collaborator,
                M5FollowState::FollowingPresenter,
                M5PresenceParticipantLiveness::Active,
                true,
            ),
        ],
        link_state: M5CollaborationLinkState::Live,
        recording_cue: M5RecordingRetentionCue::NotApplicable,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_derives_presenter_control_and_follow() {
    let resolved = resolve_presence_stack(&live_pair("x")).expect("resolves");
    assert_eq!(resolved.presenter_repr.as_deref(), Some("presenter-x"));
    assert!(resolved.self_is_following_presenter);
    assert!(resolved
        .available_actions
        .contains(&M5PresenceAction::StopFollowing));
    assert!(!resolved
        .available_actions
        .contains(&M5PresenceAction::FollowPresenter));
    assert_eq!(
        resolved.continuity_posture,
        M5PresenceContinuityPosture::Live
    );
    assert!(resolved.collaboration_remains_visible);
    assert!(resolved.non_avatar_reachable);
}

#[test]
fn resolver_orders_presenter_and_control_holder_first() {
    let input = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "observer-o",
                M5CollaborationRole::Observer,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Idle,
                false,
            ),
            participant(
                "control-c",
                M5CollaborationRole::ControlHolder,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
            participant(
                "presenter-p",
                M5CollaborationRole::Presenter,
                M5FollowState::PresentingToOthers,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
        ],
        ..live_pair("x")
    };
    let resolved = resolve_presence_stack(&input).expect("resolves");
    assert_eq!(
        resolved.ordered_participants[0].participant_repr,
        "presenter-p"
    );
    assert!(resolved.ordered_participants[0].is_presenter);
    assert_eq!(
        resolved.ordered_participants[1].participant_repr,
        "control-c"
    );
    assert!(resolved.ordered_participants[1].is_control_holder);
    // Ordered ascending by salience rank.
    let ranks: Vec<u8> = resolved
        .ordered_participants
        .iter()
        .map(|p| p.salience_rank)
        .collect();
    let mut sorted = ranks.clone();
    sorted.sort_unstable();
    assert_eq!(ranks, sorted);
}

#[test]
fn resolver_offers_follow_when_presenter_present_and_self_not_following() {
    let input = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "presenter-x",
                M5CollaborationRole::Presenter,
                M5FollowState::PresentingToOthers,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
            participant(
                "local-you",
                M5CollaborationRole::Observer,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Idle,
                true,
            ),
        ],
        ..live_pair("x")
    };
    let resolved = resolve_presence_stack(&input).expect("resolves");
    assert!(resolved
        .available_actions
        .contains(&M5PresenceAction::FollowPresenter));
    assert!(!resolved.self_is_following_presenter);
}

#[test]
fn resolver_self_presenting_marks_view_followed() {
    let input = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "local-you",
                M5CollaborationRole::Presenter,
                M5FollowState::PresentingToOthers,
                M5PresenceParticipantLiveness::Active,
                true,
            ),
            participant(
                "watcher-w",
                M5CollaborationRole::Observer,
                M5FollowState::FollowingPresenter,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
        ],
        ..live_pair("x")
    };
    let resolved = resolve_presence_stack(&input).expect("resolves");
    assert!(resolved.current_view_being_followed);
    assert_eq!(resolved.presenter_repr.as_deref(), Some("local-you"));
    // The self-presenter is not offered a follow action for their own view.
    assert!(!resolved
        .available_actions
        .contains(&M5PresenceAction::FollowPresenter));
}

#[test]
fn resolver_degraded_link_keeps_presence_and_offers_reconnect() {
    for link in [
        M5CollaborationLinkState::Degraded,
        M5CollaborationLinkState::Reconnecting,
        M5CollaborationLinkState::OfflineLocalFallback,
    ] {
        let input = M5PresenceStackResolutionInput {
            link_state: link,
            ..live_pair("x")
        };
        let resolved = resolve_presence_stack(&input).expect("resolves");
        assert!(resolved.continuity_posture.is_degraded());
        assert!(resolved.collaboration_remains_visible);
        assert!(!resolved.ordered_participants.is_empty());
        assert!(
            resolved
                .available_actions
                .contains(&M5PresenceAction::ReconnectCollaboration),
            "link {} dropped the reconnect action",
            link.as_str()
        );
    }
}

#[test]
fn resolver_ended_session_keeps_last_known_roster_without_reconnect() {
    let input = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "local-you",
                M5CollaborationRole::SessionHost,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::LastKnownLocal,
                true,
            ),
            participant(
                "control-c",
                M5CollaborationRole::ControlHolder,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::LastKnownLocal,
                false,
            ),
        ],
        link_state: M5CollaborationLinkState::SessionEnded,
        ..live_pair("x")
    };
    let resolved = resolve_presence_stack(&input).expect("resolves");
    assert_eq!(
        resolved.continuity_posture,
        M5PresenceContinuityPosture::EndedLastKnownVisible
    );
    assert!(resolved.continuity_posture.is_local_fallback());
    assert_eq!(resolved.control_holder_repr.as_deref(), Some("control-c"));
    assert!(!resolved
        .available_actions
        .contains(&M5PresenceAction::ReconnectCollaboration));
    assert_eq!(resolved.roster_count, 2);
    assert_eq!(resolved.present_count, 0);
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_title = M5PresenceStackResolutionInput {
        session_title: "  ".to_owned(),
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&empty_title),
        Err(M5PresenceStackResolutionError::EmptySessionTitle)
    );

    let no_participants = M5PresenceStackResolutionInput {
        participants: vec![],
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&no_participants),
        Err(M5PresenceStackResolutionError::EmptyParticipants)
    );

    let empty_repr = M5PresenceStackResolutionInput {
        participants: vec![participant(
            "  ",
            M5CollaborationRole::Observer,
            M5FollowState::NotFollowing,
            M5PresenceParticipantLiveness::Active,
            false,
        )],
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&empty_repr),
        Err(M5PresenceStackResolutionError::EmptyParticipantRepr)
    );

    let dup = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "same",
                M5CollaborationRole::Observer,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
            participant(
                "same",
                M5CollaborationRole::Collaborator,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Active,
                false,
            ),
        ],
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&dup),
        Err(M5PresenceStackResolutionError::DuplicateParticipant)
    );

    let dup_self = M5PresenceStackResolutionInput {
        participants: vec![
            participant(
                "a",
                M5CollaborationRole::Observer,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Active,
                true,
            ),
            participant(
                "b",
                M5CollaborationRole::Collaborator,
                M5FollowState::NotFollowing,
                M5PresenceParticipantLiveness::Active,
                true,
            ),
        ],
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&dup_self),
        Err(M5PresenceStackResolutionError::DuplicateSelfParticipant)
    );

    let forbidden = M5PresenceStackResolutionInput {
        participants: vec![participant(
            "user at https://example.test",
            M5CollaborationRole::Observer,
            M5FollowState::NotFollowing,
            M5PresenceParticipantLiveness::Active,
            false,
        )],
        ..live_pair("x")
    };
    assert_eq!(
        resolve_presence_stack(&forbidden),
        Err(M5PresenceStackResolutionError::ForbiddenPresenceMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_presence_avatar_stack_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_PRESENCE_AVATAR_STACK_PRIMITIVE_PACKET_ID
    );
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_presence_avatar_stack_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .consumer_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5PresenceConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.consumer_rows.len(),
        M5PresenceConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_presence_avatar_stack_primitive_packet();
    for row in &packet.consumer_rows {
        for part in M5PresenceAvatarStackPart::MANDATORY {
            assert!(row.stack_parts.contains(&part));
        }
        for part in M5RoleFollowBadgePart::MANDATORY {
            assert!(row.badge_parts.contains(&part));
        }
        for field in M5PresenceExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .stack_parts
            .contains(&M5PresenceAvatarStackPart::TextParticipantList));
        assert!(row
            .accessibility_routes
            .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_presence_avatar_stack_primitive_packet();
    let cases: Vec<&M5PresenceStackResolutionCase> = packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for role in M5CollaborationRole::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .ordered_participants
                .iter()
                .any(|p| p.role == role)),
            "no worked resolution exercises role {}",
            role.as_str()
        );
    }
    for follow in M5FollowState::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .ordered_participants
                .iter()
                .any(|p| p.follow_state == follow)),
            "no worked resolution exercises follow state {}",
            follow.as_str()
        );
    }
    for liveness in M5PresenceParticipantLiveness::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .ordered_participants
                .iter()
                .any(|p| p.liveness == liveness)),
            "no worked resolution exercises liveness {}",
            liveness.as_str()
        );
    }
    for posture in M5PresenceContinuityPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.continuity_posture == posture),
            "no worked resolution exercises continuity posture {}",
            posture.as_str()
        );
    }
    for cue in M5RecordingRetentionCue::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.recording_cue == cue),
            "no worked resolution exercises recording cue {}",
            cue.as_str()
        );
    }
    for action in M5PresenceAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no worked resolution exercises action {}",
            action.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_presence_avatar_stack_primitive_packet();
    for row in &packet.consumer_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet
        .consumer_rows
        .retain(|row| row.consumer_surface != M5PresenceConsumerSurface::PresenterHud);
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.vocabulary_set.link_states.pop();
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_stack_part_missing_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0]
        .stack_parts
        .retain(|p| *p != M5PresenceAvatarStackPart::TextParticipantList);
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::MandatoryStackPartMissing));
}

#[test]
fn mandatory_badge_part_missing_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0]
        .badge_parts
        .retain(|p| *p != M5RoleFollowBadgePart::FollowLabel);
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::MandatoryBadgePartMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0]
        .export_fields
        .retain(|f| *f != M5PresenceExportField::PresenterIdentity);
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0].example_resolutions[0]
        .resolved
        .continuity_posture = M5PresenceContinuityPosture::DegradedVisible;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn presenter_visibility_unproven_fails_when_no_presenter_or_followed_view() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    // Replace every example with a lone-observer resolution: no presenter, view not
    // followed.
    for row in &mut packet.consumer_rows {
        row.example_resolutions = vec![M5PresenceStackResolutionCase::resolved(
            M5PresenceStackResolutionInput {
                session_title: "lone".to_owned(),
                participants: vec![participant(
                    "solo",
                    M5CollaborationRole::Observer,
                    M5FollowState::NotFollowing,
                    M5PresenceParticipantLiveness::Active,
                    true,
                )],
                link_state: M5CollaborationLinkState::Degraded,
                recording_cue: M5RecordingRetentionCue::NotApplicable,
            },
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::PresenterVisibilityUnproven));
}

#[test]
fn degraded_continuity_unproven_fails_when_all_examples_live() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    for row in &mut packet.consumer_rows {
        for case in &mut row.example_resolutions {
            case.input.link_state = M5CollaborationLinkState::Live;
            case.resolved = resolve_presence_stack(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::DegradedContinuityUnproven));
}

#[test]
fn local_fallback_continuity_unproven_fails_when_no_fallback_control_example() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    for row in &mut packet.consumer_rows {
        for case in &mut row.example_resolutions {
            case.input.link_state = M5CollaborationLinkState::Live;
            case.resolved = resolve_presence_stack(&case.input).unwrap();
        }
    }
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::LocalFallbackContinuityUnproven));
}

#[test]
fn consumer_invariant_violation_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0].drops_presence_when_degraded = true;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::ConsumerInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.consumer_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet
        .governance_review
        .collaboration_visible_through_degraded_flows = false;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet
        .consumer_projection
        .continuity_reads_single_link_state_source = false;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PresenceAvatarStackPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_presence_avatar_stack_primitive_packet().render_markdown_summary();
    for surface in M5PresenceConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_presence_avatar_stack_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5PresenceConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5PresenceConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_presence_avatar_stack_primitive_export()
        .expect("checked M5 presence-avatar-stack primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PRESENCE_AVATAR_STACK_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_presence_avatar_stack_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed(),
        seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.consumer_rows.len(),
            M5PresenceConsumerSurface::ALL.len()
        );
    }

    let debug = seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed();
    let row = debug
        .consumer_rows
        .iter()
        .find(|r| r.consumer_surface == M5PresenceConsumerSurface::SharedDebugPane)
        .expect("shared debug pane row present");
    assert_eq!(row.qualification, M5RuntimeBoundaryQualificationClass::Beta);

    let review = seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed();
    let row = review
        .consumer_rows
        .iter()
        .find(|r| r.consumer_surface == M5PresenceConsumerSurface::ReviewSessionHeader)
        .expect("review / session header row present");
    assert_eq!(
        row.qualification,
        M5RuntimeBoundaryQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let debug: M5PresenceAvatarStackPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-presence-avatar-stack-primitive/shared_debug_pane_beta_narrowed.json"
    )))
    .expect("debug fixture parses");
    assert!(debug.validate().is_empty());
    assert_eq!(
        debug,
        seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed()
    );

    let review: M5PresenceAvatarStackPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-presence-avatar-stack-primitive/review_session_header_preview_narrowed.json"
    )))
    .expect("review fixture parses");
    assert!(review.validate().is_empty());
    assert_eq!(
        review,
        seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_presence_avatar_stack_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
