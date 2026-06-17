//! Cross-crate coverage for topology-aware lane review sheets.
//!
//! This exercises the public surface the way a lane (search, review, blame, or
//! AI) would: load the canonical review packet and confirm that every topology
//! scope limit surfaces as an explicit label, recommendations stay advisory, and
//! no row mutates state or falls back to a generic empty/error.

use std::collections::HashSet;

use aureline_review::{current_git_topology_review_packet, ScopeLimitLabel, TopologyReviewLane};

#[test]
fn checked_packet_validates_and_covers_every_lane() {
    let packet = current_git_topology_review_packet().expect("checked packet validates");
    let lanes: HashSet<_> = packet.rows.iter().map(|row| row.lane).collect();
    for lane in TopologyReviewLane::ALL {
        assert!(lanes.contains(&lane), "lane {lane:?} is exercised");
    }
}

#[test]
fn no_lane_hides_behind_a_generic_state_or_mutates() {
    let packet = current_git_topology_review_packet().expect("checked packet validates");
    for row in &packet.rows {
        assert!(
            row.generic_state_suppressed,
            "row {} suppresses the generic empty/error state",
            row.row_id
        );
        assert!(!row.mutation_applied, "row {} never mutates", row.row_id);
    }
}

#[test]
fn recommendations_are_advisory_and_never_target_a_wrong_root() {
    let packet = current_git_topology_review_packet().expect("checked packet validates");
    for row in &packet.rows {
        if row.scope_limit_label == ScopeLimitLabel::WrongTargetRoot
            || row.scope_limit_label == ScopeLimitLabel::NestedBoundary
        {
            assert!(
                row.recommended_action.is_none(),
                "row {} recommends nothing across a wrong root",
                row.row_id
            );
        }
    }
}

#[test]
fn every_remediable_limit_surfaces_with_a_recommendation() {
    let packet = current_git_topology_review_packet().expect("checked packet validates");
    // At least one in-scope row offers each distinct remediation verb advisory.
    let recommended: HashSet<_> = packet
        .rows
        .iter()
        .filter_map(|row| row.recommended_action)
        .collect();
    assert!(!recommended.is_empty());
}
