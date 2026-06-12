use super::*;

use aureline_provider::{
    HandoffProviderAcceptanceClass, PublishReviewActionClass, WorkItemMutationMode,
};

const PACKET_ID: &str = "work-item-mutation-review:stable:0001";

fn packet() -> WorkItemMutationReviewPacket {
    canonical_work_item_mutation_review_packet()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_detail_headers_fail() {
    let mut packet = packet();
    packet.detail_headers.clear();
    assert!(
        packet
            .validate()
            .contains(&WorkItemMutationReviewViolation::DetailHeadersMissing)
    );
}

#[test]
fn transition_without_side_effects_fails() {
    let mut packet = packet();
    packet.transition_reviews[0].side_effect_summaries.clear();
    assert!(
        packet
            .validate()
            .contains(&WorkItemMutationReviewViolation::TransitionReviewIncomplete)
    );
}

#[test]
fn publish_review_must_be_comment_action() {
    let mut packet = packet();
    packet.comment_publish_reviews[0].action_class = PublishReviewActionClass::LinkBranchOrReview;
    assert!(packet.validate().contains(
        &WorkItemMutationReviewViolation::CommentPublishReviewNotCommentAction
    ));
}

#[test]
fn offline_handoff_cannot_claim_provider_acceptance() {
    let mut packet = packet();
    packet.offline_handoff_packets[0].provider_acceptance_class =
        HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained;
    assert!(packet.validate().contains(
        &WorkItemMutationReviewViolation::OfflineHandoffClaimsProviderAcceptance
    ));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_passive_inspection_external_publish = false;
    assert!(
        packet
            .validate()
            .contains(&WorkItemMutationReviewViolation::TrustReviewIncomplete)
    );
}

#[test]
fn markdown_summary_mentions_core_sections() {
    let summary = packet().render_markdown_summary();
    for needle in [
        "Detail headers",
        "Transition sheets",
        "Comment publish review",
        "Offline handoff packets",
        "Conflict Or Reconcile",
    ] {
        assert!(summary.contains(needle), "summary missing {needle}");
    }
}

#[test]
fn canonical_packet_covers_tri_mode_and_conflict_review() {
    let packet = packet();
    let modes = packet
        .transition_reviews
        .iter()
        .map(|row| row.publish_mode_class)
        .collect::<std::collections::BTreeSet<_>>();

    assert!(modes.contains(&WorkItemMutationMode::LocalDraft));
    assert!(modes.contains(&WorkItemMutationMode::PublishNow));
    assert!(modes.contains(&WorkItemMutationMode::OpenInProvider));
    assert!(
        !packet.conflict_reconcile_rows.is_empty(),
        "canonical packet must surface at least one compare/reconcile row"
    );
}

#[test]
fn checked_support_export_validates() {
    let packet = current_work_item_mutation_review_export()
        .expect("checked work-item mutation review export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/provider_outage_preserves_handoff.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/policy_blocked_keeps_local_draft.json"
        )),
    ] {
        let packet: WorkItemMutationReviewPacket =
            serde_json::from_str(raw).expect("fixture parses");
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
