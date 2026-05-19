//! Conformance test for the repo-topology drill corpus.
//!
//! Loads `fixtures/workspace/m3/repo_topology_corpus/manifest.json`
//! and runs every drill. The test fails when:
//!
//! - any positive drill does not parse or project,
//! - any positive drill misses a projection expectation (surface,
//!   repo-root kind, full-coverage claim, blockers, required
//!   affordances, mutation target, body-export posture, or honesty
//!   labels),
//! - any positive drill payload mentions a raw-body export flag,
//! - any negative drill is silently accepted by the projection, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.

use std::path::PathBuf;

use aureline_qe::repo_topology::{run_corpus_from_repo_root, DrillOutcome};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn repo_topology_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "repo-topology corpus must publish at least one drill"
    );
    if !report.all_passed() {
        let failures: Vec<String> = report
            .failures()
            .iter()
            .map(|drill| {
                let reason = match &drill.outcome {
                    DrillOutcome::Pass => "<pass?>".to_string(),
                    DrillOutcome::Fail(reason) => format!("{reason:?}"),
                };
                format!(
                    "{} ({} drill) at {}: {}",
                    drill.drill_id,
                    if drill.positive {
                        "positive"
                    } else {
                        "negative"
                    },
                    drill.fixture_path.display(),
                    reason
                )
            })
            .collect();
        panic!("repo-topology corpus had failures: {}", failures.join("; "));
    }
}

#[test]
fn corpus_covers_every_blocker_class() {
    let report = run_corpus_from_repo_root(&repo_root());
    let mut blockers: Vec<String> = Vec::new();
    for drill in &report.drills {
        if !drill.positive {
            continue;
        }
        let payload = std::fs::read_to_string(&drill.fixture_path).expect("read fixture");
        let value: serde_json::Value = serde_json::from_str(&payload).expect("parse fixture");
        if let Some(expect) = value.get("expect") {
            if let Some(arr) = expect
                .get("full_coverage_blockers")
                .and_then(serde_json::Value::as_array)
            {
                for blocker in arr {
                    if let Some(token) = blocker.as_str() {
                        blockers.push(token.to_string());
                    }
                }
            }
        }
    }
    blockers.sort();
    blockers.dedup();
    for expected in [
        "sparse_or_workset_narrowed",
        "shallow_history_present",
        "partial_clone_promisor_present",
        "submodule_uninitialized",
        "nested_independent_boundary",
        "lfs_pointer_only_present",
        "lfs_partially_hydrated",
        "policy_blocked",
        "unavailable_unknown",
    ] {
        assert!(
            blockers.iter().any(|b| b == expected),
            "corpus is missing a positive drill covering blocker `{expected}`. \
             Observed blockers: {blockers:?}"
        );
    }
}

#[test]
fn corpus_covers_every_mutation_target() {
    let report = run_corpus_from_repo_root(&repo_root());
    let mut targets: Vec<String> = Vec::new();
    for drill in &report.drills {
        if !drill.positive {
            continue;
        }
        let payload = std::fs::read_to_string(&drill.fixture_path).expect("read fixture");
        let value: serde_json::Value = serde_json::from_str(&payload).expect("parse fixture");
        if let Some(target) = value
            .get("expect")
            .and_then(|expect| expect.get("mutation_target"))
            .and_then(serde_json::Value::as_str)
        {
            targets.push(target.to_string());
        }
    }
    targets.sort();
    targets.dedup();
    for expected in [
        "parent_root",
        "child_root",
        "switch_root_required",
        "read_only_until_initialized",
        "read_only_until_hydrated",
        "policy_blocked",
    ] {
        assert!(
            targets.iter().any(|t| t == expected),
            "corpus is missing a positive drill covering mutation target `{expected}`. \
             Observed targets: {targets:?}"
        );
    }
}

#[test]
fn corpus_covers_every_body_export_posture() {
    let report = run_corpus_from_repo_root(&repo_root());
    let mut postures: Vec<String> = Vec::new();
    for drill in &report.drills {
        if !drill.positive {
            continue;
        }
        let payload = std::fs::read_to_string(&drill.fixture_path).expect("read fixture");
        let value: serde_json::Value = serde_json::from_str(&payload).expect("parse fixture");
        if let Some(posture) = value
            .get("expect")
            .and_then(|expect| expect.get("body_export_posture"))
            .and_then(serde_json::Value::as_str)
        {
            postures.push(posture.to_string());
        }
    }
    postures.sort();
    postures.dedup();
    for expected in [
        "hydrated_bytes_allowed",
        "pointer_metadata_only",
        "blocked_by_policy",
    ] {
        assert!(
            postures.iter().any(|p| p == expected),
            "corpus is missing a positive drill covering body-export posture `{expected}`. \
             Observed postures: {postures:?}"
        );
    }
}
