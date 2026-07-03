//! Integration test: the embedded qualification packets parse and validate.

use aureline_profiler::{
    current_certification_qualification, current_chronology_qualification,
    current_component_certification_packet, current_component_fallback_packet,
    current_evidence_handoff_qualification, current_hotspot_workspace_qualification,
    current_integrate_profile_trace_qualification, current_memory_analysis_qualification,
    current_profile_compare_qualification, current_profile_hotpath_component_packet,
    current_profile_launcher_qualification, current_regression_baseline_qualification,
    current_replay_qualification, current_trace_heap_compare_component_packet,
    current_trace_viewer_qualification,
};

// --- Certification packet (M05-055) ---

#[test]
fn embedded_certification_packet_parses() {
    let packet = current_certification_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.certifications.is_empty());
    assert!(!packet.imported_versus_live_truths.is_empty());
    assert!(!packet.downgrade_rules.is_empty());
}

#[test]
fn embedded_certification_packet_has_no_violations() {
    let packet = current_certification_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_certification_summary_matches_computed() {
    let packet = current_certification_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn certification_status_behavior_is_correct() {
    use aureline_profiler::CertificationStatus;

    assert!(CertificationStatus::Certified.allows_promotion());
    assert!(!CertificationStatus::Pending.allows_promotion());
    assert!(!CertificationStatus::Stale.allows_promotion());
    assert!(!CertificationStatus::Underqualified.allows_promotion());
    assert!(!CertificationStatus::PolicyBlocked.allows_promotion());
    assert!(!CertificationStatus::RolledBack.allows_promotion());

    assert!(!CertificationStatus::Certified.triggers_downgrade());
    assert!(!CertificationStatus::Pending.triggers_downgrade());
    assert!(CertificationStatus::Stale.triggers_downgrade());
    assert!(CertificationStatus::Underqualified.triggers_downgrade());
    assert!(!CertificationStatus::PolicyBlocked.triggers_downgrade());
    assert!(CertificationStatus::RolledBack.triggers_downgrade());

    assert!(!CertificationStatus::Certified.shows_degraded_label());
    assert!(!CertificationStatus::Pending.shows_degraded_label());
    assert!(CertificationStatus::Stale.shows_degraded_label());
    assert!(CertificationStatus::Underqualified.shows_degraded_label());
    assert!(CertificationStatus::PolicyBlocked.shows_degraded_label());
    assert!(CertificationStatus::RolledBack.shows_degraded_label());
}

#[test]
fn origin_class_behavior_is_correct() {
    use aureline_profiler::OriginClass;

    assert!(OriginClass::LiveCapture.is_live());
    assert!(!OriginClass::ImportedArtifact.is_live());
    assert!(!OriginClass::CachedReplay.is_live());
    assert!(!OriginClass::SupportBundle.is_live());
    assert!(!OriginClass::Unknown.is_live());

    assert!(!OriginClass::LiveCapture.is_imported_or_cached());
    assert!(OriginClass::ImportedArtifact.is_imported_or_cached());
    assert!(OriginClass::CachedReplay.is_imported_or_cached());
    assert!(!OriginClass::SupportBundle.is_imported_or_cached());
    assert!(!OriginClass::Unknown.is_imported_or_cached());

    assert!(!OriginClass::LiveCapture.requires_provenance());
    assert!(OriginClass::ImportedArtifact.requires_provenance());
    assert!(OriginClass::CachedReplay.requires_provenance());
    assert!(OriginClass::SupportBundle.requires_provenance());
    assert!(!OriginClass::Unknown.requires_provenance());
}

#[test]
fn mapping_fidelity_behavior_is_correct() {
    use aureline_profiler::MappingFidelity;

    assert!(MappingFidelity::Exact.allows_source_navigation());
    assert!(MappingFidelity::Approximate.allows_source_navigation());
    assert!(MappingFidelity::Partial.allows_source_navigation());
    assert!(!MappingFidelity::Unavailable.allows_source_navigation());
    assert!(!MappingFidelity::Stale.allows_source_navigation());
    assert!(!MappingFidelity::Mismatched.allows_source_navigation());

    assert!(!MappingFidelity::Exact.blocks_stable_comparison());
    assert!(!MappingFidelity::Approximate.blocks_stable_comparison());
    assert!(!MappingFidelity::Partial.blocks_stable_comparison());
    assert!(MappingFidelity::Unavailable.blocks_stable_comparison());
    assert!(MappingFidelity::Stale.blocks_stable_comparison());
    assert!(MappingFidelity::Mismatched.blocks_stable_comparison());
}

#[test]
fn baseline_comparability_behavior_is_correct() {
    use aureline_profiler::BaselineComparability;

    assert!(BaselineComparability::Comparable.allows_comparison_with_warning());
    assert!(BaselineComparability::Partial.allows_comparison_with_warning());
    assert!(BaselineComparability::Stale.allows_comparison_with_warning());
    assert!(!BaselineComparability::Mismatch.allows_comparison_with_warning());
    assert!(!BaselineComparability::Unknown.allows_comparison_with_warning());

    assert!(!BaselineComparability::Comparable.shows_warning());
    assert!(BaselineComparability::Partial.shows_warning());
    assert!(BaselineComparability::Stale.shows_warning());
    assert!(BaselineComparability::Mismatch.shows_warning());
    assert!(!BaselineComparability::Unknown.shows_warning());
}

#[test]
fn certification_stable_surfaces_have_complete_guards() {
    let packet = current_certification_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.certification_status_visible,
                "surface {} must show certification status",
                surface.surface_id
            );
            assert!(
                surface.guards.imported_versus_live_visible,
                "surface {} must show imported versus live",
                surface.surface_id
            );
            assert!(
                surface.guards.provenance_chain_visible,
                "surface {} must show provenance chain",
                surface.surface_id
            );
            assert!(
                surface.guards.build_identity_visible,
                "surface {} must show build identity",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_fidelity_visible,
                "surface {} must show mapping fidelity",
                surface.surface_id
            );
            assert!(
                surface.guards.comparison_basis_visible,
                "surface {} must show comparison basis",
                surface.surface_id
            );
            assert!(
                surface.guards.downgrade_rules_visible,
                "surface {} must show downgrade rules",
                surface.surface_id
            );
            assert!(
                surface.guards.stale_warning_visible,
                "surface {} must show stale warning",
                surface.surface_id
            );
        }
    }
}

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

// --- Profile session / flamegraph / icicle / call-tree components (M05-797) ---

#[test]
fn embedded_profile_hotpath_component_packet_parses() {
    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.profile_session_cards.is_empty());
    assert!(!packet.profile_cost_views.is_empty());
    assert!(!packet.call_tree_rows.is_empty());
    assert!(!packet.consumer_projection_rows.is_empty());
}

#[test]
fn embedded_profile_hotpath_component_packet_has_no_violations() {
    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_profile_hotpath_component_summary_matches_computed() {
    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.hotspot_consumer_present);
    assert!(packet.summary.secondary_consumer_present);
    assert!(packet.summary.all_components_preserve_mapping_quality);
    assert!(packet.summary.all_components_have_copy_export);
}

#[test]
fn profile_session_cards_disclose_capture_identity_and_actions() {
    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    for card in &packet.profile_session_cards {
        assert!(!card.session_ref.is_empty());
        assert!(!card.build_identity_ref.is_empty());
        assert!(!card.runtime_identity_ref.is_empty());
        assert!(!card.target.process_ref.is_empty());
        assert!(!card.target.config_ref.is_empty());
        assert!(!card.target.raw_command_line_exported);
        assert!(card.duration_ms > 0);
        assert!(card.actions.compare_available);
        assert!(card.actions.export_available);
        assert!(card
            .consumer_surfaces
            .contains(&aureline_profiler::ComponentConsumerSurface::HotspotWorkspace));
    }
}

#[test]
fn flamegraph_and_icicle_views_disclose_filters_zoom_and_raw_export() {
    use aureline_profiler::ProfileCostViewMode;

    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    assert!(packet
        .profile_cost_views
        .iter()
        .any(|view| view.view_mode == ProfileCostViewMode::Flamegraph));
    assert!(packet
        .profile_cost_views
        .iter()
        .any(|view| view.view_mode == ProfileCostViewMode::Icicle));

    for view in &packet.profile_cost_views {
        assert!(view.total_samples > 0);
        assert!(view.total_time_ms > 0);
        assert!(!view.thread_process_context.thread_refs.is_empty());
        assert!(!view.thread_process_filters.thread_filter_refs.is_empty());
        assert!(view.zoom_state.depth_limit > 0);
        assert!(view.actions.export_available);
        assert!(view.actions.open_raw_available);
        assert!(view.call_tree_available);
    }
}

#[test]
fn call_tree_rows_disclose_symbolization_mapping_and_navigation() {
    let packet = current_profile_hotpath_component_packet().expect("embedded packet must parse");
    for row in &packet.call_tree_rows {
        assert!(!row.function_name.is_empty());
        assert!(row.inclusive_metric.value >= row.self_metric.value);
        assert!(!row.file_ref.is_empty());
        assert!(!row.module_ref.is_empty());
        assert!(!row.service_ref.is_empty());
        assert!(!row.thread_ref.is_empty());
        assert!(!row.caller_refs.is_empty());
        assert!(!row.callee_refs.is_empty());
        assert!(row.navigation.caller_navigation_available);
        assert!(row.navigation.callee_navigation_available);
        assert!(row.navigation.source_navigation.available);
    }
}

// --- Trace-timeline / heap-allocation / profile-compare components (M05-798) ---

#[test]
fn embedded_trace_heap_compare_component_packet_parses() {
    let packet = current_trace_heap_compare_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.trace_timelines.is_empty());
    assert!(!packet.heap_compare_cards.is_empty());
    assert!(!packet.consumer_projection_rows.is_empty());
}

#[test]
fn embedded_trace_heap_compare_component_packet_has_no_violations() {
    let packet = current_trace_heap_compare_component_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_trace_heap_compare_component_summary_matches_computed() {
    let packet = current_trace_heap_compare_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.trace_viewer_consumer_present);
    assert!(packet.summary.profile_compare_consumer_present);
    assert!(packet.summary.imported_and_live_both_present);
    assert!(packet.summary.all_components_preserve_mapping_quality);
    assert!(packet.summary.all_components_have_copy_export);
    assert!(packet.summary.all_compare_cards_disclose_baseline);
}

#[test]
fn trace_timelines_keep_imported_versus_live_and_clock_basis_visible() {
    use aureline_profiler::ArtifactOrigin;

    let packet = current_trace_heap_compare_component_packet().expect("embedded packet must parse");
    // Both a live and a non-live timeline are present and stay distinct.
    assert!(packet
        .trace_timelines
        .iter()
        .any(|t| t.artifact_origin == ArtifactOrigin::LiveCapture));
    assert!(packet
        .trace_timelines
        .iter()
        .any(|t| t.artifact_origin != ArtifactOrigin::LiveCapture));

    for timeline in &packet.trace_timelines {
        assert!(!timeline.lane_summary.process_refs.is_empty());
        assert!(!timeline.lane_summary.thread_refs.is_empty());
        assert!(!timeline.textual_fallback_ref.is_empty());
        assert!(timeline
            .copy_export
            .export_fields
            .iter()
            .any(|f| f == "artifact_origin"));
        assert!(timeline
            .copy_export
            .export_fields
            .iter()
            .any(|f| f == "clock_sync_basis"));
    }
}

#[test]
fn heap_compare_cards_foreground_baseline_before_regression() {
    use aureline_profiler::UiThresholdState;

    let packet = current_trace_heap_compare_component_packet().expect("embedded packet must parse");
    for card in &packet.heap_compare_cards {
        // Every card foregrounds baseline identity and confounder notes.
        assert!(card.baseline_disclosed());
        // Any card that claims a regression is backed by a comparable baseline
        // with visible environment deltas.
        if card.claims_regression() {
            assert!(card.regression_claim_supported());
        }
        // Imported artifacts may never claim a regression and must be narrowed.
        if card.is_imported_artifact() {
            assert_ne!(card.baseline.threshold_state, UiThresholdState::Regression);
            assert_ne!(card.reduced_capability_banner.capability_state, "full");
        }
        // The compare/confounder vocabulary survives the export.
        assert!(card
            .copy_export
            .export_fields
            .iter()
            .any(|f| f.contains("confounder")));
        assert!(card
            .copy_export
            .export_fields
            .iter()
            .any(|f| f.contains("threshold_state")));
    }
}

#[test]
fn clock_sync_basis_degraded_behavior_is_correct() {
    use aureline_profiler::ClockSyncBasis;

    assert!(!ClockSyncBasis::MonotonicSingleProcess.is_degraded());
    assert!(!ClockSyncBasis::SynchronizedMultiProcess.is_degraded());
    assert!(ClockSyncBasis::ImportedClockDomain.is_degraded());
    assert!(ClockSyncBasis::PartialClockCorrelation.is_degraded());
    assert!(ClockSyncBasis::Unknown.is_degraded());
}

// --- Component accessibility fallback / export-safe summaries (M05-801) ---

#[test]
fn embedded_component_fallback_packet_parses() {
    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.rows.len(), 10);
}

#[test]
fn embedded_component_fallback_packet_has_no_violations() {
    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_component_fallback_summary_matches_computed() {
    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.all_canvas_heavy_have_non_visual_fallback);
    assert!(packet.summary.all_keyboard_and_screen_reader_reachable);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_narrowing_disclosed);
    assert_eq!(packet.summary.red_count, 0);
}

#[test]
fn component_fallback_covers_every_frozen_family() {
    use aureline_profiler::M5ComponentFamily;

    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    for family in M5ComponentFamily::ALL {
        assert!(
            packet.rows.iter().any(|row| row.component_family == family),
            "family {family:?} must be certified"
        );
    }
}

#[test]
fn canvas_heavy_families_bind_a_non_visual_fallback_path() {
    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        if row.component_family.is_canvas_heavy() {
            // Canvas-heavy consumers render a canvas but also a non-visual path,
            // so keyboard/assistive-tech users are never stranded.
            assert!(
                row.has_non_visual_fallback(),
                "canvas-heavy family {:?} must offer a list/table/textual path",
                row.component_family
            );
            assert!(row.keyboard_reach.never_traps());
            assert!(row.screen_reader_reach.never_traps());
            assert!(row.keyboard_and_at_reachable());
        }
    }
}

#[test]
fn every_component_export_preserves_meaning_without_screenshot() {
    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        assert!(row.export_preserves_meaning());
        assert!(!row.export_summary_ref.is_empty());
        assert!(row.copy_export.screenshot_only_prohibited);
    }
}

#[test]
fn narrower_rendering_surfaces_disclose_reduced_interactivity() {
    use aureline_profiler::M5RenderingSurface;

    let packet = current_component_fallback_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        assert!(row.narrowing_disclosed());
        for surface in &row.rendering_surfaces {
            if surface.is_narrowed() {
                let disclosure = row
                    .narrowing_disclosures
                    .iter()
                    .find(|d| d.rendering_surface == *surface)
                    .unwrap_or_else(|| {
                        panic!("narrowed surface {surface:?} must disclose reduced interactivity")
                    });
                assert!(disclosure.state.never_drops_silently());
                assert!(
                    !disclosure.preserved_labels.is_empty(),
                    "narrowed surface {surface:?} must preserve labels"
                );
            }
        }
        // Every row still renders at desktop-full capability.
        assert!(row
            .rendering_surfaces
            .contains(&M5RenderingSurface::DesktopFull));
    }
}

#[test]
fn component_fallback_status_reason_states_behave_correctly() {
    use aureline_profiler::{
        ExportSummaryState, NarrowingDisclosureState, NonVisualReachState, ZoomDensityState,
    };

    assert!(NonVisualReachState::ReachableAndLabeled.never_traps());
    assert!(NonVisualReachState::DisclosedReducedButReachable.never_traps());
    assert!(!NonVisualReachState::ViewOnlyTrap.never_traps());

    assert!(ZoomDensityState::LegibleUnderZoomAndDensity.never_loses_truth());
    assert!(ZoomDensityState::DisclosedReducedLegibility.never_loses_truth());
    assert!(!ZoomDensityState::TruncatedOrLostOnZoomOrDensity.never_loses_truth());

    assert!(ExportSummaryState::ReconstructableWithoutScreenshot.never_screenshot_only());
    assert!(ExportSummaryState::DisclosedPartialCapture.never_screenshot_only());
    assert!(!ExportSummaryState::AbsentNeedsScreenshot.never_screenshot_only());

    assert!(NarrowingDisclosureState::ParityPreserved.never_drops_silently());
    assert!(NarrowingDisclosureState::DisclosedNarrowed.never_drops_silently());
    assert!(!NarrowingDisclosureState::SilentlyDropped.never_drops_silently());
}

#[test]
fn component_fallback_status_flags_stranded_rows() {
    use aureline_profiler::{ComponentFallbackStatus, NonVisualReachState};

    let mut packet = current_component_fallback_packet().expect("embedded packet must parse");
    // Trap the first row's keyboard path: it must become stranded (red) and the
    // packet must reject it.
    packet.rows[0].keyboard_reach = NonVisualReachState::ViewOnlyTrap;
    assert_eq!(packet.rows[0].status(), ComponentFallbackStatus::Stranded);
    let violations = packet.validate();
    assert!(!violations.is_empty());
}

// --- Profiler/topology component certification (M05-803) ---

#[test]
fn embedded_component_certification_packet_parses() {
    let packet = current_component_certification_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert_eq!(packet.rows.len(), 17);
}

#[test]
fn embedded_component_certification_packet_has_no_violations() {
    let packet = current_component_certification_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_component_certification_summary_matches_computed() {
    let packet = current_component_certification_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.family_coverage_complete);
    assert!(packet.summary.all_surfaces_certify_or_narrow);
    assert!(packet.summary.all_reference_one_bundle);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.green_count, 12);
    assert_eq!(packet.summary.yellow_count, 5);
}

#[test]
fn component_certification_covers_every_claimed_surface() {
    use aureline_profiler::M5ClaimedSurface;

    let packet = current_component_certification_packet().expect("embedded packet must parse");
    for surface in M5ClaimedSurface::ALL {
        assert!(
            packet.rows.iter().any(|row| row.surface == surface),
            "claimed surface {surface:?} must be certified"
        );
    }
}

#[test]
fn component_certification_spans_every_frozen_family() {
    use aureline_profiler::M5ComponentFamily;

    let packet = current_component_certification_packet().expect("embedded packet must parse");
    for family in M5ComponentFamily::ALL {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.consumed_families.contains(&family)),
            "family {family:?} must be consumed by a certified surface"
        );
    }
}

#[test]
fn component_certification_axes_match_consumed_families() {
    let packet = current_component_certification_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        assert!(
            row.axes_match_consumed_families(),
            "row {} truth axes must apply exactly to its consumed families",
            row.row_id
        );
    }
}

#[test]
fn every_claimed_surface_certifies_or_narrows() {
    use aureline_profiler::SurfaceCertificationStatus;

    let packet = current_component_certification_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        assert!(row.certifies_or_narrows());
        assert!(row.claim_narrowing_is_coherent());
        assert!(row.discloses_narrowing());
        assert_ne!(row.status(), SurfaceCertificationStatus::Blocked);
    }
}

#[test]
fn every_certified_surface_cites_one_bundle() {
    let packet = current_component_certification_packet().expect("embedded packet must parse");
    for row in &packet.rows {
        assert!(row.references_one_bundle(&packet.certification_bundle_ref));
        assert!(row.excludes_raw_material());
    }
}

#[test]
fn component_certification_truth_states_behave_correctly() {
    use aureline_profiler::{
        CaptureExecutionTruthState, ClaimExportParityState, CompareBaselineTruthState,
        GraphProvenanceTruthState, WorksetScopeTruthState,
    };

    assert!(CaptureExecutionTruthState::IdentityCertified.never_violates());
    assert!(CaptureExecutionTruthState::DisclosedReducedIdentity.is_disclosed_reduction());
    assert!(!CaptureExecutionTruthState::IdentityHiddenOrImportedAsLive.never_violates());
    assert!(CaptureExecutionTruthState::NotApplicable.is_not_applicable());

    assert!(CompareBaselineTruthState::DisclosedDeferredComparison.is_disclosed_reduction());
    assert!(!CompareBaselineTruthState::RegressionClaimedBeforeBaselineTruth.never_violates());

    assert!(WorksetScopeTruthState::DisclosedNarrowedScope.is_disclosed_reduction());
    assert!(!WorksetScopeTruthState::SilentWideningOrHiddenScope.never_violates());

    assert!(
        GraphProvenanceTruthState::DisclosedNarrowedToAvailableEvidence.is_disclosed_reduction()
    );
    assert!(!GraphProvenanceTruthState::PartialGraphPresentedAsFullTruth.never_violates());

    assert!(ClaimExportParityState::DisclosedPartialExport.is_disclosed_reduction());
    assert!(!ClaimExportParityState::LabelsDroppedOrScreenshotOnly.never_violates());
}

#[test]
fn component_certification_flags_truth_hiding_surfaces() {
    use aureline_profiler::{GraphProvenanceTruthState, SurfaceCertificationStatus};

    let mut packet = current_component_certification_packet().expect("embedded packet must parse");
    // Force the topology map to present partial graph state as full truth: it
    // must become blocked (red) and the packet must reject it.
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == "cert-topology-map")
        .expect("topology map row must exist");
    row.graph_provenance_truth = GraphProvenanceTruthState::PartialGraphPresentedAsFullTruth;
    assert_eq!(row.status(), SurfaceCertificationStatus::Blocked);
    let violations = packet.validate();
    assert!(!violations.is_empty());
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

// --- Integration packet (M05-054) ---

#[test]
fn embedded_integrate_profile_trace_packet_parses() {
    let packet =
        current_integrate_profile_trace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.surfaces.is_empty());
    assert!(!packet.incident_workspace_attachments.is_empty());
    assert!(!packet.ai_explanations.is_empty());
    assert!(!packet.support_bundle_inclusions.is_empty());
}

#[test]
fn embedded_integrate_profile_trace_packet_has_no_violations() {
    let packet =
        current_integrate_profile_trace_qualification().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_integrate_profile_trace_summary_matches_computed() {
    let packet =
        current_integrate_profile_trace_qualification().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn explanation_confidence_trustworthiness_is_correct() {
    use aureline_profiler::ExplanationConfidence;

    assert!(ExplanationConfidence::High.is_trustworthy());
    assert!(ExplanationConfidence::Medium.is_trustworthy());
    assert!(!ExplanationConfidence::Low.is_trustworthy());
    assert!(!ExplanationConfidence::Uncertain.is_trustworthy());
}

#[test]
fn integrate_profile_trace_stable_surfaces_have_complete_guards() {
    let packet =
        current_integrate_profile_trace_qualification().expect("embedded packet must parse");
    for surface in &packet.surfaces {
        if surface.claim_label.is_stable() && surface.promoted_build_surface {
            assert!(
                surface.guards.artifact_origin_visible,
                "surface {} must show artifact origin",
                surface.surface_id
            );
            assert!(
                surface.guards.build_identity_visible,
                "surface {} must show build identity",
                surface.surface_id
            );
            assert!(
                surface.guards.mapping_quality_visible,
                "surface {} must show mapping quality",
                surface.surface_id
            );
            assert!(
                surface.guards.comparison_basis_visible,
                "surface {} must show comparison basis",
                surface.surface_id
            );
            assert!(
                surface.guards.export_posture_visible,
                "surface {} must show export posture",
                surface.surface_id
            );
            assert!(
                surface.guards.incident_workspace_link_visible,
                "surface {} must show incident workspace link",
                surface.surface_id
            );
            assert!(
                surface.guards.ai_explanation_visible,
                "surface {} must show AI explanation",
                surface.surface_id
            );
            assert!(
                surface.guards.support_bundle_link_visible,
                "surface {} must show support bundle link",
                surface.surface_id
            );
            assert!(
                surface.guards.degraded_state_label_visible,
                "surface {} must show degraded state label",
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
