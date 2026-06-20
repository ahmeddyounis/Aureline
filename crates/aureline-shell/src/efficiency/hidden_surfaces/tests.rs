//! Unit coverage for the hidden-surface render-suppression policy.

use super::*;

fn requested_all() -> HiddenWorkRequest {
    HiddenWorkRequest {
        paint_passes: 6,
        animation_ticks: 12,
        rich_refreshes: 4,
        speculative_polls: 8,
        correctness_polls: 4,
    }
}

fn input(class: HiddenSurfaceClass, visibility: VisibilityState) -> HiddenSurfaceInput {
    HiddenSurfaceInput {
        surface_id: format!("surface.{}", class.as_str()),
        surface_class: class,
        visibility_state: visibility,
        requested: requested_all(),
    }
}

fn channel<'a>(
    decision: &'a HiddenSurfaceDecision,
    channel: HiddenWorkChannel,
) -> &'a HiddenWorkChannelDecision {
    decision
        .channels
        .iter()
        .find(|d| d.channel == channel.as_str())
        .expect("channel present")
}

#[test]
fn hidden_surface_suppresses_paint_animation_refresh_and_speculative_polling() {
    for class in HiddenSurfaceClass::ALL {
        let decision = HiddenSurfaceDecision::decide(
            &input(class, VisibilityState::HiddenTab),
            EfficiencyState::Nominal,
        );
        assert!(decision.hidden, "{class:?} should be hidden");
        for channel_decision in &decision.channels {
            if channel_decision.correctness_critical {
                continue;
            }
            assert_eq!(
                channel_decision.committed_units, 0,
                "{class:?} channel {} kept work alive while hidden",
                channel_decision.channel
            );
        }
        assert!(
            !decision.violates_hidden_pane_policy(),
            "{class:?} must not violate hidden-pane policy"
        );
    }
}

#[test]
fn offscreen_and_collapsed_surfaces_are_treated_as_hidden() {
    for visibility in [
        VisibilityState::OccludedWindow,
        VisibilityState::CollapsedSplit,
        VisibilityState::DetachedOffscreen,
    ] {
        let decision = HiddenSurfaceDecision::decide(
            &input(HiddenSurfaceClass::Preview, visibility),
            EfficiencyState::Nominal,
        );
        assert!(
            decision.hidden,
            "{visibility:?} should suppress like hidden"
        );
        assert_eq!(
            channel(&decision, HiddenWorkChannel::Paint).committed_units,
            0
        );
        assert_eq!(
            channel(&decision, HiddenWorkChannel::Animation).committed_units,
            0
        );
    }
}

#[test]
fn correctness_channel_is_throttled_to_a_floor_never_dropped() {
    // A running notebook cell's completion tracking must survive hiding.
    for state in [
        EfficiencyState::Nominal,
        EfficiencyState::EfficiencyAware,
        EfficiencyState::ThermalConstrained,
        EfficiencyState::ProtectCore,
        EfficiencyState::Recovery,
    ] {
        let decision = HiddenSurfaceDecision::decide(
            &input(HiddenSurfaceClass::Notebook, VisibilityState::HiddenTab),
            state,
        );
        let correctness = channel(&decision, HiddenWorkChannel::CorrectnessPoll);
        assert!(
            correctness.committed_units >= 1,
            "correctness poll dropped to zero under {state:?}"
        );
        assert!(decision.correctness_preserved);
        // Under protect-core the floor is the minimum.
        if matches!(state, EfficiencyState::ProtectCore) {
            assert_eq!(correctness.committed_units, 1);
        }
    }
}

#[test]
fn resuming_a_hidden_surface_restores_state_without_rerun_or_corruption() {
    for class in HiddenSurfaceClass::ALL {
        let decision = HiddenSurfaceDecision::decide(
            &input(class, VisibilityState::DetachedOffscreen),
            EfficiencyState::ProtectCore,
        );
        assert!(
            decision.resume.restores_without_rerun,
            "{class:?} reran work"
        );
        assert!(
            decision.resume.restores_without_cache_corruption,
            "{class:?} corrupted cache"
        );
        assert!(decision.durability_preserved);
        assert!(decision.correctness_preserved);
        assert!(!decision.resume.resume_token_kind.is_empty());
    }
}

#[test]
fn visible_inactive_preview_throttles_animation_and_refresh_but_keeps_painting() {
    let decision = HiddenSurfaceDecision::decide(
        &input(
            HiddenSurfaceClass::Preview,
            VisibilityState::VisibleBackground,
        ),
        EfficiencyState::EfficiencyAware,
    );
    assert!(!decision.hidden);
    // Still visible, so it must keep painting.
    let paint = channel(&decision, HiddenWorkChannel::Paint);
    assert_eq!(paint.committed_units, paint.requested_units);
    // Decorative motion and rich refresh throttle while inactive.
    let animation = channel(&decision, HiddenWorkChannel::Animation);
    assert!(animation.committed_units < animation.requested_units);
    assert!(animation.committed_units > 0);
    assert_eq!(
        animation.disposition,
        HiddenWorkDisposition::Throttled.as_str()
    );
}

#[test]
fn active_focused_surface_is_not_suppressed() {
    let decision = HiddenSurfaceDecision::decide(
        &input(
            HiddenSurfaceClass::Notebook,
            VisibilityState::VisibleFocused,
        ),
        EfficiencyState::ThermalConstrained,
    );
    assert!(!decision.hidden);
    assert_eq!(decision.total_saved_units, 0);
    for channel_decision in &decision.channels {
        assert_eq!(
            channel_decision.disposition,
            HiddenWorkDisposition::Maintained.as_str()
        );
    }
}

#[test]
fn audit_attributes_saved_work_to_specific_surface_classes() {
    let surfaces = vec![
        input(HiddenSurfaceClass::Notebook, VisibilityState::HiddenTab),
        input(HiddenSurfaceClass::Trace, VisibilityState::HiddenTab),
        input(HiddenSurfaceClass::Preview, VisibilityState::CollapsedSplit),
    ];
    let audit = HiddenSurfaceSuppressionAudit::for_surfaces(
        EfficiencyState::ThermalConstrained,
        &surfaces,
        "2026-06-20T15:00:00Z",
    );
    assert!(audit.passes_policy);
    assert!(audit.all_resumes_correct);
    assert_eq!(audit.hidden_surface_count, 3);
    assert!(audit.total_saved_units > 0);
    assert_eq!(audit.saved_by_class.len(), 3);
    // Every saved unit is attributed to exactly one class.
    let attributed: u32 = audit
        .saved_by_class
        .iter()
        .map(|saving| saving.saved_units_total)
        .sum();
    assert_eq!(attributed, audit.total_saved_units);

    let trace = HiddenSurfaceEnergyTrace::from_audit(&audit, "window");
    assert_eq!(trace.total_saved_units, audit.total_saved_units);
    assert_eq!(trace.trace_marks.len(), 3);
    assert_eq!(
        trace.total_saved_paint_passes
            + trace.total_saved_animation_ticks
            + trace.total_saved_refreshes
            + trace.total_saved_polls,
        audit.total_saved_units
    );
}

#[test]
fn audit_agrees_with_frozen_hidden_pane_render_policy() {
    let audit = HiddenSurfaceSuppressionAudit::for_surfaces(
        EfficiencyState::ProtectCore,
        &[
            input(HiddenSurfaceClass::Notebook, VisibilityState::HiddenTab),
            input(
                HiddenSurfaceClass::Incident,
                VisibilityState::DetachedOffscreen,
            ),
        ],
        "2026-06-20T15:00:00Z",
    );
    let render_audit = audit.as_hidden_pane_render_audit();
    assert!(render_audit.passes_hidden_pane_policy);
    assert_eq!(render_audit.hidden_pane_render_violation_count, 0);
    assert_eq!(render_audit.hidden_surface_count, 2);
}

#[test]
fn diagnostics_projection_reports_policy_pass_and_savings() {
    let audit = HiddenSurfaceSuppressionAudit::for_surfaces(
        EfficiencyState::EfficiencyAware,
        &[input(
            HiddenSurfaceClass::DocsBrowser,
            VisibilityState::HiddenTab,
        )],
        "2026-06-20T15:00:00Z",
    );
    let diagnostics = HiddenSurfaceDiagnosticsProjection::from_audit(&audit);
    assert!(diagnostics.passes_policy);
    assert!(diagnostics.all_resumes_correct);
    assert!(diagnostics.durability_preserved);
    assert!(diagnostics.total_saved_units > 0);
    assert!(diagnostics
        .protected_interactions_preserved
        .contains(&"save".to_owned()));
    assert_eq!(
        diagnostics.energy_trace_ref,
        HIDDEN_SURFACE_ENERGY_TRACE_RECORD_KIND
    );
}

#[test]
fn negative_drill_audit_flags_a_hidden_pane_that_still_painted() {
    // Construct a decision that breaks the invariant by hand: a hidden surface
    // whose paint channel still committed work. The audit must flag it.
    let mut decision = HiddenSurfaceDecision::decide(
        &input(HiddenSurfaceClass::Preview, VisibilityState::HiddenTab),
        EfficiencyState::Nominal,
    );
    let paint = decision
        .channels
        .iter_mut()
        .find(|d| d.channel == HiddenWorkChannel::Paint.as_str())
        .expect("paint channel");
    paint.committed_units = 3;
    paint.disposition = HiddenWorkDisposition::Maintained.as_str().to_owned();

    assert!(decision.violates_hidden_pane_policy());
    let audit = HiddenSurfaceSuppressionAudit::from_decisions(
        EfficiencyState::Nominal,
        vec![decision],
        "2026-06-20T15:00:00Z",
    );
    assert!(!audit.passes_policy);
    assert_eq!(audit.hidden_pane_violation_count, 1);
    assert!(
        !audit
            .as_hidden_pane_render_audit()
            .passes_hidden_pane_policy
    );
}

#[test]
fn seeded_cases_all_pass_policy_and_round_trip() {
    for case in seeded_hidden_surface_cases() {
        assert!(
            case.audit.passes_policy,
            "scenario {} should pass policy",
            case.scenario_id
        );
        assert!(case.audit.all_resumes_correct);
        assert!(case.audit.preserves_durability_truth());

        // Re-deriving from the surfaces reproduces the stored projections.
        let audit = HiddenSurfaceSuppressionAudit::for_surfaces(
            case.efficiency_state,
            &case.surfaces,
            &case.observed_at,
        );
        assert_eq!(audit, case.audit, "audit drifted in {}", case.scenario_id);
        let trace = HiddenSurfaceEnergyTrace::from_audit(&audit, &case.window_label);
        assert_eq!(
            trace, case.energy_trace,
            "trace drifted in {}",
            case.scenario_id
        );
        let diagnostics = HiddenSurfaceDiagnosticsProjection::from_audit(&audit);
        assert_eq!(
            diagnostics, case.diagnostics,
            "diagnostics drifted in {}",
            case.scenario_id
        );

        // Round-trip through JSON.
        let json = serde_json::to_string(&case).expect("serialize case");
        let restored: HiddenSurfaceCase = serde_json::from_str(&json).expect("deserialize case");
        assert_eq!(restored, case);
    }
}
