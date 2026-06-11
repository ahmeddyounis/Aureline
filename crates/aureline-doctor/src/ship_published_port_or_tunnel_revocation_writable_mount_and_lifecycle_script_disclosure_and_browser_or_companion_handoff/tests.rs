use super::*;

fn packet() -> ProjectDoctorContainerHandoffTruth {
    current_project_doctor_container_handoff_truth().expect("embedded packet parses")
}

fn violation_ids(packet: &ProjectDoctorContainerHandoffTruth) -> Vec<String> {
    packet.validate().into_iter().map(|v| v.check_id).collect()
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_CONTAINER_HANDOFF_RECORD_KIND
    );
    assert_eq!(
        packet.schema_ref,
        PROJECT_DOCTOR_CONTAINER_HANDOFF_SCHEMA_REF
    );
    assert_eq!(
        packet.overview_page,
        PROJECT_DOCTOR_CONTAINER_HANDOFF_DOC_REF
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
    for surface in HandoffSurface::ALL {
        assert!(
            packet.scenarios_in_surface(surface).next().is_some(),
            "no scenario for surface {surface}"
        );
    }
}

#[test]
fn every_engine_class_is_exercised() {
    let packet = packet();
    let present: BTreeSet<EngineClass> = packet.scenarios.iter().map(|s| s.engine_class).collect();
    for class in EngineClass::ALL {
        assert!(present.contains(&class), "no scenario for engine {class}");
    }
}

#[test]
fn every_boundary_label_is_exercised() {
    let packet = packet();
    let present: BTreeSet<BoundaryLabel> =
        packet.scenarios.iter().map(|s| s.boundary_label).collect();
    for label in BoundaryLabel::ALL {
        assert!(present.contains(&label), "no scenario for boundary {label}");
    }
}

#[test]
fn every_route_kind_and_audience_is_exercised() {
    let packet = packet();
    let kinds: BTreeSet<RouteKind> = packet
        .scenarios
        .iter()
        .map(|s| s.route.route_kind)
        .collect();
    for kind in RouteKind::ALL {
        assert!(kinds.contains(&kind), "no scenario for route kind {kind}");
    }
    let audiences: BTreeSet<AudienceScope> = packet
        .scenarios
        .iter()
        .map(|s| s.route.audience_scope)
        .collect();
    for audience in AudienceScope::ALL {
        assert!(
            audiences.contains(&audience),
            "no scenario for audience {}",
            audience.as_str()
        );
    }
}

#[test]
fn every_policy_posture_and_time_bound_is_exercised() {
    let packet = packet();
    let postures: BTreeSet<PolicyPosture> = packet
        .scenarios
        .iter()
        .map(|s| s.route.policy_posture)
        .collect();
    for posture in PolicyPosture::ALL {
        assert!(
            postures.contains(&posture),
            "no scenario for policy posture {}",
            posture.as_str()
        );
    }
    let bounds: BTreeSet<TimeBoundClass> = packet
        .scenarios
        .iter()
        .map(|s| s.route.time_bound.class)
        .collect();
    for bound in TimeBoundClass::ALL {
        assert!(
            bounds.contains(&bound),
            "no scenario for time bound {}",
            bound.as_str()
        );
    }
}

#[test]
fn every_revocation_state_is_exercised() {
    let packet = packet();
    let present: BTreeSet<RevocationState> = packet
        .scenarios
        .iter()
        .map(|s| s.route.revocation.state)
        .collect();
    for state in RevocationState::ALL {
        assert!(
            present.contains(&state),
            "no scenario for revocation state {}",
            state.as_str()
        );
    }
}

#[test]
fn every_channel_liveness_and_mutation_scope_is_exercised() {
    let packet = packet();
    let channels: BTreeSet<HandoffChannel> =
        packet.scenarios.iter().map(|s| s.handoff.channel).collect();
    for channel in HandoffChannel::ALL {
        assert!(
            channels.contains(&channel),
            "no scenario for channel {}",
            channel.as_str()
        );
    }
    let livenesses: BTreeSet<HandoffLiveness> = packet
        .scenarios
        .iter()
        .map(|s| s.handoff.liveness)
        .collect();
    for liveness in HandoffLiveness::ALL {
        assert!(
            livenesses.contains(&liveness),
            "no scenario for liveness {}",
            liveness.as_str()
        );
    }
    let scopes: BTreeSet<HandoffMutationScope> = packet
        .scenarios
        .iter()
        .map(|s| s.handoff.mutation_scope)
        .collect();
    for scope in HandoffMutationScope::ALL {
        assert!(
            scopes.contains(&scope),
            "no scenario for mutation scope {}",
            scope.as_str()
        );
    }
}

#[test]
fn every_posture_and_reason_is_exercised() {
    let packet = packet();
    let postures: BTreeSet<HandoffPosture> = packet
        .scenarios
        .iter()
        .map(|s| s.published_handoff_posture)
        .collect();
    for posture in HandoffPosture::ALL {
        assert!(
            postures.contains(&posture),
            "no scenario for posture {}",
            posture.as_str()
        );
    }
    let reasons: BTreeSet<HandoffReason> = packet
        .scenarios
        .iter()
        .map(|s| s.published_handoff_reason)
        .collect();
    for reason in HandoffReason::ALL {
        assert!(
            reasons.contains(&reason),
            "no scenario for reason {}",
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
fn every_route_is_time_bound_and_revocable() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.route.time_bound.is_time_bound(),
            "{} route not time-bound",
            scenario.scenario_id
        );
        assert!(
            scenario.route.revocation.is_revocable(),
            "{} route not revocable",
            scenario.scenario_id
        );
    }
}

#[test]
fn dead_routes_back_snapshot_handoffs_with_visible_revocation() {
    let packet = packet();
    for scenario in packet
        .scenarios
        .iter()
        .filter(|s| s.route.revocation.state.is_dead())
    {
        assert_eq!(
            scenario.handoff.liveness,
            HandoffLiveness::Snapshot,
            "{} dead route with live handoff",
            scenario.scenario_id
        );
        assert!(
            scenario.handoff.revocation_visible,
            "{} dead route without visible revocation",
            scenario.scenario_id
        );
        assert!(
            !scenario
                .route
                .revocation
                .revoked_evidence_ref
                .trim()
                .is_empty(),
            "{} dead route without revocation evidence",
            scenario.scenario_id
        );
        assert_eq!(
            scenario.published_handoff_posture,
            HandoffPosture::ShareSnapshotOnly,
            "{} dead route not snapshot-only",
            scenario.scenario_id
        );
    }
}

#[test]
fn bounded_write_handoffs_are_approval_gated() {
    let packet = packet();
    let with_write: Vec<_> = packet
        .scenarios
        .iter()
        .filter(|s| s.handoff.mutation_scope == HandoffMutationScope::BoundedWrite)
        .collect();
    assert!(
        !with_write.is_empty(),
        "corpus needs a bounded-write scenario"
    );
    for scenario in with_write {
        assert!(
            scenario.handoff.approval_gated,
            "{} bounded write not approval-gated",
            scenario.scenario_id
        );
    }
}

#[test]
fn handoffs_preserve_attribution_and_match_route() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.handoff.preserves_attribution(),
            "{} handoff drops attribution",
            scenario.scenario_id
        );
        assert_eq!(scenario.handoff.route_id, scenario.route.route_id);
        assert_eq!(scenario.handoff.target_ref, scenario.route.target_ref);
        assert_eq!(
            scenario.handoff.target_service_ref,
            scenario.route.target_service_ref
        );
        assert_eq!(scenario.handoff.engine_class, scenario.engine_class);
    }
}

#[test]
fn environment_disclosure_survives_every_flow() {
    let packet = packet();
    for scenario in &packet.scenarios {
        assert!(
            scenario.environment_disclosure.survives_required_flows(),
            "{} disclosure missing a flow",
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
fn published_gate_matches_recomputed_posture() {
    let packet = packet();
    for scenario in &packet.scenarios {
        let (posture, reason) = scenario.recompute_gate();
        assert_eq!(
            scenario.published_handoff_posture, posture,
            "{} posture mismatch",
            scenario.scenario_id
        );
        assert_eq!(
            scenario.published_handoff_reason, reason,
            "{} reason mismatch",
            scenario.scenario_id
        );
    }
}

#[test]
fn share_live_scenarios_are_clean() {
    let packet = packet();
    for scenario in packet
        .scenarios
        .iter()
        .filter(|s| s.published_handoff_posture == HandoffPosture::ShareLive)
    {
        assert_eq!(scenario.route.policy_posture, PolicyPosture::PolicyAllowed);
        assert_eq!(scenario.route.revocation.state, RevocationState::Active);
        assert_ne!(scenario.route.audience_scope, AudienceScope::Public);
        assert_eq!(
            scenario.handoff.mutation_scope,
            HandoffMutationScope::ReadOnly
        );
        assert_eq!(scenario.handoff.liveness, HandoffLiveness::Live);
        assert!(!scenario.environment_disclosure.requires_disclosure());
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
        projection.share_live_count,
        packet
            .scenarios
            .iter()
            .filter(|s| s.published_handoff_posture == HandoffPosture::ShareLive)
            .count()
    );
    for row in &projection.rows {
        assert!(!row.revocation_action_ref.trim().is_empty());
        assert!(!row.expires_at_ref.trim().is_empty());
    }
}

#[test]
fn validate_flags_gate_mismatch_when_route_revoked() {
    let mut packet = packet();
    let scenario = packet
        .scenarios
        .iter_mut()
        .find(|s| s.published_handoff_posture == HandoffPosture::ShareLive)
        .expect("a share_live scenario");
    scenario.route.revocation.state = RevocationState::Revoked;
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"container_handoff.gate_posture_mismatch".to_owned()));
    assert!(ids.contains(&"container_handoff.dead_route_live_share".to_owned()));
}

#[test]
fn validate_flags_unbounded_route() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.route.time_bound.expires_at_ref = String::new();
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.route_not_time_bound".to_owned()));
}

#[test]
fn validate_flags_non_revocable_route() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.route.revocation.revocation_action_ref = String::new();
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.route_not_revocable".to_owned()));
}

#[test]
fn validate_flags_bounded_write_without_approval() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.handoff.mutation_scope == HandoffMutationScope::BoundedWrite)
    {
        scenario.handoff.approval_gated = false;
    }
    assert!(violation_ids(&packet)
        .contains(&"container_handoff.bounded_write_without_approval".to_owned()));
}

#[test]
fn validate_flags_handoff_route_mismatch() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.handoff.route_id = "route:other".to_owned();
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.handoff_route_mismatch".to_owned()));
}

#[test]
fn validate_flags_handoff_attribution_incomplete() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.handoff.owner_ref = String::new();
    }
    let ids = violation_ids(&packet);
    assert!(ids.contains(&"container_handoff.handoff_attribution_incomplete".to_owned()));
}

#[test]
fn validate_flags_disclosure_flow_missing() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario
            .environment_disclosure
            .disclosure_persists_in
            .retain(|f| *f != DisclosureFlow::SupportBundle);
    }
    assert!(
        violation_ids(&packet).contains(&"container_handoff.disclosure_flow_missing".to_owned())
    );
}

#[test]
fn validate_flags_dead_route_live_share() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.route.revocation.state.is_dead())
    {
        scenario.handoff.liveness = HandoffLiveness::Live;
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.dead_route_live_share".to_owned()));
}

#[test]
fn validate_flags_revocation_evidence_missing() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.route.revocation.state.is_dead())
    {
        scenario.route.revocation.revoked_evidence_ref = String::new();
    }
    assert!(violation_ids(&packet)
        .contains(&"container_handoff.revocation_evidence_missing".to_owned()));
}

#[test]
fn validate_flags_boundary_inconsistent_with_engine() {
    let mut packet = packet();
    if let Some(scenario) = packet
        .scenarios
        .iter_mut()
        .find(|s| s.engine_class == EngineClass::ManagedCloud)
    {
        scenario.boundary_label = BoundaryLabel::Local;
    }
    assert!(violation_ids(&packet)
        .contains(&"container_handoff.boundary_label_inconsistent".to_owned()));
}

#[test]
fn validate_flags_missing_stay_local_alternative() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.stay_local_alternative = String::new();
    }
    assert!(
        violation_ids(&packet).contains(&"container_handoff.no_stay_local_alternative".to_owned())
    );
}

#[test]
fn validate_flags_non_metadata_safe_linkage() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.support_linkage.raw_private_material_excluded = false;
    }
    assert!(
        violation_ids(&packet).contains(&"container_handoff.linkage_not_metadata_safe".to_owned())
    );
}

#[test]
fn validate_flags_generic_explanation() {
    let mut packet = packet();
    if let Some(scenario) = packet.scenarios.first_mut() {
        scenario.explanation = "shared".to_owned();
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.explanation_generic".to_owned()));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.scenario_count = packet.summary.scenario_count.wrapping_add(1);
    assert!(violation_ids(&packet).contains(&"container_handoff.summary_mismatch".to_owned()));
}

#[test]
fn validate_flags_duplicate_scenario_id() {
    let mut packet = packet();
    if packet.scenarios.len() >= 2 {
        let first = packet.scenarios[0].scenario_id.clone();
        packet.scenarios[1].scenario_id = first;
    }
    assert!(violation_ids(&packet).contains(&"container_handoff.scenario_id_duplicate".to_owned()));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(HandoffSurface::CompanionFollow.as_str(), "companion_follow");
    assert_eq!(EngineClass::DevcontainersCli.as_str(), "devcontainers_cli");
    assert_eq!(BoundaryLabel::Managed.as_str(), "managed");
    assert_eq!(RouteKind::PublishedPort.as_str(), "published_port");
    assert_eq!(RouteKind::Tunnel.as_str(), "tunnel");
    assert_eq!(
        AudienceScope::AuthenticatedTeam.as_str(),
        "authenticated_team"
    );
    assert_eq!(
        PolicyPosture::PolicyRestricted.as_str(),
        "policy_restricted"
    );
    assert_eq!(TimeBoundClass::SessionBound.as_str(), "session_bound");
    assert_eq!(RevocationState::Revoked.as_str(), "revoked");
    assert_eq!(MountKind::HostPathBind.as_str(), "host_path_bind");
    assert_eq!(ScriptKind::InstallScript.as_str(), "install_script");
    assert_eq!(DisclosureFlow::SupportBundle.as_str(), "support_bundle");
    assert_eq!(HandoffChannel::Companion.as_str(), "companion");
    assert_eq!(HandoffLiveness::Snapshot.as_str(), "snapshot");
    assert_eq!(HandoffMutationScope::BoundedWrite.as_str(), "bounded_write");
    assert_eq!(
        HandoffPosture::ShareSnapshotOnly.as_str(),
        "share_snapshot_only"
    );
    assert_eq!(
        HandoffReason::BoundedWriteRequiresApproval.as_str(),
        "bounded_write_requires_approval"
    );
    assert_eq!(
        ParitySurface::CompanionHandoff.as_str(),
        "companion_handoff"
    );
    assert_eq!(
        HandoffSurface::CompanionFollow.finding_code_prefix(),
        "doctor.finding.companion_follow."
    );
}
