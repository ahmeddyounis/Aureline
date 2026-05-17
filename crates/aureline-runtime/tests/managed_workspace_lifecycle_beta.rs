use std::collections::HashSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    ManagedLifecyclePhaseClass, ManagedLifecycleStateClass, ManagedLocalEditingContinuityClass,
    ManagedSurfaceClass, ManagedWorkspaceLifecycleBetaRecord,
    ManagedWorkspaceLifecycleBetaSupportExport, MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND,
    MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/runtime/m3/managed_workspace_lifecycle")
}

#[test]
fn managed_workspace_lifecycle_beta_fixtures_replay_one_truth() {
    let manifest_path = fixture_root().join("manifest.yaml");
    let manifest_payload = std::fs::read_to_string(&manifest_path).expect("manifest must read");
    let manifest: FixtureManifest =
        serde_yaml::from_str(&manifest_payload).expect("manifest must parse");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.case_refs.is_empty());

    let mut observed_phases = HashSet::new();
    let mut observed_states = HashSet::new();
    let mut observed_continuity = HashSet::new();
    let mut records: Vec<ManagedWorkspaceLifecycleBetaRecord> = Vec::new();

    for case_rel in &manifest.case_refs {
        let case_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../")
            .join(case_rel);
        let payload = std::fs::read_to_string(&case_path)
            .unwrap_or_else(|err| panic!("read {case_path:?}: {err}"));
        let fixture: FixtureRecord = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse {case_path:?}: {err}"));

        assert_eq!(
            fixture.record_kind, MANAGED_WORKSPACE_LIFECYCLE_BETA_RECORD_KIND,
            "fixture {case_rel} carries the canonical record kind"
        );
        assert_eq!(
            fixture.schema_version,
            MANAGED_WORKSPACE_LIFECYCLE_BETA_SCHEMA_VERSION
        );

        let record = ManagedWorkspaceLifecycleBetaRecord::from_lineage(
            fixture.row_id.clone(),
            fixture.workspace_ref.clone(),
            fixture.workspace_instance_ref.clone(),
            fixture.generated_at.clone(),
            fixture.local_editing_continuity,
            fixture
                .lineage
                .iter()
                .map(|entry| {
                    aureline_runtime::ManagedLifecycleLineageEntry::new(
                        entry.phase,
                        entry.state,
                        entry.reason,
                        entry.observed_at.clone(),
                        entry.summary.clone(),
                    )
                })
                .collect(),
            fixture.visible_summary.clone(),
            fixture.safe_continuation.clone(),
            fixture.source_refs.clone(),
            fixture.support_packet_refs.clone(),
            Vec::new(),
        );

        assert!(
            record.validate().is_empty(),
            "fixture {case_rel} must not overclaim truth: {:?}",
            record.validate()
        );
        assert_eq!(
            record.current_phase,
            fixture.harness_expectations.expected_current_phase
        );
        assert_eq!(
            record.current_state,
            fixture.harness_expectations.expected_current_state
        );
        assert_eq!(
            record.local_editing_continuity,
            fixture
                .harness_expectations
                .expected_local_editing_continuity
        );
        assert_eq!(
            record.mutation_allowed,
            fixture.harness_expectations.expected_mutation_allowed
        );
        assert_eq!(
            record.reconnect_required,
            fixture.harness_expectations.expected_reconnect_required
        );
        assert_eq!(
            record.lineage.len(),
            fixture.harness_expectations.expected_lineage_length
        );

        for surface in ManagedSurfaceClass::ALL {
            let projection = record.projection(surface);
            assert_eq!(projection.row_id, record.row_id);
            assert_eq!(projection.workspace_ref, record.workspace_ref);
            assert_eq!(projection.current_phase_token, record.current_phase_token);
            assert_eq!(projection.current_state_token, record.current_state_token);
            assert_eq!(projection.mutation_allowed, record.mutation_allowed);
            assert_eq!(projection.reconnect_required, record.reconnect_required);
            assert_eq!(projection.lineage_tokens.len(), record.lineage.len());
        }

        observed_phases.insert(record.current_phase);
        observed_states.insert(record.current_state);
        observed_continuity.insert(record.local_editing_continuity);
        records.push(record);
    }

    for required_phase in [
        ManagedLifecyclePhaseClass::Start,
        ManagedLifecyclePhaseClass::Ready,
        ManagedLifecyclePhaseClass::Suspend,
        ManagedLifecyclePhaseClass::Resume,
        ManagedLifecyclePhaseClass::Reconnect,
        ManagedLifecyclePhaseClass::Retire,
    ] {
        assert!(
            observed_phases.contains(&required_phase),
            "missing required lifecycle phase {required_phase:?}"
        );
    }

    for required_state in [
        ManagedLifecycleStateClass::Starting,
        ManagedLifecycleStateClass::Live,
        ManagedLifecycleStateClass::Suspended,
        ManagedLifecycleStateClass::ReconnectRequired,
        ManagedLifecycleStateClass::Retired,
    ] {
        assert!(
            observed_states.contains(&required_state),
            "missing required lifecycle state {required_state:?}"
        );
    }

    for required_continuity in [
        ManagedLocalEditingContinuityClass::PreservedFullLocalEditing,
        ManagedLocalEditingContinuityClass::PreservedLocalOnlyWrites,
        ManagedLocalEditingContinuityClass::InspectOnlyUntilRecovery,
        ManagedLocalEditingContinuityClass::NotApplicable,
    ] {
        assert!(
            observed_continuity.contains(&required_continuity),
            "missing required continuity class {required_continuity:?}"
        );
    }

    let export = ManagedWorkspaceLifecycleBetaSupportExport::from_records(
        "support_export:managed_workspace_lifecycle_beta.test",
        "2026-05-16T23:00:00Z",
        &records,
    );
    assert_eq!(export.records.len(), records.len());
    assert_eq!(export.support_projections.len(), records.len());
    assert!(export.any_record_fails_closed_for_mutation);
    let rendered = export.render_plaintext();
    for record in &records {
        assert!(rendered.contains(&record.row_id));
        assert!(rendered.contains(&record.current_state_token));
    }

    let serialized = serde_json::to_string(&export).expect("export serializes");
    let round_trip: ManagedWorkspaceLifecycleBetaSupportExport =
        serde_json::from_str(&serialized).expect("export deserializes");
    assert_eq!(round_trip, export);
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    case_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct FixtureRecord {
    record_kind: String,
    schema_version: u32,
    row_id: String,
    workspace_ref: String,
    #[serde(default)]
    workspace_instance_ref: Option<String>,
    generated_at: String,
    local_editing_continuity: ManagedLocalEditingContinuityClass,
    lineage: Vec<FixtureLineageEntry>,
    visible_summary: String,
    safe_continuation: String,
    #[serde(default)]
    source_refs: Vec<String>,
    #[serde(default)]
    support_packet_refs: Vec<String>,
    harness_expectations: FixtureHarnessExpectations,
}

#[derive(Debug, Deserialize)]
struct FixtureLineageEntry {
    phase: ManagedLifecyclePhaseClass,
    state: ManagedLifecycleStateClass,
    reason: aureline_runtime::ManagedWorkspaceTransitionReason,
    observed_at: String,
    summary: String,
}

#[derive(Debug, Deserialize)]
struct FixtureHarnessExpectations {
    expected_current_phase: ManagedLifecyclePhaseClass,
    expected_current_state: ManagedLifecycleStateClass,
    expected_local_editing_continuity: ManagedLocalEditingContinuityClass,
    expected_mutation_allowed: bool,
    expected_reconnect_required: bool,
    expected_lineage_length: usize,
}
