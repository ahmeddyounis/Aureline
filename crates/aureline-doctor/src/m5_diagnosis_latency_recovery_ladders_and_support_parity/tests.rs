use super::*;

fn packet() -> ProjectDoctorM5RecoveryFieldReadiness {
    current_project_doctor_m5_recovery_field_readiness().expect("embedded packet parses")
}

fn violation_ids(packet: &ProjectDoctorM5RecoveryFieldReadiness) -> Vec<String> {
    packet.validate().into_iter().map(|v| v.check_id).collect()
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, PROJECT_DOCTOR_M5_RECOVERY_RECORD_KIND);
    assert_eq!(packet.schema_ref, PROJECT_DOCTOR_M5_RECOVERY_SCHEMA_REF);
    assert_eq!(packet.overview_page, PROJECT_DOCTOR_M5_RECOVERY_DOC_REF);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_lane_is_covered() {
    let packet = packet();
    for lane in RecoveryLane::ALL {
        assert!(
            packet.scenarios_in_lane(lane).next().is_some(),
            "no scenario for lane {lane}"
        );
    }
}

#[test]
fn every_recovery_rung_is_exercised() {
    let packet = packet();
    let present: BTreeSet<RecoveryRung> =
        packet.scenarios.iter().map(|s| s.recovery_rung).collect();
    for rung in RecoveryRung::ALL {
        assert!(present.contains(&rung), "no scenario for rung {rung}");
    }
}

#[test]
fn every_drill_outcome_is_exercised() {
    let packet = packet();
    let present: BTreeSet<DrillOutcome> =
        packet.scenarios.iter().map(|s| s.drill_outcome).collect();
    for outcome in DrillOutcome::ALL {
        assert!(
            present.contains(&outcome),
            "no scenario with drill outcome {}",
            outcome.as_str()
        );
    }
}

#[test]
fn every_promotion_action_and_reason_is_exercised() {
    let packet = packet();
    let actions: BTreeSet<PromotionAction> = packet
        .scenarios
        .iter()
        .map(|s| s.published_promotion_action)
        .collect();
    for action in PromotionAction::ALL {
        assert!(
            actions.contains(&action),
            "no scenario with action {}",
            action.as_str()
        );
    }
    let reasons: BTreeSet<NarrowingReason> = packet
        .scenarios
        .iter()
        .map(|s| s.published_narrowing_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(
            reasons.contains(&reason),
            "no scenario with reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn typed_repair_rung_carries_repair_id_and_others_do_not() {
    let packet = packet();
    for scenario in &packet.scenarios {
        if scenario.recovery_rung == RecoveryRung::TypedRepair {
            let id = scenario
                .repair_id
                .as_ref()
                .unwrap_or_else(|| panic!("{} typed repair without id", scenario.scenario_id));
            assert!(id.starts_with(DOCTOR_REPAIR_PREFIX));
        } else {
            assert!(
                scenario.repair_id.is_none(),
                "{} non-typed rung carries a repair id",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn initiating_findings_are_lane_scoped() {
    let packet = packet();
    for scenario in &packet.scenarios {
        let prefix = scenario.lane.finding_code_prefix();
        assert!(!scenario.initiating_findings.is_empty());
        for finding in &scenario.initiating_findings {
            assert!(
                finding.starts_with(&prefix),
                "{} finding {} not lane-scoped",
                scenario.scenario_id,
                finding
            );
        }
    }
}

#[test]
fn every_scenario_declares_p90_budget_and_observation() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.p90_budget().is_some(),
            "{} missing p90 budget",
            scenario.scenario_id
        );
        assert!(
            scenario.observed_p90_ms().is_some(),
            "{} missing observed p90",
            scenario.scenario_id
        );
    }
}

#[test]
fn scenarios_are_cross_surface_stable_and_carry_machine_keys() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.is_cross_surface_stable(),
            "{} not cross-surface stable",
            scenario.scenario_id
        );
        for key in REQUIRED_MACHINE_MEANING_KEYS {
            assert!(
                scenario.machine_meaning_keys.iter().any(|k| k == key),
                "{} missing machine key {key}",
                scenario.scenario_id
            );
        }
    }
}

#[test]
fn support_linkage_preserves_identity_and_is_metadata_safe() {
    let packet = packet();
    for scenario in &packet.scenarios {
        let linkage = &scenario.support_linkage;
        assert!(
            linkage.preserves_identity(),
            "{} does not preserve identity",
            scenario.scenario_id
        );
        assert!(
            linkage.is_metadata_safe(),
            "{} not metadata safe",
            scenario.scenario_id
        );
    }
}

#[test]
fn published_gate_matches_recomputed_decision() {
    let packet = packet();
    for scenario in &packet.scenarios {
        let (action, reason) = scenario.recompute_gate();
        assert_eq!(
            scenario.published_promotion_action, action,
            "{} action mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            scenario.published_narrowing_reason, reason,
            "{} reason mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn full_publications_are_fully_supported() {
    let packet = packet();
    for scenario in packet
        .scenarios
        .iter()
        .filter(|s| s.published_promotion_action == PromotionAction::PublishFull)
    {
        assert!(!scenario.is_stale(), "{}", scenario.scenario_id);
        assert_ne!(scenario.p90_latency_state(), LatencyState::Breached);
        assert!(scenario.is_escalation_complete());
        assert_eq!(scenario.drill_outcome, DrillOutcome::DiagnosedAndHandedOff);
    }
}

#[test]
fn stale_corpus_narrows_automatically() {
    let packet = packet();
    let stale: Vec<_> = packet.scenarios.iter().filter(|s| s.is_stale()).collect();
    assert!(!stale.is_empty(), "corpus needs a stale scenario");
    for scenario in stale {
        assert_eq!(
            scenario.published_promotion_action,
            PromotionAction::NarrowToAdvisory,
            "{} stale but not narrowed",
            scenario.scenario_id
        );
    }
}

#[test]
fn latency_breach_narrows_automatically() {
    let packet = packet();
    let breached: Vec<_> = packet
        .scenarios
        .iter()
        .filter(|s| s.p90_latency_state() == LatencyState::Breached)
        .collect();
    assert!(
        !breached.is_empty(),
        "corpus needs a latency-breached scenario"
    );
    for scenario in breached {
        assert_ne!(
            scenario.published_promotion_action,
            PromotionAction::PublishFull,
            "{} breached but published full",
            scenario.scenario_id
        );
    }
}

#[test]
fn export_projection_reflects_scenarios() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.scenarios.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert!(projection.raw_private_material_excluded);
    assert_eq!(
        projection.full_readiness_count,
        packet
            .scenarios
            .iter()
            .filter(|s| s.published_promotion_action == PromotionAction::PublishFull)
            .count()
    );
}

#[test]
fn validate_flags_gate_action_mismatch() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_promotion_action == PromotionAction::PublishFull)
        .expect("a publish_full scenario");
    // Stale the corpus: the gate should narrow, so the published full action
    // is now stale.
    scenario.corpus_age_days = scenario.freshness_window_days + 10;
    assert!(violation_ids(&packet).contains(&"m5_recovery.gate_action_mismatch".to_owned()));
}

#[test]
fn validate_flags_latency_breach_published_as_full() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_promotion_action == PromotionAction::PublishFull)
        .expect("a publish_full scenario");
    if let Some(observed) = scenario
        .observed_latencies
        .iter_mut()
        .find(|o| o.percentile == LatencyPercentile::P90)
    {
        observed.observed_ms = 1_000_000;
    }
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"m5_recovery.gate_action_mismatch".to_owned()));
    assert!(ids.contains(&"m5_recovery.full_publication_unsupported".to_owned()));
}

#[test]
fn validate_flags_missing_durable_evidence_for_full_publication() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_promotion_action == PromotionAction::PublishFull)
        .expect("a publish_full scenario");
    scenario.support_linkage.durable_evidence_refs = Vec::new();
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"m5_recovery.gate_action_mismatch".to_owned()));
    assert!(ids.contains(&"m5_recovery.full_publication_unsupported".to_owned()));
}

#[test]
fn validate_flags_finding_not_lane_scoped() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.initiating_findings = vec!["doctor.finding.other_lane.thing".to_owned()];
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.finding_lane_mismatch".to_owned()));
}

#[test]
fn validate_flags_finding_prefix() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.initiating_findings = vec!["illegal.finding".to_owned()];
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.finding_prefix".to_owned()));
}

#[test]
fn validate_flags_typed_repair_without_id() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.recovery_rung == RecoveryRung::TypedRepair)
    {
        scenario.repair_id = None;
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.repair_id_missing".to_owned()));
}

#[test]
fn validate_flags_repair_id_on_non_typed_rung() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.recovery_rung != RecoveryRung::TypedRepair)
    {
        scenario.repair_id = Some("repair.illegal".to_owned());
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.repair_id_unexpected".to_owned()));
}

#[test]
fn validate_flags_non_metadata_safe_linkage() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.support_linkage.raw_private_material_excluded = false;
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.linkage_not_metadata_safe".to_owned()));
}

#[test]
fn validate_flags_identity_not_preserved() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.support_linkage.preserved_scope_refs = Vec::new();
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.identity_not_preserved".to_owned()));
}

#[test]
fn validate_flags_missing_parity_surface() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario
            .parity_surfaces
            .retain(|s| *s != ParitySurface::PublicTruth);
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.parity_surface_missing".to_owned()));
}

#[test]
fn validate_flags_generic_explanation() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.explanation = "unavailable".to_owned();
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.explanation_generic".to_owned()));
}

#[test]
fn validate_flags_latency_budget_order() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        if let Some(budget) = scenario.first_actionable_latency_budgets.first_mut() {
            budget.yellow_ms = budget.target_ms;
        }
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.latency_budget_order".to_owned()));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.scenario_count = packet.summary.scenario_count.wrapping_add(1);
    assert!(violation_ids(&packet).contains(&"m5_recovery.summary_mismatch".to_owned()));
}

#[test]
fn validate_flags_duplicate_scenario_id() {
    let mut packet = packet();
    if packet.scenarios.len() >= 2 {
        let first = packet.scenarios[0].scenario_id.clone();
        packet.scenarios[1].scenario_id = first;
    }
    assert!(violation_ids(&packet).contains(&"m5_recovery.scenario_id_duplicate".to_owned()));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(RecoveryLane::NotebookKernel.as_str(), "notebook_kernel");
    assert_eq!(
        RecoveryLane::SyncDeviceRegistry.as_str(),
        "sync_device_registry"
    );
    assert_eq!(
        RecoveryRung::OpenWithoutRestore.as_str(),
        "open_without_restore"
    );
    assert_eq!(RecoveryRung::TypedRepair.as_str(), "typed_repair");
    assert_eq!(LatencyPercentile::P90.as_str(), "p90");
    assert_eq!(LatencyState::Breached.as_str(), "breached");
    assert_eq!(
        DrillOutcome::DiagnosedAndHandedOff.as_str(),
        "diagnosed_and_handed_off"
    );
    assert_eq!(
        PromotionAction::NarrowToAdvisory.as_str(),
        "narrow_to_advisory"
    );
    assert_eq!(NarrowingReason::StaleCorpus.as_str(), "stale_corpus");
    assert_eq!(ParitySurface::SupportExport.as_str(), "support_export");
    assert_eq!(
        RecoveryLane::NotebookKernel.finding_code_prefix(),
        "doctor.finding.notebook_kernel."
    );
}
