use super::*;

// ── Seed integrity ──────────────────────────────────────────────────────────

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_learning_progress_snapshots();
    validate_m5_learning_progress_snapshots(&manifest)
        .expect("seeded learning-progress manifest must pass validation");
}

#[test]
fn seeded_manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_progress_manifest_from_json(&json).expect("deserialize");
    assert_eq!(manifest, back);
    // Step, resume-point, disclosure, and digest identity survive export + reopen.
    assert_eq!(manifest.known_snapshot_ids(), back.known_snapshot_ids());
    for (orig, reopened) in manifest.snapshots.iter().zip(back.snapshots.iter()) {
        assert_eq!(orig.steps, reopened.steps);
        assert_eq!(orig.resume_point, reopened.resume_point);
        assert_eq!(orig.disclosure_state, reopened.disclosure_state);
        assert_eq!(orig.export_refs, reopened.export_refs);
    }
    for (orig, reopened) in manifest.digests.iter().zip(back.digests.iter()) {
        assert_eq!(orig.action_kinds(), reopened.action_kinds());
        assert_eq!(orig.covered_snapshot_refs, reopened.covered_snapshot_refs);
    }
}

#[test]
fn ships_snapshots_across_several_families_and_two_digests() {
    let manifest = seeded_m5_learning_progress_snapshots();
    assert!(manifest.snapshots.len() >= 4);
    let families: BTreeSet<_> = manifest.snapshots.iter().map(|s| s.family).collect();
    assert!(
        families.len() >= 4,
        "snapshots should span several families"
    );
    assert_eq!(manifest.digests.len(), 2);
}

// ── Acceptance: pause + resume without losing progress ───────────────────────

#[test]
fn every_resumable_snapshot_survives_restart_and_resumes_after_restart() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        assert!(
            snapshot.survives_restart,
            "{} must survive restart",
            snapshot.snapshot_id
        );
        if let Some(resume) = &snapshot.resume_point {
            assert!(
                resume.resumable_after_restart,
                "{} resume point must survive restart",
                snapshot.snapshot_id
            );
        }
    }
}

#[test]
fn dismissed_steps_are_always_reversible() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let mut saw_dismissed = false;
    for snapshot in &manifest.snapshots {
        for step in &snapshot.steps {
            if step.state == StepProgressState::Dismissed {
                saw_dismissed = true;
                assert!(
                    step.dismissal_reversible,
                    "dismissed step {} must be reversible",
                    step.step_ref
                );
            }
        }
    }
    assert!(saw_dismissed, "seed should exercise a dismissed step");
}

#[test]
fn completed_and_dismissed_steps_are_counted() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let notebook = manifest
        .snapshot("learning:m5:progress:notebook_intro_tour")
        .expect("notebook snapshot");
    assert_eq!(notebook.completed_step_count(), 1);
    assert_eq!(notebook.dismissed_step_count(), 1);
    assert!(notebook.has_active_progress());
}

// ── Acceptance: progress stays user-owned and never leaks into the repo ──────

#[test]
fn progress_is_user_owned_local_first_and_never_repo_or_collaborator_visible() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        assert_eq!(
            snapshot.data_ownership,
            DataOwnershipClass::UserOwnedLocalFirst,
            "{}",
            snapshot.snapshot_id
        );
        assert!(
            snapshot.privacy.qualifies_stable(),
            "{}",
            snapshot.snapshot_id
        );
        assert!(snapshot.privacy.user_owned_local_by_default);
        assert!(!snapshot.privacy.repo_visible);
        assert!(!snapshot.privacy.shared_with_collaborators);
        assert!(!snapshot.privacy.extension_telemetry_read_access);
    }
}

#[test]
fn exports_redact_payloads_and_are_user_initiated() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let mut saw_export = false;
    for snapshot in &manifest.snapshots {
        for export in &snapshot.export_refs {
            saw_export = true;
            assert!(export.qualifies_stable(), "{}", export.export_id);
            assert!(export.redacts_raw_payloads);
            assert!(export.user_initiated);
        }
    }
    assert!(saw_export, "seed should exercise an export ref");
}

// ── Acceptance: disclosure states survive support/export review ──────────────

#[test]
fn disclosure_states_are_consistent_with_evidence() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        match snapshot.disclosure_state {
            SnapshotDisclosureState::SyncEligible => {
                assert!(snapshot.sync_policy.is_sync_eligible());
                assert!(snapshot.sync_disclosed);
            }
            SnapshotDisclosureState::Exported => {
                assert!(!snapshot.export_refs.is_empty());
            }
            SnapshotDisclosureState::Reset => {
                assert!(snapshot.resume_point.is_none());
                assert!(!snapshot.has_active_progress());
            }
            SnapshotDisclosureState::LocalOnly => {}
        }
        assert!(
            snapshot.safe_for_support_export,
            "{} should be safe for support export",
            snapshot.snapshot_id
        );
    }
}

#[test]
fn disclosure_states_survive_export_and_reopen() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_progress_manifest_from_json(&json).expect("deserialize");
    for (orig, reopened) in manifest.snapshots.iter().zip(back.snapshots.iter()) {
        assert_eq!(
            orig.disclosure_state, reopened.disclosure_state,
            "{} disclosure state must survive export",
            orig.snapshot_id
        );
    }
}

// ── Acceptance: durable digest replaces ephemeral banners ────────────────────

#[test]
fn every_digest_exposes_resume_dismiss_snooze_reset_export() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for digest in &manifest.digests {
        let kinds = digest.action_kinds();
        for required in REQUIRED_DIGEST_ACTION_KINDS {
            assert!(
                kinds.contains(&required),
                "{} missing {} action",
                digest.digest_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn every_digest_action_is_command_backed_keyboard_reachable_reversible_and_non_mutating() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for digest in &manifest.digests {
        for action in &digest.actions {
            assert!(
                action.qualifies_stable(),
                "{} action {} fails an invariant",
                digest.digest_id,
                action.action_kind.as_str()
            );
            assert!(!action.command_id_ref.is_empty());
            assert!(action.keyboard_shortcut_ref.is_some());
            assert!(action.reversible);
            assert!(action.inspectable);
            assert!(!action.silent_write_allowed);
            assert!(!action.mutates_workspace);
        }
    }
}

#[test]
fn digests_are_durable_and_inspectable_not_ephemeral() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for digest in &manifest.digests {
        assert!(digest.replaces_ephemeral_banners, "{}", digest.digest_id);
        assert!(digest.durable_recovery_available, "{}", digest.digest_id);
        assert!(digest.exposure.qualifies_stable(), "{}", digest.digest_id);
        assert!(digest.exposure.in_settings);
        assert!(digest.exposure.in_help_about);
        assert!(digest.exposure.in_diagnostics);
        assert!(digest.exposure.in_support_export);
        assert!(!digest.exposure.hidden_in_transient_overlay_only);
    }
}

#[test]
fn every_snapshot_is_covered_by_a_durable_digest() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let mut covered: BTreeSet<&str> = BTreeSet::new();
    for digest in &manifest.digests {
        for snapshot_ref in &digest.covered_snapshot_refs {
            covered.insert(snapshot_ref.as_str());
        }
    }
    for snapshot in &manifest.snapshots {
        assert!(
            covered.contains(snapshot.snapshot_id.as_str()),
            "{} is stranded with no durable digest",
            snapshot.snapshot_id
        );
    }
}

// ── Guardrails: no trapped experts; educational AI keeps do fenced ───────────

#[test]
fn no_snapshot_forces_blocking_onboarding() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        assert!(
            !snapshot.blocking_onboarding_allowed,
            "{} would trap an expert",
            snapshot.snapshot_id
        );
    }
}

#[test]
fn no_snapshot_changes_authority_ownership_or_the_command_graph() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        assert!(!snapshot.authority_boundary_change_allowed);
        assert!(snapshot.command_graph_unchanged);
    }
}

#[test]
fn educational_ai_flows_route_every_do_through_the_standard_model() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        if snapshot.flow_uses_educational_ai {
            assert!(
                snapshot.educational_ai_uses_standard_preview_approval,
                "{} prepares a do outside the standard model",
                snapshot.snapshot_id
            );
        }
    }
}

// ── Verdict + privacy posture ────────────────────────────────────────────────

#[test]
fn local_only_snapshots_are_individually_stable() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        if !snapshot.sync_policy.is_sync_eligible() {
            assert_eq!(
                snapshot.verdict,
                QualificationVerdict::QualifiedStable,
                "{} should be Stable",
                snapshot.snapshot_id
            );
        }
    }
}

#[test]
fn device_sync_eligible_snapshot_narrows_to_beta_but_stays_disclosed() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let synced = manifest
        .snapshots
        .iter()
        .find(|s| s.sync_policy == DeviceSyncPolicy::DeviceSyncEligibleDisclosed)
        .expect("a device-sync-eligible snapshot");
    assert!(synced.sync_disclosed);
    assert_eq!(synced.verdict, QualificationVerdict::NarrowedBeta);
    assert!(synced
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("device_sync_eligible")));
    // The narrowest member propagates to the overall verdict.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
}

#[test]
fn digest_inherits_the_narrowest_covered_snapshot_verdict() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let synced_digest = manifest
        .digest("learning:m5:digest:synced_progress")
        .expect("synced digest");
    assert_eq!(synced_digest.verdict, QualificationVerdict::NarrowedBeta);
    assert!(synced_digest
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("covered_snapshot")));
}

#[test]
fn stored_verdicts_agree_with_derived_verdicts() {
    let manifest = seeded_m5_learning_progress_snapshots();
    for snapshot in &manifest.snapshots {
        let (derived, reasons) = derive_snapshot_verdict(snapshot);
        assert_eq!(derived, snapshot.verdict, "{}", snapshot.snapshot_id);
        assert_eq!(
            reasons, snapshot.narrowing_reasons,
            "{}",
            snapshot.snapshot_id
        );
    }
}

// ── Negative: validation catches every invariant breach ──────────────────────

#[test]
fn validation_catches_authority_boundary_change() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].authority_boundary_change_allowed = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("authority boundary")));
}

#[test]
fn validation_catches_command_graph_drift() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].command_graph_unchanged = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("command graph")));
}

#[test]
fn validation_catches_non_user_owned_state() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].data_ownership = DataOwnershipClass::RepoVisibleShared;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("user-owned local-first")));
}

#[test]
fn validation_catches_repo_visible_progress() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].privacy.repo_visible = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("repo-visible")));
}

#[test]
fn validation_catches_collaborator_shared_progress() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].privacy.shared_with_collaborators = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("shared with collaborators")));
}

#[test]
fn validation_catches_extension_telemetry_read_access() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0]
        .privacy
        .extension_telemetry_read_access = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("telemetry-grade read access")));
}

#[test]
fn validation_catches_blocking_onboarding() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].blocking_onboarding_allowed = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("blocking onboarding")));
}

#[test]
fn validation_catches_progress_that_does_not_survive_restart() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].survives_restart = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("survive restart")));
}

#[test]
fn validation_catches_non_resumable_resume_point() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let snapshot = manifest
        .snapshots
        .iter_mut()
        .find(|s| s.resume_point.is_some())
        .expect("a resumable snapshot");
    snapshot
        .resume_point
        .as_mut()
        .unwrap()
        .resumable_after_restart = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("resumable after restart")));
}

#[test]
fn validation_catches_irreversible_dismissal() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let snapshot = manifest
        .snapshots
        .iter_mut()
        .find(|s| {
            s.steps
                .iter()
                .any(|st| st.state == StepProgressState::Dismissed)
        })
        .expect("a snapshot with a dismissed step");
    let step = snapshot
        .steps
        .iter_mut()
        .find(|st| st.state == StepProgressState::Dismissed)
        .unwrap();
    step.dismissal_reversible = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("dismissed step") && e.message.contains("reversible")));
}

#[test]
fn validation_catches_educational_ai_do_outside_standard_model() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let snapshot = manifest
        .snapshots
        .iter_mut()
        .find(|s| s.flow_uses_educational_ai)
        .expect("an educational-AI flow");
    snapshot.educational_ai_uses_standard_preview_approval = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("standard preview/approval model")));
}

#[test]
fn validation_catches_undisclosed_sync_masquerade() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let synced = manifest
        .snapshots
        .iter_mut()
        .find(|s| s.sync_policy == DeviceSyncPolicy::DeviceSyncEligibleDisclosed)
        .expect("a sync-eligible snapshot");
    synced.sync_disclosed = false;
    synced.sync_verdict();
    // An undisclosed sync is a masquerade: it narrows to Preview, not Beta.
    assert_eq!(synced.verdict, QualificationVerdict::NarrowedPreview);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("masquerade")));
}

#[test]
fn validation_catches_exported_state_without_export_ref() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let snapshot = manifest
        .snapshots
        .iter_mut()
        .find(|s| s.disclosure_state == SnapshotDisclosureState::Exported)
        .expect("an exported snapshot");
    snapshot.export_refs.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("exported disclosure state without an export ref")));
}

#[test]
fn validation_catches_export_that_leaks_raw_payloads() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let snapshot = manifest
        .snapshots
        .iter_mut()
        .find(|s| !s.export_refs.is_empty())
        .expect("a snapshot with an export");
    snapshot.export_refs[0].redacts_raw_payloads = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("does not redact raw payloads")));
}

#[test]
fn validation_catches_snapshot_with_no_steps() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.snapshots[0].steps.clear();
    // Clearing steps also empties active progress; align disclosure/resume to
    // isolate the "no steps" failure.
    manifest.snapshots[0].resume_point = None;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("no steps")));
}

#[test]
fn validation_catches_digest_missing_required_action() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0]
        .actions
        .retain(|a| a.action_kind != DigestActionKind::Export);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("missing the export action")));
}

#[test]
fn validation_catches_digest_action_that_mutates_workspace() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0].actions[0].mutates_workspace = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("mutates workspace state")));
}

#[test]
fn validation_catches_ephemeral_only_digest() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0].replaces_ephemeral_banners = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("ephemeral banners")));
}

#[test]
fn validation_catches_digest_without_durable_recovery() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0].durable_recovery_available = false;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("durable recovery")));
}

#[test]
fn validation_catches_hidden_overlay_only_digest_state() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0].exposure.in_diagnostics = false;
    manifest.digests[0]
        .exposure
        .hidden_in_transient_overlay_only = true;
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("hidden from settings/help/diagnostics/support")));
}

#[test]
fn validation_catches_digest_covering_unknown_snapshot() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.digests[0]
        .covered_snapshot_refs
        .push("learning:m5:progress:does_not_exist".to_string());
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("covers unknown snapshot")));
}

#[test]
fn validation_catches_stranded_snapshot_with_no_digest() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    // Drop every digest coverage of the first snapshot.
    let stranded = manifest.snapshots[0].snapshot_id.clone();
    for digest in &mut manifest.digests {
        digest.covered_snapshot_refs.retain(|r| r != &stranded);
    }
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("not covered by any durable digest")));
}

#[test]
fn validation_catches_duplicate_snapshot_id() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let clone = manifest.snapshots[0].clone();
    manifest.snapshots.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate snapshot id")));
}

#[test]
fn validation_catches_duplicate_digest_id() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    let clone = manifest.digests[0].clone();
    manifest.digests.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate digest id")));
}

#[test]
fn validation_catches_stored_verdict_divergence() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    // Introduce a hard violation but do NOT re-derive: the stored verdict lies.
    manifest.snapshots[0].blocking_onboarding_allowed = true;
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("disagrees with derived verdict")));
}

#[test]
fn validation_catches_manifest_overall_verdict_drift() {
    let mut manifest = seeded_m5_learning_progress_snapshots();
    manifest.overall_verdict = QualificationVerdict::QualifiedStable;
    let errors = validate_m5_learning_progress_snapshots(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overall verdict")));
}

// ── Lookups ──────────────────────────────────────────────────────────────────

#[test]
fn snapshot_and_digest_lookups_resolve_known_ids_and_reject_unknown() {
    let manifest = seeded_m5_learning_progress_snapshots();
    let first = manifest.snapshots[0].snapshot_id.clone();
    assert!(manifest.snapshot(&first).is_some());
    assert!(manifest
        .snapshot("learning:m5:progress:does_not_exist")
        .is_none());
    let digest = manifest.digests[0].digest_id.clone();
    assert!(manifest.digest(&digest).is_some());
    assert!(manifest
        .digest("learning:m5:digest:does_not_exist")
        .is_none());
}
