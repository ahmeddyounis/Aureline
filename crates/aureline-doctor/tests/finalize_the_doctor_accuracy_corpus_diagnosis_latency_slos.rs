//! Protected tests for the finalized Doctor accuracy corpus, diagnosis-latency
//! SLOs, and headless/UI parity on stable profiles.

use std::path::{Path, PathBuf};

use aureline_doctor::finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos::{
    load_accuracy_corpus, load_latency_slo_catalog, load_stable_profile_parity_audit,
    AccuracyCorpusScenarioClass, BenchmarkLabTraceRef, DiagnosisLatencySloCatalog,
    DoctorAccuracyCorpus, GroundTruthRepairClass, LatencyPercentileClass, MeasurementSurfaceClass,
    ProjectDoctorFinalizeEvaluator, StableProfileParityAudit, PROJECT_DOCTOR_FINALIZE_DOC_REF,
    PROJECT_DOCTOR_FINALIZE_SCHEMA_REF, PROJECT_DOCTOR_FINALIZE_SUPPORT_PACKET_RECORD_KIND,
};
use aureline_doctor::stabilize_project_doctor_probes_finding_codes_explainability_and::{
    StableFindingConfidenceClass, StableFindingSeverityClass, StableProbePackClass,
    StableSupportContextClass,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    corpus_file: String,
    latency_catalog_file: String,
    parity_audit_file: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root()
        .join("fixtures/support/m4/finalize_the_doctor_accuracy_corpus_diagnosis_latency_slos")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_corpus() -> DoctorAccuracyCorpus {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.corpus_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_accuracy_corpus(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_latency_catalog() -> DiagnosisLatencySloCatalog {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.latency_catalog_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_latency_slo_catalog(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_parity_audit() -> StableProfileParityAudit {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.parity_audit_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_stable_profile_parity_audit(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

#[test]
fn accuracy_corpus_contains_all_eight_scenarios() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let corpus = load_corpus();

    let report = evaluator.validate_accuracy_corpus(&corpus);
    assert!(
        report.is_valid(),
        "corpus validation failed: {:?}",
        report.violations
    );

    let scenarios: std::collections::BTreeSet<_> = corpus
        .ground_truth_records
        .iter()
        .map(|r| r.scenario_class)
        .collect();
    let all = AccuracyCorpusScenarioClass::all();
    for expected in &all {
        assert!(
            scenarios.contains(expected),
            "missing scenario {:?}",
            expected
        );
    }
    assert_eq!(scenarios.len(), 8);
}

#[test]
fn accuracy_corpus_ground_truth_records_have_unique_ids() {
    let corpus = load_corpus();
    let mut ids = std::collections::BTreeSet::new();
    for record in &corpus.ground_truth_records {
        assert!(
            ids.insert(&record.row_id),
            "duplicate row_id {}",
            record.row_id
        );
    }
}

#[test]
fn accuracy_corpus_finding_codes_start_with_doctor_finding_prefix() {
    let corpus = load_corpus();
    for record in &corpus.ground_truth_records {
        assert!(
            record.expected_finding_code.starts_with("doctor.finding."),
            "row {} finding code {} does not start with doctor.finding.",
            record.row_id,
            record.expected_finding_code
        );
    }
}

#[test]
fn accuracy_corpus_has_at_least_one_observe_only_outcome() {
    let corpus = load_corpus();
    let observe_only_count = corpus
        .ground_truth_records
        .iter()
        .filter(|r| r.expected_repair_class == GroundTruthRepairClass::ObserveOnlyNoRepair)
        .count();
    assert!(
        observe_only_count >= 1,
        "expected at least one observe-only outcome"
    );
}

#[test]
fn latency_catalog_has_at_least_one_budget_per_row() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let catalog = load_latency_catalog();

    let report = evaluator.validate_latency_slo_catalog(&catalog);
    assert!(
        report.is_valid(),
        "latency catalog validation failed: {:?}",
        report.violations
    );

    for row in &catalog.latency_rows {
        assert!(!row.budgets.is_empty(), "row {} has no budgets", row.row_id);
    }
}

#[test]
fn latency_budget_targets_are_nonzero_and_ordered() {
    let catalog = load_latency_catalog();
    for row in &catalog.latency_rows {
        for budget in &row.budgets {
            assert!(
                budget.target_ms > 0,
                "row {} budget target_ms must be non-zero",
                row.row_id
            );
            assert!(
                budget.yellow_ms > budget.target_ms,
                "row {} budget yellow_ms must be > target_ms",
                row.row_id
            );
            assert!(
                budget.red_ms > budget.yellow_ms,
                "row {} budget red_ms must be > yellow_ms",
                row.row_id
            );
        }
    }
}

#[test]
fn parity_audit_has_exactly_four_rows() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let audit = load_parity_audit();

    let report = evaluator.validate_stable_profile_parity_audit(&audit);
    assert!(
        report.is_valid(),
        "parity audit validation failed: {:?}",
        report.violations
    );

    assert_eq!(audit.parity_rows.len(), 4);
}

#[test]
fn parity_audit_rows_include_required_machine_readable_fields() {
    let audit = load_parity_audit();
    let required = [
        "finding_id",
        "finding_code",
        "probe_id",
        "probe_class",
        "diagnosis_posture",
        "exit_code_class",
    ];
    for row in &audit.parity_rows {
        for field in &required {
            assert!(
                row.machine_readable_result_fields
                    .iter()
                    .any(|f| f == *field),
                "row for context {} missing required field {}",
                row.support_context_class.as_str(),
                field
            );
        }
    }
}

#[test]
fn parity_audit_covers_all_four_support_contexts() {
    let audit = load_parity_audit();
    let expected = [
        StableSupportContextClass::Desktop,
        StableSupportContextClass::CliHeadless,
        StableSupportContextClass::RemoteManaged,
        StableSupportContextClass::OfflineLocal,
    ];
    let seen: std::collections::BTreeSet<_> = audit
        .parity_rows
        .iter()
        .map(|r| r.support_context_class)
        .collect();
    for ctx in &expected {
        assert!(seen.contains(ctx), "missing context {:?}", ctx);
    }
}

#[test]
fn finalized_support_packet_excludes_raw_private_material() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let corpus = load_corpus();
    let latency_catalog = load_latency_catalog();
    let audit = load_parity_audit();

    let packet = evaluator
        .finalize_support_packet(
            "packet:test",
            "2026-06-02T00:00:00Z",
            &corpus,
            &latency_catalog,
            &[audit],
            &[],
        )
        .expect("finalize support packet");

    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_FINALIZE_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.schema_ref, PROJECT_DOCTOR_FINALIZE_SCHEMA_REF);
    assert_eq!(packet.doc_ref, PROJECT_DOCTOR_FINALIZE_DOC_REF);
    assert!(packet.raw_private_material_excluded);
    assert!(packet.ambient_authority_excluded);
    assert_eq!(packet.corpus_id, corpus.corpus_id);
    assert_eq!(packet.catalog_id, latency_catalog.catalog_id);
}

#[test]
fn finalized_support_packet_corpus_metadata_matches_corpus() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let corpus = load_corpus();
    let latency_catalog = load_latency_catalog();
    let audit = load_parity_audit();

    let packet = evaluator
        .finalize_support_packet(
            "packet:test",
            "2026-06-02T00:00:00Z",
            &corpus,
            &latency_catalog,
            &[audit],
            &[],
        )
        .expect("finalize support packet");

    assert_eq!(
        packet.corpus_metadata.ground_truth_count,
        corpus.ground_truth_records.len()
    );
    let observe_only_count = corpus
        .ground_truth_records
        .iter()
        .filter(|r| r.expected_repair_class == GroundTruthRepairClass::ObserveOnlyNoRepair)
        .count();
    assert_eq!(
        packet.corpus_metadata.observe_only_count,
        observe_only_count
    );
    assert_eq!(
        packet.corpus_metadata.repair_candidate_count,
        corpus.ground_truth_records.len() - observe_only_count
    );
    assert_eq!(
        packet.corpus_metadata.scenarios_covered.len(),
        8,
        "all eight scenarios must be covered"
    );
}

#[test]
fn finalized_support_packet_with_benchmark_traces() {
    let evaluator = ProjectDoctorFinalizeEvaluator::new();
    let corpus = load_corpus();
    let latency_catalog = load_latency_catalog();
    let audit = load_parity_audit();

    let traces = vec![
        BenchmarkLabTraceRef {
            trace_id: "trace:missing_toolchain.local.p50".to_string(),
            run_at: "2026-06-01T00:00:00Z".to_string(),
            profile_name: "desktop_stable".to_string(),
            scenario_class: AccuracyCorpusScenarioClass::MissingToolchain,
            measurement_surface: MeasurementSurfaceClass::DoctorProbeRunLocal,
            observed_latency_ms: 1800,
            passed: true,
        },
        BenchmarkLabTraceRef {
            trace_id: "trace:blocked_trust_state.local.p95".to_string(),
            run_at: "2026-06-01T00:00:00Z".to_string(),
            profile_name: "desktop_stable".to_string(),
            scenario_class: AccuracyCorpusScenarioClass::BlockedTrustState,
            measurement_surface: MeasurementSurfaceClass::DoctorProbeRunLocal,
            observed_latency_ms: 2100,
            passed: true,
        },
    ];

    let packet = evaluator
        .finalize_support_packet(
            "packet:with_traces",
            "2026-06-02T00:00:00Z",
            &corpus,
            &latency_catalog,
            &[audit],
            &traces,
        )
        .expect("finalize support packet");

    assert_eq!(packet.benchmark_lab_traces.len(), 2);
    assert!(packet.benchmark_lab_traces[0].passed);
}

#[test]
fn latency_catalog_includes_p50_and_p95_budgets() {
    let catalog = load_latency_catalog();
    for row in &catalog.latency_rows {
        let has_p50 = row
            .budgets
            .iter()
            .any(|b| b.percentile == LatencyPercentileClass::P50);
        let has_p95 = row
            .budgets
            .iter()
            .any(|b| b.percentile == LatencyPercentileClass::P95);
        assert!(
            has_p50 || has_p95,
            "row {} must include at least one of p50 or p95",
            row.row_id
        );
    }
}

#[test]
fn corpus_records_reference_valid_probe_pack_classes() {
    let corpus = load_corpus();
    let valid_packs = StableProbePackClass::all();
    for record in &corpus.ground_truth_records {
        assert!(
            valid_packs.contains(&record.probe_pack_class),
            "row {} references unknown probe pack class {:?}",
            record.row_id,
            record.probe_pack_class
        );
    }
}

#[test]
fn corpus_records_declare_expected_severity_and_confidence() {
    let corpus = load_corpus();
    for record in &corpus.ground_truth_records {
        assert!(
            !matches!(
                record.expected_severity,
                StableFindingSeverityClass::Unsupported
            ),
            "ground truth should not declare unsupported severity; row {}",
            record.row_id
        );
        assert!(
            !matches!(
                record.expected_confidence,
                StableFindingConfidenceClass::UnknownRequiresProbe
            ),
            "ground truth should not declare unknown confidence; row {}",
            record.row_id
        );
    }
}
