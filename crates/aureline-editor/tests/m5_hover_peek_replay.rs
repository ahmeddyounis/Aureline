//! Freeze gate for the canonical hover-card and documentation-peek model.
//!
//! The checked-in fixture `fixtures/editor/m5-hover-peek/canonical_model.json` is
//! the published model. This gate rebuilds the model in code and asserts it equals
//! the fixture after a serialize round-trip, so the in-code model cannot drift from
//! the published artifact without failing CI. It also re-proves every frozen
//! invariant, support-export safety, the never-silently-retarget contract, the
//! provenance-preserving promotion contract, the raw-versus-rendered escape
//! contract, and the inline non-live state contract.

use std::path::{Path, PathBuf};

use aureline_editor::{
    hover_peek_model, hover_peek_model_lines, HoverPeekContextClass, HoverPeekModel,
    HoverPeekStateClass, RawRenderedModeClass, M5_HOVER_PEEK_RECORD_KIND, M5_HOVER_PEEK_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-hover-peek/canonical_model.json")
}

fn load_fixture() -> HoverPeekModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = hover_peek_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code hover-peek model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_hover_peek`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_HOVER_PEEK_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_HOVER_PEEK_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: HoverPeekModel =
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
fn cards_are_reachable_provenance_labeled_and_never_retarget() {
    let fixture = load_fixture();
    for card in fixture.all_cards() {
        assert!(
            card.keyboard_invocable,
            "card {} must be reachable",
            card.card_id
        );
        assert!(
            card.provenance_labeled(),
            "card {} must carry provenance",
            card.card_id
        );
        assert!(
            card.target_identity_locked(),
            "card {} must not silently retarget",
            card.card_id
        );
    }
}

#[test]
fn non_live_states_disclosed_and_promotions_preserve_provenance() {
    let fixture = load_fixture();
    for card in fixture.all_cards() {
        assert!(
            card.non_live_state_disclosed(),
            "card {} must disclose its non-live state inline",
            card.card_id
        );
        assert!(
            card.promotions_preserve_provenance_and_continuity(),
            "card {} promotions must preserve provenance and return anchor",
            card.card_id
        );
        assert!(
            card.offers_all_promotion_paths(),
            "card {} must offer every promotion path when it has content",
            card.card_id
        );
        assert!(
            card.raw_escape_when_distinct(),
            "card {} must offer an open-raw escape when raw and rendered differ",
            card.card_id
        );
    }
}

#[test]
fn raw_rendered_and_wrong_provider_contracts_hold() {
    let fixture = load_fixture();
    let request = fixture
        .snapshot(HoverPeekContextClass::RequestEditor)
        .expect("request");
    assert_eq!(
        request.card.raw_rendered_mode,
        RawRenderedModeClass::RawAndRenderedDistinct
    );
    assert!(request.card.raw_escape_command_id_ref.is_some());

    let sql = fixture
        .snapshot(HoverPeekContextClass::SqlEditor)
        .expect("sql");
    assert_eq!(
        sql.card.state_class,
        HoverPeekStateClass::WrongProviderFallback
    );
    assert!(!sql.card.is_live());
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = hover_peek_model_lines(&fixture);
    assert!(lines.iter().any(|line| line.contains("Hover-peek model")));
    assert!(lines.iter().any(|line| line.contains("Context snapshots:")));
    for snapshot in &fixture.context_snapshots {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(snapshot.context_class.as_str())),
            "projection must mention context {}",
            snapshot.context_class.as_str()
        );
    }
}
