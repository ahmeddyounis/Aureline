//! Integration test: the embedded qualification packets parse and validate.

use aureline_profiler::{
    current_hotspot_workspace_qualification, current_memory_analysis_qualification,
    current_profile_launcher_qualification, current_trace_viewer_qualification,
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
