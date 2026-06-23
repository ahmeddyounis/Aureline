//! Unit tests for presentation-overlay accessibility / boundary conformance.

use crate::a11y::tree_contract::{RoleConfidence, SupportState};
use crate::presentation_mode::{
    AudienceScope, BoundaryLabel, FollowWaypoint, LeaderFollowState, PresentationSession,
    PresentationSessionBuilder, RestoreCheckpoint, WalkthroughSurfaceKind, WaypointCompletionState,
};

use super::conformance::{
    project_accessibility_report, AccessibilityProjectionInputs, HighZoomReflow,
    PresentationA11yClass, PresentationA11yViolation, PresentationSurfaceTag, ZoomTier,
};
use super::corpus::{
    presentation_a11y_support_export, seeded_presentation_a11y_corpus,
    validate_presentation_a11y_corpus, PresentationA11yCorpus,
};

fn checkpoint() -> RestoreCheckpoint {
    RestoreCheckpoint {
        checkpoint_id: "presentation:checkpoint:a11y-unit".into(),
        prior_layout_ref: "window-topology:a11y-unit:prior".into(),
        prior_focus_ref: "focus-chain:a11y-unit:prior".into(),
        prior_panel_visibility_ref: "panel-visibility:a11y-unit:prior".into(),
        accessibility_posture_ref: "a11y-posture:a11y-unit:prior".into(),
        captured_at: "2026-06-20T09:00:00Z".into(),
    }
}

fn waypoint(id: &str, boundary: BoundaryLabel) -> FollowWaypoint {
    FollowWaypoint {
        waypoint_id: id.into(),
        ordinal: 1,
        step_title: "Anchor".into(),
        surface_kind: WalkthroughSurfaceKind::Editor,
        target_object_ref: format!("obj:{id}"),
        file_path_ref: Some("src/lib.rs".into()),
        symbol_anchor_ref: Some("fn main".into()),
        branch_workspace_ref: "branch:main@workspace:local".into(),
        boundary_label: boundary,
        zoom_layout_hint_ref: None,
        reveal_action_ref: None,
        completion_state: WaypointCompletionState::Current,
        speaker_note: None,
        reuses_existing_surface: true,
        creates_parallel_artifact: false,
    }
}

fn session(state: LeaderFollowState, boundary: BoundaryLabel) -> PresentationSession {
    PresentationSessionBuilder::new(
        "presentation:session:a11y-unit",
        state,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:a11y-unit:1")
    .waypoint(waypoint("wp:a11y-unit:1", boundary))
    .build()
}

#[test]
fn class_maps_onto_the_shell_accessibility_vocabulary() {
    assert_eq!(
        PresentationA11yClass::FullyAccessible.to_support_state(),
        SupportState::FullAccessible
    );
    assert_eq!(
        PresentationA11yClass::DegradedAnnounced.to_support_state(),
        SupportState::DegradedAccessible
    );
    assert_eq!(
        PresentationA11yClass::NonConformant.to_support_state(),
        SupportState::UnsupportedBlocked
    );
    assert_eq!(
        PresentationA11yClass::FullyAccessible.to_role_confidence(),
        RoleConfidence::Exact
    );
    assert_eq!(
        HighZoomReflow::Reflows.to_support_state(),
        SupportState::FullAccessible
    );
    assert_eq!(
        HighZoomReflow::SummarizedReachable.to_support_state(),
        SupportState::DegradedAccessible
    );
}

#[test]
fn standard_zoom_overlay_is_fully_accessible() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(
        report.conformance_class,
        PresentationA11yClass::FullyAccessible
    );
    assert_eq!(report.support_state, SupportState::FullAccessible);
    assert!(report.keyboard_complete);
    assert!(!report.pointer_only);
    assert!(report.screen_reader_reachable);
    assert!(report.reduced_motion_respected);
    assert!(report.high_zoom_supported);
    assert!(report.focus_order_contiguous);
    assert!(report.no_focus_trap);
    assert!(report.boundary_labels_preserved);
    assert!(report.accessible_labels_complete);
    // The required surfaces are present and the spotlight rides on a focused step.
    assert!(report
        .surface(PresentationSurfaceTag::PresenterBar)
        .is_some());
    assert!(report
        .surface(PresentationSurfaceTag::WaypointRail)
        .is_some());
    assert!(report
        .surface(PresentationSurfaceTag::ProvenanceStrip)
        .is_some());
    assert!(report
        .surface(PresentationSurfaceTag::SpotlightFrame)
        .is_some());
}

#[test]
fn focus_ring_is_a_contiguous_order_over_actionable_surfaces() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    let mut indices: Vec<u32> = report
        .surfaces
        .iter()
        .filter_map(|s| s.focus_order_index)
        .collect();
    indices.sort_unstable();
    let actionable = report.surfaces.iter().filter(|s| s.is_actionable).count();
    assert_eq!(indices, (1..=actionable as u32).collect::<Vec<_>>());
    // The provenance strip is display-only: reachable, but not in the focus ring.
    let provenance = report
        .surface(PresentationSurfaceTag::ProvenanceStrip)
        .unwrap();
    assert!(!provenance.is_actionable);
    assert!(provenance.focus_order_index.is_none());
    assert!(provenance.screen_reader_reachable);
}

#[test]
fn high_zoom_summarizes_dense_surfaces_without_dropping_them() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Shared);
    let report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::high_zoom());
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(
        report.conformance_class,
        PresentationA11yClass::DegradedAnnounced
    );
    // The dense agenda rail and audience strip summarize; both stay reachable.
    for tag in [
        PresentationSurfaceTag::WaypointRail,
        PresentationSurfaceTag::AudienceStrip,
    ] {
        let surface = report.surface(tag).unwrap();
        assert_eq!(
            surface.high_zoom_reflow,
            HighZoomReflow::SummarizedReachable
        );
        assert_eq!(surface.support_state, SupportState::DegradedAccessible);
        assert!(surface.keyboard_reachable);
        assert!(surface.screen_reader_reachable);
        assert!(!surface.pointer_only);
        assert!(!surface.traps_focus);
    }
    assert!(report.summarized_surface_count() >= 2);
    assert!(report.high_zoom_supported);
}

#[test]
fn breakaway_banner_joins_the_focus_ring_when_broken_away() {
    let broken = session(LeaderFollowState::BrokenAway, BoundaryLabel::Shared);
    let report = project_accessibility_report(&broken, &AccessibilityProjectionInputs::standard());
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    let banner = report
        .surface(PresentationSurfaceTag::BreakawayBanner)
        .expect("breakaway banner present when broken away");
    assert!(banner.is_actionable);
    assert!(banner.focus_order_index.is_some());
    assert!(banner.keyboard_reachable);
    assert!(!banner.traps_focus);

    // When presenting, the banner is absent.
    let presenting = session(LeaderFollowState::Presenting, BoundaryLabel::Shared);
    let presenting_report =
        project_accessibility_report(&presenting, &AccessibilityProjectionInputs::standard());
    assert!(presenting_report
        .surface(PresentationSurfaceTag::BreakawayBanner)
        .is_none());
}

#[test]
fn boundary_labels_are_preserved_not_flattened() {
    for boundary in [
        BoundaryLabel::Local,
        BoundaryLabel::Remote,
        BoundaryLabel::Shared,
    ] {
        let session = session(LeaderFollowState::Presenting, boundary);
        let report =
            project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
        assert_eq!(
            report.boundary_posture.current_boundary_label,
            Some(boundary)
        );
        assert_eq!(
            report.boundary_posture.distinct_boundary_labels,
            vec![boundary]
        );
        assert!(report.boundary_posture.boundary_labels_visible);
        assert!(!report.boundary_posture.flattened_to_generic);
        // Source-bearing surfaces carry the boundary label explicitly.
        let provenance = report
            .surface(PresentationSurfaceTag::ProvenanceStrip)
            .unwrap();
        assert_eq!(provenance.boundary_label, Some(boundary));
        let spotlight = report
            .surface(PresentationSurfaceTag::SpotlightFrame)
            .unwrap();
        assert_eq!(spotlight.boundary_label, Some(boundary));
    }
}

#[test]
fn mixed_boundary_labels_are_kept_distinct() {
    let session = PresentationSessionBuilder::new(
        "presentation:session:a11y-mixed",
        LeaderFollowState::Presenting,
        AudienceScope::SharedWorkspace,
        checkpoint(),
    )
    .focus("wp:mixed:2")
    .waypoint(waypoint("wp:mixed:1", BoundaryLabel::Local))
    .waypoint(waypoint("wp:mixed:2", BoundaryLabel::Shared))
    .waypoint(waypoint("wp:mixed:3", BoundaryLabel::Remote))
    .build();
    let report = project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    assert!(report.validate().is_empty(), "{:?}", report.validate());
    assert_eq!(
        report.boundary_posture.current_boundary_label,
        Some(BoundaryLabel::Shared)
    );
    assert_eq!(
        report.boundary_posture.distinct_boundary_labels,
        vec![
            BoundaryLabel::Local,
            BoundaryLabel::Remote,
            BoundaryLabel::Shared
        ]
    );
}

#[test]
fn a_pointer_only_surface_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    report.surfaces[0].pointer_only = true;
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationA11yViolation::PointerOrMotionOnly { .. })));
}

#[test]
fn a_focus_trap_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    report.surfaces[0].traps_focus = true;
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationA11yViolation::FocusTrapped { .. })));
}

#[test]
fn an_erased_boundary_label_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Remote);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    // Tamper: strip the boundary label off the provenance strip.
    let provenance = report
        .surfaces
        .iter_mut()
        .find(|s| s.surface == PresentationSurfaceTag::ProvenanceStrip)
        .unwrap();
    provenance.boundary_label = None;
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationA11yViolation::BoundaryLabelErased { .. })));
}

#[test]
fn a_broken_focus_ring_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    // Tamper: introduce a gap in the focus ring.
    report.surfaces[0].focus_order_index = Some(99);
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationA11yViolation::FocusOrderBroken)));
}

#[test]
fn a_class_claiming_fully_accessible_at_high_zoom_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Shared);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::high_zoom());
    // Tamper: claim fully accessible despite a summarized surface.
    report.conformance_class = PresentationA11yClass::FullyAccessible;
    report.support_state = SupportState::FullAccessible;
    report.role_confidence = RoleConfidence::Exact;
    let violations = report.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PresentationA11yViolation::ConformanceClassMismatch { .. }
    )));
}

#[test]
fn an_aggregate_flag_lie_fails_validation() {
    let session = session(LeaderFollowState::Presenting, BoundaryLabel::Local);
    let mut report =
        project_accessibility_report(&session, &AccessibilityProjectionInputs::standard());
    // Tamper: claim a contiguous ring flag while breaking the ring.
    report.focus_order_contiguous = false;
    let violations = report.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PresentationA11yViolation::AggregateFlagMismatch)));
}

#[test]
fn seeded_corpus_validates_and_round_trips() {
    let corpus = seeded_presentation_a11y_corpus();
    validate_presentation_a11y_corpus(&corpus).expect("seeded corpus must validate");

    let json = serde_json::to_string(&corpus).unwrap();
    let parsed: PresentationA11yCorpus = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed, corpus);

    assert!(corpus.summary.fully_accessible_demonstrated);
    assert!(corpus.summary.degraded_announced_demonstrated);
    assert!(corpus.summary.breakaway_banner_demonstrated);
    assert!(corpus.summary.spotlight_demonstrated);
    assert!(corpus.summary.all_reports_valid);
    assert!(corpus.summary.all_keyboard_complete);
    assert!(corpus.summary.none_pointer_only);
    assert!(corpus.summary.all_screen_reader_reachable);
    assert!(corpus.summary.all_reduced_motion_respected);
    assert!(corpus.summary.all_high_zoom_supported);
    assert!(corpus.summary.all_focus_order_contiguous);
    assert!(corpus.summary.none_traps_focus);
    assert!(corpus.summary.all_boundary_labels_preserved);
    assert!(corpus.summary.all_accessible_labels_complete);
    // Every boundary label is exercised.
    assert!(corpus
        .summary
        .boundary_labels_covered
        .contains(&BoundaryLabel::Local));
    assert!(corpus
        .summary
        .boundary_labels_covered
        .contains(&BoundaryLabel::Remote));
    assert!(corpus
        .summary
        .boundary_labels_covered
        .contains(&BoundaryLabel::Shared));
    // Both zoom tiers are covered.
    assert!(corpus
        .summary
        .zoom_tiers_covered
        .contains(&ZoomTier::Standard));
    assert!(corpus
        .summary
        .zoom_tiers_covered
        .contains(&ZoomTier::HighZoom));
}

#[test]
fn checked_in_fixtures_match_the_seed_projection() {
    let corpus = seeded_presentation_a11y_corpus();
    let fixture = include_str!(
        "../../../../../fixtures/presentation/a11y-and-motion/accessibility-corpus.json"
    );
    let parsed: PresentationA11yCorpus = serde_json::from_str(fixture).expect("fixture parses");
    assert_eq!(
        parsed, corpus,
        "fixtures/presentation/a11y-and-motion drifted from the seed corpus; \
         regenerate with the dump_presentation_accessibility example"
    );
}

#[test]
fn support_export_excludes_labels_and_refs_but_keeps_boundary_posture() {
    let corpus = seeded_presentation_a11y_corpus();
    let export = presentation_a11y_support_export(
        "support-export:presentation-a11y:001",
        "2026-06-20T00:00:00Z",
        &corpus,
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.rows.len(), corpus.cases.len());

    let export_json = serde_json::to_string(&export).unwrap();
    for case in &corpus.cases {
        assert!(!export_json.contains(&case.scenario_label));
        for surface in &case.report.surfaces {
            assert!(
                !export_json.contains(&surface.accessible_label),
                "support export leaked an accessible label for {}",
                case.case_id
            );
        }
    }
    // The boundary posture is intentionally preserved, not flattened.
    for (row, case) in export.rows.iter().zip(&corpus.cases) {
        assert_eq!(
            row.distinct_boundary_labels,
            case.report.boundary_posture.distinct_boundary_labels
        );
        assert_eq!(row.conformance_class, case.report.conformance_class);
    }
}
