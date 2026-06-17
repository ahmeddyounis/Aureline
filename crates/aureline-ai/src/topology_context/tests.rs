//! Unit coverage for topology-aware AI context assembly.

use aureline_git::{
    current_git_topology_first_consumers_map, SurfaceResultTruth, TopologyConsumerSurface,
};

use super::{
    ai_remediation_for, AiContextSliceRow, AiTopologyContextPacket,
    AiTopologyContextValidationError, AI_TOPOLOGY_CONTEXT_PACKET_RECORD_KIND,
};

/// Builds a packet from the canonical map's AI-context bindings.
fn canonical_packet() -> AiTopologyContextPacket {
    let map = current_git_topology_first_consumers_map().expect("canonical map validates");
    let bindings: Vec<_> = map
        .surface_bindings
        .into_iter()
        .filter(|binding| binding.surface == TopologyConsumerSurface::AiContext)
        .collect();
    assert!(!bindings.is_empty(), "map carries ai-context bindings");
    AiTopologyContextPacket::from_ai_context_bindings(
        "ai-topology-context:test:0001",
        "2026-06-17T00:00:00Z",
        "ai-topology-context-export:test:0001",
        bindings,
    )
}

#[test]
fn built_packet_validates() {
    let packet = canonical_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "packet validates: {violations:?}");
    assert_eq!(packet.record_kind, AI_TOPOLOGY_CONTEXT_PACKET_RECORD_KIND);
    assert_eq!(packet.rows.len(), packet.bindings.len());
}

#[test]
fn only_complete_slices_are_admitted_as_truth() {
    let packet = canonical_packet();
    for row in &packet.rows {
        let complete = matches!(row.result_truth, SurfaceResultTruth::Complete);
        assert_eq!(
            row.content_is_authoritative, complete,
            "row {} is authoritative only when complete",
            row.row_id
        );
        if !complete {
            assert!(
                !row.admit_body_to_prompt,
                "row {} never admits a limited slice body",
                row.row_id
            );
        }
        assert_eq!(
            row.remediation_action,
            ai_remediation_for(row.result_truth),
            "row {} remediation matches the shared mapping",
            row.row_id
        );
    }
}

#[test]
fn at_least_one_limited_slice_is_surfaced() {
    let packet = canonical_packet();
    let limited = packet
        .rows
        .iter()
        .filter(|row| !matches!(row.result_truth, SurfaceResultTruth::Complete))
        .count();
    assert!(limited > 0, "the canonical map exercises limited slices");
}

#[test]
fn round_trips_through_json() {
    let packet = canonical_packet();
    let json = packet.export_safe_json();
    let parsed = AiTopologyContextPacket::parse_json(&json).expect("re-parses");
    assert_eq!(parsed, packet);
}

#[test]
fn tampered_admission_is_rejected() {
    let mut packet = canonical_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| !matches!(row.result_truth, SurfaceResultTruth::Complete))
        .expect("a limited row exists");
    row.admit_body_to_prompt = true;
    let violations = packet.validate();
    assert!(
        violations.iter().any(|error| matches!(
            error,
            AiTopologyContextValidationError::RowDoesNotMatchBinding { .. }
                | AiTopologyContextValidationError::LimitedSliceAdmittedAsTruth { .. }
        )),
        "tampered admission is rejected: {violations:?}"
    );
}

#[test]
fn cross_root_slice_keeps_the_boundary_visible() {
    // Project a non-active root onto AI context to model a cross-root slice, and
    // confirm the derived row keeps the boundary visible and refuses admission.
    let map = current_git_topology_first_consumers_map().expect("map validates");
    let other = map
        .roots
        .iter()
        .find(|root| root.root_id != "main")
        .expect("map has more than one root");
    let binding = other.project(TopologyConsumerSurface::AiContext, "main", "ai-cross-root");
    let row = AiContextSliceRow::for_binding(&binding, "ai-cross-root-row");
    assert!(row.crosses_repo_boundary, "the boundary stays visible");
    assert!(
        !row.admit_body_to_prompt,
        "a cross-root slice is never admitted"
    );
    assert!(!row.content_is_authoritative);
}
