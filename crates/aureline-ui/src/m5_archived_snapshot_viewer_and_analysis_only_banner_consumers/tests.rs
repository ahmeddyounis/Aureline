use super::*;

fn seed() -> M5ArchivedSnapshotViewerConsumersPacket {
    seeded_m5_archived_snapshot_viewer_consumers()
}

fn violations_of(packet: &M5ArchivedSnapshotViewerConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 15);
}

#[test]
fn every_object_class_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 5, "all five object classes adopted");
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only adopted by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_posture_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.consumer)
        .collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let postures: BTreeSet<_> = packet.consumer_bindings.iter().map(|b| b.posture).collect();
    for posture in ArchiveViewPosture::ALL {
        assert!(
            postures.contains(&posture),
            "posture {} missing",
            posture.as_str()
        );
    }
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_grammar: BTreeMap<&str, &ArchiveViewerBannerGrammar> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match profile_grammar.get(binding.evidence_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.evidence_profile_id.as_str(),
                    &binding.banner_grammar,
                );
            }
            Some(existing) => assert_eq!(
                **existing, binding.banner_grammar,
                "grammar drift on {}",
                binding.evidence_profile_id
            ),
        }
    }
    // The five profiles each fan out to more than one consumer.
    assert_eq!(profile_grammar.len(), 5);
}

#[test]
fn every_banner_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.banner_grammar.banner_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.banner_grammar.banner_role_word,
            binding.binding_id
        );
        assert!(binding.banner_grammar.all_present());
        assert!(binding.banner_grammar.analysis_only_posture_satisfied());
    }
}

#[test]
fn actions_are_analysis_only_and_open_live_matches_posture() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(binding.has_analysis_only_base_actions());
        assert!(binding.open_live_action_matches_posture());
        // A live-target-openable view offers open-current-live-object; a narrowed view never does.
        let offers = binding
            .allowed_actions
            .contains(&ArchiveAction::OpenCurrentLiveObject);
        assert_eq!(offers, !binding.is_narrowed(), "on {}", binding.binding_id);
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn narrowed_bindings_disclose_and_full_bindings_do_not() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        let disclosure = binding.disclosure();
        if binding.is_narrowed() {
            assert_eq!(
                binding.parity_state,
                ArchiveViewerParityState::FacetsDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_grammar_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.parity_state,
                ArchiveViewerParityState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(binding.posture, ArchiveViewPosture::ImportedOfflineOnly) {
            assert!(!binding.import_offline_note.trim().is_empty());
        }
        if matches!(binding.posture, ArchiveViewPosture::ExportedRedacted) {
            assert!(!binding.export_detail_note.trim().is_empty());
        }
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        if consumer_must_reference_canonical(binding.consumer) {
            assert!(
                binding.points_at_canonical_contracts(),
                "binding {} must point at canonical contracts",
                binding.binding_id
            );
        }
    }
}

#[test]
fn disclosure_resolver_matches_posture() {
    let live = resolve_archive_view_render_disclosure(ArchiveViewPosture::LiveTargetOpenable);
    assert!(!live.needs_narrow_note);
    assert!(live.offers_open_live_target);

    let metadata = resolve_archive_view_render_disclosure(ArchiveViewPosture::MetadataOnlyExit);
    assert_eq!(
        metadata.narrow_reason,
        Some(ArchiveNarrowReason::LiveTargetRemovedMetadataOnly)
    );
    assert!(metadata.needs_narrow_note);
    assert!(!metadata.offers_open_live_target);

    let imported = resolve_archive_view_render_disclosure(ArchiveViewPosture::ImportedOfflineOnly);
    assert!(imported.needs_import_offline_note);
    assert!(!imported.offers_open_live_target);

    let exported = resolve_archive_view_render_disclosure(ArchiveViewPosture::ExportedRedacted);
    assert!(exported.needs_export_detail_note);
    assert!(!exported.offers_open_live_target);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding profile to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "asvc-retirement-shell")
        .unwrap();
    packet.consumer_bindings[target]
        .banner_grammar
        .banner_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"archive_banner_grammar_drift_across_surfaces"));
}

#[test]
fn banner_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].banner_grammar.banner_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"banner_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_analysis_only_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // asvc-retirement-release carries the snapshot_labeling gate role, which must always keep a real
    // analysis-only posture.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "asvc-retirement-release")
        .unwrap();
    packet.consumer_bindings[target]
        .banner_grammar
        .analysis_only_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"analysis_only_posture_missing_for_gate_role"));
}

#[test]
fn write_capable_control_shown_as_live_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].presents_write_capable_control_as_if_current_object_open_live =
        true;
    assert!(violations_of(&packet)
        .contains(&"presents_write_capable_control_as_if_current_object_open_live"));
}

#[test]
fn open_live_action_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a narrowed (metadata-only) binding, which must not offer it.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target]
        .allowed_actions
        .push(ArchiveAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_posture_mismatch"));
}

#[test]
fn missing_analysis_only_base_action_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .allowed_actions
        .retain(|a| *a != ArchiveAction::Compare);
    assert!(violations_of(&packet).contains(&"analysis_only_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every imported-offline-route-evidence binding except one, leaving the class with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.object_class == M5HistoricalReferenceObject::ImportedOfflineRouteEvidence {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"object_class_reuse_unproven"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.consumer_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_narrow_note_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = None;
    assert!(violations_of(&packet).contains(&"narrow_note_missing"));
}

#[test]
fn unexpected_narrow_note_on_full_binding_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| !b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target].narrow_note = Some(ArchiveNarrowNote {
        reason: ArchiveNarrowReason::LiveTargetRemovedMetadataOnly,
        preserved_grammar_note: "x".to_owned(),
        next_action: ArchiveNarrowNextAction::OpenMetadataOnlyExit,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut ArchiveViewerConsumerBinding), &str); 5] = [
        (
            |b| b.presents_write_capable_control_as_if_current_object_open_live = true,
            "presents_write_capable_control_as_if_current_object_open_live",
        ),
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.dead_links_expired_or_removed_artifact_instead_of_showing_metadata = true,
            "dead_links_expired_or_removed_artifact_instead_of_showing_metadata",
        ),
        (
            |b| b.leaves_non_live_evidence_unjoined_to_capture_context = true,
            "leaves_non_live_evidence_unjoined_to_capture_context",
        ),
        (
            |b| {
                b.lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission =
                    true
            },
            "lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.consumer_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .consumer_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.consumer_bindings.len());
    assert!(lines[0].starts_with("object_class,consumer,posture,banner_role_word,parity_state"));
}

#[test]
fn markdown_summary_lists_every_profile() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_archived_snapshot_viewer_consumers_export()
        .expect("checked M5 archived-snapshot-viewer consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_ARCHIVED_SNAPSHOT_VIEWER_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let metadata = seeded_m5_archived_snapshot_viewer_consumers_metadata_only_narrowed();
    assert!(
        metadata.validate().is_empty(),
        "{:?}",
        violations_of(&metadata)
    );
    assert_eq!(metadata.consumer_bindings.len(), 15);

    let imported = seeded_m5_archived_snapshot_viewer_consumers_imported_offline_narrowed();
    assert!(
        imported.validate().is_empty(),
        "{:?}",
        violations_of(&imported)
    );
    assert_eq!(imported.consumer_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let metadata: M5ArchivedSnapshotViewerConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-archived-snapshot-viewer-consumers/metadata_only_narrowed.json"
        )))
        .expect("metadata-only fixture parses");
    assert!(metadata.validate().is_empty());
    assert_eq!(
        metadata,
        seeded_m5_archived_snapshot_viewer_consumers_metadata_only_narrowed()
    );

    let imported: M5ArchivedSnapshotViewerConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-archived-snapshot-viewer-consumers/imported_offline_narrowed.json"
        )))
        .expect("imported / offline fixture parses");
    assert!(imported.validate().is_empty());
    assert_eq!(
        imported,
        seeded_m5_archived_snapshot_viewer_consumers_imported_offline_narrowed()
    );
}
