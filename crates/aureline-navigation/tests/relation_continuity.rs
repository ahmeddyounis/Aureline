//! Freeze gate for the relation-continuity corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/relation_continuity/canonical_continuity.json` is the published
//! corpus. This gate rebuilds the corpus in code and asserts it equals the fixture after
//! a serialize round-trip, so the relation-aware continuity contract cannot drift from
//! the published artifact without failing CI. It also re-proves that every stored packet
//! equals the builder's own output, that the corpus is support-export safe, that every
//! non-bound entry discloses its drift state and recovery before any jump, that bound
//! semantic entries auto-open while fallback and captured entries do not, that rename
//! evidence survives with its relation kind and replay id, and that every frozen
//! invariant holds. This test runs under `cargo test --workspace`, so stable promotion
//! cannot harden a navigation surface without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::relation_continuity::{
    build_relation_continuity_packet, relation_continuity_set, RelationContinuityEvidenceClass,
    RelationContinuitySet, RelationRecoveryChoice, RELATION_CONTINUITY_DRIFT_STATES,
    RELATION_CONTINUITY_RECORD_KIND, RELATION_CONTINUITY_SCHEMA_REF, RELATION_NAV_ENTRY_ORDER,
};
use aureline_navigation::target_model::{ContinuityState, RelationKind};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/relation_continuity/canonical_continuity.json")
}

fn load_fixture() -> RelationContinuitySet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = relation_continuity_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code relation-continuity corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_relation_continuity`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, RELATION_CONTINUITY_RECORD_KIND);
    assert_eq!(fixture.schema_ref, RELATION_CONTINUITY_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RelationContinuitySet =
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
fn every_stored_packet_equals_builder_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = build_relation_continuity_packet(&scenario.input);
        assert_eq!(
            produced, scenario.packet,
            "scenario {} drifted from the builder",
            scenario.scenario_id
        );
    }
}

#[test]
fn entries_preserve_relation_kind_and_return_context() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for entry in &scenario.packet.entries {
            assert_eq!(entry.captured_target.relation_kind, entry.relation_kind);
            assert!(!entry.return_anchor.return_anchor_ref.is_empty());
            assert!(entry.return_anchor.restores_selection);
            assert!(entry.return_anchor.restores_viewport);
            if let Some(current) = &entry.current_target {
                // A current target — including a remap — never relabels the relation kind.
                assert_eq!(current.relation_kind, entry.relation_kind);
            }
            // Replay-safe id is derived from the stable entry id.
            assert_eq!(
                entry.replay_target_id,
                format!("aureline://replay/relation-nav/{}", entry.entry_id)
            );
        }
    }
}

#[test]
fn non_bound_entries_disclose_state_before_any_jump() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for entry in &scenario.packet.entries {
            if entry.drift_state == ContinuityState::Bound {
                continue;
            }
            // The state shows before any jump: no auto-open, a visible reason, recovery.
            assert!(
                !entry.auto_open_allowed,
                "{} auto-opened a non-bound entry",
                entry.entry_id
            );
            assert!(entry.drift_reason.as_ref().is_some_and(|r| !r.is_empty()));
            assert!(!entry.recovery_choices.is_empty());
            assert!(!entry.used_nearby_fallback);
            match entry.drift_state {
                ContinuityState::Remapped => {
                    // A remap is by stable evidence and offers an explicit open action.
                    assert!(!entry.remap_evidence_refs.is_empty());
                    assert!(entry.current_target.is_some());
                    assert!(entry
                        .recovery_choices
                        .contains(&RelationRecoveryChoice::OpenRemappedTarget));
                }
                ContinuityState::Drifted
                | ContinuityState::MissingTarget
                | ContinuityState::ScopeUnavailable
                | ContinuityState::Archived => {
                    // No current target: the surface shows the state, not a nearby guess.
                    assert!(entry.current_target.is_none());
                }
                ContinuityState::Bound => unreachable!(),
            }
        }
    }
}

#[test]
fn counts_reconcile_and_current_vs_captured_is_honest() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let packet = &scenario.packet;
        assert!(packet.counts.reconciles());
        let computed_current = packet.entries.iter().filter(|e| e.current_scope).count();
        assert_eq!(packet.counts.current_scope_count, computed_current);
        for entry in &packet.entries {
            // Current scope only for a bound, live, semantic entry.
            let expected = entry.drift_state == ContinuityState::Bound
                && entry.evidence_class == RelationContinuityEvidenceClass::Semantic;
            if entry.current_scope {
                assert!(expected, "{} claimed current scope wrongly", entry.entry_id);
            }
        }
    }
}

#[test]
fn fallback_entries_never_auto_open_or_read_as_semantic() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for entry in &scenario.packet.entries {
            if entry.evidence_class.is_fallback() {
                assert!(!entry.current_scope);
                assert!(!entry.auto_open_allowed);
                assert!(!entry.fallback_notes.is_empty());
                assert!(!entry.downgrade_reasons.is_empty());
            }
        }
    }
}

#[test]
fn rename_evidence_survives_with_relation_kind_and_replay_id() {
    let fixture = load_fixture();
    let mut seen = 0usize;
    for scenario in &fixture.scenarios {
        for evidence in &scenario.packet.rename_evidence {
            seen += 1;
            assert_eq!(evidence.root_relation_kind, RelationKind::Definition);
            assert_eq!(
                evidence.replay_target_id,
                format!("aureline://replay/rename/{}", evidence.evidence_id)
            );
            assert!(!evidence.return_anchor.return_anchor_ref.is_empty());
            if evidence.ambiguity_class.requires_disambiguation() {
                assert!(evidence.disambiguation_set_ref.is_some());
            }
        }
    }
    assert!(seen >= 2, "corpus must exercise rename evidence");
}

#[test]
fn consumers_preserve_truth_without_retargeting() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        for projection in &scenario.packet.consumer_projections {
            assert!(
                projection.preserves_truth(),
                "{:?} dropped continuity truth",
                projection.consumer_surface
            );
            assert!(!projection.silently_retargets);
            assert!(!projection.exports_code_bodies);
        }
    }
}

#[test]
fn corpus_covers_drift_states_and_entry_kinds() {
    let fixture = load_fixture();
    let packets: Vec<_> = fixture.scenarios.iter().map(|s| &s.packet).collect();
    for state in RELATION_CONTINUITY_DRIFT_STATES {
        assert!(
            packets
                .iter()
                .any(|p| p.entries.iter().any(|e| e.drift_state == state)),
            "no entry covers drift state {state:?}"
        );
    }
    for kind in RELATION_NAV_ENTRY_ORDER {
        assert!(
            packets
                .iter()
                .any(|p| p.entries.iter().any(|e| e.entry_kind == kind)),
            "no entry covers kind {}",
            kind.as_str()
        );
    }
}
