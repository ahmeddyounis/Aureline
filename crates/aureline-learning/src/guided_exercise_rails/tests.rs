use super::*;

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_guided_exercise_rails();
    validate_m5_guided_exercise_rails(&manifest)
        .expect("seeded guided-exercise-rail manifest must pass validation");
}

#[test]
fn covers_every_family_with_a_rail() {
    let manifest = seeded_m5_guided_exercise_rails();
    assert_eq!(manifest.rails.len(), M5LearningSurfaceFamily::ALL.len());
    for family in M5LearningSurfaceFamily::ALL {
        assert!(
            manifest.rail(&rail_id(family)).is_some(),
            "missing rail for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_step_references_a_stable_object() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            assert!(
                !step.relies_on_coordinates_only(),
                "{} relies on coordinates alone",
                step.step_id
            );
        }
    }
}

#[test]
fn every_step_carries_a_success_criterion() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            assert!(
                !step.success_criteria.is_empty(),
                "{} has no success criterion",
                step.step_id
            );
        }
    }
}

#[test]
fn every_step_exposes_hint_reveal_reset_skip() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            let kinds = step.action_kinds();
            for required in REQUIRED_ACTION_KINDS {
                assert!(
                    kinds.contains(&required),
                    "{} missing {} control",
                    step.step_id,
                    required.as_str()
                );
            }
        }
    }
}

#[test]
fn every_control_is_inspectable_keyboard_reachable_restart_safe_and_non_mutating() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            for action in &step.actions {
                assert!(
                    action.qualifies_stable(),
                    "{} control {} fails an invariant",
                    step.step_id,
                    action.action_kind.as_str()
                );
                assert!(!action.mutates_workspace);
                assert!(action.keyboard_shortcut_ref.is_some());
                assert!(action.inspectable);
                assert!(action.restart_safe);
            }
        }
    }
}

#[test]
fn explain_and_prepare_steps_never_touch_real_workspace() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            if matches!(
                step.step_kind,
                ExerciseStepKind::Explain | ExerciseStepKind::PreparePractice
            ) {
                assert!(
                    !step.mutation_target.touches_real_workspace(),
                    "{} ({}) escalates into real workspace",
                    step.step_id,
                    step.step_kind.as_str()
                );
                assert!(step.explain_do_separated());
            }
        }
    }
}

#[test]
fn every_apply_step_is_command_backed_through_the_standard_model() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            if step.step_kind.is_apply_capable() {
                assert!(
                    step.command_backing.qualifies_for_apply(),
                    "{} apply step is not command-backed",
                    step.step_id
                );
                assert!(step.command_backing.command_id_ref.is_some());
                assert!(step.command_backing.preview_sheet_ref.is_some());
                assert!(step.command_backing.approval_path_ref.is_some());
                assert!(step.command_backing.trust_policy_check_ref.is_some());
            }
        }
    }
}

#[test]
fn non_apply_steps_have_no_command_backing() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        for step in &rail.steps {
            if matches!(step.step_kind, ExerciseStepKind::Explain) {
                assert!(!step.command_backing.uses_standard_command_model);
                assert_eq!(step.mutation_target, MutationTarget::NoMutation);
            }
        }
    }
}

#[test]
fn progress_is_user_owned_and_restart_safe() {
    let manifest = seeded_m5_guided_exercise_rails();
    for rail in &manifest.rails {
        assert!(
            rail.progress.qualifies_stable(),
            "{} progress",
            rail.rail_id
        );
        assert!(rail.progress.survives_restart);
        assert!(rail.progress.resumable);
        assert!(rail.progress.user_owned_local);
        assert!(!rail.progress.shared_with_repo);
        // A fresh rail resolves to its first step.
        assert!(rail.current_step().is_some());
    }
}

#[test]
fn sandboxed_rails_label_sandbox_availability() {
    let manifest = seeded_m5_guided_exercise_rails();
    let notebook = manifest
        .rail(&rail_id(M5LearningSurfaceFamily::Notebook))
        .expect("notebook rail");
    assert!(notebook.sandbox_preference.prefers_sandbox);
    assert!(notebook.sandbox_preference.sandbox_available);
    // The notebook rail has a sandboxed practice step distinct from the apply step.
    assert!(notebook
        .steps
        .iter()
        .any(|s| s.mutation_target == MutationTarget::SandboxedLocalReversible));
    assert!(notebook
        .steps
        .iter()
        .any(|s| s.mutation_target == MutationTarget::WorkspaceReversibleApproved));
}

#[test]
fn docs_browser_rail_is_read_only_and_stable() {
    let manifest = seeded_m5_guided_exercise_rails();
    let docs = manifest
        .rail(&rail_id(M5LearningSurfaceFamily::DocsBrowser))
        .expect("docs_browser rail");
    assert_eq!(docs.explain_apply_class, ExplainApplyClass::ReadOnly);
    for step in &docs.steps {
        assert_eq!(step.step_kind, ExerciseStepKind::Explain);
        assert_eq!(step.mutation_target, MutationTarget::NoMutation);
    }
    assert_eq!(docs.verdict, QualificationVerdict::QualifiedStable);
}

#[test]
fn cached_and_local_only_rails_narrow_but_stay_disclosed() {
    let manifest = seeded_m5_guided_exercise_rails();

    let companion = manifest
        .rail(&rail_id(M5LearningSurfaceFamily::Companion))
        .expect("companion rail");
    assert_eq!(companion.freshness_state, FreshnessState::CachedDisclosed);
    assert_eq!(companion.verdict, QualificationVerdict::NarrowedBeta);
    assert!(companion.mirror_parity.explicit_freshness_disclosed);

    let preview = manifest
        .rail(&rail_id(M5LearningSurfaceFamily::Preview))
        .expect("preview rail");
    assert_eq!(preview.freshness_state, FreshnessState::LocalOnlyDisclosed);
    assert_eq!(preview.verdict, QualificationVerdict::NarrowedBeta);
    assert!(preview.mirror_parity.explicit_freshness_disclosed);

    // Overall manifest verdict reflects the narrowest member.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
}

#[test]
fn live_families_are_individually_stable() {
    let manifest = seeded_m5_guided_exercise_rails();
    for family in [
        M5LearningSurfaceFamily::Notebook,
        M5LearningSurfaceFamily::RequestWorkspace,
        M5LearningSurfaceFamily::DatabaseWorkspace,
        M5LearningSurfaceFamily::ProfilerTrace,
        M5LearningSurfaceFamily::DocsBrowser,
        M5LearningSurfaceFamily::TemplateScaffold,
        M5LearningSurfaceFamily::SyncOffboarding,
    ] {
        let rail = manifest.rail(&rail_id(family)).expect("rail");
        assert_eq!(
            rail.verdict,
            QualificationVerdict::QualifiedStable,
            "{} should be Stable",
            family.as_str()
        );
    }
}

#[test]
fn localization_preserves_target_and_citation_identity() {
    let manifest = seeded_m5_guided_exercise_rails();
    let rail = manifest
        .rail(&rail_id(M5LearningSurfaceFamily::Notebook))
        .expect("notebook rail");
    let targets = rail.target_ref_fingerprint();
    let citations = rail.citation_ref_fingerprint();
    assert!(!targets.is_empty());
    assert!(!citations.is_empty());
    // Overlays exist and localize labels but carry no target/citation refs.
    assert!(rail.localized_labels("fr-FR").is_some());
    assert!(rail.localized_labels("ja-JP").is_some());
    for overlay in &rail.locale_overlays {
        assert!(overlay.preserves_target_identity);
        assert!(overlay.preserves_citations);
    }
}

#[test]
fn validation_catches_coordinate_only_step() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].steps[0].stable_targets.clear();
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("coordinates alone")));
}

#[test]
fn validation_catches_missing_success_criterion() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].steps[0].success_criteria.clear();
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no success criterion")));
}

#[test]
fn validation_catches_educational_step_escalation() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    // Make an explain step touch real workspace state — a silent escalation.
    let step = &mut manifest.rails[0].steps[0];
    assert_eq!(step.step_kind, ExerciseStepKind::Explain);
    step.mutation_target = MutationTarget::WorkspaceReversibleApproved;
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("escalates")));
}

#[test]
fn validation_catches_apply_without_command_backing() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    // Find an apply step and strip its command backing.
    let rail = &mut manifest.rails[0];
    let step = rail
        .steps
        .iter_mut()
        .find(|s| s.step_kind.is_apply_capable())
        .expect("an apply step");
    step.command_backing = CommandBacking::none();
    rail.sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("standard command/preview/approval model")));
}

#[test]
fn validation_catches_irreversible_real_mutation() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    let rail = &mut manifest.rails[0];
    let step = rail
        .steps
        .iter_mut()
        .find(|s| s.step_kind.is_apply_capable())
        .expect("an apply step");
    step.mutation_target = MutationTarget::WorkspaceIrreversibleApproved;
    rail.sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("irreversibly")));
}

#[test]
fn validation_catches_control_that_mutates_workspace() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].steps[0].actions[0].mutates_workspace = true;
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("mutates workspace state")));
}

#[test]
fn validation_catches_control_without_keyboard_shortcut() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].steps[0].actions[0].keyboard_shortcut_ref = None;
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("not keyboard reachable")));
}

#[test]
fn validation_catches_missing_required_control() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    // Drop the Skip control from a step.
    manifest.rails[0].steps[0]
        .actions
        .retain(|a| a.action_kind != ExerciseActionKind::Skip);
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("missing a keyboard-reachable skip")));
}

#[test]
fn validation_catches_progress_shared_with_repo() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].progress.shared_with_repo = true;
    manifest.rails[0].sync_verdict();
    // The verdict folds the progress posture; stored verdict now diverges had we
    // not synced, but we synced, so validation passes structurally — assert the
    // verdict narrowed instead.
    assert_eq!(
        manifest.rails[0].verdict,
        QualificationVerdict::NarrowedBeta
    );
    assert!(manifest.rails[0]
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("progress")));
}

#[test]
fn validation_catches_progress_past_last_step() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    let len = manifest.rails[0].steps.len() as u32;
    manifest.rails[0].progress.current_step_index = len;
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("past the last step")));
}

#[test]
fn validation_catches_freshness_masquerade() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    let companion = manifest
        .rails
        .iter_mut()
        .find(|r| r.family == M5LearningSurfaceFamily::Companion)
        .expect("companion rail");
    companion.mirror_parity.explicit_freshness_disclosed = false;
    companion.sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("masquerade")));
}

#[test]
fn validation_catches_locale_overlay_dropping_identity() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].locale_overlays[0].preserves_target_identity = false;
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("drops target identity")));
}

#[test]
fn validation_catches_conflated_step() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0].steps[0].explain_apply_class = ExplainApplyClass::Conflated;
    manifest.rails[0].sync_verdict();
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("conflates explain/apply")));
}

#[test]
fn validation_catches_unresolved_prerequisite() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    manifest.rails[0]
        .prerequisite_rail_refs
        .push("learning:m5:exercise_rail:does_not_exist:v1".to_string());
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("unresolved prerequisite")));
}

#[test]
fn validation_catches_prerequisite_cycle() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    let a = manifest.rails[0].rail_id.clone();
    let b = manifest.rails[1].rail_id.clone();
    manifest.rails[0].prerequisite_rail_refs = vec![b.clone()];
    manifest.rails[1].prerequisite_rail_refs = vec![a.clone()];
    let errors = validate_m5_guided_exercise_rails(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("cycle")));
}

#[test]
fn in_namespace_prerequisite_resolves() {
    let mut manifest = seeded_m5_guided_exercise_rails();
    let dep = manifest.rails[1].rail_id.clone();
    manifest.rails[0].prerequisite_rail_refs = vec![dep];
    // A resolvable, acyclic prerequisite does not break validation.
    validate_m5_guided_exercise_rails(&manifest).expect("resolvable prerequisite is fine");
}

#[test]
fn manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_guided_exercise_rails();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_manifest_from_json(&json).expect("deserialize");
    assert_eq!(manifest, back);
    // Target/citation fingerprints survive export + reopen unchanged.
    for (orig, reopened) in manifest.rails.iter().zip(back.rails.iter()) {
        assert_eq!(
            orig.target_ref_fingerprint(),
            reopened.target_ref_fingerprint()
        );
        assert_eq!(
            orig.citation_ref_fingerprint(),
            reopened.citation_ref_fingerprint()
        );
    }
}
