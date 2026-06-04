//! Fixture replay for stable queue-governor and admission-control truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_queue_governor_packet, QueueGovernorStablePacket, StalenessInputs,
    QUEUE_GOVERNOR_ARTIFACT_DOC_REF, QUEUE_GOVERNOR_DOC_REF, QUEUE_GOVERNOR_FIXTURE_DIR,
    QUEUE_GOVERNOR_PACKET_ARTIFACT_REF, QUEUE_GOVERNOR_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    cases: Vec<String>,
    required_labs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    lab_id: String,
    expected_state: String,
    pressure_dimension: String,
    affected_lane: String,
    protected_action: String,
    required_plaintext_terms: Vec<String>,
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

fn fixture_root() -> PathBuf {
    repo_root().join(QUEUE_GOVERNOR_FIXTURE_DIR)
}

fn read_manifest() -> Manifest {
    let path = fixture_root().join("manifest.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn read_case(name: &str) -> CaseFixture {
    let path = fixture_root().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(QUEUE_GOVERNOR_SCHEMA_REF);
    assert_exists(QUEUE_GOVERNOR_DOC_REF);
    assert_exists(QUEUE_GOVERNOR_ARTIFACT_DOC_REF);
    assert_exists(QUEUE_GOVERNOR_FIXTURE_DIR);
    assert_exists(QUEUE_GOVERNOR_PACKET_ARTIFACT_REF);
}

#[test]
fn stable_packet_validates_scheduler_truth_invariants() {
    let packet = current_stable_queue_governor_packet();
    assert!(
        packet.validate().is_ok(),
        "stable packet findings: {:?}",
        packet.validate().err()
    );
    assert_eq!(packet.lane_rules.len(), 5);
    assert_eq!(packet.lane_summaries.len(), 5);
    assert_eq!(packet.labs.len(), 6);

    let support = packet.support_export("support:queue-governor:fixture");
    assert!(support.raw_private_material_excluded);
    let plaintext = support.render_plaintext();
    assert!(plaintext.contains("Queue governor state: Protect core"));
    assert!(plaintext.contains("Provider overlay"));
    assert!(plaintext.contains("Upload / replication"));
}

#[test]
fn queued_jobs_self_invalidate_on_revision_manifest_context_or_policy_epoch() {
    let packet = current_stable_queue_governor_packet();
    let job = packet
        .jobs
        .first()
        .expect("stable packet includes at least one job");
    let matching = StalenessInputs {
        workspace_revision: "rev:42".to_owned(),
        manifest_hash: "manifest:hotset:v7".to_owned(),
        execution_context_hash: "ctx:stable:v3".to_owned(),
        policy_epoch: "policy:2026-06-04".to_owned(),
    };
    assert!(!job.is_stale_against(&matching));

    let changed_policy = StalenessInputs {
        policy_epoch: "policy:2026-06-05".to_owned(),
        ..matching
    };
    assert!(job.is_stale_against(&changed_policy));
}

#[test]
fn fixtures_replay_required_pressure_labs() {
    let manifest = read_manifest();
    assert_eq!(manifest.record_kind, "queue_governor_lab_manifest");
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.cases.len(), 6);

    let packet = current_stable_queue_governor_packet();
    let lab_ids = packet
        .labs
        .iter()
        .map(|lab| lab.lab_id.as_str())
        .collect::<BTreeSet<_>>();
    for lab_id in &manifest.required_labs {
        assert!(lab_ids.contains(lab_id.as_str()), "missing lab {lab_id}");
    }

    for case_name in manifest.cases {
        let fixture = read_case(&case_name);
        assert_eq!(fixture.record_kind, "queue_governor_lab_case");
        assert_eq!(fixture.schema_version, 1);
        assert_eq!(fixture.case_id, fixture.expect.lab_id);

        let lab = packet
            .labs
            .iter()
            .find(|lab| lab.lab_id == fixture.expect.lab_id)
            .unwrap_or_else(|| panic!("missing lab {}", fixture.expect.lab_id));
        assert_eq!(lab.expected_state.as_str(), fixture.expect.expected_state);
        assert_eq!(
            lab.pressure_dimension.as_str(),
            fixture.expect.pressure_dimension
        );
        assert_eq!(lab.affected_lane.as_str(), fixture.expect.affected_lane);
        assert_eq!(
            lab.protected_action.as_str(),
            fixture.expect.protected_action
        );

        let plaintext = packet
            .support_export(format!("support:{}", fixture.case_id))
            .render_plaintext();
        for term in fixture.expect.required_plaintext_terms {
            assert!(plaintext.contains(&term), "plaintext missing {term}");
        }
    }
}

#[test]
fn artifact_packet_round_trips_as_stable_packet() {
    let path = repo_root().join(QUEUE_GOVERNOR_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read artifact {}: {err}", path.display()));
    let packet: QueueGovernorStablePacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse artifact {}: {err}", path.display()));
    assert!(
        packet.validate().is_ok(),
        "artifact packet findings: {:?}",
        packet.validate().err()
    );
}
