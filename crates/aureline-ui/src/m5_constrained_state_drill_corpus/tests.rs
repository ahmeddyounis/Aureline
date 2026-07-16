use super::*;

use crate::m5_write_review_sheet_fallback_paths::{CheckpointUndoClass, WriteReviewFallbackAction};

fn seed() -> M5ConstrainedStateDrillCorpusPacket {
    seeded_m5_constrained_state_drill_corpus()
}

fn violations_of(packet: &M5ConstrainedStateDrillCorpusPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_CONSTRAINED_STATE_DRILL_PACKET_ID);
    assert_eq!(packet.record_kind, M5_CONSTRAINED_STATE_DRILL_RECORD_KIND);
    assert_eq!(packet.drill_bindings.len(), 15);
}

#[test]
fn every_object_class_is_seeded_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<M5ConstrainedFileStateConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.drill_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 6, "all six object classes seeded");
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
    for consumer in M5ConstrainedFileStateConsumerSurface::ALL {
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
fn ac1_every_state_class_plus_five_mixed_state_combinations() {
    // AC1: the fixture corpus covers at least one example of every supported state class plus five mixed-state
    // combinations.
    let packet = seed();
    let primaries: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.object_class)
        .collect();
    assert_eq!(primaries.len(), 6, "every state class covered as a primary");

    let combos: BTreeSet<(M5ConstrainedFileStateObject, M5ConstrainedFileStateObject)> = packet
        .drill_bindings
        .iter()
        .filter_map(|b| b.co_applicable_object_class.map(|co| (b.object_class, co)))
        .collect();
    assert!(
        combos.len() >= 5,
        "only {} distinct mixed-state combinations",
        combos.len()
    );
    // The five named mixed combos are present.
    use M5ConstrainedFileStateObject::*;
    for combo in [
        (ReadOnly, Generated),
        (Generated, PolicyLocked),
        (PolicyLocked, Managed),
        (Projection, CapturedSnapshot),
        (Managed, CapturedSnapshot),
    ] {
        assert!(combos.contains(&combo), "mixed combo {combo:?} missing");
    }
}

#[test]
fn ac2_every_blocked_write_reason_and_fallback_is_distinguishable() {
    // AC2: drills catch lossy fallback, hidden second-state, and cross-surface disagreement; each binding names an
    // exact blocked-write reason and reviewed fallback path.
    let packet = seed();
    let reasons: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.blocked_write_reason)
        .collect();
    for reason in BlockedWriteReason::ALL {
        assert!(
            reasons.contains(&reason),
            "blocked-write reason {} not exercised",
            reason.as_str()
        );
    }
    let fallbacks: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.chosen_fallback_path)
        .collect();
    for fallback in WriteReviewFallbackAction::ALL {
        assert!(
            fallbacks.contains(&fallback),
            "fallback path {} not exercised",
            fallback.as_str()
        );
    }
}

#[test]
fn ac3_support_export_replays_denial_and_fallback() {
    // AC3: the seeded support / export packet can replay a constrained write denial and chosen fallback path.
    let packet = seed();
    for binding in &packet.drill_bindings {
        let expectation = &binding.denial_expectation;
        assert_eq!(
            expectation.blocked_write_reason,
            binding.blocked_write_reason
        );
        assert_eq!(
            expectation.chosen_fallback_path,
            binding.chosen_fallback_path
        );
        assert_eq!(
            expectation.required_write_disposition,
            binding.write_disposition
        );
        assert_eq!(
            expectation.checkpoint_undo_class,
            binding.checkpoint_undo_class
        );
        assert!(
            expectation.reviewed_fallback_ref.is_some(),
            "binding {} cannot replay its reviewed fallback",
            binding.binding_id
        );
        assert!(!expectation.denial_note.trim().is_empty());
    }
}

#[test]
fn every_binding_reason_fallback_and_disposition_are_pure_functions_of_class() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.blocked_reason_matches_class(),
            "blocked-reason/class drift on {}",
            binding.binding_id
        );
        assert!(
            binding.fallback_matches_reason(),
            "fallback/reason drift on {}",
            binding.binding_id
        );
        assert!(
            binding.disposition_matches_fallback(),
            "disposition/fallback drift on {}",
            binding.binding_id
        );
        assert!(
            binding.checkpoint_matches_fallback(),
            "checkpoint/fallback drift on {}",
            binding.binding_id
        );
        let disclosure = binding.disclosure();
        assert_eq!(binding.object_class, disclosure.primary_object_class);
        assert_eq!(
            binding.co_applicable_object_class,
            disclosure.co_applicable_object_class
        );
        assert_eq!(binding.parity_state, disclosure.parity);
        assert!(!disclosure.offers_direct_write);
    }
}

#[test]
fn six_fixture_families_are_seeded() {
    let packet = seed();
    let fixtures: BTreeSet<_> = packet
        .drill_bindings
        .iter()
        .map(|b| b.fixture_id.as_str())
        .collect();
    for family in [
        "read-only-alias-path",
        "generated-derived-artifact",
        "policy-locked-managed-mirror",
        "projection-virtual-view",
        "managed-external-source",
        "captured-workspace-snapshot",
    ] {
        assert!(fixtures.contains(family), "fixture family {family} missing");
    }
    assert_eq!(fixtures.len(), 6);
}

#[test]
fn same_fixture_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut fixture_grammar: BTreeMap<&str, &ConstrainedStateGrammar> = BTreeMap::new();
    for binding in &packet.drill_bindings {
        match fixture_grammar.get(binding.fixture_id.as_str()) {
            None => {
                fixture_grammar.insert(binding.fixture_id.as_str(), &binding.constrained_grammar);
            }
            Some(existing) => assert_eq!(
                **existing, binding.constrained_grammar,
                "grammar drift on {}",
                binding.fixture_id
            ),
        }
    }
    assert_eq!(fixture_grammar.len(), 6);
}

#[test]
fn every_state_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.constrained_grammar.state_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.constrained_grammar.state_role_word,
            binding.binding_id
        );
        assert!(binding.constrained_grammar.all_present());
        assert!(binding
            .constrained_grammar
            .write_disposition_constrained_satisfied());
        assert!(binding
            .constrained_grammar
            .canonical_source_and_write_target_present());
    }
}

#[test]
fn actions_are_closed_and_no_direct_write_leaks() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(binding.has_base_actions());
        assert!(binding.action_set_is_closed());
        assert!(binding.reviewed_fallback_action_present());
        // No direct-write / save / apply / sync action exists in the closed action enum.
        assert!(!binding.allowed_actions.iter().any(|a| {
            let token = a.as_str();
            token.contains("write_in_place")
                || token.contains("save")
                || token.contains("apply")
                || token.contains("sync")
                || token.contains("direct_write")
        }));
    }
}

#[test]
fn mixed_state_bindings_keep_both_facets_visible() {
    let packet = seed();
    let mut saw_mixed = false;
    for binding in &packet.drill_bindings {
        assert!(
            binding.mixed_state_facets_consistent(),
            "mixed-state facets inconsistent on {}",
            binding.binding_id
        );
        if binding.is_mixed_state {
            saw_mixed = true;
            assert!(binding.co_applicable_object_class.is_some());
            assert_eq!(
                binding.parity_state,
                DrillParity::MixedStateBothFacetsVisible
            );
            assert!(binding.both_state_facets_visible_when_mixed);
            assert!(binding.denial_expectation.co_applicable_state_ref.is_some());
        } else {
            assert!(binding.co_applicable_object_class.is_none());
            assert_eq!(
                binding.parity_state,
                DrillParity::SingleStateDirectWriteDenied
            );
            assert!(binding.denial_expectation.co_applicable_state_ref.is_none());
        }
    }
    assert!(saw_mixed, "no mixed-state drill present");
}

#[test]
fn every_binding_renders_canonical_source_and_write_target() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.renders_canonical_source_and_write_target(),
            "binding {} leaves canonical source / write target unstated",
            binding.binding_id
        );
        assert!(binding.canonical_source_join.all_present());
    }
}

#[test]
fn every_binding_is_bound_to_release_and_support_evidence() {
    let packet = seed();
    for binding in &packet.drill_bindings {
        assert!(
            binding.corpus_evidence.all_present(),
            "binding {} is not bound to screenshots/a11y/cli/dashboard",
            binding.binding_id
        );
        assert_eq!(
            binding.corpus_evidence.cli_support_export_ref,
            M5_CONSTRAINED_STATE_DRILL_ARTIFACT_REF
        );
        assert_eq!(
            binding.corpus_evidence.health_dashboard_ref,
            M5_CONSTRAINED_STATE_DRILL_DASHBOARD_REF
        );
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
    use M5ConstrainedFileStateObject::*;

    let alias = resolve_drill_disclosure(DrillScenario::SymlinkAliasSaveDenied);
    assert_eq!(alias.primary_object_class, ReadOnly);
    assert!(alias.co_applicable_object_class.is_none());
    assert_eq!(
        alias.blocked_write_reason,
        BlockedWriteReason::ReadOnlyPathNotDirectlyWritable
    );
    assert_eq!(
        alias.chosen_fallback_path,
        WriteReviewFallbackAction::DuplicateToEditableCopy
    );
    assert_eq!(alias.parity, DrillParity::SingleStateDirectWriteDenied);
    assert!(!alias.offers_direct_write);

    let mirror = resolve_drill_disclosure(DrillScenario::PolicyLockedManagedMirrorDenied);
    assert_eq!(mirror.primary_object_class, PolicyLocked);
    assert_eq!(mirror.co_applicable_object_class, Some(Managed));
    assert!(mirror.is_mixed_state);
    assert_eq!(mirror.parity, DrillParity::MixedStateBothFacetsVisible);
    assert_eq!(
        mirror.chosen_fallback_path,
        WriteReviewFallbackAction::RequestApproval
    );

    let projection = resolve_drill_disclosure(DrillScenario::ProjectionExportDenied);
    assert_eq!(
        projection.co_applicable_object_class,
        Some(CapturedSnapshot)
    );
    assert_eq!(
        projection.chosen_fallback_path,
        WriteReviewFallbackAction::CreateOverlayPatch
    );
    assert_eq!(
        projection.checkpoint_undo_class,
        CheckpointUndoClass::OverlayPatchRevertible
    );
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.binding_id == "csd-ro-editor")
        .unwrap();
    packet.drill_bindings[target]
        .constrained_grammar
        .state_class_label_word = "totally_different".to_owned();
    assert!(violations_of(&packet).contains(&"grammar_drift_across_surfaces"));
}

#[test]
fn state_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].constrained_grammar.state_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"state_role_word_outside_vocabulary"));
}

#[test]
fn unconstrained_write_disposition_on_gate_role_is_rejected() {
    let mut packet = seed();
    // csd-ro-tab carries the state_badge_classification gate role, which must keep a write-constrained disposition.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.binding_id == "csd-ro-tab")
        .unwrap();
    packet.drill_bindings[target]
        .constrained_grammar
        .write_disposition_word = "directly_writable".to_owned();
    assert!(violations_of(&packet).contains(&"write_disposition_unconstrained_for_gate_role"));
}

#[test]
fn blocked_reason_class_mismatch_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].blocked_write_reason = BlockedWriteReason::ManagedSourceRequiresDetach;
    assert!(violations_of(&packet).contains(&"blocked_reason_class_mismatch"));
}

#[test]
fn fallback_reason_mismatch_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].chosen_fallback_path =
        WriteReviewFallbackAction::DetachFromManagedSource;
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"fallback_reason_mismatch"));
}

#[test]
fn write_disposition_fallback_mismatch_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].write_disposition =
        M5ConstrainedFileStateWriteDisposition::ApprovalGated;
    assert!(violations_of(&packet).contains(&"write_disposition_fallback_mismatch"));
}

#[test]
fn parity_state_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| !b.is_mixed_state)
        .unwrap();
    packet.drill_bindings[target].parity_state = DrillParity::MixedStateBothFacetsVisible;
    assert!(violations_of(&packet).contains(&"parity_state_mismatch"));
}

#[test]
fn hidden_second_state_is_rejected() {
    let mut packet = seed();
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.is_mixed_state)
        .unwrap();
    packet.drill_bindings[target].both_state_facets_visible_when_mixed = false;
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"second_state_hidden"));
}

#[test]
fn missing_drill_label_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].drill_label = String::new();
    assert!(violations_of(&packet).contains(&"drill_label_missing"));
}

#[test]
fn constrained_state_not_classified_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].constrained_state_explicitly_classified = false;
    assert!(violations_of(&packet).contains(&"constrained_state_not_classified"));
}

#[test]
fn missing_reviewed_fallback_action_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .allowed_actions
        .retain(|a| *a != DrillDenialAction::OpenReviewedFallbackPath);
    assert!(violations_of(&packet).contains(&"reviewed_fallback_action_missing"));
}

#[test]
fn missing_base_action_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .allowed_actions
        .retain(|a| *a != DrillDenialAction::ExportDenialEvidence);
    assert!(violations_of(&packet).contains(&"base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0].accessibility_routes =
        vec![M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.drill_bindings.retain(|b| {
        if b.object_class == M5ConstrainedFileStateObject::Projection {
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
        vec![M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned()];
    assert!(violations_of(&packet).contains(&"support_export_reference_missing"));
}

#[test]
fn incomplete_canonical_source_join_is_rejected() {
    let mut packet = seed();
    packet.drill_bindings[0]
        .canonical_source_join
        .exact_write_target_ref = String::new();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"canonical_source_join_incomplete"));
}

#[test]
fn co_applicable_state_ref_mismatch_is_rejected() {
    let mut packet = seed();
    // Strip the co-applicable-state ref off a mixed-state drill, which must carry it.
    let target = packet
        .drill_bindings
        .iter()
        .position(|b| b.is_mixed_state)
        .unwrap();
    packet.drill_bindings[target]
        .denial_expectation
        .co_applicable_state_ref = None;
    assert!(violations_of(&packet).contains(&"co_applicable_state_ref_mismatch"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut DrillCorpusBinding), &str); 4] = [
        (
            |b| b.lets_one_constrained_state_class_hide_another = true,
            "lets_one_constrained_state_class_hide_another",
        ),
        (
            |b| b.silently_falls_back_to_lossy_direct_write = true,
            "silently_falls_back_to_lossy_direct_write",
        ),
        (
            |b| b.gives_ai_automation_import_or_repair_a_hidden_bypass = true,
            "gives_ai_automation_import_or_repair_a_hidden_bypass",
        ),
        (
            |b| b.presents_as_directly_writable_or_hides_recovery_path = true,
            "presents_as_directly_writable_or_hides_recovery_path",
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
        .retain(|b| b.object_class != M5ConstrainedFileStateObject::CapturedSnapshot);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn drill_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .drill_bindings
        .retain(|b| b.drill != DrillScenario::ProjectionExportDenied);
    assert!(violations_of(&packet).contains(&"drill_coverage_missing"));
}

#[test]
fn mixed_state_combo_coverage_gap_is_rejected() {
    let mut packet = seed();
    // Drop enough mixed-state drills to fall below five distinct combinations.
    packet.drill_bindings.retain(|b| {
        !matches!(
            b.drill,
            DrillScenario::ProjectionExportDenied
                | DrillScenario::ManagedCapturedSnapshotRestoreDenied
        )
    });
    assert!(violations_of(&packet).contains(&"mixed_state_combo_coverage_insufficient"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_CONSTRAINED_STATE_DRILL_DOC_REF);
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
        "object_class,co_applicable_class,consumer,drill,blocked_write_reason,chosen_fallback_path,write_disposition,parity_state,fixture_id"
    ));
}

#[test]
fn csv_preserves_reason_and_fallback_vocabulary() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    assert!(csv.contains(",read_only_path_not_directly_writable,"));
    assert!(csv.contains(",generated_artifact_regenerate_only,"));
    assert!(csv.contains(",policy_lock_requires_approval,"));
    assert!(csv.contains(",managed_source_requires_detach,"));
    assert!(csv.contains(",projection_requires_overlay_or_detach,"));
    assert!(csv.contains(",captured_snapshot_restore_only,"));
    assert!(csv.contains(",duplicate_to_editable_copy,"));
    assert!(csv.contains(",create_overlay_patch,"));
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
        serde_json::json!(M5_CONSTRAINED_STATE_DRILL_DASHBOARD_RECORD_KIND)
    );
    assert_eq!(
        value["support_export_ref"],
        serde_json::json!(M5_CONSTRAINED_STATE_DRILL_ARTIFACT_REF)
    );
    assert_eq!(value["drills"].as_array().unwrap().len(), 9);
    assert_eq!(value["blocked_write_reasons"].as_array().unwrap().len(), 6);
    assert_eq!(value["fallback_paths"].as_array().unwrap().len(), 5);
    assert_eq!(
        value["mixed_state_combinations"].as_array().unwrap().len(),
        5
    );
    assert_eq!(value["fixture_families"].as_array().unwrap().len(), 6);
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_constrained_state_drill_corpus_export()
        .expect("checked M5 constrained-state drill-corpus export validates");
    assert_eq!(from_disk.packet_id, M5_CONSTRAINED_STATE_DRILL_PACKET_ID);
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
        "/../../dashboards/m5-constrained-state-drill-health.json"
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
    let mixed = seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed();
    assert!(mixed.validate().is_empty(), "{:?}", violations_of(&mixed));
    assert_eq!(mixed.drill_bindings.len(), 15);

    let read_only_generated =
        seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed();
    assert!(
        read_only_generated.validate().is_empty(),
        "{:?}",
        violations_of(&read_only_generated)
    );
    assert_eq!(read_only_generated.drill_bindings.len(), 15);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let mixed: M5ConstrainedStateDrillCorpusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-constrained-state-drills/mixed_state_narrowed.json"
    )))
    .expect("mixed-state fixture parses");
    assert!(mixed.validate().is_empty());
    assert_eq!(
        mixed,
        seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed()
    );

    let read_only_generated: M5ConstrainedStateDrillCorpusPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/editor/m5-constrained-state-drills/read_only_generated_narrowed.json"
        )))
        .expect("read-only + generated fixture parses");
    assert!(read_only_generated.validate().is_empty());
    assert_eq!(
        read_only_generated,
        seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed()
    );
}
