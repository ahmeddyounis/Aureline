use std::collections::BTreeSet;

use aureline_language::{
    provider_arbitration_diagnostics_convergence::{
        current_diagnostics_convergence_corpus, current_diagnostics_convergence_fixture_refs,
        current_diagnostics_convergence_packet, ConvergenceDisplayStateClass, ConvergenceInspector,
        ProviderArbitrationDiagnosticsConvergencePacket, SeverityConvergenceClass,
        SuppressionClass,
    },
    ConfidenceOutcomeClass, DiagnosticFreshnessClass,
};

#[test]
fn corpus_loads_all_fixtures() {
    let corpus = current_diagnostics_convergence_corpus().expect("corpus parses");
    let refs: Vec<_> = current_diagnostics_convergence_fixture_refs().collect();
    assert_eq!(corpus.len(), 8, "expected 8 fixtures");
    assert_eq!(corpus.len(), refs.len());
}

#[test]
fn packet_is_export_safe_and_conformant() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    assert!(packet.is_export_safe());
    let report = packet.validate();
    assert!(
        report.is_conformant(),
        "packet validation failed: {:?}",
        report.defects
    );
}

#[test]
fn packet_covers_all_outcomes() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    let outcomes: BTreeSet<_> = packet
        .clusters
        .iter()
        .map(|c| c.convergence_outcome_class)
        .collect();
    assert!(outcomes.contains(&ConfidenceOutcomeClass::Exact));
    assert!(outcomes.contains(&ConfidenceOutcomeClass::Heuristic));
    assert!(outcomes.contains(&ConfidenceOutcomeClass::Partial));
    assert!(outcomes.contains(&ConfidenceOutcomeClass::Stale));
}

#[test]
fn packet_covers_all_display_states() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    let states: BTreeSet<_> = packet
        .clusters
        .iter()
        .map(|c| c.dominant_display_state_class)
        .collect();
    assert!(states.contains(&ConvergenceDisplayStateClass::CurrentExactLive));
    assert!(states.contains(&ConvergenceDisplayStateClass::CurrentWithSeverityConflict));
    assert!(states.contains(&ConvergenceDisplayStateClass::DowngradedForProviderDisagreement));
    assert!(states.contains(&ConvergenceDisplayStateClass::StaleOrSuperseded));
    assert!(states.contains(&ConvergenceDisplayStateClass::CurrentMixedStaticAndRuntime));
    assert!(states.contains(&ConvergenceDisplayStateClass::SuppressedOrBaselinedGoverned));
}

#[test]
fn no_hidden_disagreements_in_corpus() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    let inspector = ConvergenceInspector::new();
    let hidden = inspector.hidden_disagreements(&packet);
    assert!(
        hidden.is_empty(),
        "clusters with hidden disagreements: {:?}",
        hidden
    );
}

#[test]
fn truth_labels_preserved_in_every_cluster() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    let inspector = ConvergenceInspector::new();
    assert!(inspector.truth_labels_preserved(&packet));
}

#[test]
fn every_cluster_has_exactly_one_primary_claim() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    for cluster in &packet.clusters {
        let primary_count = cluster
            .provider_claim_rows
            .iter()
            .filter(|r| r.is_primary)
            .count();
        assert_eq!(
            primary_count, 1,
            "cluster {} must have exactly one primary claim, found {}",
            cluster.cluster_id, primary_count
        );
    }
}

#[test]
fn conflicting_severity_clusters_block_mixed_batch_fix() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    for cluster in &packet.clusters {
        if cluster.severity_convergence_class
            == SeverityConvergenceClass::ConflictingSeverityPresent
        {
            assert!(
                cluster.blocks_broad_batch_apply(),
                "cluster {} with conflicting severity must block broad batch apply",
                cluster.cluster_id
            );
        }
    }
}

#[test]
fn suppression_preserves_provider_claims() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    for cluster in &packet.clusters {
        let suppressed_providers: Vec<_> = cluster
            .provider_claim_rows
            .iter()
            .filter(|r| r.suppression_class != SuppressionClass::NotSuppressed)
            .map(|r| r.provider_id.clone())
            .collect();
        if !suppressed_providers.is_empty() {
            let unsuppressed_count = cluster
                .provider_claim_rows
                .iter()
                .filter(|r| r.suppression_class == SuppressionClass::NotSuppressed)
                .count();
            assert!(
                unsuppressed_count > 0 || cluster.aggregate_counts.suppressed_count == cluster.aggregate_counts.total_provider_claims,
                "cluster {} has suppressed providers but should still preserve unsuppressed claims when not fully suppressed",
                cluster.cluster_id
            );
        }
    }
}

#[test]
fn aggregate_counts_match_cluster_contents() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    let expected =
        ProviderArbitrationDiagnosticsConvergencePacket::build_aggregate_counts(&packet.clusters);
    assert_eq!(
        packet.aggregate_counts, expected,
        "packet aggregate counts must match derived values"
    );
}

#[test]
fn stale_clusters_downgrade_freshness() {
    let packet = current_diagnostics_convergence_packet().expect("packet builds");
    for cluster in &packet.clusters {
        if cluster.convergence_outcome_class == ConfidenceOutcomeClass::Stale {
            assert!(
                matches!(
                    cluster.cluster_freshness_class,
                    DiagnosticFreshnessClass::Stale
                        | DiagnosticFreshnessClass::DegradedCached
                        | DiagnosticFreshnessClass::Unverified
                ),
                "cluster {} with stale outcome must have stale or worse freshness",
                cluster.cluster_id
            );
        }
    }
}
