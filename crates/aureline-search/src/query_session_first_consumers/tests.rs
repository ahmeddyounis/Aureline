use super::*;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_query_session_first_consumers_packet();
    assert_eq!(
        packet.record_kind,
        QUERY_SESSION_FIRST_CONSUMERS_PACKET_RECORD_KIND
    );
    assert_eq!(packet.packet_id, QUERY_SESSION_FIRST_CONSUMERS_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_all_six_surfaces_with_one_session_each() {
    let packet = seeded_query_session_first_consumers_packet();
    assert_eq!(
        packet.durable_sessions.len(),
        ConsumerSurfaceKind::ALL.len()
    );
    for surface in ConsumerSurfaceKind::ALL {
        let count = packet
            .durable_sessions
            .iter()
            .filter(|session| session.surface == surface)
            .count();
        assert_eq!(count, 1, "surface {} session count", surface.as_str());
    }
}

#[test]
fn sessions_are_hash_only_and_carry_no_raw_text() {
    let packet = seeded_query_session_first_consumers_packet();
    for session in &packet.durable_sessions {
        assert!(session.query_session.query_text.is_none());
        assert!(session.query_session.query_hash.is_some());
    }
}

#[test]
fn every_row_keeps_complete_source_stratum_lineage() {
    let packet = seeded_query_session_first_consumers_packet();
    for session in &packet.durable_sessions {
        for row in &session.result_rows {
            assert!(
                row.result_ref.dedupe_lineage_is_complete(),
                "row {} dropped a contributing stratum",
                row.result_ref.result_id
            );
        }
    }
    // The packet jointly exercises lexical, semantic, structural, graph, docs,
    // and recents strata so consumers can inspect every contributing source.
    let tokens = packet.contributing_stratum_tokens();
    for expected in [
        "lexical_filename",
        "lexical_content",
        "semantic_vector",
        "structural_symbol",
        "graph_entity",
        "docs_index",
        "recent_targets",
    ] {
        assert!(tokens.contains(&expected), "missing stratum {expected}");
    }
}

#[test]
fn result_identity_is_durable_not_a_label_or_index() {
    let packet = seeded_query_session_first_consumers_packet();
    for session in &packet.durable_sessions {
        for row in &session.result_rows {
            let id = &row.result_ref.result_id;
            assert!(id.contains(':'), "id {id} is not a durable URN");
            assert!(id.parse::<u64>().is_err(), "id {id} is a transient index");
            assert!(
                !id.eq_ignore_ascii_case(row.display_title.trim()),
                "id {id} collapsed into its display label"
            );
            // Identity survives every presentation-churn event.
            for event in PresentationChurnEvent::ALL {
                assert!(row.stable_across_churn.contains(&event));
            }
        }
    }
}

#[test]
fn cross_surface_reuse_shares_one_identity() {
    let packet = seeded_query_session_first_consumers_packet();
    assert!(!packet.cross_surface_reuse.is_empty());
    for reuse in &packet.cross_surface_reuse {
        assert!(reuse.surfaces.len() >= 2);
        for surface in &reuse.surfaces {
            let session = packet
                .durable_sessions
                .iter()
                .find(|session| session.surface == *surface)
                .expect("surface session exists");
            let row = session
                .result_rows
                .iter()
                .find(|row| row.result_ref.result_id == reuse.shared_result_id)
                .expect("shared id is present on each reuse surface");
            assert!(row
                .result_ref
                .canonical_object_refs
                .contains(&reuse.canonical_target_ref));
        }
    }
}

#[test]
fn all_four_first_consumers_reuse_real_ids() {
    let packet = seeded_query_session_first_consumers_packet();
    let session_ids = packet.session_ids();
    let result_ids = packet.result_ids();
    for required in SessionConsumerClass::ALL {
        let binding = packet
            .consumer_bindings
            .iter()
            .find(|binding| binding.consumer == required)
            .unwrap_or_else(|| panic!("missing consumer {}", required.as_str()));
        assert!(!binding.reconstructs_from_ui_text);
        assert!(!binding.invents_private_candidate_list);
        assert!(binding.supports_reopen && binding.supports_replay);
        assert!(binding.supports_export && binding.supports_explain);
        for id in &binding.reused_query_session_ids {
            assert!(session_ids.contains(id.as_str()));
        }
        for id in &binding.reused_result_ids {
            assert!(result_ids.contains(id.as_str()));
        }
    }
}

#[test]
fn partial_index_variant_preserves_identity_and_lineage() {
    let canonical = seeded_query_session_first_consumers_packet();
    let degraded = seeded_partial_index_stale_query_session_first_consumers_packet();
    assert!(degraded.validate().is_empty());
    // Identity is unchanged across the degraded variant; only freshness narrows.
    assert_eq!(canonical.result_ids(), degraded.result_ids());

    let degraded_full_text = degraded
        .durable_sessions
        .iter()
        .find(|session| session.surface == ConsumerSurfaceKind::FullTextSearch)
        .expect("full-text session");
    assert!(degraded_full_text
        .result_rows
        .iter()
        .all(|row| matches!(row.result_ref.freshness, FreshnessClass::PartialIndex)));

    // Recent navigation reads local history and stays live under a warming index.
    let degraded_recent = degraded
        .durable_sessions
        .iter()
        .find(|session| session.surface == ConsumerSurfaceKind::RecentNavigation)
        .expect("recent session");
    assert!(degraded_recent
        .result_rows
        .iter()
        .all(|row| matches!(row.result_ref.freshness, FreshnessClass::Live)));
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked = current_query_session_first_consumers_packet()
        .expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_query_session_first_consumers_packet());
}

#[test]
fn detects_identity_collapsed_into_label() {
    let mut packet = seeded_query_session_first_consumers_packet();
    let row = &mut packet.durable_sessions[0].result_rows[0];
    row.display_title = row.result_ref.result_id.clone();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("collapse into the display label")));
}

#[test]
fn detects_dropped_source_stratum() {
    let mut packet = seeded_query_session_first_consumers_packet();
    packet.durable_sessions[0].result_rows[0]
        .result_ref
        .dedupe_lineage
        .clear();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("contributing source stratum")));
}

#[test]
fn detects_consumer_that_reconstructs_from_ui_text() {
    let mut packet = seeded_query_session_first_consumers_packet();
    packet.consumer_bindings[0].reconstructs_from_ui_text = true;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("reconstruct state from rendered UI text")));
}
