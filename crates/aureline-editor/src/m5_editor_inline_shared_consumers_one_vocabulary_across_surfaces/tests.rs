use super::*;

fn seed() -> M5EditorInlineSharedConsumersPacket {
    seeded_m5_editor_inline_shared_consumers()
}

fn violations_of(packet: &M5EditorInlineSharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(
        packet.packet_id,
        M5_EDITOR_INLINE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        packet.record_kind,
        M5_EDITOR_INLINE_SHARED_CONSUMERS_RECORD_KIND
    );
}

#[test]
fn every_component_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut component_consumers: BTreeMap<
        M5EditorInlineComponentFamily,
        BTreeSet<M5EditorInlineConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(component_consumers.len(), 8, "all eight components adopted");
    for (component, consumers) in &component_consumers {
        assert!(
            consumers.len() >= 2,
            "component {} only adopted by {} consumers",
            component.as_str(),
            consumers.len()
        );
    }
}

#[test]
fn every_consumer_surface_and_representation_is_exercised() {
    let packet = seed();
    let consumers: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.consumer)
        .collect();
    for consumer in M5EditorInlineConsumerSurface::ALL {
        assert!(
            consumers.contains(&consumer),
            "consumer {} missing",
            consumer.as_str()
        );
    }
    let representations: BTreeSet<_> = packet
        .consumer_bindings
        .iter()
        .map(|b| b.representation)
        .collect();
    for representation in EditorInlineRepresentation::ALL {
        assert!(
            representations.contains(&representation),
            "representation {} missing",
            representation.as_str()
        );
    }
}

#[test]
fn same_object_carries_identical_vocabulary_across_surfaces() {
    let packet = seed();
    let mut object_facets: BTreeMap<&str, &EditorInlineStateFacetValues> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match object_facets.get(binding.inline_object_id.as_str()) {
            None => {
                object_facets.insert(binding.inline_object_id.as_str(), &binding.state_facets);
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "vocabulary drift on {}",
                binding.inline_object_id
            ),
        }
    }
    // The eight objects each fan out to more than one consumer.
    assert_eq!(object_facets.len(), 8);
}

#[test]
fn every_state_word_is_a_frozen_disposition_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.state_facets.state_word_in_vocabulary(),
            "state word `{}` on {} is not a frozen disposition token",
            binding.state_facets.state_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
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
                EditorInlineParityState::FacetsDisclosedNarrowed
            );
            let note = binding
                .narrow_note
                .as_ref()
                .expect("narrowed binding carries a note");
            assert_eq!(Some(note.reason), disclosure.narrow_reason);
            assert_eq!(Some(note.next_action), disclosure.narrow_next_action);
            assert!(!note.preserved_vocabulary_note.trim().is_empty());
            assert!(!note.next_action_label.trim().is_empty());
        } else {
            assert_eq!(
                binding.parity_state,
                EditorInlineParityState::FacetsPreserved
            );
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            EditorInlineRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            EditorInlineRepresentation::ExportedRedacted
        ) {
            assert!(!binding.export_evidence_note.trim().is_empty());
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
fn disclosure_resolver_matches_representation() {
    assert!(
        !resolve_editor_inline_render_disclosure(EditorInlineRepresentation::DesktopFull)
            .needs_narrow_note
    );
    let compact =
        resolve_editor_inline_render_disclosure(EditorInlineRepresentation::CompactNarrowed);
    assert_eq!(
        compact.narrow_reason,
        Some(EditorInlineNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote =
        resolve_editor_inline_render_disclosure(EditorInlineRepresentation::RemoteProjected);
    assert!(remote.needs_remote_source_note);
    let exported =
        resolve_editor_inline_render_disclosure(EditorInlineRepresentation::ExportedRedacted);
    assert!(exported.needs_export_evidence_note);
}

#[test]
fn vocabulary_drift_is_rejected() {
    let mut packet = seed();
    // Find an object with two bindings and reword one surface's state.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| {
            b.inline_object_id == "diag:src/main.rs:88" && b.binding_id == "eb-diag-editor"
        })
        .unwrap();
    packet.consumer_bindings[target].state_facets.state_word = "modified".to_owned();
    assert!(violations_of(&packet).contains(&"vocabulary_drift_across_surfaces"));
}

#[test]
fn state_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].state_facets.state_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"state_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every gutter binding except one, leaving the component with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.component == M5EditorInlineComponentFamily::Gutter {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"inline_component_reuse_unproven"));
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
        vec![M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF.to_owned()];
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
    packet.consumer_bindings[target].narrow_note = Some(EditorInlineNarrowNote {
        reason: EditorInlineNarrowReason::CompactionNarrowed,
        preserved_vocabulary_note: "x".to_owned(),
        next_action: EditorInlineNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    type GuardrailCase = (fn(&mut EditorInlineConsumerBinding), &'static str);

    let cases: [GuardrailCase; 6] = [
        (
            |b| b.encodes_state_by_color_alone = true,
            "state_encoded_by_color_alone",
        ),
        (
            |b| b.lets_anchor_or_evidence_pointer_silently_drift = true,
            "anchor_or_evidence_pointer_drift",
        ),
        (
            |b| b.blurs_outdated_and_resolved_review_state = true,
            "outdated_resolved_blurred",
        ),
        (
            |b| b.presents_inferred_fix_as_exact = true,
            "inferred_fix_shown_as_exact",
        ),
        (
            |b| b.hides_evidence_in_opaque_log = true,
            "evidence_hidden_in_opaque_log",
        ),
        (
            |b| b.rewords_inline_vocabulary_per_surface = true,
            "vocabulary_reworded_per_surface",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.consumer_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn component_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .consumer_bindings
        .retain(|b| b.component != M5EditorInlineComponentFamily::EvidenceTimeline);
    assert!(violations_of(&packet).contains(&"component_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_EDITOR_INLINE_SHARED_CONSUMERS_DOC_REF);
    assert!(violations_of(&packet).contains(&"missing_source_contracts"));
}

#[test]
fn export_json_is_boundary_safe() {
    let json = seed().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("secret"));
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

#[test]
fn csv_has_a_row_per_binding() {
    let packet = seed();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.consumer_bindings.len());
    assert!(lines[0].starts_with("component,consumer,representation,state_word,parity_state"));
}

#[test]
fn markdown_summary_lists_every_object() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_editor_inline_shared_consumers_export()
        .expect("checked M5 editor-inline shared-consumer export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_EDITOR_INLINE_SHARED_CONSUMERS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_editor_inline_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 21);

    let exported = seeded_m5_editor_inline_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 21);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5EditorInlineSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-editor-inline-shared-consumers/compact_remote_narrowed.json"
    )))
    .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_editor_inline_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5EditorInlineSharedConsumersPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-editor-inline-shared-consumers/exported_redaction_narrowed.json"
        )))
        .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_editor_inline_shared_consumers_exported_redaction_narrowed()
    );
}
