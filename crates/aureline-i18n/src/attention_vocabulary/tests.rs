//! Unit tests for the governed attention/lifecycle vocabulary glossary.

use std::collections::BTreeSet;

use super::*;

#[test]
fn seeded_glossary_validates() {
    let glossary = seeded_attention_vocabulary_glossary();
    glossary.validate().expect("seeded glossary validates");
}

#[test]
fn seeded_glossary_is_drift_free() {
    let glossary = seeded_attention_vocabulary_glossary();
    assert!(
        glossary.audit_findings().is_empty(),
        "seeded glossary must carry no drift: {:?}",
        glossary.audit_findings()
    );
}

#[test]
fn every_domain_is_governed() {
    let glossary = seeded_attention_vocabulary_glossary();
    let domains: BTreeSet<_> = glossary.terms.iter().map(|t| t.domain).collect();
    for domain in AttentionTermDomain::all() {
        assert!(domains.contains(&domain), "missing domain {domain:?}");
    }
}

#[test]
fn every_term_translates_into_every_claimed_locale() {
    let glossary = seeded_attention_vocabulary_glossary();
    for term in &glossary.terms {
        for locale in &glossary.claimed_locales {
            assert!(
                term.translation(locale).is_some(),
                "{} missing {locale}",
                term.term_key
            );
        }
    }
}

#[test]
fn term_keys_are_unique() {
    let glossary = seeded_attention_vocabulary_glossary();
    let mut seen = BTreeSet::new();
    for term in &glossary.terms {
        assert!(
            seen.insert(&term.term_key),
            "duplicate key {}",
            term.term_key
        );
    }
}

#[test]
fn shared_source_words_translate_identically() {
    // "Running" appears as a lifecycle state and a badge count; the governed
    // vocabulary must translate it the same way on both surfaces.
    let glossary = seeded_attention_vocabulary_glossary();
    let lifecycle = glossary.term("attention.lifecycle.running").unwrap();
    let badge = glossary
        .term("attention.badge.durable_running_count")
        .unwrap();
    assert_eq!(lifecycle.source_term, badge.source_term);
    for locale in &glossary.claimed_locales {
        assert_eq!(
            lifecycle.translation(locale).unwrap().localized_term,
            badge.translation(locale).unwrap().localized_term,
            "running term drifts across surfaces in {locale}"
        );
    }
}

#[test]
fn arabic_terms_render_right_to_left() {
    let glossary = seeded_attention_vocabulary_glossary();
    for term in &glossary.terms {
        let ar = term.translation("ar-SA").unwrap();
        assert_eq!(ar.text_direction, TextDirection::RightToLeft);
    }
}

#[test]
fn severity_rank_is_locale_neutral_metadata() {
    let glossary = seeded_attention_vocabulary_glossary();
    for term in &glossary.terms {
        for tr in &term.translations {
            assert_eq!(
                tr.asserted_severity_rank, term.severity_rank,
                "{} severity drifted in {}",
                term.term_key, tr.locale
            );
        }
    }
}

#[test]
fn parity_report_is_green() {
    let report = seeded_attention_vocabulary_parity_report();
    report.validate().expect("parity report validates");
    assert_eq!(report.summary.parity_state, "green");
    assert_eq!(report.summary.blocked_locale_count, 0);
    for row in &report.locale_rows {
        assert!(row.is_governed(), "{} not green", row.locale);
        assert!(row.all_severity_ranks_preserved);
        assert!(row.action_order_stable);
        assert!(row.lexical_consistency_holds);
        assert!(row.no_term_collisions);
        assert!(row.all_severities_preserved_under_truncation);
    }
}

#[test]
fn domain_action_order_is_dense() {
    let report = seeded_attention_vocabulary_parity_report();
    for row in &report.domain_rows {
        assert!(row.action_order_dense, "{:?} not dense", row.domain);
    }
}

#[test]
fn drift_scenarios_are_caught() {
    let glossary = seeded_attention_vocabulary_glossary();
    let scenarios = seeded_attention_vocabulary_drift_scenarios();
    scenarios
        .validate_against(&glossary)
        .expect("every drift scenario surfaces its class");
}

#[test]
fn softening_severity_is_blocked() {
    // The guardrail: a translation cannot make a failed job read softer.
    let mut glossary = seeded_attention_vocabulary_glossary();
    let term = glossary
        .terms
        .iter_mut()
        .find(|t| t.term_key == "attention.lifecycle.failed")
        .unwrap();
    term.translations
        .iter_mut()
        .find(|t| t.locale == "es-MX")
        .unwrap()
        .asserted_severity_rank = AttentionSeverityRank::Success;

    let findings = glossary.audit_findings();
    assert!(findings
        .iter()
        .any(|f| f.class == AttentionDriftClass::SeverityRankAltered));

    let report = build_attention_vocabulary_parity_report(&glossary);
    assert_eq!(report.summary.parity_state, "blocked");
    let row = report.locale_row("es-MX").unwrap();
    assert!(!row.all_severity_ranks_preserved);
    assert_eq!(row.claim_state, "blocked");
}

#[test]
fn fallback_narrows_but_does_not_block() {
    let mut glossary = seeded_attention_vocabulary_glossary();
    let tr = glossary.terms[0]
        .translations
        .iter_mut()
        .find(|t| t.locale == "ja-JP")
        .unwrap();
    tr.localization_state = LocalizationRenderState::SourceLanguageFallback;

    let report = build_attention_vocabulary_parity_report(&glossary);
    let row = report.locale_row("ja-JP").unwrap();
    assert_eq!(row.claim_state, "narrowed");
    assert_eq!(row.source_fallback_terms, 1);
    assert_eq!(report.summary.parity_state, "narrowed");
}

#[test]
fn drift_scenario_set_round_trips_through_json() {
    let scenarios = seeded_attention_vocabulary_drift_scenarios();
    let json = serde_json::to_string(&scenarios).unwrap();
    let back: AttentionVocabularyDriftScenarioSet = serde_json::from_str(&json).unwrap();
    assert_eq!(scenarios, back);
}
