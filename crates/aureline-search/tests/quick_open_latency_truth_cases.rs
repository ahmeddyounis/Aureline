//! Fixture-driven coverage for the stable quick-open / file / symbol /
//! command-palette latency-truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_search::{
    current_stable_quick_open_latency_truth_packet, CertifiedArchetypeClass,
    LatencyConsumerSurface, LatencyFindingKind, LatencyPromotionState, LatencySurface,
    PartialIndexTruthClass, QuickOpenLatencyTruthPacket, QuickOpenLatencyTruthPacketInput,
    SessionReadinessState, QUICK_OPEN_LATENCY_TRUTH_ARTIFACT_DOC_REF,
    QUICK_OPEN_LATENCY_TRUTH_DOC_REF, QUICK_OPEN_LATENCY_TRUTH_FIXTURE_DIR,
    QUICK_OPEN_LATENCY_TRUTH_PACKET_ARTIFACT_REF, QUICK_OPEN_LATENCY_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LatencyTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: QuickOpenLatencyTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    archetype_tokens: Vec<String>,
    surface_tokens: Vec<String>,
    partial_index_truth_tokens: Vec<String>,
    visible_readiness_state_tokens: Vec<String>,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
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

fn load_fixture(file_name: &str) -> LatencyTruthFixture {
    let path = repo_root()
        .join(QUICK_OPEN_LATENCY_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "quick_open_latency_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = QuickOpenLatencyTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.archetype_tokens(),
        expect
            .archetype_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.surface_tokens(),
        expect
            .surface_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.partial_index_truth_tokens(),
        expect
            .partial_index_truth_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );
    assert_eq!(
        packet.visible_readiness_state_tokens(),
        expect
            .visible_readiness_state_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(QUICK_OPEN_LATENCY_TRUTH_SCHEMA_REF);
    assert_exists(QUICK_OPEN_LATENCY_TRUTH_DOC_REF);
    assert_exists(QUICK_OPEN_LATENCY_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(QUICK_OPEN_LATENCY_TRUTH_FIXTURE_DIR);
    assert_exists(QUICK_OPEN_LATENCY_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn budget_breach_fixture_blocks_stable() {
    assert_fixture_matches("budget_breach_blocks_stable.json");
}

#[test]
fn partial_index_unlabeled_fixture_narrows_below_stable() {
    assert_fixture_matches("partial_index_unlabeled_narrowed.json");
}

#[test]
fn session_state_collapsed_fixture_narrows_below_stable() {
    assert_fixture_matches("session_state_collapsed_narrowed.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_certifies_every_archetype_surface() {
    let packet =
        current_stable_quick_open_latency_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, LatencyPromotionState::Stable);
    assert!(packet.validate().is_empty());

    assert_eq!(packet.archetype_tokens().len(), CertifiedArchetypeClass::ALL.len());
    assert_eq!(packet.surface_tokens().len(), LatencySurface::ALL.len());
    for archetype in CertifiedArchetypeClass::ALL {
        for surface in LatencySurface::ALL {
            assert!(
                packet
                    .rows
                    .iter()
                    .any(|row| row.archetype == archetype && row.surface == surface),
                "certified archetype {} × surface {} must have a row in the stable packet",
                archetype.as_str(),
                surface.as_str()
            );
        }
    }
    for surface in LatencyConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_latency_truth_tokens_are_pinned() {
    assert_eq!(
        CertifiedArchetypeClass::COrCppNativeProject.as_str(),
        "c_or_cpp_native_project"
    );
    assert_eq!(LatencySurface::QuickOpen.as_str(), "quick_open");
    assert_eq!(
        SessionReadinessState::PolicyLimited.as_str(),
        "policy_limited"
    );
    assert_eq!(
        SessionReadinessState::ProviderLimited.as_str(),
        "provider_limited"
    );
    assert_eq!(
        PartialIndexTruthClass::IndexUnavailable.as_str(),
        "index_unavailable"
    );
    assert_eq!(
        LatencyFindingKind::SessionStateCollapsed.as_str(),
        "session_state_collapsed"
    );
}
