//! Unit tests for the overlay/navigation binding.

use super::binding::{
    project_overlay_navigation_binding, OverlaySurfaceTag, PresentationOverlayNavigationBinding,
    ShellZoneTag,
};
use super::corpus::{
    seeded_overlay_navigation_corpus, validate_overlay_navigation_corpus,
    PresentationOverlayBindingCorpus, PresentationOverlayBindingSupportExport,
};
use crate::layout::zone_registry::{
    ZoneDefaults, ZoneRegistry, ZoneRegistryInput, ZoneRegistryLayout,
};
use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, SpeakerNote, WalkthroughSurfaceKind,
    WaypointCompletionState,
};

fn expanded_layout() -> ZoneRegistryLayout {
    ZoneRegistry::new(ZoneDefaults::standard()).layout(ZoneRegistryInput {
        window_width: 1920,
        window_height: 1080,
        split_heavy: false,
        main_workspace_min_width_override: None,
    })
}

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "presentation:checkpoint:bind-unit".into(),
        prior_layout_ref: "window-topology:bind-unit:prior".into(),
        prior_focus_ref: "focus-chain:bind-unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:bind-unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:bind-unit:prior".into(),
        captured_at: "2026-06-20T09:00:00Z".into(),
    }
}

fn waypoint(id: &str, kind: WalkthroughSurfaceKind) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.into(),
        ordinal: 1,
        step_title: "Step".into(),
        surface_kind: kind,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some("src/lib.rs".into()),
        symbol_anchor_ref: Some("fn main".into()),
        branch_workspace_ref: "branch:main@workspace:local".into(),
        boundary_label: BoundaryLabel::Local,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: Some(SpeakerNote::local("note", id, "private prompt")),
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn presenting_session() -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:bind-unit",
        LeaderFollowState::Presenting,
        AudienceScope::SoloRehearsal,
        checkpoint(),
    )
    .focus("wp:bind-unit:1")
    .waypoint(waypoint("wp:bind-unit:1", WalkthroughSurfaceKind::Editor))
    .build()
}

fn binding() -> PresentationOverlayNavigationBinding {
    project_overlay_navigation_binding(&presenting_session(), &expanded_layout())
}

#[test]
fn binding_validates_and_is_a_thin_overlay() {
    let b = binding();
    assert!(b.validate().is_empty(), "{:?}", b.validate());
    assert!(b.preserves_pane_and_navigation_system);
    assert!(b.thin_overlay_not_second_shell);
    assert!(!b.grants_mutation_authority);
    assert!(!b.grants_control_authority);
}

#[test]
fn every_placement_keeps_the_underlying_pane() {
    let b = binding();
    for placement in &b.placements {
        assert!(
            placement.pane_preserved(),
            "{} replaced a pane",
            placement.surface.as_str()
        );
        assert!(!placement.replaces_underlying_pane);
        assert!(placement.underlying_pane_visible);
    }
}

#[test]
fn spotlight_rides_main_workspace_as_an_inset_only() {
    let b = binding();
    let spotlight = b
        .placement(OverlaySurfaceTag::SpotlightFrame)
        .expect("spotlight present when a waypoint is focused");
    assert_eq!(spotlight.host_zone, ShellZoneTag::MainWorkspace);
    // Inset rect is smaller than the full window.
    assert!(spotlight.rect.width < b.window.width);
    assert!(spotlight.rect.height < b.window.height);
    // No other placement rides the main workspace.
    for placement in &b.placements {
        if placement.surface != OverlaySurfaceTag::SpotlightFrame {
            assert_ne!(placement.host_zone, ShellZoneTag::MainWorkspace);
        }
    }
}

#[test]
fn actionable_surfaces_are_command_backed_and_keyboard_reachable() {
    let b = binding();
    for placement in &b.placements {
        assert!(
            placement.command_and_keyboard_ok(),
            "{} is not command-backed / keyboard reachable",
            placement.surface.as_str()
        );
        if placement.is_actionable {
            assert!(placement.command_id.is_some());
            assert!(placement.key_binding_ref.is_some());
        } else {
            // The provenance strip is display-only.
            assert_eq!(placement.surface, OverlaySurfaceTag::ProvenanceStrip);
            assert!(placement.command_id.is_none());
        }
    }
}

#[test]
fn provenance_flows_from_the_navigation_anchor() {
    let b = binding();
    assert!(b.provenance.provenance_visible_under_overlay);
    assert!(b.provenance.reuses_existing_surface);
    assert_eq!(b.provenance.file_path_ref.as_deref(), Some("src/lib.rs"));
    assert_eq!(b.provenance.symbol_anchor_ref.as_deref(), Some("fn main"));
    assert_eq!(b.provenance.boundary_label, BoundaryLabel::Local);
    assert_eq!(
        b.provenance.surface_kind,
        Some(WalkthroughSurfaceKind::Editor)
    );
}

#[test]
fn checkpoint_restores_under_all_triggers() {
    let b = binding();
    assert!(b.checkpoint.enter_checkpoints_prior_layout);
    assert!(b.checkpoint.restores_under_all_triggers);
    assert!(b.checkpoint.no_improvised_shell_on_restore);
    assert!(b.checkpoint.restore_holds());
}

#[test]
fn breakaway_banner_only_appears_when_broken_away() {
    let presenting = binding();
    assert!(presenting
        .placement(OverlaySurfaceTag::BreakawayBanner)
        .is_none());

    let broken = PresentationSessionBuilder::new(
        "presentation:session:bind-broken",
        LeaderFollowState::BrokenAway,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:bind-broken:1")
    .waypoint(waypoint("wp:bind-broken:1", WalkthroughSurfaceKind::Diff))
    .build();
    let b = project_overlay_navigation_binding(&broken, &expanded_layout());
    let banner = b
        .placement(OverlaySurfaceTag::BreakawayBanner)
        .expect("breakaway banner present when broken away");
    assert_eq!(banner.host_zone, ShellZoneTag::TransientOverlay);
    assert!(b.validate().is_empty(), "{:?}", b.validate());
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_overlay_navigation_corpus();
    validate_overlay_navigation_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: PresentationOverlayBindingCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    // A compact-window case demonstrates the floated fallback placement.
    assert!(corpus.summary.fallback_placement_demonstrated);
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_overlay_navigation_corpus();
    let fixture = include_str!(
        "../../../../fixtures/presentation/overlay-and-waypoint/overlay-navigation-corpus.json"
    );
    let parsed: PresentationOverlayBindingCorpus =
        serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/overlay-and-waypoint drifted from the seed corpus; \
         regenerate with the dump_presentation_overlay_navigation example"
    );
}

#[test]
fn support_export_excludes_raw_provenance() {
    let corpus = seeded_overlay_navigation_corpus();
    let export = PresentationOverlayBindingSupportExport::from_corpus(
        "support-export:presentation-overlay-binding:001",
        "2026-06-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.cases.len());

    let export_json = serde_json::to_string(&export).unwrap();
    for case in &corpus.cases {
        // No scenario copy, accessible labels, or file paths leak into support.
        assert!(!export_json.contains(&case.scenario_label));
        if let Some(path) = &case.binding.provenance.file_path_ref {
            assert!(
                !export_json.contains(path),
                "support export leaked a file path for {}",
                case.case_id
            );
        }
        for placement in &case.binding.placements {
            assert!(
                !export_json.contains(&placement.accessible_label),
                "support export leaked an accessible label for {}",
                case.case_id
            );
        }
    }
}
