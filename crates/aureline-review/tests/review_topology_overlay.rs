//! Cross-crate coverage for topology-aware mutation-review overlays.
//!
//! Exercises the public surface the way a mutation-review pass would: load the
//! canonical multi-root overlay and the single-root fixture, and confirm that a
//! cross-root mutation set is guarded preview-first and opt-in while a single-root
//! set is not gated.

use aureline_git::TopologyOperationScope;
use aureline_review::{current_review_topology_overlay_packet, ReviewTopologyOverlayPacket};

const SINGLE_ROOT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/git/m5/topology-propagation/review_single_root_allowed.json"
));

#[test]
fn canonical_multi_root_overlay_guards_bulk_mutation() {
    let packet = current_review_topology_overlay_packet().expect("checked packet validates");
    let preview = &packet.preview;
    assert!(preview.spans_multiple_roots);
    assert!(preview.explicit_preview_required);
    assert!(preview.auto_apply_blocked);
    assert!(preview.opt_in_required);
    assert_eq!(
        preview.required_scope,
        TopologyOperationScope::ExplicitMultiRootPreviewRequired
    );
    // Parent/child identity stays visible; only the active root mutates.
    for row in &packet.rows {
        if !row.is_active_root {
            assert!(
                !row.mutation_allowed,
                "non-active root {} is never bulk-mutated",
                row.root_ref
            );
        }
    }
}

#[test]
fn single_root_fixture_needs_no_cross_root_preview() {
    let packet =
        ReviewTopologyOverlayPacket::parse_json(SINGLE_ROOT_FIXTURE).expect("fixture validates");
    assert!(!packet.preview.spans_multiple_roots);
    assert!(!packet.preview.auto_apply_blocked);
    assert!(!packet.preview.opt_in_required);
    assert_eq!(
        packet.preview.required_scope,
        TopologyOperationScope::ActiveRootOnly
    );
}
