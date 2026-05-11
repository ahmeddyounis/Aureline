use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use aureline_history::{
    body_object_id, HistoryStorageRoot, LocalHistoryStore, MutationJournalStore,
};

use super::*;

fn unique_temp_root(label: &str) -> PathBuf {
    let pid = std::process::id();
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let mut root = std::env::temp_dir();
    root.push(format!("aureline_review_preview_{label}_{pid}_{stamp}"));
    root
}

fn fixture_engine(label: &str) -> (DestructiveCoreEngine, PathBuf) {
    let root = unique_temp_root(label);
    let storage = HistoryStorageRoot::new(&root);
    let journal = MutationJournalStore::new(storage.clone());
    let history = LocalHistoryStore::new(storage);
    let engine = DestructiveCoreEngine::new(
        "ws-destructive-core-fixture",
        "fixture user",
        journal,
        history,
    )
    .with_pinned_clock(vec![
        "2026-05-11T13:30:00Z".to_owned(),
        "2026-05-11T13:30:01Z".to_owned(),
        "2026-05-11T13:30:02Z".to_owned(),
        "2026-05-11T13:30:03Z".to_owned(),
        "2026-05-11T13:30:04Z".to_owned(),
        "2026-05-11T13:30:05Z".to_owned(),
    ]);
    (engine, root)
}

#[test]
fn protected_walk_propose_preview_apply_validate_revert_keeps_lineage_visible() {
    let (mut engine, root) = fixture_engine("protected_walk");
    engine.seed_target("src/launch.rs", b"call(legacy_fn);\nuse legacy_fn;\n".to_vec());
    engine.seed_target("src/router.rs", b"// route legacy_fn here\n".to_vec());
    engine.seed_target("docs/intro.md", b"# legacy_fn intro\n".to_vec());

    let mut packet = engine
        .propose(
            &["src/launch.rs", "src/router.rs", "docs/intro.md"],
            "legacy_fn",
            "modern_fn",
        )
        .expect("propose succeeds");

    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Propose);
    assert_eq!(packet.proposal.target_specs.len(), 3);
    assert_eq!(
        packet.proposal.declared_revert_class,
        RevertClass::RestoreFromCheckpoint
    );
    assert_eq!(
        packet.proposal.declared_consequence_class,
        ConsequenceClass::DestructiveReversibleWithCheckpoint
    );

    engine.preview(&mut packet).expect("preview succeeds");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Preview);
    let preview = packet.preview.as_ref().expect("preview present");
    assert!(preview.apply_admissibility.is_admitted());
    assert!(preview.overall_basis_drift_state.is_clean());
    assert_eq!(preview.total_match_count, 4);
    for row in &preview.rows {
        assert!(row.blocked_reason.is_none(), "row {} blocked", row.logical_ref);
        assert_eq!(row.basis_drift, BasisDriftState::NoDrift);
        assert!(row.match_count > 0);
    }

    engine.apply(&mut packet).expect("apply succeeds");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Apply);
    let apply = packet.apply.as_ref().expect("apply present");
    assert!(!apply.mutation_group_id.is_empty());
    assert!(!apply.local_history_group_id.is_empty());
    assert_eq!(apply.per_target_links.len(), 3);
    assert_eq!(apply.realized_revert_class, RevertClass::RestoreFromCheckpoint);

    // Each target now contains the post-apply bytes.
    for link in &apply.per_target_links {
        let post = engine
            .read_target(&link.logical_ref)
            .expect("target present after apply");
        let observed_digest = body_object_id(post);
        assert_eq!(observed_digest, link.post_apply_body_object_id);
        let pre_text = String::from_utf8_lossy(post);
        assert!(
            !pre_text.contains("legacy_fn"),
            "post-apply bytes must drop legacy_fn for {}",
            link.logical_ref
        );
        assert!(
            pre_text.contains("modern_fn"),
            "post-apply bytes must contain modern_fn for {}",
            link.logical_ref
        );
    }

    engine.validate(&mut packet).expect("validate succeeds");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Validate);
    let validation = packet.validation.as_ref().expect("validate present");
    assert!(validation.all_targets_matched);
    for result in &validation.per_target_results {
        assert!(result.matches, "{} mismatched", result.logical_ref);
    }

    // Lineage IDs are now bound through every phase.
    let ids = packet.lineage_ids();
    assert!(!ids.packet_id.is_empty());
    assert!(ids.preview_id.is_some());
    assert!(ids.apply_id.is_some());
    assert!(ids.mutation_group_id.is_some());
    assert!(ids.local_history_group_id.is_some());
    assert!(ids.validation_id.is_some());
    assert!(ids.revert_id.is_none());

    engine.revert(&mut packet).expect("revert succeeds");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Revert);
    let revert = packet.revert.as_ref().expect("revert present");
    assert_eq!(revert.realized_revert_class, RevertClass::RestoreFromCheckpoint);
    assert_eq!(revert.restored_target_count, 3);
    assert_eq!(revert.group_resolution, GroupResolution::Reverted);
    assert_eq!(
        revert.restored_from_local_history_group_id,
        packet
            .apply
            .as_ref()
            .map(|a| a.local_history_group_id.clone())
            .unwrap_or_default()
    );

    // Targets are now back to the pre-apply bytes.
    for link in packet
        .apply
        .as_ref()
        .map(|a| a.per_target_links.clone())
        .unwrap_or_default()
    {
        let observed = engine
            .read_target(&link.logical_ref)
            .expect("target present after revert");
        let observed_digest = body_object_id(observed);
        assert_eq!(
            observed_digest, link.pre_apply_body_object_id,
            "revert must restore pre-apply bytes for {}",
            link.logical_ref
        );
    }

    // Plaintext render contains every phase, the named undo group, and
    // the checkpoint id so support exports can be inspected by hand.
    let render = packet.render_plaintext();
    assert!(render.contains("Phase: revert"));
    assert!(render.contains("[Preview]"));
    assert!(render.contains("[Apply]"));
    assert!(render.contains("[Validate]"));
    assert!(render.contains("[Revert]"));
    assert!(render.contains("mutation_group_id:"));
    assert!(render.contains("local_history_group_id:"));

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn failure_drill_blocks_apply_when_basis_drifts_after_preview() {
    let (mut engine, root) = fixture_engine("scope_drift");
    engine.seed_target("src/launch.rs", b"call(legacy_fn);\n".to_vec());
    engine.seed_target("src/router.rs", b"// route legacy_fn here\n".to_vec());

    let mut packet = engine
        .propose(
            &["src/launch.rs", "src/router.rs"],
            "legacy_fn",
            "modern_fn",
        )
        .expect("propose succeeds");
    engine.preview(&mut packet).expect("preview succeeds");
    assert!(packet
        .preview
        .as_ref()
        .expect("preview present")
        .apply_admissibility
        .is_admitted());

    // External actor mutates one target after preview, drifting the basis.
    engine.simulate_external_mutation(
        "src/router.rs",
        b"// route legacy_fn here\n// extra log line\n".to_vec(),
    );

    // Reopen preview against the drifted basis and confirm admission is
    // refused with a typed reason.
    engine
        .reopen_preview(&mut packet)
        .expect("reopen preview succeeds");
    let preview = packet.preview.as_ref().expect("preview present after reopen");
    assert!(matches!(
        preview.apply_admissibility,
        ApplyAdmissibility::BlockedByBasisDrift { drifted_target_count: 1 }
    ));
    assert!(matches!(
        preview.overall_basis_drift_state,
        BasisDriftState::DriftDetected { .. }
    ));
    let drifted_row = preview
        .rows
        .iter()
        .find(|row| row.logical_ref == "src/router.rs")
        .expect("router row");
    assert_eq!(drifted_row.blocked_reason, Some(DiffRowBlockReason::BasisDrifted));
    assert!(matches!(
        drifted_row.basis_drift,
        BasisDriftState::DriftDetected { .. }
    ));

    // Apply must refuse to widen scope.
    let err = engine
        .apply(&mut packet)
        .expect_err("apply must be blocked by basis drift");
    match err {
        WedgeError::ApplyBlocked(ApplyAdmissibility::BlockedByBasisDrift {
            drifted_target_count,
        }) => {
            assert_eq!(drifted_target_count, 1);
        }
        other => panic!("expected BlockedByBasisDrift, got {other:?}"),
    }
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Preview);
    assert!(packet.apply.is_none());

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn apply_is_rejected_when_preview_is_skipped() {
    let (mut engine, root) = fixture_engine("skip_preview");
    engine.seed_target("src/launch.rs", b"call(legacy_fn);\n".to_vec());

    let mut packet = engine
        .propose(&["src/launch.rs"], "legacy_fn", "modern_fn")
        .expect("propose succeeds");

    let err = engine
        .apply(&mut packet)
        .expect_err("apply must require preview");
    matches!(
        err,
        WedgeError::NotInPhase {
            expected: PreviewApplyRevertPhase::Preview,
            actual: PreviewApplyRevertPhase::Propose,
        }
    );
    assert!(packet.apply.is_none());
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn keep_retires_a_validated_packet_without_a_revert() {
    let (mut engine, root) = fixture_engine("keep_path");
    engine.seed_target("src/launch.rs", b"call(legacy_fn);\n".to_vec());

    let mut packet = engine
        .propose(&["src/launch.rs"], "legacy_fn", "modern_fn")
        .expect("propose succeeds");
    engine.preview(&mut packet).expect("preview succeeds");
    engine.apply(&mut packet).expect("apply succeeds");
    engine.validate(&mut packet).expect("validate succeeds");
    engine.keep(&mut packet).expect("keep succeeds");
    assert_eq!(packet.current_phase, PreviewApplyRevertPhase::Keep);
    assert!(packet.kept);
    assert!(packet.kept_at.is_some());

    // Subsequent revert attempt is refused so the lifecycle cannot be
    // re-driven after Keep.
    let err = engine.revert(&mut packet).expect_err("revert refused after keep");
    matches!(err, WedgeError::AlreadyKept);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn target_missing_blocks_admission_with_typed_reason() {
    let (mut engine, root) = fixture_engine("target_missing");
    engine.seed_target("src/launch.rs", b"call(legacy_fn);\n".to_vec());

    let mut packet = engine
        .propose(
            &["src/launch.rs", "src/missing.rs"],
            "legacy_fn",
            "modern_fn",
        )
        .expect("propose succeeds");
    engine.preview(&mut packet).expect("preview succeeds");
    let preview = packet.preview.as_ref().expect("preview present");
    assert!(matches!(
        preview.apply_admissibility,
        ApplyAdmissibility::BlockedByTargetMissing { missing_target_count: 1 }
    ));
    let missing_row = preview
        .rows
        .iter()
        .find(|row| row.logical_ref == "src/missing.rs")
        .expect("missing row");
    assert_eq!(missing_row.blocked_reason, Some(DiffRowBlockReason::TargetMissing));
    assert!(matches!(
        missing_row.basis_drift,
        BasisDriftState::BasisMissing
    ));

    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn no_matches_blocks_admission_without_widening_scope() {
    let (mut engine, root) = fixture_engine("no_matches");
    engine.seed_target("src/launch.rs", b"untouched\n".to_vec());

    let mut packet = engine
        .propose(&["src/launch.rs"], "legacy_fn", "modern_fn")
        .expect("propose succeeds");
    engine.preview(&mut packet).expect("preview succeeds");
    let preview = packet.preview.as_ref().expect("preview present");
    assert!(matches!(
        preview.apply_admissibility,
        ApplyAdmissibility::BlockedByNoMatches
    ));
    let row = &preview.rows[0];
    assert_eq!(row.blocked_reason, Some(DiffRowBlockReason::NoMatchesFound));

    let _ = std::fs::remove_dir_all(&root);
}
