use super::*;

fn seed() -> M5FileStateBadgeGroupConsumersPacket {
    seeded_m5_file_state_badge_group_consumers()
}

fn violations_of(packet: &M5FileStateBadgeGroupConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 19);
}

#[test]
fn every_object_class_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<M5ConstrainedFileStateConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 6, "all six object classes adopted");
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
    for consumer in M5ConstrainedFileStateConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let postures: BTreeSet<_> = packet.consumer_bindings.iter().map(|b| b.posture).collect();
    for posture in BadgeRenderPosture::ALL {
        assert!(
            postures.contains(&posture),
            "posture {} missing",
            posture.as_str()
        );
    }
}

#[test]
fn one_object_spans_editor_diff_palette_and_status() {
    // AC1: at least one object renders the same vocabulary on an editor surface, a diff / review surface, a
    // palette / detail path, and a status consumer.
    let packet = seed();
    let mut per_object: BTreeMap<M5ConstrainedFileStateObject, BTreeSet<&str>> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        let family = match binding.consumer {
            M5ConstrainedFileStateConsumerSurface::EditorBanner => Some("editor"),
            M5ConstrainedFileStateConsumerSurface::DiffReviewHeader => Some("diff_review"),
            M5ConstrainedFileStateConsumerSurface::CommandPalette => Some("palette"),
            M5ConstrainedFileStateConsumerSurface::StatusBar => Some("status"),
            _ => None,
        };
        if let Some(family) = family {
            per_object
                .entry(binding.object_class)
                .or_default()
                .insert(family);
        }
    }
    assert!(
        per_object.values().any(|families| families.len() == 4),
        "no single object covers editor + diff/review + palette + status"
    );
}

#[test]
fn multi_state_objects_keep_every_state_visible() {
    // AC3: multi-state objects keep both facts visible (Generated + Policy locked, Managed + Captured
    // snapshot).
    let packet = seed();
    let multi: Vec<_> = packet
        .consumer_bindings
        .iter()
        .filter(|b| b.is_multi_state())
        .collect();
    assert!(!multi.is_empty(), "at least one multi-state binding");
    for binding in &multi {
        assert!(binding.multi_state_facets_consistent());
        assert_eq!(
            binding.co_applicable_states.len(),
            binding.badge_grammar.co_applicable_state_labels.len()
        );
    }
    // Generated + Policy locked and Managed + Captured snapshot both present.
    let has_generated_plus_policy = packet.consumer_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Generated
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::PolicyLocked)
    });
    let has_managed_plus_snapshot = packet.consumer_bindings.iter().any(|b| {
        b.object_class == M5ConstrainedFileStateObject::Managed
            && b.co_applicable_states
                .contains(&M5ConstrainedFileStateObject::CapturedSnapshot)
    });
    assert!(
        has_generated_plus_policy,
        "Generated + Policy locked present"
    );
    assert!(
        has_managed_plus_snapshot,
        "Managed + Captured snapshot present"
    );
}

#[test]
fn hidden_multi_state_facet_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_multi_state())
        .unwrap();
    // Clear the grammar labels while keeping the co-applicable states: one badge would hide the other.
    packet.consumer_bindings[target]
        .badge_grammar
        .co_applicable_state_labels
        .clear();
    // Rewording one surface also trips drift, which is expected; assert the multi-state failure is present.
    assert!(violations_of(&packet).contains(&"multi_state_facet_hidden"));
}

#[test]
fn same_profile_carries_identical_grammar_across_surfaces() {
    let packet = seed();
    let mut profile_grammar: BTreeMap<&str, &FileStateBadgeGroupGrammar> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match profile_grammar.get(binding.object_profile_id.as_str()) {
            None => {
                profile_grammar.insert(binding.object_profile_id.as_str(), &binding.badge_grammar);
            }
            Some(existing) => assert_eq!(
                **existing, binding.badge_grammar,
                "grammar drift on {}",
                binding.object_profile_id
            ),
        }
    }
    assert_eq!(profile_grammar.len(), 6);
}

#[test]
fn every_badge_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.badge_grammar.badge_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.badge_grammar.badge_role_word,
            binding.binding_id
        );
        assert!(binding.badge_grammar.all_present());
        assert!(binding.badge_grammar.write_disposition_satisfied());
    }
}

#[test]
fn actions_are_safe_and_next_step_matches_posture() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(binding.has_safe_base_actions());
        assert!(binding.safe_next_step_action_matches_posture());
        let offers = binding
            .allowed_actions
            .contains(&BadgeGroupAction::OpenSafeNextStep);
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
                BadgeGroupParityState::FacetsDisclosedNarrowed
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
            assert_eq!(binding.parity_state, BadgeGroupParityState::FacetsPreserved);
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.posture,
            BadgeRenderPosture::PaletteAvailabilityGated
        ) {
            assert!(!binding.palette_availability_note.trim().is_empty());
        }
        if matches!(binding.posture, BadgeRenderPosture::ExportRedacted) {
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
    let full = resolve_badge_group_render_disclosure(BadgeRenderPosture::FullBadgeGroup);
    assert!(!full.needs_narrow_note);
    assert!(full.offers_safe_next_step);

    let chip = resolve_badge_group_render_disclosure(BadgeRenderPosture::CompactStatusChip);
    assert_eq!(
        chip.narrow_reason,
        Some(BadgeGroupNarrowReason::CompactedToStatusChip)
    );
    assert!(chip.needs_narrow_note);
    assert!(!chip.offers_safe_next_step);

    let palette =
        resolve_badge_group_render_disclosure(BadgeRenderPosture::PaletteAvailabilityGated);
    assert!(palette.needs_palette_availability_note);
    assert!(!palette.offers_safe_next_step);

    let exported = resolve_badge_group_render_disclosure(BadgeRenderPosture::ExportRedacted);
    assert!(exported.needs_export_detail_note);
    assert!(!exported.offers_safe_next_step);
}

#[test]
fn grammar_drift_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "fsbg-generated-diff")
        .unwrap();
    packet.consumer_bindings[target]
        .badge_grammar
        .badge_role_word = "exact_write_target".to_owned();
    assert!(violations_of(&packet).contains(&"badge_group_vocabulary_drift_across_surfaces"));
}

#[test]
fn badge_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].badge_grammar.badge_role_word = "totally_made_up".to_owned();
    assert!(violations_of(&packet).contains(&"badge_role_word_outside_vocabulary"));
}

#[test]
fn dropped_write_disposition_on_gate_role_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .badge_grammar
        .write_disposition_word = "directly_writable".to_owned();
    assert!(violations_of(&packet).contains(&"write_disposition_missing_for_gate_role"));
}

#[test]
fn directly_writable_guardrail_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .presents_constrained_object_as_directly_writable_or_hides_recovery_path = true;
    assert!(violations_of(&packet)
        .contains(&"presents_constrained_object_as_directly_writable_or_hides_recovery_path"));
}

#[test]
fn safe_next_step_action_mismatch_is_rejected() {
    let mut packet = seed();
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.is_narrowed())
        .unwrap();
    packet.consumer_bindings[target]
        .allowed_actions
        .push(BadgeGroupAction::OpenSafeNextStep);
    assert!(violations_of(&packet).contains(&"safe_next_step_action_posture_mismatch"));
}

#[test]
fn missing_safe_base_action_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0]
        .allowed_actions
        .retain(|a| *a != BadgeGroupAction::CopyReason);
    assert!(violations_of(&packet).contains(&"safe_base_actions_missing"));
}

#[test]
fn undiscoverable_accessibility_state_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].accessibility_routes =
        vec![M5ConstrainedFileStateAccessibilityRoute::HighZoomReflow];
    assert!(violations_of(&packet).contains(&"accessibility_state_undiscoverable"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.object_class == M5ConstrainedFileStateObject::CapturedSnapshot {
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
        vec![M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF.to_owned()];
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
    packet.consumer_bindings[target].narrow_note = Some(BadgeGroupNarrowNote {
        reason: BadgeGroupNarrowReason::CompactedToStatusChip,
        preserved_grammar_note: "x".to_owned(),
        next_action: BadgeGroupNarrowNextAction::OpenFullBadgeGroup,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut FileStateBadgeGroupConsumerBinding), &str); 5] = [
        (
            |b| b.presents_constrained_object_as_directly_writable_or_hides_recovery_path = true,
            "presents_constrained_object_as_directly_writable_or_hides_recovery_path",
        ),
        (
            |b| {
                b.lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write =
                    true
            },
            "lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write",
        ),
        (
            |b| b.gives_ai_automation_import_or_repair_flows_a_hidden_bypass = true,
            "gives_ai_automation_import_or_repair_flows_a_hidden_bypass",
        ),
        (
            |b| b.leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated = true,
            "leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated",
        ),
        (
            |b| b.lets_one_state_class_hide_another_when_both_materially_affect_behavior = true,
            "lets_one_state_class_hide_another_when_both_materially_affect_behavior",
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
        .retain(|b| b.object_class != M5ConstrainedFileStateObject::Projection);
    assert!(violations_of(&packet).contains(&"object_class_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_FILE_STATE_BADGE_GROUP_CONSUMERS_DOC_REF);
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
    assert!(lines[0].starts_with(
        "object_class,co_applicable_states,consumer,posture,badge_role_word,parity_state"
    ));
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
    let from_disk = current_stable_m5_file_state_badge_group_consumers_export()
        .expect("checked M5 file-state badge-group consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_FILE_STATE_BADGE_GROUP_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_file_state_badge_group_consumers_compact_status_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 19);

    let palette = seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed();
    assert!(
        palette.validate().is_empty(),
        "{:?}",
        violations_of(&palette)
    );
    assert_eq!(palette.consumer_bindings.len(), 19);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5FileStateBadgeGroupConsumersPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-file-state-badge-group-consumers/compact_status_narrowed.json"
    )))
        .expect("compact-status fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_file_state_badge_group_consumers_compact_status_narrowed()
    );

    let palette: M5FileStateBadgeGroupConsumersPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/editor/m5-file-state-badge-group-consumers/palette_gated_narrowed.json"
    )))
        .expect("palette-gated fixture parses");
    assert!(palette.validate().is_empty());
    assert_eq!(
        palette,
        seeded_m5_file_state_badge_group_consumers_palette_gated_narrowed()
    );
}
