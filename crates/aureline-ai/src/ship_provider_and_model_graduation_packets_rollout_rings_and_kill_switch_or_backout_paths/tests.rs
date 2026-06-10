use super::*;

const PACKET_ID: &str = "provider-model-graduation:stable:0001";

fn proof_stale_to(
    narrowed_to: M5AiWorkflowQualificationClass,
) -> ProviderModelGraduationDowngradeRule {
    ProviderModelGraduationDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn ga_stable_route() -> ProviderModelGraduationRow {
    ProviderModelGraduationRow {
        route_id: "flagship-managed".to_owned(),
        provider_id: "aureline-managed".to_owned(),
        model_id: "flagship".to_owned(),
        route_label: "Managed flagship route".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        current_ring: RolloutRingClass::GeneralAvailability,
        ring_state: RingProgressState::Complete,
        kill_switch: GraduationKillSwitch {
            scope: KillSwitchScopeClass::RouteScoped,
            state: KillSwitchState::Armed,
            fails_closed: true,
            label: "Route-scoped halt; fails closed on ambiguity".to_owned(),
        },
        backout: GraduationBackoutPath {
            posture: M5AiWorkflowRollbackPosture::FullyReversible,
            verified: true,
            label: "Reverts to the prior broad ring with no data loss".to_owned(),
        },
        downgrade_rules: vec![
            proof_stale_to(M5AiWorkflowQualificationClass::Beta),
            ProviderModelGraduationDowngradeRule {
                trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
                narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
                auto_enforced: true,
                rationale: "An unavailable provider makes the route unavailable".to_owned(),
            },
        ],
        evidence_packet_refs: vec!["evidence:flagship-managed:m5".to_owned()],
    }
}

fn broad_beta_route() -> ProviderModelGraduationRow {
    ProviderModelGraduationRow {
        route_id: "byok-broad".to_owned(),
        provider_id: "byok-vendor".to_owned(),
        model_id: "mid".to_owned(),
        route_label: "BYOK mid route in broad rollout".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        current_ring: RolloutRingClass::Broad,
        ring_state: RingProgressState::Rolling,
        kill_switch: GraduationKillSwitch {
            scope: KillSwitchScopeClass::ProviderScoped,
            state: KillSwitchState::Armed,
            fails_closed: true,
            label: "Provider-scoped halt across every BYOK route".to_owned(),
        },
        backout: GraduationBackoutPath {
            posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
            verified: true,
            label: "Rolls back to the early-access ring from a checkpoint".to_owned(),
        },
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Preview)],
        evidence_packet_refs: vec!["evidence:byok-broad:m5".to_owned()],
    }
}

fn canary_preview_route() -> ProviderModelGraduationRow {
    ProviderModelGraduationRow {
        route_id: "local-canary".to_owned(),
        provider_id: "local-runtime".to_owned(),
        model_id: "small".to_owned(),
        route_label: "Local small route in canary".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Preview,
        current_ring: RolloutRingClass::Canary,
        ring_state: RingProgressState::Rolling,
        kill_switch: GraduationKillSwitch {
            scope: KillSwitchScopeClass::RouteScoped,
            state: KillSwitchState::Armed,
            fails_closed: true,
            label: "Route-scoped halt for the canary cohort".to_owned(),
        },
        backout: GraduationBackoutPath {
            posture: M5AiWorkflowRollbackPosture::FullyReversible,
            verified: true,
            label: "Disables the canary route with no residue".to_owned(),
        },
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Experimental)],
        evidence_packet_refs: vec!["evidence:local-canary:m5".to_owned()],
    }
}

fn backed_out_held_route() -> ProviderModelGraduationRow {
    ProviderModelGraduationRow {
        route_id: "regressed-route".to_owned(),
        provider_id: "byok-vendor".to_owned(),
        model_id: "experimental".to_owned(),
        route_label: "Regressed route rolled back from broad".to_owned(),
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        current_ring: RolloutRingClass::Broad,
        ring_state: RingProgressState::BackedOut,
        kill_switch: GraduationKillSwitch {
            scope: KillSwitchScopeClass::GlobalAllRoutes,
            state: KillSwitchState::Fired,
            fails_closed: true,
            label: "Global halt fired after a regression".to_owned(),
        },
        backout: GraduationBackoutPath {
            posture: M5AiWorkflowRollbackPosture::FullyReversible,
            verified: true,
            label: "Restored the prior route with no data loss".to_owned(),
        },
        downgrade_rules: vec![proof_stale_to(M5AiWorkflowQualificationClass::Unavailable)],
        evidence_packet_refs: vec![],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PROVIDER_MODEL_GRADUATION_SCHEMA_REF.to_owned(),
        PROVIDER_MODEL_GRADUATION_DOC_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> ProviderModelGraduationProofFreshness {
    ProviderModelGraduationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> ProviderModelGraduationPacket {
    ProviderModelGraduationPacket::new(ProviderModelGraduationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Provider And Model Graduation And Rollout".to_owned(),
        routes: vec![
            ga_stable_route(),
            broad_beta_route(),
            canary_preview_route(),
            backed_out_held_route(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    })
}

#[test]
fn graduation_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn ring_ordering_is_monotonic() {
    let ranks: Vec<u8> = RolloutRingClass::ALL
        .iter()
        .map(|ring| ring.rank())
        .collect();
    let mut sorted = ranks.clone();
    sorted.sort_unstable();
    assert_eq!(ranks, sorted);
    assert!(RolloutRingClass::GeneralAvailability.is_broad_exposure());
    assert!(RolloutRingClass::Broad.is_broad_exposure());
    assert!(!RolloutRingClass::Canary.is_broad_exposure());
}

#[test]
fn ring_state_partition() {
    assert!(RingProgressState::KillSwitched.is_halted());
    assert!(RingProgressState::BackedOut.is_halted());
    assert!(RingProgressState::Rolling.is_advancing());
    assert!(RingProgressState::Complete.is_advancing());
    assert!(!RingProgressState::Pending.is_halted());
    assert!(!RingProgressState::Pending.is_advancing());
}

#[test]
fn counts_match_fixture() {
    let packet = packet();
    assert_eq!(packet.claimed_route_count(), 3);
    assert_eq!(packet.broad_exposure_route_count(), 3);
    assert_eq!(packet.ga_route_count(), 1);
    assert_eq!(packet.backed_out_route_count(), 1);
    assert_eq!(packet.kill_switched_route_count(), 0);
    assert_eq!(packet.armed_kill_switch_count(), 4);
}

#[test]
fn no_routes_fails() {
    let mut packet = packet();
    packet.routes.clear();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::NoRoutes));
}

#[test]
fn duplicate_route_fails() {
    let mut packet = packet();
    let first = packet.routes[0].clone();
    packet.routes.push(first);
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::DuplicateRoute));
}

#[test]
fn route_row_incomplete_fails() {
    let mut packet = packet();
    packet.routes[0].kill_switch.label.clear();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::RouteRowIncomplete));
}

#[test]
fn kill_switch_not_fail_closed_fails() {
    let mut packet = packet();
    // A kill switch that fails open is never acceptable, claimed or not.
    packet.routes[3].kill_switch.fails_closed = false;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::KillSwitchNotFailClosed));
}

#[test]
fn claimed_route_kill_switch_not_armed_fails() {
    let mut packet = packet();
    // A claimed route may never run without an armed halt path.
    packet.routes[2].kill_switch.state = KillSwitchState::NotArmed;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::ClaimedRouteKillSwitchNotArmed));
}

#[test]
fn broad_exposure_without_backout_path_fails() {
    let mut packet = packet();
    // A claimed broad route may not rely on a non-reversing backout posture.
    packet.routes[1].backout.posture = M5AiWorkflowRollbackPosture::NotApplicable;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::BroadExposureWithoutBackoutPath));
}

#[test]
fn broad_exposure_backout_unverified_fails() {
    let mut packet = packet();
    packet.routes[1].backout.verified = false;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::BroadExposureBackoutUnverified));
}

#[test]
fn ga_ring_without_stable_claim_fails() {
    let mut packet = packet();
    // General availability may only carry a Stable claim.
    packet.routes[0].claimed_qualification = M5AiWorkflowQualificationClass::Beta;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::GaRingWithoutStableClaim));
}

#[test]
fn halted_route_claims_stable_fails() {
    let mut packet = packet();
    // A backed-out route may not keep claiming Stable. Move it off GA so the GA
    // check does not mask the halt check, and give it narrowing rules.
    packet.routes[3].claimed_qualification = M5AiWorkflowQualificationClass::Stable;
    packet.routes[3].current_ring = RolloutRingClass::Broad;
    packet.routes[3].downgrade_rules = vec![proof_stale_to(M5AiWorkflowQualificationClass::Beta)];
    packet.routes[3].evidence_packet_refs = vec!["evidence:regressed-route:m5".to_owned()];
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::HaltedRouteClaimsStable));
}

#[test]
fn claimed_route_missing_evidence_fails() {
    let mut packet = packet();
    packet.routes[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::ClaimedRouteMissingEvidence));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.routes[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.routes[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.routes[0].downgrade_rules[0].narrowed_to = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::MissingSourceContracts));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::ProofFreshnessIncomplete));
}

#[test]
fn raw_boundary_material_in_export_fails() {
    let mut packet = packet();
    // A raw provider endpoint must never cross the support boundary.
    packet.routes[1].kill_switch.label = "https://api.vendor.example/halt".to_owned();
    assert!(packet
        .validate()
        .contains(&ProviderModelGraduationViolation::RawBoundaryMaterialInExport));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let route = broad_beta_route();
    assert_eq!(
        route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Preview
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Beta
    );
}

#[test]
fn render_inspector_lists_every_posture() {
    let card = broad_beta_route().render_inspector();
    assert!(card.contains("byok-broad"));
    assert!(card.contains("broad"));
    assert!(card.contains("rolling"));
    assert!(card.contains("provider_scoped"));
    assert!(card.contains("armed"));
    assert!(card.contains("checkpoint_reversible"));
}

#[test]
fn markdown_summary_lists_every_route() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Route inspectors"));
    for route in &packet().routes {
        assert!(
            summary.contains(&route.route_id),
            "missing {}",
            route.route_id
        );
    }
}

#[test]
fn kill_switch_fired_backout_fixture_validates() {
    let packet: ProviderModelGraduationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_provider_and_model_graduation_packets_rollout_rings_and_kill_switch_or_backout_paths/kill_switch_fired_backout_narrowed.json"
    )))
    .expect("kill-switch fired fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The kill-switched route is halted, no longer claimed, and narrows to
    // unavailable on stale proof.
    let halted: Vec<&ProviderModelGraduationRow> = packet
        .routes
        .iter()
        .filter(|route| route.ring_state.is_halted())
        .collect();
    assert!(!halted.is_empty());
    for route in halted {
        assert!(!route.is_claimed());
        assert!(route.kill_switch.fails_closed);
        assert_eq!(
            route.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
            M5AiWorkflowQualificationClass::Unavailable
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_provider_model_graduation_export()
        .expect("checked provider/model graduation export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert!(!packet.routes.is_empty());
    assert!(packet.ga_route_count() >= 1);
    assert!(packet.broad_exposure_route_count() >= 1);
    // Every route carries a fail-closed kill switch.
    assert!(packet
        .routes
        .iter()
        .all(|route| route.kill_switch.fails_closed));
}
