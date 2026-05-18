//! Fixture replay for the dense beta pseudoloc, RTL, bidi, IME, and expansion corpus.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_i18n::{
    seeded_dense_i18n_conformance_corpus, seeded_dense_i18n_conformance_review_packet,
    DenseI18nAssertionClass, DenseI18nConformanceCorpus, DenseI18nConformanceReviewPacket,
    DenseI18nLaneCadence, DenseI18nStressMode, ImeCompositionChurnEvent,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/i18n/m3/pseudoloc_rtl_ime_corpus")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn manifest_fixture_matches_seeded_dense_corpus() {
    let from_file: DenseI18nConformanceCorpus = load_json("manifest.json");
    let from_code = seeded_dense_i18n_conformance_corpus();

    assert_eq!(from_file, from_code);
    from_file.validate().expect("dense i18n corpus validates");
}

#[test]
fn review_export_matches_seeded_packet() {
    let from_file: DenseI18nConformanceReviewPacket = load_json("review_export.json");
    let from_code = seeded_dense_i18n_conformance_review_packet();

    assert_eq!(from_file, from_code);
    assert_eq!(from_file.active_waiver_count, 0);
    assert!(from_file
        .rows
        .iter()
        .all(|row| row.result_state == "passed"));
}

#[test]
fn corpus_is_bound_to_nightly_and_release_candidate_lanes() {
    let corpus = seeded_dense_i18n_conformance_corpus();
    let cadences = corpus
        .lane_bindings
        .iter()
        .map(|lane| lane.cadence)
        .collect::<BTreeSet<_>>();

    assert!(cadences.contains(&DenseI18nLaneCadence::Nightly));
    assert!(cadences.contains(&DenseI18nLaneCadence::ReleaseCandidate));
    assert!(corpus
        .lane_bindings
        .iter()
        .filter(|lane| {
            matches!(
                lane.cadence,
                DenseI18nLaneCadence::Nightly | DenseI18nLaneCadence::ReleaseCandidate
            )
        })
        .all(|lane| lane.release_blocking_for_claimed_surfaces
            && lane.command.contains("pseudoloc_rtl_ime_corpus")));
}

#[test]
fn dense_surfaces_exercise_required_stress_modes() {
    let corpus = seeded_dense_i18n_conformance_corpus();
    let modes = corpus
        .surface_cases
        .iter()
        .flat_map(|case| case.stress_modes.iter().copied())
        .collect::<BTreeSet<_>>();

    for required in [
        DenseI18nStressMode::PseudolocExpansion,
        DenseI18nStressMode::RtlChrome,
        DenseI18nStressMode::MixedDirectionTechnicalText,
        DenseI18nStressMode::ImeComposition,
        DenseI18nStressMode::CjkFontFallback,
        DenseI18nStressMode::TextExpansion,
        DenseI18nStressMode::LocaleFallbackReview,
        DenseI18nStressMode::TranslatedSurfaceParity,
        DenseI18nStressMode::LocalizedDateNumberFormatting,
    ] {
        assert!(modes.contains(&required), "missing {required:?}");
    }
}

#[test]
fn ime_cases_cover_focus_completion_snippet_and_command_preview_churn() {
    let corpus = seeded_dense_i18n_conformance_corpus();
    let events = corpus
        .surface_cases
        .iter()
        .flat_map(|case| &case.ime_scenarios)
        .flat_map(|scenario| scenario.churn_events.iter().copied())
        .collect::<BTreeSet<_>>();

    for required in [
        ImeCompositionChurnEvent::FocusChange,
        ImeCompositionChurnEvent::CompletionPreview,
        ImeCompositionChurnEvent::SnippetTraversal,
        ImeCompositionChurnEvent::CommandPreview,
    ] {
        assert!(events.contains(&required), "missing {required:?}");
    }

    assert!(corpus
        .surface_cases
        .iter()
        .flat_map(|case| &case.ime_scenarios)
        .all(|scenario| scenario.silent_commit_forbidden
            && scenario.silent_cancel_forbidden
            && scenario.candidate_and_caret_visibility_required));
}

#[test]
fn literal_tokens_remain_unmirrored_and_copy_safe() {
    let corpus = seeded_dense_i18n_conformance_corpus();

    for case in &corpus.surface_cases {
        for token in &case.literal_tokens {
            assert!(
                token.must_remain_unmirrored && token.copy_raw_required,
                "{} token {} must remain literal",
                case.case_id,
                token.token
            );
            if case
                .stress_modes
                .contains(&DenseI18nStressMode::MixedDirectionTechnicalText)
                || case.stress_modes.contains(&DenseI18nStressMode::RtlChrome)
            {
                assert!(
                    token.copy_escaped_required,
                    "{} token {} must keep escaped copy",
                    case.case_id, token.token
                );
            }
        }
    }
}

#[test]
fn translated_surface_assertions_preserve_source_language_escape_hatches() {
    let corpus = seeded_dense_i18n_conformance_corpus();
    let asserted_refs = corpus
        .translated_surface_assertions
        .iter()
        .flat_map(|assertion| {
            assertion
                .preserved_refs
                .iter()
                .map(|item| item.ref_kind.as_str())
        })
        .collect::<BTreeSet<_>>();

    for required in [
        "command_id",
        "keyboard_path",
        "citation_anchor",
        "scope_label",
    ] {
        assert!(asserted_refs.contains(required), "missing {required}");
    }

    assert!(corpus
        .translated_surface_assertions
        .iter()
        .all(|assertion| {
            assertion.open_in_source_language_required
                && assertion.machine_output_locale_neutral
                && assertion.governed_terminology_preserved
        }));
    assert!(corpus.surface_cases.iter().any(|case| case
        .assertion_refs
        .contains(&DenseI18nAssertionClass::LocaleFallbackDisclosedAndNonBlocking)));
}
