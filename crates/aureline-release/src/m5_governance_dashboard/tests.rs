//! Inline tests for the M5 governance-dashboard lane.

use super::*;

fn packet() -> M5GovernanceDashboard {
    seeded_m5_governance_dashboard()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_GOVERNANCE_DASHBOARD_PACKET_ID);
    assert_eq!(packet.record_kind, M5_GOVERNANCE_DASHBOARD_RECORD_KIND);
    assert_eq!(packet.fitness_tiles.len(), FitnessFunction::ALL.len());
    assert_eq!(packet.nightly_rows.len(), FitnessFunction::ALL.len());
    assert_eq!(packet.service_cards.len(), Service::ALL.len());
    assert_eq!(packet.decision_right_cards.len(), DecisionRight::ALL.len());
    assert_eq!(packet.overviews.len(), ClaimedPosture::ALL.len());
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_board_is_all_passing() {
    // Acceptance: with every function passing, every service is clean, every decision exercisable,
    // every profile honored, and Stable promotion is not blocked.
    let packet = packet();
    for tile in &packet.fitness_tiles {
        assert_eq!(
            tile.state,
            FitnessState::Passing,
            "{}",
            tile.function.as_str()
        );
        assert_eq!(tile.gate, DescriptorGate::Governed);
        assert_eq!(tile.effective_qualification, QualificationClass::Stable);
        assert!(tile.waiver_standing.is_none());
    }
    for card in &packet.service_cards {
        assert!(card.is_clean());
        assert_eq!(card.worst_state, FitnessState::Passing);
    }
    for card in &packet.decision_right_cards {
        assert!(card.is_exercisable());
        assert_eq!(card.posture, DecisionPosture::Clear);
    }
    for overview in &packet.overviews {
        assert_eq!(overview.effective_posture, overview.profile);
        assert_eq!(overview.gate_decision, DescriptorGate::Governed);
    }
    assert!(packet.waiver_queue.is_empty());
    assert!(!packet.blocks_stable_promotion());
    assert_eq!(
        packet.summary.honored_profiles,
        ClaimedPosture::ALL.len() as u32
    );
}

#[test]
fn every_function_is_tiled_once_and_bound_to_owner_and_forum() {
    // Acceptance: freshness and ownership are first-class; each function names a service, owner,
    // forum, evidence class, and proof ref.
    let packet = packet();
    for function in FitnessFunction::ALL {
        let tiles: Vec<&FitnessTile> = packet
            .fitness_tiles
            .iter()
            .filter(|t| t.function == function)
            .collect();
        assert_eq!(tiles.len(), 1, "{}", function.as_str());
        let tile = tiles[0];
        assert_eq!(tile.service, function.service());
        assert_eq!(tile.owner_role, function.owner_role());
        assert_eq!(tile.forum, function.service().forum());
        assert_eq!(tile.proof_ref, function.proof_ref());
        assert_eq!(tile.evidence_class, function.evidence_class());
        assert!(!tile.proof_ref.trim().is_empty());
        assert!(!tile.corpus_id.trim().is_empty());
    }
}

#[test]
fn tile_state_derives_from_inputs_and_never_overstates() {
    let packet = packet();
    assert!(packet.conformance.tile_state_derived_from_inputs);
    for tile in &packet.fitness_tiles {
        let expected =
            derive_fitness_state(tile.measure, tile.evidence_freshness, tile.waiver_standing);
        assert_eq!(tile.state, expected);
        assert_eq!(tile.gate, expected.gate());
    }
}

#[test]
fn warning_drill_reads_warning_not_a_clean_pass() {
    // Acceptance: a warning stays distinct from a pass and from a fail.
    let packet = seeded_m5_governance_dashboard_warning();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    let tile = packet.tile(FitnessFunction::SchemaExampleParity).unwrap();
    assert_eq!(tile.state, FitnessState::Warning);
    assert_eq!(tile.gate, DescriptorGate::Narrowed);
    assert_eq!(tile.signal, DescriptorSignal::Yellow);
    assert!(!packet.blocks_stable_promotion());
    // A managed-scoped warning narrows every profile.
    for overview in &packet.overviews {
        assert_eq!(overview.gate_decision, DescriptorGate::Narrowed);
    }
}

#[test]
fn stale_evidence_narrows_exactly_the_profiles_that_require_it() {
    // Acceptance: a stale item never renders as a clean pass; it narrows deterministically.
    let packet = seeded_m5_governance_dashboard_evidence_stale_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.conformance.stale_evidence_narrows_deterministically);

    let tile = packet.tile(FitnessFunction::ClaimNoOverclaim).unwrap();
    assert_eq!(tile.evidence_freshness, FreshnessState::Stale);
    assert_eq!(tile.state, FitnessState::EvidenceStale);
    assert_eq!(tile.gate, DescriptorGate::Narrowed);

    // ClaimNoOverclaim is required under regulated; managed / self-hosted stay governed.
    assert_eq!(
        packet
            .overview(ClaimedPosture::Managed)
            .unwrap()
            .gate_decision,
        DescriptorGate::Governed
    );
    assert_eq!(
        packet
            .overview(ClaimedPosture::SelfHosted)
            .unwrap()
            .gate_decision,
        DescriptorGate::Governed
    );
    assert_eq!(
        packet
            .overview(ClaimedPosture::Regulated)
            .unwrap()
            .gate_decision,
        DescriptorGate::Narrowed
    );
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn active_waiver_narrows_and_discloses_the_queue_row() {
    // Acceptance: a waived item never renders as a clean pass; the queue discloses expiry and action.
    let packet = seeded_m5_governance_dashboard_waiver_active_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let tile = packet.tile(FitnessFunction::EvidenceFreshnessSlo).unwrap();
    assert_eq!(tile.state, FitnessState::Waived);
    assert_eq!(tile.gate, DescriptorGate::Narrowed);
    assert_eq!(tile.waiver_standing, Some(WaiverStanding::Active));

    assert_eq!(packet.waiver_queue.len(), 1);
    let row = &packet.waiver_queue[0];
    assert_eq!(row.function, FitnessFunction::EvidenceFreshnessSlo);
    assert_eq!(row.queue_state, WaiverStanding::Active);
    assert_eq!(row.tile_state, FitnessState::Waived);
    assert!(!row.expiry.trim().is_empty());
    assert!(!row.rationale.trim().is_empty());
    assert!(!row.ticket_ref.trim().is_empty());
    assert_eq!(packet.summary.open_waivers, 1);
    assert_eq!(packet.summary.expired_waivers, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn expired_waiver_blocks_and_heads_the_queue() {
    // Acceptance: an expired waiver is distinct from an active one and blocks Stable promotion.
    let packet = seeded_m5_governance_dashboard_waiver_expired_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(
        packet
            .conformance
            .missing_or_expired_blocks_stable_promotion
    );

    let tile = packet
        .tile(FitnessFunction::ProvenanceCompleteness)
        .unwrap();
    assert_eq!(tile.state, FitnessState::WaiverExpired);
    assert_eq!(tile.gate, DescriptorGate::Blocked);
    assert_eq!(tile.signal, DescriptorSignal::Red);

    let row = &packet.waiver_queue[0];
    assert_eq!(row.queue_state, WaiverStanding::Expired);
    assert_eq!(row.tile_state, FitnessState::WaiverExpired);

    let sovereign = packet.overview(ClaimedPosture::Sovereign).unwrap();
    assert_eq!(sovereign.gate_decision, DescriptorGate::Blocked);
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.expired_waivers, 1);
    assert_eq!(packet.summary.waiver_expired, 1);
}

#[test]
fn missing_evidence_blocks_exactly_the_profile_that_reads_it() {
    let packet = seeded_m5_governance_dashboard_missing_evidence_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let tile = packet.tile(FitnessFunction::RouteExplainability).unwrap();
    assert_eq!(tile.evidence_freshness, FreshnessState::Missing);
    assert_eq!(tile.state, FitnessState::Blocked);
    assert_eq!(tile.gate, DescriptorGate::Blocked);

    // RouteExplainability is required only under sovereign; weaker profiles stay governed.
    assert_eq!(
        packet
            .overview(ClaimedPosture::Regulated)
            .unwrap()
            .gate_decision,
        DescriptorGate::Governed
    );
    let sovereign = packet.overview(ClaimedPosture::Sovereign).unwrap();
    assert_eq!(sovereign.gate_decision, DescriptorGate::Blocked);
    assert_ne!(sovereign.effective_posture, ClaimedPosture::Sovereign);
    assert!(packet.blocks_stable_promotion());
    assert_eq!(packet.summary.blocked, 1);
}

#[test]
fn waived_or_stale_never_renders_a_clean_pass() {
    // Guardrail: a waived, stale, or waiver-expired item never reads green.
    for packet in [
        seeded_m5_governance_dashboard_warning(),
        seeded_m5_governance_dashboard_evidence_stale_narrowed(),
        seeded_m5_governance_dashboard_waiver_active_narrowed(),
        seeded_m5_governance_dashboard_waiver_expired_blocked(),
    ] {
        assert!(packet.conformance.waived_or_stale_never_clean_pass);
        for tile in &packet.fitness_tiles {
            if !matches!(tile.state, FitnessState::Passing) {
                assert_ne!(tile.gate, DescriptorGate::Governed);
                assert_ne!(tile.signal, DescriptorSignal::Green);
            }
        }
    }
}

#[test]
fn waiver_queue_orders_expired_before_active() {
    // Two waivers, one expired and one active; the expired one heads the queue.
    let mut states: Vec<FitnessFunctionState> = FitnessFunction::ALL
        .iter()
        .map(|function| FitnessFunctionState {
            function: *function,
            measure: FitnessMeasure::Pass,
            freshness: FreshnessState::Current,
            last_run_at: "2026-07-06T00:00:00Z".to_owned(),
            consecutive_passing_runs: 10,
            waiver: None,
        })
        .collect();
    // PackageBoundaryIntegrity (rank 0) gets an ACTIVE waiver; ProvenanceCompleteness (rank 6) gets
    // an EXPIRED waiver. Despite the function order, the expired one must sort first.
    states[0].measure = FitnessMeasure::Fail;
    states[0].waiver = Some(WaiverSeed {
        standing: WaiverStanding::Active,
        expiry: "2026-12-01T00:00:00Z".to_owned(),
        rationale: "Boundary check waived during refactor.".to_owned(),
        responsible_party: WaiverParty::ServiceOwner,
        action: WaiverClearingAction::RemediateAndReverify,
        ticket_ref: "gov-waiver-a".to_owned(),
    });
    states[6].measure = FitnessMeasure::Fail;
    states[6].waiver = Some(WaiverSeed {
        standing: WaiverStanding::Expired,
        expiry: "2026-04-01T00:00:00Z".to_owned(),
        rationale: "Provenance waiver lapsed.".to_owned(),
        responsible_party: WaiverParty::GovernanceOwner,
        action: WaiverClearingAction::RenewWaiver,
        ticket_ref: "gov-waiver-b".to_owned(),
    });
    let packet = M5GovernanceDashboard::new(M5GovernanceDashboardInput {
        packet_id: "m5-governance-dashboard:test-queue:0001".to_owned(),
        report_label: "queue ordering".to_owned(),
        corpus_id: "m5-reference-corpus:0001".to_owned(),
        corpus_label: "M5 reference corpus".to_owned(),
        evaluated_at: "2026-07-06T00:00:00Z".to_owned(),
        function_states: states,
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-07-06T00:00:00Z".to_owned(),
    });
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.waiver_queue.len(), 2);
    assert_eq!(packet.waiver_queue[0].queue_state, WaiverStanding::Expired);
    assert_eq!(packet.waiver_queue[1].queue_state, WaiverStanding::Active);
    assert!(packet.blocks_stable_promotion());
}

#[test]
fn service_and_decision_cards_bind_owner_and_forum() {
    let packet = packet();
    assert!(packet.conformance.service_cards_bind_owner_and_forum);
    assert!(packet.conformance.decision_cards_bind_owner_and_forum);
    for card in &packet.service_cards {
        assert_eq!(card.owner_role, card.service.owner_role());
        assert_eq!(card.forum, card.service.forum());
        assert!(!card.governed_functions.is_empty());
    }
    for card in &packet.decision_right_cards {
        assert_eq!(card.accountable_owner, card.decision.accountable_owner());
        assert_eq!(card.forum, card.decision.forum());
        assert!(!card.governed_services.is_empty());
    }
}

#[test]
fn boundary_change_decision_is_scoped_to_package_governance() {
    // A blocked route function does not hold the boundary-change decision, which is scoped narrowly.
    let packet = seeded_m5_governance_dashboard_missing_evidence_blocked();
    let promotion = packet
        .decision_card(DecisionRight::StablePromotion)
        .unwrap();
    assert_eq!(promotion.posture, DecisionPosture::Held);
    let boundary = packet.decision_card(DecisionRight::BoundaryChange).unwrap();
    assert_eq!(boundary.posture, DecisionPosture::Clear);
    assert_eq!(boundary.governed_services, vec![Service::PackageGovernance]);
}

#[test]
fn overview_effective_posture_never_overstates() {
    let packet = seeded_m5_governance_dashboard_waiver_expired_blocked();
    assert!(
        packet
            .conformance
            .overview_effective_posture_never_overstated
    );
    for overview in &packet.overviews {
        assert!(posture_rank(overview.effective_posture) <= posture_rank(overview.profile));
    }
}

#[test]
fn corpus_identity_is_bound_everywhere() {
    // Acceptance: dashboards and exports are bound to corpus identity.
    let packet = packet();
    assert!(packet.conformance.corpus_identity_bound);
    for tile in &packet.fitness_tiles {
        assert_eq!(tile.corpus_id, packet.corpus_id);
    }
    for row in &packet.nightly_rows {
        assert_eq!(row.corpus_id, packet.corpus_id);
    }
    for overview in &packet.overviews {
        assert_eq!(overview.corpus_id, packet.corpus_id);
    }
    assert_eq!(packet.evaluation_packet.corpus_id, packet.corpus_id);
}

#[test]
fn evaluation_packet_reuses_the_ui_vocabulary() {
    // Acceptance: exports reuse the same state and proof vocabulary the dashboard shows.
    for packet in [
        packet(),
        seeded_m5_governance_dashboard_waiver_active_narrowed(),
        seeded_m5_governance_dashboard_waiver_expired_blocked(),
    ] {
        assert!(packet.conformance.evaluation_packet_reuses_ui_vocabulary);
        let export = &packet.evaluation_packet;
        assert!(export.vocabulary.matches_canonical());
        for entry in &export.tiles {
            let tile = packet.tile(entry.function).unwrap();
            assert_eq!(entry.state, tile.state);
            assert_eq!(entry.gate, tile.gate);
            assert_eq!(entry.evidence_freshness, tile.evidence_freshness);
            assert_eq!(entry.proof_ref, tile.proof_ref);
        }
        for entry in &export.waivers {
            let row = packet
                .waiver_queue
                .iter()
                .find(|w| w.function == entry.function)
                .unwrap();
            assert_eq!(entry.queue_state, row.queue_state);
            assert_eq!(entry.tile_state, row.tile_state);
        }
    }
}

#[test]
fn channels_produce_identical_output() {
    let packet = packet();
    let desktop = packet.render_for_channel(GovernanceDashboardChannel::DesktopUi);
    let cli = packet.render_for_channel(GovernanceDashboardChannel::CliHeadless);
    let offline = packet.render_for_channel(GovernanceDashboardChannel::OfflineMirror);
    assert_eq!(desktop, cli);
    assert_eq!(cli, offline);
    assert_eq!(desktop, packet.export_safe_json());
}

#[test]
fn controlled_vocabulary_is_frozen() {
    let vocab = GovernanceDashboardVocabulary::canonical();
    assert_eq!(vocab.fitness_functions.len(), FitnessFunction::ALL.len());
    assert_eq!(vocab.fitness_states.len(), FitnessState::ALL.len());
    for needle in [
        "passing",
        "warning",
        "evidence_stale",
        "waived",
        "waiver_expired",
        "blocked",
    ] {
        assert!(vocab.fitness_states.contains(&needle.to_owned()));
    }
    for needle in ["active", "expiring_soon", "expired"] {
        assert!(vocab.waiver_standings.contains(&needle.to_owned()));
    }
    for needle in ["managed", "self_hosted", "regulated", "sovereign"] {
        assert!(vocab.deployment_profiles.contains(&needle.to_owned()));
    }
}

#[test]
fn tiles_csv_enumerates_function_state_and_proof() {
    let csv = packet().render_tiles_csv();
    let header = csv.lines().next().unwrap();
    assert!(header.starts_with("function,service,corpus_id,scope_profile,measure,"));
    assert!(header.contains("waiver_standing"));
    assert!(header.contains("proof_ref"));
    for function in FitnessFunction::ALL {
        assert!(csv.contains(&format!(
            "{},{}",
            function.as_str(),
            function.service().as_str()
        )));
    }
    assert!(csv.contains("artifacts/release-proof/m5-assurance-route-governance/"));
}

#[test]
fn overview_markdown_names_every_section() {
    let md = seeded_m5_governance_dashboard_waiver_active_narrowed().render_overview_markdown();
    assert!(md.contains("# M5 Governance Dashboard"));
    assert!(md.contains("Deployment-profile overviews"));
    assert!(md.contains("Fitness tiles"));
    assert!(md.contains("Waiver-expiry queue"));
    assert!(md.contains("Service ownership"));
    assert!(md.contains("Decision rights"));
    assert!(md.contains("evidence_freshness_slo"));
}

#[test]
fn packet_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5GovernanceDashboard = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());
}

#[test]
fn tampered_tile_state_is_rejected() {
    let mut packet = seeded_m5_governance_dashboard_evidence_stale_narrowed();
    let idx = packet
        .fitness_tiles
        .iter()
        .position(|t| t.state == FitnessState::EvidenceStale)
        .expect("a stale tile exists");
    packet.fitness_tiles[idx].state = FitnessState::Passing;
    packet.fitness_tiles[idx].gate = DescriptorGate::Governed;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5GovernanceDashboardViolation::TileStateDrift),
        "{violations:?}"
    );
}

#[test]
fn tampered_freshness_is_rejected() {
    let mut packet = packet();
    packet.fitness_tiles[0].evidence_freshness = FreshnessState::Stale;
    let violations = packet.validate();
    assert!(
        violations.contains(&M5GovernanceDashboardViolation::TileStateDrift)
            || violations.contains(&M5GovernanceDashboardViolation::SummaryDrift)
            || violations.contains(&M5GovernanceDashboardViolation::NightlyRowDrift),
        "{violations:?}"
    );
}

#[test]
fn dropping_a_function_is_rejected() {
    let mut packet = packet();
    packet
        .fitness_tiles
        .retain(|t| t.function != FitnessFunction::SchemaExampleParity);
    let violations = packet.validate();
    assert!(violations.contains(&M5GovernanceDashboardViolation::FunctionNotTiled));
}

#[test]
fn waiver_on_a_passing_function_is_rejected() {
    // A waiver may not be attached to a function that is already a clean pass.
    let mut packet = packet();
    packet.fitness_tiles[0].waiver_standing = Some(WaiverStanding::Active);
    let violations = packet.validate();
    assert!(
        violations.contains(&M5GovernanceDashboardViolation::WaiverOnPassingFunction)
            || violations.contains(&M5GovernanceDashboardViolation::TileStateDrift),
        "{violations:?}"
    );
}

#[test]
fn export_carries_no_raw_material() {
    for packet in [
        packet(),
        seeded_m5_governance_dashboard_waiver_active_narrowed(),
        seeded_m5_governance_dashboard_waiver_expired_blocked(),
        seeded_m5_governance_dashboard_missing_evidence_blocked(),
    ] {
        assert!(packet.conformance.export_carries_no_raw_material);
        assert!(!packet
            .export_safe_json()
            .to_ascii_lowercase()
            .contains("bearer_token"));
    }
}
