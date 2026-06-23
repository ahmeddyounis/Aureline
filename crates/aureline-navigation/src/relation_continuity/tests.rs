use super::*;
use crate::target_model::{ContinuityState, RelationKind, RenameApplyPosture};

#[test]
fn canonical_set_validates_and_freezes() {
    let set = relation_continuity_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 5);
    assert!(!set.invariants.is_empty());
}

#[test]
fn bound_entries_preserve_relation_and_auto_open() {
    let set = relation_continuity_set();
    let packet = &set
        .scenario("continuity.bound_peek_reveal_split")
        .expect("scenario present")
        .packet;
    assert_eq!(packet.counts.total_count, 3);
    assert_eq!(packet.counts.current_scope_count, 3);
    assert_eq!(packet.counts.captured_scope_count, 0);
    assert_eq!(packet.counts.requires_disclosure_count, 0);
    for entry in &packet.entries {
        assert!(entry.auto_open_allowed);
        assert!(entry.current_scope);
        assert_eq!(entry.drift_state, ContinuityState::Bound);
        assert!(entry.current_target.is_some());
        // Relation kind and return context survive on a temporary surface.
        assert_eq!(entry.captured_target.relation_kind, entry.relation_kind);
        assert!(!entry.return_anchor.return_anchor_ref.is_empty());
        assert!(entry.return_anchor.restores_selection);
    }
    // The bound rename preview rides along as replayable evidence.
    assert_eq!(packet.rename_evidence.len(), 1);
    assert_eq!(
        packet.rename_evidence[0].root_relation_kind,
        RelationKind::Definition
    );
}

#[test]
fn remapped_entries_cite_stable_evidence_and_never_silently_jump() {
    let set = relation_continuity_set();
    let packet = &set
        .scenario("continuity.remapped_history")
        .expect("scenario present")
        .packet;
    let remapped = packet
        .entry("entry.back.definition.remapped")
        .expect("remapped entry present");
    assert_eq!(remapped.drift_state, ContinuityState::Remapped);
    // A remap keeps the relation kind and cites stable evidence — never a nearby fallback.
    assert!(!remapped.used_nearby_fallback);
    assert!(!remapped.remap_evidence_refs.is_empty());
    assert!(remapped.current_target.is_some());
    assert_eq!(
        remapped.current_target.as_ref().unwrap().relation_kind,
        remapped.relation_kind
    );
    // It does not auto-open; the user takes the disclosed open action instead.
    assert!(!remapped.auto_open_allowed);
    assert!(remapped
        .recovery_choices
        .contains(&RelationRecoveryChoice::OpenRemappedTarget));
}

#[test]
fn drifted_missing_scope_states_stay_visible_with_no_current_target() {
    let set = relation_continuity_set();
    let packet = &set
        .scenario("continuity.drifted_missing_scope")
        .expect("scenario present")
        .packet;
    for entry in &packet.entries {
        assert_ne!(entry.drift_state, ContinuityState::Bound);
        // No jump: no current target, a visible reason, and recovery choices.
        assert!(entry.current_target.is_none());
        assert!(!entry.auto_open_allowed);
        assert!(entry.drift_reason.as_ref().is_some_and(|r| !r.is_empty()));
        assert!(!entry.recovery_choices.is_empty());
    }
    // All three are captured-only; none claim current scope.
    assert_eq!(packet.counts.current_scope_count, 0);
    assert_eq!(packet.counts.captured_scope_count, 3);
    assert_eq!(packet.counts.requires_disclosure_count, 3);
}

#[test]
fn lexical_and_runtime_evidence_never_auto_opens() {
    let set = relation_continuity_set();
    let packet = &set
        .scenario("continuity.fallback_runtime_framework")
        .expect("scenario present")
        .packet;
    let lexical = packet
        .entry("entry.peek.call.lexical")
        .expect("lexical entry present");
    assert_eq!(
        lexical.evidence_class,
        RelationContinuityEvidenceClass::LexicalFallback
    );
    assert!(lexical.evidence_class.is_fallback());
    assert!(!lexical.current_scope);
    assert!(!lexical.auto_open_allowed);
    assert!(!lexical.fallback_notes.is_empty());
    assert!(!lexical.downgrade_reasons.is_empty());
}

#[test]
fn ambiguous_entry_keeps_a_disambiguation_path() {
    let set = relation_continuity_set();
    let packet = &set
        .scenario("continuity.archived_and_ambiguous")
        .expect("scenario present")
        .packet;
    let ambiguous = packet
        .entry("entry.peek.declaration.ambiguous")
        .expect("ambiguous entry present");
    assert!(ambiguous.ambiguity_class.requires_disambiguation());
    assert!(ambiguous.disambiguation_set_ref.is_some());
    assert!(ambiguous
        .recovery_choices
        .contains(&RelationRecoveryChoice::ChooseFromDisambiguation));
    // A blocked drifted rename preview survives export with its posture and replay id.
    let evidence = &packet.rename_evidence[0];
    assert_eq!(
        evidence.apply_posture,
        RenameApplyPosture::BlockedPendingScopeReview
    );
    assert_eq!(evidence.drift_state, ContinuityState::Drifted);
    assert!(!evidence.replay_target_id.is_empty());
    assert!(evidence.disambiguation_set_ref.is_some());
}

#[test]
fn lines_view_round_trips_every_scenario() {
    let set = relation_continuity_set();
    let lines = relation_continuity_lines(&set);
    assert!(lines
        .iter()
        .any(|line| line.contains(RELATION_CONTINUITY_SET_ID)));
    for scenario in &set.scenarios {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(&scenario.scenario_id)),
            "lines view omitted {}",
            scenario.scenario_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let set = relation_continuity_set();
    let json = serde_json::to_string(&set).expect("set serializes");
    let round_trip: RelationContinuitySet = serde_json::from_str(&json).expect("set deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn corpus_covers_every_drift_state_and_entry_kind() {
    let set = relation_continuity_set();
    let packets: Vec<&RelationContinuityPacket> = set.scenarios.iter().map(|s| &s.packet).collect();
    for state in RELATION_CONTINUITY_DRIFT_STATES {
        assert!(
            packets.iter().any(|packet| packet
                .entries
                .iter()
                .any(|entry| entry.drift_state == state)),
            "no entry covers drift state {state:?}"
        );
    }
    for kind in RELATION_NAV_ENTRY_ORDER {
        assert!(
            packets
                .iter()
                .any(|packet| packet.entries.iter().any(|entry| entry.entry_kind == kind)),
            "no entry covers kind {}",
            kind.as_str()
        );
    }
}
