//! Protected drill harness for the M3 field-readiness scorecards.
//!
//! Projects the diagnosis-latency scorecard, exact-build availability
//! report, and field-readiness dashboard from the M3 support-scenario
//! corpus and asserts each projection stays metadata-safe, covers every
//! required beta lane, and downgrades when the corpus or alpha
//! scorecard goes stale. The dashboard JSON is checked in at
//! `artifacts/support/m3/field_readiness_dashboard.json`; the test
//! refuses to pass when the on-disk file diverges from the projection.
//! Set `AURELINE_UPDATE_BASELINE=1` to refresh the checked-in JSON.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::field_readiness::{
    current_field_readiness_scorecards, project_field_readiness_scorecards, EvidencePathClass,
    LatencyMeasurementState, StaleDataTrigger, M3_DIAGNOSIS_LATENCY_LANE_ROW_RECORD_KIND,
    M3_DIAGNOSIS_LATENCY_SCORECARD_RECORD_KIND, M3_DIAGNOSIS_LATENCY_SCORECARD_REF,
    M3_EXACT_BUILD_AVAILABILITY_LANE_ROW_RECORD_KIND,
    M3_EXACT_BUILD_AVAILABILITY_REPORT_RECORD_KIND, M3_EXACT_BUILD_AVAILABILITY_REPORT_REF,
    M3_FIELD_READINESS_DASHBOARD_RECORD_KIND, M3_FIELD_READINESS_DASHBOARD_REF,
    M3_FIELD_READINESS_LANE_ROW_RECORD_KIND, M3_FIELD_READINESS_METRICS_DOC_REF,
    REQUIRED_EVIDENCE_PATH_CLASSES,
};
use aureline_support::m3_scenario_corpus::{
    current_m3_scenario_corpus, required_beta_lane_classes, M3BetaLaneClass,
};
use aureline_support::scenario_scorecard::current_alpha_scorecard;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn current_field_readiness_scorecards_cover_every_required_beta_lane() {
    let bundle = current_field_readiness_scorecards().expect("scorecards project");

    assert_eq!(
        bundle.diagnosis_latency_scorecard.record_kind,
        M3_DIAGNOSIS_LATENCY_SCORECARD_RECORD_KIND
    );
    assert_eq!(
        bundle.exact_build_availability_report.record_kind,
        M3_EXACT_BUILD_AVAILABILITY_REPORT_RECORD_KIND
    );
    assert_eq!(
        bundle.field_readiness_dashboard.record_kind,
        M3_FIELD_READINESS_DASHBOARD_RECORD_KIND
    );

    let expected_lanes = required_beta_lane_classes()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let diagnosis_lanes = bundle
        .diagnosis_latency_scorecard
        .lane_rows
        .iter()
        .map(|row| row.beta_lane_class)
        .collect::<BTreeSet<_>>();
    let exact_build_lanes = bundle
        .exact_build_availability_report
        .lane_rows
        .iter()
        .map(|row| row.beta_lane_class)
        .collect::<BTreeSet<_>>();
    let dashboard_lanes = bundle
        .field_readiness_dashboard
        .lane_rows
        .iter()
        .map(|row| row.beta_lane_class)
        .collect::<BTreeSet<_>>();

    assert_eq!(diagnosis_lanes, expected_lanes);
    assert_eq!(exact_build_lanes, expected_lanes);
    assert_eq!(dashboard_lanes, expected_lanes);

    assert!(bundle.is_release_consumable());
}

#[test]
fn every_lane_row_carries_metadata_safe_baseline_and_release_gate_contribution() {
    let bundle = current_field_readiness_scorecards().expect("scorecards project");

    assert!(
        bundle
            .diagnosis_latency_scorecard
            .raw_private_material_excluded
    );
    assert!(
        bundle
            .diagnosis_latency_scorecard
            .ambient_authority_excluded
    );

    for row in &bundle.diagnosis_latency_scorecard.lane_rows {
        assert_eq!(row.record_kind, M3_DIAGNOSIS_LATENCY_LANE_ROW_RECORD_KIND);
        assert!(row.contributes_to_release_gate);
        assert!(row.stale_data_triggers.is_empty());
        assert_eq!(
            row.time_to_first_actionable_finding.state,
            LatencyMeasurementState::SeededPendingLiveMeasurement
        );
        assert!(
            row.time_to_first_actionable_finding.p50_target_seconds
                < row.time_to_first_actionable_finding.p90_target_seconds
        );
        assert!(
            row.time_to_first_actionable_finding.p90_target_seconds
                < row.time_to_first_actionable_finding.yellow_seconds
        );
        assert!(
            row.time_to_first_actionable_finding.yellow_seconds
                < row.time_to_first_actionable_finding.red_seconds
        );
        let path_classes = row
            .per_path_budgets
            .iter()
            .map(|budget| budget.path_class)
            .collect::<BTreeSet<_>>();
        let required_paths = REQUIRED_EVIDENCE_PATH_CLASSES.iter().copied().collect();
        assert_eq!(path_classes, required_paths);
        assert!(row
            .per_path_budgets
            .iter()
            .all(|budget| budget.available_at_equal_prominence));
    }

    for row in &bundle.exact_build_availability_report.lane_rows {
        assert_eq!(
            row.record_kind,
            M3_EXACT_BUILD_AVAILABILITY_LANE_ROW_RECORD_KIND
        );
        assert!(row.exact_build_identity_required);
        assert_eq!(row.seeded_exact_build_availability_pct, 100);
        assert!(row.stale_data_triggers.is_empty());
        let symbolication_lane = matches!(
            row.beta_lane_class,
            M3BetaLaneClass::SafeMode
                | M3BetaLaneClass::ExtensionBisect
                | M3BetaLaneClass::RuntimeReplayPackets
        );
        assert_eq!(row.symbolication_required, symbolication_lane);
        if symbolication_lane {
            assert_eq!(row.seeded_symbolication_availability_pct, 100);
        }
    }

    for row in &bundle.field_readiness_dashboard.lane_rows {
        assert_eq!(row.record_kind, M3_FIELD_READINESS_LANE_ROW_RECORD_KIND);
        assert!(row.contributes_to_release_gate);
        assert_eq!(row.seeded_escalation_packet_completeness_pct, 100);
        assert_eq!(row.seeded_false_safe_repair_rate_bps, 0);
        assert!(row.stale_data_triggers.is_empty());
        for required in [
            "fixture_missing",
            "drill_step_unproven",
            "drill_proves_regression",
        ] {
            assert!(
                row.claim_downgrade_trigger_classes
                    .iter()
                    .any(|trigger| trigger == required),
                "{} missing required claim-downgrade trigger {}",
                row.scenario_id,
                required
            );
        }
    }
}

#[test]
fn projection_downgrades_when_alpha_scorecard_is_missing() {
    let corpus = current_m3_scenario_corpus().expect("corpus parses");
    let bundle = project_field_readiness_scorecards(&corpus, None, "2026-05-19T00:00:00Z");

    let expected_trigger = StaleDataTrigger::AlphaScorecardMissing.as_str();
    assert!(bundle
        .diagnosis_latency_scorecard
        .stale_data_triggers
        .iter()
        .any(|trigger| trigger == expected_trigger));
    assert!(bundle
        .exact_build_availability_report
        .stale_data_triggers
        .iter()
        .any(|trigger| trigger == expected_trigger));
    assert!(bundle
        .field_readiness_dashboard
        .stale_data_triggers
        .iter()
        .any(|trigger| trigger == expected_trigger));

    for row in &bundle.diagnosis_latency_scorecard.lane_rows {
        assert_eq!(
            row.time_to_first_actionable_finding.state,
            LatencyMeasurementState::StaleDowngraded
        );
        assert!(row
            .stale_data_triggers
            .iter()
            .any(|t| t == expected_trigger));
    }
    assert!(!bundle.is_release_consumable());
}

#[test]
fn projection_downgrades_when_corpus_drops_required_lane() {
    let mut corpus = current_m3_scenario_corpus().expect("corpus parses");
    corpus
        .entries
        .retain(|entry| entry.scenario.beta_lane_class != M3BetaLaneClass::RuntimeReplayPackets);
    let alpha = current_alpha_scorecard().expect("alpha scorecard parses");
    let bundle = project_field_readiness_scorecards(&corpus, Some(&alpha), "2026-05-19T00:00:00Z");
    let expected_trigger = StaleDataTrigger::SeededCorpusMissingLane.as_str();
    assert!(bundle
        .diagnosis_latency_scorecard
        .stale_data_triggers
        .iter()
        .any(|trigger| trigger == expected_trigger));
    assert!(!bundle.is_release_consumable());
}

#[test]
fn evidence_path_classes_are_attributable_at_equal_prominence() {
    let bundle = current_field_readiness_scorecards().expect("scorecards project");
    for path in REQUIRED_EVIDENCE_PATH_CLASSES {
        assert!(bundle
            .diagnosis_latency_scorecard
            .measurement_paths
            .iter()
            .any(|token| token == path.as_str()));
        assert!(bundle
            .field_readiness_dashboard
            .measurement_paths
            .iter()
            .any(|token| token == path.as_str()));
    }

    assert_eq!(
        EvidencePathClass::LocalOnly.as_str(),
        "local_only",
        "local-only path must remain reachable at equal prominence"
    );
}

#[test]
fn field_readiness_dashboard_matches_checked_in_json_baseline() {
    let bundle = current_field_readiness_scorecards().expect("scorecards project");
    let projected = serde_json::to_string_pretty(&bundle.field_readiness_dashboard)
        .expect("dashboard serializes");
    let projected_normalized = format!("{}\n", projected.trim_end());

    let root = repo_root();
    let dashboard_path = root.join(M3_FIELD_READINESS_DASHBOARD_REF);

    if std::env::var("AURELINE_UPDATE_BASELINE").is_ok() {
        std::fs::write(&dashboard_path, &projected_normalized).expect("write dashboard");
        return;
    }

    let on_disk = std::fs::read_to_string(&dashboard_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", dashboard_path.display()));
    assert_eq!(
        on_disk, projected_normalized,
        "field-readiness dashboard JSON diverges from the projection; rerun with AURELINE_UPDATE_BASELINE=1 to refresh"
    );
}

#[test]
fn scorecard_artifacts_exist_at_declared_paths() {
    let root = repo_root();
    assert!(
        root.join(M3_DIAGNOSIS_LATENCY_SCORECARD_REF).is_file(),
        "{M3_DIAGNOSIS_LATENCY_SCORECARD_REF} missing"
    );
    assert!(
        root.join(M3_EXACT_BUILD_AVAILABILITY_REPORT_REF).is_file(),
        "{M3_EXACT_BUILD_AVAILABILITY_REPORT_REF} missing"
    );
    assert!(
        root.join(M3_FIELD_READINESS_DASHBOARD_REF).is_file(),
        "{M3_FIELD_READINESS_DASHBOARD_REF} missing"
    );
    assert!(
        root.join(M3_FIELD_READINESS_METRICS_DOC_REF).is_file(),
        "{M3_FIELD_READINESS_METRICS_DOC_REF} missing"
    );
}
