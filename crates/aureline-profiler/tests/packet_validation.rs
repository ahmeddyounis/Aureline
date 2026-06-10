//! Integration test: the embedded qualification packets parse and validate.

use aureline_profiler::{
    current_chronology_qualification, current_evidence_handoff_qualification,
    current_hotspot_workspace_qualification, current_memory_analysis_qualification,
    current_profile_compare_qualification, current_profile_launcher_qualification,
    current_regression_baseline_qualification, current_replay_qualification,
    current_trace_viewer_qualification,
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

// --- Chronology qualification packet (M05-053) ---

#[test]
fn embedded_chronology_packet_parses() {
    let packet = current_chronology_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.chronology_controls.is_empty());
    assert!(!packet.reverse_step_actions.is_empty());
    assert!(!packet.history_partiality_cues.is_empty());
    assert!(!packet.import_export_packets.is_empty());
}

#[test]
fn embedded_chronology_packet_has_no_violations() {
    let packet = current_chronology_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_chronology_summary_matches_computed() {
    let packet = current_chronology_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn chronology_partiality_severity_blocks_stable_claim_correctly() {
    use aureline_profiler::PartialitySeverity;

    assert!(PartialitySeverity::Critical.blocks_stable_claim());
    assert!(!PartialitySeverity::Warning.blocks_stable_claim());
    assert!(!PartialitySeverity::Info.blocks_stable_claim());

    assert!(PartialitySeverity::Critical.shows_degraded_label());
    assert!(PartialitySeverity::Warning.shows_degraded_label());
    assert!(!PartialitySeverity::Info.shows_degraded_label());
}

#[test]
fn chronology_packet_direction_behavior_is_correct() {
    use aureline_profiler::PacketDirection;

    assert!(PacketDirection::Import.is_import());
    assert!(!PacketDirection::Import.is_export());
    assert!(PacketDirection::Export.is_export());
    assert!(!PacketDirection::Export.is_import());
}

#[test]
fn chronology_stable_surfaces_have_complete_guards() {
    let packet = current_chronology_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.chronology_controls_visible,
                "surface {} must show chronology controls",
                surface.surface_id
            );
            assert!(
                surface.guards.reverse_step_actions_visible,
                "surface {} must show reverse step actions",
                surface.surface_id
            );
            assert!(
                surface.guards.history_partiality_cues_visible,
                "surface {} must show history partiality cues",
                surface.surface_id
            );
            assert!(
                surface.guards.import_export_packets_visible,
                "surface {} must show import/export packets",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
            assert!(
                surface.guards.degraded_state_label_visible,
                "surface {} must show degraded state label",
                surface.surface_id
            );
            assert!(
                surface.guards.integrity_check_visible,
                "surface {} must show integrity check",
                surface.surface_id
            );
        }
    }
}

// --- Evidence handoff packet ---

// --- Replay qualification packet (M05-052) ---

#[test]
fn embedded_replay_packet_parses() {
    let packet = current_replay_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.recording_mode_banners.is_empty());
    assert!(!packet.replay_expiries.is_empty());
    assert!(!packet.replay_cost_postures.is_empty());
}

#[test]
fn embedded_replay_packet_has_no_violations() {
    let packet = current_replay_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_replay_summary_matches_computed() {
    let packet = current_replay_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn replay_recording_mode_state_behavior_is_correct() {
    use aureline_profiler::RecordingModeState;

    assert!(RecordingModeState::Recorded.allows_replay());
    assert!(!RecordingModeState::Recording.allows_replay());
    assert!(!RecordingModeState::NotRecording.allows_replay());
    assert!(!RecordingModeState::Expired.allows_replay());
    assert!(!RecordingModeState::Unsupported.allows_replay());
    assert!(!RecordingModeState::PolicyBlocked.allows_replay());

    assert!(!RecordingModeState::Recorded.shows_degraded_label());
    assert!(!RecordingModeState::Recording.shows_degraded_label());
    assert!(!RecordingModeState::NotRecording.shows_degraded_label());
    assert!(RecordingModeState::Expired.shows_degraded_label());
    assert!(RecordingModeState::Unsupported.shows_degraded_label());
    assert!(RecordingModeState::PolicyBlocked.shows_degraded_label());
}

#[test]
fn replay_expiry_status_behavior_is_correct() {
    use aureline_profiler::ExpiryStatus;

    assert!(ExpiryStatus::Current.is_replayable());
    assert!(ExpiryStatus::Stale.is_replayable());
    assert!(ExpiryStatus::Pinned.is_replayable());
    assert!(!ExpiryStatus::Expired.is_replayable());
    assert!(!ExpiryStatus::Missing.is_replayable());
    assert!(!ExpiryStatus::PolicyBlocked.is_replayable());

    assert!(!ExpiryStatus::Current.shows_degraded_label());
    assert!(ExpiryStatus::Stale.shows_degraded_label());
    assert!(!ExpiryStatus::Pinned.shows_degraded_label());
    assert!(ExpiryStatus::Expired.shows_degraded_label());
    assert!(ExpiryStatus::Missing.shows_degraded_label());
    assert!(ExpiryStatus::PolicyBlocked.shows_degraded_label());
}

#[test]
fn replay_cost_posture_class_behavior_is_correct() {
    use aureline_profiler::CostPostureClass;

    assert!(!CostPostureClass::Low.requires_warning());
    assert!(!CostPostureClass::Moderate.requires_warning());
    assert!(CostPostureClass::High.requires_warning());
    assert!(CostPostureClass::Extreme.requires_warning());
    assert!(!CostPostureClass::Unknown.requires_warning());

    assert!(!CostPostureClass::Low.blocks_auto_record());
    assert!(!CostPostureClass::Moderate.blocks_auto_record());
    assert!(!CostPostureClass::High.blocks_auto_record());
    assert!(CostPostureClass::Extreme.blocks_auto_record());
    assert!(!CostPostureClass::Unknown.blocks_auto_record());
}

#[test]
fn replay_stable_surfaces_have_complete_guards() {
    let packet = current_replay_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.recording_mode_banner_visible,
                "surface {} must show recording mode banner",
                surface.surface_id
            );
            assert!(
                surface.guards.replay_expiry_visible,
                "surface {} must show replay expiry",
                surface.surface_id
            );
            assert!(
                surface.guards.cost_posture_visible,
                "surface {} must show cost posture",
                surface.surface_id
            );
            assert!(
                surface.guards.degraded_state_label_visible,
                "surface {} must show degraded state label",
                surface.surface_id
            );
            assert!(
                surface.guards.retention_policy_visible,
                "surface {} must show retention policy",
                surface.surface_id
            );
            assert!(
                surface.guards.cost_warning_visible,
                "surface {} must show cost warning",
                surface.surface_id
            );
        }
    }
}

// --- Evidence handoff packet ---

#[test]
fn embedded_evidence_handoff_packet_parses() {
    let packet = current_evidence_handoff_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.handoff_bars.is_empty());
    assert!(!packet.artifact_lineages.is_empty());
    assert!(!packet.capture_sources.is_empty());
    assert!(!packet.save_share_scopes.is_empty());
}

#[test]
fn embedded_evidence_handoff_packet_has_no_violations() {
    let packet = current_evidence_handoff_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_evidence_handoff_summary_matches_computed() {
    let packet = current_evidence_handoff_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn evidence_handoff_lineage_state_behavior_is_correct() {
    use aureline_profiler::LineageState;

    assert!(LineageState::ExactMatch.allows_navigation());
    assert!(LineageState::ProbableMismatch.allows_navigation());
    assert!(LineageState::SourceOnly.allows_navigation());
    assert!(!LineageState::ArtifactOnly.allows_navigation());
    assert!(!LineageState::RestrictedByPolicy.allows_navigation());
    assert!(!LineageState::Unavailable.allows_navigation());

    assert!(!LineageState::ExactMatch.shows_degraded_label());
    assert!(LineageState::ProbableMismatch.shows_degraded_label());
    assert!(LineageState::SourceOnly.shows_degraded_label());
    assert!(LineageState::ArtifactOnly.shows_degraded_label());
    assert!(LineageState::RestrictedByPolicy.shows_degraded_label());
    assert!(LineageState::Unavailable.shows_degraded_label());
}

#[test]
fn evidence_handoff_stable_surfaces_have_complete_guards() {
    let packet = current_evidence_handoff_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.origin_visible,
                "surface {} must show origin",
                surface.surface_id
            );
            assert!(
                surface.guards.build_id_visible,
                "surface {} must show build ID",
                surface.surface_id
            );
            assert!(
                surface.guards.commit_visible,
                "surface {} must show commit",
                surface.surface_id
            );
            assert!(
                surface.guards.capture_source_visible,
                "surface {} must show capture source",
                surface.surface_id
            );
            assert!(
                surface.guards.save_share_scope_visible,
                "surface {} must show save/share scope",
                surface.surface_id
            );
            assert!(
                surface.guards.lineage_state_visible,
                "surface {} must show lineage state",
                surface.surface_id
            );
            assert!(
                surface.guards.lineage_detail_visible,
                "surface {} must show lineage detail",
                surface.surface_id
            );
        }
    }
}

// --- Profile-compare packet (M05-050) ---

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
