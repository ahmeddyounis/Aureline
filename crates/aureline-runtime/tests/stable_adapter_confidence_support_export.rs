//! Fixture-backed coverage for the stable adapter-confidence export.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_adapter_confidence_support_export, ImportedLiveState,
    ADAPTER_CONFIDENCE_TOOLING_ARTIFACT_DOC_REF, ADAPTER_CONFIDENCE_TOOLING_DOC_REF,
    ADAPTER_CONFIDENCE_TOOLING_FIXTURE_DIR, ADAPTER_CONFIDENCE_TOOLING_SCHEMA_REF,
    BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AdapterConfidenceFixture {
    record_kind: String,
    schema_version: u32,
    fixture_id: String,
    generated_from: String,
    expect: AdapterConfidenceExpect,
}

#[derive(Debug, Deserialize)]
struct AdapterConfidenceExpect {
    lane_type_tokens: Vec<String>,
    adapter_health_reason_tokens: Vec<String>,
    imported_live_state_tokens: Vec<String>,
    non_live_receipt_postures: Vec<String>,
    target_exactness_status_tokens: Vec<String>,
    diff_counts: DiffCounts,
    redaction_safe: bool,
}

#[derive(Debug, Deserialize)]
struct DiffCounts {
    added: usize,
    removed: usize,
    renamed: usize,
    downgraded_confidence: usize,
    newly_heuristic: usize,
    newly_exact: usize,
    now_unresolved: usize,
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

fn load_fixture() -> AdapterConfidenceFixture {
    let path = repo_root()
        .join(ADAPTER_CONFIDENCE_TOOLING_FIXTURE_DIR)
        .join("stable_adapter_confidence_contract.json");
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

fn token_set(values: impl IntoIterator<Item = String>) -> BTreeSet<String> {
    values.into_iter().collect()
}

#[test]
fn stable_adapter_confidence_paths_exist() {
    assert_exists(ADAPTER_CONFIDENCE_TOOLING_SCHEMA_REF);
    assert_exists(ADAPTER_CONFIDENCE_TOOLING_FIXTURE_DIR);
    assert_exists(ADAPTER_CONFIDENCE_TOOLING_ARTIFACT_DOC_REF);
    assert_exists(ADAPTER_CONFIDENCE_TOOLING_DOC_REF);
}

#[test]
fn stable_adapter_confidence_export_matches_fixture() {
    let fixture = load_fixture();
    assert_eq!(
        fixture.record_kind,
        "adapter_confidence_stable_contract_case"
    );
    assert_eq!(fixture.schema_version, 1);
    assert_eq!(
        fixture.generated_from,
        "aureline_runtime::current_stable_adapter_confidence_support_export"
    );
    assert!(
        !fixture.fixture_id.trim().is_empty(),
        "fixture id must be non-empty"
    );

    let export = current_stable_adapter_confidence_support_export();
    assert_eq!(
        export.record_kind,
        BUILD_INTELLIGENCE_SUPPORT_EXPORT_RECORD_KIND
    );
    assert_eq!(export.redaction_safe, fixture.expect.redaction_safe);
    assert_eq!(export.adapter_health_strips.len(), 5);
    assert_eq!(export.target_rows.len(), 5);
    assert_eq!(export.run_config_cards.len(), 5);
    assert_eq!(export.receipts.len(), 3);

    assert_eq!(
        token_set(
            export
                .coverage_manifest
                .lane_type_tokens
                .iter()
                .cloned()
                .collect::<Vec<_>>()
        ),
        token_set(fixture.expect.lane_type_tokens.clone())
    );
    assert_eq!(
        token_set(
            export
                .adapter_health_strips
                .iter()
                .filter_map(|strip| strip.health_reason_token.clone())
                .collect::<Vec<_>>()
        ),
        token_set(fixture.expect.adapter_health_reason_tokens.clone())
    );
    assert_eq!(
        token_set(
            export
                .target_rows
                .iter()
                .map(|row| row.imported_live_state_token.clone())
                .collect::<Vec<_>>()
        ),
        token_set(fixture.expect.imported_live_state_tokens.clone())
    );
    assert_eq!(
        token_set(
            export
                .target_rows
                .iter()
                .map(|row| row.exactness_status_token.clone())
                .collect::<Vec<_>>()
        ),
        token_set(fixture.expect.target_exactness_status_tokens.clone())
    );

    for expected_posture in &fixture.expect.non_live_receipt_postures {
        assert!(
            export.receipts.iter().any(|receipt| {
                receipt.high_trust_action_posture_token == *expected_posture
                    && receipt.imported_live_state != ImportedLiveState::LiveWorkspaceInspection
            }),
            "missing non-live receipt posture {expected_posture}"
        );
    }

    let diff = export.discovery_diffs.first().expect("discovery diff");
    assert_eq!(diff.added.len(), fixture.expect.diff_counts.added);
    assert_eq!(diff.removed.len(), fixture.expect.diff_counts.removed);
    assert_eq!(diff.renamed.len(), fixture.expect.diff_counts.renamed);
    assert_eq!(
        diff.downgraded_confidence.len(),
        fixture.expect.diff_counts.downgraded_confidence
    );
    assert_eq!(
        diff.newly_heuristic.len(),
        fixture.expect.diff_counts.newly_heuristic
    );
    assert_eq!(
        diff.newly_exact.len(),
        fixture.expect.diff_counts.newly_exact
    );
    assert_eq!(
        diff.now_unresolved.len(),
        fixture.expect.diff_counts.now_unresolved
    );

    let plaintext = export.render_plaintext();
    assert!(plaintext.contains("lane=heuristic_fallback"));
    assert!(plaintext.contains("provenance=replayed_receipt"));
    assert!(plaintext.contains("provenance=imported_artifact"));
    assert!(plaintext.contains("posture=inspect_only"));
    assert!(plaintext.contains("posture=refresh_required"));
    assert!(plaintext.contains("newly_heuristic=1"));
    assert!(plaintext.contains("now_unresolved=1"));
    assert!(!plaintext.contains("/Users/"));
}
