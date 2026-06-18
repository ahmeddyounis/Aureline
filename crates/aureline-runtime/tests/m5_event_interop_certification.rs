//! Fixture-driven coverage for the M5 event-interop tooling-profile certification
//! matrix: every claimed M5 run/test/debug/pipeline/notebook/coverage profile,
//! the eight graded certification dimensions, the freshness/stale narrowing
//! roll-up, the certification index, and the fail-closed guardrails against
//! private session histories, missing evidence, overclaimed confidence, lost raw
//! payloads, missing fallback reasons, hidden degraded states, broken replay, and
//! broken export parity.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_event_interop_certification_input, BuildTestInteropConfidence,
    CertificationEvidenceSurface, ConsumerTruthSource, EventInteropCertificationCliHeadlessView,
    EventInteropCertificationEvidenceJoinView, EventInteropCertificationPacket,
    EventInteropCertificationPacketInput, EventInteropCertificationSupportExport, ToolingProfile,
    ToolingProfileCertification, EVENT_INTEROP_CERTIFICATION_CONFORMANCE_PACKET_REF,
    EVENT_INTEROP_CERTIFICATION_DOC_REF, EVENT_INTEROP_CERTIFICATION_ENVELOPE_SCHEMA_REF,
    EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS, EVENT_INTEROP_CERTIFICATION_INTEROP_PACKET_REF,
    EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF,
    EVENT_INTEROP_CERTIFICATION_POLICY_BASELINE_REF, EVENT_INTEROP_CERTIFICATION_SCHEMA_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/tooling/m5/event-interop-certification";

/// Each fixture is (file-name, mutation).
const CASES: [(&str, &str); 12] = [
    ("baseline_stable.json", "none"),
    (
        "private_session_history_blocks_stable.json",
        "private_session_history",
    ),
    (
        "missing_evidence_ref_blocks_stable.json",
        "missing_evidence_ref",
    ),
    (
        "adapter_hierarchy_missing_blocks_stable.json",
        "adapter_hierarchy_missing",
    ),
    (
        "confidence_overclaim_blocks_stable.json",
        "confidence_overclaim",
    ),
    (
        "fallback_reason_missing_blocks_stable.json",
        "fallback_reason_missing",
    ),
    (
        "raw_payload_not_retained_blocks_stable.json",
        "raw_payload_not_retained",
    ),
    ("replay_unstable_blocks_stable.json", "replay_unstable"),
    (
        "export_parity_broken_blocks_stable.json",
        "export_parity_broken",
    ),
    (
        "degraded_state_not_disclosed_blocks_stable.json",
        "degraded_state_not_disclosed",
    ),
    ("missing_profile_blocks_stable.json", "missing_profile"),
    ("evidence_stale_narrows_below_stable.json", "evidence_stale"),
];

#[derive(Debug, Deserialize)]
struct CertificationFixture {
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
    profile_tokens: Vec<String>,
    source_kind_tokens: Vec<String>,
    consumer_truth_source_tokens: Vec<String>,
    dimension_tokens: Vec<String>,
    claimable_profiles: Vec<String>,
    narrowed_profiles: Vec<String>,
    blocked_profiles: Vec<String>,
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

fn load_fixture(file_name: &str) -> CertificationFixture {
    let path = repo_root().join(FIXTURE_DIR).join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn profile(
    input: &mut EventInteropCertificationPacketInput,
    profile: ToolingProfile,
) -> &mut ToolingProfileCertification {
    input
        .profiles
        .iter_mut()
        .find(|row| row.profile == profile)
        .expect("profile present")
}

/// Mirrors the mutations applied by the `dump_m5_event_interop_certification`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> EventInteropCertificationPacketInput {
    let mut input = current_stable_event_interop_certification_input();
    match mutation {
        "none" => {}
        "private_session_history" => {
            profile(&mut input, ToolingProfile::TaskCenterRun).consumer_truth_source =
                ConsumerTruthSource::PrivateSessionHistory;
        }
        "missing_evidence_ref" => {
            profile(&mut input, ToolingProfile::TestSession).evidence_refs = Vec::new();
        }
        "adapter_hierarchy_missing" => {
            profile(&mut input, ToolingProfile::DebugSession).capability_packet_ref = String::new();
        }
        "confidence_overclaim" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).observed_confidence =
                BuildTestInteropConfidence::High;
        }
        "fallback_reason_missing" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).fallback_reason = None;
        }
        "raw_payload_not_retained" => {
            profile(&mut input, ToolingProfile::CoverageIntelligence)
                .raw_private_material_excluded = false;
        }
        "replay_unstable" => {
            profile(&mut input, ToolingProfile::NotebookRun).replay_stable = false;
        }
        "export_parity_broken" => {
            profile(&mut input, ToolingProfile::TaskCenterRun).export_parity_preserved = false;
        }
        "degraded_state_not_disclosed" => {
            profile(&mut input, ToolingProfile::PipelineOverlay).degraded_state_disclosed = false;
        }
        "missing_profile" => {
            input
                .profiles
                .retain(|row| row.profile != ToolingProfile::CoverageIntelligence);
        }
        "evidence_stale" => {
            let row = profile(&mut input, ToolingProfile::CoverageIntelligence);
            row.proof_age_days = row.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_list(observed: Vec<&str>, expected: &[String], label: &str) {
    let observed: Vec<String> = observed.into_iter().map(str::to_owned).collect();
    assert_eq!(&observed, expected, "{label} token drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_event_interop_certification_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet = EventInteropCertificationPacket::materialize(mutated_input(&fixture.mutation));
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
        packet.profile_tokens(),
        &fixture.expect.profile_tokens,
        "profile",
    );
    assert_token_list(
        packet.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_list(
        packet.consumer_truth_source_tokens(),
        &fixture.expect.consumer_truth_source_tokens,
        "consumer truth source",
    );
    assert_token_list(
        packet.dimension_tokens(),
        &fixture.expect.dimension_tokens,
        "dimension",
    );
    assert_eq!(
        packet.certification_index.claimable_profiles, fixture.expect.claimable_profiles,
        "fixture {} claimable drift",
        fixture.case_name
    );
    assert_eq!(
        packet.certification_index.narrowed_profiles, fixture.expect.narrowed_profiles,
        "fixture {} narrowed drift",
        fixture.case_name
    );
    assert_eq!(
        packet.certification_index.blocked_profiles, fixture.expect.blocked_profiles,
        "fixture {} blocked drift",
        fixture.case_name
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
    assert_exists(EVENT_INTEROP_CERTIFICATION_SCHEMA_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_ENVELOPE_SCHEMA_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_DOC_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_POLICY_BASELINE_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_INTEROP_PACKET_REF);
    assert_exists(EVENT_INTEROP_CERTIFICATION_CONFORMANCE_PACKET_REF);
    for reference in EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS {
        assert_exists(reference);
    }
    assert_exists(FIXTURE_DIR);
}

#[test]
fn every_fixture_lives_in_its_dir() {
    for (file_name, _) in CASES {
        let path = repo_root().join(FIXTURE_DIR).join(file_name);
        assert!(
            path.exists(),
            "fixture {file_name} must exist in {FIXTURE_DIR} ({})",
            path.display()
        );
    }
}

#[test]
fn checked_in_packet_validates_clean() {
    let path = repo_root().join(EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: EventInteropCertificationPacket = serde_json::from_str(&payload)
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
    let packet = EventInteropCertificationPacket::materialize(
        current_stable_event_interop_certification_input(),
    );

    let packet_path = repo_root().join(EVENT_INTEROP_CERTIFICATION_PACKET_ARTIFACT_REF);
    let on_disk: EventInteropCertificationPacket =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, packet, "checked-in packet drifted from the seed");

    let support_path = packet_path.with_file_name("support_export.json");
    let support: EventInteropCertificationSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.packet, packet);

    let cli_path = packet_path.with_file_name("cli_headless.json");
    let cli: EventInteropCertificationCliHeadlessView =
        serde_json::from_str(&std::fs::read_to_string(&cli_path).expect("read cli view"))
            .expect("parse cli view");
    assert!(cli.every_profile_explained());
    assert_eq!(cli.profile_digest, packet.profile_digest);
    assert_eq!(cli.profile_rows.len(), packet.profiles.len());

    for (file_name, surface) in [
        ("ai_evidence.json", CertificationEvidenceSurface::AiEvidence),
        (
            "incident_packet.json",
            CertificationEvidenceSurface::IncidentPacket,
        ),
    ] {
        let path = packet_path.with_file_name(file_name);
        let view: EventInteropCertificationEvidenceJoinView =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read evidence join"))
                .expect("parse evidence join");
        assert_eq!(view.surface, surface);
        assert!(
            view.explains_consistently(),
            "{file_name} must explain consistently"
        );
        assert_eq!(view.profile_digest, packet.profile_digest);
        assert_eq!(view.profile_rows.len(), packet.profiles.len());
        assert_eq!(view.certification_index, packet.certification_index);
    }
}

#[test]
fn all_fixtures_match_the_seed() {
    for (file_name, _) in CASES {
        assert_fixture_matches(file_name);
    }
}
