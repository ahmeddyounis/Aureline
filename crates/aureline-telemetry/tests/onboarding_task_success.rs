use std::collections::BTreeSet;
use std::path::PathBuf;

use aureline_telemetry::onboarding::{
    seeded_design_partner_capture, CompletionCheckpointClass, CompletionClass, EntryFlowDescriptor,
    EntryFlowKind, EntryRouteId, EntryVerbKind, FirstUsefulWorkTargetSurface, MeasurementSurface,
    MigrationFunnelStep, OnboardingEventInput, OnboardingEventName, OnboardingEventPhase,
    OnboardingTaskSuccessRecorder, OnboardingTelemetryContext, OutcomeClass, SemanticWarmupState,
    TargetKind, ONBOARDING_TASK_SUCCESS_FIXTURE_GENERATED_AT,
};
use aureline_telemetry::trace_event::BuildIdentityRecord;
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(path: &str) -> Value {
    let raw = std::fs::read_to_string(repo_root().join(path)).expect("fixture should be readable");
    serde_json::from_str(&raw).expect("fixture should parse")
}

fn string_set(values: &Value) -> BTreeSet<String> {
    values
        .as_array()
        .expect("expected array")
        .iter()
        .map(|value| value.as_str().expect("expected string").to_string())
        .collect()
}

#[test]
fn seeded_capture_matches_fixture_expectations() {
    let expected =
        fixture("fixtures/telemetry/onboarding_task_success/design_partner_expectations.json");
    let capture = seeded_design_partner_capture(ONBOARDING_TASK_SUCCESS_FIXTURE_GENERATED_AT);

    capture
        .validate_design_partner_proof()
        .expect("seeded capture should qualify as design-partner proof");

    assert_eq!(
        capture.schema.as_ref(),
        expected["expected_schema"].as_str().expect("schema")
    );
    assert_eq!(
        capture.record_kind.as_ref(),
        expected["expected_record_kind"]
            .as_str()
            .expect("record kind")
    );

    let actual_flows = capture
        .summary
        .observed_flow_kinds
        .iter()
        .map(|flow| flow.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_flows, string_set(&expected["expected_flow_kinds"]));

    let actual_verbs = capture
        .summary
        .distinct_entry_verbs
        .iter()
        .map(|verb| verb.as_str().to_string())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual_verbs, string_set(&expected["expected_entry_verbs"]));

    assert_eq!(
        capture.summary.first_useful_work_timing_count,
        expected["expected_first_useful_work_timing_count"]
            .as_u64()
            .expect("timing count")
    );
    assert_eq!(
        capture.summary.migration_funnel_event_count,
        expected["expected_migration_funnel_event_count"]
            .as_u64()
            .expect("migration count")
    );
    assert_eq!(
        capture.summary.design_partner_proof_ready,
        expected["expected_design_partner_proof_ready"]
            .as_bool()
            .expect("proof ready")
    );

    let migration = capture
        .events
        .iter()
        .find_map(|event| event.migration_funnel.as_ref())
        .expect("migration funnel event");
    assert_eq!(migration.step, MigrationFunnelStep::PerItemOutcomesRecorded);
    assert_eq!(
        migration.outcome_counts.total_items,
        expected["expected_import_outcome_counts"]["total_items"]
            .as_u64()
            .expect("total items")
    );
    assert_eq!(
        migration.outcome_counts.manual_review,
        expected["expected_import_outcome_counts"]["manual_review"]
            .as_u64()
            .expect("manual review")
    );

    assert_eq!(
        capture.privacy.contains_raw_project_content,
        expected["expected_privacy"]["contains_raw_project_content"]
            .as_bool()
            .expect("raw content flag")
    );
    assert!(capture.events.iter().all(|event| {
        event
            .first_useful_work
            .map(|timing| !timing.raw_project_content_captured)
            .unwrap_or(true)
    }));
}

#[test]
fn validation_rejects_missing_required_flow() {
    let mut capture = seeded_design_partner_capture(ONBOARDING_TASK_SUCCESS_FIXTURE_GENERATED_AT);
    capture
        .events
        .retain(|event| event.entry.flow_kind != EntryFlowKind::Reconnect);
    capture
        .summary
        .observed_flow_kinds
        .retain(|flow| *flow != EntryFlowKind::Reconnect);
    capture.summary.design_partner_proof_ready = false;

    let err = capture
        .validate_design_partner_proof()
        .expect_err("missing reconnect flow should fail");
    assert_eq!(
        err.check_id,
        "onboarding_task_success.flow_coverage.missing"
    );
}

#[test]
fn recorder_rejects_raw_target_refs() {
    let build = BuildIdentityRecord {
        crate_name: "aureline-telemetry".to_string(),
        crate_version: "0.0.0".to_string(),
        rustc_target_triple: "fixture-target".to_string(),
    };
    let context = OnboardingTelemetryContext::developer_local(
        "trace:onboarding-raw-target-drill",
        "session:onboarding-raw-target-drill",
        build,
    );
    let mut recorder = OnboardingTaskSuccessRecorder::new(context);
    let err = recorder
        .record_event(OnboardingEventInput {
            entry: EntryFlowDescriptor {
                flow_kind: EntryFlowKind::Open,
                entry_verb: EntryVerbKind::OpenFolder,
                entry_route_id: EntryRouteId::PlainOpen,
                measurement_surface: MeasurementSurface::SurfaceFirstOpen,
                target_kind: TargetKind::LocalFolder,
                target_ref: Some("/Users/example/private-repo".to_string()),
                deployment_profile_id: "deployment_profile:individual-local-alpha".to_string(),
            },
            event_name: OnboardingEventName::FirstUsefulEditDurable,
            event_phase: OnboardingEventPhase::UsefulWork,
            completion_checkpoint_class: Some(CompletionCheckpointClass::FirstUsefulEdit),
            completion_class: Some(CompletionClass::CompletedFirstUsefulEdit),
            outcome_class: OutcomeClass::Completed,
            first_useful_work: Some(aureline_telemetry::onboarding::FirstUsefulWorkTiming::new(
                1,
                2,
                FirstUsefulWorkTargetSurface::TreePlusReadmeOrChangedFiles,
                SemanticWarmupState::BeforeSemanticWarmup,
            )),
            migration_funnel: None,
            failure_category: None,
            evidence_refs: Vec::new(),
            occurred_tick: 2,
        })
        .expect_err("raw target ref should be rejected");

    assert_eq!(
        err.check_id,
        "onboarding_task_success.privacy.target_ref_not_opaque"
    );
}

#[test]
fn validation_rejects_migration_outcome_total_mismatch() {
    let mut capture = seeded_design_partner_capture(ONBOARDING_TASK_SUCCESS_FIXTURE_GENERATED_AT);
    let event = capture
        .events
        .iter_mut()
        .find(|event| event.migration_funnel.is_some())
        .expect("migration event");
    let migration = event.migration_funnel.as_mut().expect("migration funnel");
    migration.outcome_counts.total_items = 999;

    let err = capture
        .validate_design_partner_proof()
        .expect_err("mismatched outcome counts should fail");
    assert_eq!(
        err.check_id,
        "onboarding_task_success.migration_funnel.total_mismatch"
    );
}
