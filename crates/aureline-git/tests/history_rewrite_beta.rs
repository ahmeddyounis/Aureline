//! Corpus-level coverage for the history-rewrite beta contract.
//!
//! These tests load every fixture in
//! `fixtures/git/m3/history_rewrite_and_stash/` and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Operation provenance is preserved across the projection.
//! 3. The recovery-posture destructive-action gate matches the lifecycle.
//! 4. Blocked ref-update proposals always publish at least one
//!    `next_safe_path`.
//! 5. Support / export records keep every `raw_*_export_allowed` flag
//!    false and consumer-surface lists include both `support_export`
//!    and `audit_lane`.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_git::{
    project_history_rewrite_record, HistoryRewriteRecord, CONFLICT_SESSION_RECORD_KIND,
    HISTORY_REWRITE_OPERATION_KINDS, RECOVERY_CHECKPOINT_RECORD_KIND,
    REF_UPDATE_PROPOSAL_RECORD_KIND, SEQUENCE_EDIT_SESSION_RECORD_KIND, STASH_ENTRY_RECORD_KIND,
};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/git/m3/history_rewrite_and_stash")
}

fn collect_corpus() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = fs::read_dir(corpus_dir())
        .expect("read corpus directory")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect();
    paths.sort();
    paths
}

#[test]
fn every_fixture_projects() {
    let paths = collect_corpus();
    assert!(!paths.is_empty(), "corpus must not be empty");
    for path in paths {
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let projection = project_history_rewrite_record(&payload).unwrap_or_else(|err| {
            panic!("fixture {} must project: {}", path.display(), err.message())
        });
        assert!(
            HISTORY_REWRITE_OPERATION_KINDS
                .iter()
                .any(|kind| *kind == projection.operation_kind),
            "fixture {} carries unknown operation_kind {}",
            path.display(),
            projection.operation_kind
        );
        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "support_export"),
            "fixture {} must wire support_export",
            path.display()
        );
        assert!(
            projection
                .consumer_surfaces
                .iter()
                .any(|surface| surface == "audit_lane"),
            "fixture {} must wire audit_lane",
            path.display()
        );
        assert!(
            !projection.raw_path_export_allowed
                && !projection.raw_branch_name_export_allowed
                && !projection.raw_patch_body_export_allowed
                && !projection.raw_reflog_body_export_allowed
                && !projection.raw_stash_body_export_allowed,
            "fixture {} must keep raw_*_export_allowed false",
            path.display()
        );
    }
}

#[test]
fn destructive_gate_is_consistent_with_lifecycle() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: HistoryRewriteRecord = serde_json::from_str(&payload).expect("parse fixture");
        record.validate().expect("validate fixture");
        let projection = record.project();
        match (record.record_id(), projection.record_kind.as_str()) {
            (_, kind) if kind == CONFLICT_SESSION_RECORD_KIND => {
                if matches!(
                    projection.lifecycle_state.as_str(),
                    "continuing_after_resolution"
                        | "skipped_conflicted_step"
                        | "completed_committed"
                ) {
                    assert!(
                        projection.destructive_gate_satisfied,
                        "{} ({}): destructive lifecycle must satisfy gate",
                        path.display(),
                        projection.lifecycle_state
                    );
                }
            }
            (_, kind) if kind == SEQUENCE_EDIT_SESSION_RECORD_KIND => {
                if matches!(
                    projection.lifecycle_state.as_str(),
                    "running" | "completed_admitted"
                ) {
                    assert!(
                        projection.destructive_gate_satisfied,
                        "{} ({}): destructive lifecycle must satisfy gate",
                        path.display(),
                        projection.lifecycle_state
                    );
                }
            }
            (_, kind) if kind == STASH_ENTRY_RECORD_KIND => {
                if matches!(
                    projection.lifecycle_state.as_str(),
                    "applied_popped" | "dropped"
                ) {
                    assert!(
                        projection.destructive_gate_satisfied,
                        "{} ({}): destructive lifecycle must satisfy gate",
                        path.display(),
                        projection.lifecycle_state
                    );
                }
            }
            (_, kind) if kind == REF_UPDATE_PROPOSAL_RECORD_KIND => {
                if matches!(
                    projection.lifecycle_state.as_str(),
                    "ready_to_apply" | "applied"
                ) {
                    assert!(
                        projection.destructive_gate_satisfied,
                        "{} ({}): destructive lifecycle must satisfy gate",
                        path.display(),
                        projection.lifecycle_state
                    );
                }
                if matches!(
                    projection.lifecycle_state.as_str(),
                    "blocked_protected_branch" | "blocked_policy" | "blocked_collaboration"
                ) {
                    assert!(
                        !projection.next_safe_path_classes.is_empty(),
                        "{}: blocked proposal must publish next-safe paths",
                        path.display(),
                    );
                    assert!(
                        !projection.blocks_summary.is_empty(),
                        "{}: blocked proposal must record at least one block",
                        path.display(),
                    );
                }
            }
            (_, kind) if kind == RECOVERY_CHECKPOINT_RECORD_KIND => {
                if projection.lifecycle_state == "captured_ready_to_restore" {
                    assert!(
                        projection.destructive_gate_satisfied,
                        "{}: a captured checkpoint must satisfy the gate",
                        path.display(),
                    );
                }
            }
            _ => panic!("unexpected record_kind {}", projection.record_kind),
        }
    }
}

#[test]
fn next_safe_paths_are_quoted_to_audit_lane() {
    // For the protected-branch case, the audit lane must record both safe paths
    // that the proposal offered (open_alternate_worktree and create_temporary_branch).
    let path = corpus_dir().join("ref_update_protected_branch_blocked.json");
    let payload = fs::read_to_string(&path).expect("read protected-branch fixture");
    let record: HistoryRewriteRecord =
        serde_json::from_str(&payload).expect("parse protected-branch fixture");
    record
        .validate()
        .expect("validate protected-branch fixture");
    let projection = record.project();
    assert!(projection
        .next_safe_path_classes
        .iter()
        .any(|class| class == "open_alternate_worktree"));
    assert!(projection
        .next_safe_path_classes
        .iter()
        .any(|class| class == "create_temporary_branch"));
    assert!(
        projection
            .audit_event_ids
            .iter()
            .filter(|event| event.as_str() == "next_safe_path_offered")
            .count()
            >= 2,
        "protected-branch proposal must record at least two next_safe_path_offered events",
    );
}

#[test]
fn corpus_carries_all_record_kinds() {
    let mut seen: Vec<String> = collect_corpus()
        .into_iter()
        .map(|path| {
            let payload = fs::read_to_string(&path).expect("read fixture");
            let record: HistoryRewriteRecord =
                serde_json::from_str(&payload).expect("parse fixture");
            record.validate().expect("validate fixture");
            record.project().record_kind
        })
        .collect();
    seen.sort();
    seen.dedup();
    for expected in [
        CONFLICT_SESSION_RECORD_KIND,
        SEQUENCE_EDIT_SESSION_RECORD_KIND,
        STASH_ENTRY_RECORD_KIND,
        RECOVERY_CHECKPOINT_RECORD_KIND,
        REF_UPDATE_PROPOSAL_RECORD_KIND,
    ] {
        assert!(
            seen.iter().any(|kind| kind == expected),
            "corpus is missing fixture for {}",
            expected
        );
    }
}
