//! Integration drill for the semantic graph readiness and
//! exact-vs-imported fact label beta projection.
//!
//! The drill re-proves the on-disk presence of the schema, reviewer
//! doc, baseline report, fixture corpus, and manifest, then round-trips
//! every case through serde, runs the evaluator over the checked-in
//! corpus, and pins the closed-vocabulary tokens exposed by the
//! report.

use std::path::{Path, PathBuf};

use aureline_graph::{
    current_graph_readiness_beta_corpus, BetaConsumerSurface, ClaimAlignmentState, DowngradeLabel,
    FactLane, GraphReadinessBetaEvaluator, BetaOpenGapClass as OpenGapClass, ReadinessClaim,
    GRAPH_READINESS_BETA_CORPUS_DIR, GRAPH_READINESS_BETA_CORPUS_MANIFEST_REF,
    GRAPH_READINESS_BETA_DOC_REF, GRAPH_READINESS_BETA_REPORT_REF,
    GRAPH_READINESS_BETA_SCHEMA_REF, REQUIRED_CONSUMER_SURFACES, REQUIRED_FACT_LANES,
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
    assert_exists(GRAPH_READINESS_BETA_SCHEMA_REF);
    assert_exists(GRAPH_READINESS_BETA_DOC_REF);
    assert_exists(GRAPH_READINESS_BETA_REPORT_REF);
    assert_exists(GRAPH_READINESS_BETA_CORPUS_MANIFEST_REF);
}

#[test]
fn every_required_consumer_surface_appears_in_the_corpus() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    for surface in REQUIRED_CONSUMER_SURFACES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.case.consumer_surface == surface),
            "missing required consumer_surface = {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_required_fact_lane_appears_in_the_corpus() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    for lane in REQUIRED_FACT_LANES {
        assert!(
            corpus
                .entries
                .iter()
                .any(|entry| entry.case.observed_envelope_lane == lane),
            "missing required observed_envelope_lane = {}",
            lane.as_str()
        );
    }
}

#[test]
fn at_least_one_overclaim_case_exists() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    assert!(
        corpus
            .entries
            .iter()
            .any(|entry| entry.case.claim_alignment_state
                == ClaimAlignmentState::OverclaimBlocked),
        "corpus must seed at least one overclaim_blocked case"
    );
}

#[test]
fn checked_in_corpus_validates() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    GraphReadinessBetaEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
}

#[test]
fn corpus_round_trips_through_serde() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let json = serde_json::to_string(&entry.case).expect("case serializes to JSON");
        let parsed: aureline_graph::GraphReadinessBetaCase =
            serde_json::from_str(&json).expect("case parses back from JSON");
        assert_eq!(parsed, entry.case);
    }
}

#[test]
fn report_emits_closed_vocabulary_tokens() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    let report = GraphReadinessBetaEvaluator::new()
        .report("report:graph_readiness_beta", "2026-05-16T10:00:00Z", &corpus)
        .expect("report builds");
    assert!(report.is_export_safe());

    for row in &report.matrix_rows {
        // Closed vocabularies — emitted tokens must round-trip through serde.
        let json = serde_json::to_string(&row).expect("matrix row serializes");
        let parsed: aureline_graph::BetaReportMatrixRow =
            serde_json::from_str(&json).expect("matrix row parses");
        assert_eq!(parsed, *row);
    }

    // Per-fact-lane and per-surface summaries must add up to corpus size.
    let lane_total: u32 = report.fact_lane_summaries.iter().map(|s| s.case_count).sum();
    assert_eq!(lane_total as usize, corpus.entries.len());
    let surface_total: u32 = report
        .consumer_surface_summaries
        .iter()
        .map(|s| s.case_count)
        .sum();
    assert_eq!(surface_total as usize, corpus.entries.len());
}

#[test]
fn aligned_rows_carry_no_downgrade_or_open_gap() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        if entry.case.claim_alignment_state != ClaimAlignmentState::Aligned {
            continue;
        }
        assert_eq!(
            entry.case.downgrade_label,
            DowngradeLabel::None,
            "aligned case {} must declare downgrade_label = none",
            entry.case.case_id
        );
        assert!(
            entry
                .case
                .open_gaps
                .iter()
                .all(|gap| gap.gap_class == OpenGapClass::None),
            "aligned case {} must not declare a non-none open_gap",
            entry.case.case_id
        );
    }
}

#[test]
fn overclaim_rows_pin_red_and_overclaim_gap() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    let mut saw_any = false;
    for entry in &corpus.entries {
        if entry.case.claim_alignment_state != ClaimAlignmentState::OverclaimBlocked {
            continue;
        }
        saw_any = true;
        assert_eq!(
            entry.case.downgrade_label,
            DowngradeLabel::RedBlocksBetaRow,
            "overclaim case {} must downgrade with red_blocks_beta_row",
            entry.case.case_id
        );
        assert!(
            entry
                .case
                .open_gaps
                .iter()
                .any(|gap| gap.gap_class == OpenGapClass::OverclaimBlocked),
            "overclaim case {} must record an overclaim_blocked open_gap",
            entry.case.case_id
        );
    }
    assert!(saw_any, "at least one overclaim_blocked case is required");
}

#[test]
fn evidence_export_preserves_truth_labels_on_every_case() {
    let corpus = current_graph_readiness_beta_corpus().expect("corpus loads");
    for entry in &corpus.entries {
        let export = &entry.case.evidence_export;
        assert!(
            export.preserves_fact_lane_label
                && export.preserves_readiness_token
                && export.preserves_consumer_surface_label
                && export.preserves_envelope_packet_ref,
            "case {} must preserve fact lane / readiness / surface / envelope refs in evidence export",
            entry.case.case_id
        );
        assert!(
            export.raw_private_material_excluded
                && export.ambient_authority_excluded
                && export.preserves_user_authored_files,
            "case {} evidence_export must hold the metadata-safe baseline",
            entry.case.case_id
        );
    }
}

#[test]
fn fixture_directory_paths_are_pinned_on_disk() {
    assert_exists(GRAPH_READINESS_BETA_CORPUS_DIR);
}

#[test]
fn closed_vocabulary_tokens_are_pinned() {
    // Pin the closed-vocabulary tokens so downstream consumers cannot drift
    // them silently. These strings travel through schemas, fixtures, and
    // exported reports.
    assert_eq!(BetaConsumerSurface::Navigation.as_str(), "navigation");
    assert_eq!(BetaConsumerSurface::AiContext.as_str(), "ai_context");
    assert_eq!(BetaConsumerSurface::Review.as_str(), "review");
    assert_eq!(BetaConsumerSurface::SupportExport.as_str(), "support_export");

    assert_eq!(
        FactLane::ExactLocalGraphFact.as_str(),
        "exact_local_graph_fact"
    );
    assert_eq!(FactLane::ImportedGraphFact.as_str(), "imported_graph_fact");
    assert_eq!(FactLane::InferredGraphFact.as_str(), "inferred_graph_fact");
    assert_eq!(FactLane::PartialGraphFact.as_str(), "partial_graph_fact");
    assert_eq!(FactLane::StaleGraphFact.as_str(), "stale_graph_fact");
    assert_eq!(
        FactLane::WaitingOnGraphProvider.as_str(),
        "waiting_on_graph_provider"
    );
    assert_eq!(
        FactLane::OutOfScopeGraphFact.as_str(),
        "out_of_scope_graph_fact"
    );
    assert_eq!(FactLane::FallbackSearchFact.as_str(), "fallback_search_fact");

    assert_eq!(ReadinessClaim::Ready.as_str(), "ready");
    assert_eq!(ReadinessClaim::OutOfScope.as_str(), "out_of_scope");

    assert_eq!(ClaimAlignmentState::Aligned.as_str(), "aligned");
    assert_eq!(
        ClaimAlignmentState::WeakerClaimAccepted.as_str(),
        "weaker_claim_accepted"
    );
    assert_eq!(
        ClaimAlignmentState::OverclaimBlocked.as_str(),
        "overclaim_blocked"
    );

    assert_eq!(DowngradeLabel::None.as_str(), "none");
    assert_eq!(
        DowngradeLabel::RedBlocksBetaRow.as_str(),
        "red_blocks_beta_row"
    );
    assert_eq!(
        DowngradeLabel::YellowFactLanePartial.as_str(),
        "yellow_fact_lane_partial"
    );
    assert_eq!(
        DowngradeLabel::YellowEvidenceExportSkew.as_str(),
        "yellow_evidence_export_skew"
    );
    assert_eq!(
        DowngradeLabel::DegradedToFallbackSearchOnly.as_str(),
        "degraded_to_fallback_search_only"
    );
    assert_eq!(
        DowngradeLabel::StaleCorpusBlocksReleaseCandidate.as_str(),
        "stale_corpus_blocks_release_candidate"
    );

    assert_eq!(OpenGapClass::None.as_str(), "none");
    assert_eq!(
        OpenGapClass::ConsumerSurfacePending.as_str(),
        "consumer_surface_pending"
    );
    assert_eq!(OpenGapClass::FactLanePending.as_str(), "fact_lane_pending");
    assert_eq!(
        OpenGapClass::EvidenceExportPending.as_str(),
        "evidence_export_pending"
    );
    assert_eq!(
        OpenGapClass::FallbackTruthOnly.as_str(),
        "fallback_truth_only"
    );
    assert_eq!(OpenGapClass::OverclaimBlocked.as_str(), "overclaim_blocked");
}
