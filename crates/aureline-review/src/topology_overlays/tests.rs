//! Unit coverage for topology-aware mutation-review overlays.

use aureline_git::{
    current_git_topology_first_consumers_map, RepoIdentityKind, TopologyOperationScope,
    TopologyRootDescriptor,
};

use super::{
    MultiRootMutationPreview, ReviewTopologyOverlayPacket, ReviewTopologyOverlayValidationError,
    REVIEW_TOPOLOGY_OVERLAY_PACKET_RECORD_KIND,
};

/// Selects descriptors by root id from the canonical map.
fn roots_named(ids: &[&str]) -> Vec<TopologyRootDescriptor> {
    let map = current_git_topology_first_consumers_map().expect("canonical map validates");
    ids.iter()
        .map(|id| {
            map.roots
                .iter()
                .find(|root| root.root_id == *id)
                .unwrap_or_else(|| panic!("root {id} present"))
                .clone()
        })
        .collect()
}

fn multi_root_packet() -> ReviewTopologyOverlayPacket {
    // A parent root plus its submodule child plus a nested independent repo: the
    // classic ambient-bulk-mutation hazard.
    let roots = roots_named(&["main", "submodule", "nested"]);
    ReviewTopologyOverlayPacket::from_descriptors(
        "review-topology-overlay:multi:0001",
        "2026-06-17T00:00:00Z",
        "review-topology-overlay-export:multi:0001",
        "main",
        roots,
    )
}

#[test]
fn multi_root_packet_validates_and_guards_bulk_mutation() {
    let packet = multi_root_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "packet validates: {violations:?}");
    assert_eq!(
        packet.record_kind,
        REVIEW_TOPOLOGY_OVERLAY_PACKET_RECORD_KIND
    );

    let preview = &packet.preview;
    assert!(preview.spans_multiple_roots);
    assert!(preview.explicit_preview_required);
    assert!(preview.auto_apply_blocked);
    assert!(preview.opt_in_required);
    assert!(preview.crosses_parent_child_boundary);
    assert!(preview.crosses_nested_boundary);
    assert_eq!(
        preview.required_scope,
        TopologyOperationScope::ExplicitMultiRootPreviewRequired
    );
}

#[test]
fn parent_child_identity_stays_visible_and_only_active_root_mutates() {
    let packet = multi_root_packet();
    for row in &packet.rows {
        if row.root_ref == "main" {
            assert!(row.is_active_root);
            assert_eq!(row.identity_kind, RepoIdentityKind::ParentWithChildren);
        }
        if row.root_ref == "submodule" {
            assert_eq!(row.identity_kind, RepoIdentityKind::SubmoduleChild);
            assert_eq!(row.parent_root_ref.as_deref(), Some("main"));
        }
        // No non-active root is ever mutable in the ambient action.
        if !row.is_active_root {
            assert!(
                !row.mutation_allowed,
                "non-active root {} is not mutated",
                row.root_ref
            );
        }
    }
}

#[test]
fn single_root_packet_needs_no_cross_root_preview() {
    let roots = roots_named(&["main"]);
    let packet = ReviewTopologyOverlayPacket::from_descriptors(
        "review-topology-overlay:single:0001",
        "2026-06-17T00:00:00Z",
        "review-topology-overlay-export:single:0001",
        "main",
        roots,
    );
    assert!(packet.validate().is_empty());
    assert!(!packet.preview.spans_multiple_roots);
    assert!(!packet.preview.auto_apply_blocked);
    assert!(!packet.preview.opt_in_required);
    assert_eq!(
        packet.preview.required_scope,
        TopologyOperationScope::ActiveRootOnly
    );
}

#[test]
fn round_trips_through_json() {
    let packet = multi_root_packet();
    let json = packet.export_safe_json();
    let parsed = ReviewTopologyOverlayPacket::parse_json(&json).expect("re-parses");
    assert_eq!(parsed, packet);
}

#[test]
fn disabling_the_guard_is_rejected() {
    let mut packet = multi_root_packet();
    // Try to let a cross-root set auto-apply.
    packet.preview.auto_apply_blocked = false;
    packet.preview.opt_in_required = false;
    let violations = packet.validate();
    assert!(
        violations.iter().any(|error| matches!(
            error,
            ReviewTopologyOverlayValidationError::PreviewMismatch
                | ReviewTopologyOverlayValidationError::AmbientBulkMutationNotGuarded
        )),
        "disabling the guard is rejected: {violations:?}"
    );
}

#[test]
fn active_root_must_be_in_the_set() {
    let roots = roots_named(&["main", "submodule"]);
    let mut packet = ReviewTopologyOverlayPacket::from_descriptors(
        "review-topology-overlay:bad-active:0001",
        "2026-06-17T00:00:00Z",
        "review-topology-overlay-export:bad-active:0001",
        "main",
        roots,
    );
    // Repoint the active root at a root not present, and recompute nothing.
    packet.active_root_ref = "ghost".to_owned();
    let violations = packet.validate();
    assert!(
        violations.iter().any(|error| matches!(
            error,
            ReviewTopologyOverlayValidationError::ActiveRootNotInSet { .. }
        )),
        "an active root outside the set is rejected: {violations:?}"
    );
}

#[test]
fn preview_helper_is_order_independent_for_touched_roots() {
    let forward = MultiRootMutationPreview::for_descriptors(
        &roots_named(&["main", "submodule", "nested"]),
        "main",
        "p",
    );
    let reordered = MultiRootMutationPreview::for_descriptors(
        &roots_named(&["nested", "main", "submodule"]),
        "main",
        "p",
    );
    assert_eq!(
        forward, reordered,
        "touched roots are sorted deterministically"
    );
}
