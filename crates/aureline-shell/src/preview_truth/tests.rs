//! Fixture-driven tests for retained preview-truth records.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct PreviewTruthFixture {
    record: PreviewTruthRecord,
    expect: PreviewTruthFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct PreviewTruthFixtureExpect {
    valid: bool,
    required_violation_tokens: Vec<String>,
    required_plaintext_snippets: Vec<String>,
    required_support_tokens: Vec<String>,
}

fn fixture(name: &str) -> PreviewTruthFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/notebook/m3/trust_repair_roundtrip")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

#[test]
fn notebook_preview_separates_trust_risk_and_repair_lineage() {
    let fixture = fixture("notebook_mixed_trust_preview.yaml");
    let record = fixture.record.normalized();
    let violations = record.validate();

    assert!(fixture.expect.valid);
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
    assert!(record.is_valid());

    let support = record.support_export();
    assert!(support.violation_tokens.is_empty());
    assert!(support
        .trust_layer_tokens
        .iter()
        .any(|token| token.starts_with("document:")));
    assert!(support
        .trust_layer_tokens
        .iter()
        .any(|token| token.starts_with("runtime:")));
    assert!(support
        .trust_layer_tokens
        .iter()
        .any(|token| token.starts_with("output:")));
    assert!(support
        .round_trip_risk_tokens
        .iter()
        .any(|token| token.contains("lossy_output_representation")));
    assert!(support
        .repair_lineage_refs
        .iter()
        .any(|token| token.contains("rerun:checkpoint:notebook:before_rerun")));
    assert!(support
        .repair_lineage_refs
        .iter()
        .any(|token| token.contains("output_clear:checkpoint:notebook:before_output_clear")));

    for snippet in &fixture.expect.required_plaintext_snippets {
        assert!(
            support.plaintext_summary.contains(snippet),
            "support export plaintext should contain {snippet:?}\n{}",
            support.plaintext_summary
        );
    }
    for token in &fixture.expect.required_support_tokens {
        assert!(
            support
                .trust_layer_tokens
                .iter()
                .chain(support.round_trip_risk_tokens.iter())
                .chain(support.repair_lineage_refs.iter())
                .chain(support.safe_output_tokens.iter())
                .any(|observed| observed.contains(token)),
            "support export should contain token {token:?}: {support:?}"
        );
    }
}

#[test]
fn structured_config_preview_keeps_source_effective_live_and_roundtrip_risk() {
    let fixture = fixture("structured_config_roundtrip_risk_preview.yaml");
    let record = fixture.record.normalized();
    let violations = record.validate();

    assert!(fixture.expect.valid);
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );

    let views: BTreeSet<_> = record
        .structured_state_views
        .iter()
        .map(|view| view.view_class.as_str())
        .collect();
    assert!(views.contains("authored_source"));
    assert!(views.contains("effective_projection"));
    assert!(views.contains("live_observed_state"));
    assert!(record
        .structured_state_views
        .iter()
        .filter(|view| view.view_class != StructuredViewClass::AuthoredSource)
        .all(|view| !view.write_eligibility.can_write()));

    let support = record.support_export();
    assert!(support.violation_tokens.is_empty());
    assert!(support
        .round_trip_risk_tokens
        .iter()
        .any(|token| token == "lossy_structural:require_compare_first_review"));
    assert!(support
        .repair_lineage_refs
        .iter()
        .any(|token| token.contains("structured_apply:checkpoint:config:before_apply")));

    for snippet in &fixture.expect.required_plaintext_snippets {
        assert!(
            support.plaintext_summary.contains(snippet),
            "support export plaintext should contain {snippet:?}\n{}",
            support.plaintext_summary
        );
    }
}

#[test]
fn unqualified_preview_overclaim_and_missing_lineage_are_rejected() {
    let fixture = fixture("failure_preview_overclaim_missing_lineage.yaml");
    let record = fixture.record.normalized();
    let violations = record.validate();
    let observed: BTreeSet<_> = violations
        .iter()
        .map(PreviewTruthViolation::token)
        .collect();

    assert!(!fixture.expect.valid);
    assert!(!violations.is_empty());
    for required in &fixture.expect.required_violation_tokens {
        assert!(
            observed.contains(required.as_str()),
            "expected violation {required}, observed {observed:?}"
        );
    }

    let support = record.support_export();
    for required in &fixture.expect.required_violation_tokens {
        assert!(
            support
                .violation_tokens
                .iter()
                .any(|token| token == required),
            "support export should carry violation {required}"
        );
    }
}

#[test]
fn seeded_records_are_valid_for_wedge_inspector() {
    let notebook = seeded_notebook_preview_truth("workspace:test");
    let config = seeded_structured_config_preview_truth("workspace:test");

    assert!(notebook.validate().is_empty());
    assert!(config.validate().is_empty());
    assert!(notebook
        .support_export()
        .trust_layer_tokens
        .iter()
        .any(|token| token.starts_with("runtime:")));
    assert!(config
        .support_export()
        .round_trip_risk_tokens
        .iter()
        .any(|token| token.contains("lossy_structural")));
}
