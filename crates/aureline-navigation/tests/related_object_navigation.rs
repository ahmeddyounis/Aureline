//! Freeze gate for the related-object navigation corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/related_object_navigation/canonical_links.json` is the published
//! corpus. This gate rebuilds the corpus in code and asserts it equals the fixture after
//! a serialize round-trip, so the related-object navigation contract cannot drift from
//! the published artifact without failing CI. It also re-proves that every stored panel
//! equals the builder's own output, that the corpus is support-export safe, that every
//! panel groups links by source class, partitions its counts, discloses fallback truth,
//! names an incomplete scope, exposes competing links before a jump, labels unsupported
//! anchor parity honestly, exposes the five stable actions, and that every frozen
//! invariant holds. This test runs under `cargo test --workspace`, so stable promotion
//! cannot harden a related-object navigation claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::related_object_navigation::{
    build_related_object_panel, related_object_navigation_set, AnchorParity,
    RelatedObjectActionKind, RelatedObjectLabel, RelatedObjectNavigationSet,
    RelatedObjectSourceClass, RELATED_OBJECT_NAV_RECORD_KIND, RELATED_OBJECT_NAV_SCHEMA_REF,
    RELATED_OBJECT_SOURCE_ORDER,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/related_object_navigation/canonical_links.json")
}

fn load_fixture() -> RelatedObjectNavigationSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = related_object_navigation_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code related-object navigation corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_related_object_navigation`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, RELATED_OBJECT_NAV_RECORD_KIND);
    assert_eq!(fixture.schema_ref, RELATED_OBJECT_NAV_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RelatedObjectNavigationSet =
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
fn every_stored_panel_equals_builder_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = build_related_object_panel(&scenario.input);
        assert_eq!(
            produced, scenario.panel,
            "scenario {} drifted from the builder",
            scenario.scenario_id
        );
    }
}

#[test]
fn every_panel_groups_by_source_class_without_flattening() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let panel = &scenario.panel;
        // Groups stay in canonical source-class order.
        let order = |class| {
            RELATED_OBJECT_SOURCE_ORDER
                .iter()
                .position(|candidate| *candidate == class)
                .unwrap()
        };
        for pair in panel.groups.windows(2) {
            assert!(order(pair[0].source_class) < order(pair[1].source_class));
        }
        // Every link lands in exactly the group for its source class.
        let grouped: usize = panel.groups.iter().map(|g| g.links.len()).sum();
        assert_eq!(grouped, panel.totals.total_count);
        for group in &panel.groups {
            for link in &group.links {
                assert_eq!(link.source_class, group.source_class);
                assert_eq!(link.relation_kind, link.object_kind.relation_kind());
            }
        }
    }
}

#[test]
fn counts_reconcile_and_partition() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
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
fn non_graph_links_never_read_as_graph_proof() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for group in &scenario.panel.groups {
            if group.source_class != RelatedObjectSourceClass::GraphDerived {
                assert!(
                    !group.attribution_notes.is_empty(),
                    "non-graph group in {} must carry attribution notes",
                    scenario.scenario_id
                );
            }
            for link in &group.links {
                if link.source_class.requires_disclosure() {
                    assert!(
                        !link.downgrade_reasons.is_empty() || !link.evidence_refs.is_empty(),
                        "disclosed link {} must carry a downgrade reason or evidence",
                        link.link_id
                    );
                }
            }
        }
    }
}

#[test]
fn disambiguation_is_inspectable_before_a_jump() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let panel = &scenario.panel;
        if panel.disambiguation.requires_selection {
            assert!(panel.disambiguation.has_disambiguation_path());
            assert!(panel.requires_inspection_before_jump());
            assert!(panel
                .labels
                .contains(&RelatedObjectLabel::DisambiguationRequired));
            for action in &panel.actions {
                if action.action_kind.navigates() {
                    assert!(
                        action.gated_by_disambiguation,
                        "navigating action {} must be gated when disambiguation is pending",
                        action.action_kind.as_str()
                    );
                }
            }
        }
    }
}

#[test]
fn unsupported_parity_is_labeled_with_no_links() {
    let fixture = load_fixture();
    let mut saw_unsupported = false;
    for scenario in &fixture.scenarios {
        let panel = &scenario.panel;
        if panel.anchor_parity == AnchorParity::AnchorsUnsupported {
            saw_unsupported = true;
            assert!(panel.groups.is_empty());
            assert_eq!(panel.totals.total_count, 0);
            assert!(panel
                .labels
                .contains(&RelatedObjectLabel::UnsupportedParity));
            assert!(!panel.downgrade_reasons.is_empty());
            assert!(!panel.parity_note.trim().is_empty());
        }
    }
    assert!(
        saw_unsupported,
        "the corpus must exercise an unsupported-parity panel"
    );
}

#[test]
fn captured_scope_is_always_disclosed() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let panel = &scenario.panel;
        if panel.totals.captured_scope_count > 0 {
            assert!(
                panel.captured_scope_ref.is_some() || !panel.downgrade_reasons.is_empty(),
                "captured panel {} must carry a captured scope ref or downgrade reason",
                scenario.scenario_id
            );
            assert!(!panel.attribution_notes.is_empty());
        }
    }
}

#[test]
fn actions_are_stable_across_routes() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let panel = &scenario.panel;
        for action_kind in RelatedObjectActionKind::ALL {
            let affordance = panel
                .actions
                .iter()
                .find(|a| a.action_kind == action_kind)
                .unwrap_or_else(|| panic!("missing action {}", action_kind.as_str()));
            assert_eq!(affordance.history_effect, action_kind.history_effect());
            assert!(affordance.preserves_anchor_identity);
            assert_eq!(affordance.anchor_ref, panel.anchor_ref);
            assert_eq!(affordance.available_routes.len(), 6);
        }
    }
}
