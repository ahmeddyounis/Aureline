//! Freeze gate for the relation-resolution corpus.
//!
//! The checked-in fixture
//! `fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json`
//! is the published corpus. This gate rebuilds the corpus in code and asserts it
//! equals the fixture after a serialize round-trip, so the no-silent-aliasing
//! resolution contract cannot drift from the published artifact without failing
//! CI. It also re-proves that every stored resolution equals the resolver's own
//! output, that the corpus is support-export safe, that Go to Definition,
//! Declaration, and Implementation each resolve distinctly, and that every frozen
//! invariant holds. This test runs under `cargo test --workspace`, so stable
//! promotion cannot harden a relation-resolution claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::relation_resolution::{
    relation_resolution_set, resolve_navigation, AliasingPosture, NavigationCommand,
    RelationResolutionSet, ResolutionDisposition, RELATION_RESOLUTION_RECORD_KIND,
    RELATION_RESOLUTION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/relation_navigation_resolution/canonical_resolutions.json")
}

fn load_fixture() -> RelationResolutionSet {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_corpus_matches_checked_in_fixture() {
    let built = relation_resolution_set();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code relation-resolution corpus drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_relation_navigation_resolution`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, RELATION_RESOLUTION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, RELATION_RESOLUTION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RelationResolutionSet =
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
fn every_stored_resolution_equals_resolver_output() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let produced = resolve_navigation(&scenario.request);
        assert_eq!(
            produced, scenario.resolution,
            "scenario {} drifted from the resolver",
            scenario.scenario_id
        );
        assert!(
            scenario.resolution.is_silent_alias_free(),
            "scenario {} is not silent-alias free",
            scenario.scenario_id
        );
    }
}

#[test]
fn definition_declaration_implementation_resolve_distinctly() {
    let fixture = load_fixture();
    for command in [
        NavigationCommand::GoToDefinition,
        NavigationCommand::GoToDeclaration,
        NavigationCommand::GoToImplementation,
    ] {
        let resolved = fixture.scenarios.iter().any(|scenario| {
            let resolution = &scenario.resolution;
            resolution.command == command
                && resolution.disposition == ResolutionDisposition::ResolvedSingle
                && resolution.aliasing_posture == AliasingPosture::NoAlias
                && resolution
                    .selected_target
                    .as_ref()
                    .is_some_and(|target| target.relation_kind() == command.requested_relation())
        });
        assert!(
            resolved,
            "no distinct single-target resolution for {}",
            command.as_str()
        );
    }
}

#[test]
fn disclosed_fallback_never_relabels_relation_kind() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let resolution = &scenario.resolution;
        if resolution.aliasing_posture == AliasingPosture::DisclosedFallback {
            assert!(
                resolution.navigated_other_relation()
                    || resolution.disposition == ResolutionDisposition::OpenedDisambiguation,
                "disclosed fallback {} must preserve a non-requested relation kind",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn multi_target_resolutions_open_a_disambiguation_set() {
    let fixture = load_fixture();
    for scenario in &fixture.scenarios {
        let resolution = &scenario.resolution;
        if resolution.ambiguity_count >= 2 {
            assert_eq!(
                resolution.disposition,
                ResolutionDisposition::OpenedDisambiguation,
                "multi-target scenario {} must open disambiguation",
                scenario.scenario_id
            );
            assert!(resolution.selected_target.is_none());
        }
    }
}
