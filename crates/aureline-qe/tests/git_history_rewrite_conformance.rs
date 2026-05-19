//! Conformance test for the history-rewrite drill corpus.
//!
//! Loads `fixtures/git/m3/history_rewrite_corpus/manifest.json` and
//! runs every drill. The test fails when:
//!
//! - any positive drill does not parse, validate, or project,
//! - any positive drill misses a projection expectation (record kind,
//!   lifecycle, destructive gate, recovery posture, next-safe-path
//!   classes, blocks summary, audit events, redaction flags, or
//!   support/audit wiring),
//! - any negative drill is silently accepted by the contract, or
//! - any negative drill fails for a reason other than the recorded
//!   `expected_failure_substring`.

use std::path::PathBuf;

use aureline_qe::git_history_rewrite::{run_corpus_from_repo_root, DrillOutcome};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn history_rewrite_corpus_conformance() {
    let report = run_corpus_from_repo_root(&repo_root());
    assert!(
        !report.drills.is_empty(),
        "history-rewrite corpus must publish at least one drill"
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
        panic!(
            "history-rewrite corpus had failures: {}",
            failures.join("; ")
        );
    }
}

#[test]
fn corpus_covers_every_record_kind() {
    let report = run_corpus_from_repo_root(&repo_root());
    let mut record_kinds: Vec<String> = Vec::new();
    for drill in &report.drills {
        if !drill.positive {
            continue;
        }
        let payload = std::fs::read_to_string(&drill.fixture_path).expect("read fixture");
        let value: serde_json::Value = serde_json::from_str(&payload).expect("parse fixture");
        if let Some(kind) = value.get("record_kind").and_then(serde_json::Value::as_str) {
            record_kinds.push(kind.to_string());
        }
    }
    record_kinds.sort();
    record_kinds.dedup();
    for expected in [
        "history_rewrite_conflict_session_record",
        "history_rewrite_sequence_edit_session_record",
        "history_rewrite_stash_entry_record",
        "history_rewrite_recovery_checkpoint_record",
        "history_rewrite_ref_update_proposal_record",
    ] {
        assert!(
            record_kinds.iter().any(|kind| kind == expected),
            "corpus is missing positive drills for {expected}"
        );
    }
}

#[test]
fn corpus_covers_continue_skip_and_abort_conflict_actions() {
    let report = run_corpus_from_repo_root(&repo_root());
    let mut lifecycle_states: Vec<String> = Vec::new();
    for drill in &report.drills {
        if !drill.positive {
            continue;
        }
        let payload = std::fs::read_to_string(&drill.fixture_path).expect("read fixture");
        let value: serde_json::Value = serde_json::from_str(&payload).expect("parse fixture");
        if value.get("record_kind").and_then(serde_json::Value::as_str)
            == Some("history_rewrite_conflict_session_record")
        {
            if let Some(state) = value
                .get("lifecycle_state")
                .and_then(serde_json::Value::as_str)
            {
                lifecycle_states.push(state.to_string());
            }
        }
    }
    for expected in [
        "continuing_after_resolution",
        "skipped_conflicted_step",
        "aborted_rolled_back",
    ] {
        assert!(
            lifecycle_states.iter().any(|state| state == expected),
            "corpus must include a conflict-session drill for {expected}"
        );
    }
}
