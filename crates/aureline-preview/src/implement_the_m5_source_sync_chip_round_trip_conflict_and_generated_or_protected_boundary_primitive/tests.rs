//! Tests for the M5 source-round-trip honesty primitive: the resolver, the parity
//! matrix, and the checked-in support export.

use super::*;

fn writable_input() -> M5RoundTripStatusInput {
    M5RoundTripStatusInput {
        target_id: "target:test:0001".to_owned(),
        node_label: "TestNode".to_owned(),
        file_label: "src/components/Test.tsx".to_owned(),
        sync_class: SourceSyncClass::InSyncFromSource,
        round_trip: RoundTripCapabilityClass::ExactSourceRoundTrip,
        boundary: M5SourceBoundaryClass::AuthorOwned,
        protected_posture: ProtectedPathPosture::Unprotected,
        has_unsaved_visual_edit: false,
        source_span_ref: Some("span:test:0001".to_owned()),
        conflict: None,
        unsupported: None,
    }
}

// --- resolver: baseline writable ---

#[test]
fn resolver_writable_baseline_carries_no_disclosure() {
    let resolved = resolve_round_trip_status(&writable_input()).expect("resolves");
    assert_eq!(resolved.write_authority, M5WriteAuthority::Writable);
    assert!(resolved.writes_back());
    assert!(!resolved.is_narrowed());
    assert!(resolved.source_first_fallback.is_none());
    assert!(resolved.downgrade_trigger.is_none());
    assert!(resolved.conflict_banner.is_none());
    assert!(resolved.unsupported_card.is_none());
    assert!(resolved.boundary_notice.is_none());
    assert_eq!(resolved.chip.chip_state, M5SourceSyncChipState::InSync);
}

#[test]
fn resolver_unsaved_edit_shows_unsaved_chip() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        has_unsaved_visual_edit: true,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(resolved.chip.chip_state, M5SourceSyncChipState::Unsaved);
    assert!(resolved.chip.open_diff_action_available);
}

// --- resolver: AC1 hard blocks never silently normalized ---

#[test]
fn resolver_conflict_narrows_and_fires_banner() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        sync_class: SourceSyncClass::DriftedFromSource,
        conflict: Some(M5RoundTripConflictClass::SourceChangedUnderEdit),
        has_unsaved_visual_edit: true,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(
        resolved.write_authority,
        M5WriteAuthority::SourceOnlyFallback
    );
    assert!(!resolved.writes_back());
    assert!(resolved.has_hard_block());
    assert!(resolved.refuses_silent_normalization());
    let banner = resolved.conflict_banner.as_ref().expect("banner present");
    assert_eq!(
        banner.resolution_route,
        M5ConflictResolutionRoute::ReloadSourceReapply
    );
    assert_eq!(
        banner.source_first_fallback,
        M5SourceFirstFallback::ReloadSourceThenReapply
    );
    assert!(banner.never_silent_writeback);
    assert!(banner.preserves_selection_context);
    assert!(banner.refresh_action_available && banner.compare_action_available);
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::RoundTripConflictOpen)
    );
    assert_eq!(resolved.chip.chip_state, M5SourceSyncChipState::Conflict);
}

#[test]
fn resolver_unsupported_construct_fires_card_and_never_writes() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        round_trip: RoundTripCapabilityClass::ApproximateSourceRoundTrip,
        unsupported: Some(UnsupportedConstructReason::DynamicBinding),
        ..writable_input()
    })
    .expect("resolves");
    assert!(resolved.has_hard_block());
    assert!(resolved.refuses_silent_normalization());
    assert!(!resolved.writes_back());
    let card = resolved.unsupported_card.as_ref().expect("card present");
    assert_eq!(card.reason, UnsupportedConstructReason::DynamicBinding);
    assert!(card.preserves_selection_context);
    assert_eq!(
        card.source_first_fallback,
        M5SourceFirstFallback::OpenSourceEditDirectly
    );
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::UnsupportedConstruct)
    );
    // An approximate round-trip would otherwise write back, so this is a narrowing.
    assert!(resolved.is_narrowed());
    assert!(resolved.narrowing_is_explained());
}

#[test]
fn resolver_generated_file_blocks_silent_widening() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        boundary: M5SourceBoundaryClass::GeneratedManaged,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(resolved.write_authority, M5WriteAuthority::ReadOnly);
    assert!(!resolved.writes_back());
    let notice = resolved.boundary_notice.as_ref().expect("notice present");
    assert!(!notice.designer_write_permitted);
    assert!(notice.requires_owner_flow);
    assert!(notice.refuses_silent_widening);
    assert!(resolved.has_hard_block());
    assert!(resolved.refuses_silent_normalization());
    assert_eq!(
        resolved.source_first_fallback,
        Some(M5SourceFirstFallback::OpenManagedFileOwnerFlow)
    );
    assert_eq!(
        resolved.downgrade_trigger,
        Some(M5VisualDesignerDowngradeTrigger::ProtectedPathBlocked)
    );
}

#[test]
fn resolver_protected_blocked_path_is_read_only() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        boundary: M5SourceBoundaryClass::ProtectedReadOnly,
        protected_posture: ProtectedPathPosture::ProtectedBlocked,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(resolved.write_authority, M5WriteAuthority::ReadOnly);
    let notice = resolved.boundary_notice.as_ref().expect("notice present");
    assert!(!notice.designer_write_permitted);
}

// --- resolver: AC2 source-first fallback when round-trip drops ---

#[test]
fn resolver_source_only_round_trip_falls_back_to_source() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        round_trip: RoundTripCapabilityClass::SourceOnlyFallback,
        source_span_ref: Some("span:test:src-only".to_owned()),
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(
        resolved.write_authority,
        M5WriteAuthority::SourceOnlyFallback
    );
    assert!(resolved.offers_source_first_fallback());
    assert_eq!(
        resolved.source_first_fallback,
        Some(M5SourceFirstFallback::OpenSourceEditDirectly)
    );
}

#[test]
fn resolver_runtime_only_is_read_only_inspect_fallback() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        sync_class: SourceSyncClass::RuntimeOnlyNoSource,
        round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
        source_span_ref: None,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(resolved.write_authority, M5WriteAuthority::ReadOnly);
    assert_eq!(
        resolved.chip.chip_state,
        M5SourceSyncChipState::NeedsRefresh
    );
    assert_eq!(
        resolved.chip.recovery_route,
        M5SyncRecoveryRoute::ReattachRuntime
    );
    assert_eq!(
        resolved.source_first_fallback,
        Some(M5SourceFirstFallback::InspectOnlyNoWrite)
    );
    // Inspect-only is the surface's baseline, not a narrowing from a write path.
    assert!(!resolved.is_narrowed());
}

#[test]
fn resolver_mixed_managed_region_is_writable_with_review() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        boundary: M5SourceBoundaryClass::MixedManagedRegion,
        protected_posture: ProtectedPathPosture::ProtectedReviewRequired,
        ..writable_input()
    })
    .expect("resolves");
    assert_eq!(
        resolved.write_authority,
        M5WriteAuthority::WritableWithReview
    );
    assert!(resolved.writes_back());
    let notice = resolved.boundary_notice.as_ref().expect("notice present");
    assert!(notice.designer_write_permitted);
    assert!(!notice.requires_owner_flow);
    // A write-back path names no source-first fallback.
    assert!(resolved.source_first_fallback.is_none());
}

// --- resolver: structural guards ---

#[test]
fn resolver_rejects_write_back_without_span() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        source_span_ref: None,
        ..writable_input()
    });
    assert_eq!(
        resolved,
        Err(M5RoundTripResolutionError::MissingSpanForSourceRoundTrip)
    );
}

#[test]
fn resolver_rejects_runtime_only_with_span() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        sync_class: SourceSyncClass::RuntimeOnlyNoSource,
        round_trip: RoundTripCapabilityClass::InspectOnlyNoWrite,
        source_span_ref: Some("span:should-not-exist".to_owned()),
        ..writable_input()
    });
    assert_eq!(
        resolved,
        Err(M5RoundTripResolutionError::ContradictoryRuntimeSpan)
    );
}

#[test]
fn resolver_rejects_empty_identity() {
    assert_eq!(
        resolve_round_trip_status(&M5RoundTripStatusInput {
            target_id: "  ".to_owned(),
            ..writable_input()
        }),
        Err(M5RoundTripResolutionError::EmptyTargetId)
    );
    assert_eq!(
        resolve_round_trip_status(&M5RoundTripStatusInput {
            node_label: "".to_owned(),
            ..writable_input()
        }),
        Err(M5RoundTripResolutionError::EmptyNodeLabel)
    );
    assert_eq!(
        resolve_round_trip_status(&M5RoundTripStatusInput {
            file_label: "".to_owned(),
            ..writable_input()
        }),
        Err(M5RoundTripResolutionError::EmptyFileLabel)
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    assert_eq!(
        resolve_round_trip_status(&M5RoundTripStatusInput {
            file_label: "https://example.com/leak".to_owned(),
            ..writable_input()
        }),
        Err(M5RoundTripResolutionError::ForbiddenMaterial)
    );
}

#[test]
fn resolver_identity_is_consistent_across_components() {
    let resolved = resolve_round_trip_status(&M5RoundTripStatusInput {
        boundary: M5SourceBoundaryClass::GeneratedManaged,
        conflict: Some(M5RoundTripConflictClass::GeneratedFileProtected),
        ..writable_input()
    })
    .expect("resolves");
    assert!(resolved.identity_consistent());
    assert_eq!(resolved.chip.target_id, resolved.target_id);
    assert_eq!(
        resolved.conflict_banner.as_ref().unwrap().target_id,
        resolved.target_id
    );
    assert_eq!(
        resolved.boundary_notice.as_ref().unwrap().target_id,
        resolved.target_id
    );
}

// --- packet: seed + validation ---

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_round_trip_honesty_packet();
    assert!(
        packet.validate().is_empty(),
        "seeded packet validates: {:?}",
        packet.validate()
    );
}

#[test]
fn seeded_packet_covers_every_surface_family() {
    let packet = seeded_m5_round_trip_honesty_packet();
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
fn seeded_example_statuses_are_self_consistent() {
    let packet = seeded_m5_round_trip_honesty_packet();
    for row in &packet.surface_rows {
        for case in &row.example_statuses {
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
    assert!(M5RoundTripVocabularySet::canonical().matches_canonical());
    let packet = seeded_m5_round_trip_honesty_packet();
    assert!(packet.vocabulary_set.matches_canonical());
}

#[test]
fn chip_state_and_editor_kinds_are_one_to_one_with_value_states() {
    // Every chip state has a distinct token and a non-empty precise label.
    let tokens: BTreeSet<&str> = M5SourceSyncChipState::ALL
        .iter()
        .map(|s| s.as_str())
        .collect();
    assert_eq!(tokens.len(), M5SourceSyncChipState::ALL.len());
    for state in M5SourceSyncChipState::ALL {
        assert!(!state.label().is_empty());
    }
}

#[test]
fn missing_surface_family_is_flagged() {
    let mut packet = seeded_m5_round_trip_honesty_packet();
    packet.surface_rows.remove(0);
    let violations = packet.validate();
    assert!(violations.contains(&M5RoundTripHonestyViolation::RequiredSurfaceMissing));
}

#[test]
fn invariant_violation_is_flagged() {
    let mut packet = seeded_m5_round_trip_honesty_packet();
    packet.surface_rows[0].normalizes_unsupported_silently = true;
    let violations = packet.validate();
    assert!(violations.contains(&M5RoundTripHonestyViolation::SurfaceInvariantViolated));
}

#[test]
fn drifted_example_is_flagged() {
    let mut packet = seeded_m5_round_trip_honesty_packet();
    // Corrupt the stored resolution so it no longer matches a fresh resolve.
    packet.surface_rows[0].example_statuses[0]
        .resolved
        .write_authority = M5WriteAuthority::ReadOnly;
    let violations = packet.validate();
    assert!(violations.contains(&M5RoundTripHonestyViolation::ExampleStatusDrift));
}

// --- checked-in artifact ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_stable_m5_round_trip_honesty_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_round_trip_honesty_packet());
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_round_trip_honesty_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_round_trip_honesty_packet();
    assert_eq!(packet.record_kind, M5_ROUND_TRIP_HONESTY_RECORD_KIND);
    assert_eq!(packet.schema_version, M5_ROUND_TRIP_HONESTY_SCHEMA_VERSION);
}
