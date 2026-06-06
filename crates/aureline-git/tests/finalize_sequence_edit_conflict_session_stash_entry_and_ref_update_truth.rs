//! Corpus coverage for the stable risky-VCS truth packet.
//!
//! These tests load the canonical fixtures under
//! `fixtures/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth/`
//! and assert that conflict sessions, sequence edits, stash entries, recovery
//! checkpoints, and ref-update proposals share one restartable, exportable
//! object vocabulary.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_git::{
    parse_risky_vcs_truth_packet, project_risky_vcs_truth_packet, RiskyVcsTruthPacket,
};

fn corpus_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/review/m4/finalize-sequence-edit-conflict-session-stash-entry-and-ref-update-truth",
    )
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
fn every_fixture_validates_and_projects() {
    let paths = collect_corpus();
    assert!(!paths.is_empty(), "corpus must not be empty");
    for path in paths {
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
        let packet = parse_risky_vcs_truth_packet(&payload)
            .unwrap_or_else(|err| panic!("{} must validate: {err}", path.display()));
        let projection = packet.project();
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(projection.repo_ref, packet.repo_ref);
        assert_eq!(projection.worktree_ref, packet.worktree_ref);
        assert!(
            projection.restartable,
            "{} must be restartable",
            path.display()
        );
        assert!(
            projection.support_export_lineage_complete,
            "{} must have complete support-export lineage",
            path.display()
        );
        assert!(
            !projection.raw_export_allowed,
            "{} must keep raw export disabled",
            path.display()
        );
    }
}

#[test]
fn stable_lineage_preserves_all_object_truths() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let packet = parse_risky_vcs_truth_packet(&payload).expect("stable lineage validates");
    let projection = packet.project();
    assert_eq!(projection.object_count, 5);
    assert!(projection.conflict_provenance_preserved);
    assert!(projection.sequence_todo_bound);
    assert!(projection.stash_scope_preserved);
    assert!(projection.recovery_truth_explicit);
    assert!(projection.ref_update_reviewable);

    let sequence = &packet.sequence_edit_sessions[0];
    assert!(sequence.raw_todo_text_ref.contains(&sequence.session_id));
    assert!(sequence.structured_cards_ref.contains(&sequence.session_id));
    assert!(sequence
        .ordered_operations
        .iter()
        .any(|operation| operation.conflict_session_ref.as_deref() == Some("conflict.rebase.001")));

    let stash = &packet.stash_entries[0];
    for expected in [
        "apply_stash",
        "pop_stash",
        "drop_stash",
        "branch_from_stash",
    ] {
        assert!(
            stash
                .available_command_classes
                .iter()
                .any(|command_class| command_class == expected),
            "stash entry must preserve {expected}"
        );
    }
}

#[test]
fn reflog_only_fixture_keeps_disclosure_distinct_from_checkpoint() {
    let payload = fs::read_to_string(corpus_dir().join("reflog_only_reset_disclosed.json"))
        .expect("read fixture");
    let packet = parse_risky_vcs_truth_packet(&payload).expect("reflog fixture validates");
    let checkpoint = &packet.recovery_checkpoints[0];
    assert!(!checkpoint.checkpoint_possible);
    assert!(checkpoint.reflog_only_disclosure_required);
    assert!(checkpoint.reflog_only_disclosure_acknowledged);
    assert!(checkpoint
        .restore_options
        .iter()
        .any(|option| option == "reflog_only_disclosure"));
}

#[test]
fn blocked_publication_is_not_reviewable_but_still_exportable() {
    let payload = fs::read_to_string(corpus_dir().join("blocked_invalidated_ref_update.json"))
        .expect("read fixture");
    let projection = project_risky_vcs_truth_packet(&payload).expect("blocked fixture projects");
    assert!(!projection.ref_update_reviewable);
    assert!(projection.recovery_truth_explicit);
    assert!(projection.support_export_lineage_complete);
}

#[test]
fn rejects_duplicate_sequence_ordinals() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let mut packet: RiskyVcsTruthPacket = serde_json::from_str(&payload).expect("parse fixture");
    packet.sequence_edit_sessions[0].ordered_operations[1].ordinal = 0;
    let error = packet
        .validate()
        .expect_err("duplicate ordinal must reject");
    assert!(error.message().contains("duplicated"));
}

#[test]
fn rejects_missing_support_export_lineage() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let mut packet: RiskyVcsTruthPacket = serde_json::from_str(&payload).expect("parse fixture");
    packet
        .support_export
        .lineage_object_refs
        .retain(|object_ref| object_ref != "stash.entry.001");
    let error = packet.validate().expect_err("missing lineage must reject");
    assert!(error.message().contains("support export lineage"));
}

#[test]
fn rejects_raw_support_export_leakage() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let mut packet: RiskyVcsTruthPacket = serde_json::from_str(&payload).expect("parse fixture");
    packet.support_export.raw_patch_body_export_allowed = true;
    let error = packet.validate().expect_err("raw export must reject");
    assert!(error.message().contains("raw_*_export_allowed"));
}

#[test]
fn rejects_ready_proposal_with_ambiguous_target() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let mut packet: RiskyVcsTruthPacket = serde_json::from_str(&payload).expect("parse fixture");
    packet.ref_update_proposals[0].target_selection_state =
        "ambiguous_target_blocks_apply".to_string();
    let error = packet
        .validate()
        .expect_err("ready ambiguous target must reject");
    assert!(error.message().contains("explicit target"));
}

#[test]
fn rejects_stash_entry_that_collapses_action_classes() {
    let payload = fs::read_to_string(corpus_dir().join("stable_risky_vcs_lineage.json"))
        .expect("read fixture");
    let mut packet: RiskyVcsTruthPacket = serde_json::from_str(&payload).expect("parse fixture");
    packet.stash_entries[0].available_command_classes = vec!["apply_stash".to_string()];
    let error = packet
        .validate()
        .expect_err("collapsed stash actions must reject");
    assert!(error.message().contains("distinct action classes"));
}
