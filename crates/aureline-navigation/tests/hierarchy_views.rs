//! Freeze gate for the hierarchy-views corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/hierarchy_views/canonical_views.json` is the published
//! corpus. This gate rebuilds the corpus in code and asserts it equals the fixture
//! after a serialize round-trip, so the hierarchy-views contract cannot drift from
//! the published artifact without failing CI. It also re-proves that every stored
//! view equals the builder's own output, that the corpus is support-export safe, that
//! every view groups edges by legend, partitions its counts, names missing scope,
//! discloses inferred/runtime edges, exposes competing roots before a jump, exposes
//! the five stable actions, and that every frozen invariant holds. This test runs
//! under `cargo test --workspace`, so stable promotion cannot harden a hierarchy
//! claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::hierarchy_views::{
    build_hierarchy_view, hierarchy_views_set, HierarchyActionKind, HierarchyEdgeLegend,
    HierarchyLabel, HierarchyViewSet, HIERARCHY_LEGEND_ORDER, HIERARCHY_VIEWS_RECORD_KIND,
    HIERARCHY_VIEWS_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/hierarchy_views/canonical_views.json")
}

fn load_fixture() -> HierarchyViewSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = hierarchy_views_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code hierarchy-views corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_hierarchy_views`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, HIERARCHY_VIEWS_RECORD_KIND);
    assert_eq!(fixture.schema_ref, HIERARCHY_VIEWS_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: HierarchyViewSet =
        serde_json::from_str(&serde_json::to_string(&fixture).expect("serializes"))
            .expect("round-trips");
    assert_eq!(roundtrip, fixture);
}

#[test]
fn every_frozen_invariant_holds() {
    let fixture = load_fixture();
    assert!(!fixture.invariants.is_empty());
    for invariant in &fixture.invariants {
        assert!(
            invariant.holds,
            "frozen invariant must hold: {}",
            invariant.invariant_id
        );
    }
    assert!(fixture.all_invariants_hold());
}

#[test]
fn every_stored_view_equals_builder_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = build_hierarchy_view(&scenario.input);
        assert_eq!(
            produced, scenario.view,
            "scenario {} drifted from the builder",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_view_groups_by_legend_without_flattening() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let view = &scenario.view;
        // Tiers stay in canonical legend order.
        let order = |legend| {
            HIERARCHY_LEGEND_ORDER
                .iter()
                .position(|candidate| *candidate == legend)
                .unwrap()
        };
        for pair in view.tiers.windows(2) {
            assert!(order(pair[0].legend) < order(pair[1].legend));
        }
        // Every edge lands in exactly the tier for its legend.
        let grouped: usize = view.tiers.iter().map(|t| t.edge_refs.len()).sum();
        assert_eq!(grouped, view.totals.total_count);
    }
}

#[test]
fn counts_reconcile_and_partition() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
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
fn inferred_and_runtime_never_appear_as_direct() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for tier in &scenario.view.tiers {
            if matches!(
                tier.legend,
                HierarchyEdgeLegend::Inferred | HierarchyEdgeLegend::RuntimeObserved
            ) {
                assert!(
                    !tier.attribution_notes.is_empty() && !tier.downgrade_reasons.is_empty(),
                    "weak tier in {} must carry attribution notes and downgrade reasons",
                    scenario.scenario_id
                );
                assert_ne!(tier.legend, HierarchyEdgeLegend::Direct);
            }
        }
    }
}

#[test]
fn missing_scope_is_named_when_incomplete() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let view = &scenario.view;
        if view.scope_completeness.requires_disclosure() {
            assert!(
                !view.scope_gaps.is_empty(),
                "incomplete view {} must name a scope gap",
                scenario.scenario_id
            );
            assert!(view.labels.contains(&HierarchyLabel::IncompleteScope));
            assert!(!view.downgrade_reasons.is_empty());
        }
    }
}

#[test]
fn ambiguity_is_inspectable_before_a_jump() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let view = &scenario.view;
        if view.ambiguity.ambiguity_class.requires_disambiguation() {
            assert!(view.ambiguity.has_disambiguation_path());
            assert!(view.requires_inspection_before_jump());
            assert!(view.labels.contains(&HierarchyLabel::CompetingRoots));
            for action in &view.actions {
                if action.action_kind.navigates() {
                    assert!(
                        action.gated_by_ambiguity,
                        "navigating action {} must be gated when the root is ambiguous",
                        action.action_kind.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn actions_are_stable_across_routes() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let view = &scenario.view;
        for action_kind in HierarchyActionKind::ALL {
            let affordance = view
                .actions
                .iter()
                .find(|a| a.action_kind == action_kind)
                .unwrap_or_else(|| panic!("missing action {}", action_kind.as_str()));
            assert_eq!(affordance.history_effect, action_kind.history_effect());
            assert!(affordance.preserves_target_identity);
            assert_eq!(affordance.target_ref, view.root_target_ref);
            assert_eq!(affordance.available_routes.len(), 5);
        }
    }
}

#[test]
fn captured_scope_is_always_disclosed() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let view = &scenario.view;
        if view.totals.captured_scope_count > 0 {
            assert!(
                view.captured_scope_ref.is_some() || !view.downgrade_reasons.is_empty(),
                "captured view {} must carry a captured scope ref or downgrade reason",
                scenario.scenario_id
            );
            assert!(!view.attribution_notes.is_empty());
        }
    }
}
