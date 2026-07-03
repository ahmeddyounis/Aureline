//! Integration tests for M5 benchmark/help/migration component proof artifacts.

use std::collections::BTreeSet;

use aureline_release::m5_benchmark_help_migration_components::{
    current_benchmark_evidence_card, current_benchmark_evidence_cards,
    validate_benchmark_evidence_cards, BenchmarkEvidenceSourceClass,
    M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF, M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF,
};

const PROOF_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-benchmark-help-migration-proof/proof_packet.json"
));

const SUPPORT_EXPORT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5-benchmark-help-migration-proof/support_export.json"
));

#[test]
fn checked_in_benchmark_cards_validate_cleanly() {
    let cards = current_benchmark_evidence_cards().expect("fixtures parse");
    let violations = validate_benchmark_evidence_cards(&cards);
    assert!(
        violations.is_empty(),
        "unexpected benchmark card violations: {violations:#?}"
    );
}

#[test]
fn canonical_fixture_ref_points_to_the_lab_reference_card() {
    let card = current_benchmark_evidence_card().expect("canonical card parses");
    assert_eq!(
        card.evidence_source_class,
        BenchmarkEvidenceSourceClass::LabReferenceRun
    );
    assert_eq!(
        M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF,
        "fixtures/ui/m5-benchmark-help-migration-components/benchmark_evidence_card.json"
    );
    assert_eq!(
        M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF,
        "schemas/ui/m5-benchmark-evidence-card.schema.json"
    );
}

#[test]
fn proof_packet_names_required_source_classes_and_exports() {
    let proof: serde_json::Value =
        serde_json::from_str(PROOF_PACKET_JSON).expect("proof packet parses");
    let family = proof["component_families"]
        .as_array()
        .expect("families")
        .iter()
        .find(|family| family["family"].as_str() == Some("benchmark_evidence_card"))
        .expect("benchmark family present");

    assert_eq!(
        family["schema_ref"].as_str(),
        Some(M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF)
    );
    assert_eq!(
        family["fixture_ref"].as_str(),
        Some(M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF)
    );

    let proved: BTreeSet<_> = family["evidence_source_classes_proved"]
        .as_array()
        .expect("source classes")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "lab_reference_run",
        "self_capture",
        "design_partner_result",
        "community_report",
        "imported_evidence",
    ] {
        assert!(proved.contains(required), "missing {required}");
    }

    let export_fields: BTreeSet<_> = family["export_parity_fields"]
        .as_array()
        .expect("export fields")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "benchmark_id",
        "caveat_summary_refs",
        "compare_view",
        "trace_report_export",
    ] {
        assert!(export_fields.contains(required), "missing {required}");
    }
}

#[test]
fn support_export_preserves_source_class_coverage_and_caveat_parity() {
    let export: serde_json::Value =
        serde_json::from_str(SUPPORT_EXPORT_JSON).expect("support export parses");
    let row = export["rows"]
        .as_array()
        .expect("rows")
        .iter()
        .find(|row| row["family"].as_str() == Some("benchmark_evidence_card"))
        .expect("benchmark support row present");

    assert_eq!(
        row["workflow_budget_truth"].as_str(),
        Some("measured_value_vs_budget_cold_warm_sample_size_extension_set_power_mode_scope_as_of_visible")
    );
    assert_eq!(
        row["export_parity"].as_str(),
        Some("benchmark_id_caveat_summaries_compare_view_trace_report_export_preserved")
    );

    let coverage: BTreeSet<_> = row["source_class_coverage"]
        .as_array()
        .expect("coverage")
        .iter()
        .map(|value| value.as_str().expect("string").to_owned())
        .collect();
    for required in [
        "lab_reference_run",
        "self_capture",
        "design_partner_result",
        "community_report",
        "imported_evidence",
    ] {
        assert!(coverage.contains(required), "missing {required}");
    }
}
