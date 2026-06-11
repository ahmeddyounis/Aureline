use super::*;

fn packet() -> ProjectDoctorContainerBoundaryTruth {
    current_project_doctor_container_boundary_truth().expect("embedded packet parses")
}

fn violation_ids(packet: &ProjectDoctorContainerBoundaryTruth) -> Vec<String> {
    packet.validate().into_iter().map(|v| v.check_id).collect()
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_CONTAINER_BOUNDARY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_ref,
        PROJECT_DOCTOR_CONTAINER_BOUNDARY_SCHEMA_REF
    );
    assert_eq!(
        packet.overview_page,
        PROJECT_DOCTOR_CONTAINER_BOUNDARY_DOC_REF
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_records() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_surface_is_covered() {
    let packet = packet();
    for surface in WorkflowSurface::ALL {
        assert!(
            packet.scenarios_in_surface(surface).next().is_some(),
            "no scenario for surface {surface}"
        );
    }
}

#[test]
fn every_engine_class_is_exercised() {
    let packet = packet();
    let present: BTreeSet<EngineClass> = packet
        .scenarios
        .iter()
        .map(|s| s.engine_summary.engine_class)
        .collect();
    for class in EngineClass::ALL {
        assert!(present.contains(&class), "no scenario for engine {class}");
    }
}

#[test]
fn every_reachability_is_exercised() {
    let packet = packet();
    let present: BTreeSet<EngineReachability> = packet
        .scenarios
        .iter()
        .map(|s| s.engine_summary.reachability)
        .collect();
    for state in EngineReachability::ALL {
        assert!(
            present.contains(&state),
            "no scenario with reachability {}",
            state.as_str()
        );
    }
}

#[test]
fn every_support_class_is_exercised() {
    let packet = packet();
    let present: BTreeSet<SupportClass> = packet
        .scenarios
        .iter()
        .map(|s| s.engine_summary.support_class)
        .collect();
    for class in SupportClass::ALL {
        assert!(
            present.contains(&class),
            "no scenario with support class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_workspace_mode_and_boundary_label_is_exercised() {
    let packet = packet();
    let modes: BTreeSet<WorkspaceMode> =
        packet.scenarios.iter().map(|s| s.workspace_mode).collect();
    for mode in WorkspaceMode::ALL {
        assert!(modes.contains(&mode), "no scenario with mode {mode}");
    }
    let labels: BTreeSet<BoundaryLabel> =
        packet.scenarios.iter().map(|s| s.boundary_label).collect();
    for label in BoundaryLabel::ALL {
        assert!(
            labels.contains(&label),
            "no scenario with boundary label {}",
            label.as_str()
        );
    }
}

#[test]
fn every_definition_source_is_exercised() {
    let packet = packet();
    let present: BTreeSet<DefinitionSource> = packet
        .scenarios
        .iter()
        .map(|s| s.rebuild_review.definition_source)
        .collect();
    for source in DefinitionSource::ALL {
        assert!(
            present.contains(&source),
            "no scenario with definition source {}",
            source.as_str()
        );
    }
}

#[test]
fn every_log_availability_is_exercised() {
    let packet = packet();
    let present: BTreeSet<LogAvailability> = packet
        .scenarios
        .iter()
        .map(|s| s.log_truth.availability)
        .collect();
    for availability in LogAvailability::ALL {
        assert!(
            present.contains(&availability),
            "no scenario with log availability {}",
            availability.as_str()
        );
    }
}

#[test]
fn every_preflight_decision_and_reason_is_exercised() {
    let packet = packet();
    let decisions: BTreeSet<PreflightDecision> = packet
        .scenarios
        .iter()
        .map(|s| s.published_preflight_decision)
        .collect();
    for decision in PreflightDecision::ALL {
        assert!(
            decisions.contains(&decision),
            "no scenario with decision {}",
            decision.as_str()
        );
    }
    let reasons: BTreeSet<PreflightReason> = packet
        .scenarios
        .iter()
        .map(|s| s.published_preflight_reason)
        .collect();
    for reason in PreflightReason::ALL {
        assert!(
            reasons.contains(&reason),
            "no scenario with reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn initiating_findings_are_surface_scoped() {
    let packet = packet();
    for scenario in &packet.scenarios {
        let prefix = scenario.surface.finding_code_prefix();
        assert!(!scenario.initiating_findings.is_empty());
        for finding in &scenario.initiating_findings {
            assert!(
                finding.starts_with(&prefix),
                "{} finding {} not surface-scoped",
                scenario.scenario_id,
                finding
            );
        }
    }
}

#[test]
fn every_scenario_names_a_target_and_boundary() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            !scenario.target_ref.trim().is_empty(),
            "{} missing target_ref",
            scenario.scenario_id
        );
        assert!(
            scenario.boundary_is_consistent(),
            "{} boundary label inconsistent with mode",
            scenario.scenario_id
        );
    }
}

#[test]
fn unreachable_engines_offer_diagnostics_and_alternative() {
    let packet = packet();
    for scenario in packet
        .scenarios
        .iter()
        .filter(|s| !s.engine_summary.is_reachable())
    {
        assert!(
            !scenario.engine_summary.diagnostics_actions.is_empty(),
            "{} unreachable without diagnostics",
            scenario.scenario_id
        );
        assert!(
            !scenario
                .rebuild_review
                .stay_local_alternative
                .trim()
                .is_empty(),
            "{} blocked without a stay-local alternative",
            scenario.scenario_id
        );
        assert_eq!(
            scenario.published_preflight_decision,
            PreflightDecision::BlockedOfferAlternative,
            "{} unreachable but not blocked",
            scenario.scenario_id
        );
    }
}

#[test]
fn available_logs_carry_export_range_and_metadata_safe_posture() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.log_truth.export_range_present_when_available(),
            "{} available logs without export range",
            scenario.scenario_id
        );
        assert!(
            scenario.log_truth.is_metadata_safe(),
            "{} log posture not metadata safe",
            scenario.scenario_id
        );
    }
}

#[test]
fn trust_gated_hooks_force_disclosure() {
    let packet = packet();
    let with_hooks: Vec<_> = packet
        .scenarios
        .iter()
        .filter(|s| s.rebuild_review.has_trust_gated_hooks())
        .collect();
    assert!(
        !with_hooks.is_empty(),
        "corpus needs a trust-gated-hook scenario"
    );
    for scenario in with_hooks {
        assert_ne!(
            scenario.published_preflight_decision,
            PreflightDecision::ProceedFull,
            "{} trust-gated hook proceeds silently",
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
        let (decision, reason) = scenario.recompute_gate();
        assert_eq!(
            scenario.published_preflight_decision, decision,
            "{} decision mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            scenario.published_preflight_reason, reason,
            "{} reason mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn full_proceeds_are_fully_supported() {
    let packet = packet();
    for scenario in packet
        .scenarios
        .iter()
        .filter(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
    {
        assert!(
            scenario.engine_summary.is_reachable(),
            "{}",
            scenario.scenario_id
        );
        assert_ne!(
            scenario.engine_summary.support_class,
            SupportClass::Unsupported
        );
        assert!(!scenario.rebuild_review.has_trust_gated_hooks());
        assert!(!scenario.rebuild_review.has_side_effects());
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
        projection.proceed_full_count,
        packet
            .scenarios
            .iter()
            .filter(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
            .count()
    );
}

#[test]
fn validate_flags_gate_decision_mismatch_when_engine_goes_unreachable() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
        .expect("a proceed_full scenario");
    scenario.engine_summary.reachability = EngineReachability::Unreachable;
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"container_boundary.gate_decision_mismatch".to_owned()));
    assert!(ids.contains(&"container_boundary.full_proceed_unsupported".to_owned()));
}

#[test]
fn validate_flags_trust_gated_hook_published_as_proceed_full() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_preflight_decision == PreflightDecision::ProceedFull)
        .expect("a proceed_full scenario");
    scenario.rebuild_review.lifecycle_hooks.push(LifecycleHook {
        kind: LifecycleHookKind::PostCreate,
        command_ref: "hook:post_create:injected".to_owned(),
        trust_gated: true,
    });
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"container_boundary.trust_gated_hook_runs_silently".to_owned()));
    assert!(ids.contains(&"container_boundary.gate_decision_mismatch".to_owned()));
}

#[test]
fn validate_flags_missing_target_ref() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.target_ref = String::new();
    }
    assert!(violation_ids(&packet).contains(&"container_boundary.target_ref_missing".to_owned()));
}

#[test]
fn validate_flags_finding_not_surface_scoped() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.initiating_findings = vec!["doctor.finding.other_surface.thing".to_owned()];
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.finding_surface_mismatch".to_owned())
    );
}

#[test]
fn validate_flags_unreachable_without_diagnostics() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| !s.engine_summary.is_reachable())
    {
        scenario.engine_summary.diagnostics_actions = Vec::new();
    }
    assert!(violation_ids(&packet)
        .contains(&"container_boundary.no_diagnostics_when_unreachable".to_owned()));
}

#[test]
fn validate_flags_missing_stay_local_alternative() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.rebuild_review.stay_local_alternative = String::new();
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.no_stay_local_alternative".to_owned())
    );
}

#[test]
fn validate_flags_available_log_without_export_range() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.log_truth.availability.is_available())
    {
        scenario.log_truth.export_time_range_ref = String::new();
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.log_export_range_missing".to_owned())
    );
}

#[test]
fn validate_flags_non_metadata_safe_log_posture() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.log_truth.redaction_posture = "raw".to_owned();
    }
    assert!(violation_ids(&packet)
        .contains(&"container_boundary.log_posture_not_metadata_safe".to_owned()));
}

#[test]
fn validate_flags_boundary_label_inconsistent_with_mode() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.workspace_mode == WorkspaceMode::RemoteManaged)
    {
        scenario.boundary_label = BoundaryLabel::Local;
    }
    assert!(violation_ids(&packet)
        .contains(&"container_boundary.boundary_label_inconsistent".to_owned()));
}

#[test]
fn validate_flags_non_metadata_safe_linkage() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.support_linkage.raw_private_material_excluded = false;
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.linkage_not_metadata_safe".to_owned())
    );
}

#[test]
fn validate_flags_identity_not_preserved() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.support_linkage.preserved_scope_refs = Vec::new();
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.identity_not_preserved".to_owned())
    );
}

#[test]
fn validate_flags_missing_parity_surface() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario
            .parity_surfaces
            .retain(|s| *s != ParitySurface::IncidentPacket);
    }
    assert!(
        violation_ids(&packet).contains(&"container_boundary.parity_surface_missing".to_owned())
    );
}

#[test]
fn validate_flags_generic_explanation() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.explanation = "in_a_container".to_owned();
    }
    assert!(violation_ids(&packet).contains(&"container_boundary.explanation_generic".to_owned()));
}

#[test]
fn validate_flags_invalid_port_mapping() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| !s.rebuild_review.published_ports.is_empty())
    {
        scenario.rebuild_review.published_ports[0].host_port = 0;
    }
    assert!(violation_ids(&packet).contains(&"container_boundary.port_invalid".to_owned()));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.scenario_count = packet.summary.scenario_count.wrapping_add(1);
    assert!(violation_ids(&packet).contains(&"container_boundary.summary_mismatch".to_owned()));
}

#[test]
fn validate_flags_duplicate_scenario_id() {
    let mut packet = packet();
    if packet.scenarios.len() >= 2 {
        let first = packet.scenarios[0].scenario_id.clone();
        packet.scenarios[1].scenario_id = first;
    }
    assert!(violation_ids(&packet).contains(&"container_boundary.scenario_id_duplicate".to_owned()));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(WorkflowSurface::RemotePreview.as_str(), "remote_preview");
    assert_eq!(
        WorkflowSurface::IncidentWorkflow.as_str(),
        "incident_workflow"
    );
    assert_eq!(EngineClass::DevcontainersCli.as_str(), "devcontainers_cli");
    assert_eq!(EngineReachability::PolicyBlocked.as_str(), "policy_blocked");
    assert_eq!(SupportClass::Unsupported.as_str(), "unsupported");
    assert_eq!(WorkspaceMode::RemoteManaged.as_str(), "remote_managed");
    assert_eq!(BoundaryLabel::Managed.as_str(), "managed");
    assert_eq!(
        DefinitionSource::ManagedTemplate.as_str(),
        "managed_template"
    );
    assert_eq!(RebuildDecision::ReuseExisting.as_str(), "reuse_existing");
    assert_eq!(LifecycleHookKind::PostCreate.as_str(), "post_create");
    assert_eq!(PortVisibility::PublicTunnel.as_str(), "public_tunnel");
    assert_eq!(MountKind::HostPathBind.as_str(), "host_path_bind");
    assert_eq!(LogAvailability::Unavailable.as_str(), "unavailable");
    assert_eq!(
        PreflightDecision::BlockedOfferAlternative.as_str(),
        "blocked_offer_alternative"
    );
    assert_eq!(
        PreflightReason::SideEffectsRequireReview.as_str(),
        "side_effects_require_review"
    );
    assert_eq!(ParitySurface::BrowserHandoff.as_str(), "browser_handoff");
    assert_eq!(
        WorkflowSurface::RemotePreview.finding_code_prefix(),
        "doctor.finding.remote_preview."
    );
}
