use super::*;

// ── Seed integrity ──────────────────────────────────────────────────────────

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_learning_mode_profiles();
    validate_m5_learning_mode_profiles(&manifest)
        .expect("seeded learning-mode-profile manifest must pass validation");
}

#[test]
fn seeded_manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_learning_mode_profiles();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_profile_manifest_from_json(&json).expect("deserialize");
    assert_eq!(manifest, back);
    // Scope, control, and history identity survive export + reopen unchanged.
    assert_eq!(manifest.known_profile_ids(), back.known_profile_ids());
    for (orig, reopened) in manifest.profiles.iter().zip(back.profiles.iter()) {
        assert_eq!(orig.control_kinds(), reopened.control_kinds());
        assert_eq!(orig.change_history, reopened.change_history);
        assert_eq!(orig.scope_binding, reopened.scope_binding);
    }
}

#[test]
fn ships_one_profile_per_preset_plus_a_workspace_profile() {
    let manifest = seeded_m5_learning_mode_profiles();
    assert!(manifest
        .profiles
        .iter()
        .any(|p| p.preset == LearningModePreset::ExpertMinimal));
    assert!(manifest
        .profiles
        .iter()
        .any(|p| p.preset == LearningModePreset::BalancedDefault
            && p.scope_binding.scope == ProfileScope::UserLocal));
    assert!(manifest
        .profiles
        .iter()
        .any(|p| p.preset == LearningModePreset::GuidedLearner));
    assert!(manifest
        .profiles
        .iter()
        .any(|p| p.scope_binding.scope == ProfileScope::WorkspaceOptIn));
}

// ── Acceptance: lifecycle controls never break the command graph ─────────────

#[test]
fn every_profile_can_be_enabled_disabled_reset_and_narrowed() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        let kinds = profile.control_kinds();
        for required in REQUIRED_CONTROL_KINDS {
            assert!(
                kinds.contains(&required),
                "{} missing {} control",
                profile.profile_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn every_control_is_command_backed_keyboard_reachable_reversible_and_non_mutating() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        for control in &profile.controls {
            assert!(
                control.qualifies_stable(),
                "{} control {} fails an invariant",
                profile.profile_id,
                control.control_kind.as_str()
            );
            assert!(!control.command_id_ref.is_empty());
            assert!(control.keyboard_shortcut_ref.is_some());
            assert!(control.reversible);
            assert!(control.inspectable);
            assert!(!control.silent_write_allowed);
            assert!(!control.mutates_workspace);
        }
    }
}

#[test]
fn no_profile_changes_authority_ownership_or_the_command_graph() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(!profile.authority_boundary_change_allowed);
        assert!(profile.command_graph_unchanged);
        assert_eq!(
            profile.data_ownership,
            DataOwnershipClass::UserOwnedLocalFirst
        );
    }
}

// ── Acceptance: scope stays explicit, never leaks into the repo ──────────────

#[test]
fn scope_is_explicit_and_never_committed_or_shared() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(
            !profile.scope_binding.repo_committed,
            "{}",
            profile.profile_id
        );
        assert!(
            !profile.scope_binding.shared_with_collaborators,
            "{}",
            profile.profile_id
        );
        if profile.scope_binding.scope == ProfileScope::WorkspaceOptIn {
            assert!(
                profile.scope_binding.opt_in_explicit,
                "{} workspace scope must be explicitly opted in",
                profile.profile_id
            );
        }
        assert!(profile.scope_binding.qualifies_stable());
    }
}

#[test]
fn dismissals_and_bookmarks_are_user_owned_and_reversible() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(
            profile.dismissals.qualifies_stable(),
            "{}",
            profile.profile_id
        );
        assert!(profile.dismissals.reversible);
        assert!(profile.dismissals.user_owned_local);
        assert!(profile.dismissals.follows_profile_scope);
        assert!(
            profile.bookmarks.qualifies_stable(),
            "{}",
            profile.profile_id
        );
        assert!(profile.bookmarks.user_owned_local);
    }
}

// ── Acceptance: posture + guardrails are inspectable everywhere ──────────────

#[test]
fn state_is_inspectable_in_settings_help_diagnostics_and_support() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(
            profile.exposure.qualifies_stable(),
            "{}",
            profile.profile_id
        );
        assert!(profile.exposure.in_settings);
        assert!(profile.exposure.in_help_about);
        assert!(profile.exposure.in_diagnostics);
        assert!(profile.exposure.in_support_export);
        assert!(!profile.exposure.hidden_in_transient_overlay_only);
    }
}

#[test]
fn change_history_is_user_initiated_and_inspectable_in_support_export() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        for event in &profile.change_history {
            assert!(event.qualifies_stable(), "{}", event.event_id);
            assert!(event.user_initiated);
            assert!(event.inspectable_in_support_export);
        }
    }
}

// ── Guardrails: experts never trapped; educational AI keeps do fenced ────────

#[test]
fn no_profile_forces_blocking_onboarding() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(
            !profile.blocking_onboarding_allowed,
            "{} would trap an expert",
            profile.profile_id
        );
    }
}

#[test]
fn learner_presets_keep_explain_before_act_experts_may_opt_out() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        if profile.preset.requires_explain_before_act() {
            assert!(
                profile.explain_before_act_default,
                "{} learner preset dropped explain-before-act",
                profile.profile_id
            );
        }
    }
    // The expert-minimal preset is allowed to turn forced pre-explanation off…
    let expert = manifest
        .profiles
        .iter()
        .find(|p| p.preset == LearningModePreset::ExpertMinimal)
        .expect("expert-minimal profile");
    assert!(!expert.explain_before_act_default);
    // …but it never opts out of the mutation fence.
    assert!(expert.mutation_guardrail.permits_fenced_do());
    assert!(expert.educational_ai_uses_standard_preview_approval);
}

#[test]
fn educational_ai_routes_every_prepared_do_through_the_standard_model() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        if profile.ai_explanation_posture.permits_do() {
            assert!(
                profile.educational_ai_uses_standard_preview_approval,
                "{} prepares a do outside the standard model",
                profile.profile_id
            );
            assert!(
                profile.mutation_guardrail.permits_fenced_do(),
                "{} posture prepares a do the guardrail forbids",
                profile.profile_id
            );
        }
    }
}

#[test]
fn no_mutation_guardrail_permits_an_unfenced_live_write() {
    // There is deliberately no "unfenced" guardrail value.
    for guardrail in [
        MutationGuardrail::ExplainOnlyNoMutation,
        MutationGuardrail::PreviewRequired,
        MutationGuardrail::ApprovalRequired,
        MutationGuardrail::BlockedUntilTrust,
    ] {
        // ExplainOnly forbids any do; every other guardrail still fences the do.
        if guardrail.permits_fenced_do() {
            assert_ne!(guardrail, MutationGuardrail::ExplainOnlyNoMutation);
        }
    }
}

// ── Verdict + privacy posture ────────────────────────────────────────────────

#[test]
fn user_local_profiles_are_individually_stable() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        if profile.scope_binding.scope == ProfileScope::UserLocal
            && profile.sync_posture == SyncPosture::LocalOnly
        {
            assert_eq!(
                profile.verdict,
                QualificationVerdict::QualifiedStable,
                "{} should be Stable",
                profile.profile_id
            );
        }
    }
}

#[test]
fn portable_sync_narrows_to_beta_but_stays_disclosed() {
    let manifest = seeded_m5_learning_mode_profiles();
    let synced = manifest
        .profiles
        .iter()
        .find(|p| p.sync_posture == SyncPosture::PortableProfileSynced)
        .expect("a portable-profile-synced profile");
    assert!(synced.sync_disclosed);
    assert_eq!(synced.verdict, QualificationVerdict::NarrowedBeta);
    assert!(synced
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("portable_profile_sync")));
    // The narrowest member propagates to the overall verdict.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
}

#[test]
fn stored_verdicts_agree_with_derived_verdicts() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        let (derived, reasons) = derive_learning_mode_profile_verdict(profile);
        assert_eq!(derived, profile.verdict, "{}", profile.profile_id);
        assert_eq!(reasons, profile.narrowing_reasons, "{}", profile.profile_id);
    }
}

// ── Negative: validation catches every invariant breach ──────────────────────

#[test]
fn validation_catches_authority_boundary_change() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].authority_boundary_change_allowed = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("authority boundary")));
}

#[test]
fn validation_catches_command_graph_drift() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].command_graph_unchanged = false;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("command graph")));
}

#[test]
fn validation_catches_non_user_owned_state() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].data_ownership = DataOwnershipClass::RepoVisibleShared;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("user-owned local-first")));
}

#[test]
fn validation_catches_blocking_onboarding() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].blocking_onboarding_allowed = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("blocking onboarding")));
}

#[test]
fn validation_catches_learner_profile_dropping_explain_before_act() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let learner = manifest
        .profiles
        .iter_mut()
        .find(|p| p.preset == LearningModePreset::GuidedLearner)
        .expect("guided-learner profile");
    learner.explain_before_act_default = false;
    learner.sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("explain-before-act")));
}

#[test]
fn validation_catches_educational_ai_do_outside_standard_model() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let profile = manifest
        .profiles
        .iter_mut()
        .find(|p| p.ai_explanation_posture.permits_do())
        .expect("a do-capable profile");
    profile.educational_ai_uses_standard_preview_approval = false;
    profile.sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("standard preview/approval model")));
}

#[test]
fn validation_catches_do_posture_the_guardrail_forbids() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let profile = manifest
        .profiles
        .iter_mut()
        .find(|p| p.ai_explanation_posture.permits_do())
        .expect("a do-capable profile");
    profile.mutation_guardrail = MutationGuardrail::ExplainOnlyNoMutation;
    profile.sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("guardrail forbids")));
}

#[test]
fn validation_catches_profile_committed_into_repo() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].scope_binding.repo_committed = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("committed into repo state")));
}

#[test]
fn validation_catches_profile_shared_with_collaborators() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].scope_binding.shared_with_collaborators = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("shared with collaborators")));
}

#[test]
fn validation_catches_workspace_scope_without_explicit_opt_in() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let ws = manifest
        .profiles
        .iter_mut()
        .find(|p| p.scope_binding.scope == ProfileScope::WorkspaceOptIn)
        .expect("a workspace-scoped profile");
    ws.scope_binding.opt_in_explicit = false;
    ws.sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("explicitly opted in")));
}

#[test]
fn validation_catches_undisclosed_sync_masquerade() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let synced = manifest
        .profiles
        .iter_mut()
        .find(|p| p.sync_posture == SyncPosture::PortableProfileSynced)
        .expect("a synced profile");
    synced.sync_disclosed = false;
    synced.sync_verdict();
    // An undisclosed sync is a masquerade: it narrows to Preview, not Beta.
    assert_eq!(synced.verdict, QualificationVerdict::NarrowedPreview);
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("masquerade")));
}

#[test]
fn validation_catches_hidden_overlay_only_state() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].exposure.in_diagnostics = false;
    manifest.profiles[0]
        .exposure
        .hidden_in_transient_overlay_only = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("hidden from settings/help/diagnostics/support")));
}

#[test]
fn validation_catches_irreversible_dismissal() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].dismissals.reversible = false;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("dismissals")));
}

#[test]
fn validation_catches_non_user_owned_bookmarks() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].bookmarks.user_owned_local = false;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("bookmarks")));
}

#[test]
fn validation_catches_control_that_mutates_workspace() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].controls[0].mutates_workspace = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("mutates workspace state")));
}

#[test]
fn validation_catches_control_with_silent_write() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].controls[0].silent_write_allowed = true;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("silent write")));
}

#[test]
fn validation_catches_control_without_keyboard_shortcut() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].controls[0].keyboard_shortcut_ref = None;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("keyboard reachable")));
}

#[test]
fn validation_catches_missing_required_control() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0]
        .controls
        .retain(|c| c.control_kind != ProfileControlKind::Reset);
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("missing the reset control")));
}

#[test]
fn validation_catches_silent_change_event() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].change_history[0].user_initiated = false;
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("silent change")));
}

#[test]
fn validation_catches_duplicate_profile_id() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    let clone = manifest.profiles[0].clone();
    manifest.profiles.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate profile id")));
}

#[test]
fn validation_catches_profile_with_no_surface_family() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.profiles[0].applies_to_families.clear();
    manifest.profiles[0].sync_verdict();
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no surface family")));
}

#[test]
fn validation_catches_stored_verdict_divergence() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    // Introduce a hard violation but do NOT re-derive: the stored verdict now lies.
    manifest.profiles[0].blocking_onboarding_allowed = true;
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("disagrees with derived verdict")));
}

#[test]
fn validation_catches_manifest_overall_verdict_drift() {
    let mut manifest = seeded_m5_learning_mode_profiles();
    manifest.overall_verdict = QualificationVerdict::QualifiedStable;
    let errors = validate_m5_learning_mode_profiles(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overall verdict")));
}

// ── Lookups ──────────────────────────────────────────────────────────────────

#[test]
fn profile_lookup_resolves_known_ids_and_rejects_unknown() {
    let manifest = seeded_m5_learning_mode_profiles();
    let first = manifest.profiles[0].profile_id.clone();
    assert!(manifest.profile(&first).is_some());
    assert!(manifest
        .profile("learning:m5:profile:does_not_exist")
        .is_none());
}

#[test]
fn every_profile_applies_to_at_least_one_family() {
    let manifest = seeded_m5_learning_mode_profiles();
    for profile in &manifest.profiles {
        assert!(
            !profile.applies_to_families.is_empty(),
            "{}",
            profile.profile_id
        );
    }
}
