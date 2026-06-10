//! Integration test: the embedded qualification packets parse and validate.

use aureline_profiler::{
    current_hotspot_workspace_qualification, current_memory_analysis_qualification,
    current_profile_compare_qualification, current_profile_launcher_qualification,
    current_regression_baseline_qualification, current_trace_viewer_qualification,
};

// --- Profile launcher packet (M05-045) ---

#[test]
fn embedded_profile_launcher_packet_parses() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.launchers.is_empty());
    assert!(!packet.attach_sheets.is_empty());
    assert!(!packet.capture_modes.is_empty());
    assert!(!packet.storage_locations.is_empty());
}

#[test]
fn embedded_profile_launcher_packet_has_no_violations() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_profile_launcher_summary_matches_computed() {
    let packet = current_profile_launcher_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- Hotspot workspace packet (M05-046) ---

#[test]
fn embedded_hotspot_workspace_packet_parses() {
    let packet = current_hotspot_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.flamegraph_rows.is_empty());
    assert!(!packet.call_tree_rows.is_empty());
    assert!(!packet.session_strips.is_empty());
    assert!(!packet.mapping_quality_badges.is_empty());
    assert!(!packet.source_navigations.is_empty());
}

#[test]
fn embedded_hotspot_workspace_packet_has_no_violations() {
    let packet = current_hotspot_workspace_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_hotspot_workspace_summary_matches_computed() {
    let packet = current_hotspot_workspace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn hotspot_workspace_mapping_quality_allows_navigation_when_expected() {
    use aureline_profiler::MappingQualityLabel;

    assert!(MappingQualityLabel::Exact.allows_source_navigation());
    assert!(MappingQualityLabel::Approximate.allows_source_navigation());
    assert!(MappingQualityLabel::Partial.allows_source_navigation());
    assert!(!MappingQualityLabel::Unavailable.allows_source_navigation());
    assert!(!MappingQualityLabel::Stale.allows_source_navigation());
    assert!(!MappingQualityLabel::Mismatched.allows_source_navigation());
}

#[test]
fn hotspot_workspace_stable_surfaces_have_complete_guards() {
    let packet = current_hotspot_workspace_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.session_strip_visible,
                "surface {} must show session strip",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
            assert!(
                surface.guards.source_navigation_visible,
                "surface {} must show source navigation",
                surface.surface_id
            );
            assert!(
                surface.guards.flamegraph_visible,
                "surface {} must show flamegraph",
                surface.surface_id
            );
            assert!(
                surface.guards.call_tree_visible,
                "surface {} must show call tree",
                surface.surface_id
            );
        }
    }
}

// --- Trace viewer packet (M05-047) ---

#[test]
fn embedded_trace_viewer_packet_parses() {
    let packet = current_trace_viewer_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.event_lanes.is_empty());
    assert!(!packet.bookmarks.is_empty());
    assert!(!packet.textual_fallbacks.is_empty());
}

#[test]
fn embedded_trace_viewer_packet_has_no_violations() {
    let packet = current_trace_viewer_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_trace_viewer_summary_matches_computed() {
    let packet = current_trace_viewer_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn trace_viewer_stable_surfaces_have_complete_guards() {
    let packet = current_trace_viewer_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.event_lanes_visible,
                "surface {} must show event lanes",
                surface.surface_id
            );
            assert!(
                surface.guards.bookmarks_visible,
                "surface {} must show bookmarks",
                surface.surface_id
            );
            assert!(
                surface.guards.textual_fallback_visible,
                "surface {} must show textual fallback",
                surface.surface_id
            );
            assert!(
                surface.guards.synchronization_visible,
                "surface {} must show synchronization",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
        }
    }
}

// --- Memory-analysis packet (M05-048) ---

#[test]
fn embedded_memory_analysis_packet_parses() {
    let packet = current_memory_analysis_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.views.is_empty());
    assert!(!packet.snapshot_pairs.is_empty());
    assert!(!packet.retained_diffs.is_empty());
    assert!(!packet.allocation_diffs.is_empty());
    assert!(!packet.leak_hints.is_empty());
}

#[test]
fn embedded_memory_analysis_packet_has_no_violations() {
    let packet = current_memory_analysis_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_memory_analysis_summary_matches_computed() {
    let packet = current_memory_analysis_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn memory_analysis_leak_hint_confidence_is_actionable_when_expected() {
    use aureline_profiler::LeakHintConfidence;

    assert!(LeakHintConfidence::High.is_actionable());
    assert!(LeakHintConfidence::Medium.is_actionable());
    assert!(!LeakHintConfidence::Low.is_actionable());
    assert!(!LeakHintConfidence::Uncertain.is_actionable());
}

#[test]
fn memory_analysis_stable_surfaces_have_complete_guards() {
    let packet = current_memory_analysis_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.views_visible,
                "surface {} must show views",
                surface.surface_id
            );
            assert!(
                surface.guards.snapshot_pairs_visible,
                "surface {} must show snapshot pairs",
                surface.surface_id
            );
            assert!(
                surface.guards.retained_diffs_visible,
                "surface {} must show retained diffs",
                surface.surface_id
            );
            assert!(
                surface.guards.allocation_diffs_visible,
                "surface {} must show allocation diffs",
                surface.surface_id
            );
            assert!(
                surface.guards.leak_hints_visible,
                "surface {} must show leak hints",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
        }
    }
}

// --- Regression baseline packet (M05-049) ---

#[test]
fn embedded_regression_baseline_packet_parses() {
    let packet = current_regression_baseline_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.baseline_stores.is_empty());
    assert!(!packet.baseline_selection_uxs.is_empty());
    assert!(!packet.comparable_environment_guards.is_empty());
    assert!(!packet.environment_fingerprints.is_empty());
}

#[test]
fn embedded_regression_baseline_packet_has_no_violations() {
    let packet = current_regression_baseline_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_regression_baseline_summary_matches_computed() {
    let packet = current_regression_baseline_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn regression_baseline_environment_match_state_warns_correctly() {
    use aureline_profiler::EnvironmentMatchState;

    assert!(EnvironmentMatchState::Comparable.allows_comparison_with_warning());
    assert!(EnvironmentMatchState::Partial.allows_comparison_with_warning());
    assert!(EnvironmentMatchState::Stale.allows_comparison_with_warning());
    assert!(!EnvironmentMatchState::Mismatch.allows_comparison_with_warning());
    assert!(!EnvironmentMatchState::Unknown.allows_comparison_with_warning());

    assert!(!EnvironmentMatchState::Comparable.shows_warning());
    assert!(EnvironmentMatchState::Partial.shows_warning());
    assert!(EnvironmentMatchState::Mismatch.shows_warning());
    assert!(!EnvironmentMatchState::Unknown.shows_warning());
    assert!(EnvironmentMatchState::Stale.shows_warning());
}

#[test]
fn regression_baseline_stable_surfaces_have_complete_guards() {
    let packet = current_regression_baseline_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.baseline_identity_visible,
                "surface {} must show baseline identity",
                surface.surface_id
            );
            assert!(
                surface.guards.build_identity_visible,
                "surface {} must show build identity",
                surface.surface_id
            );
            assert!(
                surface.guards.environment_fingerprint_visible,
                "surface {} must show environment fingerprint",
                surface.surface_id
            );
            assert!(
                surface.guards.capture_mode_visible,
                "surface {} must show capture mode",
                surface.surface_id
            );
            assert!(
                surface.guards.storage_location_visible,
                "surface {} must show storage location",
                surface.surface_id
            );
            assert!(
                surface.guards.freshness_state_visible,
                "surface {} must show freshness state",
                surface.surface_id
            );
            assert!(
                surface.guards.comparison_basis_visible,
                "surface {} must show comparison basis",
                surface.surface_id
            );
            assert!(
                surface.guards.environment_match_visible,
                "surface {} must show environment match",
                surface.surface_id
            );
            assert!(
                surface.guards.mismatch_warning_visible,
                "surface {} must show mismatch warning",
                surface.surface_id
            );
            assert!(
                surface.guards.guard_criteria_visible,
                "surface {} must show guard criteria",
                surface.surface_id
            );
        }
    }
}

// --- Profile-compare packet (M05-050) ---

#[test]
fn embedded_profile_compare_packet_parses() {
    let packet = current_profile_compare_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.compare_cards.is_empty());
    assert!(!packet.threshold_states.is_empty());
    assert!(!packet.waiver_states.is_empty());
    assert!(!packet.confounder_disclosures.is_empty());
}

#[test]
fn embedded_profile_compare_packet_has_no_violations() {
    let packet = current_profile_compare_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_profile_compare_summary_matches_computed() {
    let packet = current_profile_compare_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn profile_compare_threshold_state_alert_behavior_is_correct() {
    use aureline_profiler::ThresholdState;

    assert!(ThresholdState::Within.allows_comparison());
    assert!(ThresholdState::Warning.allows_comparison());
    assert!(!ThresholdState::Breach.allows_comparison());
    assert!(ThresholdState::Waived.allows_comparison());
    assert!(ThresholdState::Provisional.allows_comparison());

    assert!(!ThresholdState::Within.is_breach());
    assert!(!ThresholdState::Warning.is_breach());
    assert!(ThresholdState::Breach.is_breach());
    assert!(ThresholdState::Waived.is_breach());
    assert!(!ThresholdState::Provisional.is_breach());

    assert!(!ThresholdState::Within.shows_alert());
    assert!(ThresholdState::Warning.shows_alert());
    assert!(ThresholdState::Breach.shows_alert());
    assert!(ThresholdState::Waived.shows_alert());
    assert!(ThresholdState::Provisional.shows_alert());
}

#[test]
fn profile_compare_waiver_status_covering_behavior_is_correct() {
    use aureline_profiler::WaiverStatus;

    assert!(WaiverStatus::Active.is_covering());
    assert!(!WaiverStatus::Expired.is_covering());
    assert!(!WaiverStatus::Pending.is_covering());
    assert!(!WaiverStatus::Retired.is_covering());
}

#[test]
fn profile_compare_confounder_severity_blocks_stable_claim_correctly() {
    use aureline_profiler::ConfounderSeverity;

    assert!(ConfounderSeverity::Critical.blocks_stable_claim());
    assert!(ConfounderSeverity::Major.blocks_stable_claim());
    assert!(!ConfounderSeverity::Minor.blocks_stable_claim());
    assert!(!ConfounderSeverity::Info.blocks_stable_claim());
}

#[test]
fn profile_compare_stable_surfaces_have_complete_guards() {
    let packet = current_profile_compare_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.compare_card_visible,
                "surface {} must show compare card",
                surface.surface_id
            );
            assert!(
                surface.guards.threshold_inspector_visible,
                "surface {} must show threshold inspector",
                surface.surface_id
            );
            assert!(
                surface.guards.waiver_badge_visible,
                "surface {} must show waiver badge",
                surface.surface_id
            );
            assert!(
                surface.guards.confounder_disclosure_visible,
                "surface {} must show confounder disclosure",
                surface.surface_id
            );
            assert!(
                surface.guards.capture_identity_visible,
                "surface {} must show capture identity",
                surface.surface_id
            );
            assert!(
                surface.guards.comparison_basis_visible,
                "surface {} must show comparison basis",
                surface.surface_id
            );
            assert!(
                surface.guards.threshold_bar_visible,
                "surface {} must show threshold bar",
                surface.surface_id
            );
            assert!(
                surface.guards.waiver_expiry_visible,
                "surface {} must show waiver expiry",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
        }
    }
}
