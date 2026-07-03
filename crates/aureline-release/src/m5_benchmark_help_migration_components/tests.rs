//! Inline tests for benchmark evidence-card component truth.

use super::*;
use std::collections::BTreeSet;

fn cards() -> Vec<BenchmarkEvidenceCard> {
    current_benchmark_evidence_cards().expect("fixtures parse")
}

#[test]
fn embedded_benchmark_cards_parse_and_validate() {
    let cards = cards();
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.is_empty(),
        "unexpected benchmark card violations: {violations:#?}"
    );
}

#[test]
fn canonical_card_exposes_workflow_budget_environment_and_freshness_truth() {
    let card = current_benchmark_evidence_card().expect("canonical card parses");
    assert_eq!(card.record_kind, M5_BENCHMARK_EVIDENCE_CARD_RECORD_KIND);
    assert_eq!(
        card.schema_version,
        M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_VERSION
    );
    assert_eq!(
        card.evidence_source_class,
        BenchmarkEvidenceSourceClass::LabReferenceRun
    );
    assert!(!card.workflow_ref.is_empty());
    assert!(!card.budget_ref.is_empty());
    assert!(!card.measured_value_repr.is_empty());
    assert!(!card.budget_value_repr.is_empty());
    assert!(!card.corpus_ref.is_empty());
    assert!(!card.hardware_or_capture_ref.is_empty());
    assert!(card.sample_size > 0);
    assert!(card.downgrade_banner.shown);
    assert!(card.trace_report_export.includes_workflow_budget_truth);
    assert!(card.trace_report_export.includes_environment_truth);
}

#[test]
fn fixture_set_proves_all_required_source_classes() {
    let classes: BTreeSet<_> = cards()
        .iter()
        .map(|card| card.evidence_source_class)
        .collect();
    for required in BenchmarkEvidenceSourceClass::REQUIRED_PROOF_CLASSES {
        assert!(
            classes.contains(&required),
            "missing benchmark evidence source class {required:?}"
        );
    }
}

#[test]
fn non_reference_source_classes_are_narrowed_and_caveated() {
    for card in cards()
        .iter()
        .filter(|card| card.evidence_source_class != BenchmarkEvidenceSourceClass::LabReferenceRun)
    {
        assert!(card.downgrade_banner.shown, "{}", card.card_id);
        assert!(!card.degraded_state.is_none(), "{}", card.card_id);
        assert!(!card.compare_view.comparable, "{}", card.card_id);
        assert!(
            !card.caveat_summary_refs.is_empty(),
            "{} missing caveats",
            card.card_id
        );
    }
}

#[test]
fn copy_export_preserves_benchmark_id_and_caveats() {
    for card in cards() {
        let copy = format!(
            "{}\n{}\n{}",
            card.copy_export.text, card.copy_export.json, card.copy_export.markdown
        );
        assert!(copy.contains(&card.benchmark_id), "{}", card.card_id);
        for caveat in &card.caveat_summary_refs {
            assert!(copy.contains(caveat), "{} missing {caveat}", card.card_id);
        }
    }
}

#[test]
fn dropping_benchmark_id_from_copy_fails_validation() {
    let mut card = current_benchmark_evidence_card().expect("canonical card parses");
    card.copy_export.text.clear();
    card.copy_export.json.clear();
    card.copy_export.markdown.clear();
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::CopyExportDropsBenchmarkId { .. }
        )),
        "expected benchmark-id export violation, got {violations:#?}"
    );
}

#[test]
fn hiding_required_downgrade_banner_fails_validation() {
    let mut card = current_benchmark_evidence_card().expect("canonical card parses");
    card.downgrade_banner.shown = false;
    let violations = card.validate();
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::MissingDowngradeBanner { .. }
        )),
        "expected missing-banner violation, got {violations:#?}"
    );
}

#[test]
fn missing_source_class_coverage_fails_validation() {
    let cards: Vec<_> = cards()
        .into_iter()
        .filter(|card| card.evidence_source_class != BenchmarkEvidenceSourceClass::ImportedEvidence)
        .collect();
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.iter().any(|violation| matches!(
            violation,
            BenchmarkEvidenceCardViolation::MissingEvidenceSourceClass {
                source_class: BenchmarkEvidenceSourceClass::ImportedEvidence
            }
        )),
        "expected imported-evidence coverage violation, got {violations:#?}"
    );
}
