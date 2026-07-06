//! Canonical seed builders for the M5 presence-avatar-stack primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical presence-avatar-stack primitive packet.
pub const M5_PRESENCE_AVATAR_STACK_PRIMITIVE_PACKET_ID: &str =
    "m5-presence-avatar-stack-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one participant.
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

/// Builds a worked resolution case from a full presence state.
fn case(
    session_title: &str,
    participants: Vec<M5PresenceParticipant>,
    link_state: M5CollaborationLinkState,
    recording_cue: M5RecordingRetentionCue,
) -> M5PresenceStackResolutionCase {
    M5PresenceStackResolutionCase::resolved(M5PresenceStackResolutionInput {
        session_title: session_title.to_owned(),
        participants,
        link_state,
        recording_cue,
    })
}

/// A base row with the shared fields filled in and the full stack-part, badge-part,
/// role, follow-state, liveness, link-state, continuity-posture, recording-cue, action,
/// export-field, and accessibility parity every surface carries.
fn base_row(
    consumer_surface: M5PresenceConsumerSurface,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5PresenceStackResolutionCase>,
) -> M5PresenceConsumerRow {
    M5PresenceConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        stack_parts: M5PresenceAvatarStackPart::ALL.to_vec(),
        badge_parts: M5RoleFollowBadgePart::ALL.to_vec(),
        collaboration_roles: M5CollaborationRole::ALL.to_vec(),
        follow_states: M5FollowState::ALL.to_vec(),
        participant_liveness_states: M5PresenceParticipantLiveness::ALL.to_vec(),
        link_states: M5CollaborationLinkState::ALL.to_vec(),
        continuity_postures: M5PresenceContinuityPosture::ALL.to_vec(),
        recording_cues: M5RecordingRetentionCue::ALL.to_vec(),
        presence_actions: M5PresenceAction::ALL.to_vec(),
        export_fields: M5PresenceExportField::ALL.to_vec(),
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5RuntimeBoundaryDowngradeTrigger::CollaborationRoleMasked,
            M5RuntimeBoundaryDowngradeTrigger::FollowStateAmbiguous,
            M5RuntimeBoundaryDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5RuntimeBoundaryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PRESENCE_AVATAR_STACK_SCHEMA_REF,
            M5_ROLE_FOLLOW_BADGE_SCHEMA_REF,
            M5_PRESENCE_AVATAR_STACK_FOLLOW_STATE_REF,
        ]),
        example_resolutions,
        masks_collaboration_role: false,
        leaves_follow_state_ambiguous: false,
        relies_on_avatar_imagery_alone: false,
        drops_presence_when_degraded: false,
    }
}

fn consumer_rows() -> Vec<M5PresenceConsumerRow> {
    use M5CollaborationLinkState as Link;
    use M5CollaborationRole as Role;
    use M5FollowState as Follow;
    use M5PresenceParticipantLiveness as Live;
    use M5RecordingRetentionCue as Rec;

    let mut rows = Vec::new();

    // 1. Collaboration strip — following the presenter on a live link, and a degraded
    //    link that keeps the roster visible with a reconnect action (AC2 example).
    rows.push(base_row(
        M5PresenceConsumerSurface::CollaborationStrip,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Collaboration strip owner",
        "The always-on collaboration strip renders the shared avatar stack and role-or-follow badges so a live session shows who is presenting and that the local user is following, while a degraded link keeps the roster and roles visible with a reconnect action rather than collapsing into a generic session banner",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-presence-strip:001",
        vec![
            case(
                "strip-follow-presenter",
                vec![
                    participant("peer-alpha", Role::Presenter, Follow::PresentingToOthers, Live::Active, false),
                    participant("local-you", Role::Collaborator, Follow::FollowingPresenter, Live::Active, true),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "strip-degraded-visible",
                vec![
                    participant("host-beta", Role::SessionHost, Follow::BeingFollowed, Live::Active, false),
                    participant("local-you", Role::Observer, Follow::NotFollowing, Live::Reconnecting, true),
                ],
                Link::Degraded,
                Rec::Recording,
            ),
        ],
    ));

    // 2. Shared terminal header — a control-holder driving on a live link, and the local
    //    user presenting their own view (view being followed).
    rows.push(base_row(
        M5PresenceConsumerSurface::SharedTerminalHeader,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Shared terminal header owner",
        "The shared terminal header renders the shared components so a control holder driving the terminal reads as presenter and control holder with the local user following, and so the local user presenting their own view reads as being followed with an explicit not-recorded cue",
        M5ShellZoneSlot::BottomPanel,
        "evidence:m5-presence-terminal:001",
        vec![
            case(
                "term-control-holder",
                vec![
                    participant("driver-x", Role::ControlHolder, Follow::PresentingToOthers, Live::Active, false),
                    participant("local-you", Role::Collaborator, Follow::FollowingPresenter, Live::Idle, true),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "term-self-presents",
                vec![
                    participant("local-you", Role::Presenter, Follow::PresentingToOthers, Live::Active, true),
                    participant("watcher-y", Role::Observer, Follow::FollowingPresenter, Live::Active, false),
                ],
                Link::Live,
                Rec::NotRecorded,
            ),
        ],
    ));

    // 3. Shared debug pane — a presenter the local user can choose to follow, and a
    //    reconnecting link that keeps a paused-follow roster visible.
    rows.push(base_row(
        M5PresenceConsumerSurface::SharedDebugPane,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Shared debug pane owner",
        "The shared debug pane renders the shared components so a session with an active presenter offers the local user a follow action, while a reconnecting link keeps the paused-follow roster and roles visible with a reconnect action rather than dropping presence",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-presence-debug:001",
        vec![
            case(
                "debug-follow-available",
                vec![
                    participant("lead-dbg", Role::Presenter, Follow::BeingFollowed, Live::Active, false),
                    participant("local-you", Role::Collaborator, Follow::NotFollowing, Live::Active, true),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "debug-reconnecting",
                vec![
                    participant("local-you", Role::Collaborator, Follow::FollowPaused, Live::Reconnecting, true),
                    participant("peer-dbg", Role::Collaborator, Follow::FollowPaused, Live::Idle, false),
                ],
                Link::Reconnecting,
                Rec::RetentionPending,
            ),
        ],
    ));

    // 4. Review / session header — the local user being followed on a retained session,
    //    and an ended session that keeps the last-known roster and control holder (AC3
    //    local-fallback example).
    rows.push(base_row(
        M5PresenceConsumerSurface::ReviewSessionHeader,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Review / session header owner",
        "The review / session header renders the shared components so a retained review session shows the local host being followed, and so an ended session keeps the last-known roster and the participant who held control visible instead of erasing who was present",
        M5ShellZoneSlot::TitleContextBar,
        "evidence:m5-presence-review:001",
        vec![
            case(
                "review-being-followed",
                vec![
                    participant("local-you", Role::SessionHost, Follow::BeingFollowed, Live::Active, true),
                    participant("author-r", Role::Collaborator, Follow::FollowingPresenter, Live::Active, false),
                ],
                Link::Live,
                Rec::Retained,
            ),
            case(
                "review-ended-last-known",
                vec![
                    participant("local-you", Role::SessionHost, Follow::NotFollowing, Live::LastKnownLocal, true),
                    participant("controller-r", Role::ControlHolder, Follow::NotFollowing, Live::LastKnownLocal, false),
                ],
                Link::SessionEnded,
                Rec::Retained,
            ),
        ],
    ));

    // 5. Presenter HUD — an offline local-fallback where the local presenter and the
    //    co-host control holder stay visible (AC3 local-fallback example), and a live
    //    session where the local user can follow the presenter.
    rows.push(base_row(
        M5PresenceConsumerSurface::PresenterHud,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Presenter HUD owner",
        "The presenter heads-up display renders the shared components so an offline local-fallback keeps the local presenter and the control-holding co-host visible with a reconnect action while recording, and so a live session offers the local observer a follow action",
        M5ShellZoneSlot::TransientOverlay,
        "evidence:m5-presence-hud:001",
        vec![
            case(
                "hud-offline-fallback",
                vec![
                    participant("local-you", Role::Presenter, Follow::PresentingToOthers, Live::LastKnownLocal, true),
                    participant("cohost-h", Role::ControlHolder, Follow::NotFollowing, Live::LastKnownLocal, false),
                ],
                Link::OfflineLocalFallback,
                Rec::Recording,
            ),
            case(
                "hud-live-follow",
                vec![
                    participant("presenter-h", Role::Presenter, Follow::PresentingToOthers, Live::Active, false),
                    participant("local-you", Role::Observer, Follow::NotFollowing, Live::Idle, true),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
        ],
    ));

    // 6. Follow-mode banner — actively following on a live link, and a degraded link with
    //    a paused follow that keeps the presenter visible.
    rows.push(base_row(
        M5PresenceConsumerSurface::FollowModeBanner,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Follow-mode banner owner",
        "The follow-mode banner renders the shared components so an active follow offers a stop-following action with the presenter named, and so a degraded link keeps the paused-follow banner and presenter visible with a reconnect action and an explicit not-recorded cue",
        M5ShellZoneSlot::TitleContextBar,
        "evidence:m5-presence-follow:001",
        vec![
            case(
                "follow-active",
                vec![
                    participant("local-you", Role::Collaborator, Follow::FollowingPresenter, Live::Active, true),
                    participant("speaker-f", Role::Presenter, Follow::PresentingToOthers, Live::Active, false),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "follow-paused-degraded",
                vec![
                    participant("local-you", Role::Collaborator, Follow::FollowPaused, Live::Reconnecting, true),
                    participant("speaker-f", Role::Presenter, Follow::PresentingToOthers, Live::Reconnecting, false),
                ],
                Link::Degraded,
                Rec::NotRecorded,
            ),
        ],
    ));

    // 7. Session roster panel — a full five-role roster on a retained session (all roles
    //    covered), and a reconnecting session where a departed participant stays
    //    accounted for.
    rows.push(base_row(
        M5PresenceConsumerSurface::SessionRosterPanel,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Session roster panel owner",
        "The session roster panel renders the shared components so a full roster distinguishes host, control holder, collaborator, observer, and presenter roles with follow badges, and so a reconnecting session keeps a departed participant accounted for rather than silently dropping them",
        M5ShellZoneSlot::RightInspector,
        "evidence:m5-presence-roster:001",
        vec![
            case(
                "roster-full",
                vec![
                    participant("owner-r", Role::SessionHost, Follow::NotFollowing, Live::Active, false),
                    participant("driver-r", Role::ControlHolder, Follow::PresentingToOthers, Live::Active, false),
                    participant("editor-r", Role::Collaborator, Follow::FollowingPresenter, Live::Idle, false),
                    participant("guest-r", Role::Observer, Follow::FollowingPresenter, Live::Idle, false),
                    participant("local-you", Role::Presenter, Follow::BeingFollowed, Live::Active, true),
                ],
                Link::Live,
                Rec::Retained,
            ),
            case(
                "roster-departed",
                vec![
                    participant("local-you", Role::SessionHost, Follow::NotFollowing, Live::Active, true),
                    participant("left-user", Role::Collaborator, Follow::NotFollowing, Live::Departed, false),
                ],
                Link::Reconnecting,
                Rec::RetentionPending,
            ),
        ],
    ));

    // 8. Activity-center presence — a quiet live session, and an offline local-fallback
    //    that keeps the last-known host visible with a reconnect action.
    rows.push(base_row(
        M5PresenceConsumerSurface::ActivityCenterPresence,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Activity-center presence owner",
        "The activity-center presence entry renders the shared components so a quiet live session lists who is present without inventing a follow claim, and so an offline local-fallback keeps the last-known host visible with a reconnect action while recording",
        M5ShellZoneSlot::ActivityRail,
        "evidence:m5-presence-activity:001",
        vec![
            case(
                "activity-quiet",
                vec![
                    participant("local-you", Role::Observer, Follow::NotFollowing, Live::Idle, true),
                    participant("peer-a", Role::Collaborator, Follow::NotFollowing, Live::Active, false),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "activity-offline",
                vec![
                    participant("local-you", Role::Collaborator, Follow::NotFollowing, Live::LastKnownLocal, true),
                    participant("host-a", Role::SessionHost, Follow::BeingFollowed, Live::LastKnownLocal, false),
                ],
                Link::OfflineLocalFallback,
                Rec::Recording,
            ),
        ],
    ));

    // 9. Shared preview header — following a designer on a live link, and a quiet
    //    live preview with an explicit not-recorded cue.
    rows.push(base_row(
        M5PresenceConsumerSurface::SharedPreviewHeader,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Shared preview header owner",
        "The shared preview header renders the shared components so a live preview session shows the local user following the presenting designer, and so a quiet live preview lists who is present with an explicit not-recorded cue",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-presence-preview:001",
        vec![
            case(
                "preview-follow",
                vec![
                    participant("local-you", Role::Collaborator, Follow::FollowingPresenter, Live::Active, true),
                    participant("designer-p", Role::Presenter, Follow::PresentingToOthers, Live::Active, false),
                ],
                Link::Live,
                Rec::NotApplicable,
            ),
            case(
                "preview-quiet",
                vec![
                    participant("local-you", Role::Observer, Follow::NotFollowing, Live::Idle, true),
                    participant("peer-p", Role::Collaborator, Follow::NotFollowing, Live::Idle, false),
                ],
                Link::Live,
                Rec::NotRecorded,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5PresenceGovernanceReview {
    M5PresenceGovernanceReview {
        one_primitive_carries_presence_role_and_follow: true,
        identity_role_follow_and_presenter_always_shown: true,
        collaboration_role_never_masked: true,
        follow_state_never_ambiguous: true,
        collaboration_visible_through_degraded_flows: true,
        local_fallback_preserves_who_was_present_and_in_control: true,
        presence_never_relies_on_avatar_imagery_alone: true,
        support_export_reconstructs_presence: true,
        no_surface_invents_second_presence_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5PresenceConsumerProjection {
    M5PresenceConsumerProjection {
        collaboration_surfaces_consume_shared_primitive: true,
        presence_resolver_reads_single_participant_source: true,
        role_follow_badges_read_single_state_source: true,
        continuity_reads_single_link_state_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5PresenceProofFreshness {
    M5PresenceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5PresenceReleasePosture {
    M5PresenceReleasePosture {
        release_packet_ref: M5_PRESENCE_AVATAR_STACK_ARTIFACT_REF.to_owned(),
        presence_audit_ref: M5_PRESENCE_AVATAR_STACK_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PRESENCE_AVATAR_STACK_SCHEMA_REF,
        M5_ROLE_FOLLOW_BADGE_SCHEMA_REF,
        M5_PRESENCE_AVATAR_STACK_DOC_REF,
        M5_PRESENCE_AVATAR_STACK_SHELL_ZONE_REF,
        M5_PRESENCE_AVATAR_STACK_COMPONENT_MATRIX_REF,
        M5_PRESENCE_AVATAR_STACK_FOLLOW_STATE_REF,
        M5_PRESENCE_AVATAR_STACK_SESSION_STATE_REF,
    ])
}

/// Builds the canonical M5 presence-avatar-stack primitive packet.
pub fn seeded_m5_presence_avatar_stack_primitive_packet() -> M5PresenceAvatarStackPrimitivePacket {
    M5PresenceAvatarStackPrimitivePacket::new(M5PresenceAvatarStackPrimitivePacketInput {
        packet_id: M5_PRESENCE_AVATAR_STACK_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 presence avatar stack and role-or-follow badge primitive: participant identity, role, follow / presenter state, recording-or-retention cue, and local-fallback continuity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5PresenceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shared debug pane is held at Beta because a slice of shared
/// debug sessions do not yet render the recording-or-retention cue on every profile;
/// every surface stays visible.
pub fn seeded_m5_presence_avatar_stack_primitive_shared_debug_pane_beta_narrowed(
) -> M5PresenceAvatarStackPrimitivePacket {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.packet_id = "m5-presence-avatar-stack-primitive:shared-debug-pane-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PresenceConsumerSurface::SharedDebugPane)
        .expect("shared debug pane row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review / session header is narrowed to Preview pending
/// local-fallback continuity parity proof across every export path; every surface stays
/// visible.
pub fn seeded_m5_presence_avatar_stack_primitive_review_session_header_preview_narrowed(
) -> M5PresenceAvatarStackPrimitivePacket {
    let mut packet = seeded_m5_presence_avatar_stack_primitive_packet();
    packet.packet_id =
        "m5-presence-avatar-stack-primitive:review-session-header-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PresenceConsumerSurface::ReviewSessionHeader)
        .expect("review / session header row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
