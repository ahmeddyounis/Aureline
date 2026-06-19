use super::*;

// ── Seed integrity ──────────────────────────────────────────────────────────

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_educational_ai_and_practice();
    validate_m5_educational_ai_and_practice(&manifest)
        .expect("seeded educational-AI manifest must pass validation");
}

#[test]
fn seeded_manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_educational_ai_and_practice();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back = reopen_educational_ai_manifest_from_json(&json).expect("deserialize");
    assert_eq!(manifest, back);
    // Citation, scope, and practice identity survive export + reopen.
    for (orig, reopened) in manifest.panels.iter().zip(back.panels.iter()) {
        assert_eq!(orig.citations, reopened.citations);
        assert_eq!(orig.truth_source_scope, reopened.truth_source_scope);
        assert_eq!(orig.open_resource_actions, reopened.open_resource_actions);
    }
    for (orig, reopened) in manifest
        .practice_indicators
        .iter()
        .zip(back.practice_indicators.iter())
    {
        assert_eq!(orig.surface_state, reopened.surface_state);
        assert_eq!(orig.reset_behavior, reopened.reset_behavior);
        assert_eq!(orig.persistence_note, reopened.persistence_note);
    }
}

#[test]
fn ships_panels_and_practice_across_several_families() {
    let manifest = seeded_m5_educational_ai_and_practice();
    assert!(manifest.panels.len() >= 4);
    assert!(manifest.practice_indicators.len() >= 3);
    assert!(
        manifest.families_covered().len() >= 4,
        "records should span several families"
    );
}

// ── Acceptance: cited, never omniscient or action-capable ────────────────────

#[test]
fn repository_truth_panels_cite_and_keep_open_actions_one_step_away() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for panel in &manifest.panels {
        if panel.claims_repository_truth {
            assert!(
                !panel.open_resource_actions.is_empty(),
                "{} must keep an open-source/open-docs action one step away",
                panel.panel_id
            );
            if panel.truth_source_scope == TruthSourceScope::LiveRepoState {
                assert!(
                    !panel.citations.is_empty(),
                    "{} claims live repo truth and must cite it",
                    panel.panel_id
                );
            }
        }
        for action in &panel.open_resource_actions {
            assert_eq!(action.steps_away, 1, "{}", panel.panel_id);
            assert!(action.keyboard_shortcut_ref.is_some(), "{}", panel.panel_id);
        }
    }
}

#[test]
fn no_panel_sounds_omniscient_or_directly_action_capable() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for panel in &manifest.panels {
        assert!(!panel.presents_as_omniscient, "{}", panel.panel_id);
        assert!(
            !panel.claims_direct_action_without_approval,
            "{}",
            panel.panel_id
        );
    }
}

#[test]
fn every_panel_keeps_explain_separate_from_do() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for panel in &manifest.panels {
        assert!(
            panel.explain_apply_class.qualifies_stable(),
            "{} conflates explain and do",
            panel.panel_id
        );
        if panel.explain_apply_class == ExplainApplyClass::ApplyRequiresApproval {
            assert!(
                panel.mutation_routes_through_standard_preview_approval,
                "{} must fence its do",
                panel.panel_id
            );
        }
    }
}

#[test]
fn notebook_panel_cites_file_symbol_and_command() {
    let manifest = seeded_m5_educational_ai_and_practice();
    let panel = manifest
        .panel("learning:m5:edu:panel:notebook_explain")
        .expect("notebook panel");
    let kinds = panel.citation_kinds();
    assert!(kinds.contains(&CitationKind::File));
    assert!(kinds.contains(&CitationKind::Symbol));
    assert!(kinds.contains(&CitationKind::Command));
    assert!(panel.has_open_resource(OpenResourceKind::OpenSource));
    assert!(panel.has_open_resource(OpenResourceKind::OpenDocs));
    assert_eq!(panel.verdict, QualificationVerdict::QualifiedStable);
}

// ── Acceptance: practice/sandbox distinct from live state ─────────────────────

#[test]
fn every_practice_indicator_declares_scope_reset_and_persistence() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for indicator in &manifest.practice_indicators {
        assert!(
            !indicator.target_scope_refs.is_empty(),
            "{} must declare a target scope",
            indicator.indicator_id
        );
        assert!(
            !indicator.persistence_note.trim().is_empty(),
            "{} must carry a persistence note",
            indicator.indicator_id
        );
        assert!(
            indicator.distinct_from_live_workspace,
            "{} must be distinct from the live workspace",
            indicator.indicator_id
        );
        assert!(
            indicator.reversible_or_discardable,
            "{} work must be reversible or discardable",
            indicator.indicator_id
        );
    }
}

#[test]
fn sandbox_and_local_only_practice_are_stable_live_practice_is_disclosed_beta() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for indicator in &manifest.practice_indicators {
        match indicator.surface_state {
            PracticeSurfaceState::Simulated | PracticeSurfaceState::LocalOnly => {
                assert_eq!(
                    indicator.verdict,
                    QualificationVerdict::QualifiedStable,
                    "{} should be Stable",
                    indicator.indicator_id
                );
            }
            PracticeSurfaceState::LiveRepoState => {
                assert_eq!(
                    indicator.verdict,
                    QualificationVerdict::NarrowedBeta,
                    "{} live practice should narrow to Beta (disclosed)",
                    indicator.indicator_id
                );
                assert!(indicator
                    .narrowing_reasons
                    .iter()
                    .any(|r| r.contains("live_repo_state_practice")));
            }
        }
    }
}

#[test]
fn live_practice_mutations_ride_the_standard_preview_approval_fence() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for indicator in &manifest.practice_indicators {
        if indicator.mutates_live_state {
            assert!(
                indicator.mutation_routes_through_standard_preview_approval,
                "{} mutates live state outside the standard fence",
                indicator.indicator_id
            );
        }
    }
}

// ── Acceptance: overlays respect quiet-hours / accessibility / offline ────────

#[test]
fn every_overlay_respects_quiet_hours_motion_and_accessibility() {
    let manifest = seeded_m5_educational_ai_and_practice();
    let overlays = manifest
        .panels
        .iter()
        .map(|p| (&p.panel_id, &p.overlay))
        .chain(
            manifest
                .practice_indicators
                .iter()
                .map(|i| (&i.indicator_id, &i.overlay)),
        );
    for (id, overlay) in overlays {
        assert!(overlay.qualifies_stable(), "{id}");
        assert!(overlay.respects_quiet_hours, "{id}");
        assert!(overlay.respects_reduced_motion, "{id}");
        assert!(overlay.keyboard_reachable, "{id}");
        assert!(overlay.screen_reader_labeled, "{id}");
        assert!(overlay.client_scoped_not_global, "{id}");
        assert!(!overlay.spams_attention_surface, "{id}");
    }
}

#[test]
fn cached_offline_parity_narrows_to_beta_and_propagates() {
    let manifest = seeded_m5_educational_ai_and_practice();
    let cached = manifest
        .panel("learning:m5:edu:panel:database_why_now_cached")
        .expect("cached panel");
    assert_eq!(cached.offline_parity, OfflineParity::CachedDisclosed);
    assert_eq!(cached.verdict, QualificationVerdict::NarrowedBeta);
    assert!(cached
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("offline_mirror_freshness_disclosed")));
    // The narrowest member propagates to the overall verdict.
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
}

// ── Verdict integrity ─────────────────────────────────────────────────────────

#[test]
fn stored_verdicts_agree_with_derived_verdicts() {
    let manifest = seeded_m5_educational_ai_and_practice();
    for panel in &manifest.panels {
        let (derived, reasons) = derive_panel_verdict(panel);
        assert_eq!(derived, panel.verdict, "{}", panel.panel_id);
        assert_eq!(reasons, panel.narrowing_reasons, "{}", panel.panel_id);
    }
    for indicator in &manifest.practice_indicators {
        let (derived, reasons) = derive_practice_indicator_verdict(indicator);
        assert_eq!(derived, indicator.verdict, "{}", indicator.indicator_id);
        assert_eq!(
            reasons, indicator.narrowing_reasons,
            "{}",
            indicator.indicator_id
        );
    }
}

// ── Negative: validation catches every invariant breach ───────────────────────

#[test]
fn validation_catches_uncited_repository_truth_claim() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let panel = manifest
        .panels
        .iter_mut()
        .find(|p| {
            p.claims_repository_truth && p.truth_source_scope == TruthSourceScope::LiveRepoState
        })
        .expect("a live-repo-truth panel");
    panel.citations.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("without a citation")));
}

#[test]
fn validation_catches_repository_truth_without_open_resource_action() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let panel = manifest
        .panels
        .iter_mut()
        .find(|p| p.claims_repository_truth)
        .expect("a repo-truth panel");
    panel.open_resource_actions.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("one step away")));
}

#[test]
fn validation_catches_omniscient_panel() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].presents_as_omniscient = true;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("omniscient")));
}

#[test]
fn validation_catches_direct_action_claim() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].claims_direct_action_without_approval = true;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("act directly without approval")));
}

#[test]
fn validation_catches_conflated_explain_and_do() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].explain_apply_class = ExplainApplyClass::Conflated;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("conflates explain and do")));
}

#[test]
fn validation_catches_unfenced_educational_ai_do() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let panel = manifest
        .panels
        .iter_mut()
        .find(|p| p.explain_apply_class == ExplainApplyClass::ApplyRequiresApproval)
        .expect("an apply-gated panel");
    panel.mutation_routes_through_standard_preview_approval = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("standard preview/approval model")));
}

#[test]
fn validation_catches_open_action_not_one_step_away() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].open_resource_actions[0].steps_away = 3;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("not one step away")));
}

#[test]
fn validation_catches_pointer_only_overlay() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].overlay.keyboard_reachable = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overlay")));
    // The pointer-only path is a hard Preview narrowing.
    assert_eq!(
        manifest.panels[0].verdict,
        QualificationVerdict::NarrowedPreview
    );
}

#[test]
fn validation_catches_overlay_that_ignores_quiet_hours() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].overlay.respects_quiet_hours = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("quiet-hours")));
    assert!(manifest.panels[0]
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("quiet_hours")));
}

#[test]
fn validation_catches_attention_spam() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].overlay.spams_attention_surface = true;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overlay")));
    assert!(manifest.panels[0]
        .narrowing_reasons
        .iter()
        .any(|r| r.contains("spams_attention_surface")));
}

#[test]
fn validation_catches_offline_dead_link() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].offline_parity = OfflineParity::MissingOnOffline;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("dead link")));
    assert_eq!(
        manifest.panels[0].verdict,
        QualificationVerdict::NarrowedPreview
    );
}

#[test]
fn validation_catches_expert_trap() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.panels[0].traps_expert_in_tutorial = true;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("traps an expert")));
}

#[test]
fn validation_catches_practice_not_distinct_from_live() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.practice_indicators[0].distinct_from_live_workspace = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("not distinct from the live workspace")));
}

#[test]
fn validation_catches_live_practice_mutation_outside_fence() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let indicator = manifest
        .practice_indicators
        .iter_mut()
        .find(|i| i.surface_state == PracticeSurfaceState::LiveRepoState)
        .expect("a live-repo-state practice surface");
    indicator.mutation_routes_through_standard_preview_approval = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e
        .message
        .contains("outside the standard preview/approval model")));
}

#[test]
fn validation_catches_practice_without_target_scope() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.practice_indicators[0].target_scope_refs.clear();
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("does not declare a target scope")));
}

#[test]
fn validation_catches_practice_without_persistence_note() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.practice_indicators[0].persistence_note = "   ".to_string();
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("no persistence note")));
}

#[test]
fn validation_catches_non_discardable_practice() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.practice_indicators[0].reversible_or_discardable = false;
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("neither reversible nor discardable")));
}

#[test]
fn validation_catches_duplicate_panel_id() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let clone = manifest.panels[0].clone();
    manifest.panels.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate panel id")));
}

#[test]
fn validation_catches_duplicate_indicator_id() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    let clone = manifest.practice_indicators[0].clone();
    manifest.practice_indicators.push(clone);
    manifest.sync_verdicts();
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("duplicate indicator id")));
}

#[test]
fn validation_catches_stored_verdict_divergence() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    // Introduce a hard violation but do NOT re-derive: the stored verdict lies.
    manifest.panels[0].presents_as_omniscient = true;
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("disagrees with derived verdict")));
}

#[test]
fn validation_catches_manifest_overall_verdict_drift() {
    let mut manifest = seeded_m5_educational_ai_and_practice();
    manifest.overall_verdict = QualificationVerdict::QualifiedStable;
    let errors = validate_m5_educational_ai_and_practice(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("overall verdict")));
}

// ── Lookups ──────────────────────────────────────────────────────────────────

#[test]
fn panel_and_indicator_lookups_resolve_known_ids_and_reject_unknown() {
    let manifest = seeded_m5_educational_ai_and_practice();
    let first = manifest.panels[0].panel_id.clone();
    assert!(manifest.panel(&first).is_some());
    assert!(manifest
        .panel("learning:m5:edu:panel:does_not_exist")
        .is_none());
    let indicator = manifest.practice_indicators[0].indicator_id.clone();
    assert!(manifest.practice_indicator(&indicator).is_some());
    assert!(manifest
        .practice_indicator("learning:m5:practice:does_not_exist")
        .is_none());
}
