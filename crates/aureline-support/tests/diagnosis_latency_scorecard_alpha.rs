//! Protected tests for the external alpha support-scenario scorecard.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::scenario_scorecard::{
    current_alpha_scorecard, current_alpha_seeded_scenario_corpus, ResultPacketClass,
    SupportScenarioFamily, SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND,
    SUPPORT_SCENARIO_SCORECARD_RECORD_KIND, SUPPORT_SCENARIO_SUPPORT_PACKET_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn alpha_scorecard_validates_against_seeded_scenario_corpus() {
    let scorecard = current_alpha_scorecard().expect("scorecard parses");
    let corpus = current_alpha_seeded_scenario_corpus().expect("scenario corpus parses");

    let violations = scorecard.validate_with_corpus(&corpus);
    assert_eq!(violations, Vec::new());
    assert_eq!(
        scorecard.record_kind,
        SUPPORT_SCENARIO_SCORECARD_RECORD_KIND
    );
    assert_eq!(scorecard.scenario_rows.len(), 6);
    assert_eq!(corpus.entries.len(), 6);
}

#[test]
fn alpha_scorecard_covers_required_external_alpha_families() {
    let scorecard = current_alpha_scorecard().expect("scorecard parses");
    let families = scorecard
        .scenario_rows
        .iter()
        .map(|row| row.scenario_family)
        .collect::<BTreeSet<_>>();

    assert_eq!(
        families,
        [
            SupportScenarioFamily::FirstRun,
            SupportScenarioFamily::SearchIndex,
            SupportScenarioFamily::TrustPolicy,
            SupportScenarioFamily::RestoreContinuity,
            SupportScenarioFamily::ProviderAuth,
            SupportScenarioFamily::CrashLoop,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>()
    );

    for row in &scorecard.scenario_rows {
        assert!(row
            .launch_wedges
            .iter()
            .any(|wedge| wedge == "python_service"));
        assert!(row.launch_wedges.iter().any(|wedge| wedge == "tsjs_web"));
        assert!(row
            .support_contexts
            .iter()
            .any(|context| context == "cli_headless_doctor"));
    }
}

#[test]
fn latency_window_runs_from_scenario_start_to_useful_result_packet() {
    let scorecard = current_alpha_scorecard().expect("scorecard parses");
    let corpus = current_alpha_seeded_scenario_corpus().expect("scenario corpus parses");

    for entry in &corpus.entries {
        let scenario = &entry.scenario;
        assert_eq!(scenario.trigger.start_event, "support_scenario_started");
        assert_eq!(scenario.measurement.start_event, "support_scenario_started");
        assert_eq!(
            scenario.expected_first_actionable_result.stop_event,
            "first_actionable_result_packet_emitted"
        );
        assert_eq!(
            scenario.measurement.stop_event,
            "first_actionable_result_packet_emitted"
        );
        assert!(!scenario
            .expected_first_actionable_result
            .result_packet_ref
            .trim()
            .is_empty());
        assert!(!scenario
            .expected_first_actionable_result
            .finding_code
            .trim()
            .is_empty());
        assert!(scenario
            .measurement
            .evidence_required_before_stop
            .iter()
            .any(|field| field == "support_bundle_ref"));
        assert!(scenario.safety.read_only_diagnosis);
        assert!(scenario.safety.raw_private_material_excluded);
        assert!(scenario
            .safety
            .forbidden_fix_classes
            .iter()
            .any(|fix| fix == "destructive_reset_without_preview"));
    }

    let crash_row = scorecard
        .scenario_rows
        .iter()
        .find(|row| row.scenario_family == SupportScenarioFamily::CrashLoop)
        .expect("crash-loop row exists");
    assert_eq!(
        crash_row.expected_result.result_packet_class,
        ResultPacketClass::RecoveryDecision
    );
    assert!(crash_row
        .support_contexts
        .iter()
        .any(|context| context == "safe_mode_support_center"));
}

#[test]
fn support_packet_and_dashboard_project_from_one_scorecard() {
    let scorecard = current_alpha_scorecard().expect("scorecard parses");
    let support_packet =
        scorecard.support_packet("support.packet.alpha.scorecard", "2026-05-14T00:05:00Z");
    let dashboard =
        scorecard.review_dashboard("dashboard.alpha.support.scorecard", "2026-05-14T00:05:00Z");

    assert_eq!(
        support_packet.record_kind,
        SUPPORT_SCENARIO_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(
        dashboard.record_kind,
        SUPPORT_SCENARIO_DASHBOARD_RECORD_KIND
    );
    assert_eq!(support_packet.scorecard_id, scorecard.scorecard_id);
    assert_eq!(dashboard.scorecard_id, scorecard.scorecard_id);
    assert_eq!(support_packet.rows.len(), scorecard.scenario_rows.len());
    assert_eq!(dashboard.rows.len(), scorecard.scenario_rows.len());
    assert!(support_packet.is_export_safe());

    let support_row_ids = support_packet
        .rows
        .iter()
        .map(|row| row.scorecard_row_id.as_str())
        .collect::<BTreeSet<_>>();
    let dashboard_row_ids = dashboard
        .rows
        .iter()
        .map(|row| row.scorecard_row_id.as_str())
        .collect::<BTreeSet<_>>();
    let scorecard_row_ids = scorecard
        .scenario_rows
        .iter()
        .map(|row| row.row_id.as_str())
        .collect::<BTreeSet<_>>();

    assert_eq!(support_row_ids, scorecard_row_ids);
    assert_eq!(dashboard_row_ids, scorecard_row_ids);
    assert!(support_packet.rows.iter().all(|row| {
        row.redaction_class == "metadata_safe_default"
            && row.exact_build_identity_required
            && row.raw_private_material_excluded
    }));
}

#[test]
fn scorecard_docs_and_fixtures_exist_at_declared_paths() {
    let root = repo_root();
    let scorecard = current_alpha_scorecard().expect("scorecard parses");
    let corpus = current_alpha_seeded_scenario_corpus().expect("scenario corpus parses");

    assert!(root
        .join("artifacts/support/diagnosis_latency_scorecard_alpha.yaml")
        .is_file());
    assert!(root
        .join("docs/support/alpha_support_scenarios.md")
        .is_file());

    for entry in corpus.entries {
        assert!(
            root.join(&entry.fixture_ref).is_file(),
            "{} must exist",
            entry.fixture_ref
        );
        assert!(scorecard
            .scenario_rows
            .iter()
            .any(|row| row.fixture_ref == entry.fixture_ref));
    }
}
