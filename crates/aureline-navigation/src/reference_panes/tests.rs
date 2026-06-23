use super::*;
use crate::target_model::AccessKind;

#[test]
fn canonical_set_validates_and_freezes() {
    let set = reference_panes_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 5);
    assert!(!set.invariants.is_empty());
}

#[test]
fn semantic_pane_groups_read_write_call_distinctly() {
    let set = reference_panes_set();
    let pane = &set
        .scenario("pane.semantic_read_write_call")
        .expect("scenario present")
        .pane;
    let read = pane.group(AccessKind::Read).expect("read group");
    let write = pane.group(AccessKind::Write).expect("write group");
    let call = pane.group(AccessKind::Call).expect("call group");
    assert_eq!(read.counts.total_count, 2);
    assert_eq!(write.counts.total_count, 1);
    assert_eq!(call.counts.total_count, 1);
    assert_eq!(read.evidence_class, ReferenceEvidenceClass::Semantic);
    assert_eq!(pane.pane_evidence_class, ReferenceEvidenceClass::Semantic);
    assert_eq!(pane.totals.captured_scope_count, 0);
    // Groups are in canonical order: read before write before call.
    let kinds: Vec<AccessKind> = pane.groups.iter().map(|g| g.access_kind).collect();
    assert_eq!(
        kinds,
        vec![AccessKind::Read, AccessKind::Write, AccessKind::Call]
    );
}

#[test]
fn generated_test_external_labels_are_visible() {
    let set = reference_panes_set();
    let pane = &set
        .scenario("pane.generated_test_external_labels")
        .expect("scenario present")
        .pane;
    assert!(pane.labels.contains(&ReferenceLabel::Generated));
    assert!(pane.labels.contains(&ReferenceLabel::TestOnly));
    assert!(pane.labels.contains(&ReferenceLabel::External));
    assert_eq!(pane.totals.generated_count, 1);
    assert_eq!(pane.totals.test_only_count, 1);
    assert_eq!(pane.totals.external_count, 1);
    // The generated occurrence is grouped under Generated, not folded into Read.
    let generated = pane.group(AccessKind::Generated).expect("generated group");
    assert!(generated.labels.contains(&ReferenceLabel::Generated));
}

#[test]
fn current_versus_captured_counts_are_separated() {
    let set = reference_panes_set();
    let pane = &set
        .scenario("pane.current_versus_captured")
        .expect("scenario present")
        .pane;
    assert_eq!(pane.totals.current_scope_count, 1);
    assert_eq!(pane.totals.captured_scope_count, 2);
    assert!(pane.totals.reconciles());
    assert!(pane.has_captured_scope());
    assert_eq!(
        pane.captured_scope_ref.as_deref(),
        Some("aureline://scope/captured-trace")
    );
    assert_eq!(pane.totals.runtime_observed_count, 1);
    assert_eq!(pane.pane_evidence_class, ReferenceEvidenceClass::Mixed);
    assert!(pane.labels.contains(&ReferenceLabel::RuntimeObserved));
    assert!(pane.labels.contains(&ReferenceLabel::ImportedSnapshot));
    assert!(!pane.fallback_notes.is_empty());
}

#[test]
fn lexical_fallback_never_shown_as_semantic() {
    let set = reference_panes_set();
    let pane = &set
        .scenario("pane.lexical_fallback_disclosed")
        .expect("scenario present")
        .pane;
    let read = pane.group(AccessKind::Read).expect("read group");
    assert_eq!(read.evidence_class, ReferenceEvidenceClass::LexicalFallback);
    assert!(read.evidence_class.is_fallback());
    assert!(!read.fallback_notes.is_empty());
    assert!(read
        .downgrade_reasons
        .contains(&DowngradeReason::LexicalFallbackOnly));
    let write = pane.group(AccessKind::Write).expect("write group");
    assert_eq!(write.evidence_class, ReferenceEvidenceClass::SyntaxFallback);
    assert_eq!(pane.totals.fallback_count, 2);
}

#[test]
fn actions_are_stable_across_every_route() {
    let set = reference_panes_set();
    for scenario in &set.scenarios {
        let pane = &scenario.pane;
        assert_eq!(pane.actions.len(), PaneActionKind::ALL.len());
        for action in &pane.actions {
            assert!(action.preserves_target_identity);
            assert_eq!(action.target_ref, pane.root_target_ref);
            assert_eq!(action.history_effect, action.action_kind.history_effect());
            assert_eq!(action.available_routes.len(), ActionRoute::ALL.len());
            for route in ActionRoute::ALL {
                assert!(action.available_routes.contains(&route));
            }
        }
        let peek = pane
            .actions
            .iter()
            .find(|a| a.action_kind == PaneActionKind::Peek)
            .expect("peek action");
        assert_eq!(peek.history_effect, HistoryEffect::PreservesCurrent);
        let open = pane
            .actions
            .iter()
            .find(|a| a.action_kind == PaneActionKind::Open)
            .expect("open action");
        assert_eq!(open.history_effect, HistoryEffect::AdvancesHistory);
    }
}

#[test]
fn every_consumer_projection_preserves_truth() {
    let set = reference_panes_set();
    for scenario in &set.scenarios {
        for projection in &scenario.pane.consumer_projections {
            assert!(projection.preserves_truth());
            assert!(!projection.flattens_to_generic_hits);
            assert!(!projection.exports_code_bodies);
        }
    }
}

#[test]
fn set_round_trips_through_json() {
    let set = reference_panes_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let round_trip: ReferencePaneSet = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn drifted_pane_fails_validation() {
    let mut set = reference_panes_set();
    set.scenarios[0].pane.summary = "tampered".to_owned();
    assert!(set.validate().is_err());
}

#[test]
fn flattening_to_generic_hits_breaks_consumer_invariant() {
    let mut set = reference_panes_set();
    set.scenarios[0].pane.consumer_projections[0].flattens_to_generic_hits = true;
    // The stored pane no longer matches the builder output.
    assert!(set.validate().is_err());
}
