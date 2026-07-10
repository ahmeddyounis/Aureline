use super::*;

fn clean_power_input() -> M5PowerStateResolutionInput {
    M5PowerStateResolutionInput {
        indicator_id: "power-state:test".to_owned(),
        pressure_sources: vec![
            EfficiencyPressureSource::OsBatterySaver,
            EfficiencyPressureSource::ThermalPressure,
        ],
        active_state: EfficiencyState::ThermalConstrained,
        pressure_signal_available: true,
        distinct_causes_named: true,
        inspect_path: "diagnostics/efficiency".to_owned(),
        proof_fresh: true,
    }
}

fn clean_throttled_input() -> M5ThrottledResolutionInput {
    M5ThrottledResolutionInput {
        row_id: "throttled:test".to_owned(),
        slowed_workloads: vec![WorkloadFamily::SpeculativePrefetch],
        paused_workloads: vec![WorkloadFamily::AiWarmup],
        preserved_protected_tasks: vec!["save".to_owned()],
        adaptive_behavior_user_visible: true,
        surface_hides_slowed_work: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_power_throttle_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_POWER_THROTTLE_CONTROLS_PACKET_ID);
}

#[test]
fn power_state_clean_names_source_and_state_and_is_distinguishable() {
    let resolved = resolve_power_state_indicator(clean_power_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.distinguishable_cause);
    assert!(!resolved.collapses_into_generic_warning);
    assert_eq!(
        resolved.source_of_change,
        vec!["os_battery_saver", "thermal_pressure"]
    );
    assert_eq!(resolved.active_state, "ThermalConstrained");
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::Slowed
    );
}

#[test]
fn power_state_nominal_is_running_full_no_action() {
    let mut input = clean_power_input();
    input.pressure_sources = vec![EfficiencyPressureSource::AcPower];
    input.active_state = EfficiencyState::Nominal;
    let resolved = resolve_power_state_indicator(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::RunningFull
    );
    assert_eq!(
        resolved.next_action,
        M5PowerThrottleNextAction::NoActionNeeded
    );
}

#[test]
fn power_state_collapsed_causes_degrade_never_clean() {
    let mut input = clean_power_input();
    input.distinct_causes_named = false;
    let resolved = resolve_power_state_indicator(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PowerStateDegradeReason::CausesCollapsedIntoGeneric)
    );
    assert!(resolved.collapses_into_generic_warning);
    assert_eq!(
        resolved.work_disposition,
        M5EfficiencyWorkDisposition::NotEvaluated
    );
}

#[test]
fn power_state_unstated_source_degrades() {
    let mut input = clean_power_input();
    input.pressure_sources = vec![];
    let resolved = resolve_power_state_indicator(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PowerStateDegradeReason::SourceOfChangeUnstated)
    );
    assert!(!resolved.distinguishable_cause);
}

#[test]
fn power_state_signal_unavailable_degrades_first() {
    let mut input = clean_power_input();
    input.pressure_signal_available = false;
    let resolved = resolve_power_state_indicator(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PowerStateDegradeReason::PressureSignalUnavailable)
    );
}

#[test]
fn power_state_missing_inspect_path_degrades() {
    let mut input = clean_power_input();
    input.inspect_path = "  ".to_owned();
    let resolved = resolve_power_state_indicator(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5PowerStateDegradeReason::InspectPathMissing)
    );
}

#[test]
fn power_state_empty_id_and_forbidden_material_error() {
    let mut input = clean_power_input();
    input.indicator_id = "".to_owned();
    assert_eq!(
        resolve_power_state_indicator(input).unwrap_err(),
        M5PowerThrottleResolutionError::EmptyIndicatorId
    );

    let mut input = clean_power_input();
    input.inspect_path = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_power_state_indicator(input).unwrap_err(),
        M5PowerThrottleResolutionError::ForbiddenMaterial
    );
}

#[test]
fn throttled_clean_names_affected_and_preserved() {
    let resolved = resolve_throttled_subsystem_row(clean_throttled_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.affected_subsystems_named);
    assert!(!resolved.silently_hid_slowed_work);
    assert_eq!(resolved.slowed_workloads, vec!["speculative_prefetch"]);
    assert_eq!(resolved.paused_workloads, vec!["ai_warmup"]);
    assert!(resolved
        .work_dispositions
        .contains(&M5EfficiencyWorkDisposition::Slowed));
    assert!(resolved
        .work_dispositions
        .contains(&M5EfficiencyWorkDisposition::Paused));
}

#[test]
fn throttled_hiding_visible_slowed_work_degrades_ac2() {
    let mut input = clean_throttled_input();
    input.surface_hides_slowed_work = true;
    let resolved = resolve_throttled_subsystem_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ThrottledDegradeReason::SlowedWorkSilentlyHidden)
    );
    assert!(resolved.silently_hid_slowed_work);
}

#[test]
fn throttled_hiding_not_yet_visible_is_not_ac2_violation() {
    let mut input = clean_throttled_input();
    input.surface_hides_slowed_work = true;
    input.adaptive_behavior_user_visible = false;
    let resolved = resolve_throttled_subsystem_row(input).unwrap();
    assert!(!resolved.silently_hid_slowed_work);
    assert_ne!(
        resolved.degrade_reason,
        Some(M5ThrottledDegradeReason::SlowedWorkSilentlyHidden)
    );
}

#[test]
fn throttled_overlap_is_ambiguous() {
    let mut input = clean_throttled_input();
    input.slowed_workloads = vec![WorkloadFamily::PreviewRefresh];
    input.paused_workloads = vec![WorkloadFamily::PreviewRefresh];
    let resolved = resolve_throttled_subsystem_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ThrottledDegradeReason::SlowedVersusPausedAmbiguous)
    );
}

#[test]
fn throttled_missing_preserved_degrades() {
    let mut input = clean_throttled_input();
    input.preserved_protected_tasks = vec![];
    let resolved = resolve_throttled_subsystem_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ThrottledDegradeReason::WhatStillWorksUnstated)
    );
}

#[test]
fn throttled_none_named_degrades() {
    let mut input = clean_throttled_input();
    input.slowed_workloads = vec![];
    input.paused_workloads = vec![];
    let resolved = resolve_throttled_subsystem_row(input).unwrap();
    assert!(!resolved.affected_subsystems_named);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ThrottledDegradeReason::NoAffectedSubsystemNamed)
    );
}

#[test]
fn throttled_empty_id_and_forbidden_material_error() {
    let mut input = clean_throttled_input();
    input.row_id = "   ".to_owned();
    assert_eq!(
        resolve_throttled_subsystem_row(input).unwrap_err(),
        M5PowerThrottleResolutionError::EmptyRowId
    );

    let mut input = clean_throttled_input();
    input.preserved_protected_tasks = vec!["bearer abc123".to_owned()];
    assert_eq!(
        resolve_throttled_subsystem_row(input).unwrap_err(),
        M5PowerThrottleResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_power_throttle_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.vocabulary_set.work_dispositions.pop();
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_POWER_STATE_INDICATOR_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5PowerThrottleAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5PowerThrottleExportField::WorkDispositions);
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.controls_rows[0].throttled_subsystem_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    // Force a clean example to also read as silently-hidden — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.throttled_subsystem_examples[0].degrade_reason = None;
    row.throttled_subsystem_examples[0].silently_hid_slowed_work = true;
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_power_throttle_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.collapses_pressure_sources_into_generic_warning = true,
            1 => row.hides_slowed_work_after_user_visible = true,
            2 => row.hides_what_still_works = true,
            _ => row.invents_alternate_state_label = true,
        }
        assert!(packet
            .validate()
            .contains(&M5PowerThrottleControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_collapsed_example_removed() {
    let mut packet = seeded_m5_power_throttle_controls();
    // Drop every degraded power-state example so no collapsed / unstated cause remains.
    for row in &mut packet.controls_rows {
        row.power_state_examples.retain(|ex| ex.is_clean());
    }
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_hidden_example_removed() {
    let mut packet = seeded_m5_power_throttle_controls();
    for row in &mut packet.controls_rows {
        row.throttled_subsystem_examples.retain(|ex| {
            ex.degrade_reason != Some(M5ThrottledDegradeReason::SlowedWorkSilentlyHidden)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet
        .governance_review
        .no_surface_hides_slowed_work_after_user_visible = false;
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet
        .consumer_projection
        .support_export_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_power_throttle_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5PowerThrottleControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_power_throttle_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_power_throttle_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_power_throttle_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_power_throttle_controls_export()
        .expect("checked M5 power/throttle controls export validates");
    assert_eq!(from_disk.packet_id, M5_POWER_THROTTLE_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_power_throttle_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_power_throttle_controls_activity_center_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::ActivityCenterUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Beta);

    let preview = seeded_m5_power_throttle_controls_diagnostics_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EfficiencyConsumerSurface::DiagnosticsUi)
        .unwrap();
    assert_eq!(row.qualification, M5EfficiencyQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5PowerThrottleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-power-state-throttled-subsystem-controls/activity_center_beta_narrowed.json"
    )))
    .expect("activity-center fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_power_throttle_controls_activity_center_beta_narrowed()
    );

    let preview: M5PowerThrottleControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-power-state-throttled-subsystem-controls/diagnostics_preview_narrowed.json"
    )))
    .expect("diagnostics fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_power_throttle_controls_diagnostics_preview_narrowed()
    );
}
