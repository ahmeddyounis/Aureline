//! Fixture replay for the activity-timeline + attention-inbox packet.
//!
//! The fixtures live under
//! `fixtures/ux/m3/activity_timeline_and_inbox/` and are minted by the
//! `aureline_shell_activity_timeline` inspector. This test keeps the
//! checked-in JSON honest by parsing it through the shared Rust
//! types and comparing it to the live seeded packet.

use aureline_shell::activity_timeline::{
    seeded_activity_timeline_and_inbox_packet, validate_activity_timeline_and_inbox_packet,
    ActivityEventRow, ActivityTimelineAndInboxPacket, ActivityTimelineSnapshot, AttentionInboxItem,
    AttentionInboxSnapshot, ChronologyLane, NarrativeSummaryCard, TimelineGroup,
    ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION, ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/ux/m3/activity_timeline_and_inbox"
);

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn packet_round_trips_through_shared_types() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    assert_eq!(
        packet.shared_contract_ref,
        ACTIVITY_TIMELINE_AND_INBOX_SHARED_CONTRACT_REF
    );
    assert_eq!(
        packet.schema_version,
        ACTIVITY_TIMELINE_AND_INBOX_SCHEMA_VERSION
    );
    validate_activity_timeline_and_inbox_packet(&packet).expect("packet must validate");
}

#[test]
fn packet_matches_seeded_builder() {
    let on_disk: ActivityTimelineAndInboxPacket = load("packet.json");
    let live = seeded_activity_timeline_and_inbox_packet();
    assert_eq!(
        on_disk, live,
        "fixtures must match the seeded chronology + inbox packet; regenerate with the headless inspector"
    );
}

#[test]
fn snapshot_round_trips_and_segments_round_trip_independently() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    let snapshot: ActivityTimelineSnapshot = load("snapshot.json");
    assert_eq!(packet.snapshot, snapshot);
    let rows: Vec<ActivityEventRow> = load("event_rows.json");
    assert_eq!(rows, packet.snapshot.rows);
    let groups: Vec<TimelineGroup> = load("timeline_groups.json");
    assert_eq!(groups, packet.snapshot.groups);
    let summary_cards: Vec<NarrativeSummaryCard> = load("narrative_summary_cards.json");
    assert_eq!(summary_cards, packet.snapshot.summary_cards);
    let inbox: AttentionInboxSnapshot = load("attention_inbox.json");
    assert_eq!(inbox, packet.snapshot.inbox);
}

#[test]
fn every_required_chronology_lane_is_present() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    for required in [
        ChronologyLane::ActivityCenter,
        ChronologyLane::Approvals,
        ChronologyLane::PolicyChanges,
        ChronologyLane::ProviderSync,
        ChronologyLane::UpdateHistory,
        ChronologyLane::ReconnectFlow,
        ChronologyLane::Recovery,
    ] {
        assert!(
            packet.summary.lanes_present.contains(&required),
            "packet must cover chronology lane {:?}",
            required
        );
    }
}

#[test]
fn quiet_hours_held_inbox_items_preserve_durable_history() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    let held: Vec<&AttentionInboxItem> = packet
        .snapshot
        .inbox
        .items
        .iter()
        .filter(|item| item.suppression_note.transient_surface_held)
        .collect();
    assert!(
        !held.is_empty(),
        "packet must exercise at least one quiet-hours-held inbox item"
    );
    for item in held {
        assert!(item.suppression_note.durable_history_preserved);
    }
    assert!(packet.summary.quiet_hours_durable_history_preserved);
}

#[test]
fn every_inbox_item_exposes_open_snooze_acknowledge_resolve_verbs() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    for item in &packet.snapshot.inbox.items {
        assert!(
            item.exposes_triage_verb_set(),
            "inbox item {} missing open / snooze / acknowledge / resolve",
            item.inbox_item_id
        );
    }
    assert!(packet.summary.triage_verb_set_complete);
}

#[test]
fn consequential_or_safety_critical_rows_have_durable_non_truncating_detail_links() {
    let packet: ActivityTimelineAndInboxPacket = load("packet.json");
    for row in &packet.snapshot.rows {
        assert!(
            row.importance_rule_satisfied(),
            "row {} importance/detail-link rule violated",
            row.event_row_id
        );
    }
    assert!(packet.summary.importance_detail_link_rule_satisfied);
}
