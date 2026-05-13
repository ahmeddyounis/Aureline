//! Fixture-driven coverage for saved named workset/sparse-scope identity.

use std::collections::BTreeSet;
use std::path::Path;

use serde_json::Value;

use aureline_workspace::{
    ScopeDegradedReason, ScopeMode, ScopeReopenPosture, ScopeReopenState, WorksetArtifactRecord,
    WorksetScopeConsumerBinding, WorksetScopeConsumerClass,
};

fn fixture_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/workset_scope_alpha")
}

fn load_artifact() -> WorksetArtifactRecord {
    let path = fixture_dir().join("aureline.workset.jsonc");
    let payload = std::fs::read_to_string(&path).expect("saved workset artifact must read");
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("saved workset artifact {path:?} must parse: {err}"))
}

#[test]
fn saved_scope_artifact_preserves_stable_identity_and_root_truth() {
    let artifact = load_artifact();
    artifact
        .validate()
        .expect("saved named sparse scope must validate");

    assert_eq!(
        artifact.stable_scope_id(),
        "scope:aureline:alpha:named_sparse"
    );
    assert_eq!(artifact.scope_mode(), ScopeMode::Sparse);
    assert_eq!(artifact.included_roots().len(), artifact.root_refs.len());
    assert_eq!(
        artifact.manifest_source_ref.as_deref(),
        Some("fixtures/workspace/workset_scope_alpha/aureline.workset.jsonc")
    );

    let root_kinds: BTreeSet<&str> = artifact
        .included_roots()
        .iter()
        .map(|root| root.root_kind.as_str())
        .collect();
    for expected in [
        "local_repo_root",
        "remote_repository",
        "container_root",
        "managed_cloud_root",
    ] {
        assert!(
            root_kinds.contains(expected),
            "missing root kind {expected}"
        );
    }

    let result_states: BTreeSet<&str> = artifact
        .included_roots()
        .iter()
        .map(|root| root.partial_truth.as_str())
        .collect();
    for expected in ["loaded", "manifest_known", "cached", "unavailable"] {
        assert!(
            result_states.contains(expected),
            "missing result state {expected}"
        );
    }
}

#[test]
fn workspace_manifest_points_to_reviewable_saved_scope_artifact() {
    let artifact = load_artifact();
    let path = fixture_dir().join("workspace_manifest.json");
    let payload = std::fs::read_to_string(&path).expect("workspace manifest must read");
    let manifest: Value = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("workspace manifest {path:?} must parse: {err}"));

    let saved_workset_refs = manifest["saved_workset_refs"]
        .as_array()
        .expect("saved_workset_refs must be an array");
    assert!(saved_workset_refs
        .iter()
        .any(|value| value.as_str() == Some(&artifact.workset_id)));

    let saved_artifact = manifest["saved_workset_artifacts"]
        .as_array()
        .expect("saved_workset_artifacts must be an array")
        .iter()
        .find(|row| row["workset_ref"].as_str() == Some(&artifact.workset_id))
        .expect("manifest must point at the saved workset artifact");
    assert_eq!(
        saved_artifact["scope_ref"].as_str(),
        Some(artifact.stable_scope_id())
    );
    assert_eq!(
        saved_artifact["artifact_ref"].as_str(),
        artifact.manifest_source_ref.as_deref()
    );
}

#[test]
fn consumer_bindings_preserve_identity_and_explain_degraded_restore() {
    let artifact = load_artifact();
    let bindings = [
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::LocalUi,
            ScopeReopenPosture::Exact,
            "mono:alpha:local",
        ),
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::RemoteUi,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::RemoteUnavailable),
            "mono:alpha:remote",
        ),
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::Headless,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::RebindingRequired),
            "mono:alpha:headless",
        ),
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::SupportExport,
            ScopeReopenPosture::Exact,
            "mono:alpha:support",
        ),
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::Navigation,
            ScopeReopenPosture::Exact,
            "mono:alpha:navigation",
        ),
        artifact.project_consumer_binding(
            WorksetScopeConsumerClass::RefactorScope,
            ScopeReopenPosture::Degraded(ScopeDegradedReason::ManagedProviderUnavailable),
            "mono:alpha:refactor",
        ),
    ];

    for binding in bindings {
        assert_eq!(
            binding.record_kind,
            WorksetScopeConsumerBinding::RECORD_KIND
        );
        assert_eq!(
            binding.schema_version,
            WorksetScopeConsumerBinding::SCHEMA_VERSION
        );
        assert_eq!(binding.stable_scope_id, artifact.stable_scope_id());
        assert_eq!(binding.workset_ref, artifact.workset_id);
        assert_eq!(binding.scope_mode, artifact.scope_mode());
        assert_eq!(binding.included_roots, artifact.included_roots);
        assert_eq!(
            binding.reviewable_artifact_ref.as_deref(),
            artifact.manifest_source_ref.as_deref()
        );
        match binding.reopen_state {
            ScopeReopenState::Exact => assert!(binding.degraded_reason.is_none()),
            ScopeReopenState::Degraded => assert!(
                binding.degraded_reason.is_some(),
                "degraded consumers must include a reason"
            ),
        }

        let json = serde_json::to_string(&binding).expect("binding must serialize");
        let parsed: WorksetScopeConsumerBinding =
            serde_json::from_str(&json).expect("binding must round-trip");
        assert_eq!(parsed, binding);
    }
}
