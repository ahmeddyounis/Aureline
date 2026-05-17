//! Integration drill for the graph drift packet beta projection.
//!
//! The drill re-proves the on-disk presence of the schema, reviewer
//! doc, baseline report, fixture corpus, and manifest, then round-trips
//! every packet through serde, runs the evaluator over the checked-in
//! corpus, and pins the closed-vocabulary tokens exposed by the
//! report.

use std::path::{Path, PathBuf};

use aureline_graph::{
    current_graph_drift_corpus, DataLaneLineage, DriftConsumerSurface, DriftDowngradeLabel,
    DriftIndicator, DriftOpenGapClass, FreshnessClass, GraphDriftPacketEvaluator, GraphDriftReport,
    ReadinessState, ScopeClass, GRAPH_DRIFT_PACKET_CORPUS_DIR,
    GRAPH_DRIFT_PACKET_CORPUS_MANIFEST_REF, GRAPH_DRIFT_PACKET_DOC_REF,
    GRAPH_DRIFT_PACKET_REPORT_REF, GRAPH_DRIFT_PACKET_SCHEMA_REF, REQUIRED_DATA_LANE_LINEAGES,
    REQUIRED_DRIFT_CONSUMER_SURFACES,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

#[test]
fn schema_doc_report_and_manifest_exist_on_disk() {
    assert_exists(GRAPH_DRIFT_PACKET_SCHEMA_REF);
    assert_exists(GRAPH_DRIFT_PACKET_DOC_REF);
    assert_exists(GRAPH_DRIFT_PACKET_REPORT_REF);
    assert_exists(GRAPH_DRIFT_PACKET_CORPUS_MANIFEST_REF);
    assert_exists(GRAPH_DRIFT_PACKET_CORPUS_DIR);
}

#[test]
fn every_required_consumer_surface_appears_in_the_corpus() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for surface in REQUIRED_DRIFT_CONSUMER_SURFACES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.packet.consumer_surface == surface),
            "missing required consumer_surface = {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_required_data_lane_lineage_appears_in_the_corpus() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for lineage in REQUIRED_DATA_LANE_LINEAGES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.packet.data_lane_lineage == lineage),
            "missing required data_lane_lineage = {}",
            lineage.as_str()
        );
    }
}

#[test]
fn at_least_one_non_aligned_packet_exists() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    assert!(
        corpus
            .entries
            .iter()
            .any(|entry| !entry.packet.drift_indicator.is_aligned()),
        "corpus must seed at least one non-aligned drift packet"
    );
}

#[test]
fn checked_in_corpus_validates() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    GraphDriftPacketEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
}

#[test]
fn corpus_round_trips_through_serde() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let json = serde_json::to_string(&entry.packet).expect("packet serializes to JSON");
        let parsed: aureline_graph::GraphDriftPacket =
            serde_json::from_str(&json).expect("packet parses back from JSON");
        assert_eq!(parsed, entry.packet);
    }
}

#[test]
fn report_emits_closed_vocabulary_tokens() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    let report: GraphDriftReport = GraphDriftPacketEvaluator::new()
        .report("report:graph_drift", "2026-05-16T10:00:00Z", &corpus)
        .expect("report builds");
    assert!(report.is_export_safe());
    assert_eq!(report.matrix_rows.len(), corpus.entries.len());

    for row in &report.matrix_rows {
        let json = serde_json::to_string(&row).expect("matrix row serializes");
        let parsed: aureline_graph::DriftReportMatrixRow =
            serde_json::from_str(&json).expect("matrix row parses");
        assert_eq!(parsed, *row);
    }

    // Per-surface and per-lineage summaries must add up to corpus size.
    let surface_total: u32 = report
        .consumer_surface_summaries
        .iter()
        .map(|s| s.packet_count)
        .sum();
    assert_eq!(surface_total as usize, corpus.entries.len());
    let lineage_total: u32 = report
        .lineage_summaries
        .iter()
        .map(|s| s.packet_count)
        .sum();
    assert_eq!(lineage_total as usize, corpus.entries.len());
}

#[test]
fn aligned_packets_carry_no_downgrade_or_open_gap() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if !entry.packet.drift_indicator.is_aligned() {
            continue;
        }
        assert_eq!(
            entry.packet.downgrade_label,
            DriftDowngradeLabel::None,
            "aligned packet {} must declare downgrade_label = none",
            entry.packet.packet_id
        );
        assert!(
            entry
                .packet
                .open_gaps
                .iter()
                .all(|gap| gap.gap_class == DriftOpenGapClass::None),
            "aligned packet {} must not declare a non-none open_gap",
            entry.packet.packet_id
        );
    }
}

#[test]
fn blocked_packets_pin_red_drift_downgrade() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if entry.packet.drift_indicator != DriftIndicator::BlockedByScope {
            continue;
        }
        assert_eq!(
            entry.packet.downgrade_label,
            DriftDowngradeLabel::RedDriftBlocksBetaRow,
            "blocked_by_scope packet {} must downgrade with red_drift_blocks_beta_row",
            entry.packet.packet_id
        );
    }
}

#[test]
fn fallback_only_packets_degrade_to_fallback_search_only() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if entry.packet.drift_indicator != DriftIndicator::FallbackOnly {
            continue;
        }
        assert_eq!(
            entry.packet.downgrade_label,
            DriftDowngradeLabel::DegradedToFallbackSearchOnly,
            "fallback_only packet {} must downgrade with degraded_to_fallback_search_only",
            entry.packet.packet_id
        );
    }
}

#[test]
fn evidence_export_preserves_truth_labels_on_every_packet() {
    let corpus = current_graph_drift_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let export = &entry.packet.evidence_export;
        assert!(
            export.preserves_readiness_token
                && export.preserves_freshness_token
                && export.preserves_scope_label
                && export.preserves_lineage_label
                && export.preserves_consumer_surface_label
                && export.preserves_envelope_packet_ref,
            "packet {} must preserve readiness / freshness / scope / lineage / surface / envelope refs",
            entry.packet.packet_id
        );
        assert!(
            export.raw_private_material_excluded
                && export.ambient_authority_excluded
                && export.preserves_user_authored_files,
            "packet {} evidence_export must hold the metadata-safe baseline",
            entry.packet.packet_id
        );
    }
}

#[test]
fn closed_vocabulary_tokens_are_pinned() {
    assert_eq!(DriftConsumerSurface::Navigation.as_str(), "navigation");
    assert_eq!(DriftConsumerSurface::AiContext.as_str(), "ai_context");
    assert_eq!(DriftConsumerSurface::Review.as_str(), "review");
    assert_eq!(
        DriftConsumerSurface::SupportExport.as_str(),
        "support_export"
    );

    assert_eq!(ReadinessState::Ready.as_str(), "ready");
    assert_eq!(ReadinessState::OutOfScope.as_str(), "out_of_scope");

    assert_eq!(FreshnessClass::Authoritative.as_str(), "authoritative");
    assert_eq!(FreshnessClass::HotSet.as_str(), "hot_set");
    assert_eq!(FreshnessClass::Warming.as_str(), "warming");
    assert_eq!(FreshnessClass::Cached.as_str(), "cached");
    assert_eq!(FreshnessClass::Stale.as_str(), "stale");
    assert_eq!(FreshnessClass::Replayed.as_str(), "replayed");
    assert_eq!(FreshnessClass::Imported.as_str(), "imported");
    assert_eq!(FreshnessClass::Unknown.as_str(), "unknown");

    assert_eq!(ScopeClass::FullLocal.as_str(), "full_local");
    assert_eq!(ScopeClass::SparseLocal.as_str(), "sparse_local");
    assert_eq!(ScopeClass::FullManaged.as_str(), "full_managed");
    assert_eq!(ScopeClass::SparseManaged.as_str(), "sparse_managed");
    assert_eq!(
        ScopeClass::MixedLocalAndManaged.as_str(),
        "mixed_local_and_managed"
    );
    assert_eq!(ScopeClass::OutOfScope.as_str(), "out_of_scope");

    assert_eq!(
        DataLaneLineage::ExactLocalGraphLineage.as_str(),
        "exact_local_graph_lineage"
    );
    assert_eq!(
        DataLaneLineage::ImportedProviderLineage.as_str(),
        "imported_provider_lineage"
    );
    assert_eq!(
        DataLaneLineage::InferredDerivedLineage.as_str(),
        "inferred_derived_lineage"
    );
    assert_eq!(
        DataLaneLineage::PartialScopeLineage.as_str(),
        "partial_scope_lineage"
    );
    assert_eq!(
        DataLaneLineage::StaleCachedLineage.as_str(),
        "stale_cached_lineage"
    );
    assert_eq!(
        DataLaneLineage::WarmingProviderLineage.as_str(),
        "warming_provider_lineage"
    );
    assert_eq!(
        DataLaneLineage::OutOfScopeLineage.as_str(),
        "out_of_scope_lineage"
    );
    assert_eq!(
        DataLaneLineage::FallbackSearchLineage.as_str(),
        "fallback_search_lineage"
    );

    assert_eq!(DriftIndicator::Aligned.as_str(), "aligned");
    assert_eq!(DriftIndicator::FreshnessSkew.as_str(), "freshness_skew");
    assert_eq!(DriftIndicator::ScopeSkew.as_str(), "scope_skew");
    assert_eq!(DriftIndicator::LineageSkew.as_str(), "lineage_skew");
    assert_eq!(DriftIndicator::StaleWarning.as_str(), "stale_warning");
    assert_eq!(DriftIndicator::WarmingWarning.as_str(), "warming_warning");
    assert_eq!(DriftIndicator::BlockedByScope.as_str(), "blocked_by_scope");
    assert_eq!(DriftIndicator::FallbackOnly.as_str(), "fallback_only");

    assert_eq!(DriftDowngradeLabel::None.as_str(), "none");
    assert_eq!(
        DriftDowngradeLabel::RedDriftBlocksBetaRow.as_str(),
        "red_drift_blocks_beta_row"
    );
    assert_eq!(
        DriftDowngradeLabel::YellowFreshnessSkew.as_str(),
        "yellow_freshness_skew"
    );
    assert_eq!(
        DriftDowngradeLabel::YellowScopeSkew.as_str(),
        "yellow_scope_skew"
    );
    assert_eq!(
        DriftDowngradeLabel::YellowLineageSkew.as_str(),
        "yellow_lineage_skew"
    );
    assert_eq!(
        DriftDowngradeLabel::DegradedToFallbackSearchOnly.as_str(),
        "degraded_to_fallback_search_only"
    );
    assert_eq!(
        DriftDowngradeLabel::StaleCorpusBlocksReleaseCandidate.as_str(),
        "stale_corpus_blocks_release_candidate"
    );

    assert_eq!(DriftOpenGapClass::None.as_str(), "none");
    assert_eq!(
        DriftOpenGapClass::FreshnessPending.as_str(),
        "freshness_pending"
    );
    assert_eq!(DriftOpenGapClass::ScopePending.as_str(), "scope_pending");
    assert_eq!(
        DriftOpenGapClass::LineagePending.as_str(),
        "lineage_pending"
    );
    assert_eq!(
        DriftOpenGapClass::EvidenceExportPending.as_str(),
        "evidence_export_pending"
    );
    assert_eq!(
        DriftOpenGapClass::FallbackTruthOnly.as_str(),
        "fallback_truth_only"
    );
    assert_eq!(DriftOpenGapClass::DriftBlocked.as_str(), "drift_blocked");
}
