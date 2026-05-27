//! Corpus-level coverage for the hardened conflict-resolution stable contract.
//!
//! These tests load every fixture in
//! `fixtures/git/m4/harden_conflict_resolution_external_change_reconciliation_and_merge/`
//! and assert that:
//!
//! 1. Every fixture parses, validates, and projects without error.
//! 2. Operation provenance is preserved across the projection.
//! 3. Conflict sessions survive restart (non-empty session_id, repo_ref, worktree_ref).
//! 4. Provenance of competing inputs is preserved (base/ours/theirs refs and sources).
//! 5. Honest downgrade from structured to raw does not imply resolution.
//! 6. Support / export records keep every `raw_*_export_allowed` flag false and
//!    consumer-surface lists include both `support_export` and `audit_lane`.
//! 7. Restart snapshots mirror current session truth.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_git::{
    parse_stable_conflict_session_record, project_stable_conflict_session,
    StableConflictSessionRecord,
    STABLE_CONFLICT_CONSUMER_SURFACES, STABLE_CONFLICT_OPERATION_KINDS,
    STABLE_CONFLICT_SESSION_RECORD_KIND,
};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/git/m4/harden_conflict_resolution_external_change_reconciliation_and_merge")
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
        let projection = project_stable_conflict_session(&payload).unwrap_or_else(|err| {
            panic!(
                "fixture {} must project: {}",
                path.display(),
                err.message()
            )
        });
        assert!(
            STABLE_CONFLICT_OPERATION_KINDS
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
                && !projection.raw_patch_body_export_allowed,
            "fixture {} must keep raw_*_export_allowed false",
            path.display()
        );
    }
}

#[test]
fn every_fixture_validates_as_record() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        record
            .validate()
            .unwrap_or_else(|err| panic!("{} must validate: {err}", path.display()));
    }
}

#[test]
fn session_survives_restart() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        assert!(
            record.survives_restart(),
            "{}: session must survive restart",
            path.display()
        );
    }
}

#[test]
fn provenance_is_preserved() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        assert!(
            record.preserves_provenance(),
            "{}: provenance must be preserved (base/ours/theirs refs and sources)",
            path.display()
        );
    }
}

#[test]
fn downgrade_does_not_imply_resolution() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        if record.downgraded_honestly() {
            assert!(
                !matches!(
                    record.lifecycle_state.as_str(),
                    "completed_committed" | "completed_handed_off"
                ),
                "{}: downgraded session must not be completed",
                path.display()
            );
            assert!(
                record.unresolved_count > 0,
                "{}: downgraded session must still have unresolved markers",
                path.display()
            );
        }
    }
}

#[test]
fn continuing_after_resolution_requires_checkpoint() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        if record.lifecycle_state == "continuing_after_resolution" {
            assert!(
                record.has_recovery_checkpoint(),
                "{}: continuing_after_resolution requires a recovery checkpoint",
                path.display()
            );
        }
    }
}

#[test]
fn support_export_packet_is_reopenable() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        assert!(
            !record.support_export.reopen_context_ref.trim().is_empty(),
            "{}: support export must carry a reopen_context_ref",
            path.display()
        );
    }
}

#[test]
fn consumer_surfaces_are_closed_vocabulary() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record: StableConflictSessionRecord =
            serde_json::from_str(&payload).expect("parse fixture");
        for surface in &record.consumer_surfaces {
            assert!(
                STABLE_CONFLICT_CONSUMER_SURFACES.contains(&surface.as_str()),
                "{}: consumer surface '{}' is not in the closed vocabulary",
                path.display(),
                surface
            );
        }
    }
}

#[test]
fn parse_and_project_roundtrip() {
    for path in collect_corpus() {
        let payload = fs::read_to_string(&path).expect("read fixture");
        let record = parse_stable_conflict_session_record(&payload).expect("parse and validate");
        let projection = record.project();
        assert_eq!(projection.record_id, record.session_id);
        assert_eq!(projection.record_kind, STABLE_CONFLICT_SESSION_RECORD_KIND);
        assert_eq!(projection.operation_kind, record.operation_kind);
        assert_eq!(projection.lifecycle_state, record.lifecycle_state);
        assert_eq!(
            projection.resolution_mode,
            record.resolution_mode.as_str()
        );
    }
}

#[test]
fn build_packet_from_input() {
    use aureline_git::{
        build_stable_conflict_session_packet, ConflictProvenanceInput,
        StableConflictSessionCommandInput, StableConflictSessionInput,
        StableConflictSessionSupportExportInput,
    };

    let input = StableConflictSessionInput {
        session_id: "session.test.build.001".to_string(),
        generated_at: "2026-05-27T12:00:00Z".to_string(),
        operation_kind: "merge".to_string(),
        lifecycle_state: "active_awaiting_resolution".to_string(),
        repo_ref: "repo.test".to_string(),
        worktree_ref: "worktree.test".to_string(),
        base_ref: "ref.base".to_string(),
        ours_ref: "ref.ours".to_string(),
        theirs_ref: "ref.theirs".to_string(),
        affected_path_tokens: vec!["path.a.rs".to_string(), "path.b.rs".to_string()],
        unresolved_count: 3,
        resolution_mode: "structured".to_string(),
        previous_session_ref: None,
        recovery_checkpoint_ref: Some("checkpoint.001".to_string()),
        provenance: ConflictProvenanceInput {
            base_source_class: "git_index_stage".to_string(),
            ours_source_class: "git_head".to_string(),
            theirs_source_class: "git_remote_tracking".to_string(),
            input_freshness_class: "fresh_observed".to_string(),
            summary_label: "Test provenance".to_string(),
        },
        commands: vec![
            StableConflictSessionCommandInput {
                command_id: "cmd.open_structured".to_string(),
                command_class: "open_structured_resolver".to_string(),
                actionable: true,
                blocked_reasons: vec![],
                summary_label: "Open structured resolver".to_string(),
            },
            StableConflictSessionCommandInput {
                command_id: "cmd.continue".to_string(),
                command_class: "continue_after_resolve".to_string(),
                actionable: false,
                blocked_reasons: vec!["unresolved_markers_remain".to_string()],
                summary_label: "Continue after resolve".to_string(),
            },
        ],
        support_export: StableConflictSessionSupportExportInput {
            support_export_id: "export.001".to_string(),
            reopen_context_ref: "context.001".to_string(),
            reopen_command_id_ref: "reopen.001".to_string(),
            consumer_surfaces: vec![
                "support_export".to_string(),
                "audit_lane".to_string(),
                "desktop_conflict_resolver".to_string(),
            ],
            redaction_class: "safe".to_string(),
            summary_label: "Test export".to_string(),
        },
        summary_label: "Test session".to_string(),
    };

    let packet = build_stable_conflict_session_packet(input).expect("build packet");
    assert_eq!(
        packet.record_kind,
        "git_stable_conflict_session_packet"
    );
    assert_eq!(packet.session.session_id, "session.test.build.001");
    assert!(packet.inspection.awaiting_resolution);
    assert!(packet.inspection.structured_open);
    assert!(!packet.inspection.downgraded);
    assert!(packet.inspection.checkpoint_captured);
    assert!(packet.inspection.restartable);
    assert!(packet.inspection.provenance_preserved);
    assert!(packet.inspection.support_export_reopenable);
    assert_eq!(packet.commands.len(), 2);
    assert!(
        packet.commands.iter().any(|cmd| cmd.command_class == "open_structured_resolver" && cmd.actionable)
    );
    assert!(
        packet.commands.iter().any(|cmd| cmd.command_class == "continue_after_resolve" && !cmd.actionable)
    );
}

#[test]
fn build_packet_downgrade_command_actionable_only_when_structured() {
    use aureline_git::{
        build_stable_conflict_session_packet, ConflictProvenanceInput,
        StableConflictSessionCommandInput, StableConflictSessionInput,
        StableConflictSessionSupportExportInput,
    };

    let mut input = StableConflictSessionInput {
        session_id: "session.test.downgrade.002".to_string(),
        generated_at: "2026-05-27T12:00:00Z".to_string(),
        operation_kind: "merge".to_string(),
        lifecycle_state: "active_awaiting_resolution".to_string(),
        repo_ref: "repo.test".to_string(),
        worktree_ref: "worktree.test".to_string(),
        base_ref: "ref.base".to_string(),
        ours_ref: "ref.ours".to_string(),
        theirs_ref: "ref.theirs".to_string(),
        affected_path_tokens: vec!["path.a.rs".to_string()],
        unresolved_count: 1,
        resolution_mode: "structured".to_string(),
        previous_session_ref: None,
        recovery_checkpoint_ref: None,
        provenance: ConflictProvenanceInput {
            base_source_class: "git_index_stage".to_string(),
            ours_source_class: "git_head".to_string(),
            theirs_source_class: "git_remote_tracking".to_string(),
            input_freshness_class: "fresh_observed".to_string(),
            summary_label: "Test provenance".to_string(),
        },
        commands: vec![StableConflictSessionCommandInput {
            command_id: "cmd.downgrade".to_string(),
            command_class: "downgrade_to_raw".to_string(),
            actionable: true,
            blocked_reasons: vec![],
            summary_label: "Downgrade to raw".to_string(),
        }],
        support_export: StableConflictSessionSupportExportInput {
            support_export_id: "export.002".to_string(),
            reopen_context_ref: "context.002".to_string(),
            reopen_command_id_ref: "reopen.002".to_string(),
            consumer_surfaces: vec!["support_export".to_string(), "audit_lane".to_string()],
            redaction_class: "safe".to_string(),
            summary_label: "Test export".to_string(),
        },
        summary_label: "Test downgrade".to_string(),
    };

    let packet = build_stable_conflict_session_packet(input.clone()).expect("build packet");
    let downgrade_cmd = packet
        .commands
        .iter()
        .find(|cmd| cmd.command_class == "downgrade_to_raw")
        .expect("downgrade command exists");
    assert!(downgrade_cmd.actionable, "downgrade must be actionable when structured");

    // Now switch to raw mode: downgrade should no longer be actionable.
    input.resolution_mode = "raw".to_string();
    let packet2 = build_stable_conflict_session_packet(input).expect("build packet");
    let downgrade_cmd2 = packet2
        .commands
        .iter()
        .find(|cmd| cmd.command_class == "downgrade_to_raw")
        .expect("downgrade command exists");
    assert!(
        !downgrade_cmd2.actionable,
        "downgrade must not be actionable when already raw"
    );
}
