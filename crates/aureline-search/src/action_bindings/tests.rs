use super::*;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_search_action_binding_packet();
    assert_eq!(packet.record_kind, SEARCH_ACTION_BINDING_PACKET_RECORD_KIND);
    assert_eq!(packet.packet_id, SEARCH_ACTION_BINDING_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_all_five_flows_once() {
    let packet = seeded_search_action_binding_packet();
    assert_eq!(packet.flows.len(), ActionFlowClass::ALL.len());
    for flow in ActionFlowClass::ALL {
        assert!(packet.flow_for(flow).is_some(), "missing {flow:?}");
    }
}

#[test]
fn realizes_all_action_kinds_and_fallback_triggers() {
    let packet = seeded_search_action_binding_packet();
    for kind in SearchActionKind::ALL {
        assert!(
            packet
                .realized_action_kind_tokens()
                .contains(&kind.as_str()),
            "missing action kind {}",
            kind.as_str()
        );
    }
    for trigger in FallbackTriggerClass::ALL {
        assert!(
            packet
                .realized_fallback_trigger_tokens()
                .contains(&trigger.as_str()),
            "missing fallback trigger {}",
            trigger.as_str()
        );
    }
}

#[test]
fn keeps_definition_and_declaration_distinguishable() {
    // Acceptance: search actions no longer silently degrade from definition to
    // declaration; both relation kinds are realized and the degrade carries an
    // explicit, recoverable fallback.
    let packet = seeded_search_action_binding_packet();
    let relations = packet.realized_relation_kind_tokens();
    assert!(relations.contains(&RelationKind::Definition.as_str()));
    assert!(relations.contains(&RelationKind::Declaration.as_str()));

    let degrade = packet
        .flows
        .iter()
        .flat_map(|flow| &flow.bindings)
        .find(|binding| binding.relation_kind_degraded())
        .expect("a relation-kind degrade is realized");
    assert_eq!(degrade.requested_relation_kind, RelationKind::Definition);
    assert_eq!(degrade.resolved_relation_kind, RelationKind::Declaration);
    let fallback = degrade
        .fallback
        .as_ref()
        .expect("degrade carries a fallback");
    assert!(fallback.relation_kind_changed);
    assert!(fallback.recoverable);
    assert!(!fallback.visible_reason.trim().is_empty());
}

#[test]
fn external_handoff_is_never_silent() {
    // Acceptance: local docs never silently become a browser handoff.
    let packet = seeded_search_action_binding_packet();
    let handoff = packet
        .flow_for(ActionFlowClass::DocsResults)
        .expect("docs flow")
        .bindings
        .iter()
        .find(|binding| binding.action_kind == SearchActionKind::ExternalHandoff)
        .expect("an external handoff binding");
    let fallback = handoff
        .fallback
        .as_ref()
        .expect("handoff carries a fallback");
    assert!(fallback.crosses_to_external_handoff);
    assert!(!fallback.visible_reason.trim().is_empty());
    assert!(fallback.recoverable);
}

#[test]
fn split_peek_open_in_place_keep_return_anchors() {
    // Acceptance: split, peek, and open-in-place preserve attributable target
    // refs and return anchors.
    let packet = seeded_search_action_binding_packet();
    for flow in &packet.flows {
        for binding in &flow.bindings {
            assert!(
                !binding.return_anchor_ref.trim().is_empty(),
                "binding {} dropped its return anchor",
                binding.binding_id
            );
            assert_ne!(
                binding.return_anchor_ref, binding.action_binding.open_target_ref,
                "binding {} return anchor collapsed into the open target",
                binding.binding_id
            );
            assert!(!binding.action_binding.open_target_ref.trim().is_empty());
        }
    }
}

#[test]
fn fallback_mode_matches_canonical_binding() {
    // The fallback reuses the canonical action-binding fallback mode verbatim.
    let packet = seeded_search_action_binding_packet();
    for flow in &packet.flows {
        for binding in &flow.bindings {
            if let Some(fallback) = &binding.fallback {
                assert_eq!(fallback.fallback_mode, binding.action_binding.fallback_mode);
                assert_ne!(fallback.fallback_mode, ActionFallbackModeClass::Direct);
            } else {
                assert_eq!(binding.fallback_trigger, FallbackTriggerClass::None);
                assert_eq!(
                    binding.action_binding.fallback_mode,
                    ActionFallbackModeClass::Direct
                );
            }
        }
    }
}

#[test]
fn all_three_consumers_reuse_one_binding_object() {
    // Acceptance: history/back-forward and support replay use the same action
    // binding objects.
    let packet = seeded_search_action_binding_packet();
    for required in ActionConsumerClass::ALL {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.consumer == required)
            .unwrap_or_else(|| panic!("missing consumer {}", required.as_str()));
        assert_eq!(projection.ingested_packet_id, packet.packet_id);
        assert!(projection.preserves_action_bindings);
        assert!(projection.preserves_relation_kinds);
        assert!(projection.preserves_return_anchors);
        assert!(projection.preserves_fallback_reasons);
        assert!(projection.reuses_same_binding_objects);
        assert!(!projection.widens_authority);
    }
}

#[test]
fn narrowed_variant_falls_back_more_but_preserves_identity_and_vocabulary() {
    let canonical = seeded_search_action_binding_packet();
    let degraded = seeded_scope_trust_narrowed_search_action_binding_packet();
    assert!(degraded.validate().is_empty());
    assert!(degraded.is_export_safe());

    // The full action-kind, relation-kind, and trigger vocabulary is preserved.
    assert_eq!(
        canonical.realized_action_kind_tokens(),
        degraded.realized_action_kind_tokens()
    );
    assert_eq!(
        canonical.realized_relation_kind_tokens(),
        degraded.realized_relation_kind_tokens()
    );
    assert_eq!(
        canonical.realized_fallback_trigger_tokens(),
        degraded.realized_fallback_trigger_tokens()
    );

    // Under narrowed scope/trust the search-results flow falls back strictly more.
    let canonical_search = canonical.flow_for(ActionFlowClass::SearchResults).unwrap();
    let degraded_search = degraded.flow_for(ActionFlowClass::SearchResults).unwrap();
    assert!(
        degraded_search.fallback_binding_count() > canonical_search.fallback_binding_count(),
        "narrowed search flow should fall back more"
    );

    // History/back-forward replay reads local material and is unchanged.
    assert_eq!(
        canonical.flow_for(ActionFlowClass::HistoryReplay),
        degraded.flow_for(ActionFlowClass::HistoryReplay)
    );
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked =
        current_search_action_binding_packet().expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_search_action_binding_packet());
}

#[test]
fn support_export_preserves_the_packet_safely() {
    let packet = seeded_search_action_binding_packet();
    let export = packet.support_export("action-binding-export-1", "2026-06-17T00:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.action_binding_packet, packet);
}

#[test]
fn detects_silent_relation_degrade() {
    // A definition jump must never silently resolve to a declaration without a
    // visible wrong-target fallback.
    let mut packet = seeded_search_action_binding_packet();
    let binding = &mut packet
        .flows
        .iter_mut()
        .find(|flow| flow.flow == ActionFlowClass::SearchResults)
        .unwrap()
        .bindings[0];
    binding.resolved_relation_kind = RelationKind::Declaration;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("relation-kind degrade must carry an explicit wrong-target fallback")));
}

#[test]
fn detects_fallback_without_visible_reason() {
    let mut packet = seeded_search_action_binding_packet();
    let binding = packet
        .flows
        .iter_mut()
        .find(|flow| flow.flow == ActionFlowClass::DocsResults)
        .unwrap()
        .bindings
        .iter_mut()
        .find(|binding| binding.fallback.is_some())
        .unwrap();
    binding.fallback.as_mut().unwrap().visible_reason = String::new();
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("fallback must keep a user-visible reason")));
}

#[test]
fn detects_dropped_return_anchor() {
    let mut packet = seeded_search_action_binding_packet();
    packet.flows[0].bindings[0].return_anchor_ref = String::new();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("must keep a return anchor")));
}

#[test]
fn detects_authority_widening() {
    let mut packet = seeded_search_action_binding_packet();
    packet.flows[0].bindings[0].authority_not_widened = false;
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("must not widen authority")));
}

#[test]
fn detects_result_identity_collapsed_into_label() {
    let mut packet = seeded_search_action_binding_packet();
    let binding = &mut packet.flows[0].bindings[0];
    binding.display_title = binding.result_id.clone();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("collapse into the display label")));
}
