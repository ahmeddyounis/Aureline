//! Freeze gate for the canonical signature-help and snippet-session model.
//!
//! The checked-in fixture
//! `fixtures/editor/m5-signature-snippet/canonical_model.json` is the published
//! model. This gate rebuilds the model in code and asserts it equals the fixture
//! after a serialize round-trip, so the in-code model cannot drift from the
//! published artifact without failing CI. It also re-proves every frozen
//! invariant, support-export safety, the signature input-meaning contract, the
//! snippet exit-path / Tab-capture contract, the IME / multi-cursor coherence
//! contract, and the no-hidden-side-effects disclosure contract.

use std::path::{Path, PathBuf};

use aureline_editor::{
    signature_snippet_model, signature_snippet_model_lines, AcceptSideEffectClass,
    SignatureSnippetModel, M5_SIGNATURE_SNIPPET_RECORD_KIND, M5_SIGNATURE_SNIPPET_SCHEMA_REF,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/m5-signature-snippet/canonical_model.json")
}

fn load_fixture() -> SignatureSnippetModel {
    let raw = std::fs::read_to_string(fixture_path())
        .unwrap_or_else(|err| panic!("fixture must read: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("fixture must parse: {err}"))
}

#[test]
fn in_code_model_matches_checked_in_fixture() {
    let built = signature_snippet_model();
    let fixture = load_fixture();
    assert_eq!(
        built, fixture,
        "the in-code signature-snippet model drifted from the checked-in fixture; \
         regenerate it with `cargo run --bin aureline_m5_signature_snippet`"
    );
}

#[test]
fn fixture_round_trips_and_is_export_safe() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, M5_SIGNATURE_SNIPPET_RECORD_KIND);
    assert_eq!(fixture.schema_ref, M5_SIGNATURE_SNIPPET_SCHEMA_REF);
    assert!(fixture.raw_payload_excluded);
    assert!(fixture.is_support_export_safe());

    let roundtrip: SignatureSnippetModel =
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
fn signature_cards_expose_input_meaning() {
    let fixture = load_fixture();
    for card in fixture.all_cards() {
        assert!(!card.obscures_active_line);
        if card.is_visible() {
            assert!(
                card.exposes_active_parameter(),
                "visible card {} must expose its active parameter",
                card.card_id
            );
            assert!(card.is_typing_loop_safe());
        }
    }
}

#[test]
fn active_snippets_expose_exit_path_and_never_hijack_tab() {
    let fixture = load_fixture();
    for strip in fixture.all_strips() {
        assert!(
            strip.exposes_exit_path(),
            "strip {} must expose its exit path",
            strip.strip_id
        );
        assert!(
            strip.does_not_hijack_tab(),
            "strip {} must not hijack Tab invisibly",
            strip.strip_id
        );
        assert!(
            strip.ime_and_multicursor_coherent_or_degraded(),
            "strip {} must stay coherent or degrade explicitly",
            strip.strip_id
        );
    }
}

#[test]
fn accept_side_effects_disclose_before_commit() {
    let fixture = load_fixture();
    let cards = fixture
        .all_cards()
        .map(|card| {
            (
                card.accept_side_effect,
                card.commit_disclosure_required,
                card.side_effect_summary.is_some(),
                card.preview_required,
            )
        })
        .collect::<Vec<_>>();
    let strips = fixture
        .all_strips()
        .map(|strip| {
            (
                strip.accept_side_effect,
                strip.commit_disclosure_required,
                strip.side_effect_summary.is_some(),
                strip.preview_required,
            )
        })
        .collect::<Vec<_>>();
    for (effect, disclose, has_summary, preview) in cards.into_iter().chain(strips) {
        if effect.requires_pre_commit_disclosure() {
            assert!(
                disclose,
                "side effect {:?} must disclose before commit",
                effect
            );
            assert!(has_summary || preview);
        }
        if matches!(
            effect,
            AcceptSideEffectClass::AddsGeneratedScaffolding | AcceptSideEffectClass::AddsDependency
        ) {
            assert!(preview, "side effect {:?} must require preview", effect);
        }
    }
}

#[test]
fn human_readable_projection_renders_for_support() {
    let fixture = load_fixture();
    let lines = signature_snippet_model_lines(&fixture);
    assert!(lines
        .iter()
        .any(|line| line.contains("Signature-snippet model")));
    assert!(lines.iter().any(|line| line.contains("Surface snapshots:")));
    for snapshot in &fixture.surface_snapshots {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(snapshot.surface_class.as_str())),
            "projection must mention surface {}",
            snapshot.surface_class.as_str()
        );
    }
}
