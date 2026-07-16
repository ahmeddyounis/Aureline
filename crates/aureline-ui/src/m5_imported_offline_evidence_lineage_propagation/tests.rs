use super::*;

fn seed() -> M5ImportedOfflineLineagePacket {
    seeded_m5_imported_offline_lineage()
}

fn violations_of(packet: &M5ImportedOfflineLineagePacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_IMPORTED_OFFLINE_LINEAGE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_IMPORTED_OFFLINE_LINEAGE_RECORD_KIND);
    assert_eq!(packet.lineage_bindings.len(), 15);
}

#[test]
fn every_object_class_is_stated_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.lineage_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 5, "all five object classes stated");
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only stated by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_disposition_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet.lineage_bindings.iter().map(|b| b.consumer).collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let dispositions: BTreeSet<_> = packet
        .lineage_bindings
        .iter()
        .map(|b| b.disposition)
        .collect();
    for disposition in EvidenceLineageDisposition::ALL {
        assert!(
            dispositions.contains(&disposition),
            "disposition {} missing",
            disposition.as_str()
        );
    }
}

#[test]
fn companion_and_support_share_imported_offline_vocabulary() {
    // AC1: at least one companion / export surface and one support / AI consumer show imported / offline evidence
    // with the same non-live vocabulary and lineage fields as the primary archive viewer.
    let packet = seed();
    let companion = packet
        .lineage_bindings
        .iter()
        .find(|b| b.consumer == M5HistoricalReferenceConsumerSurface::CompanionExport)
        .expect("a companion / export binding is present");
    let support = packet
        .lineage_bindings
        .iter()
        .find(|b| b.consumer == M5HistoricalReferenceConsumerSurface::Support)
        .expect("a support / AI binding is present");
    for binding in [companion, support] {
        assert!(binding
            .non_live_grammar
            .imported_offline_label_is_canonical());
        assert_eq!(
            binding.non_live_grammar.imported_offline_label_word,
            M5_IMPORTED_OFFLINE_LABEL
        );
        assert!(!binding
            .lineage_descriptor
            .source_snapshot_descriptor_ref
            .trim()
            .is_empty());
        assert!(binding.lineage_descriptor.lineage_join.all_present());
    }
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_grammar: BTreeMap<&str, &NonLiveEvidenceGrammar> = BTreeMap::new();
    for binding in &packet.lineage_bindings {
        match profile_grammar.get(binding.lineage_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.lineage_profile_id.as_str(),
                    &binding.non_live_grammar,
                );
            }
            Some(existing) => assert_eq!(
                **existing, binding.non_live_grammar,
                "grammar drift on {}",
                binding.lineage_profile_id
            ),
        }
    }
    assert_eq!(profile_grammar.len(), 5);
}

#[test]
fn every_historical_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        assert!(
            binding
                .non_live_grammar
                .historical_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.non_live_grammar.historical_role_word,
            binding.binding_id
        );
        assert!(binding.non_live_grammar.all_present());
        assert!(binding
            .non_live_grammar
            .mutation_blocked_posture_satisfied());
        assert!(binding
            .non_live_grammar
            .imported_offline_label_is_canonical());
    }
}

#[test]
fn actions_are_closed_and_open_live_matches_disposition() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        assert!(binding.has_base_actions());
        assert!(binding.action_set_is_closed());
        assert!(binding.open_live_action_matches_disposition());
        // No rank / narrate / apply / sync / restore action exists in the closed action enum.
        assert!(!binding.allowed_actions.iter().any(|a| {
            let token = a.as_str();
            token.contains("rank")
                || token.contains("narrate")
                || token.contains("apply")
                || token.contains("sync")
                || token.contains("restore")
        }));
        // Only a live-target-joinable lineage offers open-current-live-object.
        let offers_open = binding
            .allowed_actions
            .contains(&LineageConsumerAction::OpenCurrentLiveObject);
        assert_eq!(
            offers_open,
            binding.disposition == EvidenceLineageDisposition::LiveTargetJoinable,
            "on {}",
            binding.binding_id
        );
    }
}

#[test]
fn content_presence_matches_disposition_for_every_binding() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        assert!(
            binding.content_presence_matches_disposition(),
            "content presence mismatch on {}",
            binding.binding_id
        );
    }
}

#[test]
fn metadata_only_bindings_render_metadata_instead_of_dead_link() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        if !binding.content_available {
            assert!(
                binding.renders_metadata_instead_of_dead_link(),
                "binding {} dead-links instead of rendering metadata",
                binding.binding_id
            );
            assert!(!binding
                .lineage_descriptor
                .non_live_boundary_note
                .trim()
                .is_empty());
            assert!(binding.non_live_grammar.capture_context_present());
        }
    }
}

#[test]
fn every_binding_joins_lineage_back_to_source_descriptor() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        assert!(
            !binding
                .lineage_descriptor
                .source_snapshot_descriptor_ref
                .trim()
                .is_empty(),
            "binding {} lineage is unjoined to a source descriptor",
            binding.binding_id
        );
        assert!(binding.lineage_descriptor.lineage_join.all_present());
        // A live-target handoff ref is present exactly when the lineage is joinable.
        assert_eq!(
            binding.lineage_descriptor.live_target_handoff_ref.is_some(),
            binding.is_live_target_joinable(),
            "on {}",
            binding.binding_id
        );
        assert_eq!(
            binding.lineage_descriptor.metadata_only_exit_ref.is_some(),
            !binding.is_live_target_joinable(),
            "on {}",
            binding.binding_id
        );
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn joinable_and_boundary_bindings_split_correctly() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
        let disclosure = binding.disclosure();
        let descriptor = &binding.lineage_descriptor;
        assert_eq!(descriptor.next_action, disclosure.next_action);
        if binding.is_live_target_joinable() {
            assert_eq!(binding.parity_state, LineageParity::LiveTargetLineageJoined);
            assert!(descriptor.live_target_handoff_ref.is_some());
            assert!(descriptor.metadata_only_exit_ref.is_none());
            assert_eq!(
                descriptor.next_action,
                LineageNextAction::OpenCurrentLiveObjectThroughValidatedHandoff
            );
        } else {
            assert_eq!(
                binding.parity_state,
                LineageParity::NonLiveBoundaryDisclosed
            );
            assert!(descriptor.live_target_handoff_ref.is_none());
            assert!(descriptor.metadata_only_exit_ref.is_some());
            assert_eq!(
                descriptor.next_action,
                LineageNextAction::InspectLineageMetadataOnly
            );
        }
        assert!(!descriptor.non_live_boundary_note.trim().is_empty());
        assert!(!descriptor.next_action_label.trim().is_empty());
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.lineage_bindings {
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
fn disclosure_resolver_matches_disposition() {
    let joinable =
        resolve_lineage_disposition_disclosure(EvidenceLineageDisposition::LiveTargetJoinable);
    assert!(joinable.offers_open_live_target);
    assert!(joinable.requires_live_target_handoff_ref);
    assert!(!joinable.requires_metadata_only_exit_ref);
    assert!(joinable.expects_content_available);
    assert_eq!(
        joinable.next_action,
        LineageNextAction::OpenCurrentLiveObjectThroughValidatedHandoff
    );

    let imported =
        resolve_lineage_disposition_disclosure(EvidenceLineageDisposition::ImportedOfflineOnly);
    assert!(!imported.offers_open_live_target);
    assert!(imported.requires_metadata_only_exit_ref);
    assert!(imported.expects_content_available);

    let metadata_only =
        resolve_lineage_disposition_disclosure(EvidenceLineageDisposition::MetadataOnlyExit);
    assert!(!metadata_only.offers_open_live_target);
    assert!(!metadata_only.expects_content_available);
    assert_eq!(
        metadata_only.next_action,
        LineageNextAction::InspectLineageMetadataOnly
    );

    let exported =
        resolve_lineage_disposition_disclosure(EvidenceLineageDisposition::ExportedRedactedLineage);
    assert!(!exported.offers_open_live_target);
    assert!(exported.requires_metadata_only_exit_ref);
    assert!(exported.expects_content_available);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| b.binding_id == "iol-retirement-shell")
        .unwrap();
    packet.lineage_bindings[target]
        .non_live_grammar
        .historical_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"grammar_drift_across_surfaces"));
}

#[test]
fn historical_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0]
        .non_live_grammar
        .historical_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"historical_role_word_outside_vocabulary"));
}

#[test]
fn non_canonical_imported_offline_label_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0]
        .non_live_grammar
        .imported_offline_label_word = "showing cached stuff".to_owned();
    assert!(violations_of(&packet).contains(&"imported_offline_label_not_canonical"));
}

#[test]
fn dropped_mutation_blocked_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // iol-runbook-archive carries the snapshot_labeling gate role, which must always keep a real
    // mutation-blocked posture.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| b.binding_id == "iol-runbook-archive")
        .unwrap();
    packet.lineage_bindings[target]
        .non_live_grammar
        .mutation_blocked_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"mutation_blocked_posture_missing_for_gate_role"));
}

#[test]
fn content_presence_mismatch_is_rejected() {
    let mut packet = seed();
    // Flip the content flag on a live-target-joinable lineage, which must keep its content available.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| b.disposition == EvidenceLineageDisposition::LiveTargetJoinable)
        .unwrap();
    packet.lineage_bindings[target].content_available = false;
    assert!(violations_of(&packet).contains(&"content_presence_mismatch"));
}

#[test]
fn parity_state_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| b.disposition == EvidenceLineageDisposition::LiveTargetJoinable)
        .unwrap();
    packet.lineage_bindings[target].parity_state = LineageParity::NonLiveBoundaryDisclosed;
    assert!(violations_of(&packet).contains(&"parity_state_mismatch"));
}

#[test]
fn missing_disposition_label_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0].disposition_label = String::new();
    assert!(violations_of(&packet).contains(&"disposition_label_missing"));
}

#[test]
fn non_live_boundary_not_called_out_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0].non_live_boundary_explicitly_called_out = false;
    assert!(violations_of(&packet).contains(&"non_live_boundary_not_called_out"));
}

#[test]
fn open_live_action_disposition_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a non-joinable binding, which must not offer it.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| !b.is_live_target_joinable())
        .unwrap();
    packet.lineage_bindings[target]
        .allowed_actions
        .push(LineageConsumerAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_disposition_mismatch"));
}

#[test]
fn missing_base_action_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0]
        .allowed_actions
        .retain(|a| *a != LineageConsumerAction::ExportLineage);
    assert!(violations_of(&packet).contains(&"base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.lineage_bindings.retain(|b| {
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
        .lineage_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.lineage_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_source_descriptor_join_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0]
        .lineage_descriptor
        .source_snapshot_descriptor_ref = String::new();
    assert!(violations_of(&packet).contains(&"source_descriptor_join_missing"));
}

#[test]
fn incomplete_lineage_join_is_rejected() {
    let mut packet = seed();
    packet.lineage_bindings[0]
        .lineage_descriptor
        .lineage_join
        .provenance_lineage_ref = String::new();
    assert!(violations_of(&packet).contains(&"lineage_join_incomplete"));
}

#[test]
fn live_target_handoff_ref_mismatch_is_rejected() {
    let mut packet = seed();
    // Strip the handoff ref off a joinable binding, which must carry it.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| b.is_live_target_joinable())
        .unwrap();
    packet.lineage_bindings[target]
        .lineage_descriptor
        .live_target_handoff_ref = None;
    assert!(violations_of(&packet).contains(&"live_target_handoff_ref_mismatch"));
}

#[test]
fn metadata_only_exit_ref_mismatch_is_rejected() {
    let mut packet = seed();
    // Strip the metadata-only exit ref off a non-joinable binding, which must carry it.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| !b.is_live_target_joinable())
        .unwrap();
    packet.lineage_bindings[target]
        .lineage_descriptor
        .metadata_only_exit_ref = None;
    assert!(violations_of(&packet).contains(&"metadata_only_exit_ref_mismatch"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut ImportedOfflineLineageBinding), &str); 5] = [
        (
            |b| b.ranked_or_narrated_as_current_live_service_truth = true,
            "ranked_or_narrated_as_current_live_service_truth",
        ),
        (
            |b| b.presents_imported_offline_as_current_route_or_provider_state = true,
            "presents_imported_offline_as_current_route_or_provider_state",
        ),
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.leaks_live_secret_or_stale_authority_through_lineage = true,
            "leaks_live_secret_or_stale_authority_through_lineage",
        ),
        (
            |b| b.drops_non_live_vocabulary_in_export = true,
            "drops_non_live_vocabulary_in_export",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.lineage_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn metadata_fallback_missing_is_rejected() {
    let mut packet = seed();
    // A content-gone binding whose non-live boundary note is stripped dead-links.
    let target = packet
        .lineage_bindings
        .iter()
        .position(|b| !b.content_available)
        .unwrap();
    packet.lineage_bindings[target]
        .lineage_descriptor
        .non_live_boundary_note = String::new();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"metadata_fallback_missing"));
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .lineage_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn disposition_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every metadata-only-exit binding, leaving that disposition uncovered.
    packet
        .lineage_bindings
        .retain(|b| b.disposition != EvidenceLineageDisposition::MetadataOnlyExit);
    assert!(violations_of(&packet).contains(&"disposition_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_IMPORTED_OFFLINE_LINEAGE_DOC_REF);
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
    assert_eq!(lines.len(), 1 + packet.lineage_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,consumer,disposition,content_available,next_action,parity_state,disposition_label"
    ));
}

#[test]
fn csv_preserves_imported_offline_vocabulary() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    assert!(csv.contains(",live_target_joinable,"));
    assert!(csv.contains(",imported_offline_only,"));
    assert!(csv.contains(",metadata_only_exit,"));
    assert!(csv.contains(",exported_redacted_lineage,"));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.lineage_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_imported_offline_lineage_export()
        .expect("checked M5 imported / offline lineage export validates");
    assert_eq!(from_disk.packet_id, M5_IMPORTED_OFFLINE_LINEAGE_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let imported = seeded_m5_imported_offline_lineage_imported_offline_narrowed();
    assert!(
        imported.validate().is_empty(),
        "{:?}",
        violations_of(&imported)
    );
    assert_eq!(imported.lineage_bindings.len(), 15);

    let metadata_only = seeded_m5_imported_offline_lineage_metadata_only_narrowed();
    assert!(
        metadata_only.validate().is_empty(),
        "{:?}",
        violations_of(&metadata_only)
    );
    assert_eq!(metadata_only.lineage_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let imported: M5ImportedOfflineLineagePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/recovery/m5-imported-offline-lineage/imported_offline_narrowed.json"
    )))
    .expect("imported-offline fixture parses");
    assert!(imported.validate().is_empty());
    assert_eq!(
        imported,
        seeded_m5_imported_offline_lineage_imported_offline_narrowed()
    );

    let metadata_only: M5ImportedOfflineLineagePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-imported-offline-lineage/metadata_only_narrowed.json"
        )))
        .expect("metadata-only fixture parses");
    assert!(metadata_only.validate().is_empty());
    assert_eq!(
        metadata_only,
        seeded_m5_imported_offline_lineage_metadata_only_narrowed()
    );
}
