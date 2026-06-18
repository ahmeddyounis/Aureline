//! Fixture-driven coverage for the M5 build/test interop conformance corpora and
//! suite: the four named corpora (BSP discovery, Bazel BEP/BES, structured-output
//! JUnit/SARIF, problem-matcher/heuristic) running across every claimed M5
//! archetype, the seven graded conformance dimensions, the freshness/stale
//! narrowing roll-up, the release-evidence binding, and the fail-closed
//! guardrails against overclaimed confidence, lost raw payloads, missing fallback
//! reasons, hidden degraded states, broken replay, and broken export parity.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_interop_conformance_input, BuildTestInteropConfidence, ConformanceCase,
    ConformanceEvidenceSurface, CorpusFamily, InteropArchetype, InteropConformanceCliHeadlessView,
    InteropConformanceEvidenceJoinView, InteropConformancePacket, InteropConformancePacketInput,
    InteropConformanceSupportExport, InteropCorpus, INTEROP_CONFORMANCE_DOC_REF,
    INTEROP_CONFORMANCE_ENVELOPE_SCHEMA_REF, INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF,
    INTEROP_CONFORMANCE_SCHEMA_REF,
};
use serde::Deserialize;

/// Each fixture is (fixture-dir, file-name, mutation).
const CASES: [(&str, &str, &str); 11] = [
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "baseline_stable.json",
        "none",
    ),
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "structured_raw_payload_not_retained_blocks_stable.json",
        "structured_raw_payload_not_retained",
    ),
    (
        "fixtures/tooling/m5/structured-output-junit-sarif",
        "structured_export_parity_broken_blocks_stable.json",
        "structured_export_parity_broken",
    ),
    (
        "fixtures/tooling/m5/bsp-discovery",
        "bsp_capability_packet_missing_blocks_stable.json",
        "bsp_capability_packet_missing",
    ),
    (
        "fixtures/tooling/m5/bsp-discovery",
        "bsp_archetype_coverage_missing_blocks_stable.json",
        "bsp_archetype_coverage_missing",
    ),
    (
        "fixtures/tooling/m5/bazel-bep-bes",
        "bazel_replay_unstable_blocks_stable.json",
        "bazel_replay_unstable",
    ),
    (
        "fixtures/tooling/m5/bazel-bep-bes",
        "bazel_corpus_missing_blocks_stable.json",
        "bazel_corpus_missing",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_fallback_reason_missing_blocks_stable.json",
        "heuristic_fallback_reason_missing",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_confidence_overclaim_blocks_stable.json",
        "heuristic_confidence_overclaim",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_degraded_state_not_disclosed_blocks_stable.json",
        "heuristic_degraded_state_not_disclosed",
    ),
    (
        "fixtures/tooling/m5/problem-matcher-heuristic",
        "heuristic_evidence_stale_narrows_below_stable.json",
        "heuristic_evidence_stale",
    ),
];

#[derive(Debug, Deserialize)]
struct ConformanceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    corpus_family_tokens: Vec<String>,
    archetype_tokens: Vec<String>,
    dimension_tokens: Vec<String>,
    source_kind_tokens: Vec<String>,
    support_export_safe: bool,
}

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

fn load_fixture(dir: &str, file_name: &str) -> ConformanceFixture {
    let path = repo_root().join(dir).join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn corpus(input: &mut InteropConformancePacketInput, family: CorpusFamily) -> &mut InteropCorpus {
    input
        .corpora
        .iter_mut()
        .find(|c| c.family == family)
        .expect("corpus present")
}

fn first_case(
    input: &mut InteropConformancePacketInput,
    family: CorpusFamily,
) -> &mut ConformanceCase {
    corpus(input, family)
        .cases
        .first_mut()
        .expect("corpus has a case")
}

/// Mirrors the mutations applied by the `dump_m5_interop_conformance`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> InteropConformancePacketInput {
    let mut input = current_stable_interop_conformance_input();
    match mutation {
        "none" => {}
        "structured_raw_payload_not_retained" => {
            first_case(&mut input, CorpusFamily::StructuredOutputJunitSarif)
                .raw_private_material_excluded = false;
        }
        "structured_export_parity_broken" => {
            first_case(&mut input, CorpusFamily::StructuredOutputJunitSarif)
                .export_parity_preserved = false;
        }
        "bsp_capability_packet_missing" => {
            first_case(&mut input, CorpusFamily::BspDiscovery).capability_packet_ref =
                String::new();
        }
        "bsp_archetype_coverage_missing" => {
            corpus(&mut input, CorpusFamily::BspDiscovery)
                .cases
                .retain(|case| case.archetype != InteropArchetype::JvmBuildServer);
        }
        "bazel_replay_unstable" => {
            first_case(&mut input, CorpusFamily::BazelBepBes).replay_stable = false;
        }
        "bazel_corpus_missing" => {
            input
                .corpora
                .retain(|c| c.family != CorpusFamily::BazelBepBes);
        }
        "heuristic_fallback_reason_missing" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic).fallback_reason = None;
        }
        "heuristic_confidence_overclaim" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic).observed_confidence =
                BuildTestInteropConfidence::High;
        }
        "heuristic_degraded_state_not_disclosed" => {
            first_case(&mut input, CorpusFamily::ProblemMatcherHeuristic)
                .degraded_state_disclosed = false;
        }
        "heuristic_evidence_stale" => {
            let corpus = corpus(&mut input, CorpusFamily::ProblemMatcherHeuristic);
            corpus.proof_age_days = corpus.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_list(observed: Vec<&str>, expected: &[String], label: &str) {
    let observed: Vec<String> = observed.into_iter().map(str::to_owned).collect();
    assert_eq!(&observed, expected, "{label} token drift");
}

fn assert_fixture_matches(dir: &str, file_name: &str) {
    let fixture = load_fixture(dir, file_name);
    assert_eq!(
        fixture.record_kind, "m5_interop_conformance_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet = InteropConformancePacket::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = packet
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        observed_kinds,
        fixture
            .expect
            .expected_finding_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} finding kinds drift",
        fixture.case_name
    );
    assert_token_list(
        packet.corpus_family_tokens(),
        &fixture.expect.corpus_family_tokens,
        "corpus family",
    );
    assert_token_list(
        packet.archetype_tokens(),
        &fixture.expect.archetype_tokens,
        "archetype",
    );
    assert_token_list(
        packet.dimension_tokens(),
        &fixture.expect.dimension_tokens,
        "dimension",
    );
    assert_token_list(
        packet.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-18T00:01:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(INTEROP_CONFORMANCE_SCHEMA_REF);
    assert_exists(INTEROP_CONFORMANCE_ENVELOPE_SCHEMA_REF);
    assert_exists(INTEROP_CONFORMANCE_DOC_REF);
    assert_exists(INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF);
    for family in CorpusFamily::ALL {
        assert_exists(family.fixture_dir());
    }
}

#[test]
fn every_fixture_lives_in_its_family_dir() {
    for (dir, file_name, _) in CASES {
        let path = repo_root().join(dir).join(file_name);
        assert!(
            path.exists(),
            "fixture {file_name} must exist in {dir} ({})",
            path.display()
        );
    }
}

#[test]
fn checked_in_packet_validates_clean() {
    let path = repo_root().join(INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: InteropConformancePacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        packet.validate().is_empty(),
        "checked-in packet must validate without findings: {:?}",
        packet.validate()
    );
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn checked_in_artifacts_match_the_seed() {
    let packet = InteropConformancePacket::materialize(current_stable_interop_conformance_input());

    let packet_path = repo_root().join(INTEROP_CONFORMANCE_PACKET_ARTIFACT_REF);
    let on_disk: InteropConformancePacket =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, packet, "checked-in packet drifted from the seed");

    let support_path = packet_path.with_file_name("support_export.json");
    let support: InteropConformanceSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.packet, packet);

    let cli_path = packet_path.with_file_name("cli_headless.json");
    let cli: InteropConformanceCliHeadlessView =
        serde_json::from_str(&std::fs::read_to_string(&cli_path).expect("read cli view"))
            .expect("parse cli view");
    assert!(cli.every_corpus_runs());
    assert_eq!(cli.corpus_digest, packet.corpus_digest);
    assert_eq!(cli.case_rows.len(), packet.cases().count());

    for (file_name, surface) in [
        ("ai_evidence.json", ConformanceEvidenceSurface::AiEvidence),
        (
            "incident_packet.json",
            ConformanceEvidenceSurface::IncidentPacket,
        ),
    ] {
        let path = packet_path.with_file_name(file_name);
        let view: InteropConformanceEvidenceJoinView =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read evidence join"))
                .expect("parse evidence join");
        assert_eq!(view.surface, surface);
        assert!(
            view.explains_consistently(),
            "{file_name} must explain consistently"
        );
        assert_eq!(view.corpus_digest, packet.corpus_digest);
        assert_eq!(view.corpus_rows.len(), packet.corpora.len());
        assert_eq!(view.case_rows.len(), packet.cases().count());
        assert_eq!(view.release_evidence, packet.release_evidence);
    }
}

#[test]
fn all_fixtures_match_the_seed() {
    for (dir, file_name, _) in CASES {
        assert_fixture_matches(dir, file_name);
    }
}
