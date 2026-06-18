//! Fixture-driven coverage for the M5 replay bundle: the normalized history
//! joined to the typed raw-payload lineage, the redaction-honoring support /
//! incident / AI evidence joins, and the replay robustness drills that keep the
//! dual-retention history stable under truncation, duplicate delivery, adapter
//! drift, and export/import round-trip.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_replay_bundle_input, BuildTestInteropSourceKind, RawPayloadRetentionClass,
    ReplayBundle, ReplayBundleInput, ReplayBundleSupportExport, ReplayEvidenceJoinView,
    ReplayJoinSurface, REPLAY_BUNDLE_DOC_REF, REPLAY_BUNDLE_ENVELOPE_SCHEMA_REF,
    REPLAY_BUNDLE_FIXTURE_DIR, REPLAY_BUNDLE_PACKET_ARTIFACT_REF, REPLAY_BUNDLE_SCHEMA_REF,
};
use serde::Deserialize;

const METADATA_REF: &str = "raw:event:task:queued";

#[derive(Debug, Deserialize)]
struct BundleFixture {
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
    surface_tokens: Vec<String>,
    retention_class_tokens: Vec<String>,
    source_kind_tokens: Vec<String>,
    failure_mode_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> BundleFixture {
    let path = repo_root().join(REPLAY_BUNDLE_FIXTURE_DIR).join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_replay_bundles` regenerator, so
/// the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> ReplayBundleInput {
    let mut input = current_stable_replay_bundle_input();
    match mutation {
        "none" => {}
        "lineage_entry_missing" => {
            input
                .raw_lineage
                .retain(|entry| entry.raw_payload_ref != METADATA_REF);
        }
        "raw_payload_unbounded" => {
            for entry in &mut input.raw_lineage {
                if entry.raw_payload_ref == METADATA_REF {
                    entry.payload_byte_len = 1_000_000;
                }
            }
        }
        "retention_exposes_secret" => {
            for entry in &mut input.raw_lineage {
                if entry.retention_class == RawPayloadRetentionClass::SupportApprovalRequired {
                    entry.support_export_safe = true;
                    entry.ai_evidence_safe = true;
                }
            }
        }
        "lineage_source_mismatch" => {
            for entry in &mut input.raw_lineage {
                if entry.raw_payload_ref == METADATA_REF {
                    entry.source_kind = BuildTestInteropSourceKind::Bsp;
                }
            }
        }
        "join_missing" => {
            input
                .join_projections
                .retain(|projection| projection.surface != ReplayJoinSurface::IncidentPacket);
        }
        "join_drops_redaction" => {
            for projection in &mut input.join_projections {
                if projection.surface == ReplayJoinSurface::AiEvidence {
                    projection.honors_retention_redaction = false;
                }
            }
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
        fixture.record_kind, "m5_replay_bundle_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let bundle = ReplayBundle::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        bundle.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        bundle.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        bundle
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = bundle
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
        bundle.surface_tokens(),
        &fixture.expect.surface_tokens,
        "surface",
    );
    assert_token_list(
        bundle.retention_class_tokens(),
        &fixture.expect.retention_class_tokens,
        "retention class",
    );
    assert_token_list(
        bundle.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_list(
        bundle.failure_mode_tokens(),
        &fixture.expect.failure_mode_tokens,
        "failure mode",
    );

    let export = bundle.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-17T00:01:00Z",
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
    assert_exists(REPLAY_BUNDLE_SCHEMA_REF);
    assert_exists(REPLAY_BUNDLE_ENVELOPE_SCHEMA_REF);
    assert_exists(REPLAY_BUNDLE_DOC_REF);
    assert_exists(REPLAY_BUNDLE_FIXTURE_DIR);
    assert_exists(REPLAY_BUNDLE_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_bundle_validates_clean() {
    let path = repo_root().join(REPLAY_BUNDLE_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("bundle artifact {path:?} must read: {err}"));
    let bundle: ReplayBundle = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("bundle artifact {path:?} must parse: {err}"));
    assert!(
        bundle.validate().is_empty(),
        "checked-in bundle must validate without findings: {:?}",
        bundle.validate()
    );
    assert_eq!(bundle.promotion_state.as_str(), "stable");
}

#[test]
fn checked_in_artifacts_match_the_seed() {
    let bundle = ReplayBundle::materialize(current_stable_replay_bundle_input());

    let packet_path = repo_root().join(REPLAY_BUNDLE_PACKET_ARTIFACT_REF);
    let on_disk: ReplayBundle =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, bundle, "checked-in packet drifted from the seed");

    let support_path = repo_root()
        .join(REPLAY_BUNDLE_PACKET_ARTIFACT_REF)
        .with_file_name("support_export.json");
    let support: ReplayBundleSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.bundle, bundle);

    for (file_name, surface) in [
        ("ai_evidence.json", ReplayJoinSurface::AiEvidence),
        ("incident_packet.json", ReplayJoinSurface::IncidentPacket),
    ] {
        let path = packet_path.with_file_name(file_name);
        let view: ReplayEvidenceJoinView =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read evidence join"))
                .expect("parse evidence join");
        assert_eq!(view.surface, surface);
        assert!(view.honors_redaction(), "{file_name} must honor redaction");
        // Each export surface gates exactly the approval-only payload.
        assert_eq!(view.gated_payload_count, 1, "{file_name} gates one payload");
        assert_eq!(view.replay_digest, bundle.replay_digest);
    }
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn lineage_entry_missing_blocks_stable() {
    assert_fixture_matches("lineage_entry_missing_blocks_stable.json");
}

#[test]
fn raw_payload_unbounded_blocks_stable() {
    assert_fixture_matches("raw_payload_unbounded_blocks_stable.json");
}

#[test]
fn retention_exposes_secret_blocks_stable() {
    assert_fixture_matches("retention_exposes_secret_blocks_stable.json");
}

#[test]
fn lineage_source_mismatch_blocks_stable() {
    assert_fixture_matches("lineage_source_mismatch_blocks_stable.json");
}

#[test]
fn join_missing_blocks_stable() {
    assert_fixture_matches("join_missing_blocks_stable.json");
}

#[test]
fn join_drops_redaction_blocks_stable() {
    assert_fixture_matches("join_drops_redaction_blocks_stable.json");
}
