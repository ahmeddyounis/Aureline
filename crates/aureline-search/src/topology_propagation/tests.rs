//! Unit coverage for topology-aware search-scope propagation.

use aureline_git::{
    current_git_topology_first_consumers_map, SurfaceResultTruth, TopologyActionKind,
    TopologyConsumerSurface,
};

use super::{
    search_remediation_for, SearchScopeRow, SearchTopologyScopePacket,
    SearchTopologyScopeValidationError, SEARCH_TOPOLOGY_SCOPE_PACKET_RECORD_KIND,
    SEARCH_TOPOLOGY_SCOPE_ROW_RECORD_KIND,
};

/// Builds a packet from the canonical map's search-scope bindings.
fn canonical_packet() -> SearchTopologyScopePacket {
    let map = current_git_topology_first_consumers_map().expect("canonical map validates");
    let bindings: Vec<_> = map
        .surface_bindings
        .into_iter()
        .filter(|binding| binding.surface == TopologyConsumerSurface::SearchScope)
        .collect();
    assert!(!bindings.is_empty(), "map carries search-scope bindings");
    SearchTopologyScopePacket::from_search_bindings(
        "search-topology-scope:test:0001",
        "2026-06-17T00:00:00Z",
        "search-topology-scope-export:test:0001",
        bindings,
    )
}

#[test]
fn built_packet_validates() {
    let packet = canonical_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "packet validates: {violations:?}");
    assert_eq!(packet.record_kind, SEARCH_TOPOLOGY_SCOPE_PACKET_RECORD_KIND);
    assert_eq!(packet.rows.len(), packet.bindings.len());
}

#[test]
fn rows_only_assert_absence_when_complete() {
    let packet = canonical_packet();
    for row in &packet.rows {
        let complete = matches!(row.result_truth, SurfaceResultTruth::Complete);
        assert_eq!(
            row.zero_results_means_absent, complete,
            "row {} asserts absence only when complete",
            row.row_id
        );
        assert_eq!(row.record_kind, SEARCH_TOPOLOGY_SCOPE_ROW_RECORD_KIND);
    }
}

#[test]
fn limited_roots_carry_the_matching_remediation_verb() {
    let packet = canonical_packet();
    // Each distinct topology limit surfaces with the reviewed remediation verb.
    let mut saw_widen = false;
    let mut saw_hydrate = false;
    let mut saw_initialize = false;
    for row in &packet.rows {
        assert_eq!(
            row.remediation_action,
            search_remediation_for(row.result_truth),
            "row {} remediation matches the shared mapping",
            row.row_id
        );
        match row.remediation_action {
            Some(TopologyActionKind::Widen) => saw_widen = true,
            Some(TopologyActionKind::Hydrate) => saw_hydrate = true,
            Some(TopologyActionKind::Initialize) => saw_initialize = true,
            _ => {}
        }
    }
    assert!(saw_widen, "a sparse slice offers a widen verb");
    assert!(
        saw_hydrate,
        "a pointer-only/unfetched root offers a hydrate verb"
    );
    assert!(
        saw_initialize,
        "an uninitialized submodule offers an initialize verb"
    );
}

#[test]
fn remediation_mapping_has_no_verb_for_terminal_truths() {
    assert_eq!(search_remediation_for(SurfaceResultTruth::Complete), None);
    assert_eq!(search_remediation_for(SurfaceResultTruth::NestedRoot), None);
    assert_eq!(
        search_remediation_for(SurfaceResultTruth::GeneratedOrExcluded),
        None
    );
    assert_eq!(
        search_remediation_for(SurfaceResultTruth::WrongTargetRoot),
        None
    );
    assert_eq!(
        search_remediation_for(SurfaceResultTruth::OutsideCurrentSlice),
        Some(TopologyActionKind::Widen)
    );
}

#[test]
fn round_trips_through_json() {
    let packet = canonical_packet();
    let json = packet.export_safe_json();
    let parsed = SearchTopologyScopePacket::parse_json(&json).expect("re-parses");
    assert_eq!(parsed, packet);
}

#[test]
fn tampered_absence_claim_is_rejected() {
    let mut packet = canonical_packet();
    // Force a limited row to claim genuine absence.
    let row = packet
        .rows
        .iter_mut()
        .find(|row| !matches!(row.result_truth, SurfaceResultTruth::Complete))
        .expect("at least one limited row");
    row.zero_results_means_absent = true;
    let violations = packet.validate();
    assert!(
        violations.iter().any(|error| matches!(
            error,
            SearchTopologyScopeValidationError::RowDoesNotMatchBinding { .. }
                | SearchTopologyScopeValidationError::SilentAbsenceOverLimit { .. }
        )),
        "tampered absence claim is rejected: {violations:?}"
    );
}

#[test]
fn non_search_binding_is_rejected() {
    let map = current_git_topology_first_consumers_map().expect("map validates");
    // Take a review-surface binding and try to smuggle it into the search packet.
    let review_binding = map
        .surface_bindings
        .into_iter()
        .find(|binding| binding.surface == TopologyConsumerSurface::Review)
        .expect("map carries review bindings");
    let row = SearchScopeRow::for_binding(&review_binding, "smuggled-row");
    let mut packet = canonical_packet();
    packet.bindings.push(review_binding);
    packet.rows.push(row);
    let violations = packet.validate();
    assert!(
        violations.iter().any(|error| matches!(
            error,
            SearchTopologyScopeValidationError::BindingWrongSurface { .. }
        )),
        "a non-search binding is rejected: {violations:?}"
    );
}
