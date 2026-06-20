//! Unit tests for the reactive-command-parity packet, drills, and fixtures.

use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_reactive_command_parity_packet();
    validate_reactive_command_parity_packet(&packet)
        .expect("seeded reactive-command-parity packet must validate");
}

#[test]
fn seeded_fixtures_validate() {
    let packet = seeded_reactive_command_parity_packet();
    for fixture in seeded_reactive_command_parity_fixtures() {
        validate_reactive_command_parity_fixture(&packet, &fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
    }
}

#[test]
fn no_flow_claims_success_before_publish() {
    let packet = seeded_reactive_command_parity_packet();
    for row in &packet.flows {
        assert!(
            !row.claims_success_before_publish,
            "flow {} must not claim success before publication",
            row.flow_id
        );
        assert!(
            row.publishes_after_command_commit,
            "flow {} must publish only after the command commits",
            row.flow_id
        );
        assert!(
            row.publishes_after_journal_commit,
            "flow {} must publish only after the journal commits",
            row.flow_id
        );
        assert!(
            row.publishes_via_reactive_graph,
            "flow {} must publish through the reactive graph",
            row.flow_id
        );
        assert!(
            !row.state_before_publish.claims_current_truth(),
            "flow {} must not show published truth before publishing",
            row.flow_id
        );
    }
}

#[test]
fn every_flow_preserves_named_lineage() {
    let packet = seeded_reactive_command_parity_packet();
    for row in &packet.flows {
        for required in [
            LineageField::Actor,
            LineageField::Scope,
            LineageField::Command,
            LineageField::Checkpoint,
        ] {
            assert!(
                row.preserved_lineage.contains(&required),
                "flow {} must preserve {} lineage",
                row.flow_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn named_mutating_surfaces_are_all_covered_and_drilled() {
    let packet = seeded_reactive_command_parity_packet();
    let flow_surfaces: std::collections::BTreeSet<_> = packet
        .flows
        .iter()
        .map(|row| row.mutating_surface)
        .collect();
    let drilled: std::collections::BTreeSet<_> = packet
        .drills
        .iter()
        .map(|drill| drill.mutating_surface)
        .collect();
    for required in [
        MutatingSurface::AiApply,
        MutatingSurface::ReviewAction,
        MutatingSurface::ScaffoldUpdate,
        MutatingSurface::ProviderMutation,
        MutatingSurface::NotebookResultMutation,
        MutatingSurface::SupportRepair,
    ] {
        assert!(
            flow_surfaces.contains(&required),
            "packet must cover surface {}",
            required.as_str()
        );
        assert!(
            drilled.contains(&required),
            "packet must drill surface {}",
            required.as_str()
        );
    }
}

#[test]
fn no_drill_step_claims_truth_before_publish() {
    let packet = seeded_reactive_command_parity_packet();
    for drill in &packet.drills {
        assert!(drill.asserts_no_optimistic_truth_before_publish);
        assert!(drill.asserts_lineage_correlatable);
        for (index, step) in drill.steps.iter().enumerate() {
            if step.state_visibility.claims_current_truth() {
                assert!(
                    step.publication_stage.is_published(),
                    "drill {} step {} claims truth before publish",
                    drill.drill_id,
                    index
                );
            }
        }
    }
}

#[test]
fn divergence_drill_resolves_to_degraded_or_waiting() {
    let packet = seeded_reactive_command_parity_packet();
    let divergence_drills: Vec<_> = packet
        .drills
        .iter()
        .filter(|drill| {
            drill
                .steps
                .iter()
                .any(|step| step.phase == DrillPhase::Diverge)
        })
        .collect();
    assert!(
        !divergence_drills.is_empty(),
        "packet must drill at least one divergence"
    );
    for drill in divergence_drills {
        assert!(
            matches!(
                drill.expected_final_state_visibility,
                StateVisibility::DegradedState | StateVisibility::WaitingState
            ),
            "divergence drill {} must resolve to a degraded or waiting state, not truth",
            drill.drill_id
        );
        assert_eq!(
            drill.expected_final_publication_stage,
            PublicationStage::Diverged,
            "divergence drill {} must end at the diverged stage",
            drill.drill_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_reactive_command_parity_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let parsed: ReactiveCommandParityPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn fixture_is_seeded_for_every_flow() {
    let packet = seeded_reactive_command_parity_packet();
    let fixtures = seeded_reactive_command_parity_fixtures();
    assert_eq!(
        fixtures.len(),
        packet.flows.len(),
        "one fixture per parity flow"
    );
}

#[test]
fn rejecting_a_pre_publish_truth_claim_fails_validation() {
    let mut packet = seeded_reactive_command_parity_packet();
    // Forge an optimistic cache win: claim truth while still pending.
    packet.flows[0].claims_success_before_publish = true;
    packet.flows[0].state_before_publish = StateVisibility::PublishedTruth;
    let report = validate_reactive_command_parity_packet(&packet)
        .expect_err("a pre-publish truth claim must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "flow.claims_success_before_publish"));
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "flow.state_before_publish"));
}
