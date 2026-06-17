use super::*;

#[test]
fn seeded_canonical_packet_validates() {
    let packet = seeded_ranking_explainability_packet();
    assert_eq!(
        packet.record_kind,
        RANKING_EXPLAINABILITY_PACKET_RECORD_KIND
    );
    assert_eq!(packet.packet_id, RANKING_EXPLAINABILITY_PACKET_ID);
    let findings = packet.validate();
    assert!(findings.is_empty(), "unexpected findings: {findings:?}");
    assert!(packet.is_export_safe());
}

#[test]
fn covers_all_five_surfaces_once() {
    let packet = seeded_ranking_explainability_packet();
    assert_eq!(packet.surfaces.len(), ExplainSurfaceClass::ALL.len());
    for surface in ExplainSurfaceClass::ALL {
        assert!(packet.surface_for(surface).is_some(), "missing {surface:?}");
    }
}

#[test]
fn distinguishes_all_six_fact_labels_and_states() {
    // Acceptance: exact, context-promoted, semantic, partial-index,
    // withheld-latency, and policy-hidden stay distinguishable across rows.
    let packet = seeded_ranking_explainability_packet();
    for label in FactLabelClass::ALL {
        assert!(
            packet.covered_fact_label_tokens().contains(&label.as_str()),
            "missing fact label {}",
            label.as_str()
        );
    }
    for state in ExplainStateClass::ALL {
        assert!(
            packet.covered_state_tokens().contains(&state.as_str()),
            "missing explainer state {}",
            state.as_str()
        );
    }
}

#[test]
fn explain_sheets_reuse_the_canonical_ranking_reason() {
    let packet = seeded_ranking_explainability_packet();
    for surface in &packet.surfaces {
        for sheet in &surface.explain_sheets {
            // The embedded explanation is the canonical RankingReason object,
            // not a reminted private shape.
            assert!(
                !sheet.ranking_reason.promoted_signals.is_empty()
                    || !sheet.ranking_reason.suppressed_signals.is_empty(),
                "sheet {} dropped its ranking signals",
                sheet.explain_sheet_id
            );
            assert!(sheet.raw_score_weights_excluded);
            assert!(!sheet.reason_lines.is_empty());
        }
    }
}

#[test]
fn withheld_and_policy_hidden_candidates_are_not_silent() {
    // Acceptance: policy-hidden and withheld candidates are visible states.
    let packet = seeded_ranking_explainability_packet();
    let graph = packet
        .surface_for(ExplainSurfaceClass::GraphBackedResults)
        .expect("graph surface");
    assert!(graph.scope_counters.hidden_by_policy_rows > 0);
    assert!(graph
        .omitted_candidates
        .iter()
        .any(|row| row.explain_state == ExplainStateClass::PolicyHidden));

    let palette = packet
        .surface_for(ExplainSurfaceClass::Palette)
        .expect("palette surface");
    assert!(palette.scope_counters.omitted_by_latency_budget_rows > 0);
    assert!(palette
        .omitted_candidates
        .iter()
        .any(|row| row.explain_state == ExplainStateClass::WithheldLatency));
}

#[test]
fn omitted_rows_carry_no_literal_query_text() {
    let packet = seeded_ranking_explainability_packet();
    for surface in &packet.surfaces {
        for omitted in &surface.omitted_candidates {
            assert!(omitted.literal_query_text_excluded);
            assert!(omitted.omitted_count >= 1);
            assert!(!omitted.omission_reason.trim().is_empty());
        }
    }
}

#[test]
fn all_three_consumers_reuse_one_explanation_object() {
    // Acceptance: the same explanation object is reused by UI, support export,
    // and replay/debug tooling.
    let packet = seeded_ranking_explainability_packet();
    for required in ExplainabilityConsumerClass::ALL {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.consumer == required)
            .unwrap_or_else(|| panic!("missing consumer {}", required.as_str()));
        assert_eq!(projection.ingested_packet_id, packet.packet_id);
        assert!(projection.preserves_explain_sheets);
        assert!(projection.preserves_omitted_candidates);
        assert!(projection.preserves_counts_and_hashes);
        assert!(!projection.includes_literal_query_text);
    }
}

#[test]
fn partial_index_variant_narrows_but_preserves_identity_and_vocabulary() {
    let canonical = seeded_ranking_explainability_packet();
    let degraded = seeded_partial_index_stale_ranking_explainability_packet();
    assert!(degraded.validate().is_empty());
    assert!(degraded.is_export_safe());

    // The full state and fact-label vocabulary is preserved across the variant.
    assert_eq!(
        canonical.covered_state_tokens(),
        degraded.covered_state_tokens()
    );
    assert_eq!(
        canonical.covered_fact_label_tokens(),
        degraded.covered_fact_label_tokens()
    );

    // Under a partial index the palette withholds strictly more candidates.
    let canonical_palette = canonical.surface_for(ExplainSurfaceClass::Palette).unwrap();
    let degraded_palette = degraded.surface_for(ExplainSurfaceClass::Palette).unwrap();
    assert!(
        degraded_palette
            .scope_counters
            .omitted_by_latency_budget_rows
            > canonical_palette
                .scope_counters
                .omitted_by_latency_budget_rows
    );
    // Saved-query reopen reads local material and is unchanged.
    assert_eq!(
        canonical.surface_for(ExplainSurfaceClass::SavedQueryReopen),
        degraded.surface_for(ExplainSurfaceClass::SavedQueryReopen)
    );
}

#[test]
fn checked_in_packet_matches_seeded_canonical() {
    let checked =
        current_ranking_explainability_packet().expect("checked-in packet parses and validates");
    assert_eq!(checked, seeded_ranking_explainability_packet());
}

#[test]
fn support_export_preserves_the_packet_safely() {
    let packet = seeded_ranking_explainability_packet();
    let export = packet.support_export("explainability-export-1", "2026-06-17T00:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.explainability_packet, packet);
}

#[test]
fn detects_visible_sheet_headlining_a_withheld_state() {
    let mut packet = seeded_ranking_explainability_packet();
    packet.surfaces[0].explain_sheets[0].explain_state = ExplainStateClass::WithheldLatency;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("may not headline a withheld/policy-hidden state")));
}

#[test]
fn detects_silent_policy_hidden_omission() {
    let mut packet = seeded_ranking_explainability_packet();
    let graph = packet
        .surfaces
        .iter_mut()
        .find(|surface| surface.surface == ExplainSurfaceClass::GraphBackedResults)
        .unwrap();
    // Counts still report a policy-hidden row but the omission row is dropped.
    graph.omitted_candidates.clear();
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("no policy-hidden omission row explains them")));
}

#[test]
fn detects_literal_query_text_without_elevated_consent() {
    let mut packet = seeded_ranking_explainability_packet();
    packet.literal_query_text_included = true;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("literal query text may not be included without elevated consent")));
}

#[test]
fn elevated_consent_admits_literal_query_text() {
    // The gate opens only under elevated consent; metadata-only is the default.
    let mut packet = seeded_ranking_explainability_packet();
    packet.export_consent = ExportConsentClass::QueryTextElevated;
    packet.literal_query_text_included = true;
    let findings = packet.validate();
    assert!(
        findings.is_empty(),
        "elevated consent should validate: {findings:?}"
    );
    // ...but it is no longer metadata-safe for unconditional export.
    assert!(!packet.is_export_safe());
}

#[test]
fn detects_raw_score_weights_present() {
    let mut packet = seeded_ranking_explainability_packet();
    packet.surfaces[0].explain_sheets[0].raw_score_weights_excluded = false;
    let findings = packet.validate();
    assert!(findings.iter().any(|finding| finding
        .message
        .contains("exclude raw numeric score weights")));
}

#[test]
fn detects_result_identity_collapsed_into_label() {
    let mut packet = seeded_ranking_explainability_packet();
    let sheet = &mut packet.surfaces[0].explain_sheets[0];
    sheet.display_title = sheet.result_id.clone();
    let findings = packet.validate();
    assert!(findings
        .iter()
        .any(|finding| finding.message.contains("collapse into the display label")));
}
