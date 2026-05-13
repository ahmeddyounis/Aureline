use std::collections::HashSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    ManagedRuntimeInspectionLabel, ManagedWorkspaceAlphaRecord, ManagedWorkspaceInspectionSurface,
    ManagedWorkspaceLifecycleState, ManagedWorkspaceSupportExport,
    MANAGED_WORKSPACE_ALPHA_RECORD_KIND, MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION,
    MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/remote/suspend_resume_reattach_alpha")
}

#[test]
fn managed_workspace_alpha_fixtures_project_runtime_inspection_truth() {
    let manifest_path = fixture_root().join("manifest.yaml");
    let manifest_payload = std::fs::read_to_string(&manifest_path).expect("manifest must read");
    let manifest: FixtureManifest =
        serde_yaml::from_str(&manifest_payload).expect("manifest must parse");
    assert_eq!(manifest.status, "protected");
    assert_eq!(manifest.schema_version, 1);

    let mut observed_states = HashSet::new();
    let mut observed_labels = HashSet::new();
    let mut records = Vec::new();

    for case_file in &manifest.case_files {
        let fixture_path = fixture_root().join(&case_file.file);
        let payload = std::fs::read_to_string(&fixture_path)
            .unwrap_or_else(|err| panic!("read {fixture_path:?}: {err}"));
        let fixture: ManagedWorkspaceFixture = serde_yaml::from_str(&payload)
            .unwrap_or_else(|err| panic!("parse {fixture_path:?}: {err}"));

        assert_eq!(fixture.record_kind, "managed_workspace_alpha_case");
        assert_eq!(fixture.schema_version, 1);
        assert_eq!(
            fixture.record.record_kind,
            MANAGED_WORKSPACE_ALPHA_RECORD_KIND
        );
        assert_eq!(
            fixture.record.schema_version,
            MANAGED_WORKSPACE_ALPHA_SCHEMA_VERSION
        );
        assert_eq!(
            fixture.record.lifecycle_state,
            fixture.expect.lifecycle_state
        );
        assert!(
            fixture.record.validation_issues().is_empty(),
            "fixture {} must not overclaim truth: {:?}",
            case_file.file,
            fixture.record.validation_issues()
        );

        let inspection = fixture.record.runtime_inspection(
            ManagedWorkspaceInspectionSurface::RuntimeInspector,
            "2026-05-13T19:30:00Z",
        );
        assert_eq!(inspection.lifecycle_state, fixture.expect.lifecycle_state);
        assert_eq!(inspection.label_tokens, fixture.expect.label_tokens);
        assert_eq!(inspection.mutation_allowed, fixture.expect.mutation_allowed);
        assert_eq!(inspection.inspect_only, fixture.expect.inspect_only);
        assert_eq!(
            inspection.reconnect_required,
            fixture.expect.reconnect_required
        );
        assert_eq!(
            inspection.requires_reapproval,
            fixture.expect.requires_reapproval
        );
        for expected in &fixture.expect.required_blocked_action_tokens {
            assert!(
                inspection.blocked_action_tokens.contains(expected),
                "missing blocked action token {expected} in {}",
                case_file.file
            );
        }

        observed_states.insert(fixture.record.lifecycle_state);
        for label in &inspection.labels {
            observed_labels.insert(*label);
        }
        records.push(fixture.record);
    }

    for required_state in [
        ManagedWorkspaceLifecycleState::Suspended,
        ManagedWorkspaceLifecycleState::Resumed,
        ManagedWorkspaceLifecycleState::Reattached,
        ManagedWorkspaceLifecycleState::ReconnectRequired,
    ] {
        assert!(
            observed_states.contains(&required_state),
            "missing lifecycle state {required_state:?}"
        );
    }

    for required_label in [
        ManagedRuntimeInspectionLabel::Local,
        ManagedRuntimeInspectionLabel::HelperBacked,
        ManagedRuntimeInspectionLabel::Resumed,
        ManagedRuntimeInspectionLabel::Stale,
        ManagedRuntimeInspectionLabel::InspectOnly,
    ] {
        assert!(
            observed_labels.contains(&required_label),
            "missing runtime label {required_label:?}"
        );
    }

    let export = ManagedWorkspaceSupportExport::from_records(
        "support:managed_workspace:alpha:01",
        "2026-05-13T19:31:00Z",
        &records,
    );
    assert_eq!(
        export.record_kind,
        MANAGED_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.inspections.len(), manifest.case_files.len());
    let rendered = export.render_plaintext();
    for record in &records {
        assert!(rendered.contains(&record.runtime_ref));
        assert!(rendered.contains(record.lifecycle_state.as_str()));
    }

    let serialized = serde_json::to_string(&export).expect("support export serializes");
    let round_trip: ManagedWorkspaceSupportExport =
        serde_json::from_str(&serialized).expect("support export deserializes");
    assert_eq!(round_trip, export);
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    status: String,
    case_files: Vec<FixtureCaseFile>,
}

#[derive(Debug, Deserialize)]
struct FixtureCaseFile {
    file: String,
}

#[derive(Debug, Deserialize)]
struct ManagedWorkspaceFixture {
    record_kind: String,
    schema_version: u32,
    record: ManagedWorkspaceAlphaRecord,
    expect: ManagedWorkspaceFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct ManagedWorkspaceFixtureExpect {
    lifecycle_state: ManagedWorkspaceLifecycleState,
    label_tokens: Vec<String>,
    mutation_allowed: bool,
    inspect_only: bool,
    reconnect_required: bool,
    requires_reapproval: bool,
    #[serde(default)]
    required_blocked_action_tokens: Vec<String>,
}
