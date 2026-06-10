use super::*;

const PACKET_ID: &str = "scaffold-planner:stable:0001";
const PLANNER_LABEL: &str =
    "Scaffold Planner, Parameter Review, Environment Preflights, and Create-Empty Parity";

const READY_ROW: &str = "scaffold-plan:rust.cli.ready:2026.04";
const AWAITING_ROW: &str = "scaffold-plan:ts.web.awaiting_input:2026.04";
const PREFLIGHT_BLOCKED_ROW: &str = "scaffold-plan:python.data.preflight_blocked:2026.03";
const CREATE_EMPTY_ROW: &str = "scaffold-plan:create_empty.workspace:2026.05";

fn proof_freshness() -> ScaffoldPlannerProofFreshness {
    ScaffoldPlannerProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ScaffoldPlannerPacket {
    canonical_scaffold_planner(
        PACKET_ID.to_owned(),
        PLANNER_LABEL.to_owned(),
        "2026-06-07T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a ScaffoldPlannerPacket, plan_id: &str) -> &'a ScaffoldPlanRow {
    packet
        .rows
        .iter()
        .find(|row| row.plan_id == plan_id)
        .unwrap_or_else(|| panic!("missing plan {plan_id}"))
}

#[test]
fn scaffold_planner_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_planner_covers_plan_spectrum() {
    let packet = packet();
    let kinds: Vec<ScaffoldPlanKind> = packet.rows.iter().map(|row| row.plan_kind).collect();
    assert!(kinds.contains(&ScaffoldPlanKind::TemplateScaffold));
    assert!(kinds.contains(&ScaffoldPlanKind::CreateEmptyWorkspace));

    let states: Vec<PlanReadinessState> =
        packet.rows.iter().map(|row| row.readiness_state).collect();
    assert!(states.contains(&PlanReadinessState::ReadyForApply));
    assert!(states.contains(&PlanReadinessState::BlockedAwaitingInput));
    assert!(states.contains(&PlanReadinessState::BlockedFailedPreflight));
}

#[test]
fn create_empty_row_reaches_full_parity_in_canonical_planner() {
    let packet = packet();
    let create_empty = row(&packet, CREATE_EMPTY_ROW);
    assert!(create_empty.is_create_empty());
    assert_eq!(
        create_empty.create_empty_parity.parity_class,
        CreateEmptyParityClass::FullParityWithTemplateFlow
    );
    assert!(create_empty.create_empty_parity.shares_preflight_pipeline);
    assert!(create_empty.create_empty_parity.shares_rollback_boundary);
    assert!(create_empty.admitted_for_apply);
}

#[test]
fn blocked_plans_are_not_admitted_in_canonical_planner() {
    let packet = packet();
    assert!(!row(&packet, AWAITING_ROW).admitted_for_apply);
    assert!(!row(&packet, PREFLIGHT_BLOCKED_ROW).admitted_for_apply);
}

#[test]
fn no_plan_writes_before_confirmation() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.write_impact_preview.no_writes_before_confirmation,
            "plan {} would write before confirmation",
            row.plan_id
        );
    }
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::RowsEmpty));
}

#[test]
fn template_plan_missing_manifest_ref_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap()
        .manifest_ref = None;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::TemplateProvenanceIncomplete));
}

#[test]
fn incoherent_parameter_counts_fail() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap()
        .parameter_review
        .resolved_parameters = 99;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::ParameterCountsIncoherent));
}

#[test]
fn incoherent_preflight_counts_fail() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap()
        .environment_preflight
        .blocking_failures = 99;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::PreflightCountsIncoherent));
}

#[test]
fn blocking_parameter_admitted_fails() {
    let mut packet = packet();
    let ready = packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap();
    ready.parameter_review.review_class = ParameterReviewClass::AwaitingRequiredInput;
    // admitted_for_apply stays true.
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::BlockingParameterAdmitted));
}

#[test]
fn blocking_preflight_admitted_fails() {
    let mut packet = packet();
    let ready = packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap();
    ready.environment_preflight.preflight_class = EnvironmentPreflightClass::PreflightFailedBlocked;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::BlockingPreflightAdmitted));
}

#[test]
fn non_ready_admitted_fails() {
    let mut packet = packet();
    let ready = packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == READY_ROW)
        .unwrap();
    ready.readiness_state = PlanReadinessState::ReviewRequired;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::NonReadyAdmitted));
}

#[test]
fn create_empty_claiming_parity_without_sharing_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.plan_id == CREATE_EMPTY_ROW)
        .unwrap()
        .create_empty_parity
        .shares_rollback_boundary = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::CreateEmptyParityIncomplete));
}

#[test]
fn writes_before_confirmation_fails() {
    let mut packet = packet();
    packet.rows[0]
        .write_impact_preview
        .no_writes_before_confirmation = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::WritesBeforeConfirmation));
}

#[test]
fn missing_preflight_check_refs_fails() {
    let mut packet = packet();
    packet.rows[0]
        .environment_preflight
        .preflight_check_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::PreflightCheckRefsMissing));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::MissingSourceContracts));
}

#[test]
fn safety_review_incomplete_fails() {
    let mut packet = packet();
    packet.safety_review.create_empty_reaches_rollback_parity = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::SafetyReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_plans_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ScaffoldPlannerViolation::ProofFreshnessIncomplete));
}

#[test]
fn unresolved_parameters_block_a_ready_plan() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: READY_ROW.to_owned(),
        parameters_resolved: false,
        environment_ready: true,
        write_impact_preview_available: true,
        rollback_boundary_available: true,
        create_empty_parity_intact: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let ready = row(&packet, READY_ROW);
    assert_eq!(
        ready.readiness_state,
        PlanReadinessState::BlockedAwaitingInput
    );
    assert!(!ready.admitted_for_apply);
    assert!(ready
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::RequiredParameterUnresolved));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn failed_preflight_blocks_a_ready_plan() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: READY_ROW.to_owned(),
        parameters_resolved: true,
        environment_ready: false,
        write_impact_preview_available: true,
        rollback_boundary_available: true,
        create_empty_parity_intact: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let ready = row(&packet, READY_ROW);
    assert_eq!(
        ready.readiness_state,
        PlanReadinessState::BlockedFailedPreflight
    );
    assert_eq!(
        ready.environment_preflight.preflight_class,
        EnvironmentPreflightClass::PreflightFailedBlocked
    );
    assert!(!ready.admitted_for_apply);
    assert!(ready
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::PreflightFailed));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn broken_parity_blocks_a_create_empty_plan() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: CREATE_EMPTY_ROW.to_owned(),
        parameters_resolved: true,
        environment_ready: true,
        write_impact_preview_available: true,
        rollback_boundary_available: true,
        create_empty_parity_intact: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let create_empty = row(&packet, CREATE_EMPTY_ROW);
    assert_eq!(
        create_empty.create_empty_parity.parity_class,
        CreateEmptyParityClass::ParityBrokenBlocked
    );
    assert_eq!(
        create_empty.readiness_state,
        PlanReadinessState::BlockedParityBroken
    );
    assert!(!create_empty.admitted_for_apply);
    assert!(create_empty
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::CreateEmptyParityBroken));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn broken_parity_does_not_affect_a_template_plan() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: READY_ROW.to_owned(),
        parameters_resolved: true,
        environment_ready: true,
        write_impact_preview_available: true,
        rollback_boundary_available: true,
        create_empty_parity_intact: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let ready = row(&packet, READY_ROW);
    // A template-only plan ignores create-empty parity observations.
    assert!(ready.admitted_for_apply);
    assert!(!ready
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::CreateEmptyParityBroken));
}

#[test]
fn missing_rollback_boundary_blocks_a_plan() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: READY_ROW.to_owned(),
        parameters_resolved: true,
        environment_ready: true,
        write_impact_preview_available: true,
        rollback_boundary_available: false,
        create_empty_parity_intact: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let ready = row(&packet, READY_ROW);
    assert_eq!(
        ready.write_impact_preview.rollback_posture,
        PlanRollbackPostureClass::RollbackUnavailableReviewRequired
    );
    assert!(!ready.admitted_for_apply);
    assert!(ready
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::RollbackBoundaryUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_admission() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[ScaffoldPlanRowObservation {
        plan_id: READY_ROW.to_owned(),
        parameters_resolved: true,
        environment_ready: true,
        write_impact_preview_available: true,
        rollback_boundary_available: true,
        create_empty_parity_intact: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let ready = row(&packet, READY_ROW);
    assert!(!ready.admitted_for_apply);
    assert!(ready
        .downgrade_triggers
        .contains(&ScaffoldPlannerDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_plan() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.plan_id),
            "summary missing plan {}",
            row.plan_id
        );
    }
    assert!(summary.contains("create_empty_workspace"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_scaffold_planner_export().expect("checked scaffold-planner export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked =
        current_scaffold_planner_export().expect("checked scaffold-planner export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/parameter_unresolved_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/create_empty_parity_broken.json"
        )),
    ] {
        let packet: ScaffoldPlannerPacket =
            serde_json::from_str(raw).expect("fixture parses as scaffold-planner packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
