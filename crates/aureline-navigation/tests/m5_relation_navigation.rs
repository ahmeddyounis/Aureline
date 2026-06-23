//! Freeze gate for the relation-navigation matrix.
//!
//! The checked-in fixture
//! `fixtures/navigation/m5-relation-navigation/canonical_matrix.json` is the
//! published matrix. This gate rebuilds the matrix in code and asserts it equals
//! the fixture after a serialize round-trip, so the relation-navigation contract
//! cannot drift from the published artifact without failing CI. It also re-proves
//! support-export safety, full object/state coverage, that every named controlled
//! vocabulary is bound, that every object maps to a proof packet, and every frozen
//! invariant. This test runs under `cargo test --workspace`, so stable promotion
//! cannot harden a relation-navigation claim without current proof.

use std::path::{Path, PathBuf};

use aureline_navigation::m5_relation_navigation::{
    relation_navigation_matrix, RelationNavObjectClass, RelationNavStateClass,
    RelationNavVocabulary, RelationNavigationMatrix, M5_RELATION_NAVIGATION_RECORD_KIND,
    M5_RELATION_NAVIGATION_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/navigation/m5-relation-navigation/canonical_matrix.json")
}

fn load_fixture() -> RelationNavigationMatrix {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_matrix_matches_checked_in_fixture() {
    let built = relation_navigation_matrix();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code relation-navigation matrix drifted from the checked-in fixture; \
         regenerate it with `cargo run -p aureline-navigation --example dump_m5_relation_navigation`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_RELATION_NAVIGATION_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_RELATION_NAVIGATION_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());
    fixture.validate().expect("fixture validates");

    let roundtrip: RelationNavigationMatrix =
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
fn matrix_covers_every_object_and_state() {
    let fixture = load_fixture();
    assert_eq!(fixture.objects.len(), RelationNavObjectClass::ALL.len());
    for object in RelationNavObjectClass::ALL {
        assert!(
            fixture.object(object).is_some(),
            "missing object {}",
            object.as_str()
        );
    }
    assert_eq!(
        fixture.state_vocabulary.len(),
        RelationNavStateClass::ALL.len()
    );
    for state in RelationNavStateClass::ALL {
        assert!(
            fixture.state_term(state).is_some(),
            "missing state {}",
            state.as_str()
        );
    }
}

#[test]
fn matrix_covers_all_six_relation_objects_explicitly() {
    let fixture = load_fixture();
    for object in [
        RelationNavObjectClass::NavigationTarget,
        RelationNavObjectClass::ReferenceOccurrence,
        RelationNavObjectClass::HierarchyEdge,
        RelationNavObjectClass::RelatedObjectRelation,
        RelationNavObjectClass::RenamePreviewSet,
        RelationNavObjectClass::RelationFallbackVocabulary,
    ] {
        let entry = fixture.object(object).expect("object present");
        assert!(!entry.canonical_schema_refs.is_empty());
        assert!(!entry.produced_by_refs.is_empty());
        assert!(!entry.proof_packet_ref.is_empty());
        assert!(!entry.required_fields.is_empty());
        assert!(!entry.relation_kinds.is_empty());
    }
}

#[test]
fn every_named_controlled_vocabulary_is_bound() {
    let fixture = load_fixture();
    for vocab in RelationNavVocabulary::ALL {
        assert!(
            fixture.objects.iter().any(|o| o.binds(vocab)),
            "controlled vocabulary {} is bound by no object",
            vocab.as_str()
        );
    }
}

#[test]
fn every_object_maps_to_a_proof_packet() {
    let fixture = load_fixture();
    for object in &fixture.objects {
        assert!(
            !object.proof_packet_ref.is_empty(),
            "object {} lacks a mapped proof packet",
            object.object.as_str()
        );
    }
}

#[test]
fn every_navigable_object_binds_proof_class() {
    let fixture = load_fixture();
    for object in fixture.objects.iter() {
        assert!(
            object.binds(RelationNavVocabulary::ProofClassAxis) && object.proof_class_required,
            "object {} must carry an explicit proof class",
            object.object.as_str()
        );
    }
}
