//! Fixture replay for the governed attention/lifecycle vocabulary glossary.
//!
//! Confirms the checked-in glossary, parity report, and drift scenario fixtures
//! match the seeded truth, that the parity audit derives from the glossary, and
//! that a softened severity, reordered action, broken lexical consistency, or
//! hidden severity under truncation is caught before a localized profile claims
//! green.

use std::path::{Path, PathBuf};

use aureline_i18n::{
    build_attention_vocabulary_parity_report, seeded_attention_vocabulary_drift_scenarios,
    seeded_attention_vocabulary_glossary, seeded_attention_vocabulary_parity_report,
    AttentionDriftClass, AttentionSeverityRank, AttentionTermDomain,
    AttentionVocabularyDriftScenarioSet, AttentionVocabularyGlossary,
    AttentionVocabularyParityReport,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/i18n/activity-center-and-notifications")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn glossary_fixture_matches_seeded_packet() {
    let from_file: AttentionVocabularyGlossary = load_json("glossary.json");
    let from_code = seeded_attention_vocabulary_glossary();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("glossary validates");
}

#[test]
fn parity_fixture_matches_seeded_report() {
    let from_file: AttentionVocabularyParityReport = load_json("parity_report.json");
    let from_code = seeded_attention_vocabulary_parity_report();
    assert_eq!(from_file, from_code);
    from_file.validate().expect("parity report validates");
    assert_eq!(from_file.summary.parity_state, "green");
}

#[test]
fn drift_fixture_matches_seeded_scenarios() {
    let from_file: AttentionVocabularyDriftScenarioSet = load_json("drift_scenarios.json");
    let from_code = seeded_attention_vocabulary_drift_scenarios();
    assert_eq!(from_file, from_code);

    let glossary = seeded_attention_vocabulary_glossary();
    from_file
        .validate_against(&glossary)
        .expect("every drift scenario surfaces its class");
}

#[test]
fn every_governed_domain_is_present() {
    let glossary = seeded_attention_vocabulary_glossary();
    for domain in AttentionTermDomain::all() {
        assert!(
            glossary.terms.iter().any(|t| t.domain == domain),
            "missing {domain:?}"
        );
    }
}

#[test]
fn softened_severity_blocks_the_locale() {
    // The guardrail: translation cannot make a failed job read softer.
    let mut glossary = seeded_attention_vocabulary_glossary();
    let term = glossary
        .terms
        .iter_mut()
        .find(|t| t.term_key == "attention.lifecycle.failed")
        .unwrap();
    term.translations
        .iter_mut()
        .find(|t| t.locale == "ar-SA")
        .unwrap()
        .asserted_severity_rank = AttentionSeverityRank::Informational;

    let report = build_attention_vocabulary_parity_report(&glossary);
    assert_eq!(report.summary.parity_state, "blocked");
    let row = report.locale_row("ar-SA").unwrap();
    assert!(!row.all_severity_ranks_preserved);
    assert!(report
        .drift_findings
        .iter()
        .any(|f| f.class == AttentionDriftClass::SeverityRankAltered));
}
