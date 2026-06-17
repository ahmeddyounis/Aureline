//! Cross-crate coverage for topology-aware search-scope propagation.
//!
//! Exercises the public surface the way a search lane would: load the canonical
//! packet and the cross-root fixture, and confirm that no topology-limited root is
//! ever reported as genuine absence and that the owning root stays visible.

use std::collections::HashSet;

use aureline_git::SurfaceResultTruth;
use aureline_search::{current_search_topology_scope_packet, SearchTopologyScopePacket};

const CROSS_ROOT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/topology-propagation/search_cross_root_wrong_target.json"
));

#[test]
fn canonical_packet_validates_and_distinguishes_limits() {
    let packet = current_search_topology_scope_packet().expect("checked packet validates");
    let truths: HashSet<_> = packet.rows.iter().map(|row| row.result_truth).collect();
    // Search distinguishes the omitted slice from genuine absence, and unfetched
    // from not-found.
    assert!(truths.contains(&SurfaceResultTruth::OutsideCurrentSlice));
    assert!(
        truths.contains(&SurfaceResultTruth::NotFetched)
            || truths.contains(&SurfaceResultTruth::PointerOnly)
    );
    for row in &packet.rows {
        assert_eq!(
            row.zero_results_means_absent,
            matches!(row.result_truth, SurfaceResultTruth::Complete),
            "row {} only asserts absence when complete",
            row.row_id
        );
    }
}

#[test]
fn cross_root_fixture_never_claims_absence_off_active_root() {
    let packet =
        SearchTopologyScopePacket::parse_json(CROSS_ROOT_FIXTURE).expect("fixture validates");
    let mut saw_off_root = false;
    for row in &packet.rows {
        if !matches!(row.result_truth, SurfaceResultTruth::Complete) {
            saw_off_root = true;
            assert!(
                !row.zero_results_means_absent,
                "off-active-root row {} never claims absence",
                row.row_id
            );
            // A wrong-root / nested row asks the user to retarget rather than
            // recommending a widen against the wrong root.
            assert!(
                row.remediation_action.is_none(),
                "off-active-root row {} recommends retargeting, not widening",
                row.row_id
            );
            // The owning root stays visible so the boundary is not flattened.
            assert_ne!(row.authoritative_root_ref, "main");
        }
    }
    assert!(
        saw_off_root,
        "the cross-root fixture exercises off-active roots"
    );
}
