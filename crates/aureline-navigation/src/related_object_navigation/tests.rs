use super::*;

#[test]
fn canonical_set_validates_and_freezes() {
    let set = related_object_navigation_set();
    set.validate().expect("canonical corpus validates");
    assert!(set.all_invariants_hold());
    assert!(set.is_support_export_safe());
    assert_eq!(set.scenarios.len(), 6);
    assert!(!set.invariants.is_empty());
}

#[test]
fn editor_panel_separates_graph_from_framework() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.editor_route_and_component")
        .expect("scenario present")
        .panel;
    let graph = panel
        .group(RelatedObjectSourceClass::GraphDerived)
        .expect("graph group");
    let framework = panel
        .group(RelatedObjectSourceClass::FrameworkDerived)
        .expect("framework group");
    assert_eq!(graph.counts.total_count, 1);
    assert_eq!(framework.counts.total_count, 1);
    assert_eq!(panel.source_headline, RelatedObjectHeadline::Mixed);
    assert_eq!(
        panel.anchor_context,
        RelatedObjectAnchorContext::EditorSymbol
    );
    // Groups are in canonical order: graph before framework.
    let classes: Vec<RelatedObjectSourceClass> = panel
        .groups
        .iter()
        .map(|group| group.source_class)
        .collect();
    assert_eq!(
        classes,
        vec![
            RelatedObjectSourceClass::GraphDerived,
            RelatedObjectSourceClass::FrameworkDerived
        ]
    );
    // The framework group carries attribution notes; the graph group need not.
    assert!(!framework.attribution_notes.is_empty());
    assert!(panel.labels.contains(&RelatedObjectLabel::FrameworkDerived));
    assert!(panel.labels.contains(&RelatedObjectLabel::GraphDerived));
}

#[test]
fn curated_owner_gates_disambiguation_before_jump() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.editor_owner_doc_curated")
        .expect("scenario present")
        .panel;
    assert!(panel.requires_inspection_before_jump());
    assert_eq!(panel.disambiguation.competing_link_refs.len(), 1);
    assert!(panel.disambiguation.disambiguation_set_ref.is_some());
    assert!(panel.disambiguation.has_disambiguation_path());
    assert!(panel
        .labels
        .contains(&RelatedObjectLabel::DisambiguationRequired));
    assert!(panel
        .downgrade_reasons
        .contains(&DowngradeReason::AmbiguousCandidates));
    // Navigating actions are gated; non-navigating ones are not.
    for action in &panel.actions {
        assert_eq!(
            action.gated_by_disambiguation,
            action.action_kind.navigates()
        );
    }
}

#[test]
fn generated_artifact_keeps_runtime_and_imported_disclosed() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.generated_artifact_runtime")
        .expect("scenario present")
        .panel;
    assert_eq!(
        panel.anchor_context,
        RelatedObjectAnchorContext::GeneratedArtifact
    );
    assert!(panel.has_captured_scope());
    assert_eq!(
        panel.captured_scope_ref.as_deref(),
        Some("aureline://scope/captured-trace")
    );
    assert!(panel.labels.contains(&RelatedObjectLabel::ImportedSnapshot));
    assert!(panel
        .labels
        .contains(&RelatedObjectLabel::RuntimeObservedOnly));
    assert!(panel.labels.contains(&RelatedObjectLabel::Generated));
    assert_eq!(panel.totals.generated_count, 1);
    assert_eq!(panel.totals.runtime_observed_only_count, 1);
    assert_eq!(panel.totals.imported_snapshot_count, 1);
    assert!(panel
        .group(RelatedObjectSourceClass::FrameworkDerived)
        .is_some());
    assert!(panel
        .group(RelatedObjectSourceClass::RuntimeDerived)
        .is_some());
}

#[test]
fn notebook_panel_names_unavailable_doc_honestly() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.notebook_test_doc")
        .expect("scenario present")
        .panel;
    assert_eq!(
        panel.anchor_context,
        RelatedObjectAnchorContext::NotebookCell
    );
    assert_eq!(panel.anchor_parity, AnchorParity::PartialAnchorsSupported);
    assert_eq!(panel.totals.unavailable_count, 1);
    assert!(panel.labels.contains(&RelatedObjectLabel::Unavailable));
    assert!(panel.scope_completeness.requires_disclosure());
    assert!(panel.labels.contains(&RelatedObjectLabel::IncompleteScope));
    // The graph-proven test still reads as primary.
    assert_eq!(panel.totals.primary_count, 1);
}

#[test]
fn diff_panel_labels_unsupported_parity_with_no_links() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.diff_hunk_unsupported")
        .expect("scenario present")
        .panel;
    assert_eq!(panel.anchor_context, RelatedObjectAnchorContext::DiffHunk);
    assert_eq!(panel.anchor_parity, AnchorParity::AnchorsUnsupported);
    assert!(panel.groups.is_empty());
    assert_eq!(panel.totals.total_count, 0);
    assert_eq!(panel.source_headline, RelatedObjectHeadline::Empty);
    assert!(panel
        .labels
        .contains(&RelatedObjectLabel::UnsupportedParity));
    assert!(panel
        .downgrade_reasons
        .contains(&DowngradeReason::MissingProvider));
    assert!(!panel.parity_note.trim().is_empty());
}

#[test]
fn docs_linked_panel_discloses_lexical_fallback() {
    let set = related_object_navigation_set();
    let panel = &set
        .scenario("panel.docs_linked_component")
        .expect("scenario present")
        .panel;
    assert_eq!(
        panel.anchor_context,
        RelatedObjectAnchorContext::DocsLinkedSymbol
    );
    assert_eq!(panel.totals.lexical_fallback_count, 1);
    assert!(panel.labels.contains(&RelatedObjectLabel::LexicalFallback));
    assert!(panel
        .downgrade_reasons
        .contains(&DowngradeReason::LexicalFallbackOnly));
}

#[test]
fn counts_partition_source_and_fallback() {
    let set = related_object_navigation_set();
    for scenario in &set.scenarios {
        let panel = &scenario.panel;
        assert!(panel.totals.reconciles());
        assert!(panel.totals.source_partition_reconciles());
        assert!(panel.totals.fallback_partition_reconciles());
        let grouped: usize = panel.groups.iter().map(|g| g.counts.total_count).sum();
        assert_eq!(grouped, panel.totals.total_count);
        for group in &panel.groups {
            assert!(group.counts.reconciles());
            assert!(group.counts.source_partition_reconciles());
            assert!(group.counts.fallback_partition_reconciles());
        }
    }
}

#[test]
fn actions_are_stable_across_every_route() {
    let set = related_object_navigation_set();
    for scenario in &set.scenarios {
        let panel = &scenario.panel;
        assert_eq!(panel.actions.len(), RelatedObjectActionKind::ALL.len());
        for action in &panel.actions {
            assert!(action.preserves_anchor_identity);
            assert_eq!(action.anchor_ref, panel.anchor_ref);
            assert_eq!(action.history_effect, action.action_kind.history_effect());
            assert_eq!(
                action.available_routes.len(),
                RelatedObjectActionRoute::ALL.len()
            );
            for route in RelatedObjectActionRoute::ALL {
                assert!(action.available_routes.contains(&route));
            }
        }
        let reveal = panel
            .actions
            .iter()
            .find(|a| a.action_kind == RelatedObjectActionKind::RevealAttribution)
            .expect("reveal action");
        assert_eq!(
            reveal.history_effect,
            RelatedObjectHistoryEffect::PreservesCurrent
        );
        assert!(!reveal.gated_by_disambiguation);
    }
}

#[test]
fn every_consumer_projection_preserves_truth() {
    let set = related_object_navigation_set();
    for scenario in &set.scenarios {
        for projection in &scenario.panel.consumer_projections {
            assert!(projection.preserves_truth());
            assert!(!projection.flattens_to_generic_links);
            assert!(!projection.exports_code_bodies);
        }
    }
}

#[test]
fn object_kind_maps_to_closed_relation_vocabulary() {
    assert_eq!(
        RelatedObjectKind::Route.relation_kind(),
        RelationKind::RouteBinding
    );
    assert_eq!(
        RelatedObjectKind::Owner.relation_kind(),
        RelationKind::OwnerLink
    );
    assert_eq!(
        RelatedObjectKind::Doc.relation_kind(),
        RelationKind::DocLink
    );
    let set = related_object_navigation_set();
    for scenario in &set.scenarios {
        for link in scenario.panel.links() {
            assert_eq!(link.relation_kind, link.object_kind.relation_kind());
        }
    }
}

#[test]
fn set_round_trips_through_json() {
    let set = related_object_navigation_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let round_trip: RelatedObjectNavigationSet = serde_json::from_str(&json).expect("deserializes");
    assert_eq!(round_trip, set);
}

#[test]
fn drifted_panel_fails_validation() {
    let mut set = related_object_navigation_set();
    set.scenarios[0].panel.summary = "tampered".to_owned();
    assert!(set.validate().is_err());
}

#[test]
fn flattening_to_generic_links_breaks_consumer_invariant() {
    let mut set = related_object_navigation_set();
    set.scenarios[0].panel.consumer_projections[0].flattens_to_generic_links = true;
    // The stored panel no longer matches the builder output.
    assert!(set.validate().is_err());
}

#[test]
fn captured_only_detects_imported_and_runtime() {
    let base = RelatedObjectLink {
        link_id: "l".to_owned(),
        object_kind: RelatedObjectKind::Component,
        relation_kind: RelationKind::Type,
        source_class: RelatedObjectSourceClass::GraphDerived,
        anchor_ref: "aureline://object/a".to_owned(),
        target_ref: "aureline://object/b".to_owned(),
        alternate_target_refs: vec![],
        fallback_mode: RelatedObjectFallbackMode::Primary,
        proof_class: ProofClass::IndexedSemantic,
        confidence: NavigationConfidence::Indexed,
        freshness: FreshnessClass::WarmCached,
        scope_completeness: ScopeCompleteness::CompleteForDeclaredScope,
        generated_or_external_state: GeneratedOrExternalState::AuthoredSource,
        downgrade_reasons: vec![],
        evidence_refs: vec![],
        summary: "s".to_owned(),
    };
    assert!(!base.is_captured_only());

    let mut imported = base.clone();
    imported.fallback_mode = RelatedObjectFallbackMode::ImportedSnapshot;
    assert!(imported.is_captured_only());

    let mut runtime = base.clone();
    runtime.proof_class = ProofClass::RuntimeObserved;
    assert!(runtime.is_captured_only());

    let mut stale = base;
    stale.freshness = FreshnessClass::Stale;
    assert!(stale.is_captured_only());
}
