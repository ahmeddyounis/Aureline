//! Fixture-driven coverage for the M3 workset switcher beta lane.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::Value;

use aureline_workspace::{
    project_switcher_record, ScopeDegradedReason, ScopeDriftClass, ScopeReopenPosture,
    ScopeReopenState, SwitcherRowAction, WorksetActivationPreview, WorksetArtifactRecord,
    WorksetPortabilityLabel, WorksetReopenParityPacket, WorksetScopeConsumerClass,
    WorksetSwitcherBetaRecord, WorksetSwitcherBetaSupportExport,
};

#[derive(Debug, Clone, Deserialize)]
struct Fixture {
    #[serde(rename = "__fixture__")]
    meta: FixtureMeta,
    workspace: WorkspaceBlock,
    artifacts: Vec<WorksetArtifactRecord>,
    active_workset_ref: String,
    #[serde(default)]
    activation_preview: Option<ActivationPreviewSpec>,
    reopen_parity: ReopenParitySpec,
    expect: ExpectBlock,
}

#[derive(Debug, Clone, Deserialize)]
struct FixtureMeta {
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
struct WorkspaceBlock {
    workspace_ref: String,
    switcher_id: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ActivationPreviewSpec {
    base_workset_ref: String,
    candidate_workset_ref: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ReopenParitySpec {
    active_workset_ref: String,
    local_ui: PostureSpec,
    remote_ui: PostureSpec,
    headless: PostureSpec,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum PostureSpec {
    Exact(String),
    Degraded { degraded: String },
}

#[derive(Debug, Clone, Deserialize)]
struct ExpectBlock {
    row_count: usize,
    active_workset_ref: String,
    active_row_portability_label: String,
    #[serde(default)]
    active_row_repo_count: Option<u32>,
    #[serde(default)]
    active_row_root_taxonomy_count: Option<u32>,
    active_row_offers_export: bool,
    active_row_offers_preview: bool,
    #[serde(default)]
    candidate_row_portability_label: Option<String>,
    #[serde(default)]
    candidate_row_offers_preview: Option<bool>,
    #[serde(default)]
    active_row_has_policy_overlay: Option<bool>,
    #[serde(default)]
    preview_scope_drift: Option<String>,
    #[serde(default)]
    preview_root_additions: Option<u32>,
    #[serde(default)]
    preview_root_removals: Option<u32>,
    reopen_identity_preserved: bool,
    reopen_exact_consumers: Vec<String>,
    reopen_degraded_count: usize,
    #[serde(default)]
    reopen_degraded_consumer: Option<String>,
    #[serde(default)]
    reopen_degraded_reason: Option<String>,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m3/workset_switcher")
}

fn load_fixtures() -> Vec<(PathBuf, Fixture)> {
    let dir = fixtures_dir();
    let mut paths: Vec<_> = std::fs::read_dir(&dir)
        .expect("switcher-beta fixtures dir must exist")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|ext| ext == "json"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let payload = std::fs::read_to_string(&path).expect("fixture must read");
            let parsed: Fixture = serde_json::from_str(&payload)
                .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"));
            (path, parsed)
        })
        .collect()
}

fn portability_token(label: WorksetPortabilityLabel) -> &'static str {
    label.as_str()
}

fn drift_token(class: ScopeDriftClass) -> &'static str {
    class.as_str()
}

fn consumer_from_token(token: &str) -> WorksetScopeConsumerClass {
    match token {
        "local_ui" => WorksetScopeConsumerClass::LocalUi,
        "remote_ui" => WorksetScopeConsumerClass::RemoteUi,
        "headless" => WorksetScopeConsumerClass::Headless,
        "support_export" => WorksetScopeConsumerClass::SupportExport,
        "navigation" => WorksetScopeConsumerClass::Navigation,
        "refactor_scope" => WorksetScopeConsumerClass::RefactorScope,
        other => panic!("unknown consumer class token: {other}"),
    }
}

fn degraded_reason_from_token(token: &str) -> ScopeDegradedReason {
    match token {
        "missing_root" => ScopeDegradedReason::MissingRoot,
        "root_kind_unsupported" => ScopeDegradedReason::RootKindUnsupported,
        "rebinding_required" => ScopeDegradedReason::RebindingRequired,
        "remote_unavailable" => ScopeDegradedReason::RemoteUnavailable,
        "managed_provider_unavailable" => ScopeDegradedReason::ManagedProviderUnavailable,
        "manifest_unavailable" => ScopeDegradedReason::ManifestUnavailable,
        "policy_limited" => ScopeDegradedReason::PolicyLimited,
        other => panic!("unknown degraded reason token: {other}"),
    }
}

fn posture(spec: &PostureSpec) -> ScopeReopenPosture {
    match spec {
        PostureSpec::Exact(token) => {
            assert_eq!(token, "exact", "exact posture token must read 'exact'");
            ScopeReopenPosture::Exact
        }
        PostureSpec::Degraded { degraded } => {
            ScopeReopenPosture::Degraded(degraded_reason_from_token(degraded))
        }
    }
}

fn build_switcher(fixture: &Fixture) -> WorksetSwitcherBetaRecord {
    project_switcher_record(
        fixture.workspace.switcher_id.clone(),
        fixture.workspace.workspace_ref.clone(),
        &fixture.active_workset_ref,
        &fixture.artifacts,
        "mono:beta:test",
    )
}

fn build_parity(fixture: &Fixture) -> WorksetReopenParityPacket {
    let active = fixture
        .artifacts
        .iter()
        .find(|a| a.workset_id == fixture.reopen_parity.active_workset_ref)
        .expect("active workset must exist for parity packet");
    active.project_reopen_parity_packet(
        format!("parity:{}", fixture.meta.name),
        posture(&fixture.reopen_parity.local_ui),
        posture(&fixture.reopen_parity.remote_ui),
        posture(&fixture.reopen_parity.headless),
        "mono:beta:test",
    )
}

fn build_preview(fixture: &Fixture) -> Option<WorksetActivationPreview> {
    let spec = fixture.activation_preview.as_ref()?;
    let base = fixture
        .artifacts
        .iter()
        .find(|a| a.workset_id == spec.base_workset_ref)?;
    let candidate = fixture
        .artifacts
        .iter()
        .find(|a| a.workset_id == spec.candidate_workset_ref)?;
    Some(base.project_activation_preview(
        candidate,
        format!("preview:{}", fixture.meta.name),
        format!("diff:{}", fixture.meta.name),
        "mono:beta:test",
    ))
}

#[test]
fn every_fixture_validates_switcher_preview_and_parity() {
    for (path, fixture) in load_fixtures() {
        for artifact in &fixture.artifacts {
            artifact
                .validate()
                .unwrap_or_else(|err| panic!("artifact in {path:?} must validate: {err}"));
        }
        let switcher = build_switcher(&fixture);
        switcher
            .validate()
            .unwrap_or_else(|err| panic!("switcher in {path:?} must validate: {err}"));
        assert_eq!(switcher.rows.len(), fixture.expect.row_count);
        assert_eq!(
            switcher.active_workset_ref,
            fixture.expect.active_workset_ref
        );

        let active_row = switcher.active_row().expect("active row required");
        assert_eq!(
            portability_token(active_row.portability_label),
            fixture.expect.active_row_portability_label.as_str(),
            "active portability_label mismatch in {path:?}",
        );
        if let Some(expected_repos) = fixture.expect.active_row_repo_count {
            assert_eq!(active_row.repo_count, expected_repos);
        }
        if let Some(expected_taxonomy) = fixture.expect.active_row_root_taxonomy_count {
            assert_eq!(active_row.root_taxonomy.len() as u32, expected_taxonomy);
        }
        assert_eq!(
            active_row
                .offered_actions
                .contains(&SwitcherRowAction::ExportWorksetArtifact),
            fixture.expect.active_row_offers_export,
            "active export advertisement mismatch in {path:?}",
        );
        assert_eq!(
            active_row
                .offered_actions
                .contains(&SwitcherRowAction::PreviewActivationDiff),
            fixture.expect.active_row_offers_preview,
            "active preview advertisement mismatch in {path:?}",
        );
        if let Some(expected_overlay) = fixture.expect.active_row_has_policy_overlay {
            assert_eq!(active_row.policy_overlay.is_some(), expected_overlay);
        }
        if let Some(expected_candidate_label) =
            fixture.expect.candidate_row_portability_label.as_deref()
        {
            let candidate = switcher
                .rows
                .iter()
                .find(|row| !row.is_active)
                .expect("candidate row required");
            assert_eq!(
                portability_token(candidate.portability_label),
                expected_candidate_label
            );
            if let Some(expected_preview) = fixture.expect.candidate_row_offers_preview {
                assert_eq!(
                    candidate
                        .offered_actions
                        .contains(&SwitcherRowAction::PreviewActivationDiff),
                    expected_preview,
                );
            }
        }

        if let Some(preview) = build_preview(&fixture) {
            preview
                .validate()
                .unwrap_or_else(|err| panic!("preview in {path:?} must validate: {err}"));
            if let Some(expected_drift) = fixture.expect.preview_scope_drift.as_deref() {
                assert_eq!(
                    drift_token(preview.scope_drift),
                    expected_drift,
                    "preview scope_drift mismatch in {path:?}",
                );
            }
            if let Some(expected_additions) = fixture.expect.preview_root_additions {
                assert_eq!(
                    preview.root_additions.len() as u32,
                    expected_additions,
                    "preview root_additions mismatch in {path:?}",
                );
            }
            if let Some(expected_removals) = fixture.expect.preview_root_removals {
                assert_eq!(
                    preview.root_removals.len() as u32,
                    expected_removals,
                    "preview root_removals mismatch in {path:?}",
                );
            }
        }

        let parity = build_parity(&fixture);
        parity
            .validate()
            .unwrap_or_else(|err| panic!("parity in {path:?} must validate: {err}"));
        assert_eq!(
            parity.identity_preserved_across_consumers, fixture.expect.reopen_identity_preserved,
            "parity identity preservation mismatch in {path:?}",
        );
        let exact_tokens: Vec<String> = parity
            .exact_consumer_classes
            .iter()
            .map(|c| c.as_str().to_string())
            .collect();
        assert_eq!(
            exact_tokens, fixture.expect.reopen_exact_consumers,
            "parity exact consumer list mismatch in {path:?}",
        );
        assert_eq!(
            parity.degraded.len(),
            fixture.expect.reopen_degraded_count,
            "parity degraded row count mismatch in {path:?}",
        );
        if let Some(expected_consumer) = fixture.expect.reopen_degraded_consumer.as_deref() {
            let consumer = consumer_from_token(expected_consumer);
            let entry = parity
                .degraded
                .iter()
                .find(|d| d.consumer_class == consumer)
                .expect("degraded entry required");
            if let Some(expected_reason) = fixture.expect.reopen_degraded_reason.as_deref() {
                assert_eq!(
                    entry.reason,
                    degraded_reason_from_token(expected_reason),
                    "parity degraded reason mismatch in {path:?}",
                );
            }
        }

        // Identity contract: every binding shares the same stable_scope_id.
        let scope_ids: std::collections::BTreeSet<_> = parity
            .bindings
            .iter()
            .map(|b| b.stable_scope_id.clone())
            .collect();
        assert_eq!(scope_ids.len(), 1, "parity scope_id drift in {path:?}");
    }
}

#[test]
fn every_fixture_round_trips_support_export_bundle() {
    for (path, fixture) in load_fixtures() {
        let switcher = build_switcher(&fixture);
        let parity = build_parity(&fixture);
        let mut previews: Vec<WorksetActivationPreview> = Vec::new();
        if let Some(preview) = build_preview(&fixture) {
            previews.push(preview);
        }
        let bundle = WorksetSwitcherBetaSupportExport {
            record_kind: WorksetSwitcherBetaSupportExport::RECORD_KIND.to_string(),
            schema_version: aureline_workspace::WORKSET_SWITCHER_BETA_SCHEMA_VERSION,
            switcher,
            activation_previews: previews,
            reopen_parity_packets: vec![parity],
            emitted_at: "mono:beta:bundle".to_string(),
        };
        bundle
            .validate()
            .unwrap_or_else(|err| panic!("bundle in {path:?} must validate: {err}"));
        let payload = serde_json::to_string(&bundle).expect("bundle must serialize to JSON");
        let parsed: WorksetSwitcherBetaSupportExport = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("bundle in {path:?} must round-trip: {err}"));
        assert_eq!(parsed, bundle, "bundle round-trip mismatch in {path:?}");
    }
}

#[test]
fn fixture_coverage_includes_every_portability_label() {
    let fixtures = load_fixtures();
    let mut covered: BTreeMap<&'static str, bool> = BTreeMap::new();
    for (_, fixture) in &fixtures {
        let switcher = build_switcher(fixture);
        for row in &switcher.rows {
            covered.insert(row.portability_label.as_str(), true);
        }
    }
    for required in [
        "portable_with_rebinding",
        "local_only",
        "managed_provider_locked",
    ] {
        assert!(
            covered.contains_key(required),
            "missing fixture coverage for portability_label {required}",
        );
    }
}

#[test]
fn schema_exposes_every_record_kind() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/workspace/workset_switcher_beta.schema.json");
    let payload = std::fs::read_to_string(&schema_path).expect("schema must read");
    let schema: Value = serde_json::from_str(&payload).expect("schema must parse");
    let kinds = schema["$defs"]["record_kind"]["enum"]
        .as_array()
        .expect("record_kind enum must be present");
    let kinds: std::collections::BTreeSet<&str> = kinds.iter().filter_map(|v| v.as_str()).collect();
    for kind in [
        "workset_switcher_beta_record",
        "workset_switcher_beta_row",
        "workset_activation_preview",
        "workset_reopen_parity_packet",
        "workset_switcher_beta_support_export",
    ] {
        assert!(kinds.contains(kind), "schema missing record_kind {kind}");
    }
}

#[test]
fn schema_lists_every_portability_label() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/workspace/workset_switcher_beta.schema.json");
    let payload = std::fs::read_to_string(&schema_path).expect("schema must read");
    let schema: Value = serde_json::from_str(&payload).expect("schema must parse");
    let labels = schema["$defs"]["portability_label"]["enum"]
        .as_array()
        .expect("portability_label enum present");
    let labels: std::collections::BTreeSet<&str> =
        labels.iter().filter_map(|v| v.as_str()).collect();
    for required in [
        "portable",
        "portable_with_rebinding",
        "local_only",
        "policy_limited",
        "managed_provider_locked",
    ] {
        assert!(labels.contains(required), "schema missing label {required}");
    }
}

#[test]
fn parity_packet_for_every_fixture_carries_stable_scope_id() {
    for (path, fixture) in load_fixtures() {
        let parity = build_parity(&fixture);
        for binding in &parity.bindings {
            assert_eq!(
                binding.stable_scope_id, parity.stable_scope_id,
                "binding scope_id drift in {path:?}",
            );
            match binding.reopen_state {
                ScopeReopenState::Exact => assert!(binding.degraded_reason.is_none()),
                ScopeReopenState::Degraded => assert!(binding.degraded_reason.is_some()),
            }
        }
    }
}
