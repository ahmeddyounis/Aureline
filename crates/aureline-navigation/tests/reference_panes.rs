//! Freeze gate for the references-pane corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/reference_panes/canonical_panes.json` is the published
//! corpus. This gate rebuilds the corpus in code and asserts it equals the fixture
//! after a serialize round-trip, so the references-pane contract cannot drift from
//! the published artifact without failing CI. It also re-proves that every stored
//! pane equals the builder's own output, that the corpus is support-export safe,
//! that every pane groups occurrences by access kind, separates current-versus-
//! captured counts, discloses fallbacks, exposes the four stable actions, and that
//! every frozen invariant holds. This test runs under `cargo test --workspace`, so
//! stable promotion cannot harden a references-pane claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::reference_panes::{
    build_reference_pane, reference_panes_set, PaneActionKind, ReferenceEvidenceClass,
    ReferencePaneSet, REFERENCE_ACCESS_KIND_ORDER, REFERENCE_PANES_RECORD_KIND,
    REFERENCE_PANES_SCHEMA_REF,
};
use aureline_navigation::target_model::RelationKind;

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/reference_panes/canonical_panes.json")
}

fn load_fixture() -> ReferencePaneSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = reference_panes_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code references-pane corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_reference_panes`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, REFERENCE_PANES_RECORD_KIND);
    assert_eq!(fixture.schema_ref, REFERENCE_PANES_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: ReferencePaneSet =
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
fn every_stored_pane_equals_builder_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = build_reference_pane(&scenario.input);
        assert_eq!(
            produced, scenario.pane,
            "scenario {} drifted from the builder",
            scenario.scenario_id
        );
        assert_eq!(produced.root_relation_kind, RelationKind::Reference);
    }
}

#[test]
fn every_pane_groups_by_access_kind_without_flattening() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let pane = &scenario.pane;
        // Every occurrence lands in exactly the group for its access kind.
        for occurrence in &scenario.input.occurrences {
            let group = pane
                .group(occurrence.access_kind)
                .unwrap_or_else(|| panic!("group missing for {}", occurrence.access_kind.as_str()));
            assert!(
                group.occurrence_refs.contains(&occurrence.occurrence_id),
                "occurrence {} not grouped under its access kind",
                occurrence.occurrence_id
            );
        }
        // Groups stay in canonical access-kind order.
        let order = |access_kind| {
            REFERENCE_ACCESS_KIND_ORDER
                .iter()
                .position(|candidate| *candidate == access_kind)
                .unwrap()
        };
        for pair in pane.groups.windows(2) {
            assert!(order(pair[0].access_kind) < order(pair[1].access_kind));
        }
    }
}

#[test]
fn scope_counts_reconcile_current_and_captured() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let pane = &scenario.pane;
        assert!(pane.totals.reconciles());
        let grouped: usize = pane.groups.iter().map(|g| g.counts.total_count).sum();
        assert_eq!(grouped, pane.totals.total_count);
        for group in &pane.groups {
            assert!(group.counts.reconciles());
        }
    }
}

#[test]
fn lexical_fallback_never_appears_as_semantic() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for group in &scenario.pane.groups {
            if group.evidence_class.is_fallback() {
                assert!(
                    !group.fallback_notes.is_empty() && !group.downgrade_reasons.is_empty(),
                    "fallback group in {} must carry a fallback note and downgrade reason",
                    scenario.scenario_id
                );
                assert_ne!(group.evidence_class, ReferenceEvidenceClass::Semantic);
            }
        }
    }
}

#[test]
fn actions_are_stable_across_routes() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let pane = &scenario.pane;
        for action_kind in PaneActionKind::ALL {
            let affordance = pane
                .actions
                .iter()
                .find(|a| a.action_kind == action_kind)
                .unwrap_or_else(|| panic!("missing action {}", action_kind.as_str()));
            assert_eq!(affordance.history_effect, action_kind.history_effect());
            assert!(affordance.preserves_target_identity);
            assert_eq!(affordance.target_ref, pane.root_target_ref);
            assert_eq!(affordance.available_routes.len(), 4);
        }
    }
}

#[test]
fn captured_scope_is_always_disclosed() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let pane = &scenario.pane;
        if pane.totals.captured_scope_count > 0 {
            assert!(
                pane.captured_scope_ref.is_some() || !pane.downgrade_reasons.is_empty(),
                "captured pane {} must carry a captured scope ref or downgrade reason",
                scenario.scenario_id
            );
            assert!(!pane.fallback_notes.is_empty());
        }
    }
}
