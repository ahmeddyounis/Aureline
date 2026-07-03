//! Tests for the M5 breakpoint / device-preview row primitive: the resolver, the
//! parity matrix, and the checked-in support export.

use super::*;

fn live_source_anchored_input() -> M5BreakpointPreviewInput {
    M5BreakpointPreviewInput {
        target_id: "target:test:0001".to_owned(),
        node_label: "TestNode".to_owned(),
        viewport_label: "Desktop — 1440×900".to_owned(),
        active_breakpoint_token: "lg".to_owned(),
        theme_variant_token: "light".to_owned(),
        state_variant_token: "default".to_owned(),
        device_class: M5DevicePreviewClass::DesktopViewport,
        data_posture: M5PreviewDataPosture::Live,
        runtime_origin: M5PreviewRuntimeOrigin::LiveDevRuntime,
        freshness: PreviewFreshnessClass::Fresh,
        mapping_quality: M5BreakpointMappingQuality::Exact,
        sync_class: SourceSyncClass::InSyncFromSource,
        source_span_ref: Some("span:test:0001".to_owned()),
    }
}

// --- resolver: baseline live, source-anchored ---

#[test]
fn resolver_live_source_anchored_baseline_has_no_degrade() {
    let resolved = resolve_breakpoint_preview(&live_source_anchored_input()).expect("resolves");
    assert!(resolved.runtime_truth.is_live_data);
    assert!(!resolved.runtime_truth.is_stale);
    assert!(resolved.continuity.source_anchored);
    assert!(resolved.continuity.open_source_action_available);
    assert!(resolved
        .continuity
        .actions
        .contains(&M5BreakpointContinuityAction::CompareAcrossTargets));
    assert!(resolved
        .continuity
        .actions
        .contains(&M5BreakpointContinuityAction::OpenSourceForBreakpoint));
    assert!(resolved.discloses_runtime_truth());
    assert!(resolved.switching_stays_source_anchored());
    assert!(!resolved.has_runtime_truth_degrade());
    assert!(resolved.downgrade_trigger.is_none());
    assert!(resolved.identity_consistent());
}

// --- resolver: AC1 runtime truth always disclosed ---

#[test]
fn resolver_mock_data_discloses_non_live() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        data_posture: M5PreviewDataPosture::Mock,
        runtime_origin: M5PreviewRuntimeOrigin::LocalMockRuntime,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(!resolved.runtime_truth.is_live_data);
    assert!(resolved.is_showing_non_live_or_stale());
    assert!(resolved.discloses_runtime_truth());
    assert!(resolved
        .device_row
        .live_vs_mock_label
        .to_lowercase()
        .contains("mock"));
    // Mock, but still source-anchored and fresh: not a degrade.
    assert!(!resolved.has_runtime_truth_degrade());
}

#[test]
fn resolver_captured_snapshot_discloses_captured() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        data_posture: M5PreviewDataPosture::Captured,
        runtime_origin: M5PreviewRuntimeOrigin::CapturedSnapshot,
        mapping_quality: M5BreakpointMappingQuality::Unmapped,
        source_span_ref: None,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(!resolved.runtime_truth.is_live_data);
    assert!(resolved.is_showing_non_live_or_stale());
    assert!(resolved.discloses_runtime_truth());
    assert!(resolved
        .runtime_truth
        .truth_label
        .to_lowercase()
        .contains("captured"));
}

#[test]
fn resolver_stale_freshness_marks_stale() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        freshness: PreviewFreshnessClass::Stale,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(resolved.runtime_truth.is_stale);
    assert!(resolved.is_showing_non_live_or_stale());
    assert!(resolved.has_runtime_truth_degrade());
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::DriftedFromSource)
    );
    assert!(resolved.degrade_is_explained());
}

// --- resolver: AC2 source-anchored switching ---

#[test]
fn resolver_preserves_selection_across_targets() {
    let resolved = resolve_breakpoint_preview(&live_source_anchored_input()).expect("resolves");
    assert!(resolved.continuity.preserves_selection_context);
    assert_eq!(resolved.device_row.target_id, resolved.target_id);
    assert_eq!(resolved.runtime_truth.target_id, resolved.target_id);
    assert_eq!(resolved.continuity.target_id, resolved.target_id);
    assert!(resolved.switching_stays_source_anchored());
}

// --- resolver: AC3 degrade explained with shared trigger ---

#[test]
fn resolver_runtime_only_is_unavailable_and_reattaches() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        mapping_quality: M5BreakpointMappingQuality::Unmapped,
        sync_class: SourceSyncClass::RuntimeOnlyNoSource,
        source_span_ref: None,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(!resolved.continuity.source_anchored);
    assert!(!resolved.continuity.open_source_action_available);
    assert!(resolved
        .continuity
        .actions
        .contains(&M5BreakpointContinuityAction::ReattachRuntime));
    assert!(resolved.has_runtime_truth_degrade());
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::RuntimeUnavailable)
    );
    assert!(resolved.degrade_is_explained());
}

#[test]
fn resolver_unmapped_preview_degrades_with_unmapped_trigger() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        mapping_quality: M5BreakpointMappingQuality::Unmapped,
        data_posture: M5PreviewDataPosture::Captured,
        runtime_origin: M5PreviewRuntimeOrigin::CapturedSnapshot,
        freshness: PreviewFreshnessClass::Aging,
        source_span_ref: None,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(!resolved.continuity.source_anchored);
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::UnmappedSource)
    );
}

#[test]
fn resolver_unknown_freshness_is_unidentified() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        freshness: PreviewFreshnessClass::Unknown,
        ..live_source_anchored_input()
    })
    .expect("resolves");
    assert!(resolved.runtime_truth.is_stale);
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::UnidentifiedPosture)
    );
}

// --- resolver: structural guards ---

#[test]
fn resolver_rejects_contradictory_runtime_origin() {
    // A captured snapshot can never claim live data.
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        runtime_origin: M5PreviewRuntimeOrigin::CapturedSnapshot,
        data_posture: M5PreviewDataPosture::Live,
        ..live_source_anchored_input()
    });
    assert_eq!(
        resolved,
        Err(M5BreakpointPreviewResolutionError::ContradictoryRuntimeOrigin)
    );
    // A live dev runtime can never claim a captured snapshot posture.
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        runtime_origin: M5PreviewRuntimeOrigin::LiveDevRuntime,
        data_posture: M5PreviewDataPosture::Captured,
        ..live_source_anchored_input()
    });
    assert_eq!(
        resolved,
        Err(M5BreakpointPreviewResolutionError::ContradictoryRuntimeOrigin)
    );
}

#[test]
fn resolver_rejects_source_mapping_without_span() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        source_span_ref: None,
        ..live_source_anchored_input()
    });
    assert_eq!(
        resolved,
        Err(M5BreakpointPreviewResolutionError::MissingSpanForSourceMapping)
    );
}

#[test]
fn resolver_rejects_unmapped_with_span() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        mapping_quality: M5BreakpointMappingQuality::Unmapped,
        source_span_ref: Some("span:should-not-exist".to_owned()),
        ..live_source_anchored_input()
    });
    assert_eq!(
        resolved,
        Err(M5BreakpointPreviewResolutionError::ContradictoryUnmappedSpan)
    );
}

#[test]
fn resolver_rejects_runtime_only_with_span() {
    let resolved = resolve_breakpoint_preview(&M5BreakpointPreviewInput {
        sync_class: SourceSyncClass::RuntimeOnlyNoSource,
        mapping_quality: M5BreakpointMappingQuality::Exact,
        source_span_ref: Some("span:runtime:should-not-exist".to_owned()),
        ..live_source_anchored_input()
    });
    assert_eq!(
        resolved,
        Err(M5BreakpointPreviewResolutionError::ContradictoryRuntimeSpan)
    );
}

#[test]
fn resolver_rejects_empty_identity() {
    assert_eq!(
        resolve_breakpoint_preview(&M5BreakpointPreviewInput {
            target_id: "  ".to_owned(),
            ..live_source_anchored_input()
        }),
        Err(M5BreakpointPreviewResolutionError::EmptyTargetId)
    );
    assert_eq!(
        resolve_breakpoint_preview(&M5BreakpointPreviewInput {
            node_label: "".to_owned(),
            ..live_source_anchored_input()
        }),
        Err(M5BreakpointPreviewResolutionError::EmptyNodeLabel)
    );
    assert_eq!(
        resolve_breakpoint_preview(&M5BreakpointPreviewInput {
            viewport_label: "".to_owned(),
            ..live_source_anchored_input()
        }),
        Err(M5BreakpointPreviewResolutionError::EmptyViewportLabel)
    );
    assert_eq!(
        resolve_breakpoint_preview(&M5BreakpointPreviewInput {
            active_breakpoint_token: "".to_owned(),
            ..live_source_anchored_input()
        }),
        Err(M5BreakpointPreviewResolutionError::EmptyVariantToken)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    assert_eq!(
        resolve_breakpoint_preview(&M5BreakpointPreviewInput {
            viewport_label: "https://example.com/leak".to_owned(),
            ..live_source_anchored_input()
        }),
        Err(M5BreakpointPreviewResolutionError::ForbiddenMaterial)
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_breakpoint_preview_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_breakpoint_preview_packet();
    let present: BTreeSet<M5VisualDesignSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|r| r.surface_family)
        .collect();
    for required in M5VisualDesignSurfaceFamily::ALL {
        assert!(present.contains(&required), "missing {required:?}");
    }
}

#[test]
fn seeded_example_previews_are_self_consistent() {
    let packet = seeded_m5_breakpoint_preview_packet();
    for row in &packet.surface_rows {
        for case in &row.example_previews {
            assert!(
                case.is_self_consistent(),
                "case drifted on {:?}",
                row.surface_family
            );
        }
    }
}

#[test]
fn vocabulary_set_matches_canonical() {
    assert!(M5BreakpointVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_breakpoint_preview_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn runtime_origins_and_actions_have_distinct_tokens_and_labels() {
    let origins: BTreeSet<&str> = M5PreviewRuntimeOrigin::ALL
        .iter()
        .map(|o| o.as_str())
        .collect();
    assert_eq!(origins.len(), M5PreviewRuntimeOrigin::ALL.len());
    for origin in M5PreviewRuntimeOrigin::ALL {
        assert!(!origin.label().is_empty());
    }
    let actions: BTreeSet<&str> = M5BreakpointContinuityAction::ALL
        .iter()
        .map(|a| a.as_str())
        .collect();
    assert_eq!(actions.len(), M5BreakpointContinuityAction::ALL.len());
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_breakpoint_preview_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5BreakpointPreviewViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_breakpoint_preview_packet();
    packet.surface_rows[0].blurs_live_vs_mock = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5BreakpointPreviewViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_example_is_flagged() {
    let mut packet = seeded_m5_breakpoint_preview_packet();
    // Corrupt the stored resolution so it no longer matches a fresh resolve.
    packet.surface_rows[0].example_previews[0]
        .resolved
        .no_hidden_runtime_truth = false;
    let violations = packet.validate();
    assert!(violations.contains(&M5BreakpointPreviewViolation::ExamplePreviewDrift));
}

#[test]
fn mandatory_export_field_omission_is_flagged() {
    let mut packet = seeded_m5_breakpoint_preview_packet();
    packet.surface_rows[0].export_fields = vec![M5BreakpointExportField::DeviceClass];
    let violations = packet.validate();
    assert!(violations.contains(&M5BreakpointPreviewViolation::MandatoryExportFieldMissing));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_breakpoint_preview_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_breakpoint_preview_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_breakpoint_preview_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_breakpoint_preview_packet();
    assert_eq!(packet.record_kind, M5_BREAKPOINT_PREVIEW_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_BREAKPOINT_PREVIEW_SCHEMA_VERSION);
}
