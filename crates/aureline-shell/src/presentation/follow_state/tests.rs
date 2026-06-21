//! Unit tests for the cross-client follow-state truth.

use super::corpus::{
    follow_state_support_export, seeded_follow_state_corpus, validate_follow_state_corpus,
    FollowStateCorpus,
};
use super::state::{
    project_follow_state_truth, ClientFollowInput, ClientSurface, FollowMode, FollowStateViolation,
    LivenessClass, RecoveryAction, RecoveryKind, SnapshotIdentity, SnapshotStalenessReason,
};
use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, WalkthroughSurfaceKind, WaypointCompletionState,
};

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "presentation:checkpoint:follow-unit".into(),
        prior_layout_ref: "window-topology:follow-unit:prior".into(),
        prior_focus_ref: "focus-chain:follow-unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:follow-unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:follow-unit:prior".into(),
        captured_at: "2026-06-20T09:00:00Z".into(),
    }
}

fn waypoint() -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: "wp:follow-unit:1".into(),
        ordinal: 1,
        step_title: "Anchor".into(),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: "obj:wp:follow-unit:1".into(),
        file_path_ref: Some("src/lib.rs".into()),
        symbol_anchor_ref: Some("fn main".into()),
        branch_workspace_ref: "branch:main@workspace:local".into(),
        boundary_label: BoundaryLabel::Shared,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:follow-unit",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:follow-unit:1")
    .waypoint(waypoint())
    .build()
}

#[test]
fn follow_mode_liveness_is_explicit_not_inferred() {
    assert_eq!(FollowMode::Presenting.liveness(), LivenessClass::Live);
    assert_eq!(FollowMode::FollowingLive.liveness(), LivenessClass::Live);
    assert_eq!(
        FollowMode::BrokenAway.liveness(),
        LivenessClass::Independent
    );
    assert_eq!(
        FollowMode::RequestingFollow.liveness(),
        LivenessClass::Independent
    );
    assert_eq!(
        FollowMode::RequestingTakeOver.liveness(),
        LivenessClass::Live
    );
    assert_eq!(
        FollowMode::CachedSnapshot.liveness(),
        LivenessClass::CachedSnapshot
    );
}

#[test]
fn leader_follow_state_maps_into_the_cross_client_vocabulary() {
    assert_eq!(
        FollowMode::from_leader_follow_state(LeaderFollowState::Presenting),
        FollowMode::Presenting
    );
    assert_eq!(
        FollowMode::from_leader_follow_state(LeaderFollowState::FollowingPresenter),
        FollowMode::FollowingLive
    );
    assert_eq!(
        FollowMode::from_leader_follow_state(LeaderFollowState::BrokenAway),
        FollowMode::BrokenAway
    );
    assert_eq!(
        FollowMode::from_leader_follow_state(LeaderFollowState::RequestingFollow),
        FollowMode::RequestingFollow
    );
}

#[test]
fn live_session_validates_and_reads_live_on_every_client() {
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::following(ClientSurface::Browser),
        ClientFollowInput::following(ClientSurface::Companion),
    ];
    let truth = project_follow_state_truth(&session(), &inputs);
    assert!(truth.validate().is_empty(), "{:?}", truth.validate());
    for surface in [
        ClientSurface::Desktop,
        ClientSurface::Browser,
        ClientSurface::Companion,
    ] {
        let view = truth.client_view(surface).expect("view present");
        assert_eq!(view.liveness, LivenessClass::Live);
        assert!(view.snapshot_identity.is_none());
        assert!(view.breakaway_banner.is_none());
    }
    assert!(!truth.grants_mutation_authority);
    assert!(!truth.grants_control_authority);
}

#[test]
fn breakaway_view_carries_a_durable_banner_and_return_path() {
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::broken_away(ClientSurface::Browser, "obj:detour"),
    ];
    let truth = project_follow_state_truth(&session(), &inputs);
    assert!(truth.validate().is_empty(), "{:?}", truth.validate());
    let view = truth.client_view(ClientSurface::Browser).unwrap();
    assert_eq!(view.follow_mode, FollowMode::BrokenAway);
    assert_eq!(view.liveness, LivenessClass::Independent);
    let banner = view
        .breakaway_banner
        .as_ref()
        .expect("durable banner present");
    assert!(banner.durable);
    assert_eq!(
        banner.return_to_presenter.kind,
        RecoveryKind::ReturnToPresenter
    );
    assert!(!banner.presenter_anchor_ref.is_empty());
    assert_eq!(view.recovery_kinds(), vec![RecoveryKind::ReturnToPresenter]);
}

#[test]
fn cached_snapshot_identifies_itself_and_never_claims_live() {
    let identity = SnapshotIdentity::new(
        "snapshot:captured:t0",
        SnapshotStalenessReason::ConnectionLost,
        true,
    );
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::cached_snapshot(ClientSurface::Companion, "obj:stale", identity),
    ];
    let truth = project_follow_state_truth(&session(), &inputs);
    assert!(truth.validate().is_empty(), "{:?}", truth.validate());
    assert!(truth.no_snapshot_implies_live);
    let view = truth.client_view(ClientSurface::Companion).unwrap();
    assert_eq!(view.liveness, LivenessClass::CachedSnapshot);
    let snap = view.snapshot_identity.as_ref().expect("snapshot identity");
    assert!(snap.labeled_as_snapshot);
    assert!(!snap.claims_live_shared_route);
    // The cached view can refresh to live or return to the presenter.
    assert_eq!(
        view.recovery_kinds(),
        vec![
            RecoveryKind::RefreshLiveRoute,
            RecoveryKind::ReturnToPresenter
        ]
    );
}

#[test]
fn a_snapshot_that_claims_live_fails_validation() {
    let mut identity = SnapshotIdentity::new(
        "snapshot:captured:t0",
        SnapshotStalenessReason::ProviderOffline,
        true,
    );
    // Tamper: pretend the cached snapshot is a live shared route.
    identity.claims_live_shared_route = true;
    let inputs = [ClientFollowInput::cached_snapshot(
        ClientSurface::Companion,
        "obj:stale",
        identity,
    )];
    let truth = project_follow_state_truth(&session(), &inputs);
    let violations = truth.validate();
    assert!(
        violations.contains(&FollowStateViolation::SnapshotImpliesLive {
            surface: ClientSurface::Companion,
        })
    );
}

#[test]
fn recovery_actions_are_canonical_across_clients() {
    // Two different clients in the same independent mode get identical actions.
    let inputs = [
        ClientFollowInput::broken_away(ClientSurface::Browser, "obj:a"),
        ClientFollowInput::broken_away(ClientSurface::Companion, "obj:b"),
    ];
    let truth = project_follow_state_truth(&session(), &inputs);
    let browser = truth.client_view(ClientSurface::Browser).unwrap();
    let companion = truth.client_view(ClientSurface::Companion).unwrap();
    assert_eq!(browser.recovery_actions, companion.recovery_actions);
    for action in &browser.recovery_actions {
        assert_eq!(*action, RecoveryAction::canonical(action.kind));
    }
}

#[test]
fn take_over_is_a_distinct_state_that_stays_live() {
    let inputs = [
        ClientFollowInput::presenting(ClientSurface::Desktop),
        ClientFollowInput::requesting_take_over(ClientSurface::Browser),
    ];
    let truth = project_follow_state_truth(&session(), &inputs);
    assert!(truth.validate().is_empty(), "{:?}", truth.validate());
    let view = truth.client_view(ClientSurface::Browser).unwrap();
    assert_eq!(view.follow_mode, FollowMode::RequestingTakeOver);
    assert_eq!(view.liveness, LivenessClass::Live);
    assert!(!truth.grants_control_authority);
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_follow_state_corpus();
    validate_follow_state_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: FollowStateCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    assert!(corpus.summary.cached_snapshot_demonstrated);
    assert!(corpus.summary.breakaway_demonstrated);
    assert!(corpus.summary.take_over_demonstrated);
    assert!(corpus.summary.no_snapshot_implies_live);
    assert!(corpus.summary.no_inferred_state);
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_follow_state_corpus();
    let fixture = include_str!(
        "../../../../../fixtures/presentation/browser-and-companion-follow/follow-state-truth-corpus.json"
    );
    let parsed: FollowStateCorpus = serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/browser-and-companion-follow drifted from the seed corpus; \
         regenerate with the dump_presentation_follow_state example"
    );
}

#[test]
fn support_export_excludes_anchor_refs_and_labels() {
    let corpus = seeded_follow_state_corpus();
    let export = follow_state_support_export(
        "support-export:presentation-follow-state:001",
        "2026-06-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);

    // One row per client view across the corpus.
    let expected_rows: usize = corpus
        .cases
        .iter()
        .map(|c| c.truth.client_views.len())
        .sum();
    assert_eq!(export.rows.len(), expected_rows);

    let export_json = serde_json::to_string(&export).unwrap();
    for case in &corpus.cases {
        assert!(!export_json.contains(&case.scenario_label));
        if let Some(anchor) = &case.truth.presenter_anchor_ref {
            assert!(
                !export_json.contains(anchor),
                "support export leaked a presenter anchor for {}",
                case.case_id
            );
        }
        for view in &case.truth.client_views {
            for action in &view.recovery_actions {
                assert!(
                    !export_json.contains(&action.accessible_label),
                    "support export leaked an accessible label for {}",
                    case.case_id
                );
            }
        }
    }
}
