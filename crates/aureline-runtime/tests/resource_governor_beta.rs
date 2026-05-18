//! Fixture replay for resource-governor, queue-lane, and admission truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_resource_governor_snapshot, seeded_resource_governor_support_export,
    AdmissionDecisionClass, QueueLaneStateFlag, ResourceGovernorSnapshot,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("resource_governor_and_queue_truth")
}

#[derive(Debug, Deserialize)]
struct Manifest {
    record_kind: String,
    schema_version: u32,
    cases: Vec<String>,
    required_pressure_dimensions: Vec<String>,
    required_queue_lanes: Vec<String>,
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
    #[serde(default)]
    governor_state: Option<String>,
    #[serde(default)]
    required_pressure_dimensions: Vec<String>,
    #[serde(default)]
    required_lane_tokens: Vec<String>,
    #[serde(default)]
    required_lane_state_flags: Vec<String>,
    #[serde(default)]
    required_protected_actions: Vec<String>,
    #[serde(default)]
    required_admitted_requests: Vec<String>,
    #[serde(default)]
    required_blocked_override_ids: Vec<String>,
    #[serde(default)]
    required_checkpoint_labels: Vec<String>,
    #[serde(default)]
    required_plaintext_terms: Vec<String>,
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

fn seeded_snapshot() -> ResourceGovernorSnapshot {
    seeded_resource_governor_snapshot(
        "resource-governor:snapshot:fixture",
        "workspace.resource-governor.fixture",
        "profile.standard",
        "2026-05-18T20:00:00Z",
    )
}

#[test]
fn manifest_lists_required_pressure_dimensions_and_lanes() {
    let manifest = read_manifest();
    assert_eq!(
        manifest.record_kind,
        "resource_governor_truth_fixture_manifest"
    );
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.cases.len(), 2);
    assert_eq!(
        manifest.required_pressure_dimensions,
        vec![
            "cpu",
            "memory",
            "disk",
            "battery_thermal",
            "network",
            "optional_service_quota"
        ]
    );
    assert_eq!(
        manifest.required_queue_lanes,
        vec![
            "foreground",
            "interactive_background",
            "maintenance",
            "provider_overlay",
            "upload_replication"
        ]
    );
}

#[test]
fn protect_core_fixture_replays_through_seeded_snapshot() {
    let fixture = read_case("protect_core_pressure.json");
    assert_eq!(fixture.record_kind, "resource_governor_truth_fixture");
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(fixture.case_id, "protect_core_pressure");

    let snapshot = seeded_snapshot();
    assert_eq!(
        Some(snapshot.governor_state.as_str().to_owned()),
        fixture.expect.governor_state
    );
    assert!(snapshot.validate().is_ok());

    let pressure_dimensions = snapshot
        .pressure_inputs
        .iter()
        .map(|input| input.dimension.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    for dimension in fixture.expect.required_pressure_dimensions {
        assert!(
            pressure_dimensions.contains(&dimension),
            "missing {dimension}"
        );
    }

    let lanes = snapshot
        .lane_states
        .iter()
        .map(|lane| lane.lane.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    for lane in fixture.expect.required_lane_tokens {
        assert!(lanes.contains(&lane), "missing lane {lane}");
    }

    let flags = snapshot
        .lane_states
        .iter()
        .flat_map(|lane| lane.state_flags.iter())
        .map(|flag| flag.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    for flag in fixture.expect.required_lane_state_flags {
        assert!(flags.contains(&flag), "missing lane state {flag}");
    }

    let protected = snapshot
        .protected_actions_preserved
        .iter()
        .map(|action| action.as_str().to_owned())
        .collect::<BTreeSet<_>>();
    for action in fixture.expect.required_protected_actions {
        assert!(
            protected.contains(&action),
            "missing protected action {action}"
        );
    }

    for request_id in fixture.expect.required_admitted_requests {
        let decision = snapshot
            .admission_decisions
            .iter()
            .find(|decision| decision.request_id == request_id)
            .unwrap_or_else(|| panic!("missing admission decision {request_id}"));
        assert_eq!(decision.decision, AdmissionDecisionClass::Admit);
    }

    let blocked_overrides = snapshot
        .override_sheets
        .iter()
        .filter(|sheet| sheet.decision.is_blocked())
        .map(|sheet| sheet.override_id.clone())
        .collect::<BTreeSet<_>>();
    for override_id in fixture.expect.required_blocked_override_ids {
        assert!(
            blocked_overrides.contains(&override_id),
            "missing blocked override {override_id}"
        );
    }

    let checkpoints = snapshot
        .lane_states
        .iter()
        .filter_map(|lane| lane.checkpoint.as_ref())
        .map(|checkpoint| checkpoint.boundary_label.clone())
        .collect::<BTreeSet<_>>();
    for checkpoint in fixture.expect.required_checkpoint_labels {
        assert!(checkpoints.contains(&checkpoint), "missing {checkpoint}");
    }
}

#[test]
fn support_export_fixture_preserves_plaintext_truth() {
    let fixture = read_case("support_export_parity.json");
    let export = seeded_resource_governor_support_export(
        "resource-governor:export:fixture",
        "2026-05-18T20:01:00Z",
    );
    assert!(export.raw_private_material_excluded);
    let text = export.render_plaintext();
    for term in fixture.expect.required_plaintext_terms {
        assert!(text.contains(&term), "plaintext missing {term}");
    }
    assert!(export
        .lane_states
        .iter()
        .any(|lane| lane.state_flags.contains(&QueueLaneStateFlag::Checkpointed)));
}
