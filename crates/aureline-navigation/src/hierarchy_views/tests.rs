use super::*;

#[test]
fn canonical_set_validates_and_freezes() {
    let set = hierarchy_views_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 5);
    assert!(!set.invariants.is_empty());
}

#[test]
fn call_view_separates_direct_from_transitive() {
    let set = hierarchy_views_set();
    let view = &set
        .scenario("view.call_direct_and_transitive")
        .expect("scenario present")
        .view;
    let direct = view.tier(HierarchyEdgeLegend::Direct).expect("direct tier");
    let transitive = view
        .tier(HierarchyEdgeLegend::Transitive)
        .expect("transitive tier");
    assert_eq!(direct.counts.total_count, 2);
    assert_eq!(transitive.counts.total_count, 1);
    assert_eq!(view.view_kind, HierarchyViewKind::Call);
    assert_eq!(view.view_legend, HierarchyEdgeLegend::Mixed);
    assert_eq!(view.totals.captured_scope_count, 0);
    // Tiers are in canonical order: direct before transitive.
    let legends: Vec<HierarchyEdgeLegend> = view.tiers.iter().map(|t| t.legend).collect();
    assert_eq!(
        legends,
        vec![HierarchyEdgeLegend::Direct, HierarchyEdgeLegend::Transitive]
    );
    assert!(view.labels.contains(&HierarchyLabel::Transitive));
}

#[test]
fn runtime_and_framework_edges_stay_inferred_or_runtime() {
    let set = hierarchy_views_set();
    let view = &set
        .scenario("view.call_runtime_and_inferred")
        .expect("scenario present")
        .view;
    let runtime = view
        .tier(HierarchyEdgeLegend::RuntimeObserved)
        .expect("runtime tier");
    let inferred = view
        .tier(HierarchyEdgeLegend::Inferred)
        .expect("inferred tier");
    assert_eq!(runtime.counts.runtime_observed_count, 1);
    assert_eq!(inferred.counts.framework_count, 1);
    assert!(view.tier(HierarchyEdgeLegend::Direct).is_some());
    // Both weaker tiers carry attribution notes and downgrade reasons.
    assert!(!runtime.attribution_notes.is_empty());
    assert!(!runtime.downgrade_reasons.is_empty());
    assert!(!inferred.attribution_notes.is_empty());
    assert!(!inferred.downgrade_reasons.is_empty());
    assert!(view.labels.contains(&HierarchyLabel::RuntimeObserved));
    assert!(view.labels.contains(&HierarchyLabel::FrameworkDerived));
    assert!(view.has_captured_scope());
}

#[test]
fn type_view_names_missing_scope() {
    let set = hierarchy_views_set();
    let view = &set
        .scenario("view.type_incomplete_scope")
        .expect("scenario present")
        .view;
    assert_eq!(view.view_kind, HierarchyViewKind::Type);
    assert!(view.scope_completeness.requires_disclosure());
    assert_eq!(view.scope_gaps.len(), 1);
    assert_eq!(
        view.scope_gaps[0].scope_ref,
        "aureline://scope/external-crate"
    );
    assert!(view.labels.contains(&HierarchyLabel::IncompleteScope));
    assert!(!view.downgrade_reasons.is_empty());
    // The missing scope is named in the attribution notes.
    assert!(view
        .attribution_notes
        .iter()
        .any(|note| note.contains("external-crate")));
}

#[test]
fn override_view_exposes_competing_roots_and_gates_jumps() {
    let set = hierarchy_views_set();
    let view = &set
        .scenario("view.override_ambiguous_roots")
        .expect("scenario present")
        .view;
    assert_eq!(view.view_kind, HierarchyViewKind::Override);
    assert!(view.requires_inspection_before_jump());
    assert_eq!(view.ambiguity.competing_root_refs.len(), 2);
    assert!(view.ambiguity.disambiguation_set_ref.is_some());
    assert!(view.ambiguity.has_disambiguation_path());
    assert!(view.labels.contains(&HierarchyLabel::CompetingRoots));
    // Navigating actions are gated; non-navigating ones are not.
    for action in &view.actions {
        assert_eq!(action.gated_by_ambiguity, action.action_kind.navigates());
    }
    assert!(view
        .downgrade_reasons
        .contains(&DowngradeReason::AmbiguousCandidates));
}

#[test]
fn ownership_view_keeps_inferred_and_imported_disclosed() {
    let set = hierarchy_views_set();
    let view = &set
        .scenario("view.ownership_inferred_imported")
        .expect("scenario present")
        .view;
    assert_eq!(view.view_kind, HierarchyViewKind::Ownership);
    assert!(view.tier(HierarchyEdgeLegend::Inferred).is_some());
    assert!(view.tier(HierarchyEdgeLegend::Direct).is_none());
    assert_eq!(view.totals.framework_count, 1);
    assert_eq!(view.totals.imported_count, 1);
    assert!(view.has_captured_scope());
    assert_eq!(
        view.captured_scope_ref.as_deref(),
        Some("aureline://scope/imported-pack")
    );
    assert!(view.labels.contains(&HierarchyLabel::ImportedSnapshot));
    assert!(view.labels.contains(&HierarchyLabel::FrameworkDerived));
    assert!(view.labels.contains(&HierarchyLabel::Generated));
    assert!(view.scope_completeness.requires_disclosure());
    assert!(!view.scope_gaps.is_empty());
}

#[test]
fn legend_counts_partition_the_total() {
    let set = hierarchy_views_set();
    for scenario in &set.scenarios {
        let view = &scenario.view;
        assert!(view.totals.reconciles());
        assert!(view.totals.legend_partition_reconciles());
        let grouped: usize = view.tiers.iter().map(|t| t.counts.total_count).sum();
        assert_eq!(grouped, view.totals.total_count);
        for tier in &view.tiers {
            assert!(tier.counts.reconciles());
            assert!(tier.counts.legend_partition_reconciles());
        }
    }
}

#[test]
fn actions_are_stable_across_every_route() {
    let set = hierarchy_views_set();
    for scenario in &set.scenarios {
        let view = &scenario.view;
        assert_eq!(view.actions.len(), HierarchyActionKind::ALL.len());
        for action in &view.actions {
            assert!(action.preserves_target_identity);
            assert_eq!(action.target_ref, view.root_target_ref);
            assert_eq!(action.history_effect, action.action_kind.history_effect());
            assert_eq!(
                action.available_routes.len(),
                HierarchyActionRoute::ALL.len()
            );
            for route in HierarchyActionRoute::ALL {
                assert!(action.available_routes.contains(&route));
            }
        }
        let expand = view
            .actions
            .iter()
            .find(|a| a.action_kind == HierarchyActionKind::Expand)
            .expect("expand action");
        assert_eq!(
            expand.history_effect,
            HierarchyHistoryEffect::PreservesCurrent
        );
        assert!(!expand.gated_by_ambiguity);
    }
}

#[test]
fn every_consumer_projection_preserves_truth() {
    let set = hierarchy_views_set();
    for scenario in &set.scenarios {
        for projection in &scenario.view.consumer_projections {
            assert!(projection.preserves_truth());
            assert!(!projection.flattens_to_single_tree);
            assert!(!projection.exports_code_bodies);
        }
    }
}

#[test]
fn set_round_trips_through_json() {
    let set = hierarchy_views_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let round_trip: HierarchyViewSet = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn drifted_view_fails_validation() {
    let mut set = hierarchy_views_set();
    set.scenarios[0].view.summary = "tampered".to_owned();
    assert!(set.validate().is_err());
}

#[test]
fn flattening_to_single_tree_breaks_consumer_invariant() {
    let mut set = hierarchy_views_set();
    set.scenarios[0].view.consumer_projections[0].flattens_to_single_tree = true;
    // The stored view no longer matches the builder output.
    assert!(set.validate().is_err());
}

#[test]
fn edge_legend_maps_proof_and_depth() {
    let base = HierarchyEdge {
        edge_id: "e".to_owned(),
        source_ref: "aureline://object/a".to_owned(),
        target_ref: "aureline://node/b".to_owned(),
        edge_kind: HierarchyEdgeKind::Calls,
        proof_class: ProofClass::DirectSemantic,
        depth: 1,
        scope_completeness: ScopeCompleteness::CompleteForDeclaredScope,
        freshness: FreshnessClass::AuthoritativeLive,
        confidence: NavigationConfidence::Exact,
        runtime_or_framework_evidence_refs: vec![],
        downgrade_reasons: vec![],
        summary: "s".to_owned(),
    };
    assert_eq!(edge_legend(&base), HierarchyEdgeLegend::Direct);

    let mut deep = base.clone();
    deep.depth = 4;
    assert_eq!(edge_legend(&deep), HierarchyEdgeLegend::Transitive);

    let mut framework = base.clone();
    framework.proof_class = ProofClass::FrameworkDerived;
    assert_eq!(edge_legend(&framework), HierarchyEdgeLegend::Inferred);

    let mut runtime = base.clone();
    runtime.proof_class = ProofClass::RuntimeObserved;
    runtime.depth = 5;
    // Proof class wins over depth: runtime is never relabeled transitive.
    assert_eq!(edge_legend(&runtime), HierarchyEdgeLegend::RuntimeObserved);

    let mut lexical = base;
    lexical.proof_class = ProofClass::LexicalFallback;
    assert_eq!(edge_legend(&lexical), HierarchyEdgeLegend::Inferred);
}
