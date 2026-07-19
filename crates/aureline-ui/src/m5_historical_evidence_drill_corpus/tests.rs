use super::*;

use crate::m5_live_target_handoff_packet_and_route_validation::{
    HandoffBlockerReason, HandoffOutcome,
};

fn seed() -> M5HistoricalEvidenceDrillCorpusPacket {
    seeded_m5_historical_evidence_drill_corpus()
}

fn violations_of(packet: &M5HistoricalEvidenceDrillCorpusPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_HISTORICAL_EVIDENCE_DRILL_PACKET_ID);
    assert_eq!(packet.record_kind, M5_HISTORICAL_EVIDENCE_DRILL_RECORD_KIND);
    assert_eq!(packet.drill_bindings.len(), 15);
}

#[test]
fn every_object_class_is_seeded_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.drill_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 5, "all five object classes seeded");
    for (object_class, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object class {} only seeded by {} consumers",
            object_class.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_drill_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet.drill_bindings.iter().map(|b| b.consumer).collect();
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let drills: BTreeSet<_> = packet.drill_bindings.iter().map(|b| b.drill).collect();
    for drill in DrillScenario::ALL {
        assert!(drills.contains(&drill), "drill {} missing", drill.as_str());
    }
}

#[test]
fn ac1_four_or_more_states_and_two_or_more_handoff_outcomes() {
    // AC1: the seeded fixtures exercise at least four distinct historical-reference states and two distinct
    // live-target handoff outcomes.
    let packet = seed();
    let states: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.evidence_state)
        .collect();
    assert!(
        states.len() >= 4,
        "only {} distinct historical-reference states",
        states.len()
    );
    let outcomes: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.expected_handoff_outcome)
        .collect();
    assert!(
        outcomes.len() >= 2,
        "only {} distinct handoff outcomes",
        outcomes.len()
    );
    // Both a cleared and a blocked outcome are present.
    assert!(outcomes.contains(&HandoffOutcome::HandoffCleared));
    assert!(outcomes.iter().any(|o| o.is_blocked()));
}

#[test]
fn ac2_every_exact_blocker_is_distinguishable() {
    // AC2: QA / support automation can distinguish exact blockers such as missing target, trust block, route
    // unavailable, expired snapshot, and imported / offline evidence only.
    let packet = seed();
    let blockers: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.expected_blocker)
        .collect();
    for blocker in DrillBlocker::ALL {
        assert!(
            blockers.contains(&blocker),
            "exact blocker {} not exercised",
            blocker.as_str()
        );
    }
    for named in [
        DrillBlocker::MissingTarget,
        DrillBlocker::TrustBlock,
        DrillBlocker::RouteUnavailable,
        DrillBlocker::ExpiredSnapshot,
        DrillBlocker::ImportedOfflineEvidenceOnly,
    ] {
        assert!(blockers.contains(&named), "{} missing", named.as_str());
    }
}

#[test]
fn blocker_maps_into_handoff_module_vocabulary_and_outcome() {
    for blocker in DrillBlocker::ALL {
        let outcome = blocker.required_outcome();
        match blocker.maps_to_handoff_blocker_reason() {
            None => assert_eq!(outcome, HandoffOutcome::HandoffCleared),
            Some(reason) => {
                assert!(outcome.is_blocked());
                // The mapped reason is a member of the outcome's own allowed-reason set in the handoff module.
                assert!(
                    outcome.allowed_blocker_reasons().contains(&reason),
                    "{} maps to a reason outside {}'s allowed set",
                    blocker.as_str(),
                    outcome.as_str()
                );
            }
        }
    }
    // The two distinctly-named handoff blocker reasons used are meaningful members of the frozen vocabulary.
    assert!(HandoffBlockerReason::ALL.contains(&HandoffBlockerReason::TargetDoesNotExist));
    assert!(HandoffBlockerReason::ALL.contains(&HandoffBlockerReason::TrustPostureInsufficient));
}

#[test]
fn every_binding_blocker_matches_its_outcome_and_drill() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.blocker_matches_outcome(),
            "blocker/outcome drift on {}",
            binding.binding_id
        );
        let disclosure = binding.disclosure();
        assert_eq!(binding.evidence_state, disclosure.expected_state);
        assert_eq!(
            binding.expected_handoff_outcome,
            disclosure.expected_handoff_outcome
        );
        assert_eq!(binding.expected_blocker, disclosure.expected_blocker);
        assert_eq!(binding.parity_state, disclosure.parity);
    }
}

#[test]
fn four_fixture_families_are_seeded() {
    // The corpus seeds a last-supported retirement snapshot, a support / export evidence bundle, a runbook /
    // incident archived packet, and an imported / offline route packet.
    let packet = seed();
    let fixtures: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.fixture_id.as_str())
        .collect();
    for family in [
        "last-supported-retirement-snapshot",
        "support-export-evidence-bundle",
        "runbook-incident-archived-packet",
        "imported-offline-route-packet",
    ] {
        assert!(fixtures.contains(family), "fixture family {family} missing");
    }
    assert!(fixtures.len() >= 4);
}

#[test]
fn same_fixture_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut fixture_grammar: BTreeMap<&str, &HistoricalEvidenceGrammar> = BTreeMap::new();
    for binding in &packet.drill_bindings {
        match fixture_grammar.get(binding.fixture_id.as_str()) {
            None => {
                fixture_grammar.insert(binding.fixture_id.as_str(), &binding.non_live_grammar);
            }
            Some(existing) => assert_eq!(
                **existing, binding.non_live_grammar,
                "grammar drift on {}",
                binding.fixture_id
            ),
        }
    }
    assert_eq!(fixture_grammar.len(), 5);
}

#[test]
fn every_historical_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.drill_bindings {
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
        assert!(binding.non_live_grammar.capture_context_present());
    }
}

#[test]
fn actions_are_closed_and_open_live_matches_drill() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(binding.has_base_actions());
        assert!(binding.action_set_is_closed());
        assert!(binding.open_live_action_matches_drill());
        // No apply / sync / restore / rank / narrate action exists in the closed action enum.
        assert!(!binding.allowed_actions.iter().any(|a| {
            let token = a.as_str();
            token.contains("apply")
                || token.contains("sync")
                || token.contains("restore")
                || token.contains("rank")
                || token.contains("narrate")
        }));
        // Only a handoff-clearing drill offers open-current-live-object.
        let offers_open = binding
            .allowed_actions
            .contains(&DrillCorpusAction::OpenCurrentLiveObject);
        assert_eq!(
            offers_open,
            binding.is_cleared_handoff(),
            "on {}",
            binding.binding_id
        );
    }
}

#[test]
fn content_presence_matches_drill_for_every_binding() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.content_presence_matches_drill(),
            "content presence mismatch on {}",
            binding.binding_id
        );
    }
}

#[test]
fn expired_snapshot_drill_renders_metadata_instead_of_dead_link() {
    let packet = seed();
    let mut saw_expired = false;
    for binding in &packet.drill_bindings {
        if binding.drill == DrillScenario::ExpiredSnapshotMetadataOnlyFallback {
            saw_expired = true;
            assert!(!binding.content_available, "expired content should be gone");
            assert!(
                binding.renders_metadata_instead_of_dead_link(),
                "binding {} dead-links instead of rendering metadata",
                binding.binding_id
            );
            assert!(binding.non_live_grammar.capture_context_present());
            assert!(!binding.handoff_expectation.blocker_note.trim().is_empty());
        }
    }
    assert!(saw_expired, "no expired-snapshot drill present");
}

#[test]
fn handoff_expectation_refs_match_drill() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        let disclosure = binding.disclosure();
        let expectation = &binding.handoff_expectation;
        assert_eq!(
            expectation.live_target_handoff_ref.is_some(),
            disclosure.requires_live_target_handoff_ref,
            "handoff ref on {}",
            binding.binding_id
        );
        assert_eq!(
            expectation.metadata_only_exit_ref.is_some(),
            disclosure.requires_metadata_only_exit_ref,
            "metadata exit ref on {}",
            binding.binding_id
        );
        assert_eq!(
            expectation.satisfy_prerequisite_ref.is_some(),
            disclosure.requires_satisfy_prerequisite_ref,
            "satisfy prerequisite ref on {}",
            binding.binding_id
        );
        assert!(!expectation.blocker_note.trim().is_empty());
        // A cleared handoff carries a live-target ref and no fallback; a blocked one carries a fallback and no
        // live-target ref.
        if binding.is_cleared_handoff() {
            assert!(expectation.live_target_handoff_ref.is_some());
            assert!(expectation.metadata_only_exit_ref.is_none());
            assert!(expectation.satisfy_prerequisite_ref.is_none());
        } else {
            assert!(expectation.live_target_handoff_ref.is_none());
            assert!(
                expectation.metadata_only_exit_ref.is_some()
                    || expectation.satisfy_prerequisite_ref.is_some()
            );
        }
    }
}

#[test]
fn every_binding_is_bound_to_release_and_support_evidence() {
    // AC3: the corpus is referenced by release / support evidence, not an ad hoc local sample set.
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.corpus_evidence.all_present(),
            "binding {} is not bound to screenshots/a11y/cli/dashboard",
            binding.binding_id
        );
        assert_eq!(
            binding.corpus_evidence.cli_support_export_ref,
            M5_HISTORICAL_EVIDENCE_DRILL_ARTIFACT_REF
        );
        assert_eq!(
            binding.corpus_evidence.health_dashboard_ref,
            M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_REF
        );
    }
}

#[test]
fn every_binding_joins_provenance_back_to_source_descriptor() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            !binding
                .provenance_join
                .source_snapshot_descriptor_ref
                .trim()
                .is_empty(),
            "binding {} provenance is unjoined to a source descriptor",
            binding.binding_id
        );
        assert!(binding.provenance_join.all_present());
    }
}

#[test]
fn accessibility_state_is_discoverable_for_every_binding() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.accessibility_state_discoverable(),
            "binding {} is not keyboard/screen-reader discoverable",
            binding.binding_id
        );
    }
}

#[test]
fn support_and_export_bindings_point_at_canonical_contracts() {
    let packet = seed();
    for binding in &packet.drill_bindings {
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
fn disclosure_resolver_matches_drill() {
    let preserved = resolve_drill_disclosure(DrillScenario::PreservedLiveTargetHandoff);
    assert!(preserved.offers_open_live_target);
    assert!(preserved.requires_live_target_handoff_ref);
    assert!(!preserved.requires_metadata_only_exit_ref);
    assert!(preserved.expects_content_available);
    assert_eq!(
        preserved.expected_handoff_outcome,
        HandoffOutcome::HandoffCleared
    );

    let missing = resolve_drill_disclosure(DrillScenario::MissingLiveTarget);
    assert!(!missing.offers_open_live_target);
    assert!(missing.requires_metadata_only_exit_ref);
    assert_eq!(missing.expected_blocker, DrillBlocker::MissingTarget);

    let expired = resolve_drill_disclosure(DrillScenario::ExpiredSnapshotMetadataOnlyFallback);
    assert!(!expired.expects_content_available);
    assert_eq!(
        expired.expected_handoff_outcome,
        HandoffOutcome::BlockedByPolicy
    );

    let stale = resolve_drill_disclosure(DrillScenario::StaleImportedEvidence);
    assert!(stale.requires_satisfy_prerequisite_ref);
    assert_eq!(stale.expected_blocker, DrillBlocker::TrustBlock);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.binding_id == "hed-retirement-shell")
        .unwrap();
    packet.drill_bindings[target]
        .non_live_grammar
        .historical_role_word = "capture_time_attribution".to_owned();
    assert!(violations_of(&packet).contains(&"grammar_drift_across_surfaces"));
}

#[test]
fn historical_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .non_live_grammar
        .historical_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"historical_role_word_outside_vocabulary"));
}

#[test]
fn dropped_mutation_blocked_posture_on_gate_role_is_rejected() {
    let mut packet = seed();
    // hed-runbook-runbook carries the snapshot_labeling gate role, which must always keep a real
    // mutation-blocked posture.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.binding_id == "hed-runbook-runbook")
        .unwrap();
    packet.drill_bindings[target]
        .non_live_grammar
        .mutation_blocked_posture_word = "editable".to_owned();
    assert!(violations_of(&packet).contains(&"mutation_blocked_posture_missing_for_gate_role"));
}

#[test]
fn content_presence_mismatch_is_rejected() {
    let mut packet = seed();
    // Flip the content flag on a preserved (handoff-clearing) drill, which must keep its content available.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.drill == DrillScenario::PreservedLiveTargetHandoff)
        .unwrap();
    packet.drill_bindings[target].content_available = false;
    assert!(violations_of(&packet).contains(&"content_presence_mismatch"));
}

#[test]
fn evidence_state_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.drill == DrillScenario::MissingLiveTarget)
        .unwrap();
    packet.drill_bindings[target].evidence_state =
        HistoricalReferenceDrillState::PreservedLiveTargetJoinable;
    assert!(violations_of(&packet).contains(&"evidence_state_mismatch"));
}

#[test]
fn handoff_outcome_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.drill == DrillScenario::MissingLiveTarget)
        .unwrap();
    packet.drill_bindings[target].expected_handoff_outcome = HandoffOutcome::HandoffCleared;
    assert!(violations_of(&packet).contains(&"handoff_outcome_mismatch"));
}

#[test]
fn parity_state_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.drill == DrillScenario::PreservedLiveTargetHandoff)
        .unwrap();
    packet.drill_bindings[target].parity_state = DrillParity::NonLiveBoundaryDisclosed;
    assert!(violations_of(&packet).contains(&"parity_state_mismatch"));
}

#[test]
fn missing_drill_label_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].drill_label = String::new();
    assert!(violations_of(&packet).contains(&"drill_label_missing"));
}

#[test]
fn non_live_boundary_not_called_out_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].non_live_boundary_explicitly_called_out = false;
    assert!(violations_of(&packet).contains(&"non_live_boundary_not_called_out"));
}

#[test]
fn open_live_action_drill_mismatch_is_rejected() {
    let mut packet = seed();
    // Add an open-current-live-object action to a blocked drill, which must not offer it.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| !b.is_cleared_handoff())
        .unwrap();
    packet.drill_bindings[target]
        .allowed_actions
        .push(DrillCorpusAction::OpenCurrentLiveObject);
    assert!(violations_of(&packet).contains(&"open_live_action_drill_mismatch"));
}

#[test]
fn missing_base_action_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .allowed_actions
        .retain(|a| *a != DrillCorpusAction::ExportDrillEvidence);
    assert!(violations_of(&packet).contains(&"base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].accessibility_routes =
        vec![M5HistoricalReferenceAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.drill_bindings.retain(|b| {
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
fn missing_corpus_evidence_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].corpus_evidence.screenshot_ref = String::new();
    assert!(violations_of(&packet).contains(&"corpus_evidence_bindings_missing"));
}

#[test]
fn missing_canonical_reference_on_export_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| consumer_must_reference_canonical(b.consumer))
        .unwrap();
    packet.drill_bindings[target].source_contract_refs =
        vec![M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn missing_source_descriptor_join_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .provenance_join
        .source_snapshot_descriptor_ref = String::new();
    assert!(violations_of(&packet).contains(&"source_descriptor_join_missing"));
}

#[test]
fn incomplete_provenance_join_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .provenance_join
        .provenance_lineage_ref = String::new();
    assert!(violations_of(&packet).contains(&"provenance_join_incomplete"));
}

#[test]
fn satisfy_prerequisite_ref_mismatch_is_rejected() {
    let mut packet = seed();
    // Strip the satisfy-prerequisite ref off a needs-prerequisite drill, which must carry it.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.drill == DrillScenario::StaleImportedEvidence)
        .unwrap();
    packet.drill_bindings[target]
        .handoff_expectation
        .satisfy_prerequisite_ref = None;
    assert!(violations_of(&packet).contains(&"satisfy_prerequisite_ref_mismatch"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [crate::GuardrailCase<DrillCorpusBinding>; 5] = [
        (
            |b| b.looks_live_by_omission = true,
            "looks_live_by_omission",
        ),
        (
            |b| b.reopens_live_target_without_validating_identity_trust_route_and_authority = true,
            "reopens_live_target_without_validating_identity_trust_route_and_authority",
        ),
        (
            |b| b.dead_links_expired_or_removed_artifact = true,
            "dead_links_expired_or_removed_artifact",
        ),
        (
            |b| b.non_live_evidence_unjoined_to_capture_context = true,
            "non_live_evidence_unjoined_to_capture_context",
        ),
        (
            |b| b.presents_as_current_or_reopens_through_ambiguous_route = true,
            "presents_as_current_or_reopens_through_ambiguous_route",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.drill_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_class_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .drill_bindings
        .retain(|b| b.object_class != M5HistoricalReferenceObject::ReviewIncidentSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn drill_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every retired-line-reopen drill, leaving that drill uncovered.
    packet
        .drill_bindings
        .retain(|b| b.drill != DrillScenario::RetiredLineReopen);
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"drill_coverage_missing"));
}

#[test]
fn exact_blocker_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop every expired-snapshot drill, leaving the expired_snapshot blocker uncovered.
    packet
        .drill_bindings
        .retain(|b| b.drill != DrillScenario::ExpiredSnapshotMetadataOnlyFallback);
    assert!(violations_of(&packet).contains(&"exact_blocker_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_HISTORICAL_EVIDENCE_DRILL_DOC_REF);
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
    assert_eq!(lines.len(), 1 + packet.drill_bindings.len());
    assert!(lines[0].starts_with(
        "object_class,consumer,drill,evidence_state,handoff_outcome,expected_blocker,content_available,fixture_id"
    ));
}

#[test]
fn csv_preserves_blocker_and_drill_vocabulary() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    assert!(csv.contains(",missing_target,"));
    assert!(csv.contains(",trust_block,"));
    assert!(csv.contains(",route_unavailable,"));
    assert!(csv.contains(",expired_snapshot,"));
    assert!(csv.contains(",imported_offline_evidence_only,"));
    assert!(csv.contains(",none_cleared,"));
}

#[test]
fn markdown_summary_lists_every_binding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.drill_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn health_dashboard_surfaces_the_corpus() {
    let packet = seed();
    let dashboard = packet.render_health_dashboard();
    let value: serde_json::Value = serde_json::from_str(&dashboard).expect("dashboard parses");
    assert_eq!(
        value["record_kind"],
        serde_json::json!(M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_RECORD_KIND)
    );
    assert_eq!(
        value["support_export_ref"],
        serde_json::json!(M5_HISTORICAL_EVIDENCE_DRILL_ARTIFACT_REF)
    );
    assert_eq!(value["drills"].as_array().unwrap().len(), 6);
    assert_eq!(
        value["historical_reference_states"]
            .as_array()
            .unwrap()
            .len(),
        6
    );
    assert_eq!(value["handoff_outcomes"].as_array().unwrap().len(), 4);
    assert_eq!(value["exact_blockers"].as_array().unwrap().len(), 6);
    assert_eq!(value["fixture_families"].as_array().unwrap().len(), 5);
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_historical_evidence_drill_corpus_export()
        .expect("checked M5 historical-evidence drill-corpus export validates");
    assert_eq!(from_disk.packet_id, M5_HISTORICAL_EVIDENCE_DRILL_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_health_dashboard_matches_render() {
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../dashboards/m5-historical-evidence-drill-health.json"
    ));
    let on_disk_value: serde_json::Value =
        serde_json::from_str(on_disk).expect("checked dashboard parses");
    let rendered_value: serde_json::Value =
        serde_json::from_str(&seed().render_health_dashboard()).expect("rendered dashboard parses");
    assert_eq!(
        on_disk_value, rendered_value,
        "checked health dashboard drifted from the render"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let missing = seeded_m5_historical_evidence_drill_corpus_missing_target_narrowed();
    assert!(
        missing.validate().is_empty(),
        "{:?}",
        violations_of(&missing)
    );
    assert_eq!(missing.drill_bindings.len(), 15);

    let expired = seeded_m5_historical_evidence_drill_corpus_expired_snapshot_narrowed();
    assert!(
        expired.validate().is_empty(),
        "{:?}",
        violations_of(&expired)
    );
    assert_eq!(expired.drill_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let missing: M5HistoricalEvidenceDrillCorpusPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-historical-evidence-drills/missing_target_narrowed.json"
        )))
        .expect("missing-target fixture parses");
    assert!(missing.validate().is_empty());
    assert_eq!(
        missing,
        seeded_m5_historical_evidence_drill_corpus_missing_target_narrowed()
    );

    let expired: M5HistoricalEvidenceDrillCorpusPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m5-historical-evidence-drills/expired_snapshot_narrowed.json"
        )))
        .expect("expired-snapshot fixture parses");
    assert!(expired.validate().is_empty());
    assert_eq!(
        expired,
        seeded_m5_historical_evidence_drill_corpus_expired_snapshot_narrowed()
    );
}
