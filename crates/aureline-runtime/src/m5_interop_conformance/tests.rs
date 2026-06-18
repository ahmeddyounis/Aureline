//! Inline unit coverage for the interop conformance packet: seed stability, the
//! four named corpora and their archetype coverage, the seven graded conformance
//! dimensions, the freshness/stale-narrowing roll-up, the release-evidence
//! binding, and the fail-closed guardrails against overclaimed confidence, lost
//! raw payloads, missing fallback reasons, hidden degraded states, broken replay,
//! and broken export parity.

use super::*;

fn seed() -> InteropConformancePacket {
    seeded_interop_conformance_packet()
}

#[test]
fn seed_materializes_stable() {
    let packet = seed();
    assert!(
        packet.validate().is_empty(),
        "seed must validate clean: {:?}",
        packet.validate()
    );
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::Stable
    );
    assert_eq!(packet.record_kind, INTEROP_CONFORMANCE_RECORD_KIND);
    assert_eq!(packet.schema_version, INTEROP_CONFORMANCE_SCHEMA_VERSION);
}

#[test]
fn seed_carries_every_corpus_family() {
    let packet = seed();
    assert_eq!(
        packet.corpus_family_tokens(),
        vec![
            "bsp_discovery",
            "bazel_bep_bes",
            "structured_output_junit_sarif",
            "problem_matcher_heuristic",
        ]
    );
}

#[test]
fn seed_covers_every_claimed_archetype() {
    let packet = seed();
    assert_eq!(
        packet.archetype_tokens(),
        vec![
            "rust_cargo",
            "node_workspace",
            "python_pytest",
            "jvm_build_server",
            "bazel_monorepo",
            "polyglot_ci",
        ]
    );
}

#[test]
fn every_corpus_covers_its_dependent_archetypes() {
    let packet = seed();
    for family in CorpusFamily::ALL {
        let corpus = packet.corpus_for(family).expect("corpus present");
        let covered: Vec<InteropArchetype> = corpus.cases.iter().map(|c| c.archetype).collect();
        for archetype in archetypes_for_family(family) {
            assert!(
                covered.contains(&archetype),
                "{} must cover {}",
                family.as_str(),
                archetype.as_str()
            );
        }
    }
}

#[test]
fn every_seed_case_conforms_on_all_dimensions() {
    let packet = seed();
    for case in packet.cases() {
        assert!(case.conforms, "case {} must conform", case.case_id);
        assert_eq!(
            case.dimension_outcomes.len(),
            ConformanceDimension::ALL.len()
        );
        for outcome in &case.dimension_outcomes {
            assert!(
                outcome.passed,
                "case {} dimension {} must pass",
                case.case_id,
                outcome.dimension.as_str()
            );
        }
    }
}

#[test]
fn heuristic_corpus_runs_degraded_low_confidence_with_a_named_reason() {
    let packet = seed();
    let corpus = packet
        .corpus_for(CorpusFamily::ProblemMatcherHeuristic)
        .expect("heuristic corpus present");
    assert_eq!(
        corpus.source_kind,
        BuildTestEventSourceKind::HeuristicParser
    );
    for case in &corpus.cases {
        assert_eq!(case.negotiated_capability, AdapterCapabilityState::Degraded);
        assert_eq!(case.observed_confidence, BuildTestEventConfidence::Low);
        assert!(case.fallback_reason.is_some());
        assert!(case.degraded_state_disclosed);
        assert!(case.conforms);
    }
}

#[test]
fn seed_release_evidence_is_current_and_conforming() {
    let packet = seed();
    assert!(packet.release_evidence.all_corpora_current);
    assert!(packet.release_evidence.all_cases_conform);
    assert!(packet.release_evidence.narrowed_families.is_empty());
    assert_eq!(
        packet.release_evidence.release_evidence_ref,
        INTEROP_CONFORMANCE_RELEASE_EVIDENCE_REF
    );
}

#[test]
fn evidence_joins_explain_consistently() {
    let packet = seed();
    for surface in [
        ConformanceEvidenceSurface::SupportBundle,
        ConformanceEvidenceSurface::IncidentPacket,
        ConformanceEvidenceSurface::AiEvidence,
    ] {
        let view = packet.evidence_join(surface, "view", "2026-06-18T00:01:00Z");
        assert!(
            view.explains_consistently(),
            "{} must explain",
            surface.as_str()
        );
        assert_eq!(view.corpus_rows.len(), packet.corpora.len());
        assert_eq!(view.case_rows.len(), packet.cases().count());
        assert_eq!(view.corpus_digest, packet.corpus_digest);
        assert_eq!(view.release_evidence, packet.release_evidence);
    }
}

#[test]
fn cli_headless_view_runs_every_corpus() {
    let packet = seed();
    let view =
        packet.cli_headless_view(INTEROP_CONFORMANCE_CLI_HEADLESS_ID, "2026-06-18T00:01:00Z");
    assert!(view.every_corpus_runs());
    assert_eq!(view.corpus_rows.len(), packet.corpora.len());
    assert_eq!(view.case_rows.len(), packet.cases().count());
    assert_eq!(view.corpus_digest, packet.corpus_digest);
    assert_eq!(view.promotion_state, packet.promotion_state);
}

#[test]
fn support_export_round_trips_and_stays_safe() {
    let packet = seed();
    let export = packet.support_export(
        INTEROP_CONFORMANCE_SUPPORT_EXPORT_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: InteropConformanceSupportExport = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
    assert!(parsed.packet.is_stable());
    assert_eq!(parsed.packet.corpus_digest, packet.corpus_digest);
}

fn first_case_for(
    input: &mut InteropConformancePacketInput,
    family: CorpusFamily,
) -> &mut ConformanceCase {
    let corpus = input
        .corpora
        .iter_mut()
        .find(|c| c.family == family)
        .expect("corpus present");
    corpus.cases.first_mut().expect("corpus has a case")
}

#[test]
fn confidence_overclaim_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::ProblemMatcherHeuristic).observed_confidence =
        BuildTestEventConfidence::High;
    let packet = InteropConformancePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::ConfidenceOverclaim));
}

#[test]
fn missing_fallback_reason_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::ProblemMatcherHeuristic).fallback_reason = None;
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::FallbackReasonMissing));
}

#[test]
fn spurious_fallback_reason_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::BspDiscovery).fallback_reason =
        Some("unexpected".to_owned());
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::FallbackReasonUnexpected));
}

#[test]
fn degraded_state_not_disclosed_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::ProblemMatcherHeuristic).degraded_state_disclosed =
        false;
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::DegradedStateNotDisclosed));
}

#[test]
fn raw_payload_not_retained_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::StructuredOutputJunitSarif)
        .raw_private_material_excluded = false;
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::RawPayloadNotRetained));
}

#[test]
fn replay_unstable_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::BazelBepBes).replay_stable = false;
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::ReplayUnstable));
}

#[test]
fn export_parity_broken_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::StructuredOutputJunitSarif).export_parity_preserved =
        false;
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::ExportParityBroken));
}

#[test]
fn capability_handshake_missing_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    first_case_for(&mut input, CorpusFamily::BspDiscovery).capability_packet_ref = String::new();
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::CapabilityNegotiationMissing));
}

#[test]
fn missing_corpus_family_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    input
        .corpora
        .retain(|c| c.family != CorpusFamily::BazelBepBes);
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::MissingCorpusFamily));
}

#[test]
fn missing_archetype_coverage_blocks_stable() {
    let mut input = current_stable_interop_conformance_input();
    let corpus = input
        .corpora
        .iter_mut()
        .find(|c| c.family == CorpusFamily::BspDiscovery)
        .expect("bsp corpus");
    corpus
        .cases
        .retain(|case| case.archetype != InteropArchetype::JvmBuildServer);
    let packet = InteropConformancePacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::MissingArchetypeCoverage));
}

#[test]
fn stale_corpus_narrows_below_stable_without_blocking() {
    let mut input = current_stable_interop_conformance_input();
    let corpus = input
        .corpora
        .iter_mut()
        .find(|c| c.family == CorpusFamily::ProblemMatcherHeuristic)
        .expect("heuristic corpus");
    corpus.proof_age_days = corpus.freshness_window_days + 10;
    let packet = InteropConformancePacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::NarrowedBelowStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::CorpusEvidenceStale));
    // Narrowing is a warning, not a blocker: no blocker-level finding present.
    assert!(packet.is_stable());
    // The release-evidence roll-up records the narrowing source.
    assert!(!packet.release_evidence.all_corpora_current);
    assert!(packet
        .release_evidence
        .narrowed_families
        .contains(&CorpusFamily::ProblemMatcherHeuristic.as_str().to_owned()));
}

#[test]
fn corpus_digest_drift_is_caught() {
    let mut packet = seed();
    packet.corpus_digest = "fnv1a64:0000000000000000".to_owned();
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::CorpusDigestDrift));
}

#[test]
fn release_evidence_drift_is_caught() {
    let mut packet = seed();
    packet.release_evidence.all_cases_conform = false;
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::ReleaseEvidenceDrift));
}

#[test]
fn case_conformance_drift_is_caught() {
    let mut packet = seed();
    if let Some(corpus) = packet.corpora.first_mut() {
        if let Some(case) = corpus.cases.first_mut() {
            case.conforms = false;
        }
    }
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == InteropConformanceFindingKind::CaseConformanceDrift));
}
