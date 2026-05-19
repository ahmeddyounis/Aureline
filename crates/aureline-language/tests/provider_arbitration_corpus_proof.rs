use std::collections::BTreeSet;

use aureline_language::provider_arbitration::{
    build_downgraded_semantic_claims_matrix, classify_claim_status, classify_proof_scenario,
    current_provider_arbitration_proof_corpus, current_provider_arbitration_proof_fixture_refs,
    ApplyGateClass, ArbitrationCompletenessClass, ArbitrationHealthState, ArbitrationInspector,
    ArbitrationLocalityClass, ClaimStatusClass, ConfidenceOutcomeClass, ConflictClass,
    ConsumerSurfaceClass, DisagreementVisibilityClass, DowngradedPromiseReasonClass,
    FallbackLabelClass, LanguageActionLaneClass, ProofScenarioClass, ProviderFamily,
    DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_RECORD_KIND, DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ROW_RECORD_KIND,
    PROVIDER_ARBITRATION_PROOF_CORPUS_DIR,
};

#[test]
fn proof_corpus_parses_and_validates_against_inspector() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let report = ArbitrationInspector::new()
        .report(
            "language:provider-arbitration:proof:report:01",
            "2026-05-19T19:00:00Z",
            &corpus,
        )
        .expect("proof corpus validates against inspector contract");
    assert!(report.is_export_safe());
    assert_eq!(
        report.aggregate_counts.total_rows,
        corpus.entries.len() as u32
    );
    assert!(report.aggregate_counts.conflict_rows >= 1);
    assert!(report.aggregate_counts.quarantined_provider_rows >= 1);
    assert!(report.aggregate_counts.preview_gated_rows >= 1);
    assert!(report.aggregate_counts.side_branch_gated_rows >= 1);
}

#[test]
fn proof_corpus_covers_every_required_lane_and_outcome() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");

    let lanes = corpus
        .entries
        .iter()
        .map(|entry| entry.arbitration_decision.language_action_lane_class)
        .collect::<BTreeSet<_>>();
    for required in [
        LanguageActionLaneClass::Definition,
        LanguageActionLaneClass::References,
        LanguageActionLaneClass::Rename,
        LanguageActionLaneClass::Formatting,
        LanguageActionLaneClass::OrganizeImports,
        LanguageActionLaneClass::CodeAction,
    ] {
        assert!(lanes.contains(&required), "missing lane {required:?}");
    }

    let outcomes = corpus
        .entries
        .iter()
        .map(|entry| entry.arbitration_decision.confidence_outcome_class)
        .collect::<BTreeSet<_>>();
    for required in [
        ConfidenceOutcomeClass::Exact,
        ConfidenceOutcomeClass::Heuristic,
        ConfidenceOutcomeClass::Partial,
        ConfidenceOutcomeClass::Stale,
        ConfidenceOutcomeClass::Unavailable,
    ] {
        assert!(
            outcomes.contains(&required),
            "proof corpus must cover outcome {required:?}"
        );
    }

    let gates = corpus
        .entries
        .iter()
        .map(|entry| entry.arbitration_decision.apply_gate_class)
        .collect::<BTreeSet<_>>();
    for required in [
        ApplyGateClass::ReadyToApply,
        ApplyGateClass::PreviewRequired,
        ApplyGateClass::SideBranchRequired,
        ApplyGateClass::BlockedForHealth,
    ] {
        assert!(
            gates.contains(&required),
            "proof corpus must cover apply gate {required:?}"
        );
    }
}

#[test]
fn proof_corpus_covers_every_required_scenario_class() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let scenarios = corpus
        .entries
        .iter()
        .map(classify_proof_scenario)
        .collect::<BTreeSet<_>>();
    for required in [
        ProofScenarioClass::ProviderAgreement,
        ProofScenarioClass::ProviderDisagreement,
        ProofScenarioClass::PartialScope,
        ProofScenarioClass::ImportedSnapshot,
        ProofScenarioClass::StaleCacheReuse,
        ProofScenarioClass::ProviderCrashLoop,
        ProofScenarioClass::WideScopeRename,
        ProofScenarioClass::TextFallback,
        ProofScenarioClass::ProviderPreferenceReorder,
    ] {
        assert!(
            scenarios.contains(&required),
            "proof corpus must cover scenario {required:?}"
        );
    }
}

#[test]
fn crash_loop_drills_cover_required_provider_families() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let crash_families: BTreeSet<ProviderFamily> = corpus
        .entries
        .iter()
        .flat_map(|entry| entry.provider_health_states.iter())
        .filter(|row| row.health_state == ArbitrationHealthState::CrashLoopQuarantined)
        .map(|row| row.provider_family)
        .collect();
    for required in [
        ProviderFamily::LanguageServer,
        ProviderFamily::FrameworkPack,
        ProviderFamily::NotebookAdapter,
    ] {
        assert!(
            crash_families.contains(&required),
            "proof corpus must include crash-loop drill for {required:?}"
        );
    }
}

#[test]
fn imported_snapshot_locality_is_attested() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    assert!(
        corpus.entries.iter().any(|entry| {
            entry
                .provider_health_states
                .iter()
                .any(|row| row.locality_class == ArbitrationLocalityClass::ImportedSnapshot)
        }),
        "proof corpus must include at least one imported-snapshot locality"
    );
}

#[test]
fn no_disagreement_or_partial_scope_silently_emits_full_confidence() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    for entry in &corpus.entries {
        let decision = &entry.arbitration_decision;
        if decision.disagreement_block.conflict_class != ConflictClass::None {
            assert_ne!(
                decision.apply_gate_class,
                ApplyGateClass::ReadyToApply,
                "{} must not silently emit a ready-to-apply with a non-empty conflict",
                decision.arbitration_decision_id
            );
            assert_ne!(
                decision.disagreement_block.disagreement_visibility_class,
                DisagreementVisibilityClass::None,
                "{} must keep the disagreement visible",
                decision.arbitration_decision_id
            );
        }
        if decision.negotiated_completeness_class
            == ArbitrationCompletenessClass::PartialForClaimedScope
        {
            assert_ne!(
                decision.confidence_outcome_class,
                ConfidenceOutcomeClass::Exact,
                "{} must not label a partial-scope outcome as exact",
                decision.arbitration_decision_id
            );
        }
        if decision.confidence_outcome_class != ConfidenceOutcomeClass::Exact
            && decision.confidence_outcome_class != ConfidenceOutcomeClass::Unavailable
        {
            assert_ne!(
                decision.fallback_label_class,
                FallbackLabelClass::None,
                "{} must surface a fallback label for non-exact outcomes",
                decision.arbitration_decision_id
            );
        }
    }
}

#[test]
fn wide_scope_rename_with_partial_truth_requires_preview_or_side_branch() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let rename_rows = corpus
        .entries
        .iter()
        .filter(|entry| {
            entry.arbitration_decision.language_action_lane_class == LanguageActionLaneClass::Rename
        })
        .collect::<Vec<_>>();
    assert!(!rename_rows.is_empty());
    for entry in rename_rows {
        let decision = &entry.arbitration_decision;
        if decision.negotiated_completeness_class
            == ArbitrationCompletenessClass::PartialForClaimedScope
        {
            assert!(
                matches!(
                    decision.apply_gate_class,
                    ApplyGateClass::PreviewRequired
                        | ApplyGateClass::SideBranchRequired
                        | ApplyGateClass::BlockedForPartialScope
                        | ApplyGateClass::InspectOnly
                ),
                "{} must gate apply when rename is partial",
                decision.arbitration_decision_id
            );
        }
    }
}

#[test]
fn every_decision_routes_to_every_required_consumer() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    for entry in &corpus.entries {
        let routed: BTreeSet<ConsumerSurfaceClass> = entry
            .arbitration_decision
            .consumer_routing_rows
            .iter()
            .map(|row| row.consumer_surface_class)
            .collect();
        for required in [
            ConsumerSurfaceClass::EditorChrome,
            ConsumerSurfaceClass::QuickFixPreview,
            ConsumerSurfaceClass::DiagnosticsDetail,
            ConsumerSurfaceClass::CommandResult,
            ConsumerSurfaceClass::CliHeadlessInspect,
            ConsumerSurfaceClass::SupportExport,
        ] {
            assert!(
                routed.contains(&required),
                "{} must route to {required:?}",
                entry.arbitration_decision.arbitration_decision_id
            );
        }
    }
}

#[test]
fn fixture_refs_all_live_in_corpus_directory() {
    let refs: Vec<&str> = current_provider_arbitration_proof_fixture_refs().collect();
    assert!(refs.len() >= 18, "proof corpus must hold at least 18 fixtures");
    for fixture_ref in refs {
        assert!(
            fixture_ref.starts_with(PROVIDER_ARBITRATION_PROOF_CORPUS_DIR),
            "fixture ref {fixture_ref} must live in the proof corpus directory"
        );
    }
}

#[test]
fn downgraded_semantic_claims_matrix_round_trips_to_json() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let matrix = build_downgraded_semantic_claims_matrix(
        &corpus,
        "language:downgraded_semantic_claims_matrix:01",
        "2026-05-19T19:00:00Z",
    );
    assert_eq!(
        matrix.record_kind,
        DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_RECORD_KIND
    );
    assert!(matrix.raw_payload_excluded);
    assert!(matrix.raw_private_material_excluded);
    assert_eq!(matrix.rows.len(), corpus.entries.len());
    for row in &matrix.rows {
        assert_eq!(
            row.record_kind,
            DOWNGRADED_SEMANTIC_CLAIMS_MATRIX_ROW_RECORD_KIND
        );
    }

    let mut statuses = BTreeSet::new();
    for entry in &corpus.entries {
        statuses.insert(classify_claim_status(entry));
    }
    for required in [
        ClaimStatusClass::QualifiedForBetaClaim,
        ClaimStatusClass::DowngradedDiscloseAndProceed,
        ClaimStatusClass::BlockedForRecovery,
    ] {
        assert!(
            statuses.contains(&required),
            "matrix must cover claim status {required:?}"
        );
    }

    let serialized = serde_json::to_string(&matrix).expect("matrix serializes");
    let round_trip: aureline_language::provider_arbitration::DowngradedSemanticClaimsMatrix =
        serde_json::from_str(&serialized).expect("matrix deserializes");
    assert_eq!(matrix, round_trip);
}

#[test]
fn checked_in_downgraded_semantic_claims_matrix_matches_corpus() {
    const CHECKED_IN_MATRIX_JSON: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/language/m3/downgraded_semantic_claims_matrix.json"
    ));
    let checked_in: aureline_language::provider_arbitration::DowngradedSemanticClaimsMatrix =
        serde_json::from_str(CHECKED_IN_MATRIX_JSON)
            .expect("checked-in matrix JSON parses against the matrix schema");
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let regenerated = build_downgraded_semantic_claims_matrix(
        &corpus,
        checked_in.matrix_id.clone(),
        checked_in.captured_at.clone(),
    );
    assert_eq!(
        checked_in.rows, regenerated.rows,
        "checked-in matrix rows have drifted from the corpus; regenerate the JSON artifact"
    );
    assert_eq!(checked_in.corpus_dir, regenerated.corpus_dir);
    assert_eq!(
        checked_in.claim_qualification_doc_ref,
        regenerated.claim_qualification_doc_ref
    );
}

#[test]
fn downgraded_promise_reasons_are_consistent_with_outcome() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    for entry in &corpus.entries {
        let decision = &entry.arbitration_decision;
        if decision.confidence_outcome_class == ConfidenceOutcomeClass::Exact {
            assert_eq!(
                decision
                    .downgraded_promise_block
                    .downgraded_promise_reason_class,
                DowngradedPromiseReasonClass::None,
                "{} must not declare a downgrade reason for an exact outcome",
                decision.arbitration_decision_id
            );
        }
        if decision.confidence_outcome_class == ConfidenceOutcomeClass::Unavailable {
            assert_ne!(
                decision
                    .downgraded_promise_block
                    .downgraded_promise_reason_class,
                DowngradedPromiseReasonClass::None,
                "{} must declare a downgrade reason for an unavailable outcome",
                decision.arbitration_decision_id
            );
        }
    }
}
