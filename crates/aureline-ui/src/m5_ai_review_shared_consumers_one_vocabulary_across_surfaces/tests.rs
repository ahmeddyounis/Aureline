use super::*;

fn seed() -> M5AiReviewSharedConsumersPacket {
    seeded_m5_ai_review_shared_consumers()
}

fn violations_of(packet: &M5AiReviewSharedConsumersPacket) -> Vec<&'static str> {
    packet.validate().iter().map(|v| v.as_str()).collect()
}

#[test]
fn seed_validates_clean() {
    let packet = seed();
    assert!(packet.validate().is_empty(), "{:?}", violations_of(&packet));
    assert_eq!(packet.packet_id, M5_AI_REVIEW_SHARED_CONSUMERS_PACKET_ID);
    assert_eq!(
        packet.record_kind,
        M5_AI_REVIEW_SHARED_CONSUMERS_RECORD_KIND
    );
    assert_eq!(packet.consumer_bindings.len(), 12);
}

#[test]
fn every_object_is_adopted_by_two_or_more_consumers() {
    let packet = seed();
    let mut object_consumers: BTreeMap<
        M5AiReviewAssistObject,
        BTreeSet<M5AiReviewAssistConsumerSurface>,
    > = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        object_consumers
            .entry(binding.object)
            .or_default()
            .insert(binding.consumer);
    }
    assert_eq!(object_consumers.len(), 4, "all four objects adopted");
    for (object, consumers) in &object_consumers {
        assert!(
            consumers.len() >= 2,
            "object {} only adopted by {} consumers",
            object.as_str(),
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
    for consumer in M5AiReviewAssistConsumerSurface::ALL {
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
    for representation in AiReviewRepresentation::ALL {
        assert!(
            representations.contains(&representation),
            "representation {} missing",
            representation.as_str()
        );
    }
}

#[test]
fn same_finding_carries_identical_vocabulary_across_surfaces() {
    let packet = seed();
    let mut finding_facets: BTreeMap<&str, &AiReviewStateFacetValues> = BTreeMap::new();
    for binding in &packet.consumer_bindings {
        match finding_facets.get(binding.finding_id.as_str()) {
            None => {
                finding_facets.insert(binding.finding_id.as_str(), &binding.state_facets);
            }
            Some(existing) => assert_eq!(
                **existing, binding.state_facets,
                "vocabulary drift on {}",
                binding.finding_id
            ),
        }
    }
    // The four findings each fan out to more than one consumer.
    assert_eq!(finding_facets.len(), 4);
}

#[test]
fn every_ai_review_role_word_is_a_frozen_role_token() {
    let packet = seed();
    for binding in &packet.consumer_bindings {
        assert!(
            binding.state_facets.ai_review_role_word_in_vocabulary(),
            "role word `{}` on {} is not a frozen role token",
            binding.state_facets.ai_review_role_word,
            binding.binding_id
        );
        assert!(binding.state_facets.all_present());
        assert!(binding.state_facets.finding_lifecycle_satisfied());
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
                AiReviewParityState::FacetsDisclosedNarrowed
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
            assert_eq!(binding.parity_state, AiReviewParityState::FacetsPreserved);
            assert!(binding.narrow_note.is_none());
        }
        if matches!(
            binding.representation,
            AiReviewRepresentation::RemoteProjected
        ) {
            assert!(!binding.remote_source_note.trim().is_empty());
        }
        if matches!(
            binding.representation,
            AiReviewRepresentation::ExportedRedacted
        ) {
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
fn disclosure_resolver_matches_representation() {
    assert!(
        !resolve_ai_review_render_disclosure(AiReviewRepresentation::DesktopFull).needs_narrow_note
    );
    let compact = resolve_ai_review_render_disclosure(AiReviewRepresentation::CompactNarrowed);
    assert_eq!(
        compact.narrow_reason,
        Some(AiReviewNarrowReason::CompactionNarrowed)
    );
    assert!(compact.needs_narrow_note);
    assert!(!compact.needs_remote_source_note);
    let remote = resolve_ai_review_render_disclosure(AiReviewRepresentation::RemoteProjected);
    assert!(remote.needs_remote_source_note);
    let exported = resolve_ai_review_render_disclosure(AiReviewRepresentation::ExportedRedacted);
    assert!(exported.needs_export_detail_note);
}

#[test]
fn vocabulary_drift_is_rejected() {
    let mut packet = seed();
    // Reword one surface of a multi-binding finding to a different (still-valid) role token.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "arsc-scope-ai-panel")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .ai_review_role_word = "lifecycle_state_tracking".to_owned();
    assert!(violations_of(&packet).contains(&"ai_review_vocabulary_drift_across_surfaces"));
}

#[test]
fn ai_review_role_word_outside_vocabulary_is_rejected() {
    let mut packet = seed();
    packet.consumer_bindings[0].state_facets.ai_review_role_word = "totally_made_up".to_owned();
    let tokens = violations_of(&packet);
    assert!(tokens.contains(&"ai_review_role_word_outside_vocabulary"));
    // Rewording one surface also trips drift, which is expected and fine.
}

#[test]
fn dropped_finding_lifecycle_on_gate_role_is_rejected() {
    let mut packet = seed();
    // arsc-scope-selector carries the analyzed_scope_disclosure gate role, which must always keep a real
    // finding lifecycle.
    let target = packet
        .consumer_bindings
        .iter()
        .position(|b| b.binding_id == "arsc-scope-selector")
        .unwrap();
    packet.consumer_bindings[target]
        .state_facets
        .finding_lifecycle_word = "outdated_finding_shown_as_current".to_owned();
    assert!(violations_of(&packet).contains(&"finding_lifecycle_missing_for_gate_role"));
}

#[test]
fn reuse_below_two_consumers_is_rejected() {
    let mut packet = seed();
    // Drop every resolution-memory binding except one, leaving the object with one consumer.
    let mut kept_one = false;
    packet.consumer_bindings.retain(|b| {
        if b.object == M5AiReviewAssistObject::ResolutionMemoryRow {
            if kept_one {
                return false;
            }
            kept_one = true;
        }
        true
    });
    assert!(violations_of(&packet).contains(&"object_reuse_unproven"));
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
        vec![M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF.to_owned()];
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
    packet.consumer_bindings[target].narrow_note = Some(AiReviewNarrowNote {
        reason: AiReviewNarrowReason::CompactionNarrowed,
        preserved_vocabulary_note: "x".to_owned(),
        next_action: AiReviewNarrowNextAction::ExpandInDesktop,
        next_action_label: "x".to_owned(),
    });
    assert!(violations_of(&packet).contains(&"unexpected_narrow_note"));
}

#[test]
fn each_guardrail_is_enforced() {
    let cases: [(fn(&mut AiReviewConsumerBinding), &str); 5] = [
        (
            |b| b.lets_ai_review_results_publish_or_merge_implicitly = true,
            "lets_ai_review_results_publish_or_merge_implicitly",
        ),
        (
            |b| {
                b.hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation = true
            },
            "hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation",
        ),
        (
            |b| b.keeps_stale_findings_looking_current_after_diff_or_instruction_drift = true,
            "keeps_stale_findings_looking_current_after_diff_or_instruction_drift",
        ),
        (
            |b| {
                b.loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails = true
            },
            "loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails",
        ),
        (
            |b| {
                b.presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state = true
            },
            "presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state",
        ),
    ];
    for (mutate, token) in cases {
        let mut packet = seed();
        mutate(&mut packet.consumer_bindings[0]);
        assert!(violations_of(&packet).contains(&token), "expected {token}");
    }
}

#[test]
fn object_coverage_gap_is_rejected() {
    let mut packet = seed();
    packet
        .consumer_bindings
        .retain(|b| b.object != M5AiReviewAssistObject::ResolutionMemoryRow);
    assert!(violations_of(&packet).contains(&"object_coverage_missing"));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = seed();
    packet
        .source_contract_refs
        .retain(|r| r != M5_AI_REVIEW_SHARED_CONSUMERS_DOC_REF);
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
    assert!(lines[0].starts_with("object,consumer,representation,ai_review_role_word,parity_state"));
}

#[test]
fn markdown_summary_lists_every_finding() {
    let packet = seed();
    let summary = packet.render_markdown_summary();
    for binding in &packet.consumer_bindings {
        assert!(summary.contains(&binding.binding_id));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_ai_review_shared_consumers_export()
        .expect("checked M5 AI-review shared-consumer export validates");
    assert_eq!(from_disk.packet_id, M5_AI_REVIEW_SHARED_CONSUMERS_PACKET_ID);
    assert_eq!(
        from_disk,
        seed(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_fixture_variants_validate_and_keep_coverage() {
    let compact = seeded_m5_ai_review_shared_consumers_compact_remote_narrowed();
    assert!(
        compact.validate().is_empty(),
        "{:?}",
        violations_of(&compact)
    );
    assert_eq!(compact.consumer_bindings.len(), 12);

    let exported = seeded_m5_ai_review_shared_consumers_exported_redaction_narrowed();
    assert!(
        exported.validate().is_empty(),
        "{:?}",
        violations_of(&exported)
    );
    assert_eq!(exported.consumer_bindings.len(), 12);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_builders() {
    let compact: M5AiReviewSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-shared-consumers/compact_remote_narrowed.json"
    )))
    .expect("compact fixture parses");
    assert!(compact.validate().is_empty());
    assert_eq!(
        compact,
        seeded_m5_ai_review_shared_consumers_compact_remote_narrowed()
    );

    let exported: M5AiReviewSharedConsumersPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-shared-consumers/exported_redaction_narrowed.json"
    )))
    .expect("exported fixture parses");
    assert!(exported.validate().is_empty());
    assert_eq!(
        exported,
        seeded_m5_ai_review_shared_consumers_exported_redaction_narrowed()
    );
}
