//! Fixture replay for stable dashboard, queue, and follow-up bundle truth.

use aureline_support::stabilize_dashboard_queue_and_followup_bundle_truth::{
    canonical_dashboard_queue_followup_truth_packet, DashboardQueueFollowupTruthPacket,
    EffectiveCardState, FollowupSupportExportPacket, ProviderMutationCommand,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/stabilize-dashboard-queue-and-followup-bundle-truth",
);

fn load_packet_fixture() -> DashboardQueueFollowupTruthPacket {
    let path = format!("{FIXTURE_DIR}/canonical_packet.json");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

fn load_export_fixture() -> FollowupSupportExportPacket {
    let path = format!("{FIXTURE_DIR}/support_export_projection.json");
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn canonical_packet_fixture_matches_rust_projection() {
    let fixture = load_packet_fixture();
    let expected = canonical_dashboard_queue_followup_truth_packet();
    assert_eq!(fixture, expected);
    fixture.validate().expect("fixture packet validates");
}

#[test]
fn stale_green_dashboard_cards_have_visible_downgrades_and_evidence_paths() {
    let packet = load_packet_fixture();
    for card in &packet.dashboard_cards {
        if card.declared_green && card.source_freshness.downgrades_green() {
            assert!(
                card.visibly_downgraded,
                "{} did not downgrade",
                card.card_id
            );
            assert_ne!(card.effective_state, EffectiveCardState::Healthy);
            assert!(
                !card.downgrade_reason_tokens.is_empty(),
                "{} lacks downgrade reason tokens",
                card.card_id,
            );
            assert!(
                card.open_evidence_ref.starts_with("aureline://"),
                "{} lacks canonical evidence ref",
                card.card_id,
            );
        }
    }
}

#[test]
fn queue_fixture_explains_sort_grouping_narrowing_and_blockers() {
    let packet = load_packet_fixture();
    assert!(packet.queue_order.is_explainable());
    assert!(packet.queue_order.discloses_provider_and_policy_blockers());
    assert_eq!(packet.queue_order.hidden_scope.len(), 2);
    assert_eq!(packet.queue_order.filter_state.hidden_count, 7);
}

#[test]
fn followup_fixture_keeps_checklist_completion_non_mutating() {
    let packet = load_packet_fixture();
    packet
        .followup_bundle
        .validate()
        .expect("follow-up bundle validates");
    assert!(packet
        .followup_bundle
        .checklist_items
        .iter()
        .all(|item| !item.local_completion_mutates_provider));
}

#[test]
fn support_export_fixture_preserves_bundle_meaning() {
    let packet = load_packet_fixture();
    let export = load_export_fixture();
    assert_eq!(export, packet.support_export);
    assert!(export.preserves_bundle_meaning(&packet.followup_bundle));
    assert!(export
        .provider_mutation_commands
        .iter()
        .all(ProviderMutationCommand::is_reviewable));
}
