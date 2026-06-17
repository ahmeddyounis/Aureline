//! Cross-crate coverage for topology-aware AI context assembly.
//!
//! Exercises the public surface the way a context-assembly pass would: load the
//! canonical packet and the cross-root fixture, and confirm that only complete,
//! in-scope slices are admitted into the prompt and that a cross-root slice keeps
//! its boundary visible.

use aureline_ai::{current_ai_topology_context_packet, AiTopologyContextPacket};
use aureline_git::SurfaceResultTruth;

const CROSS_ROOT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/topology-propagation/ai_cross_root_boundary.json"
));

#[test]
fn canonical_packet_admits_only_complete_slices() {
    let packet = current_ai_topology_context_packet().expect("checked packet validates");
    let mut saw_limited = false;
    for row in &packet.rows {
        if !matches!(row.result_truth, SurfaceResultTruth::Complete) {
            saw_limited = true;
            assert!(
                !row.admit_body_to_prompt,
                "limited slice {} is never admitted as prompt material",
                row.row_id
            );
            assert!(
                !row.content_is_authoritative,
                "limited slice {} is never authoritative",
                row.row_id
            );
        }
    }
    assert!(saw_limited, "the canonical map exercises limited slices");
}

#[test]
fn cross_root_fixture_keeps_boundary_and_refuses_admission() {
    let packet =
        AiTopologyContextPacket::parse_json(CROSS_ROOT_FIXTURE).expect("fixture validates");
    let crossing = packet
        .rows
        .iter()
        .filter(|row| row.crosses_repo_boundary)
        .count();
    assert!(crossing > 0, "the fixture exercises cross-root slices");
    for row in &packet.rows {
        if row.crosses_repo_boundary {
            assert!(
                !row.admit_body_to_prompt,
                "cross-root slice {} is never admitted into the prompt",
                row.row_id
            );
            assert_ne!(row.authoritative_root_ref, "main");
        }
    }
}
