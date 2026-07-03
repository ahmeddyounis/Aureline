//! Integration coverage for the M05-802 cross-surface component consumer packet.
//!
//! These tests prove that the checked-in adoption packet demonstrates the frozen
//! M5 profiler/topology component families are reusable primitives beyond their
//! original feature pages: every consumer points back to exactly one canonical
//! family, preserves capture/source badges + scope/citation + degraded-state
//! labels even when read-only or export-only, discloses any narrowing, and the
//! five claimed consumer classes plus help/support/release evidence surfaces are
//! all represented.

use aureline_support::m5_cross_surface_component_consumers::{
    current_cross_surface_consumer_packet, AuthorityMode, ComponentConsumerSurface, ConsumerGroup,
    CrossSurfaceConsumerRow, HandoffTarget, M5ComponentFamily, CROSS_SURFACE_CONSUMER_PACKET_JSON,
    CROSS_SURFACE_CONSUMER_RECORD_KIND,
};

fn packet() -> aureline_support::m5_cross_surface_component_consumers::CrossSurfaceConsumerPacket {
    current_cross_surface_consumer_packet().expect("packet parses")
}

#[test]
fn packet_parses_and_validates_clean() {
    let packet = packet();
    assert_eq!(packet.record_kind, CROSS_SURFACE_CONSUMER_RECORD_KIND);
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {violations:?}"
    );
}

#[test]
fn summary_self_check_matches() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn all_five_consumer_groups_are_adopted() {
    let packet = packet();
    for group in ConsumerGroup::ALL {
        assert!(
            packet.rows.iter().any(|r| r.consumer_group == group),
            "consumer group {group:?} is not adopted"
        );
    }
    assert_eq!(packet.summary.consumer_group_count, 5);
}

#[test]
fn every_row_points_to_one_canonical_family() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.points_to_canonical_family(),
            "row {} does not point back to a canonical family",
            row.row_id
        );
        // The declared schema is exactly the family's canonical schema.
        assert_eq!(
            row.canonical_family_schema_ref,
            row.component_family.canonical_schema_ref(),
            "row {} schema ref drifted from the canonical family",
            row.row_id
        );
    }
}

#[test]
fn all_ten_families_are_reused_across_surfaces() {
    let packet = packet();
    for family in M5ComponentFamily::ALL {
        assert!(
            packet.rows.iter().any(|r| r.component_family == family),
            "family {family:?} is never reused by any consumer"
        );
    }
}

#[test]
fn narrowed_consumers_preserve_labels_and_disclose() {
    let packet = packet();
    let narrowed: Vec<&CrossSurfaceConsumerRow> =
        packet.rows.iter().filter(|r| r.is_narrowed()).collect();
    assert!(
        !narrowed.is_empty(),
        "expected at least one narrowed (read-only/export-only) consumer"
    );
    for row in narrowed {
        assert!(
            row.preserves_labels(),
            "narrowed row {} dropped or renamed a controlled label",
            row.row_id
        );
        assert!(
            row.discloses_narrowing(),
            "narrowed row {} did not disclose its reduction",
            row.row_id
        );
        // The banner capability_state mirrors the authority narrowing.
        assert_eq!(
            row.reduced_capability_banner.capability_state,
            row.authority_mode.capability_state(),
            "narrowed row {} banner state drifted",
            row.row_id
        );
    }
}

#[test]
fn handoff_rows_carry_a_companion_or_browser_note() {
    let packet = packet();
    let handoff_rows: Vec<&CrossSurfaceConsumerRow> = packet
        .rows
        .iter()
        .filter(|r| r.handoff_target != HandoffTarget::None)
        .collect();
    assert!(
        !handoff_rows.is_empty(),
        "expected at least one companion/browser/handoff consumer"
    );
    for row in handoff_rows {
        assert!(
            !row.handoff_note_ref.trim().is_empty(),
            "handoff row {} is missing its handoff note",
            row.row_id
        );
    }
}

#[test]
fn ai_review_incident_support_point_at_canonical_evidence() {
    let packet = packet();
    // AC2: AI/review/incident/support consumers point back to one canonical
    // component family for their performance or topology evidence object.
    for group in [ConsumerGroup::AiReview, ConsumerGroup::IncidentSupport] {
        let rows: Vec<&CrossSurfaceConsumerRow> =
            packet.rows.iter().filter(|r| r.consumer_group == group).collect();
        assert!(!rows.is_empty(), "group {group:?} has no consumers");
        for row in rows {
            assert!(
                row.points_to_canonical_family(),
                "group {group:?} row {} does not cite a canonical family",
                row.row_id
            );
        }
    }
}

#[test]
fn help_support_release_reference_canonical_components() {
    let packet = packet();
    // AC3: help/support/release artifacts reference the reusable components
    // instead of cloning surface-local prose.
    for surface in [
        ComponentConsumerSurface::DocsHelp,
        ComponentConsumerSurface::SupportExport,
        ComponentConsumerSurface::ReleaseProof,
    ] {
        let row = packet
            .rows
            .iter()
            .find(|r| r.consumer_surface == surface)
            .unwrap_or_else(|| panic!("no consumer for {surface:?}"));
        assert!(
            row.references_canonical_not_local_prose,
            "{surface:?} clones surface-local prose instead of the canonical component"
        );
        assert!(row.points_to_canonical_family());
    }
    assert!(packet.summary.help_support_release_reference_present);
}

#[test]
fn a_performance_and_a_graph_family_both_reach_a_narrower_consumer() {
    // The point of the lane: a profiler family (performance) and a graph family
    // (topology/ownership/explainer) both survive on a narrower consumer with
    // labels intact — proving cross-surface reuse, not feature-local clones.
    let packet = packet();
    let perf_family_narrowed = packet.rows.iter().any(|r| {
        r.is_narrowed()
            && matches!(
                r.component_family,
                M5ComponentFamily::ProfileSessionCard
                    | M5ComponentFamily::CallTreeRow
                    | M5ComponentFamily::TraceTimeline
                    | M5ComponentFamily::HeapProfileCompareCard
            )
    });
    let graph_family_narrowed = packet.rows.iter().any(|r| {
        r.is_narrowed()
            && matches!(
                r.component_family,
                M5ComponentFamily::WorksetSwitcherRow
                    | M5ComponentFamily::TopologyNodeCard
                    | M5ComponentFamily::OwnershipCard
                    | M5ComponentFamily::ExplainerSectionCard
            )
    });
    assert!(perf_family_narrowed, "no profiler family reaches a narrower consumer");
    assert!(graph_family_narrowed, "no graph family reaches a narrower consumer");
}

#[test]
fn embedded_json_matches_expected_record_kind() {
    assert!(CROSS_SURFACE_CONSUMER_PACKET_JSON.contains(CROSS_SURFACE_CONSUMER_RECORD_KIND));
    // Detect a tampered/renamed authority label surviving into the packet.
    for mode in [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::CompareOnly,
        AuthorityMode::ExportOnly,
    ] {
        // capability_state() is a stable label; ensure it is a known token.
        assert!(!mode.capability_state().is_empty());
    }
}
