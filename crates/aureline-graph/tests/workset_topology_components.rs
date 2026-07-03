//! Integration test: the embedded M05-799 workset/topology component packet
//! parses, validates, and preserves scope + freshness/confidence/provenance
//! truth across its first consumers.

use aureline_graph::{
    current_m5_workset_topology_component_packet, ComponentConsumerSurface, Confidence,
    FreshnessState, ProvenanceClass, WorksetScope, WorksetTopologyRelationFidelity,
};

#[test]
fn embedded_workset_topology_component_packet_parses() {
    let packet =
        current_m5_workset_topology_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.schema_version, 1);
    assert!(!packet.workset_switcher_rows.is_empty());
    assert!(!packet.topology_node_cards.is_empty());
    assert!(!packet.relationship_chips.is_empty());
    assert!(!packet.consumer_projection_rows.is_empty());
}

#[test]
fn embedded_workset_topology_component_packet_has_no_violations() {
    let packet =
        current_m5_workset_topology_component_packet().expect("embedded packet must parse");
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn embedded_workset_topology_summary_matches_computed() {
    let packet =
        current_m5_workset_topology_component_packet().expect("embedded packet must parse");
    assert_eq!(packet.summary, packet.computed_summary());
    assert!(packet.summary.search_consumer_present);
    assert!(packet.summary.topology_consumer_present);
    assert!(packet.summary.scope_limited_and_full_both_present);
    assert!(packet.summary.no_row_permits_silent_widening);
    assert!(
        packet
            .summary
            .all_nodes_preserve_freshness_confidence_provenance
    );
    assert!(packet.summary.all_components_have_copy_export);
}

#[test]
fn workset_rows_state_scope_limitation_and_never_widen_silently() {
    let packet =
        current_m5_workset_topology_component_packet().expect("embedded packet must parse");

    // AC1: consumers can state whether a result is limited by the active workset;
    // both a scope-limited slice and a full workspace are present and distinct.
    assert!(packet
        .workset_switcher_rows
        .iter()
        .any(|r| r.is_scope_limited()));
    assert!(packet
        .workset_switcher_rows
        .iter()
        .any(|r| r.workset_scope == WorksetScope::FullWorkspace));

    for row in &packet.workset_switcher_rows {
        // AC2: no row ever widens implicitly.
        assert!(row.no_silent_widening);
        assert!(!row.permits_silent_widening());
        // Repo-lens count and scope truth survive the export.
        assert!(row.included_root_count() >= 1);
        assert!(row
            .copy_export
            .exports_all(&["workset_scope", "no_silent_widening"]));
    }
}

#[test]
fn topology_nodes_and_chips_preserve_degraded_language() {
    let packet =
        current_m5_workset_topology_component_packet().expect("embedded packet must parse");

    // AC3: fresh AND degraded (stale/partial) node truth are both represented and
    // survive the export projection.
    assert!(packet
        .topology_node_cards
        .iter()
        .any(|c| !c.freshness_state.is_degraded()));
    let degraded = packet
        .topology_node_cards
        .iter()
        .find(|c| c.freshness_state.is_degraded())
        .expect("a degraded topology node card must be present");
    assert_eq!(degraded.freshness_state, FreshnessState::Stale);
    assert_eq!(degraded.confidence, Confidence::Low);
    assert_eq!(degraded.provenance_class, ProvenanceClass::Inferred);

    for card in &packet.topology_node_cards {
        assert!(card.preserves_truth_in_export());
        assert!(card
            .consumer_surfaces
            .contains(&ComponentConsumerSurface::TopologyMap));
    }

    // Relationship chips keep direction/confidence/provenance and disclose a note
    // whenever the relation is partial/stale/blocked.
    assert!(packet
        .relationship_chips
        .iter()
        .any(|c| c.relation_fidelity == WorksetTopologyRelationFidelity::Blocked));
    for chip in &packet.relationship_chips {
        assert!(!chip.missing_required_note());
        if chip.relation_fidelity.requires_partiality_note() {
            assert!(!chip.partial_or_blocked_note_ref.is_empty());
        }
    }
}
